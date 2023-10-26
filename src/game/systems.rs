use bevy::prelude::*;

use crate::game::components::*;

pub fn camera_controls(
    mut commands: Commands,
    mut cam_query: Query<&mut Transform, With<Camera>>,
    input: Res<Input<KeyCode>>
)
{
    let Ok(mut transform) = cam_query.get_single_mut()
    else { 
        println!("Failed to get transform component on camera.");
        return; 
    };
}
pub fn spawn_player (
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>
)
{
    commands.spawn(Player
    {
        mesh: PbrBundle
        {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: materials.add(Color::AZURE.into()),
            .. default()
        }
    });
}

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