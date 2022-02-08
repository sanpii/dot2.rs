/// Arrow modifier that determines if the shape is clipped.
/// For example `Side::Left` means only left side is visible.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
    Both,
}

impl Side {
    pub fn as_slice(self) -> &'static str {
        match self {
            Self::Left => "l",
            Self::Right => "r",
            Self::Both => "",
        }
    }
}
