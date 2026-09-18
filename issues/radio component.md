Также, как были реализованы компоненты Switch, Select и другие, необходимо перенести компоненты `Radio` и `RadioGroup`, повторяя функционал `wa-radio` и `wa-radio-group` из WebAwesome: `tmp/webawesome/packages/webawesome/src/components/radio` и `.../radio-group`.

Делать после задачи `issues/button group component.md`. Зависимости между ними нет: сегментированный вид радио в WA делается не группой кнопок, а самим радио с `appearance="button"` внутри `RadioGroup`.

Состояние и клавиатура должны работать нативно, как у `Switch`: скрытый `<input type="radio">` внутри `<label>`, отмеченное состояние стилизуется через `:checked`. Браузер сам даёт группировку по `name`, переключение стрелками с переносом, пропуск `disabled` и проверку `required`. Почти вся логика `radio-group.ts` (406 строк) существует из-за shadow DOM и form-associated custom elements — нам она не нужна, так что код в `crates/web`, скорее всего, не понадобится.

`Radio` (`crates/lib/src/component/radio.rs`) — пропы `checked`, `disabled`, `name`, `value`, `appearance` (`Default` | `Button`) и `children` как подпись. Разметка по образцу `Switch`: `<label class="radio …"><input type="radio" class="control" …><span class="label">…</span></label>`. В отличие от WA кружок отмеченного состояния рисуем на CSS, а не вставляем SVG в разметку. Состояния по `name`/`value` несут сами радио, как у нас `SelectOption` несёт `selected` — группа их не проставляет, потому что дети приходят непрозрачным `Renderable`.

`RadioGroup` (`crates/lib/src/component/radio_group.rs`) — пропы `label`, `hint`, `orientation` (`crate::orientation::Orientation`), `size`, `required`, `disabled` и `children`. Разметка — `<fieldset class="radio-group …" role="radiogroup" aria-orientation=…>` с `<legend>` или подписью в `class=LABEL`, детьми и `<small class="hint">`, как у `Switch` и `Input`. Размер задавать классами `size-*`, они наследуются вложенными радио.

Стили — порт `radio.styles.ts` и `radio-group.styles.ts` в `webassets/style/components/radio.css` и `radio_group.css`: обычный вид с кружком, `appearance=button` (высота `--wa-form-control-height`, рамка, фон, скрытый кружок), отмеченное и `disabled` состояния, фокус. Скругление в сегментированном виде: WA расставляет из скрипта атрибуты `data-wa-radio-first` / `inner` / `last`, а у нас плоская разметка, поэтому хватит `:first-child` / `:last-child` и учёта ориентации в CSS. Константы классов — в `crates/lib/src/class.rs`.

Валидацию переносим только нативную (`required` на радио группы). Свой текст ошибки и `setCustomValidity` из WA не повторяем — этого нет и в остальных наших компонентах формы.

Тесты разметки — `crates/lib/src/tests/radio.rs`. Для визуального тестирования нужен раздел Radio Group в `examples/client` (маршрут, пункт меню в разделе Components) с примерами из доки WA (`tmp/webawesome/packages/webawesome/docs/docs/components/radio-group.md`): базовый пример, Initial Value, Hint, Radio Buttons, Disabled, Orientation, Size, Validation.
