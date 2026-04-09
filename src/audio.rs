use crate::audio::audio_manager::AudioManager;
use crate::audio::pipe_wire_audio_manager::PipeWireAudioManager;

mod audio_error;
pub mod audio_manager;
mod pipe_wire_audio_manager;
pub mod audio_config;

pub fn get_audio_manager(dry_run: bool) -> Box<dyn AudioManager> {
    Box::new(PipeWireAudioManager { dry_run })
}
