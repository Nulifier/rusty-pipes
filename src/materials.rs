use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct PipeMaterial {
	/// Identifier of the pipe material
	pub id: String,

	/// Name of the pipe material
	pub name: String,

	/// Absolute roughness of the pipe material in meters
	pub absolute_roughness: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fluid {
	/// Identifier of the fluid
	pub id: String,

	/// Name of the fluid
	pub name: String,

	/// Kinematic viscosity of the fluid in m^2/s
	pub kinematic_viscosity: f64,

	/// Density of the fluid in kg/m^3
	pub density: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
	/// Identifier of the component
	pub id: String,

	/// Name of the component
	pub name: String,

	pub minor_loss_coefficient: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialList {
	pub pipe_materials: Vec<PipeMaterial>,
	pub fluids: Vec<Fluid>,
	pub components: Vec<Component>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialLibrary {
	// pub pipe_materials: Vec<PipeMaterial>,
	// pub fluids: Vec<Fluid>,
	// pub components: Vec<Component>,
	materials: MaterialList,

	#[serde(skip)]
	pipe_material_lookup: BTreeMap<String, usize>,

	#[serde(skip)]
	fluid_lookup: BTreeMap<String, usize>,

	#[serde(skip)]
	component_lookup: BTreeMap<String, usize>,
}

impl MaterialLibrary {
	pub fn new(materials: MaterialList) -> MaterialLibrary {
		let pipe_material_lookup = materials
			.pipe_materials
			.iter()
			.enumerate()
			.map(|(i, pipe_material)| (pipe_material.id.clone(), i))
			.collect();
		let fluid_lookup = materials
			.fluids
			.iter()
			.enumerate()
			.map(|(i, fluid)| (fluid.id.clone(), i))
			.collect();
		let component_lookup = materials
			.components
			.iter()
			.enumerate()
			.map(|(i, component)| (component.id.clone(), i))
			.collect();

		MaterialLibrary {
			materials,
			pipe_material_lookup,
			fluid_lookup,
			component_lookup,
		}
	}

	pub fn get_pipe_material(&self, id: &str) -> &PipeMaterial {
		let index = self.pipe_material_lookup.get(id).unwrap();
		&self.materials.pipe_materials[*index]
	}

	pub fn get_fluid(&self, id: &str) -> &Fluid {
		let index = self.fluid_lookup.get(id).unwrap();
		&self.materials.fluids[*index]
	}

	pub fn get_component(&self, id: &str) -> &Component {
		let index = self.component_lookup.get(id).unwrap();
		&self.materials.components[*index]
	}
}
