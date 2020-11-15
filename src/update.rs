use std::collections::HashMap;

use super::state::*;

pub fn update(state: &mut State, input: &HashMap<char, bool>) {
    if let Some(&v) = input.get(&'a') {
        if v {
            state.ship.rotate(-5.0);
        }
    }
    if let Some(&v) = input.get(&'d') {
        if v {
            state.ship.rotate(5.0);
        }
    }
    if let Some(&v) = input.get(&'w') {
        if v {
            state.ship.accelerate(0.05);
        }
    }
    state.ship.tick();
    //state.ship.rotate(1.0);
}
