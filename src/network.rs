use crate::constants::GRAVITY;
use crate::materials::{PipeMaterial, Fluid};
use crate::calculations;

#[derive(Debug)]
pub enum NodeType {
	Pump,
}

#[derive(Debug)]
pub struct Node {
	pub node_type: NodeType,
	pub elevation: f64,

	///////////////////////////////////
	// State variables

	/// Fluid pressure at the node in kilopascals
	pub pressure: f64,
}

#[derive(Debug)]
pub struct Pipe<'a> {
	pub start_node_index: usize,
	pub end_node_index: usize,

	/// Length of the pipe in meters
	pub length: f64,

	// Diameter of the pipe in meters
	pub diameter: f64,

	pub pipe_material: &'a PipeMaterial,

	pub minor_loss_coefficients: f64,

	/// Pump head pressure at start of pipe in kilopascals
	pub pump_head: f64,

	///////////////////////////////////
	// State variables

	/// Flow velocity in m/s
	pub flow_velocity: f64,
}

#[derive(Debug)]
pub struct Network<'a> {
	pub nodes: Vec<Node>,
	pub links: Vec<Pipe<'a>>,

	pub fluid: &'a Fluid,
}

impl Network<'_> {
	pub fn update(&mut self) {
		for link in self.links.iter_mut() {
			let start_node = &self.nodes[link.start_node_index];
			let end_node = &self.nodes[link.end_node_index];

			let head_pump = link.pump_head;
			let head_dynamic = (start_node.pressure - end_node.pressure) / (self.fluid.density * GRAVITY);
			let head_difference = end_node.elevation - start_node.elevation;

			let numerator = 2.0 * GRAVITY * (head_pump - head_dynamic - head_difference);

			let reynolds_number = calculations::calc_reynolds_number(link.flow_velocity, link.diameter, self.fluid.kinematic_viscosity);
			let friction_factor = calculations::calc_friction_factor(reynolds_number, link.pipe_material.absolute_roughness, link.diameter);

			let denominator = friction_factor * link.length / link.diameter + link.minor_loss_coefficients;

			// Calculate flow velocity
			link.flow_velocity = (numerator / denominator).sqrt();
		}
	}
}
