Необходимо реализовать компонент `Popup` — полноценный порт компонента `wa-popup` из WebAwesome (`tmp/webawesome`) — как переиспользуемую основу для позиционирования всплывающих элементов. В оригинале `wa-popup` построен поверх JS-библиотеки `@floating-ui/dom`; логику позиционирования следует реализовать на Rust в `crates/web` (подмножество floating-ui): anchor, placement, offset, distance/skidding, flip, shift, sync width/height, auto-size с CSS-переменными `--auto-size-available-width/height`, arrow, autoUpdate (репозиционирование при скролле/ресайзе), `data-current-placement` для стилизации.

Контекст: на `wa-popup` в WebAwesome построены `wa-select`, `wa-dropdown`, `wa-tooltip`, `wa-popover`, `wa-color-picker`, `wa-time-input` — порт нужен до реализации этих компонентов. После реализации перевести существующий `Select` с упрощённого CSS-позиционирования листбокса (`.select-popup`, без flip и auto-size) на `Popup`.

Для визуального тестирования необходим также новый раздел Popup с примерами использования в `examples/client` (по мотивам доки `wa-popup`: placement, distance/skidding, flip, shift, sync, auto-size, arrow).

---

● Задача крупная. Изучу wa-popup целиком — компонент, стили и доку:
● План ясен. Проверю API wasm-dom для слушателей на window и создам задачи:
● Задача из issues/popup component.md выполнена. Все проверки прошли: fmt, clippy (-D warnings), 43 теста (5 новых для Popup), WASM-клиент собран.

  Компонент (crates/lib/src/components/popup.rs) — Popup<A, R> с двумя слотами: anchor (первый ребёнок хоста) и children (содержимое .popup-body); альтернативно внешний якорь по id через anchor_id. Пропы
  зеркалят wa-popup: placement (12 значений через enum Placement), active, distance/skidding, flip/flip_padding, shift/shift_padding, auto_size/auto_size_padding (enum AutoSize), sync (enum SyncSize),
  arrow/arrow_placement/arrow_padding, hover_bridge. Вся конфигурация сериализуется в data-* атрибуты хоста — это контракт между разметкой и движком.

  Движок (crates/web/src/components/popup.rs) — подмножество floating-ui на Rust/web-sys, всё в координатах вьюпорта (.popup-body — position: fixed):
  - порядок middleware как в оригинале: sync → flip → auto-size → offset/выравнивание → shift;
  - flip с best-fit-фолбэком (если не влезает ни с одной стороны — берётся сторона с большим местом);
  - auto-size выставляет --auto-size-available-width/height на хосте с повторным замером после применения (CSS-переменные ограничивают max-размер, как в WA);
  - стрелка: режимы anchor (центр якоря с клампом по padding), start/end/center, включая формулы с --arrow-padding-offset/--arrow-size-diagonal из WA;
  - hover-bridge: полигон между якорем и попапом через 8 CSS-переменных — прямой порт;
  - data-current-placement на хосте для стилизации (поворот стрелки, transform-origin);
  - autoUpdate: вместо floating-ui — capturing-слушатель scroll на window (ловит скролл любых вложенных контейнеров) + resize, репозиционирующие все .popup.active. API: set_popup_active, reposition,
  init_popups, listen_popups.

  CSS (popup.css) — порт popup.styles.ts: хост display: contents, стрелка-ромб с clip-path и поворотами по placement, hover-bridge, встроенные анимации show/hide/show-with-scale/hide-with-scale (keyframes
  переименованы в popup-*, чтобы не конфликтовать глобально).

  Select переведён на Popup: .select-popup стал popup-хостом (data-placement="bottom" data-flip data-shift data-sync="width" data-auto-size="vertical" data-auto-size-padding="10" — ровно та конфигурация, что у
  wa-select), листбокс обёрнут в .popup-body; открытие/закрытие идёт через set_popup_active, из select.css убрано absolute-позиционирование, max-height листбокса теперь var(--auto-size-available-height, …), а
  transform-origin следует data-current-placement. То есть селект получил недостающие flip и auto-size.

  Галерея — раздел Popup с секциями Placement, Distance & Skidding, Arrows, Sync, Flip, Shift, Auto-Size (попапы статически активны, привязка при скролле видна вживую), маршрут и пункт меню,
  listen_popups/init_popups подключены в init()/reinit().

  Из оригинала сознательно не переносилось: boundary: 'scroll' и кастомные boundary-элементы (граница — всегда вьюпорт), flip-fallback-placements/flip-fallback-strategy (только противоположная сторона +
  best-fit), виртуальные якоря и Popover API (не нужен без shadow DOM). Событие wa-reposition не диспатчится — добавлю, если появится потребитель.

---

Проверь корректность переноса компонента Popup и реализуй первый пример из документации wa-popup, в котором пользователем выбираются параметры popup с использованием компонентов Select, Input, Switch.

---

● Кодовая база продвинулась (добавлены Switch и Divider). Сначала ревизия порта Popup, затем интерактивный пример. Смотрю новые компоненты Switch (разметка, события) и текущее состояние движка:
● Switch — нативный чекбокс, события change всплывают. Перечитаю хвост движка popup для ревизии:
● Порт проверен (детали в итогах). Теперь интерактивный пример. Проверю API нового Divider для разметки демо:
● Всё ясно. Добавляю зависимости для DOM-логики демо в example-client и реализую интерактивный пример:
● Обе части выполнены. Все проверки прошли: fmt, clippy (-D warnings), 54 теста, WASM-клиент собран.
  
  1. Ревизия порта Popup. Прошёл движок против popup.ts построчно — исправлений не потребовалось, порт корректен для заявленного подмножества:
  
  - Порядок middleware functionally эквивалентен floating-ui (offset → sync → flip → shift → size): у нас sync применяется до замеров, distance учитывается в проверке «влезает ли» при flip, база+offset
  считаются после auto-size с повторным замером (переменные могут ужать попап), shift клампит поперечную ось последним.
  - Стрелка — формулы совпадают с WA: режимы start/end через calc(Npx - var(--arrow-padding-offset)), center через calc(50% - var(--arrow-size-diagonal)), anchor — центр якоря с клампом по padding; статическая
  сторона calc(var(--arrow-base-offset) - var(--arrow-size-diagonal)). 
  - Hover-bridge — сверил все четыре ветки расчёта углов полигона с WA (anchor выше/ниже/слева/справа) — маппинг координат идентичен.
  - Замеры через offsetWidth/Height (не искажаются scale-анимацией show/hide), data-current-placement совпадает с форматом WA (bottom, bottom-start, …), селекторы CSS-поворотов стрелки эквивалентны WA-шным.
  - Интеграция с Select: снятие hidden до активации (иначе замер нулевой), деактивация после hide-анимации с защитой от быстрого переоткрытия — порядок соответствует handleOpenChange в wa-select.
  
  2. Интерактивный первый пример из доки wa-popup — реализован на странице Popup в галерее, полностью на компонентах библиотеки:
  
  - Разметка: .popup-overview с demo-попапом (arrow=true, брендовый --arrow-color), <Divider/> и панелью контролов — Select «Placement» (все 12 значений), два Input типа Number («Distance», «Skidding»), два
  Switch («Active» вкл., «Arrow» выкл.), как в оригинале.
  - Связка (listen_popup_overview/init_popup_overview в examples/client/src/components/popup.rs, зарегистрированы в init()/reinit()): делегированные слушатели change/input на документе — изменение любого
  контрола внутри .popup-overview читает значения (у Select — из его value-input, у Switch — checked чекбокса) и применяет их: data-placement/data-distance/data-skidding на хост, видимость стрелки через hidden,
  активность через set_popup_active с репозиционированием. Демо-попап ищется как :scope > .popup, чтобы не зацепить вложенный popup-хост самого Select.
  - Заодно пример стал живой проверкой каскада: Select (на Popup) управляет другим Popup, события change нашего Select прозрачно работают в делегированной схеме.
  
  Отличие от WA-демо: там переключатель Arrow пересоздаёт узел стрелки через re-render Lit; у нас стрелка отрендерена статически и переключается атрибутом hidden — визуально поведение одинаковое. В
  example-client для этого добавлены зависимости wasm-dom/web-sys (демо-логика живёт в галерее, а не в библиотеке).

---

Перенеси компонент Popup в layouts.

---

● Перенос затрагивает библиотеку, web-крейт (для симметрии структуры), CSS и галерею. Посмотрю текущие module-файлы:
● Переношу файлы через git mv и правлю все ссылки:
● Осталось перенести пункт меню Popup из секции Components в Layouts. Смотрю секцию Layouts:
● Компонент Popup перенесён из components в layouts во всех четырёх местах, где он живёт. Все проверки прошли: fmt, clippy (-D warnings), 61 тест, WASM-клиент собран.

  Что перенесено (через git mv, история файлов сохранена):

  - Библиотека: crates/lib/src/components/popup.rs → crates/lib/src/layouts/popup.rs; публичный путь теперь wingy_hypertext::layouts::popup::{Popup, Placement, AutoSize, SyncSize, ArrowPlacement}.
  - Web-крейт (для симметрии структуры): crates/web/src/components/popup.rs → crates/web/src/layouts/popup.rs; путь wingy_hypertext_web::layouts::popup::{set_popup_active, reposition, init_popups, 
  listen_popups}.
  - CSS: webassets/style/components/popup.css → webassets/style/layouts/popup.css, импорт в index.css перенесён в группу layouts.
  - Галерея: страница examples/client/src/components/popup.rs → examples/client/src/layouts/popup.rs; маршрут "popup" теперь ведёт на layouts::popup::overview(), а пункт меню «Popup» перемещён из секции
  Components в секцию Layouts (после Page). Обвязка интерактивного примера (listen_popup_overview/init_popup_overview) переехала вместе со страницей.

  Обновлены все ссылки: импорты в тестах (crate::layouts::popup), в web-логике Select (crate::layouts::popup), в галерее (страница Popup, lib.rs), а также упоминания путей в doc-комментариях библиотечного и
  web-модуля и в комментарии исходника примера. Класс-константы (POPUP, POPUP_BODY, POPUP_HOVER_BRIDGE) и сами CSS-классы не менялись — на разметку и поведение перенос не влияет.

---

Нужно было перенести Popup не в layouts, а перенести в новый раздел helpers.

---

● Понял — WA относит Popup к категории Helpers. Создаю новый раздел helpers и переношу туда. Сначала посмотрю корневые модули:
● Готово — Popup перенесён из layouts в новый раздел helpers (как в категоризации WA, где Popup — Helpers). Все проверки прошли: fmt, clippy (-D warnings), 61 тест, WASM-клиент собран, helpers/popup.css
  копируется в target/web.

  Что сделано (файлы перемещены через git mv):

  - Библиотека: новый модуль crates/lib/src/helpers.rs + crates/lib/src/helpers/popup.rs; публичный путь — wingy_hypertext::helpers::popup::{Popup, Placement, …}; pub mod helpers; добавлен в корень крейта.
  - Web-крейт: crates/web/src/helpers/popup.rs, путь wingy_hypertext_web::helpers::popup::{set_popup_active, reposition, init_popups, listen_popups}.
  - CSS: webassets/style/helpers/popup.css; в index.css появилась отдельная группа импортов helpers (после layouts).
  - Галерея: страница — examples/client/src/helpers/popup.rs; маршрут ведёт на helpers::popup::overview(); в меню добавлена новая секция Helpers (после Layouts) с пунктом Popup, из Layouts пункт убран.

  Все ссылки обновлены: тесты (crate::helpers::popup), web-логика Select, обвязка интерактивного примера в init()/reinit(), doc-комментарии. Из layouts.rs всех трёх крейтов объявления popup удалены; разметка,
  классы и поведение не менялись.

---

Измени описания и примеры использования вспомогательного компонента Popup с учётом его изменённой структуры: теперь явно нужно указывать anchor и подкомпонент PopupBody. Примеры использования есть в тестах popup.

---

● Готово. Описания и примеры Popup приведены к новой структуре (явный anchor + PopupBody).

  Документация

  crates/lib/src/helper/popup.rs
  - Док Popup описывает композицию из детей и содержит два примера: якорь внутри попапа (PopupAnchor + PopupBody) и внешний якорь по anchor_id, когда попап несёт только тело (как у Tooltip). Добавлено
  пояснение, что позиционируется только тело, поэтому arrow/hover_bridge — свойства PopupBody, а компоненты со своим телом (DropdownMenu, листбокс Select) рендерят PopupBody сами.
  - anchor_id: убрана ссылка на несуществующий проп anchor — теперь «оставьте пустым, если якорь рендерится первым ребёнком».
  - Появились доки у PopupBody (позиционируемый элемент; без него попап ничего не позиционирует) и уточнён док PopupAnchor (рендерится как есть, только маркирует роль, ставится перед телом, можно опустить).
  - Поля: arrow (последний ребёнок тела, выравнивается Popup::arrow_placement/arrow_padding), hover_bridge (рендерится перед телом, соседом внутри попапа), arrow_placement.

  crates/web/src/helper/popup.rs — поправлен путь модуля (helper::popup) и описано, как определяется якорь (data-anchor либо ребёнок хоста, не являющийся частью попапа — отсюда и работает PopupAnchor).

  Примеры галереи

  examples/client/src/helper/popup.rs — все 11 демо-попапов (overview, Placement, Distance & Skidding, Arrows, Sync, Flip, Shift, Auto-Size) перестроены на <PopupAnchor> + <PopupBody>, показываемые в
  CodeExampleSource сниппеты приведены в соответствие. Добавлен абзац о композиции, в разделе Arrows текст переписан: arrow задаётся на PopupBody, а выравнивание — arrow_placement/arrow_padding самого попапа.

  Заодно — чтобы сборка соответствовала новой структуре

  - Dropdown и Select: убран уже несуществующий bare=true (тело им приносят DropdownMenu и PopupBody листбокса).
  - Tooltip: содержимое обёрнуто в <PopupBody arrow=(self.arrow) hover_bridge=true> — итоговая разметка не изменилась, тесты тултипа прошли без правок.
  - Новый тест anchor_component в crates/lib/src/tests/popup.rs: обёртка PopupAnchor даёт ту же разметку, что и прямая передача элемента.

  Проверки

  cargo +nightly fmt --check, cargo clippy --all-targets -- -D warnings, cargo test (97 passed, +1 новый), cargo make client, cargo doc (новых предупреждений нет — остались только два прежних [Escape]). В
  браузере: страница Popup — 10/10 проверок (у каждого попапа есть якорь и position: fixed-тело рядом с ним, placement разрешается как запрошено, контролы demo по-прежнему управляют distance/arrow/active),
  Tooltip — прежние 46 проверок, Dropdown и Select — меню и листбокс открываются и остаются привязанными к триггеру (2/2).

---

PopupAnchor не несёт никакой полезной функции, убери его и исправь примеры и описания.

---

● PopupAnchor удалён.
  
  Что изменилось
  
  - crates/lib/src/helper/popup.rs — структура PopupAnchor и её Renderable убраны. В доке Popup якорь описан как «первый ребёнок — любой одиночный элемент», пример стал <Popup  
  …><button>"Anchor"</button><PopupBody arrow=true>…</PopupBody></Popup>. Второй пример (внешний якорь по anchor_id + только тело, как у Tooltip) не изменился. В доке PopupBody убрана ссылка на удалённый
  компонент.
  - crates/web/src/helper/popup.rs — из описания того, как определяется якорь, убрано упоминание PopupAnchor: остался только сам механизм (data-anchor либо ребёнок хоста, не являющийся
  .popup-body/.popup-hover-bridge).
  - examples/client/src/helper/popup.rs — во всех 11 демо и в показываемых сниппетах обёртка снята: якорь передаётся напрямую ((anchor()) в живых примерах, <span class="anchor"></span> в коде). Вводный абзац
  переписан без упоминания обёртки. 
  - crates/lib/src/tests/popup.rs — тест anchor_component, проверявший обёртку, удалён вместе с ней (остальные тесты, где якорь — обычный элемент, уже покрывают композицию).
  
  Проверки
  
  cargo +nightly fmt --check, cargo clippy --all-targets -- -D warnings, cargo test (96 passed), cargo make client, cargo doc (новых предупреждений нет). В браузере после пересборки: страница Popup — 10/10 (у
  каждого из 11 попапов распознан якорь, тело позиционируется рядом, placement разрешается как запрошено, контролы demo работают), Dropdown/Select — 2/2, Tooltip — 27/27.

---

