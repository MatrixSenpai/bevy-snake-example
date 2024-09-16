use bevy::prelude::*;
use crate::{
    ARENA_WIDTH, ARENA_HEIGHT,
    positioning_plugin::{Position, Size},
    objective_plugin::Objective,
};

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

pub struct SnakePlugin;
impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(Time::<Fixed>::from_seconds(0.15))
            .insert_resource(SnakeSegments::default())
            .insert_resource(LastTailPosition::default())
            .add_event::<GrowthEvent>()
            .add_event::<GameOverEvent>()
            .add_systems(Startup, (spawn_snake))
            .add_systems(FixedUpdate, (
                capture_movement_direction,
                snake_movement.after(capture_movement_direction),
                snake_objective_collision.after(snake_movement),
                snake_growth.after(snake_objective_collision),
                game_over.after(snake_growth),
            ));
    }
}

fn spawn_snake(
    mut commands: Commands,
    mut segments: ResMut<SnakeSegments>
) {
    *segments = SnakeSegments(vec![
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(Position { x: 3, y: 3 })
            .insert(Size::default())
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2, }),
    ]);
}

fn capture_movement_direction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_head_query: Query<&mut SnakeHead>,
) {
    if let Some(mut head) = snake_head_query.iter_mut().next() {
        let direction = if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            Direction::Right
        } else if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            Direction::Left
        } else if keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            Direction::Up
        } else if keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            Direction::Down
        } else {
            head.direction
        };

        if direction != head.direction.opposite() {
            head.direction = direction;
        }
    }
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments.0.iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<_>>();

        let mut head_position = positions.get_mut(head_entity).unwrap();

        match &head.direction {
            Direction::Right => head_position.x += 1,
            Direction::Left  => head_position.x -= 1,
            Direction::Up    => head_position.y += 1,
            Direction::Down  => head_position.y -= 1,
        };

        if head_position.x < 0 || head_position.y < 0 || head_position.x as u32 >= ARENA_WIDTH || head_position.y as u32 >= ARENA_HEIGHT {
            game_over_writer.send(GameOverEvent);
        }

        if segment_positions.contains(&head_position) {
            game_over_writer.send(GameOverEvent);
        }

        segment_positions.iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(position, segment)| {
                *positions.get_mut(*segment).unwrap() = *position;
            });

        *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
    }
}

fn snake_objective_collision(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    objective_positions: Query<(Entity, &Position), With<Objective>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_position in head_positions.iter() {
        for (entity, objective_position) in objective_positions.iter() {
            if objective_position == head_position {
                commands.entity(entity).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        segments.0.push(spawn_segment(commands, last_tail_position.0.unwrap()))
    }
}

fn game_over(
    mut commands: Commands,
    mut game_over_reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    heads: Query<Entity, With<SnakeHead>>,
    objectives: Query<Entity, With<Objective>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if game_over_reader.read().next().is_some() {
        info!("Game over!");
        for entity in objectives.iter().chain(segments.iter()).chain(heads.iter()) {
            commands.entity(entity).despawn();
        }

        spawn_snake(commands, segments_res);
    }
}

pub fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_SEGMENT_COLOR,
            ..default()
        },
        ..default()
    })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeSegment;
#[derive(Default, Resource)]
struct SnakeSegments(Vec<Entity>);

#[derive(Default, Resource)]
struct LastTailPosition(Option<Position>);

#[derive(Event)]
struct GrowthEvent;

#[derive(Event)]
struct GameOverEvent;

#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left  => Self::Right,
            Self::Right => Self::Left,
            Self::Up    => Self::Down,
            Self::Down  => Self::Up,
        }
    }
}