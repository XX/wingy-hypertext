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

