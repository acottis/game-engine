use std::collections::HashMap;

use winit::event::VirtualKeyCode;
use crate::engine::physics::State;
use crate::globals::{JUMP_TICKS, PLAYER_SPEED};
use super::game::Game;
use super::entity::{Shape2D, Transform2D, Entity};
/// handle keypresses
/// 
fn match_key(
    player: &mut Shape2D, 
    keys_down: &HashMap<Option<VirtualKeyCode>, u8>, 
    dt: f32
) {
    for key in keys_down.keys() {
        match key {
            // Move right
            Some(VirtualKeyCode::D) | Some(VirtualKeyCode::Right)  => { 
                // Handle edge of screen
                if player.x() >= 1.1 { player.set_x(-1.1) }
                player.shift_x(PLAYER_SPEED * dt); 
                 
            },
            // Move Left
            Some(VirtualKeyCode::A) | Some(VirtualKeyCode::Left)  => { 
                // Handle edge of screen
                if player.x() <= -1.1 { player.set_x(1.1) }
                player.shift_x(-PLAYER_SPEED * dt);
            },
            // Jump
            Some(VirtualKeyCode::W) | Some(VirtualKeyCode::Space)  => {
                if player.state() ==  State::None {
                        player.set_state(State::Jumping(JUMP_TICKS))
                }
            },
            Some(key) => {
                println!("We dont handle {key:?}");
            },
            None => todo!("WTF: {key:?}")
        };
    }
}
/// Run logic on the inputs in [keys_down] HasMap
/// 
pub fn handle(game: &mut Game){
    // We handle our player no matter what shape

    // If we have no players, dont do any input handling
    if game.players.len() == 0 { return }

    let player = &mut game.entities[game.players[0]];

    match_key(player, &game.keys_down, game.dt)

}