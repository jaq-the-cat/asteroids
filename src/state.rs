use super::ship::*;
use super::stuff::*;

pub struct State {
    pub ship: Ship,
    pub bullets: Vec<Point>,
    pub asteroids: Vec<i32>,
}
