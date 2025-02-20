use crate::label::Text::*;

/// each node is an index in a vector in the graph.
type Node = usize;

struct Edge {
    from: usize,
    to: usize,
    label: &'static str,
    style: crate::Style,
    start_arrow: crate::Arrow,
    end_arrow: crate::Arrow,
    color: Option<&'static str>,
}

type Subgraph = usize;

fn edge(
    from: usize,
    to: usize,
    label: &'static str,
    style: crate::Style,
    color: Option<&'static str>,
) -> Edge {
    Edge {
        from,
        to,
        label,
        style,
        start_arrow: crate::Arrow::default(),
        end_arrow: crate::Arrow::default(),
        color,
    }
}

fn edge_with_arrows(
    from: usize,
    to: usize,
    label: &'static str,
    style: crate::Style,
    start_arrow: crate::Arrow,
    end_arrow: crate::Arrow,
    color: Option<&'static str>,
) -> Edge {
    Edge {
        from,
        to,
        label,
        style,
        start_arrow,
        end_arrow,
        color,
    }
}

struct LabelledGraph {
    /// The name for this graph. Used for labeling generated `digraph`.
    name: &'static str,

    /// Each node is an index into `node_labels`; these labels are
    /// used as the label text for each node. (The node *names*,
    /// which are unique identifiers, are derived from their index
    /// in this array.)
    ///
    /// If a node maps to None here, then just use its name as its
    /// text.
    node_labels: Vec<Option<&'static str>>,

    node_styles: Vec<crate::Style>,

    /// Each edge relates a from-index to a to-index along with a
    /// label; `edges` collects them.
    edges: Vec<Edge>,

    subgraphs: Vec<Vec<Node>>,
}

// A simple wrapper around LabelledGraph that forces the labels to
// be emitted as EscStr.
struct LabelledGraphWithEscStrs {
    graph: LabelledGraph,
}

enum NodeLabels<L> {
    AllNodesLabelled(Vec<L>),
    UnlabelledNodes(usize),
    SomeNodesLabelled(Vec<Option<L>>),
}

type Trivial = NodeLabels<&'static str>;

impl NodeLabels<&'static str> {
    fn to_opt_strs(self) -> Vec<Option<&'static str>> {
        match self {
            Self::UnlabelledNodes(len) => vec![None; len],
            Self::AllNodesLabelled(lbls) => lbls.into_iter().map(Some).collect(),
            Self::SomeNodesLabelled(lbls) => lbls,
        }
    }

    fn len(&self) -> usize {
        match self {
            &Self::UnlabelledNodes(len) => len,
            Self::AllNodesLabelled(lbls) => lbls.len(),
            Self::SomeNodesLabelled(lbls) => lbls.len(),
        }
    }
}

impl LabelledGraph {
    fn new(
        name: &'static str,
        node_labels: Trivial,
        edges: Vec<Edge>,
        subgraphs: Vec<Vec<Node>>,
        node_styles: Option<Vec<crate::Style>>,
    ) -> Self {
        let count = node_labels.len();

        Self {
            name,
            node_labels: node_labels.to_opt_strs(),
            edges,
            node_styles: match node_styles {
                Some(nodes) => nodes,
                None => vec![crate::Style::None; count],
            },
            subgraphs,
        }
    }
}

impl LabelledGraphWithEscStrs {
    fn new(name: &'static str, node_labels: Trivial, edges: Vec<Edge>) -> Self {
        Self {
            graph: LabelledGraph::new(name, node_labels, edges, vec![], None),
        }
    }
}

fn id_name<'a>(n: &Node) -> crate::Result<crate::Id<'a>> {
    crate::Id::new(format!("N{}", *n))
}

impl<'a> crate::Labeller<'a> for LabelledGraph {
    type Node = Node;
    type Edge = &'a Edge;
    type Subgraph = Subgraph;

    fn graph_id(&'a self) -> crate::Result<crate::Id<'a>> {
        crate::Id::new(self.name)
    }

    fn node_id(&'a self, n: &Node) -> crate::Result<crate::Id<'a>> {
        id_name(n)
    }

    fn node_label(&'a self, n: &Node) -> crate::Result<crate::label::Text<'a>> {
        let label = match self.node_labels[*n] {
            Some(l) => LabelStr(l.into()),
            None => LabelStr(id_name(n)?.name),
        };

        Ok(label)
    }

    fn edge_start_arrow(&'a self, e: &Self::Edge) -> crate::Arrow {
        e.start_arrow.clone()
    }

    fn edge_end_arrow(&'a self, e: &Self::Edge) -> crate::Arrow {
        e.end_arrow.clone()
    }

    fn edge_label(&'a self, e: &&'a Edge) -> crate::label::Text<'a> {
        LabelStr(e.label.into())
    }

    fn node_style(&'a self, n: &Node) -> crate::Style {
        self.node_styles[*n]
    }

    fn edge_style(&'a self, e: &&'a Edge) -> crate::Style {
        e.style
    }

    fn edge_color(&'a self, e: &&'a Edge) -> Option<crate::label::Text<'a>> {
        e.color.map(|c| LabelStr(c.into()))
    }

    fn subgraph_id(&'a self, s: &Self::Subgraph) -> Option<crate::Id<'a>> {
        crate::Id::new(format!("cluster_{}", s)).ok()
    }
}

impl<'a> crate::Labeller<'a> for LabelledGraphWithEscStrs {
    type Node = Node;
    type Edge = &'a Edge;
    type Subgraph = ();

    fn graph_id(&'a self) -> crate::Result<crate::Id<'a>> {
        self.graph.graph_id()
    }

    fn node_id(&'a self, n: &Node) -> crate::Result<crate::Id<'a>> {
        self.graph.node_id(n)
    }

    fn node_label(&'a self, n: &Node) -> crate::Result<crate::label::Text<'a>> {
        let label = match self.graph.node_label(n)? {
            LabelStr(s) | EscStr(s) | HtmlStr(s) => EscStr(s),
        };

        Ok(label)
    }

    fn edge_label(&'a self, e: &&'a Edge) -> crate::label::Text<'a> {
        match self.graph.edge_label(e) {
            LabelStr(s) | EscStr(s) | HtmlStr(s) => EscStr(s),
        }
    }
}

impl<'a> crate::GraphWalk<'a> for LabelledGraph {
    type Node = Node;
    type Edge = &'a Edge;
    type Subgraph = Subgraph;

    fn nodes(&'a self) -> crate::Nodes<'a, Node> {
        (0..self.node_labels.len()).collect()
    }

    fn edges(&'a self) -> crate::Edges<'a, &'a Edge> {
        self.edges.iter().collect()
    }

    fn source(&'a self, edge: &&'a Edge) -> Node {
        edge.from
    }

    fn target(&'a self, edge: &&'a Edge) -> Node {
        edge.to
    }

    fn subgraphs(&'a self) -> crate::Subgraphs<'a, Subgraph> {
        std::borrow::Cow::Owned((0..self.subgraphs.len()).collect::<Vec<_>>())
    }

    fn subgraph_nodes(&'a self, s: &Subgraph) -> crate::Nodes<'a, Node> {
        std::borrow::Cow::Borrowed(&self.subgraphs[*s])
    }
}

impl<'a> crate::GraphWalk<'a> for LabelledGraphWithEscStrs {
    type Node = Node;
    type Edge = &'a Edge;
    type Subgraph = ();

    fn nodes(&'a self) -> crate::Nodes<'a, Node> {
        self.graph.nodes()
    }

    fn edges(&'a self) -> crate::Edges<'a, &'a Edge> {
        self.graph.edges()
    }

    fn source(&'a self, edge: &&'a Edge) -> Node {
        edge.from
    }

    fn target(&'a self, edge: &&'a Edge) -> Node {
        edge.to
    }
}

fn test_input(g: LabelledGraph) -> crate::Result<String> {
    let mut writer = Vec::new();
    crate::render(&g, &mut writer)?;

    let mut s = String::new();
    std::io::Read::read_to_string(&mut &*writer, &mut s)?;

    Ok(s)
}

// All of the tests use raw-strings as the format for the expected outputs,
// so that you can cut-and-paste the content into a .dot file yourself to
// see what the graphviz visualizer would produce.

#[test]
fn empty_graph() {
    let labels: Trivial = NodeLabels::UnlabelledNodes(0);
    let r = test_input(LabelledGraph::new(
        "empty_graph",
        labels,
        vec![],
        vec![],
        None,
    ));

    assert_eq!(
        r.unwrap(),
        r#"digraph empty_graph {
}
"#
    );
}

#[test]
fn single_node() {
    let labels: Trivial = NodeLabels::UnlabelledNodes(1);
    let r = test_input(LabelledGraph::new(
        "single_node",
        labels,
        vec![],
        vec![],
        None,
    ));

    assert_eq!(
        r.unwrap(),
        r#"digraph single_node {
    N0[label="N0"];
}
"#
    );
}

#[test]
fn single_node_with_style() {
    let labels: Trivial = NodeLabels::UnlabelledNodes(1);
    let styles = Some(vec![crate::Style::Dashed]);
    let r = test_input(LabelledGraph::new(
        "single_node",
        labels,
        vec![],
        vec![],
        styles,
    ));

    assert_eq!(
        r.unwrap(),
        r#"digraph single_node {
    N0[label="N0"][style="dashed"];
}
"#
    );
}

#[test]
fn single_edge() {
    let labels: Trivial = NodeLabels::UnlabelledNodes(2);
    let result = test_input(LabelledGraph::new(
        "single_edge",
        labels,
        vec![edge(0, 1, "E", crate::Style::None, None)],
        vec![],
        None,
    ));

    assert_eq!(
        result.unwrap(),
        r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"];
}
"#
    );
}

#[test]
fn single_edge_with_style() {
    let labels: Trivial = NodeLabels::UnlabelledNodes(2);
    let result = test_input(LabelledGraph::new(
        "single_edge",
        labels,
        vec![edge(0, 1, "E", crate::Style::Bold, Some("red"))],
        vec![],
        None,
    ));

    assert_eq!(
        result.unwrap(),
        r#"digraph single_edge {
    N0[label="N0"];
    N1[label="N1"];
    N0 -> N1[label="E"][style="bold"][color="red"];
}
"#
    );
}

#[test]
fn test_some_labelled() {
    let labels: Trivial = NodeLabels::SomeNodesLabelled(vec![Some("A"), None]);
    let styles = Some(vec![crate::Style::None, crate::Style::Dotted]);
    let result = test_input(LabelledGraph::new(
        "test_some_labelled",
        labels,
        vec![edge(0, 1, "A-1", crate::Style::None, None)],
        vec![],
        styles,
    ));

    assert_eq!(
        result.unwrap(),
        r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"];
}
"#
    );
}

#[test]
fn single_cyclic_node() {
    let labels: Trivial = NodeLabels::UnlabelledNodes(1);
    let r = test_input(LabelledGraph::new(
        "single_cyclic_node",
        labels,
        vec![edge(0, 0, "E", crate::Style::None, None)],
        vec![],
        None,
    ));

    assert_eq!(
        r.unwrap(),
        r#"digraph single_cyclic_node {
    N0[label="N0"];
    N0 -> N0[label="E"];
}
"#
    );
}

#[test]
fn hasse_diagram() {
    let labels = NodeLabels::AllNodesLabelled(vec!["{x,y}", "{x}", "{y}", "{}"]);
    let r = test_input(LabelledGraph::new(
        "hasse_diagram",
        labels,
        vec![
            edge(0, 1, "", crate::Style::None, Some("green")),
            edge(0, 2, "", crate::Style::None, Some("blue")),
            edge(1, 3, "", crate::Style::None, Some("red")),
            edge(2, 3, "", crate::Style::None, Some("black")),
        ],
        vec![],
        None,
    ));

    assert_eq!(
        r.unwrap(),
        r#"digraph hasse_diagram {
    N0[label="{x,y}"];
    N1[label="{x}"];
    N2[label="{y}"];
    N3[label="{}"];
    N0 -> N1[label=""][color="green"];
    N0 -> N2[label=""][color="blue"];
    N1 -> N3[label=""][color="red"];
    N2 -> N3[label=""][color="black"];
}
"#
    );
}

#[test]
fn left_aligned_text() {
    let labels = NodeLabels::AllNodesLabelled(vec![
        "if test {\
       \\l    branch1\
       \\l} else {\
       \\l    branch2\
       \\l}\
       \\lafterward\
       \\l",
        "branch1",
        "branch2",
        "afterward",
    ]);

    let mut writer = Vec::new();

    let g = LabelledGraphWithEscStrs::new(
        "syntax_tree",
        labels,
        vec![
            edge(0, 1, "then", crate::Style::None, None),
            edge(0, 2, "else", crate::Style::None, None),
            edge(1, 3, ";", crate::Style::None, None),
            edge(2, 3, ";", crate::Style::None, None),
        ],
    );

    crate::render(&g, &mut writer).unwrap();
    let mut r = String::new();
    std::io::Read::read_to_string(&mut &*writer, &mut r).unwrap();

    assert_eq!(
        r,
        r#"digraph syntax_tree {
    N0[label="if test {\l    branch1\l} else {\l    branch2\l}\lafterward\l"];
    N1[label="branch1"];
    N2[label="branch2"];
    N3[label="afterward"];
    N0 -> N1[label="then"];
    N0 -> N2[label="else"];
    N1 -> N3[label=";"];
    N2 -> N3[label=";"];
}
"#
    );
}

#[test]
fn simple_id_construction() {
    let id1 = crate::Id::new("hello");

    match id1 {
        Ok(_) => {}
        Err(..) => panic!("'hello' is not a valid value for id anymore"),
    }
}

#[test]
fn test_some_arrow() {
    let labels: Trivial = NodeLabels::SomeNodesLabelled(vec![Some("A"), None]);
    let styles = Some(vec![crate::Style::None, crate::Style::Dotted]);
    let start = crate::Arrow::default();
    let end = crate::Arrow::from_arrow(crate::arrow::Shape::crow());
    let result = test_input(LabelledGraph::new(
        "test_some_labelled",
        labels,
        vec![edge_with_arrows(
            0,
            1,
            "A-1",
            crate::Style::None,
            start,
            end,
            None,
        )],
        vec![],
        styles,
    ));
    assert_eq!(
        result.unwrap(),
        r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="crow"];
}
"#
    );
}

#[test]
fn test_some_arrows() {
    let labels: Trivial = NodeLabels::SomeNodesLabelled(vec![Some("A"), None]);
    let styles = Some(vec![crate::Style::None, crate::Style::Dotted]);
    let start = crate::Arrow::from_arrow(crate::arrow::Shape::tee());
    let end = crate::Arrow::from_arrow(crate::arrow::Shape::Crow(crate::Side::Left));
    let result = test_input(LabelledGraph::new(
        "test_some_labelled",
        labels,
        vec![edge_with_arrows(
            0,
            1,
            "A-1",
            crate::Style::None,
            start,
            end,
            None,
        )],
        vec![],
        styles,
    ));

    assert_eq!(
        result.unwrap(),
        r#"digraph test_some_labelled {
    N0[label="A"];
    N1[label="N1"][style="dotted"];
    N0 -> N1[label="A-1"][arrowhead="lcrow" dir="both" arrowtail="tee"];
}
"#
    );
}

#[test]
fn badly_formatted_id() {
    let id2 = crate::Id::new("Weird { struct : ure } !!!");

    if id2.is_ok() {
        panic!("graphviz id suddenly allows spaces, brackets and stuff");
    }
}

#[test]
fn subgraph() {
    let labels = NodeLabels::AllNodesLabelled(vec!["{x,y}", "{x}", "{y}", "{}"]);
    let r = test_input(LabelledGraph::new(
        "di",
        labels,
        vec![
            edge(0, 1, "", crate::Style::None, None),
            edge(0, 2, "", crate::Style::None, None),
            edge(1, 3, "", crate::Style::None, None),
            edge(2, 3, "", crate::Style::None, None),
        ],
        vec![vec![0, 1], vec![2, 3]],
        None,
    ));
    assert_eq!(
        r.unwrap(),
        r#"digraph di {
    subgraph cluster_0 {
        label="";

        N0;
        N1;
    }

    subgraph cluster_1 {
        label="";

        N2;
        N3;
    }

    N0[label="{x,y}"];
    N1[label="{x}"];
    N2[label="{y}"];
    N3[label="{}"];
    N0 -> N1[label=""];
    N0 -> N2[label=""];
    N1 -> N3[label=""];
    N2 -> N3[label=""];
}
"#
    );
}
