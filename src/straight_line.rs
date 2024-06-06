use crate::Point;

#[derive(Debug)]
pub struct StraightLine {
    pub start: Point,
    pub end: Point,
}

impl StraightLine {
    pub fn new(start: Point, end: Point) -> Self {
        StraightLine { start, end }
    }

    // maybe like this
    pub fn transform(&mut self, f: impl Fn(&mut Point)) {
        f(&mut self.start);
        f(&mut self.end);
    }
}
