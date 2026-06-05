export function set_page_header_height() {
    const header = document.querySelector('.page-header');
    if (header) {
        document.documentElement.style.setProperty('--page-header-height', header.offsetHeight + 'px');
    }
}

export function init_page() {
    set_page_header_height();
}

export function init_page_element(page_el) {
    if (page_el && !page_el.dataset.initialized) {
        page_el.dataset.initialized = 'true';
        init_page();
    }
}
