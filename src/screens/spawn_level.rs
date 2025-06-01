use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{gameplay::player::Player, level::LevelsLoaded, theme::widgets};

use super::Screen;

/// set to true when the player has spawned, and set to false when leaving gameplay
#[derive(Resource, Default)]
struct LevelSpawningStatus {
    player_spawned: bool,
    instructions_read: bool,
}

pub fn plugin(app: &mut App) {
    // spawn the level in the background, the title screen is valuable time to speed up things
    // we're ready to go as soon as we leave the loading screen.
    app.add_systems(OnExit(Screen::Gameplay), (reset_spawning_status))
        .init_resource::<LevelSpawningStatus>();
    app.add_systems(OnEnter(Screen::SpawnLevel), spawn_spawn_level_screen)
        .add_systems(Update, spawn_player.run_if(in_state(Screen::SpawnLevel)))
        .add_systems(
            Update,
            update_instruction_status
                .run_if(in_state(Screen::SpawnLevel).and(input_just_pressed(KeyCode::KeyQ))),
        )
        .add_systems(
            Update,
            update_ready_to_proceed_text.run_if(in_state(Screen::SpawnLevel)),
        );
}

fn reset_spawning_status(mut status: ResMut<LevelSpawningStatus>) {
    status.player_spawned = false;
    status.instructions_read = false;
}

// spawn the player when
// 1. the level has loaded
// 2. the user has clicked play
//
// This in turn will kick off a set of observers that will eventually create the player camera.
fn spawn_player(
    level_ready: Res<LevelsLoaded>,
    mut status: ResMut<LevelSpawningStatus>,
    mut commands: Commands,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if !status.instructions_read || status.player_spawned {
        return;
    }
    if !level_ready.all_ready() {
        return;
    }
    info!("Spawning player!");
    commands.spawn(Player);
    next_screen.set(Screen::Gameplay);
    status.player_spawned = true;
}

fn update_instruction_status(mut status: ResMut<LevelSpawningStatus>) {
    status.instructions_read = true;
}

#[derive(Component)]
pub struct ReadyToProceed;

fn spawn_spawn_level_screen(mut commands: Commands) {
    commands.spawn((
        widgets::ui_root("Loading Screen"),
        StateScoped(Screen::SpawnLevel),
        children![
            widgets::header("Here's the gist"),
            widgets::label("\
                You're on the clock. You have 5 seconds at the beginning of every round \
                to get to the portal. You don't have time to use \
                the portal. So you have some options:\n\n\
                On the first level, you will be shown all the components:\n\
                - Portal: the method to proceed to the next level. Run through it!\n\
                - Lever: A device used to open the portal. There may be multiple per level. Click it!\n\
                - Timebank: An item that will add a few seconds to your time. Jump on it!\n
                \n\
                When picking up a timebank, you unlock the ability to start the level over.\
                The actions of your previous run will be performed again.\n\
                Press the [E] key to rewind your events.\n
                \n\
                So, if the room requires you press two levers, but they're too far to reach in time, you \
                will need to hit one lever, rewind, and hit the other. Once both levers are pressed in time, \
                you will be able to proceed. Best of luck :)"),
            (widgets::label(""), ReadyToProceed)
        ],
    ));
}

fn update_ready_to_proceed_text(
    level: Res<LevelsLoaded>,
    mut text: Query<&mut Text, With<ReadyToProceed>>,
) {
    let mut text = text.single_mut().unwrap();

    if level.all_ready() {
        text.0 = "Press [Q] to continue".to_string();
    } else {
        text.0 = format!("Loading Rooms {}/{}", level.num_loaded(), level.length());
    }
}
