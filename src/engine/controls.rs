use winit::event::VirtualKeyCode;
use crate::engine::physics::State;
use crate::globals::{JUMP_TICKS, PLAYER_SPEED};
use super::game::Game;
use super::entity::{
    Shape2D, Rectangle, Transform2D, Entity, Triangle, Pentagon
};
/// Run logic on the inputs in [keys_down] HasMap
/// 
pub fn handler(game: &mut Game){
    // This is temporary
    let player = match game.entities[game.players[0]] {
        Shape2D::Rectangle(ref mut r) => {
            r
        }
        _=> todo!()
    };   
    for (key, _) in &game.keys_down {
        match key {
            // Move right
            Some(VirtualKeyCode::D) | Some(VirtualKeyCode::Right)  => { 
                if player.x() >= 1.0 { player.set_x(-1.0) }
                else { 
                    player.shift_x(PLAYER_SPEED * game.dt); 
                }  
            },
            // Move Left
            Some(VirtualKeyCode::A) | Some(VirtualKeyCode::Left)  => { 
                if player.x() <= -1.0 { player.set_x(1.0) }
                else { 
                    player.shift_x(-PLAYER_SPEED * game.dt); }  
            },
            // Jump
            Some(VirtualKeyCode::W) | Some(VirtualKeyCode::Space)  => {
                match player.state() {
                    State::None => {
                        player.set_state(State::Jumping(JUMP_TICKS))
                    }
                    _=> {}
                };
            },
            Some(key) => {
                println!("We dont handle {key:?}");
            }
            None => todo!("WTF: {key:?}")
        };
    }
}