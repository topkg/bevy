//! This example illustrates UI text with strikethrough and underline decorations

use bevy::{
    color::palettes::css::{GREEN, NAVY, RED, YELLOW},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new("struck\nstruck"),
        // Just add the `Strikethrough` component to any `Text`, `Text2d` or `TextSpan` and its text will be struck through
        Strikethrough,
        Underline,
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 67.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            right: px(5),
            ..default()
        },
        TextBackgroundColor::BLACK,
    ));
}
