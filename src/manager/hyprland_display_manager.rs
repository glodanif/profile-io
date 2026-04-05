use super::display_manager::DisplayManager;
use super::mode::Mode;
use super::monitor::Monitor;
use super::size::Size;
use super::transformation::Transformation;
use crate::error::DataModuleError;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;

pub struct HyprlandManager {}

const HYPRLAND_CMD: &str = "hyprctl";

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
    transform: u32,
    disabled: bool,
    mirror_of: String,
    available_modes: Vec<String>,
}

impl From<HyprlandMonitor> for Monitor {
    fn from(hypr: HyprlandMonitor) -> Self {
        Monitor {
            id: hypr.id,
            name: hypr.name,
            model: hypr.model,
            description: hypr.description,
            scale: hypr.scale,
            transformation: Transformation::from_code(hypr.transform as u8),
            resolution: Size {
                width: hypr.width,
                height: hypr.height,
            },
            refresh_rate: hypr.refresh_rate,
            is_enabled: !hypr.disabled,
            mirror_of_name: if hypr.mirror_of == "none" {
                None
            } else {
                Some(hypr.mirror_of)
            },
            current_position: Size {
                width: hypr.x as u32,
                height: hypr.y as u32,
            },
            modes: parse_modes(&hypr.available_modes),
        }
    }
}

fn parse_modes(mode_strings: &[String]) -> Vec<Mode> {
    // HashMap to group refresh rates by resolution
    let mut modes_map: HashMap<(u32, u32), Vec<f32>> = HashMap::new();

    for mode_str in mode_strings {
        // Parse strings like "1920x1080@60.00Hz"
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
            // Remove "Hz" suffix and parse refresh rate
            let refresh_rate_str = parts[1].trim_end_matches("Hz");
            if let Ok(refresh_rate) = refresh_rate_str.parse::<f32>() {
                modes_map
                    .entry((width, height))
                    .or_insert_with(Vec::new)
                    .push(refresh_rate);
            }
        }
    }

    // Convert HashMap to Vec<Mode>
    let mut modes: Vec<Mode> = modes_map
        .into_iter()
        .map(|((width, height), mut refresh_rates)| {
            // Sort refresh rates in descending order
            refresh_rates.sort_by(|a, b| b.partial_cmp(a).unwrap());
            Mode {
                resolution: Size { width, height },
                refresh_rate: refresh_rates,
            }
        })
        .collect();

    // Sort modes by resolution (width first, then height) in descending order
    modes.sort_by(|a, b| {
        b.resolution
            .width
            .cmp(&a.resolution.width)
            .then_with(|| b.resolution.height.cmp(&a.resolution.height))
    });

    modes
}

impl DisplayManager for HyprlandManager {
    fn get_monitors(&self) -> Result<String, DataModuleError> {
        let output = Command::new(HYPRLAND_CMD)
            .args(&["monitors", "all", "-j"])
            .output()
            .map_err(|_| DataModuleError::FailedToGetMonitors)?;

        if !output.status.success() {
            return Err(DataModuleError::FailedToGetMonitors);
        }

        let json_str =
            String::from_utf8(output.stdout).map_err(|_| DataModuleError::FailedToGetMonitors)?;

        let hyprland_monitors: Vec<HyprlandMonitor> =
            serde_json::from_str(&json_str).map_err(|_| DataModuleError::FailedToGetMonitors)?;

        let monitors: Vec<Monitor> = hyprland_monitors.into_iter().map(Monitor::from).collect();

        let result_json = serde_json::to_string_pretty(&monitors)
            .map_err(|_| DataModuleError::FailedToGetMonitors)?;

        Ok(result_json)
    }
}
