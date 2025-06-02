use bevy::{prelude::*, window::PrimaryWindow};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlanarCursorPosition { value: None });
        app.add_systems(PreUpdate, update_cursor_position);
    }
}

#[derive(Resource)]
pub struct PlanarCursorPosition {
    pub value: Option<Vec2>,
}

fn update_cursor_position(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut position: ResMut<PlanarCursorPosition>,
) {
    let (main_camera, main_camera_transform) = *camera;
    position.value = window.cursor_position().and_then(|pos| {
        main_camera
            .viewport_to_world_2d(main_camera_transform, pos)
            .ok()
    });
}
