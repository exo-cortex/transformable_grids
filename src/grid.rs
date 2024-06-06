use crate::{point::Point, straight_line::StraightLine};

struct Grid {
    straight_lines: Vec<StraightLine>,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            straight_lines: Vec::new(),
        }
    }
    pub fn add_line(&mut self, line: StraightLine) {
        self.straight_lines.push(line);
    }

    pub fn add_horizontal_lines(
        &mut self,
        line_start: Point,
        line_end: Point,
        duplicate: usize,
        shift_direction: Point,
    ) {
        (0..duplicate).into_iter().for_each(|i| {
            let shift = shift_direction * (i as f64);
            let line = StraightLine::new(line_start + shift, line_end + shift);
            self.add_line(line);
        });
    }

    pub fn render_line(&self) -> Vec<Vec<Point>> {
        let steps = 100;
        self.straight_lines
            .iter()
            .map(|l| {
                (0..steps)
                    .into_iter()
                    .map(|lerp_pos| l.start + ((l.end - l.start) * lerp_pos as f64))
                    .collect::<Vec<Point>>()
            })
            .collect()
    }
}

pub struct GridBuilder {
    aabb: Option<(Point, Point)>,
    lines: Vec<StraightLine>,
}

impl GridBuilder {
    pub fn new() -> Self {
        GridBuilder {
            aabb: None,
            lines: Vec::new(),
        }
    }

    pub fn centered_square(mut self, mid: Point, size: f64) -> Self {
        self.aabb = Some((
            mid - Point::new(0.5, 0.5) * size,
            mid + Point::new(0.5, 0.5) * size,
        ));
        self
    }

    pub fn grid(mut self, subdivisions: usize) -> Self {
        let aabb = self.aabb.unwrap();

        // horizontal lines
        (0..subdivisions + 2).into_iter().for_each(|l| {
            let y_pos =
                aabb.0.y() + (aabb.1.y() - aabb.0.y()) * l as f64 / (subdivisions + 1) as f64;
            self.lines.push(StraightLine::new(
                Point::new(aabb.0.x(), y_pos),
                Point::new(aabb.1.x(), y_pos),
            ))
        });
        // vertical lines
        (0..subdivisions + 1).into_iter().for_each(|l| {
            let x_pos =
                aabb.0.x() + (aabb.1.x() - aabb.0.x()) * l as f64 / (subdivisions + 1) as f64;
            self.lines.push(StraightLine::new(
                Point::new(x_pos, aabb.0.y()),
                Point::new(x_pos, aabb.1.y()),
            ))
        });
        self
    }

    pub fn print_grid(&self) {
        for line in &self.lines {
            println!("{:?}", line);
        }
    }

    // pub fn build(self) -> Grid {
    //     Grid {}
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builder_new() {
        assert_eq!(1, 2);
    }
}
