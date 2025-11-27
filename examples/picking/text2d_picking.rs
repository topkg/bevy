//! Demonstrates picking for text2d. The picking backend only tests against the
//! text2d bounds, so the 2d text can be picked by clicking on its transparent areas.

use bevy::prelude::*;
use std::fmt::Debug;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// Set up a scene that tests all anchor types.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // spawn black square behind text to show anchor point
    commands
        .spawn((Sprite::from_color(Color::BLACK, Vec2::splat(50.0)),))
        .observe(recolor_on1::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
        .observe(recolor_on1::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
        .observe(recolor_on1::<Pointer<Press>>(Color::srgb(1.0, 1.0, 0.0)))
        .observe(recolor_on1::<Pointer<Release>>(Color::srgb(0.0, 1.0, 1.0)));

    commands
        .spawn((
            Text2d("12345AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string()),
            TextColor(Color::WHITE),
            Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
            TextFont {
                font_size: 96.,
                ..default()
            },
        ))
        .observe(recolor_on::<Pointer<Over>>(Color::srgb(0.0, 1.0, 1.0)))
        .observe(recolor_on::<Pointer<Out>>(Color::srgb(1.0, 1.0, 1.0)))
        .observe(recolor_on::<Pointer<Press>>(Color::srgb(1.0, 1.0, 0.0)))
        .observe(recolor_on::<Pointer<Release>>(Color::srgb(0.0, 1.0, 1.0)));
}

// An observer that changes the target entity's color.
fn recolor_on<E: EntityEvent + Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(On<E>, Query<&mut TextColor>) {
    move |ev, mut text| {
        let Ok(mut c) = text.get_mut(ev.event_target()) else {
            return;
        };
        c.0 = color;
    }
}

fn recolor_on1<E: EntityEvent + Debug + Clone + Reflect>(
    color: Color,
) -> impl Fn(On<E>, Query<&mut Sprite>) {
    move |ev, mut sprites| {
        let Ok(mut sprite) = sprites.get_mut(ev.event_target()) else {
            return;
        };
        sprite.color = color;
    }
}
