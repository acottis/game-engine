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
}

pub fn update(game: &mut Game){
    for entity in &mut game.entities{
        match entity {
            Shape2D::Rectangle(ref mut r) => {
                match r.state() {
                    State::None => {}
                    State::Jumping(i) => {
                        match i {
                            1.. => {  
                                r.shift_y(JUMP_SPEED * game.dt); 
                                r.set_state(State::Jumping(i-1)); 
                            },
                            0 => { 
                                r.set_state(State::Falling(JUMP_TICKS));  
                            },
                        }
                    }
                    State::Falling(i) => {
                        match i {
                            1.. => {  
                                r.shift_y(-JUMP_SPEED * game.dt); 
                                r.set_state(State::Falling(i-1)); 
                            },
                            0 => { 
                                r.set_state(State::None);  
                            },
                        }
                    }
                }
            },
            _=> {}
        };
    }
}