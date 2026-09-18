Нужно перенести компоненты `Checkbox` и `CheckboxGroup`, повторяя функционал `wa-checkbox` и `wa-checkbox-group` из WebAwesome: `tmp/webawesome/packages/webawesome/src/components/checkbox` и `.../checkbox-group`.

Компонента для флажка у нас нет: `Switch` закрывает только роль переключателя, а `Select` с `multiple` — другой сценарий. Группа тоже нужна: в WA `wa-checkbox-group` объединяет не только флажки, но и `wa-switch`, то есть заодно даёт нашему `Switch` нормальную групповую раскладку с подписью, подсказкой и ориентацией.

Делать по образцу уже готовых `Radio` и `RadioGroup` — разметка, состав пропов и расположение файлов у них почти такие же.

`Checkbox` (`crates/lib/src/component/checkbox.rs`) — пропы `checked`, `indeterminate`, `disabled`, `required`, `name`, `value`, `hint` и `children` как подпись. В отличие от радио у флажка есть собственная подсказка. Разметка как у `Radio`: `<label class="checkbox"><input class="control" type="checkbox" …><span class="checkbox-indicator">…</span><span class="label">…</span></label>` плюс `<small class="hint">`, если задана подсказка. Галочку и чёрточку берём из iconic: `fontawesome::solid::Check` и `fontawesome::solid::Minus`, обе кладём в индикатор, а показ нужной решает CSS.

Промежуточное состояние (`indeterminate`) — единственное, что нельзя выразить разметкой: это свойство DOM, а не атрибут. Поэтому компонент рендерит маркер (например, `data-indeterminate`), а `crates/web` (`component::checkbox`) в `init_checkboxes()` проставляет `input.indeterminate = true`. Вызов добавить в `reinit`, потому что после htmx-свопа разметка создаётся заново. Больше клиентского кода не нужно: CSS опирается на нативную псевдоклассу `:indeterminate`, а по клику браузер сам снимает это состояние и переключает `checked`.

`CheckboxGroup` (`crates/lib/src/component/checkbox_group.rs`) — пропы `label`, `hint`, `orientation` (по умолчанию `Vertical`), `required`, `disabled`, `bare` и `children`, ровно как у `RadioGroup` в его текущем виде: `<fieldset class="checkbox-group …" role="group">` с `<legend class="label">`, контейнером `.checkbox-group-items` и подсказкой. Ориентацию брать через `Orientation::as_vertical_class()`. Роль здесь именно `group`, а не `radiogroup`: флажки независимы.

Стили — порт `checkbox.styles.ts` и `checkbox-group.styles.ts` в `webassets/style/components/checkbox.css` и `checkbox_group.css`. Два отличия от группы радио, которые легко потерять:

- Элементы группы флажков не растягиваются: в WA у контейнера `align-items: start`, чтобы кликабельная подпись не занимала всю ширину. Отступы — `gap` 0.5em по вертикали и 1em по горизонтали. У группы радио всё наоборот, там элементы тянутся на всю ширину.
- Звёздочку «обязательно» показывает только подпись группы. В WA это сделано через `--wa-form-control-required-content: ''` для вложенных элементов; у нас разметка плоская, поэтому достаточно ограничить правило прямым потомком, как в `radio_group.css`.

Отдельно учесть, что в `native.css` есть правило `label:has(input[type='checkbox'])` с `width: fit-content`. Для флажка это совпадает с задумкой WA, так что переопределять ширину, как пришлось в `.radio` и `.switch-toggle`, здесь не нужно — но при первой же правке раскладки об этом правиле стоит помнить.

Раскладка группы должна работать и со `Switch` внутри (пример Switches в доке). Проверить, что наш `.switch` с его `width: auto` не конфликтует с `align-items: start` у контейнера.

Валидацию переносим только нативную (`required` на самих флажках), как в `Radio`. Константы классов — `CHECKBOX`, `CHECKBOX_GROUP`, `CHECKBOX_GROUP_ITEMS`, `CHECKBOX_INDICATOR` — в `crates/lib/src/class.rs`.

Тесты разметки — `crates/lib/src/tests/checkbox.rs`.

Для визуального тестирования нужны два отдельных раздела в `examples/client`, как две страницы доки WA: у каждого свой маршрут и свой пункт меню в разделе Components.

- **Checkbox** (`examples/client/src/component/checkbox.rs`, маршрут `/checkbox`) — примеры из `tmp/webawesome/packages/webawesome/docs/docs/components/checkbox.md`: базовый, Initial Value, Hint, Indeterminate, Disabled, Size.
- **Checkbox Group** (`examples/client/src/component/checkbox_group.rs`, маршрут `/checkbox-group`) — примеры из `checkbox-group.md`: Label, Hint, Orientation, Size, Disabled, Switches, Required.

Пример Custom Validity со страницы флажка не переносим: свой текст ошибки мы не поддерживаем.

---

● Задача issues/checkbox component.md выполнена.
  
  Компоненты. Checkbox (crates/lib/src/component/checkbox.rs): checked, indeterminate, disabled, required, name, value, hint, подпись через children. CheckboxGroup (checkbox_group.rs): label, hint, orientation
  (по умолчанию Vertical), required, disabled, bare — тот же набор, что у RadioGroup, с role="group".
  
  Отступление от задачи по разметке. В задаче было <label class="checkbox"> плюс <small class="hint"> рядом. Я сделал как у Switch: обёртка <div class="checkbox">, внутри <label class="checkbox-toggle"> и
  подсказка после метки. Причина: подсказка не должна входить в метку (в WA она вне её, клик по ней не переключает флажок), а два корневых элемента сломали бы раскладку внутри flex-контейнера группы — метка и
  подсказка стали бы отдельными элементами потока. Поэтому классов вышло больше, чем в задаче: добавились CHECKBOX_TOGGLE, CHECKBOX_CHECK, CHECKBOX_INDETERMINATE.
  
  Промежуточное состояние — как и планировалось, единственное место с клиентским кодом: компонент рендерит data-indeterminate, а init_checkboxes() в crates/web проставляет свойство; вызов добавлен в reinit. 
  
  Стили: checkbox.css и checkbox_group.css. Элементы группы не растягиваются (align-items: start, gap 0.5em и 1em), звёздочка «обязательно» только у легенды группы — у вложенных флажков и переключателей она
  гасится. Ширину переопределять не пришлось, как и предполагалось в задаче.
  
  Ещё одна правка по ходу: галочка не отображалась — иконки в индикаторе не получали размер. По конвенции проекта добавил .checkbox-check и .checkbox-indeterminate в список контейнеров сырых иконок в
  webassets/style/utils/icon.css.
  
  Галерея: две отдельные страницы, как в WA — /checkbox (базовый, Initial Value, Hint, Indeterminate, Disabled, Size) и /checkbox-group (Label, Hint, Orientation, Size, Disabled, Switches, Required), оба пункта
  в меню, между ними ссылка.
  
  Проверка. fmt, clippy (-D warnings, включая wasm) и 119 тестов (8 новых) проходят. В Firefox:
  - промежуточное состояние применяется из маркера: свойство true, видна чёрточка, галочка скрыта, индикатор залит;
  - клик снимает его нативно и ставит галочку;
  - подсказка — блочная и вне метки;
  - группа: align-items: start, gap 8px, элемент 80px при ширине контейнера 547px, горизонтальная раскладка строкой;
  - звёздочка только у легенды, у пункта её нет, при этом у поля стоит required;
  - переключатели внутри группы работают и тоже по содержимому;
  - отключённая группа отключает все поля нативно.

---

Галочка в Checkbox рисуется неправильно: она маленькая и сдвинута влево.

---

● Исправил.
  
  Причина. Обе иконки лежали в индикаторе как соседние flex-элементы. Скрытая через visibility продолжает занимать место, поэтому видимая уезжала в левую половину, а svg ещё и сжимался по ширине, деля 20px на
  двоих.

  Замеры до исправления (индикатор 20×20 в позиции x=305):
  
  ┌──────────┬─────────────────────────┬─────────────────────────────┐
  │          │          было           │            стало            │
  ├──────────┼─────────────────────────┼─────────────────────────────┤
  │ галочка  │ x=307, 7×13             │ x=308, 14×14 — по центру    │
  ├──────────┼─────────────────────────┼─────────────────────────────┤
  │ чёрточка │ x=316 (рядом, а не под) │ x=308, ровно поверх галочки │
  ├──────────┼─────────────────────────┼─────────────────────────────┤
  │ сам svg  │ 9×16 (сжат)             │ 14×16, пропорции сохранены  │
  └──────────┴─────────────────────────┴─────────────────────────────┘
  
  Что изменил. Одно правило в webassets/style/components/checkbox.css: обе иконки теперь position: absolute; inset: 0 с центрированием, поэтому они лежат друг на друге и не влияют на раскладку. Масштаб
  --checked-icon-scale (0.8) сохранён, как в WA.
  
  Проверка в Firefox:
  - галочка и чёрточка центрированы в индикаторе (проверил вычислением центров);
  - промежуточное состояние по-прежнему применяется из маркера (property=true), видна чёрточка;
  - иконка масштабируется вместе с размером: индикатор/иконка — 15/8×10, 18/10×11, 20/11×13, 25/14×16, 31/18×20 для размеров от extra small до extra large;
  - на скриншотах галочка видна во всех размерах.
  
  fmt и 119 тестов проходят.

---

Поясни, зачем indeterminate устанавливается в input чекбокса? В WA вроде бы такого нет.

---

● В WA это есть — просто спрятано в механике Lit.
  
  Что делает WA. В checkbox.ts внутренний <input> получает именно DOM-свойство:

  .indeterminate=${live(this.indeterminate)}        // строка 253 — property binding, не атрибут
  
  и отдельно синхронизируется при изменениях:
  
  @watch(['checked', 'indeterminate'])
  … 
  this.input.indeterminate = this.indeterminate; // force a sync update   // строка 177
  
  А стили опираются на нативную псевдоклассу этого свойства:
  
  [part~='control']:has(:checked, :indeterminate) { … }   /* строка 57 */
  input:not(:checked, :indeterminate) + & { … }           /* строка 94 */
  
  Почему без этого нельзя. У indeterminate нет HTML-атрибута: спецификация определяет только IDL-свойство. Разметки <input type="checkbox" indeterminate> не существует, браузер её проигнорирует, и
  :indeterminate не сработает. Поэтому свойство обязан кто-то выставить из кода — и WA, и мы.
  
  В чём разница между нами и WA. У WA indeterminate — это атрибут на самом веб-компоненте (@property({ reflect: true })), то есть <wa-checkbox indeterminate> в разметке работает. Но дальше Lit при каждом
  рендере переносит его в свойство внутреннего input.
  
  У нас рендер серверный и статический, клиентского рендера нет вообще. Атрибут data-indeterminate в нашей разметке — это ровно аналог того самого отражённого атрибута на wa-checkbox, а init_checkboxes() —
  аналог их property binding: единственный проход, который переносит маркер в свойство. Дальше всё нативно: браузер сам снимает состояние по клику, а CSS работает через :indeterminate, как и в WA.
  
  Есть и мелкое отличие: WA дополнительно выставляет кастомное состояние customStates.set('indeterminate', …) для селектора :state(indeterminate) снаружи компонента. Нам это не нужно, у нас разметка плоская и
  всё доступно обычными селекторами.

---

