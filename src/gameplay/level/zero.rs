use bevy::prelude::*;

use crate::{
    gameplay::level::{LevelProps, Levels},
    world::BLOCK_LEN,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, create_level);
}

fn create_level(mut levels: ResMut<Levels>) {
    //this is the debug level
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
                //gravity column
                sphere!(Gravity, -40., 0.),
                sphere!(Normal, -40., 5.),
                sphere!(Multiplier, -40., -5.),
                //absorber column
                sphere!(Absorber, -30., 0.),
                sphere!(Multiplier, -30., 5.),
                sphere!(Exploder, -30., 10.),
                sphere!(Bouncy, -30., 15.),
                //exploder testing column
                sphere!(Exploder, -20., 0.),
                sphere!(Multiplier, -23., 5.),
                sphere!(Multiplier, -23., -5.),
                sphere!(Multiplier, -17., 5.),
                sphere!(Multiplier, -17., -5.),
                //others
                sphere!(TimeFreeze, -10., 0.),
                sphere!(Normal, 0., 0.),
                // exploder column
                sphere!(Exploder, 10., 0.),
                sphere!(Exploder, 10., 5.),
                sphere!(Exploder, 10., 10.),
                sphere!(Normal, 8., 8.),
                // bouncy column
                sphere!(Bouncy, 20., 0.),
                sphere!(Multiplier, 20., 5.),
                sphere!(Normal, 20., 10.),
                // bouncy timefreeze column
                sphere!(Bouncy, 30., 0.),
                sphere!(TimeFreeze, 30., 5.),
                sphere!(Multiplier, 30., 10.),
                // multiplier column
                sphere!(Multiplier, 40., 0.),
                sphere!(Multiplier, 40., 5.),
                sphere!(Multiplier, 40., 10.),
                sphere!(Normal, 40., 15.),
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

    // this is another debug level
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

    // Level 4: Absorber Introduction - Test absorber gaining properties
    levels.insert(
        4,
        LevelProps::new(
            Some(2),
            vec![
                vert!(10., -3., 3.),
                vert!(-10., -3., 3.),
                horz!(0., -10., 10.),
            ],
            vec![
                sphere!(Absorber, 0., 2.), // Central absorber
                sphere!(Bouncy, -5., 2.),  // Bouncy to be absorbed
                sphere!(Normal, 5., 2.),   // Target to destroy
            ],
        ),
    );

    // Level 5: Multiplier Chain Reaction
    levels.insert(
        5,
        LevelProps::new(
            Some(1),
            vec![horz!(3., -8., 8.), horz!(-3., -8., 8.)],
            vec![
                sphere!(Multiplier, -6., 0.),
                sphere!(Multiplier, -2., 0.),
                sphere!(Multiplier, 2., 0.),
                sphere!(Normal, 6., 0.),
            ],
        ),
    );

    // Level 6: TimeFreeze Chain Reaction
    levels.insert(
        6,
        LevelProps::new(
            Some(1),
            vec![horz!(4., -8., 8.), horz!(-4., -8., 8.)],
            vec![
                sphere!(TimeFreeze, 16., 0.), // Freeze time for repositioning
                sphere!(Exploder, 4., 2.),    // Will explode after time resumes
                sphere!(Normal, 8., 2.),      // Target 1
                sphere!(Normal, 8., -2.),     // Target 2
                sphere!(Normal, 4., -2.),     // Target 3
            ],
        ),
    );

    // Level 7: Gravity Well Puzzle
    levels.insert(
        7,
        LevelProps::new(
            Some(2),
            vec![
                vert!(8., -8., 8.),
                vert!(-8., -8., 8.),
                horz!(8., -8., 8.),
                horz!(-8., -8., 8.),
            ],
            vec![
                sphere!(Gravity, 0., 0.),    // Central gravity well
                sphere!(Normal, 6., 6.),     // Will be pulled toward center
                sphere!(Normal, -6., -6.),   // Will be pulled toward center
                sphere!(Multiplier, 0., 4.), // Multiplier near gravity
            ],
        ),
    );

    // Level 8: Exploder Maze
    levels.insert(
        8,
        LevelProps::new(
            Some(1),
            vec![
                // Maze walls
                vert!(2., -4., 0.),
                vert!(2., 2., 4.),
                vert!(-2., -4., 0.),
                vert!(-2., 2., 4.),
                horz!(1., -6., -2.),
                horz!(-1., 2., 6.),
            ],
            vec![
                sphere!(Exploder, 0., 0.), // Central exploder
                sphere!(Normal, 4., 3.),   // Protected by maze
                sphere!(Normal, -4., -3.), // Protected by maze
            ],
        ),
    );

    // Level 10: Absorber Evolution
    levels.insert(
        10,
        LevelProps::new(
            Some(2),
            vec![
                // L-shaped walls with gaps
                vert!(6., -6., -2.),
                horz!(-2., 0., 6.),
                vert!(-6., -6., -2.),
                horz!(-2., -6., -2.),
            ],
            vec![
                sphere!(Absorber, 0., 0.), // Will evolve
                sphere!(Exploder, 3., 1.), // Give exploder properties
                sphere!(Normal, 8., -4.),  // Target 1
                sphere!(Normal, -8., -4.), // Target 2
                sphere!(Normal, 2., -5.),  // Target 3
                sphere!(Normal, -2., -5.), // Target 4
            ],
        ),
    );

    // Level 11: The Gauntlet - Mixed mechanics
    levels.insert(
        11,
        LevelProps::new(
            Some(3),
            vec![
                // Corridor design
                horz!(4., -12., 12.),
                horz!(-4., -12., 12.),
                vert!(12., -4., 4.),
                vert!(-12., -4., 4.),
            ],
            vec![
                sphere!(TimeFreeze, -8., 0.), // Start with time control
                sphere!(Multiplier, -4., 2.),
                sphere!(Bouncy, 0., 0.),
                sphere!(Absorber, 4., -2.),
                sphere!(Exploder, 8., 0.),
                sphere!(Normal, 10., 2.),
                sphere!(Normal, 10., -2.),
                sphere!(Gravity, 6., 0.),
            ],
        ),
    );

    // Level 12: Spiral Galaxy
    levels.insert(
        12,
        LevelProps::new(
            Some(3),
            vec![
                // Spiral walls
                vert!(0., -1., 1.),
                horz!(2., 0., 4.),
                vert!(4., 1., 5.),
                horz!(4., -4., 0.),
                vert!(-2., -3., 1.),
                horz!(-2., -6., -2.),
            ],
            vec![
                sphere!(Gravity, 0., 0.), // Center of spiral
                sphere!(Normal, 2., 4.),  // Outer ring
                sphere!(Normal, 4., 2.),
                sphere!(Normal, 2., -4.),
                sphere!(Normal, -4., -2.),
                sphere!(Normal, -2., 4.),
                sphere!(Multiplier, 1., 1.),   // Near center
                sphere!(TimeFreeze, -1., -1.), // Near center
            ],
        ),
    );

    // Level 13: Ping Pong Chaos
    levels.insert(
        13,
        LevelProps::new(
            Some(2),
            vec![
                // Ping pong table
                horz!(6., -8., 8.),
                horz!(-6., -8., 8.),
                vert!(8., -6., 6.),
                vert!(-8., -6., 6.),
                // Net
                //vert!(8., -2., 2.),
            ],
            vec![
                sphere!(Bouncy, -4., 0.),    // Left paddle area
                sphere!(Bouncy, 4., 0.),     // Right paddle area
                sphere!(Normal, 0., 4.),     // Above net
                sphere!(Normal, 0., -4.),    // Below net
                sphere!(Multiplier, 2., 0.), // Near net
            ],
        ),
    );

    // Level 14: The Fortress
    levels.insert(
        14,
        LevelProps::new(
            Some(4),
            vec![
                // Outer walls
                vert!(10., -8., 8.),
                vert!(-10., -8., 8.),
                horz!(8., -10., 10.),
                horz!(-8., -10., 10.),
                // Inner fortress
                vert!(4., -4., 4.),
                vert!(-4., -4., 4.),
                horz!(4., -4., 4.),
                horz!(-4., -4., 4.),
                // Gate
                horz!(-4., -1., 1.),
            ],
            vec![
                sphere!(Absorber, 0., 2.),    // King in castle
                sphere!(Exploder, 0., 6.),    // Explosive guard
                sphere!(Gravity, 6., 0.),     // Gravity trap
                sphere!(TimeFreeze, -6., 0.), // Time magic
                sphere!(Normal, 8., 8.),      // Corner guard
                sphere!(Normal, -8., -8.),    // Corner guard
            ],
        ),
    );

    // Level 15: Final Exam - All mechanics
    levels.insert(
        15,
        LevelProps::new(
            Some(5),
            vec![
                // Complex maze
                vert!(6., -10., -2.),
                vert!(6., 2., 10.),
                vert!(-6., -10., -2.),
                vert!(-6., 2., 10.),
                horz!(0., -6., 6.),
                vert!(2., -6., 0.),
                vert!(-2., -6., 0.),
                horz!(4., 2., 10.),
                horz!(-4., -10., -2.),
            ],
            vec![
                // Starting area
                sphere!(TimeFreeze, 0., -8.),
                sphere!(Multiplier, -3., -8.),
                sphere!(Multiplier, 3., -8.),
                // Middle section
                sphere!(Absorber, 0., 2.),
                sphere!(Bouncy, -4., 0.),
                sphere!(Exploder, 4., 0.),
                sphere!(Gravity, 0., 4.),
                // End section
                sphere!(Normal, 8., 8.),
                sphere!(Normal, -8., 8.),
                sphere!(Normal, 8., -8.),
                sphere!(Normal, -8., -8.),
                sphere!(Normal, 0., 8.),
            ],
        ),
    );
}
