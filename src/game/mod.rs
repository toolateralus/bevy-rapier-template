use bevy::prelude::*;

use self::systems::*;

pub struct GamePlugin;

mod components;
mod systems;


impl Plugin for GamePlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_camera));
    }
}