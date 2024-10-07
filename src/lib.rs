use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct RobotPosition {
    pub x: usize,
    pub y: usize,
}

#[wasm_bindgen]
pub fn robot_position_to_string(robot_position: RobotPosition) -> String {
    format!("{:?}", robot_position)
}
