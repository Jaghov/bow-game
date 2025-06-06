use bevy::prelude::*;

use crate::{
    gameplay::level::{LevelProps, Levels},
    world::BLOCK_LEN,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, create_level);
}

fn create_level(mut levels: ResMut<Levels>) {
    levels.insert(
        0,
        LevelProps::new(
            None,
            vec![
                vert!(8., -5., 5.),
                horz!(6., -8., 8.),
                vert!(-8., -5., 5.),
                horz!(-6., -8., 8.),
            ],
            vec![
                sphere!(Gravity, -40., 0.),
                sphere!(Absorber, -30., 0.),
                sphere!(TimeFreeze, -10., 0.),
                sphere!(Normal, 0., 0.),
                sphere!(Exploder, 10., 0.),
                sphere!(Exploder, 10., 5.),
                sphere!(Exploder, 10., 10.),
                sphere!(Normal, 8., 8.),
                sphere!(Bouncy, 20., 0.),
                sphere!(Multiplier, 40., 0.),
            ],
        ),
    );

    levels.insert(
        1,
        LevelProps::new(
            Some(1),
            vec![
                vert!(6., -5., 5.),
                horz!(6., -6., 6.),
                vert!(-6., -5., 5.),
                horz!(-6., -6., 6.),
            ],
            vec![sphere!(Normal, 5., 5.)],
        ),
    );

    levels.insert(
        2,
        LevelProps::new(
            Some(1),
            vec![
                vert!(8., -5., 5.),
                horz!(6., -8., 8.),
                vert!(-8., -5., 5.),
                horz!(-6., -8., 8.),
            ],
            vec![
                sphere!(Multiplier, 5., 0.),
                sphere!(Normal, 10., 0.),
                sphere!(Normal, 10., 5.),
                sphere!(Normal, 10., -5.),
            ],
        ),
    );

    let mut l3_spheres = Vec::new();
    let range = (BLOCK_LEN * 3.5).round() as i32;
    let domain = (BLOCK_LEN * 2.).round() as i32;
    for i in (-range..range) {
        let x = i as f32 * 2.2;
        for j in (-domain..domain) {
            let y = j as f32 * 2.2;
            l3_spheres.push(sphere!(Exploder, x, y));
        }
    }

    levels.insert(
        3,
        LevelProps::new(
            Some(1),
            vec![
                vert!(8., -5., 5.),
                horz!(6., -8., 8.),
                vert!(-8., -5., 5.),
                horz!(-6., -8., 8.),
            ],
            l3_spheres,
        ),
    );
}
