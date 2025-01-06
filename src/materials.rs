#[derive(Debug)]
pub struct PipeMaterial {
    /// Absolute roughness of the pipe material in meters
    pub absolute_roughness: f64,
}

#[allow(dead_code)]
pub const COMMERCIAL_STEEL: PipeMaterial = PipeMaterial {
    absolute_roughness: 0.000045,
};

#[derive(Debug)]
pub struct Fluid {
    /// Kinematic viscosity of the fluid in m^2/s
    pub kinematic_viscosity: f64,

    /// Density of the fluid in kg/m^3
    pub density: f64,
}

#[allow(dead_code)]
pub const WATER: Fluid = Fluid {
    kinematic_viscosity: 1.0e-6,
    density: 1000.0,
};

#[allow(dead_code)]
pub const OIL_WTI: Fluid = Fluid {
    kinematic_viscosity: 0.00010735,    // @ 38 degrees C
    density: 894.61,
};

pub struct Component {
    pub minor_loss_coefficient: f64,
}

#[allow(dead_code)]
pub const ELBOW_FLANGED_SHORT_RADIUS_90: Component = Component {
    minor_loss_coefficient: 0.3,
};

#[allow(dead_code)]
pub const ELBOW_FLANGED_SHORT_RADIUS_45: Component = Component {
    minor_loss_coefficient: 0.3,
};

#[allow(dead_code)]
pub const ELBOW_FLANGED_LONG_RADIUS_45: Component = Component {
    minor_loss_coefficient: 0.2,
};

#[allow(dead_code)]
pub const ELBOW_FLANGED_LONG_RADIUS_90: Component = Component {
    minor_loss_coefficient: 0.2,
};
