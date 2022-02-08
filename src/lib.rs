#![warn(warnings)]
#![doc = include_str!("../README.md")]

pub mod label;

mod arrow;
mod fill;
mod graph_walk;
mod id;
mod kind;
mod render;
mod side;
mod style;

pub use arrow::Arrow;
pub use fill::Fill;
pub use graph_walk::GraphWalk;
pub use id::Id;
pub use kind::Kind;
pub use label::Labeller;
pub use render::{render, render_opts};
pub use side::Side;
pub use style::Style;

/// Escape tags in such a way that it is suitable for inclusion in a
/// Graphviz HTML label.
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('\"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub type Nodes<'a, N> = std::borrow::Cow<'a, [N]>;
pub type Edges<'a, E> = std::borrow::Cow<'a, [E]>;
pub type Subgraphs<'a, S> = std::borrow::Cow<'a, [S]>;

#[cfg(test)]
mod tests;
