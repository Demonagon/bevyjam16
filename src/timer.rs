//! A mini module providing a command method, `after`, which
//! allows the creation of simple delayed commands using observers.
//! To use it, do :
//! commands.after(duration).observe(observer);
//! where the observer reacts to the ZST event Expired.

use std::time::Duration;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_automatic_timers);
}

#[derive(Event)]
pub struct Expired;

#[derive(Component)]
pub struct AutomaticTimer {
    pub timer: Timer,
}

pub fn update_automatic_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: Query<(Entity, &mut AutomaticTimer)>,
) {
    let delta = time.delta();
    for (entity, mut timer) in &mut timers {
        if timer.timer.tick(delta).just_finished() {
            commands.trigger_targets(Expired, [entity]);
            commands.entity(entity).despawn();
        }
    }
}

pub trait CreateTimer {
    fn after(&mut self, duration: Duration) -> EntityCommands;
}

impl<'w, 's> CreateTimer for Commands<'w, 's> {
    fn after(&mut self, duration: Duration) -> EntityCommands {
        self.spawn(AutomaticTimer {
            timer: Timer::new(duration, TimerMode::Once),
        })
    }
}
