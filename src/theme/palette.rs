use bevy::{ecs::component::Mutable, prelude::*};

use crate::gameplay::{GameSet, stopwatch::Stopwatch};

pub fn plugin(app: &mut App) {
    app.init_resource::<Palette>()
        .add_systems(Update, update_palette.in_set(GameSet::RecordInput))
        .add_systems(
            Update,
            (
                update_palette_colors::<BackgroundColor>,
                update_palette_colors::<TextColor>,
            ),
        );
}

#[derive(Component, Clone, Copy)]
pub struct UsePaletteColor(bool);

impl UsePaletteColor {
    pub fn light() -> Self {
        Self(true)
    }
    pub fn dark() -> Self {
        Self(false)
    }
}

#[derive(Resource)]
pub struct Palette {
    pub light: LinearRgba,
    pub dark: LinearRgba,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            light: LinearRgba::WHITE,
            dark: FULL_TIME_COLOR,
        }
    }
}

fn update_palette(mut palette: ResMut<Palette>, time: Res<Stopwatch>) {
    let color = NO_TIME_COLOR
        + (FULL_TIME_COLOR - NO_TIME_COLOR) * time.remaining().div_duration_f32(time.duration());

    palette.dark = color;
}

trait UsePalette: Component<Mutability = Mutable> {
    fn update(&mut self, color: Color);
    fn is_clear(&self) -> bool;
}

impl UsePalette for BackgroundColor {
    fn is_clear(&self) -> bool {
        self.0.is_fully_transparent()
    }
    fn update(&mut self, color: Color) {
        self.0 = color;
    }
}
impl UsePalette for TextColor {
    fn is_clear(&self) -> bool {
        self.0.is_fully_transparent()
    }
    fn update(&mut self, color: Color) {
        self.0 = color;
    }
}

fn update_palette_colors<T: UsePalette>(
    palette: Res<Palette>,
    mut palette_user: Query<(&mut T, &UsePaletteColor)>,
) {
    for (mut user, color) in &mut palette_user {
        if user.is_clear() {
            continue;
        }
        let UsePaletteColor(light) = color;
        if *light {
            user.update(palette.light.into());
        } else {
            user.update(palette.dark.into());
        }
    }
}

// /// #ddd369
// pub(crate) const LABEL_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);

// /// #fcfbcc
// pub(crate) const HEADER_TEXT: Color = Color::srgb(0.988, 0.984, 0.800);

// /// #ececec
// pub(crate) const BUTTON_TEXT: Color = Color::srgb(0.925, 0.925, 0.925);
// /// #4666bf
// pub(crate) const BUTTON_BACKGROUND: Color = Color::srgb(0.275, 0.400, 0.750);
// /// #6299d1
// pub(crate) const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.384, 0.600, 0.820);
// // #3d4999
// pub(crate) const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.239, 0.286, 0.600);

// /// #2b2c2f, taken from the Bevy website
// pub(crate) const SCREEN_BACKGROUND: Color = Color::srgb(0.16862746, 0.17254902, 0.18431373);

pub(crate) const NO_TIME_COLOR: LinearRgba = LinearRgba::RED;
pub(crate) const FULL_TIME_COLOR: LinearRgba = LinearRgba::BLUE;
