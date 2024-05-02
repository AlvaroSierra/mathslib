use crate::{generals::tensor::{MathVec, MathVecTrait}, geometry::Intersect};

use super::geodetic::{earth_radius, Bearing, GeodeticCoordinate2D, GeodeticCoordinate2DTrait, NVector};

// TODO: Rename to segment or alike
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

impl<Coordinate: GeodeticCoordinate2DTrait> Intersect<Coordinate> for Line<Coordinate> {
    fn intersects(&self, line: Line<Coordinate>) -> Vec<Coordinate> {
        todo!()
    }
}

pub trait Length {
    fn length(&self) -> f32;
}

impl<Coordinate: Into<NVector> + Clone> Length for Line<Coordinate> {
    fn length(&self) -> f32 {
        let a: NVector = self.x0.clone().into();
        let b: NVector = self.x1.clone().into();

        let numerator: f32 = a.cross_product(b).magnitude();
        let denominator: f32 = a.dot_product(b);

        earth_radius * (numerator / denominator).atan()
    }
}

pub(crate) trait GreatCircle {
    fn great_circle(self) -> NVector;
}

impl<Coordinate: Into<NVector> + Clone> GreatCircle for Line<Coordinate> {
    fn great_circle(self) -> NVector{
        let x0: NVector = self.x0.into();
        let x1: NVector = self.x1.into();

        x0.cross_product(x1)
    }
}
