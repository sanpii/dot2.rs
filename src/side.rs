/// Arrow modifier that determines if the shape is clipped.
/// For example `Side::Left` means only left side is visible.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
    Both,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Left => "l",
            Self::Right => "r",
            Self::Both => "",
        };

        write!(f, "{s}")
    }
}
