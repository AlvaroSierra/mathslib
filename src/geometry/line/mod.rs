use crate::{generals::tensor::MathVec, geometry::Intersect};

use super::geodetic::{Bearing, GeodeticCoordinate2D};

pub struct Line<Coordinate> {
    x0: Coordinate,
    x1: Coordinate,
}

impl<Coordinate> Line<Coordinate> {
    pub fn new(x0: Coordinate, x1: Coordinate) -> Self {
        Self {
            x0,
            x1
        }
    }
}

impl Line<GeodeticCoordinate2D> {
    pub fn bearing(&self) -> Bearing {

        let north_vector = MathVec::new([0, 0, 1]);
        todo!()
    }
}

impl<Coordinate> Intersect<Coordinate> for Line<Coordinate> {
    fn intersects(&self, line: Line<Coordinate>) -> Vec<Coordinate> {
        todo!()
    }
}
