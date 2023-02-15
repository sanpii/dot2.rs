/// Graph kind determines if `digraph` or `graph` is used as keyword
/// for the graph.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Kind {
    Digraph,
    Graph,
}

impl Kind {
    /// The edgeop syntax to use for this graph kind.
    pub(crate) fn edgeop(self) -> &'static str {
        match self {
            Self::Digraph => "->",
            Self::Graph => "--",
        }
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::Digraph => "digraph",
            Self::Graph => "graph",
        };

        write!(f, "{s}")
    }
}
