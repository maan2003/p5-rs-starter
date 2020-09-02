// import all the functions from p5_sys
use p5_sys::*;

// import the #[wasm_bindgen] annotation
use wasm_bindgen::prelude::*;

const CANVAS_HEIGHT: f64 = 400;
const CANVAS_WIDTH : f64 = 400; 

// This is the state of sketch.
#[wasm_bindgen]
pub struct State {
    // f64 is float or number in javascript.
    x: f64, 
    vel: f64 //velocity of the rectangle. 
}

// this annotation makes this available to js.
#[wasm_bindgen]
pub fn setup() -> State {
    createCanvas(CANVAS_WIDTH, CANVAS_HEIGHT); 
    
    // you can remove return keyword because it is implied for
    // the last expresion at the end of the function
    return State {
        x: 10.0,
        vel: 5.0
    }
}

#[wasm_bindgen]
pub fn draw(state: &mut State) {
    background(124.0);

    const RECT_WIDTH : f64 = 100.0;
    const RECT_HEIGHT: f64 = 150.0;
    
    state.x += state.vel; 

    //if the rectangle is outside/at the edge of the canvas...
    if state.x + RECT_WIDTH >= CANVAS_WIDTH || state.x <= 0 { 
        state.vel = -state.vel;
        //invert the direction.
        //makes the rectangle move in the opposite direction.
    } 

    rect(state.x, 30.0, RECT_WIDTH, RECT_HEIGHT);
}