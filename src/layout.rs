//! Final and cached data structures that represent the high-level UI layout

use crate::geometry::{Point, Size};
use crate::sys::abs;

/// Whether we are performing a full layout, or we merely need to size the node
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RunMode {
    /// A full layout for this node and all children should be computed
    PeformLayout,
    /// The layout algorithm should be executed such that an accurate container size for the node can be determined.
    /// Layout steps that aren't necessary for determining the container size of the current node can be skipped.
    ComputeSize,
}

/// Whether styles should be taken into account when computing size
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SizingMode {
    /// Only content contributions should be taken into account
    ContentSize,
    /// Inherent size styles should be taken into account in addition to content contributions
    InherentSize,
}

/// The amount of space available to a node in a given axis
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AvailableSpace {
    /// The amount of space available is the specified number of pixels
    Definite(f32),
    /// The amount of space available is indefinite and the node should be laid out under a min-content constraint
    MinContent,
    /// The amount of space available is indefinite and the node should be laid out under a max-content constraint
    MaxContent,
}

impl AvailableSpace {
    /// A definite available space of zero
    pub const ZERO: AvailableSpace = AvailableSpace::Definite(0.0);

    /// Returns true for definite values, else false
    pub fn is_definite(self) -> bool {
        matches!(self, AvailableSpace::Definite(_))
    }

    /// Convert to Option
    /// Definite values become Some(value). Contraints become None.
    pub fn into_option(self) -> Option<f32> {
        match self {
            AvailableSpace::Definite(value) => Some(value),
            _ => None,
        }
    }

    /// Return the definite value or a default value
    pub fn unwrap_or(self, default: f32) -> f32 {
        self.into_option().unwrap_or(default)
    }

    /// Return the definite value. Panic is the value is not definite.
    #[track_caller]
    pub fn unwrap(self) -> f32 {
        self.into_option().unwrap()
    }

    /// If passed value is Some then return AvailableSpace::Definite containing that value, else return self
    pub fn maybe_set(self, value: Option<f32>) -> AvailableSpace {
        match value {
            Some(value) => AvailableSpace::Definite(value),
            None => self,
        }
    }

    /// Compare equality with another AvailableSpace, treating definite values
    /// that are within f32::EPSILON of each other as equal
    pub fn is_roughly_equal(self, other: AvailableSpace) -> bool {
        use AvailableSpace::*;
        match (self, other) {
            (Definite(a), Definite(b)) => abs(a - b) < f32::EPSILON,
            (MinContent, MinContent) => true,
            (MaxContent, MaxContent) => true,
            _ => false,
        }
    }
}

impl From<f32> for AvailableSpace {
    fn from(value: f32) -> Self {
        Self::Definite(value)
    }
}

impl Size<AvailableSpace> {
    /// A Size<AvailableSpace> containing a MaxContent constraint in both dimensions
    pub const MAX_CONTENT: Size<AvailableSpace> =
        Size { width: AvailableSpace::MaxContent, height: AvailableSpace::MaxContent };

    /// A Size<AvailableSpace> containing a MinContent constraint in both dimensions
    pub const MIN_CONTENT: Size<AvailableSpace> =
        Size { width: AvailableSpace::MinContent, height: AvailableSpace::MinContent };

    /// Convert Size<AvailableSpace> into Size<Option<f32>>
    pub fn as_options(self) -> Size<Option<f32>> {
        Size { width: self.width.into_option(), height: self.height.into_option() }
    }
}

/// The final result of a layout algorithm for a single [`Node`](crate::node::Node).
#[derive(Copy, Debug, Clone)]
pub struct Layout {
    /// The relative ordering of the node
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// This is effectively a topological sort of each tree.
    pub order: u32,
    /// The width and height of the node
    pub size: Size<f32>,
    /// The bottom-left corner of the node
    pub location: Point<f32>,
}

impl Layout {
    /// Creates a new zero-[`Layout`].
    ///
    /// The Zero-layout has size and location set to ZERO.
    /// The `order` value of this layout is set to the minimum value of 0.
    /// This means it should be rendered below all other [`Layout`]s.
    #[must_use]
    pub const fn new() -> Self {
        Self { order: 0, size: Size::ZERO, location: Point::ZERO }
    }

    /// Creates a new zero-[`Layout`] with the supplied `order` value.
    ///
    /// Nodes with a higher order should be rendered on top of those with a lower order.
    /// The Zero-layout has size and location set to ZERO.
    #[must_use]
    pub const fn with_order(order: u32) -> Self {
        Self { order, size: Size::ZERO, location: Point::ZERO }
    }
}

/// Cached intermediate layout results
#[derive(Debug, Clone, Copy)]
pub struct Cache {
    /// The initial cached size of the node itself
    pub(crate) known_dimensions: Size<Option<f32>>,
    /// The initial cached size of the parent's node
    pub(crate) available_space: Size<AvailableSpace>,
    /// Whether or not layout should be recomputed
    pub(crate) run_mode: RunMode,

    /// The cached size of the item
    pub(crate) cached_size: Size<f32>,
}
