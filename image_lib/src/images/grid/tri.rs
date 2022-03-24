use crate::point::Point;
use core::cmp;

pub struct Tri {
    first: Point,
    second: Point,
    third: Point,
}

impl Tri {
    pub fn new(first: Point, second: Point, third: Point) -> Tri {
        Tri {
            first,
            second,
            third,
        }
    }

    pub fn area(&self) -> i32 {
        // TODO: Fix Mult Overflow
        // Shoelace formula for triangle area
        (self.first.x as i32 * (self.second.y as i32 - self.third.y as i32)
            + self.second.x as i32 * (self.third.y as i32 - self.first.y as i32)
            + self.third.x as i32 * (self.first.y as i32 - self.second.y as i32))
            .abs()
            / 2
    }

    fn draw_triangle<T>(&self, mut on_point: T)
    where
        T: FnMut(Point),
    {
        // First sorts the verticies by height
        let mut sorted = [self.first, self.second, self.third];
        sorted.sort_by(|l, r| r.y.cmp(&l.y));
        let v1 = sorted[0];
        let v2 = sorted[1];
        let v3 = sorted[2];

        // If v1.y = v3.y then we know that the area of the tri must be 0
        if v1.y == v3.y {
            return;
        } else {
            let v1 = (v1.x as i64, v1.y as i64);
            let v2 = (v2.x as i64, v2.y as i64);
            let v3 = (v3.x as i64, v3.y as i64);
            for y in v3.1..v1.1 {
                // Gets v1 -> v3 (as v1 is highest and v3 is lowest) x
                let long_x = v1.0 + ((v3.0 - v1.0) * (y - v1.1)) / (v3.1 - v1.1);

                // If v1.x == v2.x then it is assumed that the tri containing v1 & v2 has zero area
                if v1.1 == v2.1 && v2.1 == y {
                } else if v2.1 == v3.1 && v2.1 == y { // Likewise for the v2 & v3 tri
                } else {
                    // Decides which short side we are getting x for
                    let mut short_x = 0;
                    if y > v2.1 {
                        short_x = v1.0 + ((y - v1.1) * (v2.0 - v1.0)) / (v2.1 - v1.1);
                    } else {
                        short_x = v2.0 + ((y - v2.1) * (v3.0 - v2.0)) / (v3.1 - v2.1);
                    }
                    for x in if short_x < long_x {
                        short_x..long_x
                    } else {
                        long_x..short_x
                    } {
                        on_point(Point {
                            x: x as u16,
                            y: y as u16,
                        });
                    }
                }
            }
        }
    }

    /// Rasterizes the tri, the draw_{x}_{y} are booleans dictating whether the raster
    /// should include the line between x and y
    pub fn rasterize<T>(
        &self,
        draw_first_second: bool,
        draw_second_third: bool,
        draw_first_third: bool,
        mut on_point: T,
    ) where
        T: FnMut(Point),
    {
        // TODO: This is terrible... fix it!!!
        let min_x = cmp::min(cmp::min(self.first.x, self.second.x), self.third.x);
        let min_y = cmp::min(cmp::min(self.first.y, self.second.y), self.third.y);
        let max_x = cmp::max(cmp::max(self.first.x, self.second.x), self.third.x);
        let max_y = cmp::max(cmp::max(self.first.y, self.second.y), self.third.y);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                // If the point is in the tri, then rasterize it
                if self.area()
                    == Tri::new(self.first, self.second, Point { x, y }).area()
                        + Tri::new(self.second, self.third, Point { x, y }).area()
                        + Tri::new(self.first, self.third, Point { x, y }).area()
                {
                    on_point(Point { x, y });
                }
            }
        }
    }
}
