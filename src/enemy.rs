use std::time::Duration;

use crate::{
    constants::*,
    score::Points,
    shots::{self, Hit},
    GameState,
};
use bevy::prelude::*;
use rand::Rng as _;

pub struct EnemyPlugin;

const INITIAL_MOVE_DELAY: f32 = 0.6;
const FINAL_MOVE_DELAY: f32 = 0.1;
const MOVE_X: f32 = 4.0;
const MOVE_Y: f32 = 8.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyDirection::default())
            .insert_resource(MoveTimer {
                timer: Timer::from_seconds(1., TimerMode::Repeating),
            })
            .insert_resource(ShootTimer::default())
            .add_systems(OnEnter(GameState::Playing), startup)
            .add_systems(
                Update,
                (update_move_timer, move_enemies, update_temporaries)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                (swap_enemy_direction, shoot).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct ShootTimer(Timer);

#[derive(Component)]
struct Shooter;

#[derive(Component)]
struct NonShooter;

#[derive(Component)]
struct Temporary {
    timer: Timer,
}

#[derive(Resource)]
struct MoveTimer {
    timer: Timer,
}

fn startup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    // enemies
    let enemy_atlas_a = assets.load("sprites/a.png");
    let layout_a =
        TextureAtlasLayout::from_grid(UVec2::splat(16), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout_a = texture_atlas_layout.add(layout_a);

    let enemy_atlas_b = assets.load("sprites/b.png");
    let layout_b =
        TextureAtlasLayout::from_grid(UVec2::new(22, 16), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout_b = texture_atlas_layout.add(layout_b);

    let enemy_atlas_c = assets.load("sprites/c.png");
    let layout_c =
        TextureAtlasLayout::from_grid(UVec2::new(24, 16), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout_c = texture_atlas_layout.add(layout_c);

    // starting position for enemies
    let enemy_start = Vec2::new(
        ENEMY_SIZE.x / 2.0 + ENEMY_WALL_GAP,
        -ENEMY_SIZE.y / 2.0 + ARENA_SIZE.y - 55.,
    );
    for y in 0..5 {
        let mut current_enemy_pos = enemy_start;
        current_enemy_pos.y -= (ENEMY_SIZE.y / 2. + 8.) * y as f32;
        let atlas = match y {
            0 => (&enemy_atlas_a, &texture_atlas_layout_a, Vec2::new(16., 16.)),
            1 | 2 => (&enemy_atlas_b, &texture_atlas_layout_b, Vec2::new(22., 16.)),
            _ => (&enemy_atlas_c, &texture_atlas_layout_c, Vec2::new(24., 16.)),
        };
        let points = match y {
            0 => 30,
            1 | 2 => 20,
            _ => 10,
        };
        for x in 0..11 {
            let mut sprite = Sprite::from_atlas_image(
                atlas.0.clone(),
                TextureAtlas {
                    layout: atlas.1.clone(),
                    index: 0,
                },
            );
            sprite.custom_size = Some(atlas.2 / 2.);

            let mut enemy = cmds.spawn((
                Enemy { x, y },
                sprite,
                Transform::from_translation(current_enemy_pos.extend(0.0)),
                shots::Collider::enemy_layer(),
                Points(points),
            ));

            if y == 4 {
                enemy.insert(Shooter);
            } else {
                enemy.insert(NonShooter);
            }

            enemy.observe(on_hit);

            current_enemy_pos.x += 12. + ENEMY_SPACING;
        }
    }

    let _ = assets.load::<AudioSource>("sounds/enemy-killed.ogg");
}

#[derive(Component, Default, Debug)]
pub struct Enemy {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Resource, PartialEq, Eq)]
pub enum EnemyDirection {
    #[default]
    Right,
    Left,
}

#[derive(Event, Default)]
pub struct EnemyDirectionChanged;

impl EnemyDirection {
    pub fn reverse(&mut self) {
        if *self == EnemyDirection::Right {
            *self = EnemyDirection::Left;
        } else {
            *self = EnemyDirection::Right;
        }
    }

    pub fn as_f32(&self) -> f32 {
        match self {
            EnemyDirection::Right => 1.0,
            EnemyDirection::Left => -1.0,
        }
    }
}

fn update_move_timer(
    mut timer: ResMut<MoveTimer>,
    time: Res<Time>,
    enemies: Query<Entity, With<Enemy>>,
) {
    let enemy_count = enemies.iter().count();
    let percent = 1. - enemy_count as f32 / (11 * 5) as f32;
    let delay = INITIAL_MOVE_DELAY + (FINAL_MOVE_DELAY - INITIAL_MOVE_DELAY) * percent;
    timer.timer.set_duration(Duration::from_secs_f32(delay));
    timer.timer.tick(time.delta());
}

fn move_enemies(
    direction: Res<EnemyDirection>,
    mut transforms: Query<&mut Transform, With<Enemy>>,
    mut sprites: Query<&mut Sprite, With<Enemy>>,
    timer: Res<MoveTimer>,
) {
    if timer.timer.finished() {
        for mut enemy in transforms.iter_mut() {
            enemy.translation.x += MOVE_X * direction.as_f32();
        }
        for mut sprite in sprites.iter_mut() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 1 - atlas.index;
            }
        }
    }
}

fn swap_enemy_direction(
    current: ResMut<EnemyDirection>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
) {
    let direction = current.into_inner();
    let mut needs_reverse = false;
    for enemy in enemies.iter() {
        let x = enemy.translation.x;
        needs_reverse = match direction {
            EnemyDirection::Right => x + ENEMY_SIZE.x / 2. + ENEMY_WALL_GAP >= ARENA_SIZE.x,
            EnemyDirection::Left => x - ENEMY_SIZE.x / 2. - ENEMY_WALL_GAP <= 0.,
        };

        if needs_reverse {
            break;
        }
    }

    if needs_reverse {
        direction.reverse();
        for mut enemy in enemies.iter_mut() {
            enemy.translation.y -= MOVE_Y;
        }
    }
}

impl Default for ShootTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1., TimerMode::Once))
    }
}

fn shoot(
    cmds: Commands,
    assets: Res<shots::EnemyShotSpritesWithAtlas>,
    time: Res<Time>,
    mut timer: ResMut<ShootTimer>,
    enemies: Query<&mut Transform, With<Shooter>>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() {
        timer.0.reset();

        let mut rng = rand::rng();
        let rand = rng.random_range(0..enemies.iter().len());

        let enemy = enemies.iter().nth(rand).unwrap();

        shots::spawn_enemy_shots(
            cmds,
            assets,
            Vec2::new(enemy.translation.x, enemy.translation.y - ENEMY_SIZE.y / 2.),
        );
    }
}

fn on_hit(
    trigger: Trigger<Hit>,
    mut cmds: Commands,
    assets: Res<AssetServer>,
    all_enemies: Query<&Transform, With<Enemy>>,
    shooters: Query<&Enemy, With<Shooter>>,
    non_shooters: Query<(Entity, &Enemy), Without<Shooter>>,
) {
    let entity = trigger.entity();
    let enemy = all_enemies.get(entity).unwrap();

    // spawn explosion
    cmds.spawn((
        Sprite {
            image: assets.load("sprites/enemy-explosion.png"),
            custom_size: Some(Vec2::new(12., 8.)),
            ..default()
        },
        Transform::from_xyz(enemy.translation.x, enemy.translation.y, 0.0),
        Temporary {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
    ));

    // play sound
    cmds.spawn(AudioPlayer::new(assets.load("sounds/enemy-killed.ogg")));

    // despawn enemy
    cmds.entity(entity).despawn();

    // if the hit enemy was a shooter, find the next shooter above and promote it
    if let Ok(coords) = shooters.get(entity) {
        if let Some(promotee) = non_shooters
            .iter()
            .filter(|(_, candidate_coords)| candidate_coords.x == coords.x)
            .max_by_key(|(_, e)| e.y)
        {
            cmds.entity(promotee.0).insert(Shooter);
        }
    }
}

fn update_temporaries(
    mut cmds: Commands,
    mut enemies: Query<(Entity, &mut Temporary)>,
    time: Res<Time>,
) {
    for (entity, mut temporary) in enemies.iter_mut() {
        temporary.timer.tick(time.delta());

        if temporary.timer.finished() {
            cmds.entity(entity).despawn();
        }
    }
}
