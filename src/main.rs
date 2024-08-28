// // // disable console on windows for release builds
// // #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// // use bevy::asset::AssetMetaCheck;
// // use bevy::prelude::*;
// // use bevy::window::PrimaryWindow;
// // use bevy::winit::WinitWindows;
// // use bevy::DefaultPlugins;
// // use bevy_game::GamePlugin; // ToDo: Replace bevy_game with your new crate name.
// // use std::io::Cursor;
// // use winit::window::Icon;

// // fn main() {
// //     App::new()
// //         .insert_resource(Msaa::Off)
// //         .insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)))
// //         .add_plugins(
// //             DefaultPlugins
// //                 .set(WindowPlugin {
// //                     primary_window: Some(Window {
// //                         title: "Bevy game".to_string(), // ToDo
// //                         // Bind to canvas included in `index.html`
// //                         canvas: Some("#bevy".to_owned()),
// //                         fit_canvas_to_parent: true,
// //                         // Tells wasm not to override default event handling, like F5 and Ctrl+R
// //                         prevent_default_event_handling: false,
// //                         ..default()
// //                     }),
// //                     ..default()
// //                 })
// //                 .set(AssetPlugin {
// //                     meta_check: AssetMetaCheck::Never,
// //                     ..default()
// //                 }),
// //         )
// //         .add_plugins(GamePlugin)
// //         .add_systems(Startup, set_window_icon)
// //         .run();
// // }

// // // Sets the icon on windows and X11
// // fn set_window_icon(
// //     windows: NonSend<WinitWindows>,
// //     primary_window: Query<Entity, With<PrimaryWindow>>,
// // ) {
// //     let primary_entity = primary_window.single();
// //     let Some(primary) = windows.get_window(primary_entity) else {
// //         return;
// //     };
// //     let icon_buf = Cursor::new(include_bytes!(
// //         "../build/macos/AppIcon.iconset/icon_256x256.png"
// //     ));
// //     if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
// //         let image = image.into_rgba8();
// //         let (width, height) = image.dimensions();
// //         let rgba = image.into_raw();
// //         let icon = Icon::from_rgba(rgba, width, height).unwrap();
// //         primary.set_window_icon(Some(icon));
// //     };
// // }

// use bevy::prelude::*;
// use bevy_game::*;

// fn main() {
//     App::new()
//         .insert_resource(ClearColor(Color::srgb(1.0, 0.0, 0.0)))
//         .add_plugins(DefaultPlugins)
//         .add_systems(Startup, setup)
//         .add_systems(Update, change_clear_color)
//         .run();
// }

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
//     info!("Here is some info");
//     warn!("Here is a warning");
//     error!("Here is an error");
// }

// fn change_clear_color(input: Res<ButtonInput<KeyCode>>, mut clear_color: ResMut<ClearColor>, mut state: Local<bool>) {
//     if input.just_pressed(KeyCode::Space) {
//         info!("Changing color");
//         *state = !*state;
//         if *state {
//             clear_color.0 = Color::srgb(0.0, 1.0, 0.0);
//         } else {
//             clear_color.0 = Color::srgb(0.0, 0.0, 1.0);
//         }
//     }
// }

use bevy::prelude::*;
use bevy_game::games::MineSweeperPlugin;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MineSweeperPlugin::default(),
        ))
        .run();
}