Также, как были добавлены ранее компоненты Badge, Button и Input, необходимо добавить компонет Callout с базовой начальной функциональностью и настройками стилей. Ориентироваться стоит на реализацию Callout в `tmp/webawesome`. Для визуального тестирования работы необходим также новый раздел Callout с примерами использования в examples/client.

---

● Готово. Компонент Callout реализован по образцу Badge/Button/Input с ориентацией на tmp/webawesome. Все проверки CI пройдены: cargo +nightly fmt --check, cargo clippy --all-targets -- -D warnings, cargo test
  (30 тестов, из них 6 новых для Callout), а WASM-клиент собран через cargo make client.
  
  Что сделано:
  
  Библиотека (crates/lib/src/components/callout.rs) — структура Callout<I, R> по стандартному рецепту (Props, #[const_str(CLASS = CALLOUT)], CommonAttrs, variant/appearance). Рендерит <div class="callout …"> с
  опциональным контейнером callout-icon и обёрткой callout-message — зеркально частям icon/message из wa-callout. Иконка передаётся пропом: <Callout icon=(…)>…</Callout>, как слот icon в Web Awesome.
  
  Доработка макроса (crates/macros/src/derive.rs) — #[prop(convert)] раньше работал только с одним generic-параметром: сгенерированный метод передекларировал бы все параметры структуры и конфликтовал с
  impl-блоком. Теперь метод объявляет только заменяемый параметр, что позволило Callout иметь два независимых слота (icon и children). Для существующих компонентов генерируемый код не изменился.
  
  Отличие от других компонентов: у Callout ручной impl Default — вариант Brand и appearance FilledOutlined (у Badge/Button берутся Neutral/Accent из enum-дефолтов). Так callout по умолчанию выглядит как в Web
  Awesome — тихая заливка с рамкой, а не сплошной тёмный блок; в callout.ts это задано явно (variant = 'brand', базовый CSS соответствует filled-outlined).
  
  Стили (webassets/style/components/callout.css) — порт callout.styles.ts: flex-контейнер, модификаторы plain/outlined/filled/filled-outlined/accent, стили иконки и сообщения через @scope (.callout) как в
  badge.css. Подключён в index.css.
  
  Галерея (examples/client/src/components/callout.rs) — раздел с примерами по мотивам доки Web Awesome: дефолтный callout, Variants (5 вариантов с тематическими иконками), Appearance, Size (через size-*
  классы), Without an Icon. Добавлены маршрут "callout" в main_section и пункт «Callout» в меню (lib.rs), а в fontawesome.rs — шесть новых иконок (circle-info, circle-check, circle-exclamation,
  triangle-exclamation, gear).
  
  Проект не запускал, как вы просили. Замечу, что по конвенции репозитория выполненные issue переносятся в issues/done/ (коммит «feat: done issue») — файл issues/callout component.md не трогал, оставляю это на
  ваш коммит.

---

Измени компонент Callout таким образом, чтобы если установлен специальный флаг через props, компонент ожидал ручные нстройки разметки своего тела, переданные через children и сам не вставлял в тело никаких дополнительних div и прочих элементов.

---

● Готово. В Callout добавлен флаг raw_body: bool: когда он установлен, компонент рендерит children как есть внутри внешнего <div class="callout …"> — без контейнеров callout-icon/callout-message и любых других
  добавочных элементов; проп icon в этом режиме игнорируется (это отмечено в doc-комментарии поля).

  Что изменилось:
  
  - crates/lib/src/components/callout.rs — новое поле raw_body (по умолчанию false, обычное поведение не изменилось), ветвление в render_to через @if self.raw_body { (self.children) } @else { … }.
  - crates/lib/src/tests/callout.rs — новый тест raw_body: builder-вариант и rsx-вариант (<Callout raw_body=true><span>…</span></Callout> даёт <div class="callout brand filled-outlined"><span>…</span></div>),
  проверка, что icon игнорируется, и пустое тело без обёрток.
  
  Проверки: cargo +nightly fmt --check, cargo clippy --all-targets -- -D warnings и cargo test — всё чисто, 31 тест прошёл (7 по Callout).
  
  Использование:
  
  <Callout variant=Warning raw_body=true>
      <div class="my-own-layout">"Полностью ручная разметка тела"</div>
  </Callout>

---

Мне хотелось бы более короткое имя для такого prop, не составное. Может быть просто raw? Или manual? Посоветуй лучшие варианты.

---

 ☐ Имя пропа 

Как назвать флаг, отключающий обёртки callout-icon/callout-message?

❯ 1. bare (Recommended)
     «Голое» тело без обёрток. Точно передаёт смысл — никаких лишних элементов — и не пересекается с другими терминами: в hypertext «raw» уже означает неэкранированный HTML (Raw, dangerously_*), а plain занят 
     Appearance::Plain.
  2. raw
     Коротко и привычно, но в экосистеме hypertext «raw» ассоциируется с рендером без экранирования — флаг могут прочитать как «children вставляются без escape», что неверно.
  3. manual
     Ясно передаёт «ручную разметку тела», без конфликтов. Чуть длиннее и менее употребимо как имя булева пропа (звучит как режим целого компонента, а не тела).
  4. Type something.                                                                                                                                                                             
──────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
  5. Chat about this

---

1

---

● Переименовываю raw_body → bare: 

● Переименовал в bare — флаг теперь читается как <Callout bare=true>…</Callout>. Все проверки прошли: fmt, clippy, 31 тест.
  
  Почему именно bare, а не raw или manual:
  
  - raw в экосистеме hypertext уже нагружен другим смыслом — Raw/dangerously_create означают вставку HTML без экранирования. Флаг raw=true легко прочитать как «children не экранируются», что неверно и
  потенциально настораживает при ревью.
  - manual точен по смыслу, но как булев проп звучит как режим всего компонента, а не тела, и напрашивается на составное manual_body — от чего вы как раз хотели уйти.
  - bare — короткое, однословное, точно передаёт «голое тело без обёрток» и ни с чем в кодовой базе не конфликтует (plain, кстати, тоже не подошёл бы — занят Appearance::Plain).
  
  Изменения только в двух файлах: поле, Default и render_to в crates/lib/src/components/callout.rs, тест bare в crates/lib/src/tests/callout.rs.

---

Не лучше ли сделать так, чтобы icon всё равно добавлялся, если явно указан в prop, даже если bare установлен в true?

---

● Согласен — так лучше. Явно переданный проп, который молча игнорируется, — это неожиданное поведение; а bare логичнее трактовать как «никаких неявных обёрток», а не «игнорируй другие пропы». К тому же
  комбинация «стандартная иконка + полностью ручное тело» практически полезна. Реализую:

● Сделано. Теперь bare отключает только неявную обёртку callout-message, а явно заданная иконка рендерится всегда:

  - <Callout bare=true>…</Callout> → <div class="callout …">…children как есть…</div>
  - <Callout bare=true icon=(…)>…</Callout> → <div class="callout …"><div class="callout-icon">…</div>…children как есть…</div>

  В render_to блок иконки вынесен из ветвления — он общий для обоих режимов, ветвится только обёртка сообщения. Doc-комментарий поля обновлён: «icon container is still emitted when the icon prop is set
  explicitly». Тест bare теперь проверяет, что с bare=true icon="*" контейнер иконки присутствует, а обёртки сообщения нет.

  Все проверки прошли: fmt, clippy (-D warnings), 31 тест.

---

Добавь в examples/client для Callout ещё один пример, в котором используется bare prop и внутренняя разметка компонента задаётся явно вручную (включая иконку).

---

● Добавил в галерею раздел Bare Body (между «Size» и «Without an Icon» в examples/client/src/components/callout.rs). В примере тело компонента задаётся полностью вручную через children с флагом bare, включая
  иконку:

  <Callout variant=Success bare=true>
      <div class=CALLOUT_ICON>
          (fontawesome::icon("circle-check"))
      </div>
      <div class=CALLOUT_MESSAGE>
          <strong>"All checks have passed"</strong><br/>
          "This body markup is provided entirely by hand via children."
      </div>
  </Callout>

  Разметка использует константы CALLOUT_ICON/CALLOUT_MESSAGE из wingy_hypertext::class (по конвенции репозитория — без строковых литералов), поэтому визуально пример совпадает со стандартным рендером, но
  демонстрирует ручную сборку тела. Раздел оформлен как остальные: превью с resize, исходник с кнопкой копирования и пояснительный текст про bare.

  Проверки пройдены: fmt, clippy (-D warnings), 31 тест, WASM-клиент собран (cargo make client).

---

Провёл небольшие ручные правки

---

Для примера Callout с кнопкой закрытия (with close button) в examples/client добавь описание к примеру и укажи необходимость задания обработчика нажатия на кнопку закрытия с помощью готового вызова wingy_hypertext_web::components::callout::listen_close_callout или вручную.

---

● Посмотрю текущее состояние примера с кнопкой закрытия в галерее и модуль wingy_hypertext_web::components::callout:
● Иконка на месте. Добавляю описание к примеру:
● Добавил описание к разделу «With close button» в examples/client/src/components/callout.rs. Текст в пустом абзаце теперь объясняет:

  - у Callout нет встроенного поведения закрытия — кнопка с классом close размещается в контенте как обычный Button;
  - клик нужно обрабатывать явно: либо один раз при старте вызвать готовый wingy_hypertext_web::components::callout::listen_close_callout (с примечанием, что именно так делает эта страница в init()) — он ловит
  клики по элементу close и удаляет ближайший callout из DOM;
  - либо зарегистрировать собственный обработчик вручную.

  Проверки пройдены: fmt, clippy (-D warnings), 31 тест, WASM-клиент пересобран (cargo make client).

---

В примере Callout с кнопкой закрытия (with close button) в examples/client при наведении на xmark icon button в danger callout бэкграунд кнопки закрытия не меняется. Почему это происходит? И как сделать так, чтобы бэкграунд немного менялся для того, чтобы пользователь видел, что курсор уже над кнопкой (как меняется в примере plain icon button в разделе button examples/client)?

---

● Разберусь: у plain-кнопки hover-фон — это --wa-color-fill-quiet, а фон самого callout (filled-outlined) — та же переменная. Проверю правило hover в button.css и переменные вариантов:
● Диагноз подтверждён. Добавляю правило в callout.css, чтобы hover plain-кнопки внутри callout отличался от его фона:
● Почему фон «не меняется». На самом деле hover-правило срабатывает, просто новый фон совпадает со старым. Цепочка такая:

  1. У plain-кнопки hover-фон — var(--wa-color-fill-quiet) (правило .button.plain:hover в button.css).
  2. Класс варианта (danger на кнопке и на самом callout) через variants.css задаёт --wa-color-fill-quiet: var(--wa-color-danger-fill-quiet).
  3. Callout в примере — danger с дефолтным appearance filled-outlined, то есть его собственный фон — тот же var(--wa-color-fill-quiet).

  Итог: при наведении кнопка закрашивается ровно тем же danger-fill-quiet, на котором и так лежит, — визуально ничего не происходит. В разделе Button тот же plain icon button стоит на белой поверхности
  страницы, поэтому там заливка fill-quiet видна. Это, кстати, не наше расхождение с Web Awesome — upstream-кнопка ведёт себя так же; просто паттерн «кнопка внутри callout» там не встречается.

  Исправление. В webassets/style/components/callout.css добавил правило: plain-кнопка внутри callout при hover домешивает к fill-quiet токен --wa-color-mix-hover — так же, как это делают filled-кнопки, у
  которых hover-цвет обязан отличаться от их собственного фона:

  @media (hover: hover) {
    .callout .button.plain:not(.disabled):not(.loading):hover {
      background-color: color-mix(in oklab,
        var(--wa-color-fill-quiet, var(--wa-color-brand-fill-quiet)),
        var(--wa-color-mix-hover));
    }
  }

  Специфичность у правила выше, чем у базового hover в button.css, так что оно надёжно перекрывает его для любых plain-кнопок в любых callout (в plain/outlined callout с прозрачным фоном такой hover тоже
  остаётся видимым). Состояние :active не трогал — оно и так использует --wa-color-mix-active и отличается от фона. Статика пересобрана, правило в target/web на месте.

---

Доработал руками
