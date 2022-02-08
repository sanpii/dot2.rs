/// This enumeration represents all possible arrow edge
/// as defined in [grapviz documentation](http://www.graphviz.org/content/arrow-shapes).
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Shape {
    /// No arrow will be displayed
    NoArrow,
    /// Arrow that ends in a triangle. Basically a normal arrow.
    /// NOTE: there is error in official documentation, this supports both fill and side clipping
    Normal(crate::Fill, crate::Side),
    /// Arrow ending in a small square box
    Box(crate::Fill, crate::Side),
    /// Arrow ending in a three branching lines also called crow's foot
    Crow(crate::Side),
    /// Arrow ending in a curve
    Curve(crate::Side),
    /// Arrow ending in an inverted curve
    ICurve(crate::Fill, crate::Side),
    /// Arrow ending in an diamond shaped rectangular shape.
    Diamond(crate::Fill, crate::Side),
    /// Arrow ending in a circle.
    Dot(crate::Fill),
    /// Arrow ending in an inverted triangle.
    Inv(crate::Fill, crate::Side),
    /// Arrow ending with a T shaped arrow.
    Tee(crate::Side),
    /// Arrow ending with a V shaped arrow.
    Vee(crate::Side),
}

impl Shape {
    /// Constructor which returns no arrow.
    pub fn none() -> Self {
        Self::NoArrow
    }

    /// Constructor which returns normal arrow.
    pub fn normal() -> Self {
        Self::Normal(crate::Fill::Filled, crate::Side::Both)
    }

    /// Constructor which returns a regular box arrow.
    pub fn boxed() -> Self {
        Self::Box(crate::Fill::Filled, crate::Side::Both)
    }

    /// Constructor which returns a regular crow arrow.
    pub fn crow() -> Self {
        Self::Crow(crate::Side::Both)
    }

    /// Constructor which returns a regular curve arrow.
    pub fn curve() -> Self {
        Self::Curve(crate::Side::Both)
    }

    /// Constructor which returns an inverted curve arrow.
    pub fn icurve() -> Self {
        Self::ICurve(crate::Fill::Filled, crate::Side::Both)
    }

    /// Constructor which returns a diamond arrow.
    pub fn diamond() -> Self {
        Self::Diamond(crate::Fill::Filled, crate::Side::Both)
    }

    /// Constructor which returns a circle shaped arrow.
    pub fn dot() -> Self {
        Self::Diamond(crate::Fill::Filled, crate::Side::Both)
    }

    /// Constructor which returns an inverted triangle arrow.
    pub fn inv() -> Self {
        Self::Inv(crate::Fill::Filled, crate::Side::Both)
    }

    /// Constructor which returns a T shaped arrow.
    pub fn tee() -> Self {
        Self::Tee(crate::Side::Both)
    }

    /// Constructor which returns a V shaped arrow.
    pub fn vee() -> Self {
        Self::Vee(crate::Side::Both)
    }

    /// Function which renders given Shape into a String for displaying.
    pub fn to_dot_string(&self) -> String {
        let mut res = String::new();

        match *self {
            Shape::Box(fill, side)
            | Shape::ICurve(fill, side)
            | Shape::Diamond(fill, side)
            | Shape::Inv(fill, side)
            | Shape::Normal(fill, side) => {
                res.push_str(fill.as_slice());
                match side {
                    crate::Side::Left | crate::Side::Right => res.push_str(side.as_slice()),
                    crate::Side::Both => {}
                };
            }
            Shape::Dot(fill) => res.push_str(fill.as_slice()),
            Shape::Crow(side) | Shape::Curve(side) | Shape::Tee(side) | Shape::Vee(side) => {
                match side {
                    crate::Side::Left | crate::Side::Right => res.push_str(side.as_slice()),
                    crate::Side::Both => {}
                }
            }
            Shape::NoArrow => {}
        };

        match *self {
            Shape::NoArrow => res.push_str("none"),
            Shape::Normal(_, _) => res.push_str("normal"),
            Shape::Box(_, _) => res.push_str("box"),
            Shape::Crow(_) => res.push_str("crow"),
            Shape::Curve(_) => res.push_str("curve"),
            Shape::ICurve(_, _) => res.push_str("icurve"),
            Shape::Diamond(_, _) => res.push_str("diamond"),
            Shape::Dot(_) => res.push_str("dot"),
            Shape::Inv(_, _) => res.push_str("inv"),
            Shape::Tee(_) => res.push_str("tee"),
            Shape::Vee(_) => res.push_str("vee"),
        };

        res
    }
}

/// This structure holds all information that can describe an arrow connected to
/// either start or end of an edge.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Arrow {
    pub arrows: Vec<Shape>,
}

impl Arrow {
    /// Return `true` if this is a default arrow.
    pub(crate) fn is_default(&self) -> bool {
        self.arrows.is_empty()
    }

    /// Arrow constructor which returns a default arrow
    pub fn default() -> Arrow {
        Arrow { arrows: vec![] }
    }

    /// Arrow constructor which returns an empty arrow
    pub fn none() -> Arrow {
        Arrow {
            arrows: vec![Shape::NoArrow],
        }
    }

    /// Arrow constructor which returns a regular triangle arrow, without modifiers
    pub fn normal() -> Arrow {
        Arrow {
            arrows: vec![Shape::normal()],
        }
    }

    /// Arrow constructor which returns an arrow created by a given Shape.
    pub fn from_arrow(arrow: Shape) -> Arrow {
        Arrow {
            arrows: vec![arrow],
        }
    }

    /// Function which converts given arrow into a renderable form.
    pub fn to_dot_string(&self) -> String {
        let mut cow = String::new();
        for arrow in &self.arrows {
            cow.push_str(&arrow.to_dot_string());
        }
        cow
    }
}

impl Into<Arrow> for [Shape; 2] {
    fn into(self) -> Arrow {
        Arrow {
            arrows: vec![self[0], self[1]],
        }
    }
}

impl Into<Arrow> for [Shape; 3] {
    fn into(self) -> Arrow {
        Arrow {
            arrows: vec![self[0], self[1], self[2]],
        }
    }
}

impl Into<Arrow> for [Shape; 4] {
    fn into(self) -> Arrow {
        Arrow {
            arrows: vec![self[0], self[1], self[2], self[3]],
        }
    }
}
