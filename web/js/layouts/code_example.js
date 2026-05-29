//
// Resizing previews
//
document.addEventListener('mousedown', handleResizerDrag);
document.addEventListener('touchstart', handleResizerDrag, { passive: true });

function handleResizerDrag(event) {
    const resizer = event.target.closest('.code-example-resizer');
    const preview = event.target.closest('.code-example-preview');

    if (!resizer || !preview) return;

    let startX = event.changedTouches ? event.changedTouches[0].pageX : event.clientX;
    let startWidth = parseInt(document.defaultView.getComputedStyle(preview).width, 10);

    event.preventDefault();
    preview.classList.add('code-example-preview--dragging');
    document.documentElement.addEventListener('mousemove', dragMove);
    document.documentElement.addEventListener('touchmove', dragMove);
    document.documentElement.addEventListener('mouseup', dragStop);
    document.documentElement.addEventListener('touchend', dragStop);

    function dragMove(event) {
        const width = startWidth + (event.changedTouches ? event.changedTouches[0].pageX : event.pageX) - startX;
        preview.style.width = `${width}px`;
    }

    function dragStop() {
        preview.classList.remove('code-example-preview--dragging');
        document.documentElement.removeEventListener('mousemove', dragMove);
        document.documentElement.removeEventListener('touchmove', dragMove);
        document.documentElement.removeEventListener('mouseup', dragStop);
        document.documentElement.removeEventListener('touchend', dragStop);
    }
}

//
// Code example open animation
//
document.addEventListener('click', event => {
  const toggle = event.target?.closest('.code-example-toggle');

  if (toggle) {
    const code_example = toggle.closest('.code-example');
    if (!code_example) {
      return;
    }

    const open = !code_example.classList.contains('open');
    void set_code_example_open(code_example, toggle, open);
  }
});

const CODE_EXAMPLE_ANIMATIONS = new WeakMap();

function get_animation_generation(code_example) {
    return CODE_EXAMPLE_ANIMATIONS.get(code_example) || 0;
}

function bump_animation_generation(code_example) {
    const generation = get_animation_generation(code_example) + 1;
    CODE_EXAMPLE_ANIMATIONS.set(code_example, generation);
    return generation;
}

function cancel_source_animations(source) {
    source.getAnimations().forEach(animation => animation.cancel());
}

function get_code_example_durations(source) {
    const style = getComputedStyle(source);
    const show_duration = parse_duration(style.getPropertyValue('--code-example-show-duration').trim() || '200ms');
    const hide_duration = parse_duration(style.getPropertyValue('--code-example-hide-duration').trim() || '200ms');

    return { show_duration, hide_duration };
}

function set_code_example_source_accessibility(source, open) {
    if (open) {
        source.removeAttribute('aria-hidden');
    } else {
        source.setAttribute('aria-hidden', 'true');
    }
}

function set_code_example_source_collapsed(source, collapsed) {
    if (collapsed) {
        source.style.height = '0';
        source.style.opacity = '0';
        return;
    }

    source.style.height = 'auto';
    source.style.opacity = '';
}

function reset_code_example_element(code_example) {
    const source = code_example.querySelector('.code-example-source');
    const preview = code_example.querySelector('.code-example-preview');

    if (source) {
        cancel_source_animations(source);
        source.classList.remove('is-animating');
    }

    if (preview) {
        preview.classList.remove('is-dragging');
        preview.style.removeProperty('width');
    }
}

function init_code_examples() {
    document.querySelectorAll('.code-example').forEach(code_example => {
        const source = code_example.querySelector('.code-example-source');
        if (!source) {
            return;
        }

        reset_code_example_element(code_example);

        const open = code_example.classList.contains('open');
        set_code_example_source_collapsed(source, !open);
        set_code_example_source_accessibility(source, open);
    });
}

async function set_code_example_open(code_example, toggle, open) {
    const source = code_example.querySelector('.code-example-source');
    if (!source) {
        return;
    }

    const generation = bump_animation_generation(code_example);
    cancel_source_animations(source);
    source.classList.remove('is-animating');

    if (prefers_reduced_motion() || source.classList.contains('no-animation')) {
        toggle.setAttribute('aria-expanded', open ? 'true' : 'false');
        code_example.classList.toggle('open', open);
        set_code_example_source_collapsed(source, !open);
        set_code_example_source_accessibility(source, open);
        return;
    }

    const { show_duration, hide_duration } = get_code_example_durations(source);

    if (open) {
        toggle.setAttribute('aria-expanded', 'true');
        code_example.classList.add('open');
        set_code_example_source_accessibility(source, true);
        source.classList.add('is-animating');
        source.style.height = '0';
        source.style.opacity = '0';

        await new Promise(resolve => requestAnimationFrame(resolve));

        await animate(
            source,
            [
                { height: '0', opacity: '0' },
                { height: `${source.scrollHeight}px`, opacity: '1' },
            ],
            { duration: show_duration, easing: 'linear' },
        );

        if (get_animation_generation(code_example) !== generation) {
            return;
        }

        source.style.height = 'auto';
        source.style.opacity = '';
        source.classList.remove('is-animating');
    } else {
        toggle.setAttribute('aria-expanded', 'false');
        source.classList.add('is-animating');
        // Remove .open before the animation so the chevron rotation and panel collapse run together,
        // mirroring the open path where .open is added before the panel animates.
        code_example.classList.remove('open');
        // Setting an explicit pixel height flushes layout, so no rAF is needed here
        // (unlike the open path, which animates from height: 0 and must measure scrollHeight first).
        const startHeight = source.scrollHeight;
        source.style.height = `${startHeight}px`;

        await animate(
            source,
            [
                { height: `${startHeight}px`, opacity: '1' },
                { height: '0', opacity: '0' },
            ],
            { duration: hide_duration, easing: 'linear' },
        );

        if (get_animation_generation(code_example) !== generation) {
            return;
        }

        set_code_example_source_collapsed(source, true);
        source.classList.remove('is-animating');
        set_code_example_source_accessibility(source, false);
    }
}

// Initial pass for first paint; turbo:load re-syncs after client-side navigation.
init_code_examples();
document.addEventListener('turbo:load', init_code_examples);
