use crate::generals::tensor::MathVec;
use crate::generals::traits::Zero;

// In this library a vector is the same as a Cartesian coordinates point
pub type CartesianCoordinates2D<T> = MathVec<T, 2>;
pub type CartesianVelocity2D<T> = MathVec<T, 2>;

impl<T: Copy> CartesianCoordinates2D<T> {
    pub fn x(&self) -> &T {
        unsafe { self.data().get_unchecked(0) }
    }

    pub fn y(&self) -> &T {
        unsafe { self.data().get_unchecked(1) }
    }
}

pub struct Segment<T>(pub CartesianCoordinates2D<T>, pub CartesianCoordinates2D<T>);

pub fn do_intersect<
    T: std::cmp::Ord
        + Zero
        + std::ops::SubAssign
        + Copy
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>,
>(
    p: Segment<T>,
    q: Segment<T>,
) -> bool {
    let o1 = orientation(p.0, q.0, p.1);
    let o2 = orientation(p.0, q.0, q.1);
    let o3 = orientation(p.1, q.1, p.0);
    let o4 = orientation(p.1, q.1, q.0);

    if (o1 != o2) && (o3 != o4) {
        return true;
    }

    if (o1 == Orientation::Collinear) && on_segment(&p.0, &p.1, &q.0) {
        return true;
    }

    if (o2 == Orientation::Collinear) && on_segment(&p.0, &q.1, &q.0) {
        return true;
    }

    if (o3 == Orientation::Collinear) && on_segment(&p.1, &p.0, &q.1) {
        return true;
    }

    if (o4 == Orientation::Collinear) && on_segment(&p.1, &q.0, &q.1) {
        return true;
    }

    false
}

#[derive(PartialEq)]
enum Orientation {
    Clockwise,
    Counterclockwise,
    Collinear,
}

fn orientation<
    T: std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + Copy
        + std::ops::SubAssign
        + Zero
        + PartialEq<T>
        + PartialOrd,
>(
    p: CartesianCoordinates2D<T>,
    q: CartesianCoordinates2D<T>,
    r: CartesianCoordinates2D<T>,
) -> Orientation {
    let pq = (q - p).add_dim(T::zero());
    let qr = (r - q).add_dim(T::zero());
    let n = pq.cross_product(qr);
    let p = n.data().get(2).unwrap();

    if *p == T::zero() {
        return Orientation::Collinear;
    }

    if *p > T::zero() {
        return Orientation::Clockwise;
    }

    Orientation::Counterclockwise
}

fn on_segment<T: Copy + std::cmp::Ord>(
    p: &CartesianCoordinates2D<T>,
    q: &CartesianCoordinates2D<T>,
    r: &CartesianCoordinates2D<T>,
) -> bool {
    (*q.x() <= *p.x().max(r.x()))
        && (*q.x() >= *p.x().min(r.x()))
        && (*q.y() <= *p.y().max(r.y()))
        && (*q.y() >= *p.y().min(r.y()))
}
