use bevy::prelude::*;

pub struct InitPlugin;
impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //TODO! plane to sphere
    //TODO element on the sphere could be child of the sphere
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 5.0,
            sectors: 25,
            stacks: 25,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    /* //TODO
         cube should be the temporary player
        and move around the sphere without levitate or fall
    */
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 5.2, 0.0),
        ..default()
    });
    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(9.0, 13.0, 9.0),
        ..default()
    });
    //TODO! camera should follow the cube
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-12.0, 12.5, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
