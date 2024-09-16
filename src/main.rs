#![allow(unused, dead_code)]

mod positioning_plugin;
mod objective_plugin;
mod snake_plugin;

use bevy::prelude::*;

use positioning_plugin::*;
use objective_plugin::{ObjectivePlugin, ObjectiveSpawnConfig};
use snake_plugin::SnakePlugin;

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

const BACKGROUND_COLOR: ClearColor = ClearColor(Color::srgb(0.04, 0.04, 0.04));

fn main() {
    dotenv::dotenv().ok();

    App::new()
        .insert_resource(BACKGROUND_COLOR)
        .add_systems(Startup, (setup_camera))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake".into(),
                resolution: (500.0, 500.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((PositionPlugin, ObjectivePlugin, SnakePlugin))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ObjectiveSpawnConfig::default());
}
