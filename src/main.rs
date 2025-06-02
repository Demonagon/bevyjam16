use bevy::asset::AssetMetaCheck;
use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
// use bevy::math::CompassOctant;
use bevy::prelude::*;

use avian2d::prelude::*;

pub mod assets;
pub mod input;
pub mod objects;
pub mod spawner;
pub mod timer;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics in web builds on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            input::GameInputPlugin,
            PhysicsPlugins::default(),
            timer::plugin,
            objects::plugin,
        ))
        .init_resource::<assets::Colors>()
        .init_resource::<assets::Meshes>()
        .insert_resource(Gravity::ZERO)
        .add_systems(Startup, setup)
        .add_systems(Update, shoot_ball)
        .run();
}

fn setup(mut commands: Commands, meshes: Res<assets::Meshes>, colors: Res<assets::Colors>) {
    commands.spawn(Camera2d);

    for x in -10..10 {
        for y in -10..10 {
            commands.spawn((
                Transform::from_translation(Vec3::new(x as f32 * 100.0, y as f32 * 100.0, -1.0)),
                Text2d(String::from(format!("{}, {}", x, y))),
            ));
        }
    }

    // commands.spawn(spawner::spawner(
    //     Vec2::default(),
    //     [CompassOctant::West, CompassOctant::East],
    //     &meshes,
    //     &colors,
    // ));

    // commands.spawn(spawner::spawner(
    //     Vec2::new(-200.0, 0.0),
    //     [CompassOctant::North, CompassOctant::South],
    //     &meshes,
    //     &colors,
    // ));

    // commands.spawn(spawner::spawner(
    //     Vec2::new(200.0, 0.0),
    //     [CompassOctant::North, CompassOctant::South],
    //     &meshes,
    //     &colors,
    // ));

    // commands.spawn(objects::bouncing_wall(
    //     Transform::from_scale(Vec3::splat(50.0)),
    //     &meshes,
    //     &colors,
    // ));

    commands.spawn(objects::white_hole(
        Vec2::default(),
        200.0,
        10.0,
        &meshes,
        &colors,
    ));
}

fn shoot_ball(
    mouse_position: Res<input::PlanarCursorPosition>,
    meshes: Res<assets::Meshes>,
    colors: Res<assets::Colors>,
    mut clicks: EventReader<MouseButtonInput>,
    mut commands: Commands,
) {
    let start_point = Vec2::new(0.0, -300.0);
    let Some(mouse_position) = mouse_position.value else {
        return;
    };

    let speed = 250.0;

    for click in clicks.read() {
        if click.button != MouseButton::Left || click.state != ButtonState::Pressed {
            continue;
        }

        let target = Dir2::new(mouse_position - start_point).unwrap();

        commands.spawn(objects::projectile(
            start_point,
            target,
            speed,
            &meshes,
            &colors,
        ));
    }
}
