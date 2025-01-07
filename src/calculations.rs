pub fn calc_reynolds_number(flow_velocity: f64, diameter: f64, kinematic_viscosity: f64) -> f64 {
    (flow_velocity * diameter / kinematic_viscosity).max(0.2)
}

#[allow(dead_code)]
pub fn calc_friction_factor_laminar(reynolds_number: f64) -> f64 {
    64.0 / reynolds_number
}

#[allow(dead_code)]
pub fn calc_friction_factor_turbulent(reynolds_number: f64, absolute_roughness: f64, diameter: f64) -> f64 {
    // f = 1.325 / (ln(epsilon/d/3.7 + 5.74 / Re^0.9))^2

    let epsilon_d = absolute_roughness / diameter;
    let a = epsilon_d / 3.7;
    let b = 5.74 / reynolds_number.powf(0.9);
    let c = (a + b).ln().powf(2.0);

    1.325 / c
}

pub fn calc_friction_factor(reynolds_number: f64, absolute_roughness: f64, diameter: f64) -> f64 {
    // Uses the Swamee equation to calculate the friction factor
    // This is to not have a discontinuity at the transition from laminar to turbulent flow
    // See https://en.wikipedia.org/wiki/Darcy_friction_factor_formulae#Swamee_equation

    // f = ((64 / Re)^8 + 9.5 (ln(epsilon/d/3.7 + 5.74 / Re^0.9) - (2500 / Re)^6)^-16)^0.125
    // f = (a           + 9.5 (ln(b                            ) - c            )^-16)^0.125)

    let epsilon_d = absolute_roughness / diameter;

    let a = (64.0 / reynolds_number).powf(8.0);
    let b = epsilon_d / 3.7 + 5.74 / reynolds_number.powf(0.9);
    let c = (2500.0 / reynolds_number).powf(6.0);
    let d = 9.5 * (b.ln() - c).powf(-16.0);

    (a + d).powf(0.125)
}