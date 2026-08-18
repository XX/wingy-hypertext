use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use strum::{AsRefStr, IntoStaticStr};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::ANIMATION;

/// The direction of playback as well as the behavior when reaching the end of
/// an iteration, mirroring the CSS `animation-direction` values.
#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum Direction {
    #[default]
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

/// How the animation applies styles to its target before and after its execution.
#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum Fill {
    #[default]
    Auto,
    None,
    Forwards,
    Backwards,
    Both,
}

/// An animation primitive mirroring Web Awesome's `wa-animation`: animates its
/// first element child declaratively with one of the nearly 100 baked-in
/// presets, or custom keyframes, powered by the Web Animations API. The
/// playback logic is implemented in `wingy-hypertext-web` (`helpers::animation`)
/// and must be wired up on the client with `init_animations`; the configuration
/// is carried by `data-*` attributes on the host element.
///
/// Like `wa-animation`, only the first element child is animated — to animate
/// multiple elements, either wrap them in a single container or use multiple
/// `Animation`s. The animation does not start until `play` is set; the
/// attribute is automatically removed when the animation finishes or gets
/// canceled.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = ANIMATION)]
#[props(builder)]
pub struct Animation<'a> {
    /// The name of the built-in animation to use. For custom animations, use `keyframes`.
    #[prop(into)]
    pub name: Option<Cow<'a, str>>,

    /// Plays the animation. When omitted, the animation will be paused.
    pub play: bool,

    /// The number of milliseconds to delay the start of the animation.
    pub delay: Option<i32>,

    pub direction: Option<Direction>,

    /// The number of milliseconds each iteration of the animation takes to
    /// complete. The default is `1000`.
    pub duration: Option<i32>,

    /// The easing function to use for the animation. This can be a baked-in
    /// easing name (e.g. `easeInOutCubic`) or a custom easing function such as
    /// `cubic-bezier(0, 1, .76, 1.14)`.
    #[prop(into)]
    pub easing: Option<Cow<'a, str>>,

    /// The number of milliseconds to delay after the active period of an animation sequence.
    pub end_delay: Option<i32>,

    pub fill: Option<Fill>,

    /// The number of iterations to run before the animation completes.
    /// Defaults to infinity, which loops.
    pub iterations: Option<f64>,

    /// The offset at which to start the animation, usually between 0 (start) and 1 (end).
    pub iteration_start: Option<f64>,

    /// The keyframes to use for the animation, as a JSON array of keyframe
    /// objects. If this is set, `name` will be ignored.
    #[prop(into)]
    pub keyframes: Option<Cow<'a, str>>,

    /// Sets the animation's playback rate. The default is `1`; a negative
    /// value can be used to reverse the animation. This value can be changed
    /// without causing the animation to restart.
    pub playback_rate: Option<f64>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// The element to animate. Rendered as the content of the host element;
    /// only the first element child is animated.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Animation<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        let play = self.play.then_some("");

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                data-name=[&self.name]
                data-play=[play]
                data-delay=[self.delay]
                data-direction=[self.direction.map(|direction| direction.into_str())]
                data-duration=[self.duration]
                data-easing=[&self.easing]
                data-end-delay=[self.end_delay]
                data-fill=[self.fill.map(|fill| fill.into_str())]
                data-iterations=[self.iterations]
                data-iteration-start=[self.iteration_start]
                data-keyframes=[&self.keyframes]
                data-playback-rate=[self.playback_rate]
                (self.get_attrs())
            >
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}
