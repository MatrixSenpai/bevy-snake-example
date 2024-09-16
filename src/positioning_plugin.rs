use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct PositionPlugin;
impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, (size_scaling, position_translation));
    }
}

fn size_scaling(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Size, &mut Transform)>,
) {
    let primary_window = window_query.get_single().unwrap();
    for (sprite_size, mut transform) in query.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * primary_window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * primary_window.height(),
            1.0,
        );
    }
}

fn position_translation(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    }

    let primary_window = window_query.get_single().unwrap();
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, primary_window.width(), ARENA_WIDTH as f32),
            convert(pos.y as f32, primary_window.height(), ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self { width: x, height: x }
    }
}
impl Default for Size {
    fn default() -> Self {
        Self::square(0.8)
    }
}
