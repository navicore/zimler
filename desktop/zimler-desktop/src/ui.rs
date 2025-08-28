use crate::EngineResource;
use bevy::prelude::*;
use zimler_engine::EngineCommand;

pub struct ZimlerUiPlugin;

impl Plugin for ZimlerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            (
                handle_keyboard_input,
                update_waveform_display,
                update_envelope_display,
            ),
        );
    }
}

#[derive(Component)]
struct WaveformDisplay;

#[derive(Component)]
struct EnvelopeDisplay;

#[derive(Component)]
struct VoiceIndicator(#[allow(dead_code)] usize);

fn setup_ui(mut commands: Commands) {
    // Main UI container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
    )).with_children(|parent| {
        // Top bar
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
        )).with_children(|parent| {
            parent.spawn((
                Text::new("ZIMLER"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });

        // Main content area
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(20.0)),
                column_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.1)),
        )).with_children(|parent| {
            // Left panel - Waveform
            parent.spawn((
                Node {
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.05, 0.05, 0.06)),
                BorderColor(Color::srgb(0.3, 0.3, 0.35)),
                WaveformDisplay,
            ));

            // Right panel - Controls
            parent.spawn((
                Node {
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            )).with_children(|parent| {
                // Envelope section
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(200.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.05, 0.05, 0.06)),
                    BorderColor(Color::srgb(0.3, 0.3, 0.35)),
                    EnvelopeDisplay,
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("ENVELOPE"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                    ));
                });

                // Voice indicators
                parent.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(5.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.05, 0.05, 0.06)),
                )).with_children(|parent| {
                    for i in 0..16 {
                        parent.spawn((
                            Node {
                                width: Val::Px(20.0),
                                height: Val::Px(20.0),
                                border: UiRect::all(Val::Px(1.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.22)),
                            BorderColor(Color::srgb(0.4, 0.4, 0.45)),
                            VoiceIndicator(i),
                        ));
                    }
                });

                // Instructions
                parent.spawn((
                    Text::new("Press A-K for notes C-B\nSpace to load sample\nE to change envelope"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                ));
            });
        });
    });
}

fn handle_keyboard_input(keyboard: Res<ButtonInput<KeyCode>>, engine: Res<EngineResource>) {
    // Musical keyboard mapping
    let key_to_note = [
        (KeyCode::KeyA, 60), // C
        (KeyCode::KeyW, 61), // C#
        (KeyCode::KeyS, 62), // D
        (KeyCode::KeyE, 63), // D#
        (KeyCode::KeyD, 64), // E
        (KeyCode::KeyF, 65), // F
        (KeyCode::KeyT, 66), // F#
        (KeyCode::KeyG, 67), // G
        (KeyCode::KeyY, 68), // G#
        (KeyCode::KeyH, 69), // A
        (KeyCode::KeyU, 70), // A#
        (KeyCode::KeyJ, 71), // B
        (KeyCode::KeyK, 72), // C
    ];

    for (key, note) in key_to_note {
        if keyboard.just_pressed(key) {
            let _ = engine.handle.send_command(EngineCommand::TriggerNote {
                note,
                velocity: 0.8,
            });
        }
        if keyboard.just_released(key) {
            let _ = engine
                .handle
                .send_command(EngineCommand::ReleaseNote { note });
        }
    }

    // Load sample with spacebar
    if keyboard.just_pressed(KeyCode::Space) {
        let _ = engine.handle.send_command(EngineCommand::LoadSample {
            slot: 0,
            path: "test.wav".to_string(),
        });
    }
}

fn update_waveform_display(
    _engine: Res<EngineResource>,
    mut _query: Query<&mut BackgroundColor, With<WaveformDisplay>>,
) {
    // TODO: Implement waveform visualization
}

fn update_envelope_display(
    _engine: Res<EngineResource>,
    mut _query: Query<&mut BackgroundColor, With<EnvelopeDisplay>>,
) {
    // TODO: Implement envelope visualization
}
