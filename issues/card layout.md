Нужно перенести компонент `Card`, повторяя функционал `wa-card` из WebAwesome: `tmp/webawesome/packages/webawesome/src/components/card`. Это контейнер с рамкой, который группирует связанное содержимое и действия: товар, статью, профиль и т. п. В WA компонент относится к категории Layout, поэтому место ему в `crates/lib/src/layout/`, рядом с `Drawer` и `Details`.

Клиентского поведения у карточки нет: в `card.ts` только рендер и `HasSlotController`, который проставляет `with-header`/`with-media`/`with-footer` по наличию слотов. Задача сводится к разметке и стилям, модуль в `crates/web` не нужен.

## Разметка

`crates/lib/src/layout/card.rs`. По правилу проекта внешняя разметка передаётся через `children` подкомпонентов, а не через пропы. Каждый слот WA становится подкомпонентом, как у `Drawer` (`DrawerHeader`/`DrawerHeaderActions`/`DrawerBody`/`DrawerFooter`):

```
<div class="card outlined vertical">
  <div class="card-media">…</div>
  <div class="card-header">
    …
    <div class="card-header-actions">…</div>
  </div>
  <div class="card-body">…</div>
  <div class="card-footer">
    …
    <div class="card-footer-actions">…</div>
  </div>
</div>
```

```
<div class="card outlined horizontal">
  <div class="card-media">…</div>
  <div class="card-body">…</div>
  <div class="card-actions">…</div>
</div>
```

- `Card` — пропы-значения: `appearance` (`Outlined` по умолчанию, плюс `Accent`, `Filled`, `FilledOutlined`, `Plain` — все уже есть в общем `Appearance`), `orientation` (`Vertical` по умолчанию — у общего `Orientation` по умолчанию `Horizontal`, поэтому значение задать явно, как `Appearance::Outlined` у `Accordion`). Секции — `children`.
- Подкомпоненты: `CardMedia`, `CardHeader`, `CardHeaderActions`, `CardBody`, `CardFooter`, `CardFooterActions`, `CardActions` (последний — только для горизонтальной карточки, как слот `actions` в WA).
- Порядок секций задаёт пользователь, но в WA он фиксирован (media → header → body → footer). Документировать ожидаемый порядок; стили скруглений (ниже) на него опираются.

### Отказ от `with-*`

В WA атрибуты `with-header`/`with-media`/`with-footer` нужны только потому, что в shadow DOM нет `:has-slotted`, а при SSR наличие слотов неизвестно. У нас разметка в light DOM: секция либо отрисована, либо её нет. Поэтому пустые секции не рендерятся вообще, а `with-*`-пропов нет. То, что в WA зависит от `with-*`, переводится на структурные селекторы:

- скругления верхних углов — у первой секции (`.card > :first-child`), нижних — у последней (`.card > :last-child`), вместо цепочек `:host(:not([with-media])) .header` и т. п.;
- класс `has-actions` (раскладка `space-between` в шапке/подвале) — через `:has(> .card-header-actions)` / `:has(> .card-footer-actions)`, либо просто всегда `flex` с `justify-content: space-between` — решить при реализации.

## Стили

`webassets/style/layouts/card.css` — порт `card.styles.ts`:

- корень: `--spacing` (по умолчанию `var(--wa-space-l)`), внутреннее `--inner-border-radius`, `flex-direction: column`, фон, рамка, скругление и толщина через `--wa-panel-*`, тень `--wa-shadow-s`;
- пять внешних видов, включая `accent` с цветом текста `--wa-color-neutral-on-loud`;
- скругления первой/последней секций (см. выше); у `plain` медиа скруглена со всех сторон;
- `.card-media`: `display: flex; overflow: hidden`, прямые потомки — `display: block; width: 100%` без скруглений (замена `::slotted(*)` на `> *`);
- `.card-header` с нижней, `.card-footer` с верхней границей (`border-*-style: inherit`), отступы как в WA;
- горизонтальная ориентация: `flex-direction: row`, скругления медиа только слева (логические свойства для RTL), медиа `object-fit: cover` на всю высоту, `.card-actions` — `flex` по центру с отступом `--spacing`;
- размер: в WA подключён `size.styles.ts` (`font-size` по `size`). Проверить, как размер задаётся у других наших компонентов (`common/component/size.css`, классы `wa-size-*`), и сделать так же.

Тема `awesome` (`webassets/style/common/themes/awesome.css`, правило `wa-card::part(header|footer)`) делает границы шапки и подвала пунктирными — перевести селекторы на `.card-header`/`.card-footer`, чтобы правило работало.

## Тесты и примеры

Тесты разметки — `crates/lib/src/tests/card.rs`: базовая карточка (только тело), шапка с действиями, подвал с действиями, медиа, все внешние виды, горизонтальная ориентация с `CardActions`, `id`/классы/стили.

Раздел Card в `examples/client` (маршрут `/card`, пункт меню в разделе Layouts) со всеми примерами доки WA (`tmp/webawesome/packages/webawesome/docs/docs/components/card.md`): обзорный (медиа, текст, кнопка в подвале и `Rating` в действиях подвала), Basic Card, Header, Footer, Media (картинка в `wa-frame:landscape` и видео), Appearance, Orientation. Ширину карточек в примерах (`max-width: 300px` и т. п.) задавать через стили галереи, а не inline. Иконки `ellipsis`/`gear` — из `iconic`. Внешние картинки/видео из примеров WA (unsplash, uploads.webawesome.com) можно оставить ссылками или подобрать замену — решить при реализации.

---

Сделано:

- Компонент `crates/lib/src/layout/card.rs`: `Card` (`appearance` — `Outlined` по умолчанию, `orientation` — `Vertical` по умолчанию) и секции `CardMedia`, `CardHeader`, `CardHeaderActions`, `CardBody`, `CardFooter`, `CardFooterActions`, `CardActions`. Секции — одинаковые `<div>` с классом, поэтому генерируются локальным `macro_rules! card_section`.
- Класс ориентации ставится только у горизонтальной карточки (`horizontal`): добавлены `Orientation::as_horizontal_class()` и `class::HORIZONTAL`. Новые классы `CARD*`, а для галереи — `CAPTION_S` (`wa-caption-s`) и `FRAME_LANDSCAPE` (`wa-frame:landscape`).
- `with-*` не нужны: скругления берут первая/последняя секции (`.card > :first-child`/`:last-child`, у горизонтальной — левые/правые углы), раскладка с действиями — через `:has(> .card-header-actions)`/`:has(> .card-footer-actions)`. `CardHeaderActions`/`CardFooterActions` — flex-обёртки с небольшим `gap`: в WA действия кладутся в слот без обёртки.
- Стили `webassets/style/layouts/card.css`. Размер задаётся классами `wa-size-*` на карточке, отдельного пропа нет.
- Отличие от плана: пунктир темы `awesome` вынесен не в `common/themes/awesome.css` (это почти дословная копия стилей WA, её правки затрёт следующая синхронизация), а в конец `card.css` селектором `.wa-theme-awesome .card > .card-header|.card-footer`.
- Тесты `crates/lib/src/tests/card.rs` (8 штук).
- Раздел `/card` в галерее (пункт меню в Layouts) со всеми примерами доки WA: обзорный, Basic Card, Header, Footer, Media, Appearance, Orientation. Ширина карточек задаётся через `<style>` внутри превью, как в других разделах галереи. Картинки и видео — внешние ссылки из доки WA. `wa-frame:landscape` ставится прямо на `CardMedia`, без лишнего `div`.
- В горизонтальной карточке медиа обёрнута в `.card-media`, поэтому при нехватке места картинка сжимается сильнее `max-width` (в примере ~134px). У WA картинка — сам flex-элемент и тоже сжимается (по расчёту ~160px), так что поведение близкое. Если нужна фиксированная ширина, достаточно `flex-shrink: 0` на `.card-media` и `inline-size: auto` на картинке.
