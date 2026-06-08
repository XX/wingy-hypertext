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

    fn action_mut(&mut self) -> &mut Action;

    fn set_action(&mut self, action: impl Into<Cow<'static, str>>) {
        self.action_mut().action = Some(action.into());
    }

    fn set_args(&mut self, args: impl Into<Cow<'static, str>>) {
        self.action_mut().args = Some(args.into());
    }
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
    fn action_mut(&mut self) -> &mut Action {
        self
    }
}

impl<T: AsMut<Action>> ActionSetters for T {
    fn action_mut(&mut self) -> &mut Action {
        self.as_mut()
    }
}
