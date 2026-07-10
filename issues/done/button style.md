В новой версии WebAwesome (см. `tmp/webawesome`) компонент Button имеет отличный стиль нажатия от того, что реализован сейчас в `wingy_hypertext::components::button::Button`: иначе кнопка ведёт себя при наведении и при нажатии она как-бы немного углубляется. Необходимо привести в соответсвие компонет `Button` чтобы поведение при наведении и нажатии было аналогичным тому, что реализовано сейчас в WebAwesome.

---

● Задача выполнена. Поведение кнопки при наведении и нажатии приведено к новой версии Web Awesome — эффект «вдавливания» реализован через transform с темовыми токенами, Rust-код не менялся, только CSS.

  Что именно отличалось: в новой версии WA (button.styles.ts) кнопка при hover/active дополнительно трансформируется через токены --wa-button-transform-hover/--wa-button-transform-active, а transform участвует
  в transition — при нажатии кнопка слегка сжимается (scale(0.9875) в теме default). В локальном порте этого не было вовсе, как и самих токенов в темах.

  Изменения:

  - webassets/style/components/button.css — в базовое правило .button добавлены transform в transition-property и transform-origin: center; добавлен блок «Hover and active transforms» (точный порт из upstream):
  для .button:not(.disabled):not(.loading) на hover применяется var(--wa-button-transform-hover) (под @media (hover: hover)), на active — var(--wa-button-transform-active), и оба сбрасываются в none при
  prefers-reduced-motion: reduce.
  - Токены добавлены во все три локальные темы (блок /* Buttons */ перед /* Panels */, как в upstream), значения из соответствующих тем WA:
    - themes/default.css — hover none, active scale(0.9875) (именно это даёт «углубление»; эту тему использует галерея);
    - themes/awesome.css — active translate(var(--wa-shadow-offset-x-s), var(--wa-shadow-offset-y-s)) (нужные shadow-offset-токены в локальной теме уже есть);
    - themes/shoelace.css — оба none.

  Статика пересобрана (cargo make client), обновлённый CSS в target/web проверен.

  Замечание по границе задачи: upstream добавил такой же transform и в native.css для нативных <button>/a.wa-button — локальный native.css я не трогал, так как задача про компонент Button. Если нужно
  синхронизировать и нативные кнопки, скажите.

---

Да, синхронизируй и для native

---

● Синхронизировал. В webassets/style/common/native.css блок дефолтных стилей нативных кнопок (button, input[type=button/reset/submit], a.wa-button) теперь повторяет upstream:
  
  - :hover обёрнут в @media (hover: hover) и получил transform: var(--wa-button-transform-hover);
  - :active получил transform: var(--wa-button-transform-active) — то же «вдавливание», что и у компонента;
  - добавлен сброс transform: none для обоих состояний при prefers-reduced-motion: reduce.
  
  Как и в upstream, transform применяется только в дефолтном блоке (модификаторы .wa-plain/.wa-outlined/и т.д. наследуют его, переопределяя лишь цвета), transition-property и transform-origin в native.css не
  трогал — их там нет и в новой версии WA. Кнопки file-selector тоже без transform, в точности как в референсе.
  
  Статика пересобрана, обновлённый native.css в target/web проверен — токены на месте.

---

Проверь и другие стили, относящиеся к кнопкам (цвет, фон и так далее) и если есть расхождения с актуальными в `tmp/webawesome` - поправь и их тоже.

---

● Сравню остальные стили кнопок построчно. Дочитаю локальный регион Buttons в native.css:
● Теперь native.css — радиусы с групповыми переменными, удаление line-height, групповые отступы:
● Пересоберу статику и проверю результат:
● Проверил все стили кнопок против актуального tmp/webawesome. Цвета и фоны (базовые и всех модификаторов plain/outlined/filled/filled-outlined/accent, включая hover/active color-mix) уже полностью совпадали —
  и в components/button.css, и в native.css. Расхождения нашлись в другом, поправил:
  
  webassets/style/components/button.css:
  - Удалён line-height: calc(...) — в новой версии WA его нет, центрирование делает flex.
  - .start/.end получили недостающие flex: 0 0 auto; display: flex; align-items: center; pointer-events: none; (как у upstream), отступы 0.75em сохранены.
  
  webassets/style/common/native.css (регион Buttons):
  - Удалён line-height: calc(var(--wa-form-control-height) - var(--border-width) * 2) — он к тому же ссылался на несуществующую переменную --border-width.
  - Одиночный border-radius заменён на четыре логических радиуса с групповыми переменными --_button-*-radius (нужно для группировки кнопок; при их отсутствии — прежний fallback --wa-form-control-border-radius).
  То же для .wa-pill с fallback --wa-border-radius-pill.
  - Добавлены групповые отступы margin-inline-start/margin-block-start: var(--_button-horizontal/vertical-indent) в базовый блок и отдельный override для .wa-outlined — для схлопывания смежных рамок в группах,
  как в upstream. Без групповых переменных margins остаются нулевыми, одиночные кнопки не меняются.
  
  Статика пересобрана и проверена в target/web.
  
  Что не синхронизировал сознательно (структурные отличия, требуют изменений разметки/компонента, а не CSS):
  - width: 100% на .button — в WA он нужен из-за обёртки :host в shadow DOM, у нас .button и есть сам элемент;
  - .is-icon-button (квадратные иконочные кнопки) — у нас нет такого признака в разметке, действует локальная адаптация .button:has(.icon) { padding: 0 0.75em };
  - стили .loading со спиннером — проп loading у компонента Button пока вообще не выводится в класс при рендере (заготовка);
  - правила для wa-badge/caret/.label — таких слотов в нашей разметке нет.
  
  Если какой-то из этих пунктов (например, loading или иконочные кнопки) хотите довести до соответствия WA — это отдельная доработка компонента, могу заняться.

---

Проверь соответствия цветов кнопки при нажатии. В демо WebAwesome цвета иные, ни как сейчас в компоненте Button в данном проекте.

---

● Сравню токены --wa-color-mix-active/--wa-color-mix-hover и цветовые палитры в локальной теме с upstream:
● Проверю, нет ли других расхождений в файлах тем (палитры и пр.) — сделаю полный diff:
● Нашёл и исправил причину. Цвета нажатия отличались из-за устаревших токенов --wa-color-mix-hover/--wa-color-mix-active в локальных темах: старая версия WA замешивала фон с чёрным (black 10%/20%), из-за чего
  нажатая кнопка всегда темнела. В актуальной версии логика другая:
  
  - hover — микс с инвертированным по светлоте текущим цветом: oklch(from currentColor calc(1 - l) c h);
  - active — микс с цветом поверхности var(--wa-color-surface-default), то есть при нажатии кнопка теперь слегка осветляется к фону (в светлой теме), а не затемняется — именно та разница, что видна в демо
  WebAwesome.
  
  Обновил все три локальные темы, светлую и тёмную схемы, значениями из соответствующих upstream-тем:
  
  ┌──────────┬────────────────────────────┬────────────────────────────┐
  │   Тема   │    Light hover / active    │    Dark hover / active     │
  ├──────────┼────────────────────────────┼────────────────────────────┤
  │ default  │ инверсия 10% / surface 10% │ инверсия 20% / surface 20% │
  ├──────────┼────────────────────────────┼────────────────────────────┤
  │ awesome  │ инверсия 15% / surface 10% │ инверсия 20% / surface 20% │
  ├──────────┼────────────────────────────┼────────────────────────────┤
  │ shoelace │ l + 0.1 10% / l − 0.1 10%  │ l − 0.1 15% / l + 0.1 15%  │
  └──────────┴────────────────────────────┴────────────────────────────┘
  
  Эти токены используются во всех hover/active color-mix(...) кнопок (и компонентных, и нативных), так что правка тем покрывает всё сразу.
  
  Также проверил остальное: оставшиеся миксы с black (--wa-color-surface-lowered, --wa-color-overlay-modal) совпадают с upstream — это не дрейф. Прочие расхождения в темах не про кнопки и не про цвета
  (отсутствующие токены --wa-font-size-3xs/5xl, --wa-space-5xl, правила для wa-page, стили лейблов wa-button в shoelace) — их не трогал; блок теней кнопок в awesome-теме эквивалентен upstream с точностью до
  формы селекторов. Галерея использует тему default — эффект будет виден сразу. Статика пересобрана и проверена.
