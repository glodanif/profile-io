use crate::audio::audio_error::AudioError;

pub trait AudioManager {
    fn get_audio_sinks(&self) -> Result<Vec<String>, AudioError>;
    fn set_audio_sink(&self, sink: String) -> Result<(), AudioError>;
}
