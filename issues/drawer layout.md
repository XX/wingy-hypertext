Также, как ранее были сделаны другие компоненты, необходимо реализовать layout-компонент `Drawer`, повторяя функционал компонента `wa-drawer` из WebAwesome (`tmp/webawesome`). Логику нового компонента вместо JS следует реализовать на Rust в `crates/web`. Необходим также новый раздел Drawer в `examples/client` с примерами использования, повторяющими примеры из WebAwesome для `wa-drawer`.

---

## Результат

Реализовано:

- **`crates/lib/src/layouts/drawer.rs`** — компонент `Drawer` (рендерится как нативный `<dialog>`): свойства `placement` (`DrawerPlacement`: Top/End/Bottom/Start), `label`, `open`, `without_header`, `light_dismiss`, слоты `header_actions` и `footer` (dyn-дети через `DynRenderable`), встроенная кнопка закрытия с `data-drawer="close"`. Классы добавлены в `crates/lib/src/class.rs`.
- **`webassets/style/layouts/drawer.css`** — портированные стили `wa-drawer` (размещение, размеры, анимации show/hide/pulse, backdrop), подключены в `webassets/style/index.css`.
- **`crates/web/src/layouts/drawer.rs`** — логика на Rust/WASM: открытие/закрытие модального `<dialog>` с анимациями, декларативные триггеры `data-drawer="open <id>"` / `data-drawer="close"`, закрытие по Escape, light dismiss, pulse при отмене, блокировка скролла body, отменяемые bubbling-события `wg-show`/`wg-hide` и `wg-after-show`/`wg-after-hide` (с `detail.source`). Хелпер `dispatch_custom` добавлен в `crates/web/src/utils/event.rs`. Подключено через `init_drawers`/`listen_drawers`.
- **`examples/client/src/layouts/drawer.rs`** — раздел галереи со всеми примерами из документации `wa-drawer` (overview, without header, footer, декларативное открытие/закрытие, placement, size, scrolling, header actions, light dismissal, preventing closing, initial focus). Маршрут `/drawer` и пункт меню добавлены в `examples/client/src/lib.rs`.
- Тесты рендеринга: **`crates/lib/src/tests/drawer.rs`** (8 тестов).

Проверено: `cargo +nightly fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (76 passed), `cargo make client` — всё зелёное.

---

● Задача issues/drawer layout.md выполнена.
  
  Что сделано
  
  1. Компонент Drawer — crates/lib/src/layouts/drawer.rs
  Рендерится как нативный <dialog>, следует общему паттерну (Default, AsRef, AsMut, Props, DynRenderable, #[const_str(CLASS = DRAWER)]). Свойства повторяют wa-drawer: placement (enum DrawerPlacement:
  Top/End/Bottom/Start, по умолчанию End), label, open, without_header, light_dismiss, а также слоты header_actions и footer как dyn-дети. Встроена кнопка закрытия с data-drawer="close". Новые CSS-константы — в
  class.rs.
  
  2. Стили — webassets/style/layouts/drawer.css
  Порт drawer.styles.ts: размещение по краям, --size/--spacing, анимации show/hide/pulse и backdrop, reduced-motion. Важный момент: базовый .drawer не задаёт display, чтобы закрытый <dialog> оставался скрытым
  (UA display:none), а flex включается на .drawer[open].
              
  3. Логика на Rust/WASM — crates/web/src/layouts/drawer.rs
  Модальное открытие/закрытие <dialog> с анимациями, декларативные data-drawer="open <id>" / data-drawer="close", Escape, light dismiss, pulse при отмене, блокировка скролла body и отменяемые всплывающие
  события wg-show/wg-hide/wg-after-show/wg-after-hide (с detail.source). Добавлен хелпер dispatch_custom в utils/event.rs и web-sys фичи (HtmlDialogElement, CustomEvent).
  
  4. Раздел галереи — examples/client/src/layouts/drawer.rs
  Все примеры из документации WebAwesome. Маршрут /drawer, пункт меню и вызовы init_drawers/listen_drawers подключены в lib.rs.
                                       
  5. Тесты — crates/lib/src/tests/drawer.rs (8 тестов).
  
  Проверки                                                        
                                                                         
  - cargo +nightly fmt --check ✅
  - cargo clippy --all-targets -- -D warnings ✅                                
  - cargo test — 76 passed ✅                                                      
  - cargo make client (WASM + wasm-bindgen + wasm-opt) ✅
                                                                                          
  Файл задачи оставил в issues/ (не переносил в done/), добавив в него краткие заметки о результате.
                                                                                                    
  Интерактивную проверку в браузере выполнить не удалось — установка расширения Claude in Chrome была отклонена. Сервер поднимается (target/debug/example-server, HTTP 200 на 127.0.0.1:9080); при желании можете
  проверить дровер в браузере вручную — например, ! target/debug/example-server и открыть http://127.0.0.1:9080/drawer.
