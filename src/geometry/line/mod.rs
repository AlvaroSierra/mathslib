use crate::{generals::tensor::MathVecTrait, geometry::Intersect};
use uom::si::length::{kilometer, Length as LengthDim};

use super::geodetic::{EARTH_RADIUS, Bearing, GeodeticCoordinate2D, NVector};

// TODO: Rename to segment or alike
/// Line made from the shortest path between two points
#[derive(Copy, Clone)]
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
    pub fn bearing(self) -> Bearing {

        let north_vector: NVector = NVector::new([0f32, 0f32, 1f32]);
        let c2 = Line::<NVector>::new(self.x0.into(), north_vector).great_circle();
        let c1 = self.great_circle();
        let a: NVector = self.x1.into();

        let cross_product = c1.cross_product(c2);

        let sign = match cross_product.clone().dot_product(a) >= 0f32 {
            true => 1f32,
            false => -1f32
        };
        
        let sin = sign * cross_product.magnitude();
        let cos = c1.dot_product(c2);

        Bearing::new((sin / cos).atan().into()).expect("This function should never fail but raise issue if it does")
    }

    pub fn bisect(self) -> GeodeticCoordinate2D {
        self.x0.new_from(self.bearing().into(), LengthDim::<uom::si::SI<f32>, f32>::new::<kilometer>(self.length() / 2f32))
    }
}

impl<Coordinate: From<NVector> + Into<NVector> + Clone> Intersect<Coordinate> for Line<Coordinate> {
    fn intersects(self, line: Line<Coordinate>) -> Vec<Coordinate> {
        // TODO: Calculate this from two great circle values instead of doing the conversion inside

        let self_nvec: NVector = self.great_circle();
        let line_nvec: NVector = line.great_circle();

        let _n1: Coordinate = self_nvec.cross_product(line_nvec).into();
        let _n2: Coordinate = line_nvec.cross_product(self_nvec).into();


        todo!()
    }
}

pub trait Length {
    fn length(&self) -> f32;
}

impl<Coordinate: Into<NVector> + Copy> Length for Line<Coordinate> {
    fn length(&self) -> f32 {
        let a: NVector = self.x0.into();
        let b: NVector = self.x1.into();

        let numerator: f32 = a.cross_product(b).magnitude();
        let denominator: f32 = a.dot_product(b);

        EARTH_RADIUS * (numerator / denominator).atan()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_geodetic() {
        
    }
}
