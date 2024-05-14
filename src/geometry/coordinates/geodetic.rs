use thiserror::Error;
use uom::si::angle::{degree, radian, Angle};
use uom::si::length::Length;

use crate::generals::tensor::{MathVec, MathVecTrait};
use crate::generals::traits::Pow;

pub const EARTH_RADIUS: f32 = 6371f32;

#[derive(Debug, Clone, Copy)]
pub struct Bearing {
    bearing: f64,
}

impl Bearing {
    pub fn new(bearing: f64) -> Result<Self, SetBearingError> {
        if !(0f64..=360f64).contains(&bearing) {
            return Err(SetBearingError::OutOfRange);
        }

        Ok(Self { bearing })
    }

    pub fn update(&mut self, bearing: f64) -> Result<(), SetBearingError> {
        if !(0f64..=360f64).contains(&bearing) {
            return Err(SetBearingError::OutOfRange);
        }
        self.bearing = bearing;
        Ok(())
    }

    pub fn bearing(&self) -> f64 {
        self.bearing
    }

    pub fn radians(&self) -> f64 {
        (self.bearing / 180f64) * core::f64::consts::PI
    }
}

impl From<Bearing> for AngleF32 {

    fn from(value: Bearing) -> Self {
        Self::new::<degree>(value.bearing as f32)
    }
}

#[derive(Debug, Error)]
pub enum SetBearingError {
    #[error("Bearing given is out of range, it should be from 0 to 360")]
    OutOfRange,
}

type AngleF32 = Angle<uom::si::SI<f32>, f32>;

pub trait GeodeticCoordinate2DTrait {
    fn latitude(&self) -> &AngleF32;
    fn longitude(&self) -> &AngleF32;
}

/// Coordinate in a Geodetic coordinate system only representing a position with no specific
/// altitude
#[derive(Copy, Clone)]
pub struct GeodeticCoordinate2D {
    latitude: AngleF32,
    longitude: AngleF32,
}

#[derive(Debug, Error)]
pub enum CreateGeodeticCoordinate2DError {
    #[error(
        "Invalid latitude, it must be within the range (-90º, 90º] or equivalent in other units"
    )]
    InvalidLatitude,

    #[error(
        "Invalid longitude, it must be within the range (-180ª, 180ª] or equivalent in other units"
    )]
    InvalidLongitude,
}

impl GeodeticCoordinate2D {
    pub fn new(
        latitude: AngleF32,
        longitude: AngleF32,
    ) -> Result<Self, CreateGeodeticCoordinate2DError> {
        if AngleF32::new::<degree>(90f32) < latitude || latitude <= AngleF32::new::<degree>(-90f32)
        {
            return Err(CreateGeodeticCoordinate2DError::InvalidLatitude);
        }

        if longitude > AngleF32::new::<degree>(180f32)
            || longitude <= AngleF32::new::<degree>(-180f32)
        {
            return Err(CreateGeodeticCoordinate2DError::InvalidLongitude);
        }

        Ok(Self {
            latitude,
            longitude,
        })
    }

    pub fn at_ellipsoidal_height(
        self,
        ellipsoidal_height: Length<uom::si::SI<f32>, f32>,
    ) -> GeodeticCoordinate3D {
        GeodeticCoordinate3D {
            position: self,
            ellipsoidal_height,
        }
    }

    pub fn new_from(self, angle: AngleF32, distance: Length<uom::si::SI<f32>, f32>) -> Self {
        let n1: NVector = self.into();
        let angular_distance = distance.get::<uom::si::length::kilometer>() / EARTH_RADIUS;

        let n = NVector::new([0f32, 0f32, 1f32]);
        
        let de = MathVec::new(n.cross_product(n1).unit_vector());
        let dn = n1.cross_product(de);

        let de_sin = de * f32::from(angle.sin());
        let dn_cos = dn * f32::from(angle.sin());

        let d = dn_cos + de_sin;

        let x = n1 * angular_distance.cos();
        let y = d * angular_distance.sin();

        let n2: NVector = x + y;
 
        n2.into()
    }
}

impl GeodeticCoordinate2DTrait for GeodeticCoordinate2D {
    #[inline]
    fn latitude(&self) -> &AngleF32 {
        &self.latitude
    }

    #[inline]
    fn longitude(&self) -> &AngleF32 {
        &self.longitude
    }
}

/// This struct is a representation of geodetic coordinates which represent the outward-pointing unit vector which is normal to that position to the ellipsoid. This crate relies in this type for much of the geometry computation
/// 
/// 
/// See https://en.wikipedia.org/wiki/N-vector
pub(crate) type NVector = crate::generals::tensor::MathVec<f32, 3>;

impl<GeodeticCoordinate: GeodeticCoordinate2DTrait> From<GeodeticCoordinate> for NVector {
    fn from(value: GeodeticCoordinate) -> Self {
        Self::new([(value.latitude().cos() * value.longitude().cos()).into(), (value.latitude().cos() * value.longitude().sin()).into(), value.latitude().sin().into()])
    }
}

impl From<NVector> for GeodeticCoordinate2D {
    fn from(value: NVector) -> Self {
        let value = value.data();
        
        let a: f32 = value[0].pow(2) + value[1].pow(2);
        let b: f32 = a.pow(0.5);

        GeodeticCoordinate2D {
            latitude: AngleF32::new::<radian>((value[2] / b).atan()),
            longitude: AngleF32::new::<radian>((value[1] / value[0]).atan())
        }
    }
}

pub trait GeodeticCoordinate3DTrait {
    fn ellipsoidal_height(&self) -> &Length<uom::si::SI<f32>, f32>;
}

/// Coordinate in a Geodetic coordinate system with ellipsoidal height
pub struct GeodeticCoordinate3D {
    position: GeodeticCoordinate2D,
    ellipsoidal_height: Length<uom::si::SI<f32>, f32>,
}

impl GeodeticCoordinate3D {
    pub fn new(
        latitude: AngleF32,
        longitude: AngleF32,
        ellipsoidal_height: Length<uom::si::SI<f32>, f32>,
    ) -> Result<Self, CreateGeodeticCoordinate2DError> {
        Ok(Self {
            position: GeodeticCoordinate2D::new(latitude, longitude)?,
            ellipsoidal_height,
        })
    }
}

impl GeodeticCoordinate2DTrait for GeodeticCoordinate3D {
    #[inline]
    fn longitude(&self) -> &AngleF32 {
        self.position.longitude()
    }

    #[inline]
    fn latitude(&self) -> &AngleF32 {
        self.position.latitude()
    }
}

impl GeodeticCoordinate3DTrait for GeodeticCoordinate3D {
    #[inline]
    fn ellipsoidal_height(&self) -> &Length<uom::si::SI<f32>, f32> {
        &self.ellipsoidal_height
    }
}
