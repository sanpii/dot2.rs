/// Graph kind determines if `digraph` or `graph` is used as keyword
/// for the graph.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Kind {
    Digraph,
    Graph,
}

impl Kind {
    /// The keyword to use to introduce the graph.
    /// Determines which edge syntax must be used, and default style.
    pub(crate) fn keyword(&self) -> &'static str {
        match *self {
            Kind::Digraph => "digraph",
            Kind::Graph => "graph"
        }
    }

    /// The edgeop syntax to use for this graph kind.
    pub(crate) fn edgeop(&self) -> &'static str {
        match *self {
            Kind::Digraph => "->",
            Kind::Graph => "--",
        }
    }
}
