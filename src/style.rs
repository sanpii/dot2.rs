/// The style for a node or edge.
/// See <https://www.graphviz.org/doc/info/attrs.html#k:style> for descriptions.
/// Note that some of these are not valid for edges.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Style {
    None,
    Solid,
    Dashed,
    Dotted,
    Bold,
    Rounded,
    Diagonals,
    Filled,
    Striped,
    Wedged,
}

impl Style {
    pub fn as_slice(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Bold => "bold",
            Self::Rounded => "rounded",
            Self::Diagonals => "diagonals",
            Self::Filled => "filled",
            Self::Striped => "striped",
            Self::Wedged => "wedged",
        }
    }
}
