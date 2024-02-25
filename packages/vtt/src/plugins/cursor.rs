use crate::entities::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition { x: 0., y: 0. })
            .add_systems(Update, set_cursor_position);
    }
}

pub fn set_cursor_position(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let window = windows.get_single();
    if let Ok(primary_window) = window {
        if let Some(position) = primary_window.cursor_position() {
            cursor_position.x = position.x;
            cursor_position.y = position.y;
        }
    }
}
