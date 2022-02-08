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
    pub fn to_dot_string(self) -> String {
        let mut res = String::new();

        match self {
            Self::Box(fill, side)
            | Self::ICurve(fill, side)
            | Self::Diamond(fill, side)
            | Self::Inv(fill, side)
            | Self::Normal(fill, side) => {
                res.push_str(fill.as_slice());
                match side {
                    crate::Side::Left | crate::Side::Right => res.push_str(side.as_slice()),
                    crate::Side::Both => {}
                };
            }
            Self::Dot(fill) => res.push_str(fill.as_slice()),
            Self::Crow(side) | Self::Curve(side) | Self::Tee(side) | Self::Vee(side) => {
                match side {
                    crate::Side::Left | crate::Side::Right => res.push_str(side.as_slice()),
                    crate::Side::Both => {}
                }
            }
            Self::NoArrow => {}
        };

        match self {
            Self::NoArrow => res.push_str("none"),
            Self::Normal(_, _) => res.push_str("normal"),
            Self::Box(_, _) => res.push_str("box"),
            Self::Crow(_) => res.push_str("crow"),
            Self::Curve(_) => res.push_str("curve"),
            Self::ICurve(_, _) => res.push_str("icurve"),
            Self::Diamond(_, _) => res.push_str("diamond"),
            Self::Dot(_) => res.push_str("dot"),
            Self::Inv(_, _) => res.push_str("inv"),
            Self::Tee(_) => res.push_str("tee"),
            Self::Vee(_) => res.push_str("vee"),
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
    pub fn default() -> Self {
        Self { arrows: vec![] }
    }

    /// Arrow constructor which returns an empty arrow
    pub fn none() -> Self {
        Self {
            arrows: vec![Shape::NoArrow],
        }
    }

    /// Arrow constructor which returns a regular triangle arrow, without modifiers
    pub fn normal() -> Self {
        Self {
            arrows: vec![Shape::normal()],
        }
    }

    /// Arrow constructor which returns an arrow created by a given Shape.
    pub fn from_arrow(arrow: Shape) -> Self {
        Self {
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

impl From<[Shape; 2]> for Arrow {
    fn from(shape: [Shape; 2]) -> Self {
        Self {
            arrows: vec![shape[0], shape[1]],
        }
    }
}

impl From<[Shape; 3]> for Arrow {
    fn from(shape: [Shape; 3]) -> Self {
        Self {
            arrows: vec![shape[0], shape[1], shape[2]],
        }
    }
}

impl From<[Shape; 4]> for Arrow {
    fn from(shape: [Shape; 4]) -> Self {
        Self {
            arrows: vec![shape[0], shape[1], shape[2], shape[3]],
        }
    }
}
