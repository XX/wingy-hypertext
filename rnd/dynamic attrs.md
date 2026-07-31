Проконсультируй, есть ли возможность пробрасывать коллекцию атрибутов из компонента в тег в макросе rsx? Например, динамические data-* атрибуты компонента должны устанавливаться в атрибуты вложенного div. Какие есть варианты это организовать?

---

● Теперь напишу проверочный демо-крейт, чтобы дать точный ответ по вариантам.
● Разобрался по коду. Короткий ответ: встроенного «spread» атрибутов сейчас нет, и по устройству макроса его нельзя эмулировать «легально» — но есть три рабочих обходных пути и понятная точка расширения.

  Почему нет

  - Имя атрибута в макросе — всегда набор литералов времени компиляции: AttributeName (crates/hypertext-macros/src/html/mod.rs:584) допускает только data-*, ns:name, @x/:x, обычное имя и "строка" (unchecked). В
  Element::generate (mod.rs:381) имя пишется через g.push_lits(...), т.е. в результирующую строку зашивается константа.
  - В позиции атрибута парсер вообще не принимает выражение: цикл в parse_element (syntaxes/rsx.rs:113) читает только Attribute.
  - Context заперт (crates/hypertext/src/context.rs:37): есть только Node и AttributeValue. Контекста «список атрибутов», в который мог бы рендериться пользовательский тип, не существует.

  Хорошая новость: data-* и "..."-имена не проходят проверку типов атрибутов (AttributeName::check → None для Data и Unchecked, mod.rs:604), так что валидация здесь не мешает.

  Вариант 1 — фиксированный набор ключей + опциональные значения (без unsafe)

  Если множество имён известно на этапе компиляции, а динамика только в наличии/значении, всё уже есть — AttributeKind::Option (mod.rs:542):

  rsx! {
      <div class="card" data-id=[self.data_id] data-role=[self.data_role] data-open[self.open]>
  }

  None — атрибут не выводится вовсе. Плюс .class со списком/тогглами для слияния классов. Это самый чистый путь, и на практике 90% «динамических data-*» сюда попадают.

  Вариант 2 — свой тип-коллекция + ручная сборка открывающего тега

  Для настоящего рантайм-словаря. Компонент принимает коллекцию как обычный prop (билдеру всё равно, тип любой), а тег собирается вручную; rsx! остаётся для потрохов:
```rust                           
  pub struct Attrs(Vec<(Cow<'static, str>, Cow<'static, str>)>);

  impl Attrs {                                                                             
      fn render_into(&self, buffer: &mut Buffer) {                                         
          for (name, value) in &self.0 {                                                   
              debug_assert!(!name.chars().any(|c| c.is_whitespace() || c.is_control()      
                  || matches!(c, '=' | '/' | '>' | '"' | '\'')));                          
              // XSS SAFETY: имя провалидировано, значение экранировано как значение       
              // атрибута в двойных кавычках.                                              
              let s = buffer.dangerously_get_string();                                     
              s.push(' '); s.push_str(name); s.push_str("=\"");                            
              s.push_str(&escape_attr(value)); s.push('"');                                
          }                                                                                
      }                                                                                    
  }                                                                                        
                                                                                           
  fn card<'a>(attrs: &'a Attrs, children: &'a dyn Renderable) -> impl Renderable + 'a {    
      // XSS SAFETY: имя тега статично, атрибуты валидируются/экранируются в Attrs.        
      Lazy::dangerously_create(move |buffer: &mut Buffer| {                                
          buffer.dangerously_get_string().push_str("<div class=\"card\"");                 
          attrs.render_into(buffer);                                                       
          buffer.dangerously_get_string().push('>');                                       
          buffer.push(rsx! { <span class="card__body">(children)</span> });                
          buffer.dangerously_get_string().push_str("</div>");                              
      })                                                                                   
  }
```

  Проверил на живом крейте — даёт
  <div class="card" data-id="42" data-label="a &amp; &quot;b&quot;" aria-hidden="true"><span class="card__body"><p>hello</p></span></div>.
  
  Цена: теряется type-check элемента для этого одного тега, и «грязный» код изолирован в одной функции-обёртке на компонент. Эквивалентная версия через (Raw::dangerously_create(format!("<div 
  class=\"card\"{tail}>"))) внутри rsx! тоже работает — читается чуть приятнее, но аллоцирует строку.
  
  Вариант 3 — не делайте так                                                               
  
  Технически можно «выйти» из значения атрибута через RawAttribute::dangerously_create, дописав закрывающую кавычку и пары. Работает (проверил), но оставляет мусорный data-attrs="" и полностью ломает инвариант
  «значение экранировано». Только как аварийный костыль.
  
  Вариант 4 — расширить макрос (то, чего тут действительно не хватает)                     
  
  Логичное продолжение вашего PROPOSAL, и изменения локальные:                             
  
  1. Синтаксис. В позиции атрибута ( сейчас свободен в обоих синтаксисах, так что <div (attrs) class="x"> и div (attrs) { } не конфликтуют ни с чем. Альтернативы — ..(attrs) (визуально ближе к Rust-спреду) или
  {..attrs}.
  2. Новый контекст. Добавить context::Attributes (в sealed-модуль + AsMut<Buffer<Attributes>> for Buffer, по образцу AttributeBuffer, buffer.rs:170) и безопасный API Buffer<Attributes>::push_attribute(name: 
  &str, value: impl Renderable<AttributeValue>), который сам валидирует имя и экранирует значение. Тогда пользовательский тип реализует Renderable<Attributes> вообще без dangerously_* — в отличие от варианта 2.
  3. Генератор. Новый AttributeKind::Spread(expr) → g.push_expr::<Attributes>(...) в Element::generate; check() для него возвращает None, чтобы не ломать ElementCheck.
  4. Что решить в дизайне: дубликаты (браузер берёт первый — значит спред, стоящий после статических атрибутов, не сможет переопределить class); слияние class/style (это ровно то, для чего у вас в PROPOSAL
  появился CommonAttrs — вероятно, стоит дать отдельный тип со сливающейся семантикой, а не общий словарь); нужен ли спред у void-элементов (да, безвредно).
  
  Со стороны компонентов при этом ничего изобретать не надо: prop типа Attrs и сеттер .attr(name, value) в билдере, а внутри — <div (self.attrs)>.
  
  Рекомендация: если ключи известны — вариант 1; если нужен словарь и патчить макрос сейчас не хочется — вариант 2, спрятанный в одну обёртку; и параллельно нести спред (п. 4) в апстрим — без него любая
  библиотека компонентов на hypertext обязана либо перечислять все атрибуты, либо уходить в dangerously_*.
