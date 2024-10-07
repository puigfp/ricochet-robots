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
impl RobotPosition {
    #[wasm_bindgen(constructor)]
    pub fn new(x: usize, y: usize) -> RobotPosition {
        RobotPosition { x, y }
    }

    // We can't generate WASM bindings for functions of the Display trait!
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
