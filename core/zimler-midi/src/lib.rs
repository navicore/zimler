#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]

use crossbeam::channel::{bounded, Receiver, Sender};
use midir::MidiInputConnection;

#[derive(Debug, Clone)]
pub enum MidiMessage {
    NoteOn {
        channel: u8,
        note: u8,
        velocity: u8,
    },
    NoteOff {
        channel: u8,
        note: u8,
    },
    ControlChange {
        channel: u8,
        controller: u8,
        value: u8,
    },
    PitchBend {
        channel: u8,
        value: u16,
    },
}

pub struct MidiHandler {
    #[allow(dead_code)]
    tx: Sender<MidiMessage>,
    rx: Receiver<MidiMessage>,
    #[allow(dead_code)]
    connection: Option<MidiInputConnection<()>>,
}

impl Default for MidiHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl MidiHandler {
    pub fn new() -> Self {
        let (tx, rx) = bounded(256);
        Self {
            tx,
            rx,
            connection: None,
        }
    }

    pub fn get_receiver(&self) -> Receiver<MidiMessage> {
        self.rx.clone()
    }

    pub fn parse_midi(data: &[u8]) -> Option<MidiMessage> {
        if data.len() < 2 {
            return None;
        }

        let status = data[0];
        let channel = status & 0x0F;

        match status & 0xF0 {
            0x90 if data.len() >= 3 => {
                let velocity = data[2];
                if velocity > 0 {
                    Some(MidiMessage::NoteOn {
                        channel,
                        note: data[1],
                        velocity,
                    })
                } else {
                    Some(MidiMessage::NoteOff {
                        channel,
                        note: data[1],
                    })
                }
            }
            0x80 if data.len() >= 3 => Some(MidiMessage::NoteOff {
                channel,
                note: data[1],
            }),
            0xB0 if data.len() >= 3 => Some(MidiMessage::ControlChange {
                channel,
                controller: data[1],
                value: data[2],
            }),
            0xE0 if data.len() >= 3 => {
                let value = (u16::from(data[2]) << 7) | u16::from(data[1]);
                Some(MidiMessage::PitchBend { channel, value })
            }
            _ => None,
        }
    }
}
