use crate::audio::audio_config::AudioConfig;
use crate::audio::audio_error::AudioError;
use crate::audio::audio_manager::AudioManager;
use std::process::Command;
use std::thread;
use std::time::Duration;

const PIPE_WIRE_CMD: &str = "pactl";

pub struct PipeWireAudioManager;

impl AudioManager for PipeWireAudioManager {
    fn get_audio_sinks(&self) -> Result<Vec<String>, AudioError> {
        todo!()
    }

    fn set_audio_sink(&self, sink: &AudioConfig) -> Result<(), AudioError> {
        let prefix = &sink.sink_name;
        let attempts = 10;

        for _ in 0..attempts {
            let sink_name = self.find_sink_by_prefix(prefix)?;

            if let Some(name) = sink_name {
                match self.set_default_sink(&name) {
                    Ok(()) => return Ok(()),
                    Err(e) => {
                        eprintln!("Failed to set sink '{}': {}", name, e);
                    }
                }
            }

            thread::sleep(Duration::from_millis(500));
        }

        Err(AudioError::FailedToSetAudioSink(format!(
            "Failed to set sink with prefix '{}' after {} attempts",
            prefix, attempts
        )))
    }
}

impl PipeWireAudioManager {
    fn find_sink_by_prefix(&self, prefix: &str) -> Result<Option<String>, AudioError> {
        let output = Command::new(PIPE_WIRE_CMD)
            .args(&["list", "short", "sinks"])
            .output()
            .map_err(|e| AudioError::CommandExecutionError(e.to_string()))?;

        if !output.status.success() {
            return Err(AudioError::CommandExecutionError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let sink_name = parts[1];
                if sink_name.starts_with(prefix) {
                    return Ok(Some(sink_name.to_string()));
                }
            }
        }

        Ok(None)
    }

    fn set_default_sink(&self, sink_name: &str) -> Result<(), AudioError> {
        let output = Command::new(PIPE_WIRE_CMD)
            .args(&["set-default-sink", sink_name])
            .output()
            .map_err(|e| AudioError::CommandExecutionError(e.to_string()))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(AudioError::FailedToSetAudioSink(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }
    }
}
