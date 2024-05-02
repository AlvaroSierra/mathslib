mod coordinates;
pub mod line;
pub mod shape;
pub use coordinates::*;


pub trait Intersect<Coordinate> {
    fn intersects(&self, line: line::Line<Coordinate>) -> Vec<Coordinate>;
}