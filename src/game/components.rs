use bevy::prelude::*;
use bevy_rapier3d::rapier::prelude::{RigidBodyBuilder, ColliderBuilder};

#[derive(Bundle)]
pub struct Player
{
    pub mesh:PbrBundle,
}