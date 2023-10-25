use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use game::GamePlugin;

mod game;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin))
        .add_plugins((RapierPhysicsPlugin::<NoUserData>::default(),RapierDebugRenderPlugin::default()))
        .run();
        //for debug //.add_systems(Startup, (setup_graphics, setup_physics))
}


