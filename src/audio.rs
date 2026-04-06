use crate::audio::audio_manager::AudioManager;
use crate::audio::pipe_wire_audio_manager::PipeWireAudioManager;

mod audio_error;
pub mod audio_manager;
mod pipe_wire_audio_manager;

pub fn get_audio_manager() -> Box<dyn AudioManager> {
    Box::new(PipeWireAudioManager {})
}
