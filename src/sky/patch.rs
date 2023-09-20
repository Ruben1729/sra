#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SkyPatch {
    altitude: f32,
    azimuth: f32,
    azimuth_range: f32,
    irradiance: f32,
}

impl SkyPatch {
    pub fn new(altitude: f32, azimuth: f32) -> Self {
        Self {
            altitude,
            azimuth,
            azimuth_range: 0.,
            irradiance: 0.,
        }
    }

    pub fn set_altitude(&mut self, altitude: f32) {
        self.altitude = altitude;
    }

    pub fn set_azimuth(&mut self, azimuth: f32) {
        self.azimuth = azimuth;
    }

    pub fn set_azimuth_range(&mut self, range: f32) {
        self.azimuth_range = range;
    }

    pub fn set_irradiance(&mut self, irradiance: f32) {
        self.irradiance = irradiance;
    }

    pub fn altitude(&self) -> f32 {
        self.altitude
    }

    pub fn azimuth(&self) -> f32 {
        self.azimuth
    }
    pub fn azimuth_range(&self) -> f32 {
        self.azimuth_range
    }

    pub fn irradiance(&self) -> f32 {
        self.irradiance
    }

    pub fn altitude_deg(&self) -> f32 {
        self.altitude.to_degrees().round()
    }

    pub fn azimuth_deg(&self) -> f32 {
        self.azimuth.to_degrees().round()
    }

    pub fn azimuth_range_deg(&self) -> f32 {
        self.azimuth_range.to_degrees().round()
    }
}
