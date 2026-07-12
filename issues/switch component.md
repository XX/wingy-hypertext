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

---

Измени внутреннее устройство компонента `Switch` таким образом, чтобы он поддерживал свойство `bare` при котором пользователь мог собственноручно указать разметку внутреннеего содержания, включая html-разметку для hint:
```rust
<Switch bare=true>
    <SwitchLabel>
        "Email me about new releases"
    </SwitchLabel>
    <small class=HINT>"You can change this "<strong>"at any time"</strong>" in settings."</small>
</Switch>
```
Это поведение похоже на то, которое было реализовано в `Callout`, но с использованием субкомпонента `SwitchLabel`.

---

● Понял — по аналогии с Input/TextField и Callout::bare. Посмотрю, как Input использует композицию с TextField в тестах:
● Проверю, реализует ли hypertext Renderable для ссылок (нужно для передачи children по ссылке во внутренний SwitchLabel):
● Готово. Switch теперь поддерживает bare, а разметка label-блока вынесена в субкомпонент SwitchLabel — ровно по образцу Callout::bare и композиции Input/TextField. Все проверки зелёные: fmt, clippy без
  предупреждений, 54 теста (2 новых), сборка WASM-клиента успешна.

  Как это устроено в crates/lib/src/components/switch.rs:

  - SwitchLabel — новый субкомпонент, рендерящий label-блок целиком: <label><input class="control" type="checkbox" role="switch" ...><span class="track"><span class="thumb"></span></span><span 
  class="label">children</span></label>. Несёт пропсы состояния контрола (checked, disabled, required, name, value) плюс собственный CommonAttrs.
  - Switch в обычном режиме рендерит как раньше, но теперь внутренне собирает SwitchLabel из своих пропсов (children передаются по ссылке через children: self.children.as_ref(), без клонирования) — разметка не
  изменилась, все прежние тесты прошли без правок.
  - Switch bare=true рендерит children как есть внутри корневого <div class="switch">, и пользователь сам компонует тело из SwitchLabel и произвольной HTML-разметки hint. Проп hint при этом продолжает работать
  независимо (рендерится после тела, если задан) — по аналогии с Callout, где bare убирает только контейнер сообщения.

  В bare-режиме пропсы состояния указываются на SwitchLabel, а не на Switch (как у Input/TextField, пропсы не проксируются внутрь children); required/disabled на самом Switch по-прежнему вешают классы состояния
  на корневой элемент.

  Тесты: switch_label (standalone-рендер субкомпонента) и bare — включая дословно ваш пример с <small class=HINT> и <strong>; ожидаемая разметка совпала. CSS менять не пришлось — селекторы (.switch 
  .control:checked ~ .track и т.д.) не зависят от того, кто отрендерил label-блок. В галерее раздел Hint дополнен вторым примером с bare + HTML-hint (аналог «use the hint slot» из доки WA).

---

`SwitchLabel`, наверное, лучше переименовать в `SwitchInput` или `SwitchControl` (предложи хорошие имена на выбор)?

---

Как назвать субкомпонент, рендерящий label-блок переключателя (input + трек с бегунком + текст label)?

❯ 1. SwitchControl (Recommended)
     В терминологии форм «control» — интерактивный виджет целиком, и именно этот структ несёт все пропсы контрола (checked, name, value, disabled, required). Лёгкий минус: класс "control" внутри висит на 
     нативном <input>, а в WA part="control" — это трек.
  2. SwitchToggle
     Подчёркивает, что это переключаемая часть (без hint). Минус: тавтология — switch и toggle синонимы.
  3. SwitchBase
     Дословно повторяет WA: у wa-switch этот <label> помечен part="base". Максимальная верность порту, но имя малоговорящее для пользователя библиотеки.
  4. Оставить SwitchLabel
     Элемент рендерится как <label>, а дефолтный слот wa-switch в доке так и называется «The switch's label». Пропсы состояния на нём — как у <label>-обёртки нативного чекбокса.
  5. Type something.

---

Toggle

---

● Переименовал SwitchLabel → Toggle по вашему выбору. Затронуты три файла: crates/lib/src/components/switch.rs (структура, док-комментарии и внутренняя сборка в Switch::render_to),
  crates/lib/src/tests/switch.rs (импорт, тест switch_label переименован в toggle, оба bare-сценария) и страница галереи examples/client/src/components/switch.rs (импорт, текст раздела Hint и оба примера с
  bare).

  Использование теперь выглядит так:

  <Switch bare=true>
      <Toggle>
          "Email me about new releases"
      </Toggle>
      <small class=HINT>"You can change this "<strong>"at any time"</strong>" in settings."</small>
  </Switch>

  Все проверки зелёные: fmt, clippy без предупреждений, 54 теста проходят, WASM-клиент собирается.

  Замечу на будущее: имя Toggle — глобальное в components::switch, без префикса Switch*. Если позже появится порт wa-checkbox или другой toggle-подобный контрол, возможно, стоит держать в уме риск коллизии имён
  на уровне прелюдии/реэкспортов, но внутри модуля switch оно читается отлично.

