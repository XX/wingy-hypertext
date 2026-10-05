Нужно перенести компонент `Details`, повторяя функционал `wa-details` из WebAwesome: `tmp/webawesome/packages/webawesome/src/components/details`. Он сворачивает содержимое под заголовком и раскрывает его по клику. В WA компонент относится к категории Layout, поэтому у нас ему место в `crates/lib/src/layout/`, рядом с `Drawer` и `Divider`.

Основа — нативный `<details>`/`<summary>`: открытие, доступность, обход табуляцией и печать браузер даёт сам. Клиентский код нужен только для анимации, отменяемых событий и запрета переключения в состоянии `disabled`.

## Разметка

`crates/lib/src/layout/details.rs`. Структура повторяет `render()` из `details.ts` (строки 288–328), только без shadow DOM:

```
<details class="details outlined icon-end" name="…" open>
  <summary class="details-header" role="button" aria-expanded aria-controls aria-disabled tabindex>
    <span class="details-summary">…</span>
    <span class="details-icon">
      <span class="details-expand-icon">…</span>
      <span class="details-collapse-icon">…</span>
    </span>
  </summary>
  <div class="details-body" role="region" aria-labelledby>
    <div class="details-content">…</div>
  </div>
</details>
```

Атрибут `name` ставится прямо на нативный `<details>`: браузер сам закрывает остальные элементы группы, как это делает `closeOthersWithSameName` в WA. Нужно проверить, не ломает ли это анимацию закрытия: соседний элемент браузер закроет мгновенно. Если ломает, перехватить и закрывать соседей с анимацией из клиентского кода.

`aria-controls` и `aria-labelledby` ссылаются на `id` содержимого и заголовка. Если пользователь не задал `id`, их нужно сгенерировать уникальными для страницы. Посмотреть, как это решено у других компонентов, которым нужны связанные `id`.

Пропы: `open`, `summary` (строкой или разметкой, вместо слота `summary`), `name`, `disabled`, `appearance` (`Outlined` по умолчанию, плюс `Filled`, `FilledOutlined`, `Plain`), `icon_placement` (`End` по умолчанию, `Start`), `expand_icon` и `collapse_icon` (по умолчанию шеврон из `iconic`), `children` для содержимого. Для `appearance` есть `Appearance`, но значения `plain` там может не быть. Тогда либо добавить его в общий enum, либо завести свой для `Details`.

## Стили

`webassets/style/components/details.css` (или `layouts/`, смотря где лежат стили `Drawer`) — порт `details.styles.ts`: кастомные свойства `--spacing`, `--show-duration`, `--hide-duration`; четыре внешних вида; `disabled`; фокус-кольцо на `summary`; скрытие нативного маркера (`::marker`, `::-webkit-details-marker`); скругления заголовка у открытого элемента; расположение иконки в начале (`flex-direction: row-reverse`); поворот иконки на 90° у открытого и на −90° в RTL через `:dir(rtl)`, как в `switch.css`; переключение expand/collapse иконок по `[open]`; `overflow: hidden` на время анимации; стили печати.

## Поведение

`crates/web/src/layout/details.rs` (или `component/`, по месту стилей). Переносим логику из `details.ts`:

- **Клик по заголовку** (`handleSummaryClick`, строки 125–162): если клик пришёл с интерактивного элемента внутри заголовка (`a`, `button`, `input`, `textarea`, `select`), ничего не делать, иначе отменить нативное переключение и выполнить своё с анимацией. В `disabled` ничего не переключается.
- **Клавиатура** (`handleSummaryKeyDown`, строки 164–185): `Enter` и `Space` переключают, `ArrowUp`/`ArrowLeft` закрывают, `ArrowDown`/`ArrowRight` открывают.
- **Анимация** (`handleOpenChange`, строки 202–281): высота от `0` до `scrollHeight` и прозрачность, длительность из `--show-duration`/`--hide-duration`, затем высота `auto` или `0`. При закрытии `open` снимается с `<details>` только после анимации, иначе содержимое исчезнет сразу. Быстрое повторное переключение отслеживается счётчиком поколений, как в WA и в нашем `Dropdown`. Использовать хелперы из `util/animate.rs` (`linear_animate`) и учитывать `prefers_reduced_motion()`.
- **События**: `wg-show`, `wg-after-show`, `wg-hide`, `wg-after-hide` (константы уже есть в `util/event.rs`). `wg-show` и `wg-hide` отменяемые: отмена оставляет элемент в прежнем состоянии.
- **Внешнее изменение `open`**: в WA `MutationObserver` следит за атрибутом `open` у внутреннего `<details>`. У нас `<details>` и есть корень, так что нужно решить, нужен ли наблюдатель (например, чтобы анимировать открытие, когда браузер сам раскрывает элемент при поиске по странице через `hidden="until-found"`) или достаточно события `toggle`.
- `aria-expanded` и класс анимации обновляются в одном месте.
- `init_details()` для первичной настройки и после htmx-свопа (вызов в `reinit`), плюс делегированные слушатели на документе, как у остальных компонентов.

## Тесты и примеры

Тесты разметки — `crates/lib/src/tests/details.rs`: по умолчанию, `open`, `disabled`, внешние виды, расположение иконки, своя разметка в заголовке, свои иконки, `name`.

Раздел Details в `examples/client` (маршрут `/details`, пункт меню в разделе Layouts) со всеми примерами доки WA (`tmp/webawesome/packages/webawesome/docs/docs/components/details.md`): базовый, Expanded Initially, Disabled, Expand & Collapse Icons, Icon Placement, HTML in Summary, Right-to-Left Languages, Appearance, Grouping Details.

---

Сделано:

- Компонент `crates/lib/src/layout/details.rs`: `open`, `summary` (строка или разметка), `name`, `disabled`, `appearance` (`Outlined` по умолчанию; `Plain` в общем `Appearance` уже был), `icon_placement` (`DetailsIconPlacement`, класс `icon-start`), `expand_icon`/`collapse_icon` (по умолчанию шеврон, помечен `details-default-icon`, в RTL зеркалится только он, как в WA), `children`. Через `DynRenderable` (стираются `summary` и обе иконки).
- `id`: если у `Details` задан `id`, сервер рендерит `{id}-header`/`{id}-body` и связи `aria-controls`/`aria-labelledby`; иначе `id` генерирует клиент в `init_details`, как у `Tooltip`.
- Логическое состояние — класс `open`, нативный атрибут `open` снимается только после анимации сворачивания. Стили иконки и углов заголовка следуют классу.
- Группы `name`: нативный `name` оставлен (группа работает и без WASM). Чтобы браузер не схлопывал соседей мгновенно, на время анимации сворачивания у них временно снимается `name` и потом возвращается.
- Внешние изменения `open` (поиск по странице, скрипт, нативное закрытие в группе) отслеживаются событием `toggle` (ловится в фазе перехвата на документе); `MutationObserver` не понадобился. У `disabled` внешнее изменение откатывается.
- Поворот иконки в CSS завёрнут в `:where()`, поэтому отключается простым `.my-details .details-icon { rotate: none; }` — аналог `::part(icon)` из WA.
- Квадратных `square-plus`/`square-minus` в `iconic` нет, в примере Expand & Collapse Icons взяты `solid::Plus`/`Minus`.

---

● Задача issues/details component.md выполнена: Details перенесён из WA. Форматирование, clippy и cargo test (145 тестов, из них 12 новых) проходят. Галерея собирается через cargo make client, раздел я проверил
  в Firefox. Я ничего не коммитил, а файл задачи оставил в issues/ — только дописал в него итоги. 
  
  Что сделано:
  - Компонент crates/lib/src/layout/details.rs. Пропы: open, summary (строка или разметка), name, disabled, appearance (Outlined по умолчанию), icon_placement, expand_icon/collapse_icon, children. Основа —
    нативный <details>, поэтому открытие и группы работают и без WASM.
  - Стили webassets/style/layouts/details.css — перенос details.styles.ts. Они перекрывают общие стили для <details> из native.css: отступы и встроенный шеврон.
  - Поведение crates/web/src/layout/details.rs (init_details / listen_details): анимация высоты, клик по заголовку с пропуском ссылок и кнопок, Enter/Space и стрелки, события wg-show/wg-hide (их можно отменить)
    и wg-after-*.
  - Галерея: маршрут /details, пункт в разделе Layouts, все 9 примеров из документации WA.
  
  Решения по трём вопросам из задачи:
  - Группы name. Атрибут name остался нативным. Соседние элементы браузер закрыл бы мгновенно, поэтому на время анимации сворачивания у них временно снимается name, а потом возвращается.
  - plain. Это значение уже было в общем Appearance, отдельный enum не понадобился. 
  - Внешнее изменение open. Наблюдатель за атрибутом не понадобился, хватило события toggle. Оно ловит поиск по странице, изменение из скрипта и закрытие соседа браузером. У disabled такое изменение
    откатывается.
    
  Чем отличается от WA:
  - Логическое состояние — класс open. Нативный атрибут open снимается только после анимации сворачивания, чтобы содержимое было видно, пока оно сворачивается.
  - Если у Details задан id, сервер сам рендерит связи aria-controls/aria-labelledby. Иначе id генерирует клиент.
  - Поворот иконки отключается простым .my-details .details-icon { rotate: none; } — это замена ::part(icon) из WA.
  - Иконок square-plus/square-minus в iconic нет, поэтому в примере с кастомными иконками стоят solid::Plus/Minus.
  
  Что проверено в Firefox:
  - Во время анимации атрибут open держится до конца сворачивания.
  - Стрелки, Enter и Space переключают, без двойного срабатывания.
  - disabled не открывается ни кликом, ни клавишами, ни из скрипта.
  - Группа закрывает соседа с анимацией, порядок событий как в WA.
  - Ссылка в заголовке не переключает элемент.
  - Отмена wg-show/wg-hide работает.
  - Быстрый двойной клик не ломает состояние.
  - В RTL иконка повёрнута на −90°.
  - После перехода по меню и навигации назад/вперёд состояние восстанавливается.
