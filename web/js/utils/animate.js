/** Same as `el.animate()`, except returns a promise that doesn't throw an error when the animation is canceled. */
export async function animate(el, keyframes, options) {
    return el.animate(keyframes, options).finished.catch(() => {
        /* suppress errors in Safari */
    });
}

/**
 * Applies a class to the specified element to animate it.
 * The class is removed after the animation finishes and then the promise resolves.
 */
export function animate_with_class(el, class_name) {
    return new Promise(resolve => {
        const controller = new AbortController();
        const signal = controller.signal;

        if (el.classList.contains(class_name)) {
            return;
        }

        el.classList.add(class_name);

        let resolved = false;

        const onEnd = () => {
            if (resolved) {
                return;
            }

            resolved = true;

            el.classList.remove(class_name);
            resolve();

            controller.abort();
        };

        el.addEventListener('animationend', onEnd, {
            once: true,
            signal
        });

        el.addEventListener('animationcancel', onEnd, {
            once: true,
            signal
        });

        // if there are no animations or animation is set to 0ms, end immediately
        requestAnimationFrame(() => {
            if (!resolved && el.getAnimations().length === 0) {
                onEnd();
            }
        });
    });
}

/** Parses a CSS duration and returns the number of milliseconds. */
export function parse_duration(duration) {
    duration = duration.toString().toLowerCase();

    if (duration.indexOf('ms') > -1) {
        return parseFloat(duration) || 0;
    }

    if (duration.indexOf('s') > -1) {
        return (parseFloat(duration) || 0) * 1000;
    }

    return parseFloat(duration) || 0;
}

/** Tells if the user has enabled the "reduced motion" setting in their browser or OS. */
export function prefers_reduced_motion() {
    const query = window.matchMedia('(prefers-reduced-motion: reduce)');
    return query.matches;
}

window.animate = animate;
window.animate_with_class = animate_with_class;
window.parse_duration = parse_duration;
window.prefers_reduced_motion = prefers_reduced_motion;
