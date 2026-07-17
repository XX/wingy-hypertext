use hypertext::prelude::hypertext_elements;
use hypertext::{Buffer, Renderable, RenderableExt, rsx};
use wingy_hypertext_macros::DynRenderable;

#[derive(Default, DynRenderable)]
struct TwoSlots<A: Renderable = (), R: Renderable = ()> {
    first: Option<A>,
    second: Option<R>,
}

impl<A: Renderable, R: Renderable> TwoSlots<A, R> {
    fn render_to(&self, buffer: &mut Buffer, first: Option<&dyn Renderable>, second: Option<&dyn Renderable>) {
        rsx! {
            <div>(first)"|"(second)</div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, DynRenderable)]
#[render_to(render_plain)]
struct Plain {
    text: &'static str,
}

impl Plain {
    fn render_plain(&self, buffer: &mut Buffer) {
        rsx! {
            <span>(self.text)</span>
        }
        .render_to(buffer);
    }
}

#[derive(Default, DynRenderable)]
struct WithSkipped<A: Renderable = (), R: Renderable = ()> {
    #[skip_render]
    ignored: Option<A>,
    children: Option<R>,
}

impl<A: Renderable, R: Renderable> WithSkipped<A, R> {
    fn render_to(&self, buffer: &mut Buffer, children: Option<&dyn Renderable>) {
        let marker = if self.ignored.is_some() { "+" } else { "-" };
        rsx! {
            <div>(marker)(children)</div>
        }
        .render_to(buffer);
    }
}

#[test]
fn erases_fields_in_declaration_order() {
    let two_slots = TwoSlots {
        first: Some(rsx! { "a" }),
        second: Some(rsx! { "b" }),
    };
    assert_eq!(two_slots.render().as_inner(), "<div>a|b</div>");
}

#[test]
fn delegates_to_custom_method() {
    let plain = Plain { text: "hi" };
    assert_eq!(plain.render().as_inner(), "<span>hi</span>");
}

#[test]
fn skip_render_field_is_not_passed_to_the_delegate() {
    let with_skipped = WithSkipped {
        ignored: Some(rsx! { "a" }),
        children: Some(rsx! { "b" }),
    };
    assert_eq!(with_skipped.render().as_inner(), "<div>+b</div>");
}
