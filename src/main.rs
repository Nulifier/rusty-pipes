mod constants;
mod materials;
mod network;

fn main() {
	let mut network = crate::network::Network {
		nodes: vec![
			crate::network::Node {
				node_type: crate::network::NodeType::Pump,
				elevation: 10.0,
				pressure: 0.0
			},
			crate::network::Node {
				node_type: crate::network::NodeType::Pump,
				elevation: 0.0,
				pressure: 0.0
			},
		],
		links: vec![crate::network::Pipe {
			start_node_index: 0,
			end_node_index: 1,
			length: 100.0,
			diameter: 0.5,
			absolute_roughness: crate::materials::COMMERCIAL_STEEL.absolute_roughness,
			minor_loss_coefficients: 0.0,
			pump_head: 100.0,
			flow_velocity: 0.0,
		}],
		fluid_density: crate::materials::OIL_WTI.density,
		fluid_kinematic_viscosity: crate::materials::OIL_WTI.kinematic_viscosity,
	};

	// Calculate flow velocity
	network.update();

	println!("Flow velocity: {}", network.links[0].flow_velocity);
}
