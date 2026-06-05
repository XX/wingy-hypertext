use std::borrow::Cow;

pub trait ActionSetters {
    fn action(mut self, action: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_action(action);
        self
    }

    fn args(mut self, args: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_args(args);
        self
    }

    fn set_action(&mut self, action: impl Into<Cow<'static, str>>);

    fn set_args(&mut self, args: impl Into<Cow<'static, str>>);
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Action {
    pub action: Option<Cow<'static, str>>,
    pub args: Option<Cow<'static, str>>,
}

impl Action {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ActionSetters for Action {
    fn set_action(&mut self, action: impl Into<Cow<'static, str>>) {
        self.action = Some(action.into());
    }

    fn set_args(&mut self, args: impl Into<Cow<'static, str>>) {
        self.args = Some(args.into());
    }
}

impl<T: AsMut<Action>> ActionSetters for T {
    fn set_action(&mut self, action: impl Into<Cow<'static, str>>) {
        self.as_mut().set_action(action);
    }

    fn set_args(&mut self, args: impl Into<Cow<'static, str>>) {
        self.as_mut().set_args(args);
    }
}
