use thiserror::Error;

use crate::geometry::{Intersect, geodetic::GeodeticCoordinate2D, line::Line};


pub struct Polygon<Coordinate> {
    points: Vec<Coordinate>,
}

#[derive(Debug, Error)]
pub enum CreatePolygonError {
    #[error("A polygon must have at least three points, you gave {length:?}")]
    ListOfPointsTooShort { length: usize }
}

impl<Coordinate> Polygon<Coordinate> {
    pub fn new(points: Vec<Coordinate>) -> Result<Self, CreatePolygonError> {

        // Other implementations of polygons rely on this precondition being true
        if points.len() < 3 {
            return Err(CreatePolygonError::ListOfPointsTooShort { length: points.len() })
        }

        Ok(
            Self {
                points
            }
        )
    }
}

///
/// 
impl<Coordinate: Clone> From<Polygon<Coordinate>> for Vec<Line<Coordinate>> {
    // TODO: Consider the possibility of returning Rc so that 

    fn from(value: Polygon<Coordinate>) -> Self {
        let mut lines: Vec<Line<Coordinate>> = vec![];
        
        let mut iterator = value.points.into_iter();
        let mut previous = iterator.next().unwrap();

        for i in iterator {
            lines.push(Line::new(previous, i.clone()));
            previous = i; 
        }


        lines
    }
}

impl<'a, Coordinate: Clone> From<&'a Polygon<Coordinate>> for Vec<Line<&'a Coordinate>> {
    // TODO: Consider the possibility of returning Rc so that 

    fn from(value: &'a Polygon<Coordinate>) -> Self {
        let mut lines: Vec<Line<&Coordinate>> = vec![];
        
        let mut iterator = value.points.iter();
        let mut previous = iterator.next().unwrap();

        for i in iterator {
            lines.push(Line::new(previous, &i));
            previous = &i; 
        }

        lines
    }
}



impl Intersect<GeodeticCoordinate2D> for Polygon<GeodeticCoordinate2D> {
    fn intersects(&self, line: Line<GeodeticCoordinate2D>) -> Vec<GeodeticCoordinate2D> {
        todo!()
    }
}
