Также, как ранее были реализованы компоненты `Select`, `Drawer` и другие, необходимо реализовать компонент `Dropdown` вместе с компонентом элемента меню (`DropdownItem`), повторяя функционал компонентов `wa-dropdown` и `wa-dropdown-item` из WebAwesome (`tmp/webawesome`). Логику вместо JS следует реализовать на Rust в `crates/web`. Для визуального тестирования необходим также новый раздел Dropdown в `examples/client` с примерами использования, повторяющими примеры из документации `wa-dropdown` и `wa-dropdown-item`.

Позиционирование меню строить на уже реализованном хелпере `Popup` (`crates/lib/src/helpers/popup.rs` + `crates/web/src/helpers/popup.rs`) — в оригинале `wa-dropdown` также построен поверх `wa-popup`. Отдельно писать CSS-позиционирование, как это было сделано в первой версии `Select`, не нужно.

Свойства `Dropdown` (по мотивам оригинала): `open`, `size` (xs/s/m/l/xl), `placement` (top/top-start/top-end/bottom/bottom-start/bottom-end/left/… ), `distance`, `skidding`, слот триггера (кнопка или произвольный элемент) и слот с элементами меню.

Свойства `DropdownItem`: `variant` (default/danger — «destructive items»), `type` (normal/checkbox) с `checked`, `value`, `disabled`, слоты `icon`, `details` и `submenu` (вложенные подменю), а также выравнивание по соседям (`checkbox-adjacent`, `submenu-adjacent`). Дополнительно нужны разделители и метки групп внутри меню (в примерах доки — `wa-divider` и элементы с ролью подписи).

Поведение на Rust в `crates/web`:

- открытие/закрытие меню с анимациями, авто-позиционирование через `Popup`;
- клавиатурная навигация (стрелки, Home/End, Enter/Space, Escape), typeahead-поиск по первым буквам;
- открытие/закрытие подменю (наведение, стрелки влево/вправо, задержки), стек открытых подменю;
- закрытие по клику вне меню и по выбору элемента, корректная работа нескольких дропдаунов на странице;
- переключение `checked` у элементов типа `checkbox`;
- отменяемые всплывающие события в принятой в проекте нотации `wg-*` (по аналогии с `Drawer`): `wg-show`/`wg-hide`, `wg-after-show`/`wg-after-hide` и `wg-select` (с `detail`, содержащим выбранный элемент/его `value`). Сам компонент не выполняет прикладное действие — обработку решает потребитель.

Стили портировать из `dropdown.styles.ts` и `dropdown-item.styles.ts` в `webassets/style/components/dropdown.css` (по аналогии с остальными компонентами), классы добавить константами в `crates/lib/src/class.rs`. Как и для прочих компонентов, нужны тесты рендеринга в `crates/lib/src/tests/`.

---

## Результат

Реализовано:

- **`crates/lib/src/component/dropdown.rs`** — компоненты `Dropdown` и `DropdownItem`.
  `Dropdown`: `placement` (переиспользуется `PopupPlacement`, по умолчанию `BottomStart`), `open`, `distance`,
  `skidding`, `size` (`DropdownSize`: ExtraSmall…ExtraLarge → классы `size-*`), слот `trigger` (dyn-ребёнок) и
  дети-элементы меню. Меню оборачивается в `Popup` (`flip`, `shift` с `shift_padding=10`, `auto_size=Vertical`
  с `auto_size_padding=10`), триггер выступает якорем попапа.
  `DropdownItem`: `variant` (`Danger` — деструктивное действие), `checkbox` + `checked`, `disabled`, `value`,
  `label` (для typeahead), `checkbox_adjacent`/`submenu_adjacent`, слоты `icon`, `details`, `submenu`
  (три dyn-ребёнка через `DynRenderable`). Рендерится в `role="menuitem"`/`"menuitemcheckbox"` с полным набором
  aria-атрибутов. Классы добавлены в `crates/lib/src/class.rs`.
- **`webassets/style/components/dropdown.css`** — порт `dropdown.styles.ts` и `dropdown-item.styles.ts`
  (меню и подменю, transform-origin по разрешённому placement, анимации show/hide, заголовки групп и разделители,
  варианты/состояния элементов, выравнивание по чекбоксам и подменю), подключён в `webassets/style/index.css`.
- **`crates/web/src/component/dropdown.rs`** — логика на Rust/WASM: открытие/закрытие с анимациями,
  выбор элемента (с переключением `checked` у чекбоксов), подменю по наведению и с клавиатуры, клавиатурная
  навигация (стрелки, Home/End, Enter/Space, Escape, Tab) и typeahead, закрытие по клику вне меню и при
  открытии другого дропдауна, синхронизация выравнивания элементов и aria-атрибутов триггера в `init_dropdowns`.
  События: `wg-show`/`wg-hide` (отменяемые, с `detail.source`), `wg-after-show`/`wg-after-hide` и отменяемый
  `wg-select` (`detail.item`, `detail.value`) — отмена оставляет меню открытым. Константа `SELECT` добавлена
  в `crates/web/src/util/event.rs`.
- **`crates/web/src/helper/popup.rs`** — позиционирование вынесено в публичную `place(anchor, popup, config,
  size_host)` с публичными `PopupConfig::new(placement)`, `Side`, `Align`, `placement_str`; `reposition` теперь
  использует её. Подменю позиционируются этой же функцией (`right-start`, `skidding=-5`, flip, shift, auto-size)
  и переанкориваются при скролле/ресайзе.
- **`examples/client/src/component/dropdown.rs`** — раздел галереи со всеми примерами из документации
  `wa-dropdown`/`wa-dropdown-item` (overview, иконки, заголовки групп и разделители, details, чекбоксы,
  деструктивные элементы, подменю, disabled, placement, distance, offset, size, реакция на выбор).
  Маршрут `/dropdown` и пункт меню добавлены в `examples/client/src/lib.rs`.
- Тесты рендеринга: **`crates/lib/src/tests/dropdown.rs`** (10 тестов).

Отклонения от оригинала:

- Вместо `type="normal|checkbox"` — булево свойство `checkbox` (`type` — зарезервированное слово в Rust).
- Подменю закрывается при наведении на соседний элемент того же уровня, а не по «safe triangle» с таймаутом,
  как в WA: детерминированнее и без мерцания при диагональном движении курсора.
- `Escape` сначала сворачивает самое глубокое открытое подменю и только затем закрывает весь дропдаун
  (в WA `Escape` всегда закрывает дропдаун целиком).
- Элементы с подменю получают дополнительный `padding-inline-end`, чтобы длинная подпись не подтекала
  под абсолютно спозиционированный индикатор (в WA это возможно).

Доработка после обновления `iconic` (появились недостающие chevron'ы, `Cargo.lock` обновлён на `e32a642`):

- Индикатор подменю — `fontawesome_ext::regular::ChevronRight` вместо повёрнутого на -90° через CSS
  `ChevronDown`; поворот из `webassets/style/components/dropdown.css` убран, для RTL оставлено
  зеркалирование `transform: scaleX(-1)`, как в WA.
- Каретка триггеров в примерах галереи переведена на `fontawesome_ext::regular::ChevronDown` — тот же
  «тонкий» chevron, что использует `Select` для иконки раскрытия и `CodeExample` для кнопки кода.
- Обновлён тест `item_submenu`. Проверки прогнаны повторно: fmt, clippy, `cargo test` (86 passed),
  `cargo make client`, браузерные проверки 20/20; иконка рендерится без поворота (7×14 px, не перекрывает подпись).

Исправление подсветки первого элемента:

- Класс `.active` больше не даёт серый фон — он лишь помечает элемент, на который переходит фокус при
  клавиатурной навигации, а подсветка приходит от `:focus-visible` (как в `wa-dropdown`, где `active`
  управляет только `tabindex`). Раньше меню, открытое мышью, всегда показывало первый элемент выделенным,
  будто под курсором. Правки только в `webassets/style/components/dropdown.css`.
- Проверено в браузере: при открытии мышью фон первого элемента прозрачный и рамки нет
  (`focus-visible: false`, фокус при этом на элементе — стрелки работают сразу); после `ArrowDown`
  и при открытии с клавиатуры (`Enter` на триггере) элемент подсвечивается фоном и focus-ring.

Убран класс `dropdown-popup`:

- Попап — единственный прямой ребёнок хоста дропдауна, поэтому вместо отдельного класса-маркера
  используется `.dropdown > .popup` (в CSS) и `:scope > .popup` (в селекторах `crates/web`).
  Константа `DROPDOWN_POPUP` удалена из `crates/lib/src/class.rs`. В отличие от `Select`, где
  `select-popup` несёт собственные стили (размеры поля ввода), у дропдауна класс не нёс ничего.
- Обновлены тесты рендеринга и браузерные проверки (20/20 после изменения).

Проверено: `cargo +nightly fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`
(86 passed, из них 10 новых), `cargo make client` — всё зелёное. Дополнительно поведение проверено в браузере
(headless Chromium через puppeteer-core, 20 проверок): открытие по клику и позиционирование меню относительно
триггера, aria-состояния, навигация стрелками/Home/End, typeahead, закрытие по Escape с возвратом фокуса на
триггер, выбор с `wg-select` и значением элемента, закрытие по клику вне меню, чекбоксы с отменой события
(меню остаётся открытым), открытие подменю по наведению и его позиционирование, вход/выход в подменю по
стрелкам, выбор элемента подменю, `placement=RightStart`, `distance=30`, `skidding=30`, недоступность
`disabled`-элементов.

---

● Задача issues/dropdown component.md выполнена.

  Что сделано
  
  Компоненты — crates/lib/src/component/dropdown.rs
  - Dropdown: placement (переиспользован PopupPlacement, по умолчанию BottomStart), open, distance, skidding, size, слот trigger (dyn-ребёнок) и дети-элементы меню. Меню обёрнуто в Popup (flip, shift,
  auto_size=Vertical), триггер — якорь попапа; своего CSS-позиционирования нет.
  - DropdownItem: variant=Danger, checkbox + checked, disabled, value, label, checkbox_adjacent/submenu_adjacent, слоты icon, details, submenu; role="menuitem"/"menuitemcheckbox" с aria-атрибутами.
  
  Стили — webassets/style/components/dropdown.css: порт dropdown.styles.ts и dropdown-item.styles.ts, включая transform-origin по разрешённому placement, анимации, заголовки групп, разделители и выравнивание по
  соседям.
  
  Логика — crates/web/src/component/dropdown.rs: открытие/закрытие с анимациями, выбор (с переключением чекбоксов), подменю по наведению и с клавиатуры, стрелки/Home/End/Enter/Space/Escape/Tab, typeahead,
  закрытие по клику вне и при открытии другого дропдауна, синхронизация выравнивания и aria в init_dropdowns. События wg-show/wg-hide/wg-after-* и отменяемый wg-select (detail.item, detail.value).
  
  Хелпер Popup — позиционирование вынесено в публичную place(anchor, popup, config, size_host); reposition использует её, подменю позиционируются ей же (right-start, flip/shift/auto-size) и переанкориваются при
  скролле/ресайзе.
  
  Галерея — examples/client/src/component/dropdown.rs: все примеры из доки WA, маршрут /dropdown и пункт меню.
  
  Тесты — crates/lib/src/tests/dropdown.rs (10 тестов).
  
  Отклонения от оригинала (описаны в issue)
  
  - checkbox: bool вместо type="normal|checkbox" (type — зарезервированное слово).
  - Подменю закрывается при наведении на соседний элемент уровня, а не по «safe triangle» с таймаутом — детерминированнее и без мерцания.
  - Escape сначала сворачивает глубочайшее подменю, затем закрывает дропдаун.
  - Индикатор подменю — ChevronDown, повёрнутый на -90° (в iconic нет ChevronRight).
  
  Проверки
  
  cargo +nightly fmt --check, cargo clippy --all-targets -- -D warnings, cargo test (86 passed, +10 новых), cargo make client — всё зелёное.
  
  Дополнительно прогнал поведение в реальном браузере (headless Chromium через локально поставленный в scratchpad puppeteer-core) — 20/20 проверок: открытие и позиционирование меню, aria-состояния, навигация и
  typeahead, Escape с возвратом фокуса, wg-select со значением, закрытие по внешнему клику, чекбоксы с отменой события, подменю (наведение, позиция, стрелки, выбор), placement=RightStart, distance=30,
  skidding=30, недоступность disabled-элементов. Скриншоты подтвердили вёрстку (выравнивание чекмарков, details, danger, вложенное подменю).
  
  Файл задачи оставил в issues/ с добавленными заметками о результате — в done/ не переносил.

---

 Доработай с учётом обновления в iconic - добавились недостающие chevron.

 ---

● Готово — CSS-обходной путь с поворотом иконки убран.
  
  Изменения
  
  - Cargo.lock — iconic обновлён на e32a642 (появились ChevronRight/ChevronLeft/ChevronUp в fontawesome::solid и fontawesome_ext::regular, а также Bars).
  - crates/lib/src/component/dropdown.rs — индикатор подменю теперь fontawesome_ext::regular::ChevronRight (тот же «тонкий» набор, что использует Select для иконки раскрытия и CodeExample для кнопки кода).
  - webassets/style/components/dropdown.css — убран rotate: -90deg у .submenu-icon; для RTL оставлено зеркалирование transform: scaleX(-1), как в оригинальном wa-dropdown-item.
  - examples/client/src/component/dropdown.rs — каретка триггеров в примерах переведена с fontawesome::solid::ChevronDown на fontawesome_ext::regular::ChevronDown для визуальной согласованности с Select.
  - crates/lib/src/tests/dropdown.rs — тест item_submenu ждёт новую иконку.
  - Заметки в issues/dropdown component.md обновлены: пункт об отсутствии ChevronRight перенесён в раздел о доработке.
  
  Остальные повороты в CSS не трогал — в select.css и code_example.css это легитимная анимация раскрытия (chevron-down поворачивается на 180°), а не подмена иконки.
  
  Проверки
  
  cargo +nightly fmt --check, cargo clippy --all-targets -- -D warnings, cargo test (86 passed), cargo make client — зелёные. Браузерные проверки прогнаны повторно: 20/20. Иконка подменю рендерится без
  трансформаций (viewBox 0 0 256 512, 7×14 px, полностью внутри элемента, 23 px зазора до подписи), скриншот подтверждает корректные chevron'ы у «Export»/«Options» и у каретки триггеров.

---

Почему-то первый элемент меню всегда выделен серым, как будто над ним курсор.

---

● Исправлено — правка только в webassets/style/components/dropdown.css.

  Причина. Я стилизовал .dropdown-item.active тем же серым фоном, что hover и focus-visible. А .active навешивается на первый элемент сразу при открытии меню (как и в wa-dropdown), чтобы стрелки работали без
  дополнительного нажатия. В оригинале же active управляет только tabindex, а видимая подсветка приходит от :focus-visible, который при открытии мышью не срабатывает.

  Что сделал. Убрал .active из трёх правил (базовый фон, danger-вариант, цвет details) — подсветка теперь только на :hover, :focus-visible и .submenu-open.

  Проверил в браузере:

  ┌───────────────────────────────────────────┬──────────────────────────────────────────────────────────────────────────┐
  │                 Сценарий                  │                              Первый элемент                              │
  ├───────────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ Открытие мышью                            │ фон прозрачный, рамки нет, но фокус на элементе — стрелки работают сразу │
  ├───────────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ После ArrowDown                           │ подсвечен фоном + focus-ring (переехал на второй элемент)                │
  ├───────────────────────────────────────────┼──────────────────────────────────────────────────────────────────────────┤
  │ Открытие с клавиатуры (Enter на триггере) │ подсвечен сразу                                                          │
  └───────────────────────────────────────────┴──────────────────────────────────────────────────────────────────────────┘

  Скриншоты подтверждают: меню, открытое мышью, без выделения; при клавиатурной навигации выделение на месте. Прогнал повторно интерактивный набор (20/20), fmt, clippy, cargo test (86 passed). Заметку о причине
  и исправлении добавил в issue.

---

Зачем понадобился класс dropdown-popup, разве нельзя обойтись просто popup, который вложен в элемент с dropdown?

---

● Хороший вопрос — класс действительно лишний. Проверю, где он используется, и уберу.
● Незачем — вы правы, класс был лишним, убрал его.

  Почему он вообще появился: я скопировал приём у Select, где select-popup действительно нужен — там на этот класс навешаны собственные стили (flex: 1 1 auto; width: 100%; position: relative), делающие обёртку
  попапа коробкой размером с поле ввода. У дропдауна же класс не нёс ни одного стиля и служил только «адресом» в селекторах. А попап — единственный прямой ребёнок хоста, так что .dropdown > .popup и :scope > 
  .popup адресуют его однозначно (вложенные дропдауны отсекаются комбинатором >, а подменю — вообще не .popup, они позиционируются через place).

  Что изменилось:
  - crates/lib/src/component/dropdown.rs — убран .class(DROPDOWN_POPUP); разметка стала <div class="dropdown"><div class="popup" …>.
  - crates/lib/src/class.rs — константа DROPDOWN_POPUP удалена.
  - crates/web/src/component/dropdown.rs — три селектора переведены на :scope > .popup ….
  - webassets/style/components/dropdown.css — восемь правил transform-origin стали .dropdown > .popup[data-current-placement…] .dropdown-menu.
  - Обновлены ожидания в тестах рендеринга.

  Проверки: fmt, clippy -D warnings, cargo test (86 passed), cargo make client, браузерный набор — 20/20; подсветка первого элемента по-прежнему не появляется при открытии мышью.

---

В классе dropdown-item-icon тоже нет необходимости?

---

● Нет, здесь класс нужен — случай другой, чем с dropdown-popup.

  dropdown-popup был чистым маркером: ни одного стиля, только «адрес» для селекторов, причём элемент и так однозначно адресуется структурно.

  dropdown-item-icon — это стилевой хук с реальными правилами:

  .dropdown-item .dropdown-item-icon:not(:empty) {
    display: flex;
    flex: 0 0 auto;
    align-items: center;
    margin-inline-end: 0.75em;
    font-size: var(--wa-font-size-smaller);
  }

  Обёртка задаёт иконке поведение флекс-элемента, отступ до подписи и уменьшенный кегль — иначе произвольное содержимое слота (SVG, эмодзи, любой renderable) поедет по базовой линии и слипнется с текстом. Это
  же и в оригинале: у wa-dropdown-item иконочный слот обёрнут в <span id="icon" part="icon"> со своими правилами, и в этом репозитории такой же приём у Callout (callout-icon: flex: 0 0 auto, align-items: 
  center, margin-inline-end) и у Select (check, expand-icon).

  Структурно его не заменить: :first-child у элемента с чекбоксом — это галочка, а не иконка, так что позиционные селекторы сломались бы. Общий утилитарный class=ICON тоже не подходит как замена — он навязывает
  фиксированную коробку 1.25em × 1em, а отступ до подписи всё равно пришлось бы задавать дропдаун-специфичным селектором.

  Одна деталь для сведения: :not(:empty) в правиле — не защита от отсутствия иконки (обёртка рендерится только когда слот задан), а страховка от icon=(rsx! {}), чтобы пустой спан не давал фантомный отступ
  0.75em. Если считаете это лишней перестраховкой — уберу.

  Из трёх подклассов элемента все несут нагрузку: dropdown-item-label — flex: 1 1 auto; min-width: 0 и плюс из него crates/web берёт текст для typeahead, dropdown-item-details — выравнивание вправо и
  приглушённый цвет.

---
