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

// #[derive(Clone, Component)]
// pub struct Formation {
//     pub start: (f32, f32),
//     pub radius: (f32, f32),
//     pub pivot: (f32, f32),
//     pub speed: f32,
//     pub angle: f32,
// }

// #[derive(Default, Resource)]
// pub struct FormationMaker {
//     current_format: Option<Formation>,
//     current_members: u32,
// }

// impl FormationMaker {
//     pub fn make(&mut self, win_size: &WinSize) -> Formation {
//         if (&self.current_format == Some(tmpl) &&
//             !(self.current_members >= FORMATION_MEMBERS_MAX),
//         )
//         {
//             self.current_members += 1;
//             tmpl.clone()
//         } else {
//             let mut rng = thread_rng();

//             // get the start value
//             let w_span = win_size.w / 2.0 + 100.;
//             let h_span = win_size.h / 2.0 + 100.;

//             let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
//             let y = rng.gen_range(-h_span..h_span) as f32;
//             let start = (x, y);

//             // get the pivot x and y
//             let w_span = win_size.w / 4.;
//             let h_span = win_size.h / 3. - 50.;
//             let pivot = (rng.gen_range(-w_span..w_span), rng.gen_range(-0.0..h_span));

//             // get the radius
//             let radius = (rng.gen_range(80.0..150.0), 100.);

//             // get the start angle
//             let angle = (y - pivot.1).atan2(x - pivot.0);

//             // get the speed
//             let speed = rng.gen_range(0.5..1.5) * BASE_SPEED;

//             // create formation
//             let formation = Formation {
//                 start,
//                 pivot,
//                 radius,
//                 speed,
//                 angle,
//             };

//             // store template
//             self.current_format = Some(formation.clone());

//             // reset members to 1
//             self.current_members = 1;

//             formation
//         }
//     }
// }

// Bullet

#[derive(Component)]
pub struct BulletToSpawn(pub Vec3);

#[derive(Component)]
pub struct BulletToSpawnTimer(pub Timer);

impl Default for BulletToSpawnTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}
