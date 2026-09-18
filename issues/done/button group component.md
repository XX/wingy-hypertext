Также, как были реализованы компоненты Badge, Button, Tag и другие, необходимо перенести компонент `ButtonGroup`, повторяя функционал `wa-button-group` из WebAwesome: `tmp/webawesome/packages/webawesome/src/components/button-group`.

Компонент (`crates/lib/src/component/button_group.rs`) следует общему рецепту: `Default, AsRef, AsMut, Props` + `#[const_str(CLASS = BUTTON_GROUP)]` + `#[props(builder)]`, `CommonAttrs` через `#[as_ref] #[as_mut]` и `children`. Пропы — как у оригинала:

- `label` — текст для `aria-label`, на экране не показывается, читается скринридером;
- `orientation` — переиспользовать `crate::orientation::Orientation` (`Horizontal` по умолчанию, `Vertical`), класс ориентации добавляется к базовому.

Разметка — один контейнер с `role="group"`, `aria-label` и `aria-orientation`, внутри которого лежат `Button`. Константу `BUTTON_GROUP` добавить в `crates/lib/src/class.rs`.

Стили (`webassets/style/components/button_group.css`) — порт `button-group.styles.ts`: `display: inline-flex`, `isolation: isolate`, направление по ориентации, подъём `z-index` у кнопки под курсором и у кнопки в фокусе или отмеченной. Самое важное — группа задаёт дочерним кнопкам уже существующие кастомные свойства: `--_button-horizontal-indent` / `--_button-vertical-indent` (и их `-outlined` варианты) для схлопывания соседних рамок и `--_button-*-radius` для скругления только у крайних кнопок. Наш `webassets/style/components/button.css` эти свойства уже читает, так что правки в нём не потребуются. Вместо `::slotted` у нас обычные дочерние селекторы.

Скорее всего, код в `crates/web` не понадобится: в WA скрипт навешивает на кнопки классы `button-hover` и `button-focus` только потому, что из shadow DOM не достать состояние слотированных элементов. У нас разметка плоская, те же правила выражаются селекторами `:hover`, `:focus-visible` и `[aria-checked='true']`. Если в процессе окажется, что чего-то не хватает, поведение добавить в `crates/web` обычным делегированным слушателем.

Тесты разметки — `crates/lib/src/tests/button_group.rs`, по образцу остальных компонентов.

Для визуального тестирования нужен раздел Button Group в `examples/client` (маршрут, пункт меню в разделе Components) с примерами из доки WA (`tmp/webawesome/packages/webawesome/docs/docs/components/button-group.md`): базовый пример, Orientation, Pill, Dropdowns, Split Buttons, Tooltips, Toolbars. Пример Native Buttons тоже уместен: он показывает, что группа работает с обычными `<button class="button">`. Отдельно проверить группу с `Dropdown` внутри: триггер в группе не должен ломать позиционирование меню.

Правила подъёма `z-index` для отмеченной кнопки (`[aria-checked='true']`, `[checked]`) стоит перенести как есть, хотя сейчас их некому использовать: в актуальной версии WA компонента `wa-radio-button` уже нет (упоминание осталось только в селекторе внутри `button-group.ts`), а сегментированный переключатель делается отдельно — через `wa-radio` с `appearance="button"` внутри `wa-radio-group`, без участия группы кнопок.

---

● Задача issues/button group component.md выполнена.
  
  Компонент (crates/lib/src/component/button_group.rs): пропы label (идёт в aria-label) и orientation, разметка — <div class="button-group [vertical]" role="group" aria-label aria-orientation>. Константа
  BUTTON_GROUP добавлена в class.rs.
  
  Стили (webassets/style/components/button_group.css, подключены в index.css): порт button-group.styles.ts. Группа выставляет дочерним элементам --_button-*-indent и --_button-*-radius, которые уже читают и
  button.css, и common/native.css. Правки в них не понадобились. Вместо ::slotted — обычные дочерние селекторы, :first-child и :last-child.
  
  Кода в crates/web не потребовалось, как и предполагалось в задаче: классы button-hover и button-focus из WA заменены селекторами :hover и :focus. Селекторы подъёма z-index я нацелил на сами кнопки, а не на
  прямых детей группы: кнопка может быть завёрнута в display: contents (триггер Dropdown), у такого элемента нет бокса и z-index к нему неприменим.
  
  Тесты (crates/lib/src/tests/button_group.rs): 4 теста — разметка по умолчанию, label с ориентацией, общие атрибуты, группа с кнопками. Всего в проекте 104 теста, все проходят. fmt и clippy (-D warnings, 
  включая wasm-клиент) чистые.
  
  Галерея: раздел Button Group (маршрут /button-group, пункт меню после Button) со всеми примерами из доки WA — базовый, Orientation, Pill, Dropdowns, Split Buttons, Tooltips, Toolbars, Native Buttons.
  
  Проверка в Firefox:
  - горизонтальная группа: радиусы 6/0/0/6, 0/0/0/0, 0/6/6/0, отступы 0px, 1px, 1px;
  - вертикальная: 6/6/0/0, 0/0/0/0, 0/0/6/6, направление column;
  - триггер Dropdown внутри группы наследует раскладку через display: contents — радиусы 0/0/0/0, отступ 1px;
  - меню открывается под триггером (bottom-start, выровнено по левому краю) и рисуется поверх соседнего содержимого;
  - в панели инструментов тултип показывается только у наведённой кнопки;
  - группа нативных <button class="wa-filled"> выглядит так же, как из компонентов.
  
  Одно отступление от задачи: вспомогательную функцию для кнопок панели сделать не вышло — #[renderable] не переносит ограничения с генерик-параметра, поэтому кнопки в примере Toolbars расписаны явно.

---

