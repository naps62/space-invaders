use std::time::Duration;

use crate::{
    constants::*,
    shots::{self, Hit},
};
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::Rng as _;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyDirection::default())
            .insert_resource(ShootTimer::default())
            .add_systems(Startup, startup)
            .add_systems(
                Update,
                move_enemies.run_if(on_timer(Duration::from_secs_f32(1.))),
            )
            .add_systems(FixedUpdate, (swap_enemy_direction, shoot));
    }
}

#[derive(Resource)]
struct ShootTimer(Timer);

#[derive(Component)]
struct Shooter;

#[derive(Component)]
struct NonShooter;

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

impl Default for ShootTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1., TimerMode::Once))
    }
}

fn shoot(
    cmds: Commands,
    assets: Res<shots::SpriteWithAtlas>,
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
    shooters: Query<&Enemy, With<Shooter>>,
    non_shooters: Query<(Entity, &Enemy), Without<Shooter>>,
) {
    let entity = trigger.entity();

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
