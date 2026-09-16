В галерее перестал обновляться URL при переходе между разделами: контент подставляется, а `location.pathname` остаётся прежним. Заодно не работают «Назад»/«Вперёд» — записи в историю не пишутся вовсе, и первое же нажатие «Назад» уводит из приложения.

Расширение `client-router.ext.js` должно работать не хуже прежнего патча `client_patch.js`: все механизмы htmx обязаны отрабатывать так, как если бы ответ пришёл с сервера, а не от клиентского роутера.

---

● Починено.

## Причина

Коммит `fedf686` (23 августа) заменил перехват XHR (`client_patch.js`) на htmx-расширение. Расширение отменяло запрос на `htmx:configRequest` и подставляло содержимое само:

```js
event.preventDefault();
htmx.swap(target, router(path, event), { swapStyle: swap_spec });
```

htmx на отменённое `configRequest` выходит из `issueAjaxRequest` немедленно — в 2.0.8 это буквально:

```js
if (!ae(r, "htmx:configRequest", C)) { re(s); m(); return e }
```

Всё, что дальше по конвейеру, не выполняется. `hx-push-url` в том числе: он разбирается при обработке **ответа** и читает `t.pathInfo.finalRequestPath` / `responsePath`, то есть данные, которых без запроса не существует. А `htmx.swap()` — низкоуровневый API: подставляет содержимое и запускает settle, историю не трогает.

Вместе с историей отключилось и всё остальное: out-of-band-подстановки, `hx-select`, заголовки `HX-*`, индикаторы, события `afterRequest`/`afterOnLoad`, обработка ошибок.

Прежний патч подделывал сам XHR, поэтому htmx проходил конвейер целиком.

## Решение

Перехват перенесён на уровень транспорта — точка входа `htmx:beforeSend`. К этому моменту htmx уже собрал запрос, создал XHR и повесил на него свой `onload` со всей обработкой ответа. Расширение:

1. глушит отправку у **этого экземпляра** (`xhr.send = () => {}`), не трогая прототип — то, из-за чего когда-то ушли от глобального патча;
2. подменяет на нём `readyState`, `status`, `statusText`, `responseText`, `response`, `responseURL`, `getAllResponseHeaders`, `getResponseHeader`;
3. вызывает `xhr.onload()` через `setTimeout(0)` — настоящий ответ тоже не приходит внутри `send()`.

Дальше htmx работает как обычно: для него ответ пришёл. `responseURL` заполняется намеренно — из него htmx вычисляет `responsePath`, который и уходит в `hx-push-url`.

`htmx:beforeSend` подходит потому, что событие некэнселируемое и срабатывает **после** `beforeRequest` и добавления индикаторов, но **до** `send()`. То есть индикаторы ставятся и снимаются штатно.

### Промах кеша истории

Отдельная ветка `htmx:historyCacheMiss` была написана и **убрана после проверки исходника**: этот путь подставляет ответ в history-элемент, то есть в `body` (`historyElt: _t()`, `swapStyle: "innerHTML"`). Роутер отдаёт фрагмент для `.main-content` — подставив его в `body`, мы снесли бы всю оболочку. У старого патча был ровно тот же изъян.

Вместо этого в `main.js` выставлен `htmx.config.refreshOnHistoryMiss = true`: сервер отдаёт оболочку на любой односегментный путь, `main.js` рендерит `render_root(location.pathname)`, так что перезагрузка всегда даёт правильный результат. Расширение при этом остаётся общего назначения — политика промаха принадлежит приложению.

## Как проверялось

Headless Chromium, CDP.

**URL и история**

```
start          path='/'         main=''                len=2
click /badge   path='/badge'    main='Badge…'          len=3
click /switch  path='/switch'   main='Switch…'         len=4
click /tooltip path='/tooltip'  main='Tooltip…'        len=5
back           path='/switch'   main='Switch…'
back           path='/badge'    main='Badge…'
forward        path='/switch'   main='Switch…'
```

**Конвейер htmx целиком**

```
htmx:beforeRequest → beforeSend → beforeSwap → historyItemCreated →
pushedIntoHistory → afterSwap → afterRequest → afterOnLoad → afterSettle
```

Наружу не уходит ничего: `XMLHttpRequest.prototype.send` не вызывается ни разу, класс `htmx-request` снимается с элемента после запроса.

**Возможности, недоступные прежней ручной подстановке**

- out-of-band: в ответ дописан `<div id="oob-marker" hx-swap-oob="true">OOB OK</div>` — htmx нашёл и подставил его в отдельный элемент;
- `hx-select=".head"` — в цель попали только семь совпавших `.head`, а не весь фрагмент;
- `hx-swap="outerHTML"` — цель заменена целиком.

**Промах кеша истории** — принудительный (`pushState` на путь, который htmx не снимал): перезагрузка, оболочка на месте, раздел `/tooltip` отрисован.

**Регрессия мобильного меню** — ящик открывается, ссылка изнутри ведёт на `/dropdown` и обновляет URL, возврат на десктоп возвращает меню.

`cargo make lint-test` — чисто, 98 тестов.

## Оговорка

События прогресса (`htmx:xhr:loadstart`, `progress`, `loadend`) не возникают: для локального ответа их нечему порождать. Прежний патч их тоже не подавал, так что планка «не хуже» выдержана; при необходимости их можно диспатчить вручную.
