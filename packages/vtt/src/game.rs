use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::plugins;

pub struct Game {
    app: App,
}

impl Game {
    pub fn new(canvas_id: String) -> Self {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                canvas: Some(canvas_id),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(plugins::camera::CameraPlugin(
            plugins::camera::CameraMode::MODE2D,
        ))
        .add_systems(Startup, setup);

        Self { app }
    }

    pub fn run(&mut self) {
        self.app.run()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        ..default()
    });
}
