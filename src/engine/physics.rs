//! Here be physics
//! 
use super::Game;
use super::entity::{Entity, Transform2D};
use crate::globals::JUMP_SPEED;

#[derive(Debug, Clone, Copy)]
pub struct Physics {
    pub state: State,
    pub collides: bool,
}

impl Physics {
    pub fn new(state: State, collides: bool) -> Self {
        Self {
            state,
            collides
        }
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self { state: State::Static, collides: true }
    }
}

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
            State::None => {
                shape.shift_y(-JUMP_SPEED * dt); 
            }
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
                shape.shift_y(-JUMP_SPEED * dt); 
            }
            State::Static => {}
        }
    }
}
/// Main collision logic
fn collision(game: &mut Game) {

    let player = &game.entities[0];
    let mut new_player_y: Option<f32> = None;

    // IF your next movement takes you below a collision
    // THEN snap to that collision
    for entity in &game.entities{
        if  entity.collides() == true &&
            player.y() < entity.max_y() &&
            player.y() + (JUMP_SPEED * game.dt) >= entity.max_y() &&
            player.max_x() > entity.x() && player.x() < entity.max_x()
        {
            new_player_y = Some(entity.max_y());   
        }
    }

    let player = &mut game.entities[0];
    // If we found a collision we snap the player to the edge
    if let Some(new_y) = new_player_y{
        player.set_y(new_y);
        player.set_state(State::None);
    }

    // Hack to stop us going off the screen
    // Need to think about this
    if player.y() < -0.95{
        player.set_y(-0.95);
    }
}

/// Main physics loop
pub fn update(game: &mut Game){

    for entity in &mut game.entities{
        entity.state().handle(entity, game.dt);
    }

    collision(game);
}
