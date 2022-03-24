/// The root of the grid is top left, equally, in rasterization, (0,0) is the top left as well
use super::Colour;
use super::Point;
use super::Polygon;
use super::Tri;
use rand::Rng;
use std::cmp::{max, min};
use std::iter::StepBy;
use std::ops::Range;
mod member;
mod mutation;
mod rasters;
pub use member::{BreedMetadata, FitnessMetadata, GAImageMember};
pub use rasters::{AxisResolution, Resolution};

/// W - 1 and H - 1 are upper bounds for the horizontal and vertical values
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct GridVertex {
    // The horizontal index of the vertex in the grid
    horizontal: usize,
    // The vertical index of the vertex in the grid
    vertical: usize,
}

impl GridVertex {
    fn new(horizontal: usize, vertical: usize) -> GridVertex {
        GridVertex {
            horizontal,
            vertical,
        }
    }

    fn up(&self) -> GridVertex {
        GridVertex::new(self.horizontal, self.vertical - 1)
    }
    fn up_right(&self) -> GridVertex {
        GridVertex::new(self.horizontal + 1, self.vertical - 1)
    }
    fn right(&self) -> GridVertex {
        GridVertex::new(self.horizontal + 1, self.vertical)
    }
    fn down_right(&self) -> GridVertex {
        GridVertex::new(self.horizontal + 1, self.vertical + 1)
    }
    fn down(&self) -> GridVertex {
        GridVertex::new(self.horizontal, self.vertical + 1)
    }
    fn down_left(&self) -> GridVertex {
        GridVertex::new(self.horizontal - 1, self.vertical + 1)
    }
    fn left(&self) -> GridVertex {
        GridVertex::new(self.horizontal - 1, self.vertical)
    }
    fn up_left(&self) -> GridVertex {
        GridVertex::new(self.horizontal - 1, self.vertical - 1)
    }
}

/// The set of edges between neighbours of a vertex
enum GridNeighbourEdgeSet {
    Centre([(GridVertex, GridVertex); 6]),

    TopLeftCorner([(GridVertex, GridVertex); 2]),
    TopRightCorner([(GridVertex, GridVertex); 1]),
    BottomLeftCorner([(GridVertex, GridVertex); 1]),
    BottomRightCorner([(GridVertex, GridVertex); 2]),

    LeftEdge([(GridVertex, GridVertex); 3]),
    TopEdge([(GridVertex, GridVertex); 3]),
    RightEdge([(GridVertex, GridVertex); 3]),
    BottomEdge([(GridVertex, GridVertex); 3]),
}

/// A grid based image, it has W nodes in the horizontal axis and H nodes in the vertical axis
#[derive(Clone)]
pub struct GridImage<const W: usize, const H: usize> {
    /// The positions of every vertex in the graph, this is a lookup table for the point function
    /// indexed by horizontal then vertical
    vertex_positions: [[Point; W]; H],
    /// Stores the colours of each vertex that is capable of having a colour
    /// we store colours for the right and bottom edges even though they cannot be used
    colours: [[(Colour, Colour); W]; H],
}

impl<const W: usize, const H: usize> GridImage<W, H> {
    pub fn new_uniform() -> GridImage<W, H> {
        let width = u16::MAX;
        let height = u16::MAX;

        let mut vertex_positions = [[Point { x: 0, y: 0 }; W]; H];
        for y in 0..H {
            let y_separation = height / (H - 1) as u16;
            for x in 0..W {
                let x_separation = width / (W - 1) as u16;
                vertex_positions[y][x] = Point {
                    x: x as u16 * x_separation,
                    y: y as u16 * y_separation,
                }
            }
        }
        let colours = [0; H].map(|_| {
            [0; W].map(|_| {
                (
                    Colour {
                        r: rand::random(),
                        g: rand::random(),
                        b: rand::random(),
                    },
                    Colour {
                        r: rand::random(),
                        g: rand::random(),
                        b: rand::random(),
                    },
                )
            })
        });
        let mut image = GridImage {
            vertex_positions,
            colours,
        };
        // Applies a random number of structure mutations
        // for _ in 0..rand::thread_rng().gen_range(0..=2048) {
        //     image.mutate_structure();
        // }
        image
    }

    pub fn get_vertex_positions(&self) -> &[[Point; W]; H] {
        &self.vertex_positions
    }

    pub fn get_colours(&self) -> &[[(Colour, Colour); W]; H] {
        &self.colours
    }

    pub fn get_random_inner_vertex(&self) -> GridVertex {
        GridVertex {
            horizontal: (rand::random::<usize>() % (W - 2)) + 1,
            vertical: (rand::random::<usize>() % (H - 2)) + 1,
        }
    }

    /// Looks up a vertex's position (applies the point function)
    fn get_vert_position(&self, vert: &GridVertex) -> Point {
        self.vertex_positions[vert.vertical][vert.horizontal]
    }

    fn get_vert_colours(&self, vert: &GridVertex) -> (Colour, Colour) {
        self.colours[vert.vertical][vert.horizontal]
    }

    /// Gets the neighbours of a vertex in the grid, there are at most 8 neighbours and so this
    /// returns at most 8 neighbours
    fn neighbour_edge_set(&self, vert: &GridVertex) -> GridNeighbourEdgeSet {
        match vert {
            // -- If the vertex is a corner vertex, it has 3 neighbours
            // Top Left
            GridVertex {
                horizontal: 0,
                vertical: 0,
            } => GridNeighbourEdgeSet::TopLeftCorner([
                (vert.right(), vert.down_right()),
                (vert.down_right(), vert.down()),
            ]),
            // Bottom Left
            GridVertex {
                horizontal: h,
                vertical: v,
            } if *h == 0 && *v == H - 1 => {
                GridNeighbourEdgeSet::BottomLeftCorner([(vert.up(), vert.right())])
            }
            // Top Right
            GridVertex {
                horizontal: h,
                vertical: v,
            } if *h == W - 1 && *v == 0 => {
                GridNeighbourEdgeSet::TopRightCorner([(vert.down(), vert.left())])
            }
            // Bottom Right
            GridVertex {
                horizontal: h,
                vertical: v,
            } if *h == W - 1 && *v == H - 1 => GridNeighbourEdgeSet::BottomRightCorner([
                (vert.left(), vert.up_left()),
                (vert.up_left(), vert.up()),
            ]),
            // If the vertex is an edge vertex, it has 5 neighbours
            GridVertex {
                horizontal: h,
                vertical: _,
            } if *h == 0 => GridNeighbourEdgeSet::LeftEdge([
                (vert.up(), vert.right()),
                (vert.right(), vert.down_right()),
                (vert.down_right(), vert.down()),
            ]),
            GridVertex {
                horizontal: h,
                vertical: _,
            } if *h == W - 1 => GridNeighbourEdgeSet::RightEdge([
                (vert.down(), vert.left()),
                (vert.left(), vert.up_left()),
                (vert.up_left(), vert.up()),
            ]),
            GridVertex {
                horizontal: _,
                vertical: v,
            } if *v == 0 => GridNeighbourEdgeSet::TopEdge([
                (vert.right(), vert.down_right()),
                (vert.down_right(), vert.down()),
                (vert.down(), vert.left()),
            ]),
            GridVertex {
                horizontal: _,
                vertical: v,
            } if *v == H - 1 => GridNeighbourEdgeSet::BottomEdge([
                (vert.left(), vert.up_left()),
                (vert.up_left(), vert.up()),
                (vert.up(), vert.right()),
            ]),
            // Otherwise, the vertex has 8 neighbours
            _ => GridNeighbourEdgeSet::Centre([
                (vert.up(), vert.right()),
                (vert.right(), vert.down_right()),
                (vert.down_right(), vert.down()),
                (vert.down(), vert.left()),
                (vert.left(), vert.up_left()),
                (vert.up_left(), vert.up()),
            ]),
        }
    }

    /// Gets a random point within a vertex's surrounding polygon
    fn get_random_point_in_vertex_polygon(&self, vert: &GridVertex) -> Point {
        // The goal is to pick one of the triangles making up the polygon
        // (randomly, relative to area), and then within that triangle, pick a random point,
        // respective to https://mathworld.wolfram.com/TrianglePointPicking.html
        // Gets the horizontal and vertical index of the vertex in the grid respectively

        // Maps a pair of vertices to their respective positions
        let vert_point_map = |(left, right): (GridVertex, GridVertex)| {
            (
                self.get_vert_position(&left),
                self.get_vert_position(&right),
            )
        };

        // Gets the polygon surrounding the vert and uses it to generate a random point
        match self.neighbour_edge_set(vert) {
            GridNeighbourEdgeSet::Centre(e) => {
                Polygon::new(self.get_vert_position(vert), e.map(vert_point_map)).get_random_point()
            }
            GridNeighbourEdgeSet::TopLeftCorner(e) | GridNeighbourEdgeSet::BottomRightCorner(e) => {
                Polygon::new(self.get_vert_position(vert), e.map(vert_point_map)).get_random_point()
            }
            GridNeighbourEdgeSet::TopRightCorner(e) | GridNeighbourEdgeSet::BottomLeftCorner(e) => {
                Polygon::new(self.get_vert_position(vert), e.map(vert_point_map)).get_random_point()
            }
            GridNeighbourEdgeSet::LeftEdge(e)
            | GridNeighbourEdgeSet::TopEdge(e)
            | GridNeighbourEdgeSet::RightEdge(e)
            | GridNeighbourEdgeSet::BottomEdge(e) => {
                Polygon::new(self.get_vert_position(vert), e.map(vert_point_map)).get_random_point()
            }
        }
    }

    // /// Given a vertex and delta, tries to 'apply' the delta to the vertex's position, limiting
    // /// it if the 'applied' delta is out of bounds.
    // /// Note, this doesn't actually modify the structure, it just returns the new position
    // fn apply_delta_to_vertex(&self, vert: &GridVertex, delta: (u16, u16)) -> Point {}
}
