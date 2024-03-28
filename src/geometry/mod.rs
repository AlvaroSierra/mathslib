pub mod polar;
pub mod geodetic;

pub mod cartesian {
    use crate::generals::tensor::MathVec;

    // In this library a vector is the same as a Cartesian coordinates point
    pub type CartesianCoordinates2D<T> = MathVec<T, 2>;

    impl<T: Copy> CartesianCoordinates2D<T> {
        pub fn x(&self) -> &T {
            unsafe { self.data().get_unchecked(0) }
        }

        pub fn y(&self) -> &T {
            unsafe { self.data().get_unchecked(1) }
        }
    }

    pub struct Segment (CartesianCoordinates2D<f64>, CartesianCoordinates2D<f64>);

    pub fn do_intersect(p: Segment, q: Segment) -> bool {
        let o1 = orientation(p.0, q.0, p.1);
        let o2 = orientation(p.0, q.0, q.1);
        let o3 = orientation(p.1, q.1, p.0);
        let o4 = orientation(p.1, q.1, q.0);

        if (o1 != o2) && (o3 != o4) {
            return true
        }

        if (o1 == Orientation::Collinear) && on_segment(&p.0, &p.1, &q.0) { 
            return true
        }

        if (o2 == Orientation::Collinear) && on_segment(&p.0, &q.1, &q.0) { 
            return true
        }

        if (o3 == Orientation::Collinear) && on_segment(&p.1, &p.0, &q.1) { 
            return true 
        }
    
        if (o4 == Orientation::Collinear) && on_segment(&p.1, &q.0, &q.1) {
            return true
        }

        false
    }

    #[derive(PartialEq)]
    enum Orientation {
        Clockwise,
        Counterclockwise,
        Collinear
    }

    fn orientation(p: CartesianCoordinates2D<f64>, q: CartesianCoordinates2D<f64>, r: CartesianCoordinates2D<f64>) -> Orientation {
        let pq = (q.clone() - p).add_dim(0f64);
        let qr = (r - q).add_dim(0f64); 
        let n = pq.cross_product(qr);
        let p = n.data().get(2).unwrap();

        if *p == 0f64 {
            return Orientation::Collinear;
        }

        if *p > 0f64 {
            return Orientation::Clockwise;
        }
        
        return  Orientation::Counterclockwise;
    }

    fn on_segment(p: &CartesianCoordinates2D<f64>, q: &CartesianCoordinates2D<f64>, r: &CartesianCoordinates2D<f64>) -> bool { 
        (*q.x() <= p.x().max(*r.x())) && (*q.x() >= p.x().min(*r.x())) && (*q.y() <= p.y().max(*r.y())) && (*q.y() >= p.y().min(*r.y()))
    }
}
