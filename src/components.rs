use bevy::{
    math::{Vec2, Vec3},
    prelude::{Component, Resource},
    render::render_resource::Operations,
    time::{Timer, TimerMode},
};

use rand::{thread_rng, Rng};

use crate::{WinSize, BASE_SPEED};

// Common

#[derive(Component)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub despawn: bool,
    pub player: bool,
    pub enemy: bool,
}

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

// Player

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

// Enemy

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;
