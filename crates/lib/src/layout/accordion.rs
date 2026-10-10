use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome_ext;
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttributeSetters, CommonAttrs};
use crate::class::{
    ACCORDION, ACCORDION_ITEM, ACCORDION_ITEM_CONTENT, ACCORDION_ITEM_DEFAULT_ICON, ACCORDION_ITEM_HEADING,
    ACCORDION_ITEM_ICON, ACCORDION_ITEM_LABEL, ACCORDION_ITEM_PANEL, ACCORDION_ITEM_TRIGGER, DISABLED, EXPANDED,
};
use crate::component::head::{Head, HeadLevel};
use crate::icon_placement::ExpandIconPlacement;
use crate::span_component;

/// How the items of an accordion can be expanded.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum AccordionMode {
    /// Any number of items open at once; each toggles independently.
    #[default]
    Multiple,
    /// One item open at a time: opening another collapses it, and clicking the
    /// open item keeps it open.
    Single,
    /// Like `Single`, but clicking the open item collapses it, so zero open
    /// items is a valid state.
    SingleCollapsible,
}

impl AccordionMode {
    /// The `data-mode` value the client reads; the default mode is left out.
    pub const fn as_attr(self) -> Option<&'static str> {
        match self {
            Self::Multiple => None,
            Self::Single => Some("single"),
            Self::SingleCollapsible => Some("single-collapsible"),
        }
    }
}

/// A vertically stacked set of interactive headings, each revealing a section
/// of content, composed of [`AccordionItem`]s:
///
/// ```ignore
/// rsx! {
///     <Accordion mode=Single>
///         <AccordionItem label="First" expanded=true>"The first section."</AccordionItem>
///         <AccordionItem label="Second">"The second section."</AccordionItem>
///     </Accordion>
/// }
/// ```
///
/// The items render independently of the accordion, so the appearance and the
/// icon placement are classes on the accordion the item styles follow; the
/// heading level is set on each item. The expand/collapse behavior (animation,
/// modes, keyboard navigation, `wg-expand`/`wg-collapse` events) is implemented
/// in `wingy-hypertext-web` (`layout::accordion`) and must be wired up on the
/// client with `init_accordions`/`listen_accordions`.
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = ACCORDION)]
#[props(builder)]
pub struct Accordion<'a> {
    #[prop(impl_from)]
    pub appearance: Appearance,

    pub mode: AccordionMode,

    pub icon_placement: ExpandIconPlacement,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for Accordion<'a> {
    fn default() -> Self {
        Self {
            appearance: Appearance::Outlined,
            mode: AccordionMode::default(),
            icon_placement: ExpandIconPlacement::default(),
            attributes: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Renderable for Accordion<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line =
            self.class_line_with(&[Self::CLASS, self.appearance.into_str(), self.icon_placement.as_class()]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                data-mode=[self.mode.as_attr()]
                (self.get_attrs())
            >
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

/// An expandable section of an [`Accordion`]. With a `label` it renders the
/// trigger and the panel itself, and the children are the panel's content.
/// Without one, the children are rendered as is: compose the item from
/// [`AccordionItemTrigger`] and [`AccordionItemPanel`] to put markup in the
/// header or replace the icon.
///
/// When the item has an id, the trigger and the panel get `{id}-trigger` and
/// `{id}-panel` and refer to each other; otherwise the client links them.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = ACCORDION_ITEM)]
#[props(builder)]
pub struct AccordionItem<'a> {
    /// The label shown in the header.
    #[prop(into)]
    pub label: Option<Cow<'a, str>>,

    /// Renders the item already expanded.
    pub expanded: bool,

    /// Keeps the item from being toggled.
    pub disabled: bool,

    /// The heading level wrapping the trigger, `H3` by default; `NoHeading`
    /// for an accordion outside the document outline (inside a navigation, say).
    pub heading_level: Option<HeadLevel>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for AccordionItem<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.expanded { EXPANDED } else { "" },
            if self.disabled { DISABLED } else { "" },
        ]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                @if let Some(label) = &self.label {
                    @let trigger_id = id.map(|id| format!("{id}-trigger"));
                    @let panel_id = id.map(|id| format!("{id}-panel"));
                    @let cow_trigger_id = trigger_id.as_deref().map(Cow::Borrowed);
                    @let cow_panel_id = panel_id.as_deref().map(Cow::Borrowed);

                    <AccordionItemTrigger
                        id=(trigger_id.as_deref().unwrap_or_default())
                        expanded=(self.expanded)
                        disabled=(self.disabled)
                        self_heading_level=(self.heading_level)
                        self_panel_id=(cow_panel_id)
                    >
                        (label)
                    </AccordionItemTrigger>
                    <AccordionItemPanel
                        id=(panel_id.as_deref().unwrap_or_default())
                        expanded=(self.expanded)
                        self_trigger_id=(cow_trigger_id)
                    >
                        (self.children)
                    </AccordionItemPanel>
                } @else {
                    (self.children)
                }
            </div>
        }
        .render_to(buffer);
    }
}

/// The item's header: a button toggling the panel, wrapped in a heading. By
/// default its children are the label, followed by the expand/collapse icon.
/// With `bare` the children are rendered in the button as is: compose it from
/// [`AccordionItemLabel`] and [`AccordionItemIcon`] to replace the icon.
///
/// The common attributes (id, classes, styles) go to the button.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = ACCORDION_ITEM_TRIGGER)]
#[props(builder)]
pub struct AccordionItemTrigger<'a> {
    pub expanded: bool,

    pub disabled: bool,

    pub heading_level: Option<HeadLevel>,

    /// The id of the panel the button controls.
    #[prop(into)]
    pub panel_id: Option<Cow<'a, str>>,

    pub bare: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for AccordionItemTrigger<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        let expanded = if self.expanded { "true" } else { "false" };
        let disabled = if self.disabled { "true" } else { "false" };
        let tabindex = if self.disabled { "-1" } else { "0" };

        let button = rsx! {
            <button
                id=[id]
                class=[&class_line]
                style=[&style_line]
                type="button"
                aria-expanded=(expanded)
                aria-controls=[&self.panel_id]
                aria-disabled=(disabled)
                tabindex=(tabindex)
                (self.get_attrs())
            >
                @if self.bare {
                    (self.children)
                } @else {
                    <AccordionItemLabel>(self.children)</AccordionItemLabel>
                    <AccordionItemIcon/>
                }
            </button>
        };
        let heading_level = self.heading_level.unwrap_or(HeadLevel::H3);

        // The heading places the item in the document outline (the W3C
        // accordion pattern); the accordion inherits the surrounding font at
        // every level.
        rsx! {
            @if heading_level == HeadLevel::NoHeading {
                (button)
            } @else {
                <Head level=heading_level class=ACCORDION_ITEM_HEADING>(button)</Head>
            }
        }
        .render_to(buffer);
    }
}

span_component!(
    /// The item's label inside the trigger.
    AccordionItemLabel,
    ACCORDION_ITEM_LABEL
);

/// The item's expand/collapse icon, rotating as the item expands. Without
/// children it is the default chevron, mirrored in right-to-left text.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = ACCORDION_ITEM_ICON)]
#[props(builder)]
pub struct AccordionItemIcon<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for AccordionItemIcon<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let is_default = self.children.is_none();
        let class_line =
            self.class_line_with(&[Self::CLASS, if is_default { ACCORDION_ITEM_DEFAULT_ICON } else { "" }]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <span id=[id] class=[&class_line] style=[&style_line] aria-hidden="true" (self.get_attrs())>
                @if let Some(icon) = &self.children {
                    (icon)
                } @else {
                    (fontawesome_ext::regular::ChevronRight)
                }
            </span>
        }
        .render_to(buffer);
    }
}

/// The item's panel holding its content. A collapsed panel is
/// `hidden="until-found"`: out of the tab order and the accessibility tree,
/// yet a search in the page still finds and expands it.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = ACCORDION_ITEM_PANEL)]
#[props(builder)]
pub struct AccordionItemPanel<'a> {
    pub expanded: bool,

    /// The id of the trigger labeling the panel.
    #[prop(into)]
    pub trigger_id: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for AccordionItemPanel<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);
        let hidden = (!self.expanded).then_some("until-found");

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role="region"
                aria-labelledby=[&self.trigger_id]
                hidden=[hidden]
                (self.get_attrs())
            >
                <div class=ACCORDION_ITEM_CONTENT>(self.children)</div>
            </div>
        }
        .render_to(buffer);
    }
}
