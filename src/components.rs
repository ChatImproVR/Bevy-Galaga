use bevy::{
    math::{Vec2, Vec3},
    prelude::Component,
    time::{Timer, TimerMode}
};

// Common

#[derive(Component)]
pub struct Speed{
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable{
    pub despawn: bool,
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

// Bullet

#[derive(Component)]
pub struct BulletToSpawn(pub Vec3);

#[derive(Component)]
pub struct BulletToSpawnTimer(pub Timer);

impl Default for BulletToSpawnTimer{
    fn default() -> Self{
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}