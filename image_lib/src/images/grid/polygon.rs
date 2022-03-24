use super::Point;
use super::Tri;
use rand::Rng;

/// E = number of edges in the polygon
pub struct Polygon<const E: usize> {
    // The root of the polygon, (all edges are a part of a tri formed with the root)
    root: Point,
    edges: [(Point, Point); E],
}

impl<const E: usize> Polygon<E> {
    pub fn new(root: Point, edges: [(Point, Point); E]) -> Polygon<E> {
        Polygon { root, edges }
    }

    /// Generates a random point within the polygon
    pub fn get_random_point(&self) -> Point {
        // Generates the areas of every tri that makes up the polygon
        let areas: [i32; E] = self
            .edges
            .map(|(left, right)| Tri::new(self.root, left, right).area().abs());
        // Finds the total of the areas
        let total_area: i32 = areas.iter().sum();

        if total_area == 0 {
            self.root
        } else {
            // Randomly picks a tri to generates a point in,
            // the probability of a region being chosen is proportional to its area

            let mut tri = rand::thread_rng().gen_range(0..total_area);

            // Finds the tri chosen
            let mut chosen_tri: (Point, Point) = self.edges[0];
            for (i, area) in areas.iter().enumerate() {
                if *area < tri {
                    tri -= *area;
                } else {
                    chosen_tri = self.edges[i];
                    break;
                }
            }
            // Picks a random point in that region
            // To do this, we randomly scale the vector from root to left and to right,
            // and pick the corresponding point
            let (left, right) = chosen_tri;
            // Picks a random amount to scale the vectors by (ensures that the point is in
            // the triangle)
            // https://blogs.sas.com/content/iml/2020/10/19/random-points-in-triangle.html
            let mut left_scale = rand::random::<f32>();
            let mut right_scale = rand::random::<f32>();
            if left_scale + right_scale > 1 as f32 {
                left_scale = 1 as f32 - left_scale;
                right_scale = 1 as f32 - right_scale;
            }
            left_scale /= 2.0;
            right_scale /= 2.0;

            let x = self.root.x as f32
                + (left_scale * (left.x as f32 - self.root.x as f32))
                + (right_scale * (right.x as f32 - self.root.x as f32));
            let y = self.root.y as f32
                + (left_scale * (left.y as f32 - self.root.y as f32))
                + (right_scale * (right.y as f32 - self.root.y as f32));

            Point {
                x: x as u16,
                y: y as u16,
            }
        }
    }
}
