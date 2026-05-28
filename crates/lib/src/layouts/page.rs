use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "page")]
#[props(builder)]
pub struct Page<R: Renderable = ()> {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for Page<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </div>
            <script>"set_page_header_height()"</script>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "page-header")]
#[props(builder)]
pub struct PageHeader<R: Renderable = ()> {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for PageHeader<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <header id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </header>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "page-body")]
#[props(builder)]
pub struct PageBody<R: Renderable = ()> {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for PageBody<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "page-menu")]
#[props(builder)]
pub struct PageMenu<R: Renderable = ()> {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for PageMenu<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "page-main")]
#[props(builder)]
pub struct PageMain<R: Renderable = ()> {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for PageMain<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <main id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </main>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "page-aside")]
#[props(builder)]
pub struct PageAside<R: Renderable = ()> {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for PageAside<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <aside id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </aside>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "page-footer")]
#[props(builder)]
pub struct PageFooter<R: Renderable = ()> {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for PageFooter<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <footer id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </footer>
        }
        .render_to(buffer);
    }
}
