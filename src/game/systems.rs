use bevy::prelude::*;

use crate::game::components::*;


// /// for debug / referennce for using Rapier3D
// fn setup_physics(mut commands: Commands) {
//     commands
//         .spawn(Collider::cuboid(100.0, 0.1, 100.0))
//         .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
    
//     commands
//         .spawn(RigidBody::Dynamic)
//         .insert(Collider::ball(0.5))
//         .insert(Restitution::coefficient(0.7))
//         .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
// }

pub fn spawn_camera(
    mut commands: Commands
) {
    let cam = Camera3dBundle
    {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };
    
    commands.spawn(cam);
}

pub fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>
) {
  
    let floor = PbrBundle
    {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(250.0))),
        material: materials.add(Color::CYAN.into()),
        .. default()    
    };
    
    commands.spawn(floor);
    
}