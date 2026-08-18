use std::borrow::Cow;

pub trait ActionSetters<'a> {
    fn action(mut self, action: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_action(action);
        self
    }

    fn args(mut self, args: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_args(args);
        self
    }

    fn action_mut(&mut self) -> &mut Action<'a>;

    fn set_action(&mut self, action: impl Into<Cow<'a, str>>) {
        self.action_mut().action = Some(action.into());
    }

    fn set_args(&mut self, args: impl Into<Cow<'a, str>>) {
        self.action_mut().args = Some(args.into());
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Action<'a> {
    pub action: Option<Cow<'a, str>>,
    pub args: Option<Cow<'a, str>>,
}

impl<'a> Action<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> ActionSetters<'a> for Action<'a> {
    fn action_mut(&mut self) -> &mut Action<'a> {
        self
    }
}

impl<'a, T: AsMut<Action<'a>>> ActionSetters<'a> for T {
    fn action_mut(&mut self) -> &mut Action<'a> {
        self.as_mut()
    }
}
