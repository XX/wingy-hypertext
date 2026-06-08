use std::borrow::Cow;

macro_rules! htmx_attrs {
    ($($method:ident),+ $(,)?) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        enum HtmxAttr {
            $($method),+
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct Htmx {
            attrs: Vec<(HtmxAttr, Cow<'static, str>)>,
        }

        impl Htmx {
            pub fn new() -> Self {
                Self::default()
            }

            fn get_attr(&self, attr: HtmxAttr) -> Option<&str> {
                self.attrs
                    .iter()
                    .find(|(key, _)| *key == attr)
                    .map(|(_, value)| value.as_ref())
            }

            fn set_attr(&mut self, attr: HtmxAttr, value: Cow<'static, str>) {
                if let Some(slot) = self.attrs.iter_mut().find(|(key, _)| *key == attr) {
                    slot.1 = value;
                } else {
                    self.attrs.push((attr, value));
                }
            }

            $(
                pub fn $method(&self) -> Option<&str> {
                    self.get_attr(HtmxAttr::$method)
                }
            )+
        }

        pub trait HtmxSetters {
            fn htmx_mut(&mut self) -> &mut Htmx;

            $(
                #[must_use]
                fn $method(mut self, value: impl Into<Cow<'static, str>>) -> Self
                where
                    Self: Sized,
                {
                    self.htmx_mut().set_attr(HtmxAttr::$method, value.into());
                    self
                }
            )+
        }

        impl HtmxSetters for Htmx {
            fn htmx_mut(&mut self) -> &mut Htmx {
                self
            }
        }

        impl<T: AsMut<Htmx>> HtmxSetters for T {
            fn htmx_mut(&mut self) -> &mut Htmx {
                self.as_mut()
            }
        }
    };
}

htmx_attrs! {
    hx_get,
    hx_post,
    hx_put,
    hx_patch,
    hx_delete,
    hx_push_url,
    hx_select,
    hx_select_oob,
    hx_swap,
    hx_swap_oob,
    hx_target,
    hx_trigger,
    hx_vals,
    hx_boost,
    hx_confirm,
    hx_disable,
    hx_disabled_elt,
    hx_disinherit,
    hx_encoding,
    hx_ext,
    hx_headers,
    hx_history,
    hx_history_elt,
    hx_include,
    hx_indicator,
    hx_inherit,
    hx_params,
    hx_preserve,
    hx_prompt,
    hx_replace_url,
    hx_request,
    hx_sync,
    hx_validate,
    sse_connect,
    sse_swap,
    sse_close,
    ws_connect,
    ws_send,
}
