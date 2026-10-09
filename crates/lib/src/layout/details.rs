use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome_ext;
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttributeSetters, CommonAttrs};
use crate::class::{
    DETAILS, DETAILS_BODY, DETAILS_COLLAPSE_ICON, DETAILS_CONTENT, DETAILS_DEFAULT_ICON, DETAILS_EXPAND_ICON,
    DETAILS_HEADER, DETAILS_ICON, DETAILS_SUMMARY, DISABLED, OPEN,
};
use crate::icon_placement::ExpandIconPlacement;

/// A brief summary that expands to reveal additional content: progressively
/// disclosed information, grouped FAQs, hidden advanced options:
///
/// ```ignore
/// rsx! {
///     <Details summary="Toggle Me">
///         "Click the summary to expand and collapse the details."
///     </Details>
/// }
/// ```
///
/// Rendered as a native `<details>`, so it opens and closes, groups by `name`
/// and is announced by assistive technology even before the client code runs.
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = DETAILS)]
#[props(builder)]
pub struct Details<'a> {
    #[prop(impl_from)]
    pub appearance: Appearance,

    /// Renders the details already expanded.
    pub open: bool,

    /// Groups related details: when one opens, the others with the same name close.
    #[prop(into)]
    pub name: Option<Cow<'a, str>>,

    /// Keeps the details from being toggled.
    pub disabled: bool,

    pub icon_placement: ExpandIconPlacement,

    /// The summary shown in the header.
    #[prop(into)]
    pub summary: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for Details<'a> {
    fn default() -> Self {
        Self {
            appearance: Appearance::Outlined,
            open: false,
            name: None,
            disabled: false,
            icon_placement: ExpandIconPlacement::default(),
            summary: None,
            attributes: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Renderable for Details<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            self.appearance.into_str(),
            self.icon_placement.as_class(),
            if self.open { OPEN } else { "" },
            if self.disabled { DISABLED } else { "" },
        ]);
        let style_line = self.style_line_with(&[]);
        let open = self.open.then_some("");

        rsx! {
            <details
                id=[id]
                class=[&class_line]
                style=[&style_line]
                name=[&self.name]
                open=[open]
                (self.get_attrs())
            >
                @if let Some(summary) = &self.summary {
                    @let header_id = id.map(|id| format!("{id}-header"));
                    @let body_id = id.map(|id| format!("{id}-body"));
                    @let cow_header_id = header_id.as_deref().map(Cow::Borrowed);
                    @let cow_body_id = body_id.as_deref().map(Cow::Borrowed);

                    <DetailsHeader
                        id=(header_id.as_deref().unwrap_or_default())
                        open=(self.open)
                        self_body_id=(cow_body_id)
                        disabled=(self.disabled)
                    >
                        (summary)
                    </DetailsHeader>
                    <DetailsBody
                        id=(body_id.as_deref().unwrap_or_default())
                        self_header_id=(cow_header_id)
                    >
                        (self.children)
                    </DetailsBody>
                } @else {
                    (self.children)
                }
            </details>
        }
        .render_to(buffer);
    }
}

#[derive(AsRef, AsMut, Default, Props)]
#[const_str(CLASS = DETAILS_HEADER)]
#[props(builder)]
pub struct DetailsHeader<'a> {
    /// Renders the details already expanded.
    pub open: bool,

    #[prop(into)]
    pub body_id: Option<Cow<'a, str>>,

    /// Keeps the details from being toggled.
    pub disabled: bool,

    pub bare: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DetailsHeader<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        let expanded = if self.open { "true" } else { "false" };
        let disabled = self.disabled.then_some("true");
        let tabindex = self.disabled.then_some("-1");

        rsx! {
            <summary
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role="button"
                aria-expanded=(expanded)
                aria-controls=[&self.body_id]
                aria-disabled=[disabled]
                tabindex=[tabindex]
                (self.get_attrs())
            >
                @if self.bare {
                    (self.children)
                } @else {
                    <span class=DETAILS_SUMMARY>(self.children)</span>
                    <span class=DETAILS_ICON aria-hidden="true">
                        <span class=(DETAILS_EXPAND_ICON, " ", DETAILS_DEFAULT_ICON)>
                            (fontawesome_ext::regular::ChevronRight)
                        </span>
                        <span class=(DETAILS_COLLAPSE_ICON, " ", DETAILS_DEFAULT_ICON)>
                            (fontawesome_ext::regular::ChevronRight)
                        </span>
                    </span>
                }
            </summary>
        }
        .render_to(buffer);
    }
}

#[derive(AsRef, AsMut, Default, Props)]
#[const_str(CLASS = DETAILS_BODY)]
#[props(builder)]
pub struct DetailsBody<'a> {
    #[prop(into)]
    pub header_id: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DetailsBody<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role="region"
                aria-labelledby=[&self.header_id]
                (self.get_attrs())
            >
                <div class=DETAILS_CONTENT>(self.children)</div>
            </div>
        }
        .render_to(buffer);
    }
}
