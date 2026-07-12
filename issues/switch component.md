Необходимо реализовать компонент `Switch` - полноценный порт компонента `wa-switch` из WebAwesome (`tmp/webawesome`). Как в прочих компонентах (таких как `Button`, `Badge` и т.д.) необходимо добавить тесты. Для визуального тестирования необходим также новый раздел Switch с примерами использования в `examples/client` (по мотивам доки `wa-switch`).

---

● Задача выполнена. Компонент Switch реализован как порт wa-switch, с 6 тестами и разделом в галерее. Все проверки CI зелёные: cargo +nightly fmt --check, cargo clippy --all-targets -- -D warnings, cargo test
  (52 теста, из них 6 новых), полная сборка WASM-клиента через cargo make client успешна.

  Что сделано:

  Библиотека (crates/lib):
  - src/components/switch.rs — компонент Switch по канонической схеме. Свойства: checked, disabled, required, name, value, hint и children (label). Разметка: <div class="switch [required] 
  [disabled]"><label><input class="control" type="checkbox" role="switch" ...><span class="track"><span class="thumb"></span></span><span class="label">…</span></label><small class="hint">…</small></div>.
  - src/class.rs — константы SWITCH, TRACK, THUMB (остальные — CONTROL, LABEL, HINT, DISABLED, REQUIRED — уже были).
  - src/tests/switch.rs — тесты default, label, hint, states, name_and_value, additional_attributes.

  Стили: webassets/style/components/switch.css — порт switch.styles.ts (--width/--height/--thumb-size, трек, бегунок, фокус-кольцо, RTL-правила, disabled через label:has(> .control:disabled), forced-colors),
  подключён в index.css.

  Галерея: examples/client/src/components/switch.rs — раздел с примерами из доки (Label, Hint, Initial Value, Disabled, Size, Custom Properties), плюс маршрут "switch" и пункт меню "Switch" после "Select".

  Ключевые решения порта (WA — это Lit-элемент с shadow DOM и JS, здесь — статическая разметка):
  - Переключение без JS. WA переключает состояние обработчиками кликов и классом checked; здесь чекбокс — настоящий нативный <input type="checkbox"> внутри <label>, поэтому клик по label переключает его
  нативно, а CSS стилизует состояние через .control:checked ~ .track вместо класса. Благодаря этому не понадобился модуль в crates/web.
  - Без aria-checked. WA обновляет его из JS при каждом изменении; в статической разметке он бы устаревал после первого клика, а нативный checked-стейт чекбокса и так корректно передаётся в
  accessibility-дерево.
  - Переименования классов. Внутренний WA-класс input → control (чтобы не пересекаться с компонентом Input, у которого .input — корневой класс), WA-класс switch у трека → track (корневой .switch занят самим
  компонентом).
  - Слот label рендерится всегда (пустой <span class="label"> при отсутствии детей) — как в WA, где слот присутствует в shadow DOM всегда; заодно это делает <Switch/> и <Switch></Switch> эквивалентными.
  - Размеры — через существующие классы size-*/font-size, как у Badge/Input: --wa-form-control-toggle-size задан в em, поэтому переключатель масштабируется от шрифта, отдельный проп size не нужен (в WA он к
  тому же помечен как частично deprecated).

