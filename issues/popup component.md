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
