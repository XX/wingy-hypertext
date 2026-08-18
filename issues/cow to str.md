Во многих компонентах для хранения строковых значений используется тип Cox<'static, str>. Учитывая, что теперь большинство компонентов имеют лайфтайм 'a, возникает идея хранить эти строки в полях типа &'a str. Проверь, возможно ли это и если да и никаких особых неудобств это не вызывает, переделай поля на использование &str вместо Cow.

---

Возможно: все `Cow<'static, str>` в `crates/lib` заменены на `&'a str`, `std::borrow::Cow` из библиотеки ушёл полностью (остался только в `crates/web/src/helper/popup.rs`, где строка собирается в рантайме — это не поле компонента).

Что изменилось:

- Общие суб-структуры получили лайфтайм: `Link<'a>`, `Action<'a>`, `Htmx<'a>` (у `CommonAttrs<'a>` он уже был). Соответствующие трейты стали `LinkSetters<'a>`, `ActionSetters<'a>`, `HtmxSetters<'a>` — по образцу уже существовавшего `CommonAttributeSetters<'a>`, бланкетные impl'ы через `AsMut<...<'a>>` работают так же.
- `CommonAttrs`: `id: &'a str`, `classes/styles: Vec<&'a str>`, имя в `NamedAttribute<'a>` — тоже `&'a str` (`N: AsRef<str>` в hypertext это позволяет). Геттеры больше не требуют `&'a self`, `id()` возвращает `Option<&'a str>`, `join_not_empty` работает по `&[&str]` без `Deref`.
- Поля компонентов: `CopyButton::from`, `Select` (name/placeholder/label/hint), `SelectOption` (value/label), `Input` и `TextField` (name/value/placeholder/label/hint), `Switch`/`Toggle` (name/value/hint), `DropdownItem` (value/label), `Tooltip::anchor_id`, `Popup::anchor_id`, `Animation` (name/easing/keyframes), `CodeExampleSource::code_block_id`.
- У строковых полей убран `#[prop(into)]`: сеттер принимает `&'a str` напрямую. Это не сужает вызовы (в `Into<&str>` всё равно ничего, кроме `&str`, не конвертировалось), а `&String`/`&Cow` теперь проходят через deref-coercion, чего с `impl Into<...>` не было.
- Поля стали `Copy` — исчезли `.clone()` в `Input`/`Switch`/`Tooltip` при пробросе пропсов в дочерние компоненты.
- `CodeExampleSource`: производный `{id}-code-block` больше не `Cow::Owned`, а локальный `String` + `as_deref()`; `Head` так же отдаёт `href` в `Anchor` через локал.

Единственное неудобство: динамическую строку нельзя передать временным значением — `name=(&format!("..."))` теперь E0716 (temporary dropped while borrowed), нужна привязка `let`/`@let` перед разметкой. В самой библиотеке и галерее таких мест три (`head.rs`, `copy_button.rs`, `examples/client/src/helper/animation.rs`), везде решается одной строкой. То же касается пользователей библиотеки: владеющие строки теперь должны жить дольше компонента — для SSR это обычный паттерн (данные готовятся до рендера).

Проверки: `cargo make lint-test` (fmt + clippy `-D warnings` + 96 тестов) — чисто, тесты рендеринга не потребовали правок; `cargo make client` собирается. Размер `client_bg.wasm`: 531288 → 530782 байт (−506); заметного выигрыша по размеру нет — выигрыш в отсутствии аллокаций/веток `Cow` при рендере и в `Copy`-полях.

---

● Задача выполнена — переход возможен, Cow<'static, str> из crates/lib убран полностью.
                                                                                           
  Что сделано

  - Общие суб-структуры получили лайфтайм: Link<'a>, Action<'a>, Htmx<'a>; их трейты стали LinkSetters<'a>/ActionSetters<'a>/HtmxSetters<'a> — ровно по образцу уже существовавшего CommonAttributeSetters<'a>,
  бланкетные impl'ы через AsMut<…<'a>> работают без изменений.
  - CommonAttrs: id: &'a str, classes/styles: Vec<&'a str>, имя в NamedAttribute<'a> тоже &'a str (в hypertext там N: AsRef<str>). Геттеры больше не требуют &'a self, join_not_empty работает по &[&str].
  - Строковые поля компонентов: CopyButton::from, Select/SelectOption, Input/TextField, Switch/Toggle, DropdownItem, Tooltip::anchor_id, Popup::anchor_id, Animation, CodeExampleSource::code_block_id.
  - У этих полей убран #[prop(into)] — сеттер принимает &'a str напрямую; вызовы не сузились (в Into<&str> и раньше проходил только &str), зато заработал deref-coercion для &String.
  - Поля стали Copy — ушли .clone() при пробросе пропсов в Input/Switch/Tooltip; в CodeExampleSource производный {id}-code-block теперь локальный String + as_deref().
  
  Единственное неудобство: временное значение передать нельзя — name=(&format!("…")) даёт E0716, нужна привязка let/@let (проверил отдельным тестом). В библиотеке и галерее это три места, все поправлены одной
  строкой каждое.
  
  Проверки: cargo make lint-test — fmt, clippy -D warnings и 96 тестов чисто, тесты рендеринга правок не потребовали; cargo make client собирается, client_bg.wasm 531288 → 530782 байт (выигрыш по размеру
  символический, реальный плюс — отсутствие аллокаций/ветвлений Cow и Copy-поля).
  
  Отчёт дописан в issues/cow to str.md (файл оставил в issues/, не переносил в done/).

---

Всё же давай вернёмся к варианту с Cow, только лайфтайм у него пусть будет не 'static, а 'a.

---

● Понял — возвращаем Cow, но с лайфтаймом 'a. Откачу файлы библиотеки к исходному состоянию и применю замену 'static → 'a.
● Теперь общие структуры — attributes.rs, link.rs, action.rs, htmx.rs

Сделано: файлы `crates/lib` откатил к исходному состоянию и заменил `Cow<'static, str>` на `Cow<'a, str>`. То есть `#[prop(into)]` у строковых полей, `.clone()` при пробросе пропсов в дочерние компоненты и исходный код `CodeExampleSource`/`Head` (`Cow::Owned(format!(...))`) вернулись как были.

Из варианта с `&'a str` сохранились лайфтаймы у общих суб-структур: `Link<'a>`, `Action<'a>`, `Htmx<'a>` и трейты `LinkSetters<'a>`, `ActionSetters<'a>`, `HtmxSetters<'a>` — без этого `Cow<'a, str>` в них негде взять. `join_not_empty` принимает `&[Cow<'_, str>]` (функция намеренно не generic — одна копия на все компоненты).

Что это даёт по сравнению с обоими прежними вариантами — работает и то, и другое:

- заимствованная не-`'static` строка: `<Input name=(local.as_str()) />` (с `Cow<'static, str>` требовался `to_string()`);
- владеющая строка временным значением прямо в разметке: `<Input placeholder=(format!("field-{n}")) />` (с `&'a str` это E0716);
- `&String` тоже проходит — в std есть `impl<'a> From<&'a String> for Cow<'a, str>`.

Оба случая проверил разовым тестом рендеринга (`name=(local.as_str()) placeholder=(format!("field-{n}"))` → `name="borrowed-name" placeholder="field-3"`), после проверки тест удалил.

Единственная правка в галерее: в списках анимаций и easing (`examples/client/src/helper/animation.rs`) оставил `name.as_str()` вместо `name.clone()` — теперь это `Cow::Borrowed` без аллокации на каждую из ~130 опций.

Проверки: `cargo make lint-test` (fmt + clippy `-D warnings` + 96 тестов) — чисто, тесты рендеринга правок не потребовали; `cargo make client` собирается. Размер `client_bg.wasm`: 531288 → 531119 байт.

---

