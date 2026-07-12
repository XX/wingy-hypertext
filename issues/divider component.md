Необходимо реализовать layout-компонент `Divider` - полноценный порт компонента `wa-divider` из WebAwesome (`tmp/webawesome`). Как в прочих компонентах (таких как `Button`, `Badge` и т.д.) необходимо добавить тесты. Для визуального тестирования необходим также новый раздел Divider с примерами использования в `examples/client` (по мотивам доки `wa-divider`, исключая пример "Dropdown Dividers", так как компонент Dropdown ещё не реализован).

---

● Задача выполнена. Компонент Divider реализован как порт wa-divider из Web Awesome, с тестами и разделом в галерее. Все проверки CI проходят: cargo +nightly fmt --check, cargo clippy --all-targets -- -D 
  warnings (без предупреждений), cargo test (46 тестов, включая 3 новых), и полная сборка WASM-клиента через cargo make client успешна.

  Что сделано:

  Библиотека (crates/lib):
  - src/layouts/divider.rs — новый layout-компонент Divider по канонической схеме (Props, #[const_str(CLASS = DIVIDER)], #[props(builder)], встроенный CommonAttrs). Поле orientation: Orientation (enum
  Horizontal/Vertical на strum с const_into_str, как Variant). Рендерится в <div class="divider [vertical]" role="separator" aria-orientation="..."> — role и aria-orientation соответствуют тому, что wa-divider
  выставляет в connectedCallback/watch. Поля children нет, так как у wa-divider нет слота.
  - src/class.rs — добавлены константы DIVIDER, VERTICAL и TEXT_CENTER (wa-text-center, нужна для примера Spacing).
  - src/tests/divider.rs — тесты default, orientation, additional_attributes по образцу тестов Badge (builder и rsx-формы).

  Стили:
  - webassets/style/layouts/divider.css — прямой порт divider.styles.ts: кастомные свойства --color/--width/--spacing, горизонтальный вариант через .divider:not(.vertical), вертикальный через .divider.vertical
  (селекторы :host([orientation]) переведены на классы, как принято в этом репозитории). Подключён в index.css.

  Галерея (examples/client):
  - src/layouts/divider.rs — страница-обзор по мотивам доки wa-divider: базовый пример, Width, Color, Spacing и Orientation, плюс info-заметка из доки в виде Callout. Пример "Dropdown Dividers" пропущен, как и
  требовалось.
  - src/lib.rs — маршрут "divider" в main_section и живая ссылка "Divider" в секции меню "Layouts" (между заглушками "Code Example" и "Page").

  Один нюанс: форма rsx! { <Divider></Divider> } не компилируется (нет children) — это сознательно, используется только самозакрывающаяся <Divider/>, по прецеденту Anchor в head.rs.

---

Провёл ревью и внёс небольшие корректировки, в частности вынес в отдельный модуль Orientation.
