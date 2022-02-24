//! Here be physics
//! 
use super::Game;
use super::entity::{Entity, Transform2D};
use crate::globals::JUMP_SPEED;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State{
    None,
    Jumping(u32), // Jumping with T time
    Falling,
    // For terrain that has no phsyics applied
    Static,
}
impl State {
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
                        shape.set_state(State::Falling);  
                    },
                }
            }
            State::Falling => {
                if shape.y() > -0.95 {
                    shape.shift_y(-JUMP_SPEED * dt); 
                } else {
                    shape.set_state(State::None);
                }
            }
            State::Static => {}
        }
    }
}
/// Main collision logic
fn collision(game: &mut Game) {

    let player = &mut game.entities[0];

    if player.y() < -0.95 {
        player.set_y(-0.95)
    } 
}

// pub fn grounded<T>(shape: &T) -> bool 
//     where T: Entity + Transform2D 
// {
//     if shape.y() <= 0.95 {
//         true
//     }else{
//         false
//     }
// }

/// Main physics loop
pub fn update(game: &mut Game){

    for entity in &mut game.entities{
        entity.state().handle(entity, game.dt);
    }

    collision(game);
}
