Необходимо реализовать в новом крейте `crates/web` Rust-альтернативу JS-скриптам из `webassets/js`. Код на Rust должен компилироваться в WASM и должен быть максимально близок к JS-прототипу и должен быть способен заменить эти JS-скрипты на клиенте. Можно использовать дополнительные крейты (такие как web-sys и js-sys, а также другие) для упрощения реализации вызовов Web API из Rust. Пример в `examples/client` должен быть переведён на использование новых альтернативных функций из Rust, вместо старых JS-модулей.

---

● A scaffold exists (empty lib.rs, package wingy-hypertext-web). Let me rewrite the manifest with the needed deps, then build out the modules.

● Готово. Задача impl js scripts in rust.md выполнена и перенесена в issues/done/.

  Что сделано

  Новый крейт crates/web (wingy-hypertext-web) — Rust/WASM-порт клиентских скриптов, по одному модулю на каждый JS-файл из webassets/js, максимально близко к прототипу:

  ┌───────────────────────────┬────────────────────────────────────────────────────────────────────────────────────────┐
  │        JS-прототип        │                                      Rust-модуль                                       │
  ├───────────────────────────┼────────────────────────────────────────────────────────────────────────────────────────┤
  │ utils/animate.js          │ utils/animate.rs — animate, animate_with_class, parse_duration, prefers_reduced_motion │
  ├───────────────────────────┼────────────────────────────────────────────────────────────────────────────────────────┤
  │ utils/action.js           │ utils/action.rs — реестр действий, register/run/dispatch_action, listen_click_actions  │
  ├───────────────────────────┼────────────────────────────────────────────────────────────────────────────────────────┤
  │ components/copy_button.js │ components/copy_button.rs — handle_copy, show_status, register_copy_action             │
  ├───────────────────────────┼────────────────────────────────────────────────────────────────────────────────────────┤
  │ components/head.js        │ components/head.rs — init_scroll_to_anchor                                             │
  ├───────────────────────────┼────────────────────────────────────────────────────────────────────────────────────────┤
  │ layouts/page.js           │ layouts/page.rs — set_page_header_height, init_page_element                            │
  ├───────────────────────────┼────────────────────────────────────────────────────────────────────────────────────────┤
  │ layouts/code_example.js   │ layouts/code_example.rs — drag-resize, анимация раскрытия, init/listen_code_examples   │
  └───────────────────────────┴────────────────────────────────────────────────────────────────────────────────────────┘

  Использованы web-sys/js-sys/wasm-bindgen-futures. Плюс src/dom.rs с хелперами (window, next_animation_frame, set_timeout и т.п.).

  Перевод примера: examples/client теперь зависит от нового крейта и экспортирует тонкие #[wasm_bindgen]-обёртки (register_copy_action, init_code_examples, listen_code_examples, listen_click_actions,
  init_page_element, init_scroll_to_anchor). main.js вызывает их вместо import './js/...'. На JS остались только сторонние вендоры (htmx, highlight.js) и перехват htmx-запросов (client_patch.js).

  Старый JS-функционал оставлен в webassets/js/ без изменений — как вы просили.

  Сборка: биндинги Web Animations в web-sys пока за --cfg=web_sys_unstable_apis. Флаг добавлен в .cargo/config.toml и продублирован в RUSTFLAGS внутри Makefile.toml (env-переменная иначе перекрывает config).

  Проверки

  - cargo make client — собирается, в target/web/dist/client.js присутствуют все 6 новых функций + render_root/request.
  - cargo make lint-test — nightly fmt --check, clippy -D warnings, тесты (24 passed) — всё зелёное.

  CLAUDE.md дополнён описанием нового крейта и нюанса с web_sys_unstable_apis.

  Одно осознанное отклонение от прототипа: в copy_button JS читал window.getElementById (всегда undefined — баг), в Rust-порте использован document.getElementById, чтобы функция реально работала. Помечено
  комментарием в коде.

  Я не запускал галерею в браузере вживую — поведение кнопок/анимаций стоит проверить вручную через cargo make run (http://127.0.0.1:9080). Хотите, запущу сервер?

---

Убрал .cargo/config.toml, добавил choose_profile_web в Makefile.

