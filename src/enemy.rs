use std::time::Duration;

use crate::constants::*;
use bevy::{
    prelude::*,
    time::{common_conditions::on_timer, Stopwatch},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyDirection::default())
            .insert_resource(EnemyShootTimer::default())
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                move_enemies.run_if(on_timer(Duration::from_secs_f32(0.5))),
            )
            .add_systems(FixedUpdate, (swap_enemy_direction, shoot));
    }
}

fn startup(
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    // enemies
    let enemy_atlas_a = assets.load("a.png");
    let layout_a =
        TextureAtlasLayout::from_grid(UVec2::splat(16), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout_a = texture_atlas_layout.add(layout_a);

    let enemy_atlas_b = assets.load("b.png");
    let layout_b =
        TextureAtlasLayout::from_grid(UVec2::new(22, 16), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout_b = texture_atlas_layout.add(layout_b);

    let enemy_atlas_c = assets.load("c.png");
    let layout_c =
        TextureAtlasLayout::from_grid(UVec2::new(24, 16), 2, 1, Some(UVec2::splat(1)), None);
    let texture_atlas_layout_c = texture_atlas_layout.add(layout_c);

    // starting position for enemies
    let enemy_start = Vec2::new(
        ENEMY_SIZE.x / 2.0 + ENEMY_WALL_GAP,
        -ENEMY_SIZE.y / 2.0 + ARENA_SIZE.y - ENEMY_WALL_GAP,
    );
    for y in 0..5 {
        let mut current_enemy_pos = enemy_start;
        current_enemy_pos.y -= (ENEMY_SIZE.y / 2. + 8.) * y as f32;
        let atlas = match y {
            0 => (&enemy_atlas_a, &texture_atlas_layout_a, Vec2::new(16., 16.)),
            1 | 2 => (&enemy_atlas_b, &texture_atlas_layout_b, Vec2::new(22., 16.)),
            _ => (&enemy_atlas_c, &texture_atlas_layout_c, Vec2::new(24., 16.)),
        };
        for _x in 0..11 {
            let mut sprite = Sprite::from_atlas_image(
                atlas.0.clone(),
                TextureAtlas {
                    layout: atlas.1.clone(),
                    index: 0,
                },
            );
            sprite.custom_size = Some(atlas.2 / 2.);

            cmds.spawn(EnemyBundle::new(current_enemy_pos, sprite));
            current_enemy_pos.x += 12. + ENEMY_SPACING;
        }
    }
}

#[derive(Component, Default)]
pub struct Enemy;

#[derive(Bundle, Default)]
pub struct EnemyBundle {
    enemy: Enemy,
    sprite: Sprite,
    transform: Transform,
}

impl EnemyBundle {
    pub fn new(location: Vec2, sprite: Sprite) -> Self {
        Self {
            sprite,
            transform: Transform::from_translation(location.extend(0.0)),
            ..default()
        }
    }
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

#[derive(Resource)]
pub struct EnemyShootTimer(Stopwatch);

impl Default for EnemyShootTimer {
    fn default() -> Self {
        Self(Stopwatch::new())
    }
}

fn move_enemies(
    direction: Res<EnemyDirection>,
    mut transforms: Query<&mut Transform, With<Enemy>>,
    mut sprites: Query<&mut Sprite, With<Enemy>>,
) {
    for mut enemy in transforms.iter_mut() {
        enemy.translation.x += 1.0 * direction.as_f32();
    }
    for mut sprite in sprites.iter_mut() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = 1 - atlas.index;
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
            enemy.translation.y -= 5.0;
        }
    }
}

fn shoot(
    mut commands: Commands,
    mut enemies: Query<(&mut Transform, &mut EnemyShootTimer)>,
    time: Res<Time>,
    mut stopwatch: ResMut<EnemyShootTimer>,
) {
    stopwatch.0.tick(time.delta());
}
