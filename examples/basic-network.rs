use rusty_pipes::*;

pub fn load_materials() -> materials::MaterialList {
	let material_str = include_str!("materials.json");
	serde_json::from_str(material_str).unwrap()
}

fn main() {
	let materials = materials::MaterialLibrary::new(load_materials());

	let mut network = network::Network {
		nodes: vec![
			network::Node {
				node_type: network::NodeType::Pump,
				elevation: 10.0,
				pressure: 0.0,
			},
			crate::network::Node {
				node_type: network::NodeType::Pump,
				elevation: 0.0,
				pressure: 0.0,
			},
		],
		links: vec![network::Pipe {
			start_node_index: 0,
			end_node_index: 1,
			length: 100.0,
			diameter: 0.5,
			pipe_material: materials.get_pipe_material("COMMERCIAL_STEEL"),
			minor_loss_coefficients: 0.0,
			pump_head: 10.0,
			flow_velocity: 0.0,
		}],
		fluid: &materials.get_fluid("OIL_WTI"),
	};

	// Calculate flow velocity
	for _ in 0..50 {
		network.update();
	}

	println!("Flow velocity: {}", network.links[0].flow_velocity);
}
