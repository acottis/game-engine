use std::collections::HashMap;

use winit::event::VirtualKeyCode;
use crate::engine::physics::State;
use crate::globals::{JUMP_TICKS, PLAYER_SPEED};
use super::game::Game;
use super::entity::{Shape2D, Transform2D, Entity};
/// handle keypresses
/// 
fn match_key<T>(
    player: &mut T, 
    keys_down: &HashMap<Option<VirtualKeyCode>, u8>, 
    dt: f32
) where T: Transform2D + Entity {
    for key in keys_down.keys() {
        match key {
            // Move right
            Some(VirtualKeyCode::D) | Some(VirtualKeyCode::Right)  => { 
                if player.x() >= 1.0 { player.set_x(-1.0) }
                else { 
                    player.shift_x(PLAYER_SPEED * dt); 
                }  
            },
            // Move Left
            Some(VirtualKeyCode::A) | Some(VirtualKeyCode::Left)  => { 
                if player.x() <= -1.0 { player.set_x(1.0) }
                else { 
                    player.shift_x(-PLAYER_SPEED * dt); }  
            },
            // Jump
            Some(VirtualKeyCode::W) | Some(VirtualKeyCode::Space)  => {
                if player.state() ==  State::None {
                        player.set_state(State::Jumping(JUMP_TICKS))
                }
            },
            Some(key) => {
                println!("We dont handle {key:?}");
            }
            None => todo!("WTF: {key:?}")
        };
    }
}
/// Run logic on the inputs in [keys_down] HasMap
/// 
pub fn handler(game: &mut Game){
    // We handle our player no matter what shape
    match game.entities[game.players[0]] {
        Shape2D::Rectangle(ref mut shape) => {
            match_key(shape, &game.keys_down, game.dt)
        },
        Shape2D::Triangle(ref mut shape) => {
            match_key(shape, &game.keys_down, game.dt)
        }
        _=> todo!()
    };

}