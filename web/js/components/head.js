function init_scroll_to_anchor() {
    const hash = window.location.hash;

    if (hash) {
        const target_element = document.querySelector(hash);

        if (target_element) {
            const offset = parseFloat(document.documentElement.style.getPropertyValue('--page-header-height')) || 0;
            const element_pos = target_element.getBoundingClientRect().top;
            const to_pos = element_pos + window.pageYOffset - offset;

            window.scrollTo({
                top: to_pos,
                behavior: 'smooth'
            });
        }
    } else {
        window.scrollTo({
            top: 0,
            behavior: 'smooth'
        });
    }
}

window.init_scroll_to_anchor = init_scroll_to_anchor;
