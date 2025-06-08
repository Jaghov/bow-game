use bevy::prelude::*;

use crate::{Screen, gameplay::level::Levels};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ScoreCard>()
        .add_systems(OnEnter(Screen::Gameplay), wipe_scorecard);
}

#[derive(Resource, Default)]
pub struct ScoreCard {
    courses: Vec<CourseScore>,
}
impl ScoreCard {
    /// replace existing card with new course scores
    pub fn wipe_with(&mut self, courses: Vec<CourseScore>) {
        self.courses = courses;
    }
}

pub enum CourseScore {
    /// State when the user has finished a course or
    /// has begun playing a course
    Played { arrows_shot: i32, par: i32 },
    /// State when the user has yet to start a course.
    Unplayed { par: i32 },
}

fn wipe_scorecard(mut scorecard: ResMut<ScoreCard>, levels: Res<Levels>) {
    scorecard.wipe_with(
        levels
            .iter()
            .map(|level| CourseScore::Unplayed { par: level.par() })
            .collect(),
    );

    //todo
}
