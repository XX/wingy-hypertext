Нужно перенести компонент `Accordion`, повторяя функционал `wa-accordion` и `wa-accordion-item` из WebAwesome: `tmp/webawesome/packages/webawesome/src/components/accordion` и `.../accordion-item`. Это вертикальный набор заголовков-переключателей, каждый раскрывает свой раздел содержимого. В WA компонент относится к категории Layout, поэтому место ему в `crates/lib/src/layout/`, рядом с `Details`.

## Связь с Details

На первый взгляд `Accordion` — это набор `Details` с общим `name`, но в WA он устроен иначе, и повторять это стоит именно так:

- Разметка следует [шаблону accordion из W3C APG](https://www.w3.org/WAI/ARIA/apg/patterns/accordion/): `<button aria-expanded aria-controls>` внутри заголовка `<h1>`–`<h6>` и панель `role="region"`. Нативный `<summary>` в заголовок не обернуть: он должен быть первым потомком `<details>` и сам имеет роль кнопки, поэтому заголовок внутри него для скринридера теряется. Навигация по заголовкам — главное, что отличает аккордеон от набора `Details`.
- Режим `single` (открытый пункт не закрывается кликом по нему) нативный `name` не умеет: он даёт только поведение `single-collapsible`.
- Стрелки, `Home`/`End` переводят фокус между пунктами всего аккордеона, а не переключают один пункт, как у `Details`.

Поэтому `AccordionItem` строится на `<button>` + `<div role="region">`, а не на `<details>`. Без WASM пункты не переключаются (как и `Dropdown`), но начальное состояние сервер отрисовывает полностью.

Переиспользовать из `Details`:

- **Анимацию высоты** — `animate_body`, `reset_body`, `durations`, счётчик поколений (`GENERATION`, `bump_generation`) в `crates/web/src/layout/details.rs`. Логика та же, что в `handleExpandedChange` у `accordion-item.ts`: высота от `0` до `scrollHeight` с прозрачностью, затем `auto` или `0`. Вынести её в общий модуль (например, `crates/web/src/helper/collapse.rs` или `util/`), параметризовав элементом-телом, и перевести на него `Details`. У WA в аккордеоне есть `--easing`, у `Details` нет — общий хелпер должен его принимать.
- **Связь `id`** — `aria-controls`/`aria-labelledby`: если у пункта задан `id`, сервер рендерит `{id}-trigger`/`{id}-panel`, иначе их генерирует клиент через `util::id::ensure_id`, как в `Details`.
- **Расположение иконки** — `DetailsIconPlacement` по смыслу общий. Переименовать в общий `IconPlacement` (рядом с `Appearance`) или завести свой для аккордеона — решить при реализации.
- **Внешний вид** — общий `Appearance` (`Outlined` по умолчанию, `Filled`, `FilledOutlined`, `Plain`), как у `Details`.
- **Поворот иконки** через `:where()` и `:dir(rtl)`, как в `details.css`, чтобы его можно было отключить простым селектором.

## Разметка

`crates/lib/src/layout/accordion.rs`. По правилу проекта внешняя разметка передаётся через `children` подкомпонентов, а не через пропы.

```
<div class="accordion outlined icon-start" data-mode="single">
  <div class="accordion-item expanded disabled" id="…">
    <h3 class="accordion-item-heading">
      <button class="accordion-item-trigger" type="button" id="…-trigger"
              aria-expanded aria-controls="…-panel" aria-disabled tabindex>
        <span class="accordion-item-label">…</span>
        <span class="accordion-item-icon" aria-hidden="true">…chevron…</span>
      </button>
    </h3>
    <div class="accordion-item-panel" id="…-panel" role="region" aria-labelledby="…-trigger">
      <div class="accordion-item-content">…</div>
    </div>
  </div>
</div>
```

- `Accordion` — пропы-значения: `mode` (`Multiple` по умолчанию, `Single`, `SingleCollapsible`), `appearance`, `icon_placement`. Пункты — `children`.
- `AccordionItem` — `label` (строкой, как `summary` у `Details`), `expanded`, `disabled`, `heading_level`, `children` — содержимое. Для разметки в заголовке и своей иконки — режим `bare` с подкомпонентами `AccordionItemTrigger` (заголовок с кнопкой) и `AccordionItemPanel`, как `DetailsHeader`/`DetailsBody` с `bare` у `Details`. По умолчанию пункт собирается из этих же подкомпонентов.

В WA `Accordion` проставляет `appearance`, `icon-placement` и `heading-level` своим пунктам. У нас пункты рендерятся независимо и родителя не видят, поэтому:

- `appearance` и `icon_placement` — классы на корне `.accordion`, а стили пунктов вешаются через дочерний комбинатор (`.accordion.icon-start > .accordion-item …`), чтобы не протекать во вложенные аккордеоны.
- `heading_level` меняет тег, CSS тут не поможет. Сделать его пропом `AccordionItem` (`h3` по умолчанию, `None` — без обёртки, как `heading-level="none"`) или найти способ задавать его один раз на аккордеон — решить при реализации.

## Стили

`webassets/style/layouts/accordion.css` — порт `accordion.styles.ts` и `accordion-item.styles.ts`: рамка и скругление на корне, разделители между пунктами (у `filled` — зазор вместо границы), четыре внешних вида, кастомные свойства `--spacing`, `--show-duration`, `--hide-duration`, `--easing`; кнопка-триггер во всю ширину с фокус-кольцом, утопленным внутрь (иначе его срежет `overflow: hidden` корня); иконка в начале через `row-reverse`; поворот иконки на 90° у раскрытого пункта и на −90° в RTL; `disabled`; `overflow: hidden` на панели, кроме раскрытой без анимации; размер от `font-size` корня. Внутренняя кнопка-триггер — служебная, на неё распространяется задача `isolate internal buttons.md`: глобальные стили кнопок из `native.css` надо сбросить.

## Поведение

`crates/web/src/layout/accordion.rs`, логика из `accordion.ts` и `accordion-item.ts`:

- **Клик по триггеру** (`handleItemTrigger`): у `disabled` ничего; раскрытый пункт в `single` не закрывается, в остальных режимах закрывается; при раскрытии в `single`/`single-collapsible` остальные пункты своего аккордеона сворачиваются. Вложенные аккордеоны не затрагиваются: пункт принадлежит ближайшему `.accordion` (`ownsItem`).
- **Клавиатура**: `Enter`/`Space` на триггере переключают; `ArrowDown`/`ArrowUp` циклически, `Home`/`End` — фокус на первый/последний доступный триггер своего аккордеона.
- **Анимация** — общий хелпер из `Details`, с учётом `prefers_reduced_motion()` и счётчика поколений для быстрых повторных кликов.
- **События** на корне `.accordion` с `detail.item`: отменяемые `wg-expand`/`wg-collapse` и `wg-after-expand`/`wg-after-collapse` (добавить константы в `util/event.rs`). Отмена оставляет пункт в прежнем состоянии.
- **Раскрыть/свернуть все** (`expandAll`/`collapseAll`): публичные функции; `expand_all` в режимах `single*` ничего не делает.
- `init_accordions()` в `reinit` (связь `id`, синхронизация высоты панели с состоянием после свопа), `listen_accordions()` с делегированными слушателями на документе.
- По желанию: `hidden="until-found"` на свёрнутой панели, чтобы поиск по странице раскрывал пункт (событие `beforematch`). В WA этого нет, но для SSR-разметки это естественное улучшение — решить при реализации.

## Тесты и примеры

Тесты разметки — `crates/lib/src/tests/accordion.rs`: по умолчанию, `expanded`, `disabled`, режимы, внешние виды, расположение иконки, уровни заголовка и без заголовка, `bare` с разметкой в заголовке и своей иконкой, `id` и связи ARIA.

Раздел Accordion в `examples/client` (маршрут `/accordion`, пункт меню в разделе Layouts) со всеми примерами доки WA (`tmp/webawesome/packages/webawesome/docs/docs/components/accordion.md`): базовый, Expanded Initially, Disabled, Heading Level, Size, Appearance, Mode, Icon Placement, Custom Icon, HTML in the Label, Expand & Collapse All, Nested Accordions, Preventing Expand or Collapse. Иконок `circle-plus`/`square-plus`/`square-minus` в `iconic` может не быть — подобрать замену, как в `Details`.

После выноса анимации раздел Details должен работать как раньше.

---

Сделано:

- Компонент `crates/lib/src/layout/accordion.rs`: `Accordion` (`appearance`, `mode` — `AccordionMode::{Multiple, Single, SingleCollapsible}`, `icon_placement`), `AccordionItem` (`label` строкой, `expanded`, `disabled`, `heading_level`), подкомпоненты `AccordionItemTrigger` (кнопка в заголовке, с `bare`), `AccordionItemLabel`, `AccordionItemIcon` (без детей — шеврон по умолчанию), `AccordionItemPanel`. Пункт с `label` собирается из них же.
- Уровень заголовка — проп пункта (`HeadingLevel::{H1..H6, None}`, `H3` по умолчанию): он меняет тег, а пункты рендерятся независимо от аккордеона. Внешний вид и расположение иконки — классы на `.accordion`, стили пунктов идут через дочерние комбинаторы, вложенные аккордеоны не задеваются. Режим — `data-mode`.
- `DetailsIconPlacement` стал общим `crate::icon_placement::IconPlacement`; `Details`, его тесты и галерея переведены.
- Свёрнутая панель — `hidden="until-found"`: её содержимое вне порядка табуляции и дерева доступности, а поиск по странице находит и раскрывает его (`beforematch`, пункт становится раскрытым, в режимах `single*` остальные сворачиваются). Для этого глобальное `[hidden] { display: none !important }` в `index.css` теперь не трогает `until-found`; браузеры без поддержки считают его обычным `hidden`.
- Анимация высоты вынесена из `Details` в `crates/web/src/util/collapse.rs` (`animate_body`, `reset_body`, счётчик поколений), с `--easing` из CSS (по умолчанию линейная). У `.details` задано `--easing: linear`, чтобы вложенный в аккордеон `Details` не унаследовал его плавность.
- Поведение `crates/web/src/layout/accordion.rs`: клик, `Enter`/`Space` (обрабатываются в `keydown` с отменой, как в WA), стрелки и `Home`/`End` по доступным пунктам своего аккордеона; события `wg-expand`/`wg-collapse` (отменяемые) и `wg-after-expand`/`wg-after-collapse` на аккордеоне с `detail.item`; `expand_all`/`collapse_all`; `init_accordions` связывает `id` и синхронизирует `aria-expanded`/`hidden` с классом `expanded`. Как в WA, в режимах `single*` остальные пункты сворачиваются без своих событий, а `disabled`-пункт не сворачивается и ими.
- Стили `webassets/style/layouts/accordion.css`. Триггер изолирован от `native.css` через `all: unset` (стили `native.css` лежат в слое `wa-native`, наши — вне слоёв). Слот `.accordion-item-icon` добавлен в список размеров SVG в `utils/icon.css`.
- Галерея: маршрут `/accordion`, пункт в разделе Layouts, все примеры из доки WA. В Custom Icon вместо `circle-plus`/`square-plus`/`square-minus` (их нет в `iconic`) — `solid::Plus`/`Minus`.
- Тесты: `crates/lib/src/tests/accordion.rs` (9 тестов).

Проверено: `cargo +nightly fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (164 теста), `cargo make client`. В headless Firefox: все примеры, режимы, клавиатура, раскрыть/свернуть все, вложенность, отмена `wg-collapse`, связи `id`, уровни заголовка, RTL, иконка в начале, поиск по странице через `window.find`; анимация `Details` после выноса хелпера работает как раньше.
