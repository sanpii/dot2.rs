#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Option {
    NoEdgeLabels,
    NoNodeLabels,
    NoEdgeStyles,
    NoEdgeColors,
    NoNodeStyles,
    NoNodeColors,
    NoArrows,

    Fontname(String),
    DarkTheme,
}

/// Renders directed graph `g` into the writer `w` in DOT syntax.
/// (Simple wrapper around `render_opts` that passes a default set of options.)
pub fn render<'a, N, E, G, W>(g: &'a G, w: &mut W) -> std::io::Result<()>
where
    N: Clone + 'a,
    E: Clone + 'a,
    G: crate::Labeller<'a, Node = N, Edge = E> + crate::GraphWalk<'a, Node = N, Edge = E>,
    W: std::io::Write,
{
    render_opts(g, w, &[])
}

/// Renders directed graph `g` into the writer `w` in DOT syntax.
/// (Main entry point for the library.)
pub fn render_opts<'a, N, E, G, W>(g: &'a G, w: &mut W, options: &[self::Option]) -> std::io::Result<()>
where
    N: Clone + 'a,
    E: Clone + 'a,
    G: crate::Labeller<'a, Node = N, Edge = E> + crate::GraphWalk<'a, Node = N, Edge = E>,
    W: std::io::Write,
{
    use std::io::Write;

    writeln!(w, "{} {} {{", g.kind().keyword(), g.graph_id().as_slice())?;

    // Global graph properties
    let mut graph_attrs = Vec::new();
    let mut content_attrs = Vec::new();
    let font;
    if let Some(fontname) = options.iter().find_map(|option| {
        if let self::Option::Fontname(fontname) = option { Some(fontname) } else { None }
    }) {
        font = format!(r#"fontname="{}""#, fontname);
        graph_attrs.push(&font[..]);
        content_attrs.push(&font[..]);
    }
    if options.contains(&self::Option::DarkTheme) {
        graph_attrs.push(r#"bgcolor="black""#);
        graph_attrs.push(r#"fontcolor="white""#);
        content_attrs.push(r#"color="white""#);
        content_attrs.push(r#"fontcolor="white""#);
    }
    if !(graph_attrs.is_empty() && content_attrs.is_empty()) {
        writeln!(w, r#"    graph[{}];"#, graph_attrs.join(" "))?;
        let content_attrs_str = content_attrs.join(" ");
        writeln!(w, r#"    node[{}];"#, content_attrs_str)?;
        writeln!(w, r#"    edge[{}];"#, content_attrs_str)?;
    }

    let mut text = Vec::new();
    for n in g.nodes().iter() {
        write!(w, "    ")?;
        let id = g.node_id(n);

        let escaped = &g.node_label(n).to_dot_string();

        write!(text, "{}", id.as_slice()).unwrap();

        if !options.contains(&self::Option::NoNodeLabels) {
            write!(text, "[label={}]", escaped).unwrap();
        }

        let style = g.node_style(n);
        if !options.contains(&self::Option::NoNodeStyles) && style != crate::Style::None {
            write!(text, "[style=\"{}\"]", style.as_slice()).unwrap();
        }

        let color = g.node_color(n);
        if !options.contains(&self::Option::NoNodeColors) {
            if let Some(c) = color {
                let colorstring = c.to_dot_string();
                write!(text, "[color=").unwrap();
                write!(text, "{}", colorstring).unwrap();
                write!(text, "]").unwrap();
            }
        }

        if let Some(s) = g.node_shape(n) {
            write!(text, "[shape={}]", &s.to_dot_string()).unwrap();
        }

        writeln!(text, ";").unwrap();
        w.write_all(&text)?;

        text.clear();
    }

    for e in g.edges().iter() {
        let escaped_label = &g.edge_label(e).to_dot_string();
        let start_arrow = g.edge_start_arrow(e);
        let end_arrow = g.edge_end_arrow(e);
        let start_arrow_s = start_arrow.to_dot_string();
        let end_arrow_s = end_arrow.to_dot_string();

        write!(w, "    ")?;
        let source = g.source(e);
        let target = g.target(e);
        let source_id = g.node_id(&source);
        let target_id = g.node_id(&target);

        write!(text, "{} {} {}", source_id.as_slice(), g.kind().edgeop(), target_id.as_slice()).unwrap();

        if !options.contains(&self::Option::NoEdgeLabels) {
            write!(text, "[label={}]", escaped_label).unwrap();
        }

        let style = g.edge_style(e);
        if !options.contains(&self::Option::NoEdgeStyles) && style != crate::Style::None {
            write!(text, "[style=\"{}\"]", style.as_slice()).unwrap();
        }

        let color = g.edge_color(e);
        if !options.contains(&self::Option::NoEdgeColors) {
            if let Some(c) = color {
                let colorstring = c.to_dot_string();
                write!(text, "[color=").unwrap();
                write!(text, "{}", colorstring).unwrap();
                write!(text, "]").unwrap();
            }
        }

        if !options.contains(&self::Option::NoArrows) &&
            (!start_arrow.is_default() || !end_arrow.is_default()) {
            write!(text, "[").unwrap();
            if !end_arrow.is_default() {
                write!(text, "arrowhead=\"").unwrap();
                write!(text, "{}", end_arrow_s).unwrap();
                write!(text, "\"").unwrap();
            }
            if !start_arrow.is_default() {
                write!(text, " dir=\"both\" arrowtail=\"").unwrap();
                write!(text, "{}", start_arrow_s).unwrap();
                write!(text, "\"").unwrap();
            }

            write!(text, "]").unwrap();
        }

        writeln!(text, ";").unwrap();
        w.write_all(&text)?;

        text.clear();
    }

    writeln!(w, "}}")
}