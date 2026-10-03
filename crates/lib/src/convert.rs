#[inline]
pub fn bool_to_str(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

/// Renders a number the way the client-side behavior writes it back: without a
/// trailing `.0`, so `50` stays `50` in the markup and in the form data.
#[inline]
pub fn number_to_string(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// The percentage `value` takes between `min` and `max`, clamped to the track.
#[inline]
pub fn percentage(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        0.0
    } else {
        (((value - min) / (max - min)) * 100.0).clamp(0.0, 100.0)
    }
}

/// The number of UTF-16 code units in `value` — the length a browser counts for
/// `maxlength`, which is what the character count has to agree with.
#[inline]
pub fn utf16_len(value: &str) -> usize {
    value.chars().map(char::len_utf16).sum()
}
