use bevy::prelude::*;

use crate::{
    Screen,
    gameplay::level::{Level, LevelState, Levels},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ScoreCard>()
        .add_systems(OnEnter(Screen::Gameplay), wipe_scorecard)
        .add_systems(OnEnter(LevelState::NewLevel), start_playing_level)
        .add_observer(count_arrow);
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

    pub fn get_mut(&mut self, course: usize) -> Option<&mut CourseScore> {
        self.courses.get_mut(course)
    }

    pub fn get(&self, course: usize) -> Option<&CourseScore> {
        self.courses.get(course)
    }
}

pub struct CourseScore {
    /// When the user has finished a course or
    /// has begun playing a course, this is some.
    ///
    /// Otherwise, this is none.
    arrows_shot: Option<i32>,
    par: i32,
}
impl CourseScore {
    pub fn arrows_shot(&self) -> Option<i32> {
        self.arrows_shot
    }
    pub fn course_par(&self) -> i32 {
        self.par
    }
}

fn wipe_scorecard(mut scorecard: ResMut<ScoreCard>, levels: Res<Levels>) {
    scorecard.wipe_with(
        levels
            .iter()
            .map(|level| CourseScore {
                arrows_shot: None,
                par: level.par(),
            })
            .collect(),
    );

    //todo
}

#[derive(Event)]
pub struct ArrowCountsTowardsScore;

fn count_arrow(
    _: Trigger<ArrowCountsTowardsScore>,
    mut scorecard: ResMut<ScoreCard>,
    level: Res<Level>,
) {
    let Some(course) = scorecard.get_mut(level.0) else {
        error!("Couldn't get course score for level {}", &*level);
        return;
    };

    match &mut course.arrows_shot {
        Some(shot) => {
            *shot += 1;
        }
        None => {
            course.arrows_shot = Some(1);
        }
    }
}

fn start_playing_level(mut scorecard: ResMut<ScoreCard>, level: Res<Level>) {
    let Some(course) = scorecard.get_mut(level.0) else {
        error!("Couldn't get course score for level {}", &*level);
        return;
    };

    course.arrows_shot = Some(0);
}
