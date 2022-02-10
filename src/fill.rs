/// Arrow modifier that determines if the shape is empty or filled.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Fill {
    Open,
    Filled,
}

impl std::fmt::Display for Fill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Open => "o",
            Self::Filled => "",
        };

        write!(f, "{s}")
    }
}
