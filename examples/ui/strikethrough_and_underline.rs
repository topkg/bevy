//! This example illustrates UI text with strikethrough and underline decorations

use bevy::{
    color::palettes::css::{GREEN, NAVY, RED, YELLOW},
    prelude::*,
    text::{Customline, Customlines},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let font_size = 320.;
    let height :f32= font_size * 1.5008;

    commands.spawn((
        Text::new("[jpfqyi| hello world,youlqrst "),
        // Just add the `Strikethrough` component to any `Text`, `Text2d` or `TextSpan` and its text will be struck through
        // Strikethrough,
        // StrikethroughColor(RED.into()),
        // Underline,
        Customlines(vec![
            Customline {
                postion: (height * 173. / 1508.).round() as usize,
                color: RED.into(),
                thickness: 1,
            },
            Customline {
                postion: (height * 520. / 1508.).round() as usize,
                color: RED.into(),
                thickness: 1,
            },
            Customline {
                postion: (height * 1187. / 1508.).round() as usize,
                color: RED.into(),
                thickness: 1,
            },
            Customline {
                postion: height.round() as usize,
                color: RED.into(),
                thickness: 1,
            },
        ]),
        TextFont {
            font: asset_server.load("fonts/sassoon-primary.otf"),
            font_size,
            ..default()
        },
        // TextLayout::new_with_justify(Justify::Center),
        // Node {
        //     // position_type: PositionType::Absolute,
        //     // bottom: px(5),
        //     // right: px(5),
        //     ..default()
        // },
        TextBackgroundColor::BLACK,
    ));
}
