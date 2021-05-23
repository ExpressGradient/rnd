extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    ui::{RenderUi, UiBundle},
};
use crate::pong::Pong;


mod pong;
mod systems;

fn main() -> Result<(), amethyst::Error> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_config_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_config_path)?;

    // Central Repository for all Game Logic
    // Bundles are sets of systems preconfigured to work together
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderingBundle::<DefaultBackend>::new()
            // RenderToWindow plugin provides all the scaffolding for opening a window
            .with_plugin(RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0.0, 0.0, 0.0, 0.1]))
            // RenderFlat2D plugin is used to render entities with a SpriteRender component
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderUi::default()))?
        // Transform Bundle handles tracking Entity positions
        .with_bundle(TransformBundle::new())?
        // Input Bundle handles all Input
        .with_bundle(input_bundle)?
        // UI Bundle
        .with_bundle(UiBundle::<StringBindings>::new())?
        // Not adding a Bundle but a System
        // Manages Paddles
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        // Manages Ball
        .with(systems::MoveBallsSystem, "ball_system", &[])
        // Manages Paddle and Ball Collision
        .with(systems::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        // Manages Player Score
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let assets_dir = app_root.join("assets");

    // Application is Amethyst's root object, binds the OS event loop, state machines, timers and other core components in a central place.
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    // Start the Game Loop
    game.run();

    Ok(())
}
