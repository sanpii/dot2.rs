/// GraphWalk is an abstraction over a directed graph = (nodes,edges)
/// made up of node handles `N` and edge handles `E`, where each `E`
/// can be mapped to its source and target nodes.
///
/// The lifetime parameter `'a` is exposed in this trait (rather than
/// introduced as a generic parameter on each method declaration) so
/// that a client impl can choose `N` and `E` that have substructure
/// that is bound by the self lifetime `'a`.
///
/// The `nodes` and `edges` method each return instantiations of
/// `Cow<[T]>` to leave implementors the freedom to create
/// entirely new vectors or to pass back slices into internally owned
/// vectors.
pub trait GraphWalk<'a> {
    type Node: Clone;
    type Edge: Clone;
    type Subgraph: Clone;

    /// Returns all the nodes in this graph.
    fn nodes(&'a self) -> crate::Nodes<'a, Self::Node>;
    /// Returns all of the edges in this graph.
    fn edges(&'a self) -> crate::Edges<'a, Self::Edge>;
    /// The source node for `edge`.
    fn source(&'a self, edge: &Self::Edge) -> Self::Node;
    /// The target node for `edge`.
    fn target(&'a self, edge: &Self::Edge) -> Self::Node;

    /// Retuns all the subgraphs in this graph.
    fn subgraphs(&'a self) -> crate::Subgraphs<'a, Self::Subgraph> {
        std::borrow::Cow::Borrowed(&[])
    }

    /// Retuns all the subgraphs in this graph.
    fn subgraph_nodes(&'a self, _s: &Self::Subgraph) -> crate::Nodes<'a, Self::Node> {
        std::borrow::Cow::Borrowed(&[])
    }
}
