//! Here be physics
//! 
use super::Game;
use super::entity::{Shape2D, Entity, Transform2D};
use crate::globals::{JUMP_SPEED, JUMP_TICKS};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State{
    None,
    Jumping(u32), // Jumping with T time
    Falling(u32),
    // For terrain that has no phsyics applied
    Static,
}
impl State{
    fn handle<T>(&self, shape: &mut T, dt: f32) 
        where T: Transform2D + Entity 
    {
        match self {
            State::None => {}
            State::Jumping(i) => {
                match i {
                    1.. => {  
                        shape.shift_y(JUMP_SPEED * dt); 
                        shape.set_state(State::Jumping(i-1)); 
                    },
                    0 => { 
                        shape.set_state(State::Falling(JUMP_TICKS));  
                    },
                }
            }
            State::Falling(i) => {
                match i {
                    1.. => {  
                        shape.shift_y(-JUMP_SPEED * dt); 
                        shape.set_state(State::Falling(i-1)); 
                    },
                    0 => { 
                        shape.set_state(State::None);  
                    },
                }
            }
            State::Static => {}
        }
    }
}
/// Main collision logic
fn collision(){

}

/// Main physics loop
pub fn update(game: &mut Game){
    for entity in &mut game.entities{
        match entity {
            Shape2D::Rectangle(ref mut shape) => {
                shape.state().handle(shape, game.dt);
            },
            Shape2D::Triangle(ref mut shape) => {
                shape.state().handle(shape, game.dt);
            },
            _=> {}
        };
    }
}