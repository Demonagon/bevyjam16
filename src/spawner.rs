use std::time::Duration;

use bevy::{
    ecs::{
        component::{ComponentHook, HookContext, Mutable, StorageType},
        world::DeferredWorld,
    },
    prelude::*,
};

use avian2d::prelude::*;

use crate::{
    assets::*,
    timer::{CreateTimer, Expired},
};

/// The directions in which an atom splits when collided
#[derive(Clone)]
pub struct AtomSplits {
    pub directions: Vec<Dir2>,
}

impl Component for AtomSplits {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;

    fn on_add() -> Option<ComponentHook> {
        Some(|mut deferred_world: DeferredWorld, context: HookContext| {
            deferred_world.commands().entity(context.entity).observe(
                |trigger: Trigger<OnCollisionStart>,
                 split: Query<(&Transform, &AtomSplits)>,
                 mut commands: Commands,
                 meshes: Res<Meshes>,
                 colors: Res<Colors>| {
                    commands.entity(trigger.target()).despawn();
                    commands.entity(trigger.collider).despawn();

                    let Ok((transform, atom_splits)) = split.get(trigger.target()) else {
                        return;
                    };

                    let position = transform.translation;
                    let speed = 250.0;

                    for dir in &atom_splits.directions {
                        let spawn = commands
                            .spawn((
                                crate::objects::projectile(
                                    position.truncate(),
                                    *dir,
                                    speed,
                                    &meshes,
                                    &colors,
                                ),
                                ColliderDisabled,
                            ))
                            .id();

                        commands.after(Duration::from_secs_f32(0.5)).observe(
                            move |_: Trigger<Expired>, mut commands: Commands| {
                                commands.entity(spawn).remove::<ColliderDisabled>();
                            },
                        );
                    }
                },
            );
        })
    }
}

pub fn spawner<D: Into<Dir2>, I: IntoIterator<Item = D>>(
    position: Vec2,
    directions: I,
    meshes: &Res<Meshes>,
    colors: &Res<Colors>,
) -> impl Bundle {
    (
        Transform::from_translation(position.extend(0.0)).with_scale(Vec3::splat(10.0)),
        RigidBody::Static,
        Sensor,
        meshes.circle(),
        colors.black(),
        Collider::circle(1.0),
        CollisionEventsEnabled,
        AtomSplits {
            directions: directions.into_iter().map(|a| a.into()).collect(),
        },
    )
}
