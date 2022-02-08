/// Arrow modifier that determines if the shape is empty or filled.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Fill {
    Open,
    Filled,
}

impl Fill {
    pub fn as_slice(self) -> &'static str {
        match self {
            Fill::Open => "o",
            Fill::Filled => "",
        }
    }
}
