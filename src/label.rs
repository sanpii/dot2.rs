// There is a tension in the design of the labelling API.
//
// For example, I considered making a `Labeller<T>` trait that
// provides labels for `T`, and then making the graph type `G`
// implement `Labeller<Node>` and `Labeller<Edge>`. However, this is
// not possible without functional dependencies. (One could work
// around that, but I did not explore that avenue heavily.)
//
// Another approach that I actually used for a while was to make a
// `Label<Context>` trait that is implemented by the client-specific
// Node and Edge types (as well as an implementation on Graph itself
// for the overall name for the graph). The main disadvantage of this
// second approach (compared to having the `G` type parameter
// implement a Labelling service) that I have encountered is that it
// makes it impossible to use types outside of the current crate
// directly as Nodes/Edges; you need to wrap them in newtype'd
// structs. See e.g., the `No` and `Ed` structs in the examples. (In
// practice clients using a graph in some other crate would need to
// provide some sort of adapter shim over the graph anyway to
// interface with this library).
//
// Another approach would be to make a single `Labeller<N,E>` trait
// that provides three methods (graph_label, node_label, edge_label),
// and then make `G` implement `Labeller<N,E>`. At first this did not
// appeal to me, since I had thought I would need separate methods on
// each data variant for dot-internal identifiers versus user-visible
// labels. However, the identifier/label distinction only arises for
// nodes; graphs themselves only have identifiers, and edges only have
// labels.
//
// So in the end I decided to use the third approach described above.

/// Each instance of a type that implements `Label<C>` maps to a
/// unique identifier with respect to `C`, which is used to identify
/// it in the generated .dot file. They can also provide more
/// elaborate (and non-unique) label text that is used in the graphviz
/// rendered output.

/// The graph instance is responsible for providing the DOT compatible
/// identifiers for the nodes and (optionally) rendered labels for the nodes and
/// edges, as well as an identifier for the graph itself.
pub trait Labeller<'a> {
    type Node;
    type Edge;
    type Subgraph;

    /// Must return a DOT compatible identifier naming the graph.
    fn graph_id(&'a self) -> crate::Result<crate::Id<'a>>;

    /// Maps `n` to a unique identifier with respect to `self`. The
    /// implementor is responsible for ensuring that the returned name
    /// is a valid DOT identifier.
    fn node_id(&'a self, n: &Self::Node) -> crate::Result<crate::Id<'a>>;

    /// Maps `n` to one of the [graphviz `shape` names][1]. If `None`
    /// is returned, no `shape` attribute is specified.
    ///
    /// [1]: https://www.graphviz.org/content/node-shapes
    fn node_shape(&'a self, _node: &Self::Node) -> Option<Text<'a>> {
        None
    }

    /// Maps `n` to a label that will be used in the rendered output.
    /// The label need not be unique, and may be the empty string; the
    /// default is just the output from `node_id`.
    fn node_label(&'a self, n: &Self::Node) -> crate::Result<Text<'a>> {
        self.node_id(n).map(|x| Text::LabelStr(x.name))
    }

    /// Maps `n` to a style that will be used in the rendered output.
    fn node_style(&'a self, _n: &Self::Node) -> crate::Style {
        crate::Style::None
    }

    /// Maps `n` to one of the [graphviz `color` names][1]. If `None`
    /// is returned, no `color` attribute is specified.
    ///
    /// [1]: https://graphviz.gitlab.io/_pages/doc/info/colors.html
    fn node_color(&'a self, _node: &Self::Node) -> Option<Text<'a>> {
        None
    }

    /// Maps `e` to arrow style that will be used on the end of an edge.
    /// Defaults to default arrow style.
    fn edge_end_arrow(&'a self, _e: &Self::Edge) -> crate::Arrow {
        crate::Arrow::default()
    }

    /// Maps `e` to arrow style that will be used on the end of an edge.
    /// Defaults to default arrow style.
    fn edge_start_arrow(&'a self, _e: &Self::Edge) -> crate::Arrow {
        crate::Arrow::default()
    }

    /// Maps `e` to a label that will be used in the rendered output.
    /// The label need not be unique, and may be the empty string; the
    /// default is in fact the empty string.
    fn edge_label(&'a self, _e: &Self::Edge) -> Text<'a> {
        Text::LabelStr("".into())
    }

    /// Maps `e` to a style that will be used in the rendered output.
    fn edge_style(&'a self, _e: &Self::Edge) -> crate::Style {
        crate::Style::None
    }

    /// Maps `e` to one of the [graphviz `color` names][1]. If `None`
    /// is returned, no `color` attribute is specified.
    ///
    /// [1]: https://graphviz.gitlab.io/_pages/doc/info/colors.html
    fn edge_color(&'a self, _e: &Self::Edge) -> Option<Text<'a>> {
        None
    }

    /// Maps `s` to a unique subgraph identifier.
    /// Prefix this identifier by `cluster_` to draw this subgraph in its own distinct retangle.
    fn subgraph_id(&'a self, _s: &Self::Subgraph) -> Option<crate::Id<'a>> {
        None
    }

    /// Maps `s` to the corresponding subgraph label.
    fn subgraph_label(&'a self, _s: &Self::Subgraph) -> Text<'a> {
        Text::LabelStr("".into())
    }

    /// Maps `s` to the corresponding subgraph style (default to `Style::None`).
    fn subgraph_style(&'a self, _s: &Self::Subgraph) -> crate::Style {
        crate::Style::None
    }

    /// Maps `s` to the corresponding subgraph shape.
    /// If `None` is returned (default), no `shape` attribute is specified.
    fn subgraph_shape(&'a self, _s: &Self::Subgraph) -> Option<Text<'a>> {
        None
    }

    /// Maps `s` to one of the [graphviz `color` names][1]. If `None`
    /// is returned, no `color` attribute is specified.
    fn subgraph_color(&'a self, _s: &Self::Subgraph) -> Option<crate::label::Text<'a>> {
        None
    }

    /// The kind of graph, defaults to `Kind::Digraph`.
    #[inline]
    fn kind(&self) -> crate::Kind {
        crate::Kind::Digraph
    }
}

/// The text for a graphviz label on a node or edge.
pub enum Text<'a> {
    /// This kind of label preserves the text directly as is.
    ///
    /// Occurrences of backslashes (`\`) are escaped, and thus appear
    /// as backslashes in the rendered label.
    LabelStr(std::borrow::Cow<'a, str>),

    /// This kind of label uses the graphviz label escString type:
    /// <https://www.graphviz.org/content/attrs#kescString>
    ///
    /// Occurrences of backslashes (`\`) are not escaped; instead they
    /// are interpreted as initiating an escString escape sequence.
    ///
    /// Escape sequences of particular interest: in addition to `\n`
    /// to break a line (centering the line preceding the `\n`), there
    /// are also the escape sequences `\l` which left-justifies the
    /// preceding line and `\r` which right-justifies it.
    EscStr(std::borrow::Cow<'a, str>),

    /// This uses a graphviz [HTML string label][html]. The string is
    /// printed exactly as given, but between `<` and `>`. **No
    /// escaping is performed.**
    ///
    /// [html]: https://www.graphviz.org/content/node-shapes#html
    HtmlStr(std::borrow::Cow<'a, str>),
}

impl<'a> Text<'a> {
    pub fn label<S: Into<std::borrow::Cow<'a, str>>>(s: S) -> Self {
        Self::LabelStr(s.into())
    }

    pub fn html<S: Into<std::borrow::Cow<'a, str>>>(s: S) -> Self {
        Self::HtmlStr(s.into())
    }

    fn escape_char<F>(c: char, mut f: F)
    where
        F: FnMut(char),
    {
        match c {
            // not escaping \\, since Graphviz escString needs to
            // interpret backslashes; see EscStr above.
            '\\' => f(c),
            _ => {
                for c in c.escape_default() {
                    f(c)
                }
            }
        }
    }

    fn escape_str(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        for c in s.chars() {
            Self::escape_char(c, |c| out.push(c));
        }
        out
    }

    /// Decomposes content into string suitable for making `EscStr` that
    /// yields same content as self. The result obeys the law
    /// render(`lt`) == render(`EscStr(lt.pre_escaped_content())`) for
    /// all `lt: Text`.
    fn pre_escaped_content(self) -> std::borrow::Cow<'a, str> {
        match self {
            Self::EscStr(s) => s,
            Self::LabelStr(s) => {
                if s.contains('\\') {
                    (*s).escape_default().to_string().into()
                } else {
                    s
                }
            }
            Self::HtmlStr(s) => s,
        }
    }

    /// Puts `suffix` on a line below this label, with a blank line separator.
    #[must_use]
    pub fn suffix_line(self, suffix: Self) -> Self {
        let mut prefix = self.pre_escaped_content().into_owned();
        let suffix = suffix.pre_escaped_content();

        prefix.push_str(r"\n\n");
        prefix.push_str(&suffix);

        Self::EscStr(prefix.into())
    }
}

impl<'a> std::fmt::Display for Text<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Self::LabelStr(ref s) => format!("\"{}\"", s.escape_default()),
            Self::EscStr(ref s) => format!("\"{}\"", Self::escape_str(s)),
            Self::HtmlStr(ref s) => format!("<{s}>"),
        };

        write!(f, "{s}")
    }
}
