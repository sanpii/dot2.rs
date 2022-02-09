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
pub fn render<'a, N, E, S, G, W>(g: &'a G, w: &mut W) -> crate::Result
where
    N: Clone + 'a,
    E: Clone + 'a,
    S: Clone + 'a,
    G: crate::Labeller<'a, Node = N, Edge = E, Subgraph = S>
        + crate::GraphWalk<'a, Node = N, Edge = E, Subgraph = S>,
    W: std::io::Write,
{
    render_opts(g, w, &[])
}

/// Renders directed graph `g` into the writer `w` in DOT syntax.
/// (Main entry point for the library.)
pub fn render_opts<'a, N, E, S, G, W>(
    g: &'a G,
    w: &mut W,
    options: &[self::Option],
) -> crate::Result
where
    N: Clone + 'a,
    E: Clone + 'a,
    S: Clone + 'a,
    G: crate::Labeller<'a, Node = N, Edge = E, Subgraph = S>
        + crate::GraphWalk<'a, Node = N, Edge = E, Subgraph = S>,
    W: std::io::Write,
{
    writeln!(w, "{} {} {{", g.kind().keyword(), g.graph_id()?.as_slice())?;

    render_subgraphs(g, &g.subgraphs(), w, options)?;
    render_nodes(g, &g.nodes(), w, options)?;
    render_edges(g, &g.edges(), w, options)?;

    writeln!(w, "}}")?;

    Ok(())
}

fn render_subgraphs<
    'a,
    N: Clone + 'a,
    E: Clone + 'a,
    S: Clone + 'a,
    G: crate::Labeller<'a, Node = N, Edge = E, Subgraph = S>
        + crate::GraphWalk<'a, Node = N, Edge = E, Subgraph = S>,
    W: std::io::Write,
>(
    g: &'a G,
    subgraphs: &crate::Subgraphs<'a, S>,
    w: &mut W,
    options: &[crate::render::Option],
) -> crate::Result {
    for s in subgraphs.iter() {
        write!(w, "subgraph ")?;

        let id = g
            .subgraph_id(s)
            .map(|x| format!("{} ", x.name))
            .unwrap_or_default();

        write!(w, "{}", &id)?;

        writeln!(w, "{{")?;

        if !options.contains(&crate::render::Option::NoNodeLabels) {
            let label = format!("label={};\n", g.subgraph_label(s).to_dot_string());
            write!(w, "{label}")?;
        }

        let style = g.subgraph_style(s);
        if !options.contains(&crate::render::Option::NoNodeStyles) && style != crate::Style::None {
            writeln!(w, "style=\"{}\";", style.as_slice())?;
        }

        let color = g.subgraph_color(s);
        if !options.contains(&crate::render::Option::NoNodeColors) {
            if let Some(c) = color {
                writeln!(w, "color={};", c.to_dot_string())?;
            }
        }

        if let Some(s) = g.subgraph_shape(s) {
            write!(w, "shape=\"{}\";", s.to_dot_string())?;
        }

        writeln!(w)?;

        for n in g.subgraph_nodes(s).iter() {
            writeln!(w, "{};", g.node_id(n)?.as_slice())?;
        }

        writeln!(w, "\n}}\n")?;
    }

    Ok(())
}

pub fn render_nodes<'a, N, E, S, G, W>(
    g: &'a G,
    nodes: &crate::Nodes<'a, N>,
    w: &mut W,
    options: &[crate::render::Option],
) -> crate::Result
where
    N: Clone + 'a,
    E: Clone + 'a,
    S: Clone + 'a,
    G: crate::Labeller<'a, Node = N, Edge = E, Subgraph = S>
        + crate::GraphWalk<'a, Node = N, Edge = E, Subgraph = S>,
    W: std::io::Write,
{
    // Global graph properties
    let mut graph_attrs = Vec::new();
    let mut content_attrs = Vec::new();
    let font;

    if let Some(fontname) = options.iter().find_map(|option| {
        if let self::Option::Fontname(fontname) = option {
            Some(fontname)
        } else {
            None
        }
    }) {
        font = format!(r#"fontname="{fontname}""#);
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

    for n in nodes.iter() {
        write!(w, "    ")?;
        let id = g.node_id(n)?;

        let escaped = &g.node_label(n)?.to_dot_string();

        write!(w, "{}", id.as_slice())?;

        if !options.contains(&self::Option::NoNodeLabels) {
            write!(w, "[label={escaped}]")?;
        }

        let style = g.node_style(n);
        if !options.contains(&self::Option::NoNodeStyles) && style != crate::Style::None {
            write!(w, "[style=\"{}\"]", style.as_slice())?;
        }

        let color = g.node_color(n);
        if !options.contains(&self::Option::NoNodeColors) {
            if let Some(c) = color {
                let colorstring = c.to_dot_string();
                write!(w, "[color={colorstring}]")?;
            }
        }

        if let Some(s) = g.node_shape(n) {
            write!(w, "[shape={}]", &s.to_dot_string())?;
        }

        writeln!(w, ";")?;
    }

    Ok(())
}

pub fn render_edges<'a, N, E, S, G, W>(
    g: &'a G,
    edges: &crate::Edges<'a, E>,
    w: &mut W,
    options: &[crate::render::Option],
) -> crate::Result
where
    N: Clone + 'a,
    E: Clone + 'a,
    S: Clone + 'a,
    G: crate::Labeller<'a, Node = N, Edge = E, Subgraph = S>
        + crate::GraphWalk<'a, Node = N, Edge = E, Subgraph = S>,
    W: std::io::Write,
{
    for e in edges.iter() {
        let escaped_label = &g.edge_label(e).to_dot_string();
        let start_arrow = g.edge_start_arrow(e);
        let end_arrow = g.edge_end_arrow(e);
        let start_arrow_s = start_arrow.to_dot_string();
        let end_arrow_s = end_arrow.to_dot_string();

        write!(w, "    ")?;
        let source = g.source(e);
        let target = g.target(e);
        let source_id = g.node_id(&source)?;
        let target_id = g.node_id(&target)?;

        write!(
            w,
            "{} {} {}",
            source_id.as_slice(),
            g.kind().edgeop(),
            target_id.as_slice()
        )?;

        if !options.contains(&self::Option::NoEdgeLabels) {
            write!(w, "[label={escaped_label}]")?;
        }

        let style = g.edge_style(e);
        if !options.contains(&self::Option::NoEdgeStyles) && style != crate::Style::None {
            write!(w, "[style=\"{}\"]", style.as_slice())?;
        }

        let color = g.edge_color(e);
        if !options.contains(&self::Option::NoEdgeColors) {
            if let Some(c) = color {
                let colorstring = c.to_dot_string();
                write!(w, "[color=")?;
                write!(w, "{}", colorstring)?;
                write!(w, "]")?;
            }
        }

        if !options.contains(&self::Option::NoArrows)
            && (!start_arrow.is_default() || !end_arrow.is_default())
        {
            write!(w, "[")?;
            if !end_arrow.is_default() {
                write!(w, "arrowhead=\"{end_arrow_s}\"")?;
            }
            if !start_arrow.is_default() {
                write!(w, " dir=\"both\" arrowtail=\"{start_arrow_s}\"")?;
            }

            write!(w, "]")?;
        }

        writeln!(w, ";")?;
    }

    Ok(())
}
