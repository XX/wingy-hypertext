Также, как ранее были реализованы компоненты `Dropdown`, `Select`, `Drawer` и другие, необходимо реализовать компонент `Tooltip`, повторяющий функционал компонента `wa-tooltip` из WebAwesome (`tmp/webawesome/packages/webawesome/src/components/tooltip`). Логику вместо JS следует реализовать на Rust в `crates/web`. Для визуального тестирования необходим также новый раздел Tooltip в `examples/client` с примерами использования, повторяющими примеры из документации `wa-tooltip`.

Позиционирование строить на уже реализованном хелпере `Popup` (`crates/lib/src/helper/popup.rs` + `crates/web/src/helper/popup.rs`) — в оригинале `wa-tooltip` также построен поверх `wa-popup`. Хелпер уже умеет всё, что нужно тултипу: `flip`, `shift`, стрелка (`arrow` + `PopupBody`) и `hover_bridge` (заполнение зазора между якорем и попапом, чтобы указатель не «терял» их при движении). Отдельно писать CSS-позиционирование не нужно.

Свойства `Tooltip` (по мотивам оригинала):

- `for` — `id` элемента-якоря, который тултип описывает (переиспользовать `anchor_id` у `Popup`); тултип рендерится соседом якоря, а не оборачивает его;
- `placement` (top/top-start/top-end/right/… — переиспользовать `PopupPlacement`, по умолчанию `Top`);
- `open`, `disabled` (у выключенного тултипа показ не происходит вовсе);
- `distance` (по умолчанию 8) и `skidding` (по умолчанию 0);
- `show_delay` (по умолчанию 150 мс) и `hide_delay` (по умолчанию 0);
- `trigger` — набор способов активации: `hover`, `focus`, `click`, `manual` (в оригинале строка со списком через пробел; в Rust уместнее набор флагов/битовую маску либо срез значений, по умолчанию `hover focus`);
- `without_arrow` — убрать стрелку;
- слот с содержимым (только текст и презентационная разметка — интерактивный контент внутрь не кладём, как и предупреждает документация).

Поведение на Rust в `crates/web`:

- показ/скрытие с анимациями (`show-with-scale`/`hide-with-scale` в оригинале — использовать принятый в проекте механизм анимаций, как у `Dropdown`), авто-позиционирование через `Popup` и перепозиционирование при скролле/ресайзе;
- активация по `hover` (с учётом `show_delay`/`hide_delay` и перехода указателя на сам тултип или обратно на якорь), по `focus`/`blur`, по `click` (повторный клик закрывает), режим `manual` — только программно/через атрибут;
- закрытие по `Escape`, при этом закрывается только верхний открытый dismissible-элемент (в оригинале — `dismissible-stack`; посмотреть, как это решено у `Dropdown`/`Drawer`, и не закрывать тултип «сквозь» открытый поверх него оверлей);
- корректная работа нескольких тултипов на странице и при перерисовке фрагментов (htmx `reinit`);
- доступность: `id` тултипа добавляется в `aria-labelledby` якоря (WA сознательно выбрал `aria-labelledby`, а не `aria-describedby`, — см. комментарий в `tooltip.ts`), при удалении/переанкоривании — убирается; тултипу проставить `role="tooltip"`;
- отменяемые всплывающие события в принятой в проекте нотации `wg-*`: `wg-show`/`wg-hide` и `wg-after-show`/`wg-after-hide` (константы уже есть в `crates/web/src/util/event.rs`).

Стили портировать из `tooltip.styles.ts` в `webassets/style/components/tooltip.css` (по аналогии с остальными компонентами): `--max-width` (по умолчанию `30ch`) для переноса содержимого, токены `--wa-tooltip-*` (фон, цвет, размер шрифта, скругление, рамка, размер стрелки), `transform-origin` по разрешённому placement, `z-index` попапа, запрет выделения текста. Классы добавить константами в `crates/lib/src/class.rs`. Как и для прочих компонентов, нужны тесты рендеринга в `crates/lib/src/tests/`.

Примеры в галерее (`examples/client/src/component/tooltip.rs`, маршрут `/tooltip` и пункт меню в `examples/client/src/lib.rs`), повторяющие документацию:

- базовый (`for` + кнопка);
- Placement — сетка кнопок со всеми 12 вариантами размещения;
- Triggers — `trigger="click"`;
- HTML in Tooltips — форматированное содержимое с переносами строк;
- Customizing — `--max-width` и `without_arrow`;
- Showing & Hiding Manually — `trigger="manual"` и переключение `open` (в галерее — через обработчик на Rust).

---

## Результат

Реализовано:

- **`crates/lib/src/component/tooltip.rs`** — компонент `Tooltip`: `placement` (переиспользуется
  `PopupPlacement`, по умолчанию `Top`), `anchor_id` (id якоря), `open`, `disabled`, `distance`
  (по умолчанию 8), `skidding`, `show_delay`, `hide_delay`, `trigger`, `without_arrow` и слот
  содержимого. Тултип рендерится хостом `div.tooltip[role=tooltip]` с конфигурацией в `data-*`,
  внутри — `Popup` (`flip`, `shift` с `shift_padding=8`, `arrow` по `!without_arrow`, `hover_bridge`),
  в теле попапа — `div.tooltip-body` с содержимым. Триггеры типизированы: перечисление
  `TooltipTrigger` (`Hover`/`Focus`/`Click`/`Manual`) и множество `TooltipTriggers`, собираемое
  через `|` (`Hover | Click`), по умолчанию `Hover | Focus`; в разметку выводится тем же
  списком через пробел, что и в WA. Классы `TOOLTIP`/`TOOLTIP_BODY` добавлены в `crates/lib/src/class.rs`.
- **`webassets/style/components/tooltip.css`** — порт `tooltip.styles.ts`: `--max-width` (30ch),
  токены `--wa-tooltip-*` (фон, цвет, размер шрифта, скругление, рамка, размер стрелки), рамки
  стрелки, `z-index` попапа, `transform-origin` по разрешённому placement, запрет выделения текста;
  подключён в `webassets/style/index.css`.
- **`crates/web/src/component/tooltip.rs`** — логика на Rust/WASM: показ/скрытие с анимациями
  (`show-with-scale`/`hide-with-scale` из `helpers/popup.css`), триггеры `hover` (с `show_delay`/
  `hide_delay` и учётом перехода указателя на сам тултип и обратно на якорь), `focus` (через
  всплывающие `focusin`/`focusout`), `click` (повторный клик закрывает) и `manual`, дизмисс по
  `Escape`, генерация недостающего `id` и подстановка его в `aria-labelledby` якоря, сброс и
  повторная инициализация в `init_tooltips`. События: `wg-show`/`wg-hide` (отменяемые,
  с `detail.source`) и `wg-after-show`/`wg-after-hide`. Отложенные показ/скрытие хранят handle
  таймера в `data-timer` на самом элементе — состояние, как и у остальных компонентов, живёт в DOM.
- **`examples/client/src/component/tooltip.rs`** — раздел галереи со всеми примерами из документации
  `wa-tooltip` (overview, Placement со всеми 12 размещениями, Triggers, HTML in Tooltips,
  Customizing с `--max-width` и `without_arrow`, Showing & Hiding Manually с переключением из Rust).
  Маршрут `/tooltip`, пункт меню и вызовы `init_tooltips`/`listen_tooltips` добавлены в
  `examples/client/src/lib.rs`, стили демо-сетки размещений — в `examples/client/webassets/main.css`.
- Тесты рендеринга: **`crates/lib/src/tests/tooltip.rs`** (9 тестов).

Отклонения от оригинала:

- Вместо `for` — свойство `anchor_id` (`for` — зарезервированное слово в Rust); имя совпадает
  с `anchor_id` у хелпера `Popup`, куда оно и передаётся.
- `trigger` — не строка со списком через пробел, а типизированное множество `TooltipTriggers`.
- Порядок дизмисса по `Escape`: вместо стека dismissible-элементов WA тултип уступает клавишу,
  если на странице открыт `.dropdown.open` или `.drawer.open` — они всегда открываются поверх
  тултипа, а их состояние уже отражено в разметке.
- `disabled` только запрещает показ (как и в WA), но не закрывает уже открытый тултип: класс
  меняется потребителем, наблюдателя за атрибутами у нас нет.
- Хосту тултипа проставлен `role="tooltip"` (в `wa-tooltip` роли нет — там тултип и так связан
  с якорем через `aria-labelledby`).

Проверено: `cargo +nightly fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test` (96 passed, из них 9 новых), `cargo make client` — всё зелёное. Поведение проверено
в браузере (headless Chromium через CDP, 46 проверок): структура и `aria-labelledby` у всех 18
тултипов страницы, показ по наведению строго после `show_delay` и скрытие после `hide_delay`,
позиционирование (8 px над якорем, центрирование, стрелка), все 12 размещений разрешаются в
запрошенные, переход указателя на сам тултип и возврат на якорь не закрывают его, события
`wg-show`/`wg-after-show`/`wg-hide`/`wg-after-hide` и отмена `wg-show`, триггеры `focus`, `click`
и `manual`, дизмисс по `Escape` (и его уступка открытому поверх оверлею), `--max-width`,
`without_arrow`, разметка в содержимом, `disabled`, переанкоривание при скролле и ресайзе,
переинициализация после htmx-навигации без дублей в `aria-labelledby`.

---

● Задача issues/tooltip component.md выполнена.
  
  Что сделано
  
  Компонент — crates/lib/src/component/tooltip.rs
  Tooltip: placement (переиспользован PopupPlacement, по умолчанию Top), anchor_id, open, disabled, distance (8 по умолчанию), skidding, show_delay, hide_delay, trigger, without_arrow + слот содержимого.
  Рендерится хостом div.tooltip[role=tooltip] с конфигурацией в data-*, внутри — Popup (flip, shift, arrow, hover_bridge), в теле попапа div.tooltip-body. Триггеры типизированы: TooltipTrigger
  (Hover/Focus/Click/Manual) + множество TooltipTriggers, собираемое через |.
  
  Стили — webassets/style/components/tooltip.css: порт tooltip.styles.ts (токены --wa-tooltip-*, --max-width: 30ch, рамки стрелки, transform-origin по разрешённому placement).
  
  Логика — crates/web/src/component/tooltip.rs: показ/скрытие с анимациями show-with-scale/hide-with-scale, триггеры hover (с задержками и учётом перехода указателя на сам тултип), focus (через всплывающие
  focusin/focusout), click, manual; Escape; генерация id и подстановка в aria-labelledby якоря; события wg-show/wg-hide/wg-after-*. Handle таймера живёт в data-timer — состояние в DOM, как у остальных
  компонентов.
  
  Галерея — examples/client/src/component/tooltip.rs со всеми примерами из документации wa-tooltip, маршрут /tooltip, пункт меню, init_tooltips/listen_tooltips.
  
  Тесты — crates/lib/src/tests/tooltip.rs (9 шт.).
  
  Отклонения от оригинала
  
  - for → anchor_id (for — ключевое слово в Rust; имя совпадает с Popup::anchor_id).
  - trigger — типизированное множество вместо строки со списком через пробел. 
  - Вместо стека dismissible-элементов WA тултип уступает Escape, если открыт .dropdown.open/.drawer.open.
  - disabled запрещает показ, но не закрывает уже открытый (наблюдателя за атрибутами нет).
  - Хосту проставлен role="tooltip" (в WA роли нет).
  
  Проверки
  
  cargo +nightly fmt --check, cargo clippy --all-targets -- -D warnings, cargo test (96 passed, из них 9 новых), cargo make client — зелёные. Плюс 46 браузерных проверок в headless Chromium через CDP: задержки
  показа/скрытия, позиционирование (8 px над якорем, центрирование, стрелка), все 12 размещений разрешаются в запрошенные, переход указателя на тултип и обратно, полный цикл wg-* событий и отмена wg-show,
  триггеры focus/click/manual, Escape и его уступка оверлею, --max-width, without_arrow, disabled, переанкоривание при скролле/ресайзе, переинициализация после htmx-навигации без дублей в aria-labelledby.
  
  Файл задачи оставил в issues/ — дописал в него раздел «Результат», переносить в done/ не стал.

---

