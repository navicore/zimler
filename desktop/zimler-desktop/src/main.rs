#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_const_for_fn)]

use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use zimler_engine::{EngineConfig, EngineHandle, ZimlerEngine};

mod audio_backend;
mod ui;

use ui::ZimlerUiPlugin;

#[derive(Resource, Clone)]
struct EngineResource {
    handle: EngineHandle,
}

fn main() {
    // Initialize audio engine
    let config = EngineConfig::default();
    let engine = Arc::new(Mutex::new(ZimlerEngine::new(config.clone())));
    let handle = engine.lock().unwrap().get_api_handle();

    // Start audio backend in a separate thread
    let engine_clone = engine.clone();
    std::thread::spawn(move || {
        let _audio_backend =
            audio_backend::AudioBackend::new(engine_clone, config).expect("Failed to init audio");

        // Keep audio thread alive
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zimler - Serge-Inspired Sampler".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(EngineResource { handle })
        .add_plugins(ZimlerUiPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);
}
