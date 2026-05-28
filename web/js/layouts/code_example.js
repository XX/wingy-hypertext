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
