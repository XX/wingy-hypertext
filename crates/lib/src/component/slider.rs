use std::borrow::Cow;
use std::ops::RangeInclusive;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    DISABLED, HINT, LABEL, SLIDER, SLIDER_CONTROL, SLIDER_INDICATOR, SLIDER_MARKER, SLIDER_MARKERS, SLIDER_REFERENCES,
    SLIDER_THUMB, SLIDER_THUMB_MAX, SLIDER_THUMB_MIN, SLIDER_TRACK,
};
use crate::component::tooltip::{Tooltip, TooltipTrigger, TooltipTriggers};
use crate::convert::{bool_to_str, number_to_string, percentage};
use crate::helper::popup::PopupPlacement;
use crate::orientation::Orientation;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SliderTooltip {
    pub placement: PopupPlacement,
    pub distance: i32,
}

impl Default for SliderTooltip {
    fn default() -> Self {
        Self {
            placement: PopupPlacement::Top,
            distance: 8,
        }
    }
}

impl SliderTooltip {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_placement(mut self, placement: PopupPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn with_distance(mut self, distance: i32) -> Self {
        self.distance = distance;
        self
    }
}

/// A slider lets the user choose a number — or a range of numbers — by dragging
/// a thumb along a track:
///
/// ```ignore
/// rsx! {
///     <Slider label="Volume" min=0.0 max=100.0 value=50.0 name="volume"/>
/// }
/// ```
///
/// The positions of the thumb, of the filled part of the track and of the
/// markers are rendered server-side, so the slider looks right before any
/// script runs. Dragging, the keyboard and the tooltip are implemented in
/// `wingy-hypertext-web` (`component::slider`) and wired up with
/// `init_sliders`/`listen_sliders`.
///
/// The value is submitted by a hidden input, since the control itself isn't a
/// native one: a range slider submits `{name}-min` and `{name}-max`.
///
/// `children` are the references shown along the track — any elements, spread
/// from the start of the track to its end.
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = SLIDER)]
#[props(builder)]
pub struct Slider<'a> {
    pub value: f64,

    pub min: f64,

    pub max: f64,

    /// The granularity the value must adhere to when incrementing and
    /// decrementing. It also drives the markers.
    pub step: f64,

    /// Turns the slider into a range selection with two thumbs.
    pub range: Option<RangeInclusive<f64>>,

    /// The value the filled part of the track starts from. Defaults to `min`,
    /// so the track fills from its start.
    pub indicator_offset: Option<f64>,

    #[prop(impl_from)]
    pub orientation: Orientation,

    pub disabled: bool,

    /// Blocks dragging and the keyboard, but keeps the slider focusable and
    /// readable. Native form controls ignore `readonly`, so it is applied by
    /// the client-side behavior.
    pub readonly: bool,

    /// Draws a marker at every step along the track.
    pub markers: bool,

    /// Shows a tooltip with the current value while the slider is focused or
    /// dragged. It anchors to the thumb, so the slider needs an `id`.
    pub tooltip: Option<SliderTooltip>,

    /// The name the value is submitted under.
    #[prop(into)]
    pub name: Option<Cow<'a, str>>,

    #[prop(into)]
    pub label: Option<Cow<'a, str>>,

    #[prop(into)]
    pub hint: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// The references shown along the track.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for Slider<'a> {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            range: None,
            indicator_offset: None,
            orientation: Orientation::Horizontal,
            disabled: false,
            readonly: false,
            markers: false,
            tooltip: None,
            name: None,
            label: None,
            hint: None,
            attributes: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Slider<'a> {
    /// The positions of the markers, one per step.
    fn marker_positions(&self) -> Vec<f64> {
        if !self.markers || self.step <= 0.0 || self.max <= self.min {
            return Vec::new();
        }

        let steps = ((self.max - self.min) / self.step).floor() as usize;
        (0..=steps)
            .map(|index| percentage(self.step.mul_add(index as f64, self.min), self.min, self.max))
            .collect()
    }

    /// The filled part of the track, as the start and end percentages the
    /// stylesheet reads.
    fn indicator(&self) -> (f64, f64) {
        if let Some(range) = &self.range {
            let min = percentage(*range.start(), self.min, self.max);
            let max = percentage(*range.end(), self.min, self.max);
            (min.min(max), min.max(max))
        } else {
            let offset = percentage(self.indicator_offset.unwrap_or(self.min), self.min, self.max);
            let value = percentage(self.value, self.min, self.max);
            (offset.min(value), offset.max(value))
        }
    }
}

impl<'a> Renderable for Slider<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            self.orientation.into_str(),
            if self.disabled { DISABLED } else { "" },
        ]);
        let style_line = self.style_line_with(&[]);

        let (start, end) = self.indicator();
        let markers = self.marker_positions();

        // The thumbs of a range slider carry the slider role, one per value
        let thumb_id = |suffix: &str| id.map(|id| format!("{id}{suffix}"));
        let min_thumb_id = thumb_id("-thumb-min");
        let max_thumb_id = thumb_id("-thumb-max");
        let single_thumb_id = thumb_id("-thumb");
        // The hint is announced with the value, so it needs an id to point at
        let hint_id = self.hint.as_ref().and_then(|_| thumb_id("-hint"));
        // `&str` copies into the track markup below, leaving the owned ids for the tooltips
        let (min_thumb, max_thumb, single_thumb, hint_anchor) = (
            min_thumb_id.as_deref(),
            max_thumb_id.as_deref(),
            single_thumb_id.as_deref(),
            hint_id.as_deref(),
        );

        let track = rsx! {
            <div class=SLIDER_TRACK>
                <div class=SLIDER_INDICATOR style=(format!("--start: {start}%; --end: {end}%"))></div>
                @if !markers.is_empty() {
                    <div class=SLIDER_MARKERS aria-hidden="true">
                        @for marker in &markers {
                            <span class=SLIDER_MARKER style=(format!("--position: {marker}%"))></span>
                        }
                    </div>
                }
                @if let Some(range) = &self.range {
                    @let min_position = percentage(*range.start(), self.min, self.max);
                    @let max_position = percentage(*range.end(), self.min, self.max);
                    <span
                        id=[min_thumb]
                        class=(SLIDER_THUMB, " ", SLIDER_THUMB_MIN)
                        style=(format!("--position: {min_position}%"))
                        role="slider"
                        tabindex=(if self.disabled { "-1" } else { "0" })
                        aria-valuemin=(number_to_string(self.min))
                        aria-valuenow=(number_to_string(*range.start()))
                        aria-valuetext=(number_to_string(*range.start()))
                        aria-valuemax=(number_to_string(self.max))
                        aria-describedby=[hint_anchor]
                        aria-label=(self.label.as_deref().map_or_else(
                            || "Minimum value".to_string(),
                            |label| format!("{label} (minimum value)"),
                        ))
                        aria-orientation=(self.orientation.into_str())
                        aria-disabled=(bool_to_str(self.disabled))
                        aria-readonly=(bool_to_str(self.readonly))
                    ></span>
                    <span
                        id=[max_thumb]
                        class=(SLIDER_THUMB, " ", SLIDER_THUMB_MAX)
                        style=(format!("--position: {max_position}%"))
                        role="slider"
                        tabindex=(if self.disabled { "-1" } else { "0" })
                        aria-valuemin=(number_to_string(self.min))
                        aria-valuenow=(number_to_string(*range.end()))
                        aria-valuetext=(number_to_string(*range.end()))
                        aria-valuemax=(number_to_string(self.max))
                        aria-describedby=[hint_anchor]
                        aria-label=(self.label.as_deref().map_or_else(
                            || "Maximum value".to_string(),
                            |label| format!("{label} (maximum value)"),
                        ))
                        aria-orientation=(self.orientation.into_str())
                        aria-disabled=(bool_to_str(self.disabled))
                        aria-readonly=(bool_to_str(self.readonly))
                    ></span>
                } @else {
                    @let position = percentage(self.value, self.min, self.max);
                    <span
                        id=[single_thumb]
                        class=SLIDER_THUMB
                        style=(format!("--position: {position}%"))
                    ></span>
                }
            </div>
        };

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                data-min=(number_to_string(self.min))
                data-max=(number_to_string(self.max))
                data-step=(number_to_string(self.step))
                data-indicator-offset=[self.indicator_offset.map(number_to_string)]
                data-readonly=[self.readonly.then_some("")]
                (self.get_attrs())
            >
                @if let Some(label) = &self.label {
                    <label class=LABEL>(label)</label>
                }
                @if let Some(range) = &self.range {
                    <div class=SLIDER_CONTROL>
                        (&track)
                        @if let Some(children) = self.children {
                            <div class=SLIDER_REFERENCES aria-hidden="true">(children)</div>
                        }
                    </div>
                    <input type="hidden" name=[self.name.as_ref().map(|name| format!("{name}-min"))]
                        value=(number_to_string(*range.start()))/>
                    <input type="hidden" name=[self.name.as_ref().map(|name| format!("{name}-max"))]
                        value=(number_to_string(*range.end()))/>
                } @else {
                    <div
                        class=SLIDER_CONTROL
                        role="slider"
                        tabindex=(if self.disabled { "-1" } else { "0" })
                        aria-valuemin=(number_to_string(self.min))
                        aria-valuenow=(number_to_string(self.value))
                        aria-valuetext=(number_to_string(self.value))
                        aria-valuemax=(number_to_string(self.max))
                        aria-describedby=[hint_anchor]
                        aria-label=[&self.label]
                        aria-orientation=(self.orientation.into_str())
                        aria-disabled=(bool_to_str(self.disabled))
                        aria-readonly=(bool_to_str(self.readonly))
                    >
                        (&track)
                        @if let Some(children) = self.children {
                            <div class=SLIDER_REFERENCES aria-hidden="true">(children)</div>
                        }
                    </div>
                    <input type="hidden" name=[&self.name] value=(number_to_string(self.value))/>
                }
                @if let Some(hint) = &self.hint {
                    <small id=[hint_anchor] class=HINT>(hint)</small>
                }
                // The tooltips anchor to the thumbs by id, so they need one
                @if let Some(tooltip) = self.tooltip {
                    @if let Some(range) = &self.range {
                        @if let Some(anchor) = min_thumb {
                            @let text = number_to_string(*range.start());
                            <Tooltip
                                anchor_id=(anchor)
                                placement=(tooltip.placement)
                                distance=(tooltip.distance)
                                trigger=(TooltipTriggers::from(TooltipTrigger::Manual))
                            >
                                (&text)
                            </Tooltip>
                        }
                        @if let Some(anchor) = max_thumb {
                            @let text = number_to_string(*range.end());
                            <Tooltip
                                anchor_id=(anchor)
                                placement=(tooltip.placement)
                                distance=(tooltip.distance)
                                trigger=(TooltipTriggers::from(TooltipTrigger::Manual))
                            >
                                (&text)
                            </Tooltip>
                        }
                    } @else if let Some(anchor) = single_thumb {
                        @let text = number_to_string(self.value);
                        <Tooltip
                            anchor_id=(anchor)
                            placement=(tooltip.placement)
                            distance=(tooltip.distance)
                            trigger=(TooltipTriggers::from(TooltipTrigger::Manual))
                        >
                            (&text)
                        </Tooltip>
                    }
                }
            </div>
        }
        .render_to(buffer);
    }
}
