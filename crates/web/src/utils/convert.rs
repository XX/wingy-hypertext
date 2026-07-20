pub use wingy_hypertext::convert::*;

/// Mimics JavaScript's `parseFloat`: reads a leading (optionally signed) decimal number, defaulting to 0.
pub fn parse_float(input: impl AsRef<str>) -> f64 {
    let trimmed = input.as_ref().trim_start();
    let bytes = trimmed.as_bytes();
    let mut end = 0;
    let mut seen_dot = false;

    if let Some(&first) = bytes.first()
        && (first == b'+' || first == b'-')
    {
        end += 1;
    }

    while let Some(&byte) = bytes.get(end) {
        match byte {
            b'0'..=b'9' => end += 1,
            b'.' if !seen_dot => {
                seen_dot = true;
                end += 1;
            },
            _ => break,
        }
    }

    trimmed[..end].parse::<f64>().unwrap_or(0.0)
}

/// Parses a CSS duration and returns the number of milliseconds.
pub fn parse_duration_millis(duration: &str) -> f64 {
    let duration = duration.trim().to_lowercase();

    if duration.contains("ms") {
        return parse_float(&duration);
    }

    if duration.contains('s') {
        return parse_float(&duration) * 1000.0;
    }

    parse_float(&duration)
}
