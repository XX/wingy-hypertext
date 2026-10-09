use crate::class::ICON_START;

/// The side of a header its expand/collapse icon is placed on.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ExpandIconPlacement {
    Start,
    #[default]
    End,
}

impl ExpandIconPlacement {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::Start => ICON_START,
            Self::End => "",
        }
    }
}
