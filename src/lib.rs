// import all the functions from p5_sys
use p5_sys::*;

// import the #[wasm_bindgen] annotation
use wasm_bindgen::prelude::wasm_bindgen;

// This is the state of sketch.
#[wasm_bindgen]
pub struct State {
    // f64 is float or number in javascript.
    x: f64,
}

// this annotation makes this available to js.
#[wasm_bindgen]
pub fn setup() -> State {
    createCanvas(400., 400.);
    background(124.0);
    
    // you can remove return keyword because it is implied for
    // the last expresion at the end of the function
    return State {
        x: 10.0,
    }
}

#[wasm_bindgen]
pub fn draw(state: &mut State) {
    state.x += 5.0;
    rect(state.x, 30.0, 100.0, 150.0);
}