use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub enum CameraMode {
    MODE2D,
    MODE3D,
}

pub struct CameraPlugin(pub CameraMode);

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        match self.0 {
            CameraMode::MODE2D => {
                app.add_systems(Startup, Self::setup_2d_camera);
            }
            _ => unimplemented!("3d not implemented yet"),
        }
    }
}

impl CameraPlugin {
    fn setup_2d_camera(mut commands: Commands) {
        commands.spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            Camera,
        ));
    }
}
