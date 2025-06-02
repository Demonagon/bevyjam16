use crate::assets::*;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, update_force_emitters);
}

pub fn projectile(
    position: Vec2,
    direction: Dir2,
    speed: f32,
    meshes: &Res<Meshes>,
    colors: &Res<Colors>,
) -> impl Bundle {
    (
        Transform::from_translation(position.extend(0.0)).with_scale(Vec3::splat(5.0)),
        meshes.circle(),
        colors.white(),
        RigidBody::Dynamic,
        Collider::circle(1.0),
        Restitution::new(1.0),
        CollisionEventsEnabled,
        LinearVelocity::from(direction * speed),
    )
}

pub fn bouncing_wall(
    transform: Transform,
    meshes: &Res<Meshes>,
    colors: &Res<Colors>,
) -> impl Bundle {
    (
        transform,
        meshes.square(),
        colors.white(),
        RigidBody::Kinematic,
        Restitution::new(1.0),
        Collider::rectangle(1.0, 1.0),
    )
}

#[derive(Component)]
#[require(Sensor, CollidingEntities)]
pub struct ForceEmitter {
    pub range: f32,
    pub value: f32,
}

pub fn black_hole(
    position: Vec2,
    radius: f32,
    strength: f32,
    meshes: &Res<Meshes>,
    colors: &Res<Colors>,
) -> impl Bundle {
    (
        Transform::from_translation(position.extend(-1.0)).with_scale(Vec3::splat(radius)),
        meshes.circle(),
        colors.black(),
        RigidBody::Kinematic,
        Collider::circle(1.0),
        ForceEmitter {
            range: radius,
            value: -strength,
        },
    )
}

pub fn white_hole(
    position: Vec2,
    radius: f32,
    strength: f32,
    meshes: &Res<Meshes>,
    colors: &Res<Colors>,
) -> impl Bundle {
    (
        Transform::from_translation(position.extend(-1.0)).with_scale(Vec3::splat(radius)),
        meshes.circle(),
        colors.gray(),
        RigidBody::Kinematic,
        Collider::circle(1.0),
        ForceEmitter {
            range: radius,
            value: strength,
        },
    )
}

pub fn update_force_emitters(
    mut commands: Commands,
    emitters: Query<(&Transform, &CollidingEntities, &ForceEmitter)>,
    objects: Query<(&Transform, Option<&ExternalImpulse>), Without<ForceEmitter>>,
) {
    for (transform, entities, force) in &emitters {
        let position = transform.translation.truncate();

        for entity in entities.iter() {
            let Ok((transform, impulses)) = objects.get(*entity) else {
                continue;
            };

            let entity_position = transform.translation.truncate();

            let distance = position.distance(entity_position);

            if distance <= 0.0 {
                continue;
            }

            let distance_factor = force.range - distance;

            if distance_factor <= 0.0 {
                continue;
            }

            let strength = distance_factor * -force.value;

            let mut impulses = impulses.copied().unwrap_or_default();
            impulses.apply_impulse((position - entity_position).clamp_length(strength, strength));

            commands.entity(*entity).insert(impulses);
        }
    }
}
