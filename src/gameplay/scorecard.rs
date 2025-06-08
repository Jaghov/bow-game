use bevy::{
    color::palettes::{css::GREEN, tailwind::RED_700},
    prelude::*,
};

use crate::{
    Screen,
    gameplay::level::{Level, LevelState, Levels},
};

pub const AT_PAR: Color = Color::BLACK;
pub const BELOW_PAR: Color = Color::Srgba(GREEN);
pub const ABOVE_PAR: Color = Color::Srgba(RED_700);

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
    pub fn iter(&self) -> std::slice::Iter<'_, CourseScore> {
        self.courses.iter()
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

    pub fn arrows_shot_ui(&self) -> impl Bundle + use<> {
        let par = self.course_par();
        let mut arrows_fired = Text::default();
        let mut arrows_tc = TextColor(AT_PAR);

        match self.arrows_shot() {
            Some(arrows) => {
                let diff = arrows - par;
                let sign = if diff < 0 {
                    arrows_tc.0 = BELOW_PAR;
                    ""
                } else if diff > 0 {
                    arrows_tc.0 = ABOVE_PAR;
                    "+"
                } else {
                    arrows_tc.0 = AT_PAR;
                    "+"
                };
                arrows_fired.0 = format!("{}({}{})", arrows, sign, diff);
            }
            None => {
                arrows_fired.0 = "".to_string();
                arrows_tc.0 = AT_PAR;
            }
        }

        (arrows_fired, arrows_tc)
    }
    pub fn par_ui(&self) -> impl Bundle + use<> {
        //todod
        //

        (Text::new(self.par.to_string()), TextColor(Color::BLACK))
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
