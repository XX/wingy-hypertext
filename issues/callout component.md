Также, как были добавлены ранее компоненты Badge, Button и Input, необходимо добавить компонет Callout с базовой начальной функциональностью и настройками стилей. Ориентироваться стоит на реализацию Callout в `tmp/webawesome`. Для визуального тестирования работы необходим также новый раздел Callout с примерами использования в examples/client.

---

● Готово. Компонент Callout реализован по образцу Badge/Button/Input с ориентацией на tmp/webawesome. Все проверки CI пройдены: cargo +nightly fmt --check, cargo clippy --all-targets -- -D warnings, cargo test
  (30 тестов, из них 6 новых для Callout), а WASM-клиент собран через cargo make client.
  
  Что сделано:
  
  Библиотека (crates/lib/src/components/callout.rs) — структура Callout<I, R> по стандартному рецепту (Props, #[const_str(CLASS = CALLOUT)], CommonAttrs, variant/appearance). Рендерит <div class="callout …"> с
  опциональным контейнером callout-icon и обёрткой callout-message — зеркально частям icon/message из wa-callout. Иконка передаётся пропом: <Callout icon=(…)>…</Callout>, как слот icon в Web Awesome.
  
  Доработка макроса (crates/macros/src/derive.rs) — #[prop(convert)] раньше работал только с одним generic-параметром: сгенерированный метод передекларировал бы все параметры структуры и конфликтовал с
  impl-блоком. Теперь метод объявляет только заменяемый параметр, что позволило Callout иметь два независимых слота (icon и children). Для существующих компонентов генерируемый код не изменился.
  
  Отличие от других компонентов: у Callout ручной impl Default — вариант Brand и appearance FilledOutlined (у Badge/Button берутся Neutral/Accent из enum-дефолтов). Так callout по умолчанию выглядит как в Web
  Awesome — тихая заливка с рамкой, а не сплошной тёмный блок; в callout.ts это задано явно (variant = 'brand', базовый CSS соответствует filled-outlined).
  
  Стили (webassets/style/components/callout.css) — порт callout.styles.ts: flex-контейнер, модификаторы plain/outlined/filled/filled-outlined/accent, стили иконки и сообщения через @scope (.callout) как в
  badge.css. Подключён в index.css.
  
  Галерея (examples/client/src/components/callout.rs) — раздел с примерами по мотивам доки Web Awesome: дефолтный callout, Variants (5 вариантов с тематическими иконками), Appearance, Size (через size-*
  классы), Without an Icon. Добавлены маршрут "callout" в main_section и пункт «Callout» в меню (lib.rs), а в fontawesome.rs — шесть новых иконок (circle-info, circle-check, circle-exclamation,
  triangle-exclamation, gear).
  
  Проект не запускал, как вы просили. Замечу, что по конвенции репозитория выполненные issue переносятся в issues/done/ (коммит «feat: done issue») — файл issues/callout component.md не трогал, оставляю это на
  ваш коммит.
