use crate::display::display_error::DisplayError;

use super::display_manager::DisplayManager;
use super::mode::Mode;
use super::monitor::Monitor;
use super::size::Size;
use crate::profile::Profile;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

const HYPRLAND_CMD: &str = "hyprctl";

pub struct HyprlandManager;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HyprlandMonitor {
    id: u32,
    name: String,
    model: String,
    description: String,
    width: u32,
    height: u32,
    refresh_rate: f64,
    x: i32,
    y: i32,
    scale: f64,
    transform: u8,
    disabled: bool,
    mirror_of: String,
    available_modes: Vec<String>,
}

impl From<HyprlandMonitor> for Monitor {
    fn from(monitor: HyprlandMonitor) -> Self {
        Monitor {
            id: monitor.id,
            name: monitor.name,
            model: monitor.model,
            description: monitor.description,
            scale: monitor.scale,
            transformation: monitor.transform,
            resolution: Size {
                width: monitor.width,
                height: monitor.height,
            },
            refresh_rate: monitor.refresh_rate,
            is_enabled: !monitor.disabled,
            mirror_of_name: if monitor.mirror_of == "none" {
                None
            } else {
                Some(monitor.mirror_of)
            },
            current_position: Size {
                width: monitor.x as u32,
                height: monitor.y as u32,
            },
            modes: parse_modes(&monitor.available_modes),
        }
    }
}

fn parse_modes(mode_strings: &[String]) -> Vec<Mode> {
    let mut modes_map: HashMap<(u32, u32), Vec<f32>> = HashMap::new();

    for mode_str in mode_strings {
        let parts: Vec<&str> = mode_str.split('@').collect();
        if parts.len() != 2 {
            continue;
        }

        let resolution_parts: Vec<&str> = parts[0].split('x').collect();
        if resolution_parts.len() != 2 {
            continue;
        }

        if let (Ok(width), Ok(height)) = (
            resolution_parts[0].parse::<u32>(),
            resolution_parts[1].parse::<u32>(),
        ) {
            let refresh_rate_str = parts[1].trim_end_matches("Hz");
            if let Ok(refresh_rate) = refresh_rate_str.parse::<f32>() {
                modes_map
                    .entry((width, height))
                    .or_insert_with(Vec::new)
                    .push(refresh_rate);
            }
        }
    }

    let mut modes: Vec<Mode> = modes_map
        .into_iter()
        .map(|((width, height), mut refresh_rates)| {
            refresh_rates.sort_by(|a, b| b.partial_cmp(a).unwrap());
            Mode {
                resolution: Size { width, height },
                refresh_rate: refresh_rates,
            }
        })
        .collect();

    modes.sort_by(|a, b| {
        b.resolution
            .width
            .cmp(&a.resolution.width)
            .then_with(|| b.resolution.height.cmp(&a.resolution.height))
    });

    modes
}

impl DisplayManager for HyprlandManager {
    fn get_monitors(&self) -> Result<Vec<Monitor>, DisplayError> {
        let output = Command::new(HYPRLAND_CMD)
            .args(&["monitors", "all", "-j"])
            .output()
            .map_err(|_| DisplayError::CommandExecutionError)?;

        if !output.status.success() {
            return Err(DisplayError::CommandExecutionError);
        }

        let json_str = String::from_utf8(output.stdout)
            .map_err(|_| DisplayError::CommandOutputParseError)?;

        let hyprland_monitors: Vec<HyprlandMonitor> = serde_json::from_str(&json_str)
            .map_err(|_| DisplayError::EncodingError("get_monitors"))?;

        Ok(hyprland_monitors.into_iter().map(Monitor::from).collect())
    }

    fn get_monitors_json(&self) -> Result<String, DisplayError> {
        let monitors = self.get_monitors()?;
        let result_json = serde_json::to_string_pretty(&monitors)
            .map_err(|_| DisplayError::EncodingError("get_monitors_json"))?;
        Ok(result_json)
    }

    fn set_monitors_profile(&self, profile: &Profile) -> Result<(), DisplayError> {
        profile.monitors.iter().try_for_each(|monitor| {
            let config = if monitor.is_enabled {
                format!(
                    "{},{}x{}@{},{}x{},{}",
                    monitor.name,
                    monitor.resolution.width,
                    monitor.resolution.height,
                    monitor.refresh_rate,
                    monitor.current_position.width,
                    monitor.current_position.height,
                    monitor.scale,
                )
            } else {
                format!("{},disable", monitor.name)
            };
            let output = Command::new(HYPRLAND_CMD)
                .args(&["keyword", "monitor", config.as_str()])
                .output();
            match output {
                Ok(_) => Ok(()),
                Err(_) => Err(DisplayError::CommandExecutionError),
            }
        })?;

        Ok(())
    }
}
