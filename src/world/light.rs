use std::time::Duration;

use bevy::prelude::*;

use crate::gameplay::GAMEPLAY_CAMERA_OFFSET;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_light)
        .add_observer(start_light_animation);

    app.add_systems(Update, move_light.run_if(resource_exists::<LightAnimation>));
}

#[derive(Event)]
pub struct SetLightPosition {
    to: Transform,
    pub duration: Duration,
}

#[allow(dead_code)]
impl SetLightPosition {
    pub fn to_below() -> Self {
        SetLightPosition {
            to: Transform::from_xyz(4., -50., GAMEPLAY_CAMERA_OFFSET + 5.)
                .looking_at(Vec3::ZERO, Vec3::Y),
            duration: Duration::from_secs(2),
        }
    }
    pub fn to_gameplay() -> Self {
        SetLightPosition {
            to: Transform::from_xyz(0., 50., GAMEPLAY_CAMERA_OFFSET + 5.)
                .looking_at(Vec3::new(0., -50., 0.), Vec3::Y),
            duration: Duration::from_secs(2),
        }
    }
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        SetLightPosition::to_gameplay().to,
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
    light: Query<&Transform, With<DirectionalLight>>,
) {
    info!("Starting light animation");
    commands.remove_resource::<LightAnimation>();
    let event = trigger.event();
    let light = light.single().unwrap();
    let timer = Timer::new(event.duration, TimerMode::Once);
    commands.insert_resource(LightAnimation {
        start: *light,
        end: event.to,
        timer,
    });
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
