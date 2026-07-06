export async function handle_copy(event, from) {
    let target = event.target.closest('.copy-button');
    if (!target) return;

    let value_to_copy = from;
    let is_property = false;
    let is_attribute = false;
    let element = null;

    if (from) {
        is_property = from.includes(".");
        is_attribute = from.includes("[") && from.includes("]");

        let id = from;
        let field = "";
        
        if (is_property) {
            [id, field] = from.trim().split(".");
        } else if (is_attribute) {
            [id, field] = from.trim().replace(/\]$/, "").split("[");
        }

        const root = document;
        element = "getElementById" in root ? root.getElementById(id) : null;
    } else {
        element = target.parentNode;
    }

    if (element) {
        if (is_attribute) {
            value_to_copy = element.getAttribute(field) || "";
        } else if (is_property) {
            value_to_copy = element[field] || "";
        } else {
            value_to_copy = element.textContent || "";
        }
    } else {
        show_status(target, "error");
    }

    if (!value_to_copy) {
        show_status(target, "error");
    } else {
        try {
            await navigator.clipboard.writeText(value_to_copy);
            show_status(target, "success");
        } catch (error) {
            show_status(target, "error");
      }
    }
}

export async function show_status(target, status) {
    const copy_icon = target.querySelector('.copy-button-copy');
    const success_icon = target.querySelector('.copy-button-success');
    const error_icon = target.querySelector('.copy-button-error');

    if (copy_icon) {
        const icon_to_show = status === "success" ? success_icon : error_icon;
        await WingyAnimation.animate_with_class(copy_icon, "hide");
        copy_icon.hidden = true;
        if (icon_to_show) icon_to_show.hidden = false;
        await WingyAnimation.animate_with_class(icon_to_show, "show");
    }
    setTimeout(async () => {
        if (copy_icon) {
            const icon_to_show = status === "success" ? success_icon : error_icon;
            await WingyAnimation.animate_with_class(icon_to_show, "hide");
            if (icon_to_show) icon_to_show.hidden = true;
            copy_icon.hidden = false;
            await WingyAnimation.animate_with_class(copy_icon, "show");
        }
    }, 1000);
}

export function register_copy_action() {
    WingyAction.register("copy", ({ from }, { event, element: _ }) => {
        handle_copy(event, from);
    });
}
