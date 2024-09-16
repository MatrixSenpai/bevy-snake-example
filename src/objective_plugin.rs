use std::time::Duration;
use bevy::prelude::*;
use rand::prelude::*;

use crate::{ARENA_WIDTH, ARENA_HEIGHT};
use crate::positioning_plugin::{Position, Size};

const OBJECTIVE_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

pub struct ObjectivePlugin;
impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (objective_spawner));
    }
}

fn objective_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<ObjectiveSpawnConfig>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: OBJECTIVE_COLOR,
                ..default()
            },
            ..default()
        })
            .insert(Objective)
            .insert(Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            })
            .insert(Size::default());
    }
}

#[derive(Component)]
pub struct Objective;

#[derive(Resource)]
pub struct ObjectiveSpawnConfig {
    timer: Timer,
}
impl Default for ObjectiveSpawnConfig {
    fn default() -> Self {
        Self { timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating) }
    }
}