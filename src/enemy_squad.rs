use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct EnemySquadBundle {
    squad: EnemySquad,
    transform: Transform,
}

#[derive(Component, Default)]
pub struct EnemySquad;
