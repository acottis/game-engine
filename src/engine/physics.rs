//! Here be physics
//! 
use super::Game;
use super::entity::{Entity, Transform2D};
use crate::globals::{PLAYER_SPEED, JUMP_SPEED};

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

    let player = game.entities[0];
    let player_y = player.y();
    let player_x = player.x();
    let player_max_x = player.max_x();
    let mut new_player_y: f32 = player_y;

    // IF your next movement takes you below a collision
    // THEN snap to that collision
    for entity in &game.entities{
        if entity.collides() == true {
            if player_y < entity.max_y() &&
            player_y + (JUMP_SPEED * game.dt) >= entity.max_y() &&
            player_max_x > entity.x() && 
            player_x < entity.max_x()
            {
                new_player_y = entity.max_y();
                println!("Collision!");
            }
        }
        //     if (old_player_y < entity.max_y()) && 
        //         (old_player_x > entity.x() && old_player_x < entity.max_x()){
                    
        //         if (entity.max_y() - new_player_y) < distance {
        //             new_player_y = entity.max_y();
        //             distance = entity.max_y() - new_player_y
        //         }
        //     }
        // }
    }

    let player = &mut game.entities[0];
    player.set_y(new_player_y)
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
