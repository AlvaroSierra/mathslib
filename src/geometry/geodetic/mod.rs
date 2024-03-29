use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Bearing {
    bearing: f64
}

impl Bearing {
    pub fn new(bearing: f64) -> Result<Self, SetBearingError> { 
        if 0f64 > bearing || bearing > 360f64 {
            return Err(SetBearingError::OutOfRange)
        }

        Ok(Self { bearing })
    }
    
    pub fn update(&mut self, bearing: f64) -> Result<(), SetBearingError> { 
        if 0f64 > bearing || bearing > 360f64 {
            return Err(SetBearingError::OutOfRange)
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

#[derive(Debug, Error)]
pub enum SetBearingError {
    #[error("Bearing given is out of range, it should be from 0 to 360")]
    OutOfRange,
}
