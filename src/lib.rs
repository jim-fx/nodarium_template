use glam::Vec2;
use wasm_bindgen::prelude::*;

nodarium_macros::include_definition_file!("src/definition.json");

#[wasm_bindgen]
pub fn execute(input: &[i32]) -> Vec<i32> {
    let arguments = nodarium_utils::split_args(input);

    let height = nodarium_utils::evaluate_float(arguments[0]);
    let radius = nodarium_utils::evaluate_float(arguments[1]);

    let mut geometry_data = nodarium_utils::geometry::create_geometry_data(16, 16);

    let geometry = nodarium_utils::geometry::wrap_geometry_data(&mut geometry_data);

    // bottom circle
    for i in 0..8 {
        let x = radius * (2.0 * std::f32::consts::PI * i as f32 / 8.0).cos();
        let y = radius * (2.0 * std::f32::consts::PI * i as f32 / 8.0).sin();

        let vec = Vec2::new(x, y).normalize();

        // bottom circle
        geometry.positions[i * 3 + 0] = x;
        geometry.positions[i * 3 + 1] = 0.0;
        geometry.positions[i * 3 + 2] = y;

        geometry.normals[i * 3 + 0] = vec[0];
        geometry.normals[i * 3 + 1] = 0.0;
        geometry.normals[i * 3 + 2] = vec[1];

        // top circle
        geometry.positions[24 + i * 3 + 0] = x;
        geometry.positions[24 + i * 3 + 1] = height;
        geometry.positions[24 + i * 3 + 2] = y;

        geometry.normals[24 + i * 3 + 0] = vec[0];
        geometry.normals[24 + i * 3 + 1] = 0.0;
        geometry.normals[24 + i * 3 + 2] = vec[1];

        geometry.faces[i * 6 + 0] = (i + 8) as i32;
        geometry.faces[i * 6 + 1] = (i as i32 + 1) % 8;
        geometry.faces[i * 6 + 2] = (i) as i32;

        geometry.faces[i * 6 + 3] = 8 + (i % 8) as i32;
        geometry.faces[i * 6 + 4] = 8 + (i as i32 + 1) % 8;
        geometry.faces[i * 6 + 5] = (i as i32 + 1) % 8;
    }

    nodarium_utils::concat_arg_vecs(vec![geometry_data])
}
