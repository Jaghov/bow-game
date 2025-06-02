use std::time::Duration;

use bevy::prelude::*;

use crate::gameplay::GAMEPLAY_CAMERA_OFFSET;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<LightPositions>()
        .add_systems(Startup, spawn_light)
        .add_observer(start_light_animation);

    app.add_systems(Update, move_light.run_if(resource_exists::<LightAnimation>));
    //todo
}

#[derive(Event)]
pub struct SetLightPosition {
    above: bool,
    pub duration: Duration,
}

#[allow(dead_code)]
impl SetLightPosition {
    pub fn to_above() -> Self {
        SetLightPosition {
            above: true,
            duration: Duration::from_secs(2),
        }
    }
    pub fn to_below() -> Self {
        SetLightPosition {
            above: false,
            duration: Duration::from_secs(2),
        }
    }
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

#[derive(Resource)]
struct LightPositions {
    above: Transform,
    below: Transform,
}
impl Default for LightPositions {
    fn default() -> Self {
        Self {
            above: Transform::from_xyz(0., 50., GAMEPLAY_CAMERA_OFFSET + 5.)
                .looking_at(Vec3::ZERO, Vec3::Y),
            below: Transform::from_xyz(4., -50., GAMEPLAY_CAMERA_OFFSET + 5.)
                .looking_at(Vec3::ZERO, Vec3::Y),
        }
    }
}

fn spawn_light(mut commands: Commands, pos: Res<LightPositions>) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        pos.above,
    ));
}

#[derive(Resource)]
pub struct LightAnimation {
    start: Transform,
    end: Transform,
    timer: Timer,
}

fn start_light_animation(
    trigger: Trigger<SetLightPosition>,
    mut commands: Commands,
    positions: Res<LightPositions>,
    light: Query<&Transform, With<DirectionalLight>>,
) {
    info!("Starting light animation");
    commands.remove_resource::<LightAnimation>();
    let event = trigger.event();
    let light = light.single().unwrap();
    let timer = Timer::new(event.duration, TimerMode::Once);
    let animation = if event.above {
        LightAnimation {
            start: *light,
            end: positions.above,
            timer,
        }
    } else {
        LightAnimation {
            start: *light,
            end: positions.below,
            timer,
        }
    };
    commands.insert_resource(animation);
}
fn move_light(
    mut commands: Commands,
    mut anim: ResMut<LightAnimation>,
    time: Res<Time>,
    mut light: Query<&mut Transform, With<DirectionalLight>>,
) {
    anim.timer.tick(time.delta());
    let Ok(mut light) = light.single_mut() else {
        return;
    };

    let progress = anim.timer.fraction();

    // Apply ease-in-out curve (smoothstep)
    let eased_progress = progress * progress * (3.0 - 2.0 * progress);

    // Interpolate translation
    let new_translation = anim
        .start
        .translation
        .lerp(anim.end.translation, eased_progress);

    // Interpolate rotation
    let new_rotation = anim.start.rotation.slerp(anim.end.rotation, eased_progress);

    // Apply the interpolated transform
    light.translation = new_translation;
    light.rotation = new_rotation;

    if anim.timer.finished() {
        commands.remove_resource::<LightAnimation>();
    }
}
