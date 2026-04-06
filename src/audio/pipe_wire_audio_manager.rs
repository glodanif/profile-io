use crate::audio::audio_error::AudioError;
use crate::audio::audio_manager::AudioManager;

const PIPE_WIRE_CMD: &str = "wpctl";

pub struct PipeWireAudioManager;

impl AudioManager for PipeWireAudioManager {
    fn get_audio_sinks(&self) -> Result<Vec<String>, AudioError> {
        todo!()
    }

    fn set_audio_sink(&self, sink: String) -> Result<(), AudioError> {
        todo!()
    }
}
