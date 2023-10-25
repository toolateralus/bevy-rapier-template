use bevy::prelude::*;
use bevy_rapier3d::rapier::prelude::{RigidBodyBuilder, ColliderBuilder};

#[derive(Bundle)]
struct Player
{
    transform:TransformBundle,
    mesh:PbrBundle,
}


