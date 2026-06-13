Также, как были добавлены ранее компоненты Button и Badge, необходимо добавить компонет Input с базовой начальной функциональностью и настройками стилей. Ориентироваться стоит на реализацию Input в `tmp/webawesome`. Для визуального тестирования работы необходим также новый раздел Input с примерами использования поля ввода в examples/client.

---

Done. The Input component is implemented, wired into the gallery, and the whole pipeline (library build, WASM client, wasm-bindgen bundling, static-asset copy, lint, fmt, and 21 tests) is green.

  What I added

  Library component — crates/lib/src/components/input.rs, following the same recipe as Button/Badge (#[derive(Default, AsRef, AsMut, Props)] + #[const_str(CLASS = "input")] + #[props(builder)], manual
  Renderable impl). Based on Web Awesome's wa-input, it covers the basic functionality:
  - InputType enum (Text, Email, Number, Password, Search, Tel, Url, Date, DatetimeLocal, Time) — same strum/const_into_str pattern as Variant/Appearance.
  - Reuses the shared Appearance (Outlined/Filled/FilledOutlined), plus pill, disabled, readonly, required, and name/value/placeholder/label/hint text props, plus the standard CommonAttrs (id/class/style).
  - Renders label → .text-field wrapper → native <input> → hint, matching Web Awesome's structure.
  - Registered in crates/lib/src/components.rs.
  
  Styles — web/style/components/input.css, adapted from input.styles.ts (shadow-DOM :host/::slotted selectors converted to plain .input ... classes, like the existing button.css/badge.css). Imported in
  web/style/index.css.

  Gallery section — examples/client/src/components/input.rs with an overview() page demonstrating labels, hint, placeholder, types, appearance, size, pill, and disabled/readonly/required. Registered in
  components.rs, routed in lib.rs ("input" => …), and added to the sidebar nav.

  Tests — crates/lib/src/tests/input.rs (5 snapshot-style tests) registered in tests.rs.

  Notes / decisions

  - I scoped to "basic functionality" per the issue and left out Web Awesome's interactive extras (clear button, password toggle, start/end slots, constraint validation) since those require web-component/JS
  behavior beyond this project's static-render model.
  - A default <Input /> emits class="input accent" because the shared Appearance defaults to Accent (same as Button/Badge). There's no .input.accent rule, so it falls back to the base .text-field style, which I
  styled to match Web Awesome's default outlined look — visually correct, and consistent with the existing component pattern the issue asked me to follow.

  To see it live: cargo make run, then open http://127.0.0.1:9080/input.
