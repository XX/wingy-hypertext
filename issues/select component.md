Также, как были реализованы такие компонеты, как `Button` и `Input`, необходимо реализовать компонент `Select` с компонентом `Option`, повторяя функционал компонентов `wa-select` и `wa-option` из WebAwesome: `tmp/webawesome`. Для визуального тестирования работы необходим также новый раздел Select с примерами использования в `examples/client`.

---

● Задача выполнена. Компоненты Select и SelectOption реализованы, раздел Select добавлен в галерею. Все проверки CI прошли: fmt, clippy (-D warnings), 37 тестов (6 новых для Select), WASM-клиент собран.

  Ключевое проектное решение. wa-select в Web Awesome — это кастомный listbox на 1160 строк JS (popup, клавиатурная навигация, теги для multiple, clearable). Эта библиотека — rendering-only, и её Input
  реализован как нативный <input> в стилизованной обёртке. Задача прямо говорит «как были реализованы Button и Input», поэтому Select сделан так же — на нативном <select>:

  <div class="select [pill] [required] [appearance]">
    <label class="label">…</label>            <!-- проп label -->
    <div class="select-field">
      <select class="control" name multiple disabled required>
        <option class="option" value … >…</option>   <!-- SelectOption -->
      </select>
    </div>
    <small class="hint">…</small>             <!-- проп hint -->
  </div>

  Библиотека (crates/lib/src/components/select.rs):
  - Select<R> — пропы appearance, pill, disabled, required, multiple, name, label, hint + children (опции), по рецепту Input.
  - SelectOption<R> — value, label, disabled, selected (функционал wa-option, применимый к нативному <option>). Назван SelectOption, а не Option, — иначе неизбежен конфликт со std::option::Option в каждом файле
  использования; причина зафиксирована в doc-комментарии.
  - Константы SELECT, SELECT_FIELD, OPTION в class.rs.

  Стили (webassets/style/components/select.css) — по образцу input.css: label/hint/required-звёздочка, .select-field (бокс с фокус-кольцом, appearance-модификаторы, pill, disabled через :has(:disabled)), сброс
  нативного вида у .control. Стрелка-шеврон приходит из уже существующих стилей select в native.css — в select.css она только репозиционирована к краю контрола, так как горизонтальные отступы лежат на обёртке.
  Для multiple поле растягивается и рендерится как открытый listbox.

  Галерея (examples/client/src/components/select.rs) — секции по мотивам доки wa-select, осуществимые на нативном select: базовый пример, Label, Hint, Appearance, Pill, Size, Disabled (весь select и отдельные
  опции), Initial Value (selected), Multiple. Маршрут "select" и пункт меню добавлены в lib.rs.

  Из функционала wa-select сознательно не переносилось то, что требует кастомного listbox и JS: placeholder, clearable, теги в multiple, placement, start/end-декорации и lazy loading. Если что-то из этого нужно
  — это отдельная задача с поведением в crates/web.

---

Необходимо перенести по возмоности весь функцинал из wa-select, логику следует реализовать на Rust в crates/web вместо JS.

---

● Готово. Функционал wa-select перенесён: нативный <select> заменён на полноценный кастомный дропдаун, вся логика — на Rust в crates/web, без JS. Проверки пройдены: fmt, clippy (-D warnings), 38 тестов,
  WASM-клиент собран.

  Разметка (crates/lib/src/components/select.rs) — теперь зеркалит структуру wa-select: .combobox с readonly display-input (показывает выбранное), скрытым value-input (несёт name/значение для формы),
  опциональной кнопкой очистки и поворачивающейся иконкой-шевроном; под ним .listbox (role="listbox") с опциями. SelectOption рендерится как <div role="option" data-value> с галочкой и обёрткой лейбла. Новые
  пропы: placeholder и with_clear. ARIA-атрибуты (combobox/listbox/option, aria-expanded, aria-selected, aria-multiselectable) расставлены как в WA.

  Логика (crates/web/src/components/select.rs, ~450 строк web-sys) — порт обработчиков wa-select; всё состояние живёт в DOM (классы/атрибуты), в духе остальных модулей крейта:
  - открытие/закрытие с анимацией show/hide через существующий animate_with_class (длительность настраивается токенами --show-duration/--hide-duration, при prefers-reduced-motion анимации отключены CSS-ом);
  - выбор кликом: single закрывает листбокс, multiple переключает опцию и держит его открытым; заблокированные опции игнорируются;
  - клавиатура: ArrowUp/ArrowDown (с зацикливанием), Home/End, Enter/Space — выбор, Escape — закрытие, плюс type-to-select с буфером на 1 секунду и Backspace, как в оригинале;
  - закрытие по клику и уходу фокуса за пределы селекта; клик по лейблу фокусирует контрол;
  - кнопка очистки видна только при непустом значении;
  - события input и change всплывают с корня .select при каждом изменении;
  - init_selects() синхронизирует начальное selected-состояние (значение, подпись, кнопку очистки) после каждого рендера, listen_selects() ставит делегированные слушатели один раз — в галерее подключены в
  reinit()/init() соответственно.

  CSS — порт select.styles.ts + option.styles.ts: combobox с фокус-кольцом и appearance/pill-модификаторами, листбокс с тенью на --wa-color-surface-raised, подсветка .current через
  --wa-form-control-activated-color, hover/disabled состояния опций, видимая галочка у .selected.

  Галерея — добавлены секции Placeholder и Clearable, обновлены вводный текст и Multiple.

  Что сознательно не переносилось (можно доделать отдельно): позиционирование с flip/shift (placement — листбокс всегда снизу), чипы-теги в multiple (показывается «N options selected», как в WA без тегов),
  слоты start/end, lazy loading, события wa-show/wa-hide и валидация форм через ElementInternals (значение для формы — скрытый input, значения multiple разделяются пробелом, по семантике атрибута value в WA).

---

Требуется ли также перенос компонента wa-popup, чтобы Select максимально близко соответствовал ws-select?

---

● Короткий ответ: обязательным не является — но без него остаются два реальных пробела. Посмотрю масштаб wa-popup, чтобы оценка была точной:
● Нет, для текущего состояния — не требуется, но с двумя оговорками. Разбор:
  
  Что wa-popup даёт wa-select. В оригинале wa-popup — это обёртка над JS-библиотекой @floating-ui/dom (сам компонент 619 строк + внешняя библиотека), из которой select использует: привязку листбокса к
  комбобоксу, sync="width", flip (открытие вверх, когда снизу нет места), shift, auto-size="vertical" (max-height по доступному месту во вьюпорте) и data-current-placement для transform-origin анимации.
  
  Что уже покрыто нашим портом без popup. Синхронизация ширины и привязка — бесплатно через position: absolute; left: 0; right: 0 внутри .select-popup; позиционирование снизу с отступом; transform-origin: top
  для анимации. Для подавляющего большинства форм этого достаточно, и визуально поведение совпадает с демо WA.
  
  Реальные пробелы:
  1. Нет flip — у нижнего края окна листбокс уйдёт за вьюпорт вместо открытия вверх.
  2. Нет auto-size — max-height захардкожен (min(20em, 50vh)) вместо расчёта по доступному месту.
  
  Рекомендация. Полный порт wa-popup сейчас не оправдан: это фактически реализация подмножества floating-ui на Rust (offset/flip/shift/size/autoUpdate) — отдельный крупный проект. Он станет осмысленным, когда
  дойдёт очередь до других компонентов на его основе — в WA это dropdown, tooltip, popover, color-picker, time-input. Тогда его стоит делать как переиспользуемую утилиту в crates/web, а не как часть select.
  
  Практичный промежуточный шаг — мини-позиционирование прямо в crates/web/components/select.rs (~60–80 строк): при открытии замерить getBoundingClientRect() комбобокса против высоты вьюпорта, при нехватке места
  снизу добавить класс placement-top (CSS: bottom: 100% + transform-origin: bottom) и выставить инлайновый max-height по доступному месту. Это закрывает оба пробела на ~90% без floating-ui.
  
  Могу реализовать этот мини-вариант, либо, если в планах есть Tooltip/Dropdown, — завести отдельный issue на полноценный порт wa-popup. Скажите, какой путь предпочитаете.

---

Да, в планах есть реализация многих других компонентов. Заведи отдельный issue на полноценный порт wa-popup.

---

● Посмотрю формат существующих issue и заведу новый:                                                                  
● Формат понятен. Создаю issues/popup component.md:
● Issue заведён: issues/popup component.md, в формате существующих задач репозитория. Содержание:
  
  - Что делать: компонент Popup — полноценный порт wa-popup как переиспользуемая основа позиционирования; логика на Rust в crates/web (подмножество floating-ui: anchor, placement, offset, flip, shift, sync
  width/height, auto-size с переменными --auto-size-available-*, arrow, autoUpdate при скролле/ресайзе, data-current-placement).
  - Контекст: в WA на wa-popup построены wa-select, wa-dropdown, wa-tooltip, wa-popover, wa-color-picker, wa-time-input, поэтому порт нужен до реализации этих компонентов; после него — перевести текущий Select
  с упрощённого CSS-позиционирования (без flip/auto-size) на Popup. 
  - Визуальное тестирование: раздел Popup в examples/client с примерами по мотивам доки wa-popup.

  Строку в issues/BACKLOG.md не добавлял — его вы ведёте сами; если нужно, могу дописать туда пункт «Реализовать компонент Popup».

---

Реализуй созданную задачу

---

Проверь корректность переноса компонента Select и реализации примеров из документации wa-select, с учётом обновлений в компонентах Popup, Select, Input, Switch и других.

---

● Задача ревизионная. Сначала соберу текущее состояние (компоненты обновлялись параллельно) — lib/web/CSS/галерея Select:
● Ревизия завершена, найденные расхождения исправлены. Все проверки прошли: fmt, clippy (-D warnings), 55 тестов, WASM-клиент собран.

  Что проверял и что оказалось корректным. Сверил crates/web/components/select.rs с обработчиками select.ts (mousedown/клик по опции/clear/label, Escape/Enter/Space/стрелки/Home/End/type-to-select с таймаутом,
  focusin-закрытие, форматы «N options selected»), интеграцию с Popup (порядок «снять hidden → активировать попап → мерить», деактивация после hide-анимации, конфигурация flip shift sync=width 
  auto-size=vertical padding=10 — байт-в-байт как у wa-select), разметку и ARIA, CSS листбокса/комбобокса. Обновления Switch/Divider/Input конфликтов с Select не внесли.

  Найденные расхождения — исправлены:

  1. Нет пропа placement — у wa-select есть placement: 'top' | 'bottom' (default bottom), прокидываемый в popup; наш Select жёстко рендерил data-placement="bottom". Добавил enum SelectPlacement { Bottom 
  (default), Top } и проп placement (+ тест).
  2. Фокус после клика по опции в multiple — WA всегда возвращает фокус на display-input после pointer-выбора (handleOptionClick, строка 589), у нас — только в single. Исправил в handle_option_click (для
  клавиатурного выбора фокус остаётся на опции — тоже как в WA).
  3. Guard на clear-button в keydown — WA игнорирует нажатия, когда таргет — кнопка очистки (handleDocumentKeyDown, строки 383–389); добавил.
  4. Не было стилей группировки — порт ::slotted(wa-divider) { --spacing: 0.5em } и стилей small-заголовков из select.styles.ts в select.css (наш Divider использует ту же переменную --spacing).

  Примеры из доки wa-select. Было покрыто: базовый, Label, Hint, Placeholder, Clearable, Appearance, Pill, Size, Disabled, Initial Value, Multiple. Добавил ставшие реализуемыми:
  - Grouping Options — Divider + <small> заголовки внутри Select (один в один пример из доки, включая оговорку про скринридеры);
  - Placement — placement=Top с примечанием про flip.

  Осталось непокрытым осознанно (требует того, чего нет в light-DOM-порте): Start & End Decorations (слоты в комбобоксе — потребует двух дополнительных generic-слотов у Select; реализуемо, но это расширение
  API, а не исправление), Custom Tags и Lazy Loading Options (переопределение рендера тегов и динамическая подгрузка — JS-паттерны поверх wa-select, не сам компонент). Если Start/End нужны — скажите, добавлю
  отдельно.

---

Заметил баг: при снятии фокуса с элемента после выбора опции в поле самого select промеликивает чёрная линия слева и справа, как будто некоторая рамка обводки поля ввода чёрного цвета, которая по ширине меньше самого элемента, на долю секунды становится видна внутри него и пропадает.

---

● Разберу симптом: чёрная рамка ýже комбобокса, видна слева/справа, мигает при потере фокуса — это похоже на outline внутреннего display-input, возвращающийся из outline: none (цвет = currentColor, почти
  чёрный) с transition в transparent. Проверю взаимодействие с глобальными стилями native.css:

● Баг исправлен. Причина была не в логике, а в каскаде CSS:
  
  Механизм. Глобальный native.css даёт всем текстовым <input> (наши display-input/value-input в light DOM под него попадают, в отличие от shadow DOM у WA) базовый outline: <style> <width> transparent с
  transition: outline-color. Наш select.css глушил outline только в состоянии :focus через outline: none — а этот шортхенд сбрасывает outline-color в initial-значение currentColor (почти чёрный). В момент
  потери фокуса правило :focus переставало действовать, возвращался базовый outline из native.css: стиль solid применялся мгновенно, а outline-color плавно анимировался от прежнего computed-значения 
  (currentColor = чёрный) к transparent. Сверху и снизу кольцо срезал overflow: hidden комбобокса (input почти на всю его высоту), поэтому видны были только вертикальные чёрные чёрточки слева и справа — ровно
  то, что вы описали.
  
  Исправление (webassets/style/components/select.css):
  - outline: none перенесён из :focus-правила в базовое правило .select .display-input (с комментарием, почему именно так) — теперь у внутреннего инпута нативного фокус-кольца нет ни в каком состоянии, и
  анимировать при блюре нечего. Отдельное :focus-правило удалено за ненадобностью.
  - То же добавлено скрытому .select .value-input — он невидим (opacity: 0), но подвержен той же механике.
  
  Видимое поведение фокуса не изменилось: кольцо фокуса, как и раньше (и как в WA), рисуется на самом комбобоксе через .select:focus-within .combobox { outline-color: var(--wa-color-focus) }. Именно так уже
  сделано у Input (outline: none в базовом правиле .control) — поэтому там этого мерцания не было.
  
  Статика пересобрана, фикс в target/web проверен.
