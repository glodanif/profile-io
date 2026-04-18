use crate::display::display_error::DisplayError;

use super::display_manager::DisplayManager;
use super::mode::Mode;
use super::monitor::Monitor;
use super::size::Size;
use crate::profile::Profile;
use serde::Deserialize;
use std::collections::HashMap;
use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::thread;
use std::time::Duration;

const HYPRLAND_CMD: &str = "hyprctl";

pub struct HyprlandManager {
    pub dry_run: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HyprlandWorkspace {
    id: i32,
    windows: u32,
}

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

impl HyprlandManager {
    fn run(&self, args: &[&str]) -> Result<std::process::Output, std::io::Error> {
        if self.dry_run {
            println!("[DRY RUN] {} {}", HYPRLAND_CMD, args.join(" "));
            Ok(std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        } else {
            Command::new(HYPRLAND_CMD).args(args).output()
        }
    }

    fn get_workspaces(&self) -> Result<Vec<HyprlandWorkspace>, DisplayError> {
        let output = Command::new(HYPRLAND_CMD)
            .args(&["workspaces", "-j"])
            .output()
            .map_err(|_| {
                DisplayError::CommandExecutionError(format!(
                    "Failed to execute command {} workspaces -j",
                    HYPRLAND_CMD
                ))
            })?;

        if !output.status.success() {
            return Err(DisplayError::CommandExecutionError(format!(
                "Failed to execute command {} workspaces -j",
                HYPRLAND_CMD
            )));
        }

        let json_str =
            String::from_utf8(output.stdout).map_err(|_| DisplayError::CommandOutputParseError)?;

        let workspaces: Vec<HyprlandWorkspace> = serde_json::from_str(&json_str)
            .map_err(|_| DisplayError::EncodingError("get_workspaces"))?;

        Ok(workspaces)
    }
}

impl DisplayManager for HyprlandManager {
    fn get_monitors(&self) -> Result<Vec<Monitor>, DisplayError> {
        let output = self.run(&["monitors", "all", "-j"])
            .map_err(|_| {
                DisplayError::CommandExecutionError(format!(
                    "Failed to execute command {} monitors all -j",
                    HYPRLAND_CMD
                ))
            })?;

        if !output.status.success() {
            return Err(DisplayError::CommandExecutionError(format!(
                "Failed to execute command {} monitors all -j",
                HYPRLAND_CMD
            )));
        }

        let json_str =
            String::from_utf8(output.stdout).map_err(|_| DisplayError::CommandOutputParseError)?;

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
        for monitor in &profile.monitors {
            if !monitor.is_enabled {
                let config = format!("{},disable", monitor.name);
                match self.run(&["keyword", "monitor", config.as_str()]) {
                    Ok(output) if output.status.success() => {
                        if !self.dry_run {
                            println!("Successfully disabled monitor: {}", monitor.name);
                        }
                    }
                    Ok(output) => {
                        let msg = String::from_utf8_lossy(&output.stdout);
                        eprintln!("Failed to disable monitor {}: {}", monitor.name, msg.trim());
                        return Err(DisplayError::CommandExecutionError(format!(
                            "Failed to disable monitor {}: {}",
                            monitor.name, msg.trim()
                        )));
                    }
                    Err(e) => {
                        eprintln!("Failed to execute command for monitor {}: {}", monitor.name, e);
                        return Err(DisplayError::CommandExecutionError(format!(
                            "Failed to execute command for monitor {}: {}",
                            monitor.name, e
                        )));
                    }
                }
            }
        }
        if self.dry_run {
            println!("[DRY RUN] Waiting 500ms");
        } else {
            thread::sleep(Duration::from_millis(500));
        }
        for monitor in &profile.monitors {
            if monitor.is_enabled {
                let config = format!(
                    "{},{}x{}@{},{}x{},{}",
                    monitor.name,
                    monitor.resolution.width,
                    monitor.resolution.height,
                    monitor.refresh_rate,
                    monitor.current_position.width,
                    monitor.current_position.height,
                    monitor.scale,
                );
                match self.run(&["keyword", "monitor", config.as_str()]) {
                    Ok(output) if output.status.success() => {
                        if !self.dry_run {
                            println!("Successfully configured monitor: {} ({}x{}@{}Hz)",
                                     monitor.name, monitor.resolution.width,
                                     monitor.resolution.height, monitor.refresh_rate);
                        }
                    }
                    Ok(output) => {
                        let msg = String::from_utf8_lossy(&output.stdout);
                        eprintln!("Failed to configure monitor {}: {}", monitor.name, msg.trim());
                        return Err(DisplayError::CommandExecutionError(format!(
                            "Failed to configure monitor {}: {}",
                            monitor.name, msg.trim()
                        )));
                    }
                    Err(e) => {
                        eprintln!("Failed to execute command for monitor {}: {}", monitor.name, e);
                        return Err(DisplayError::CommandExecutionError(format!(
                            "Failed to execute command for monitor {}: {}",
                            monitor.name, e
                        )));
                    }
                }
            }
        }

        if self.dry_run {
            println!("[DRY RUN] Waiting 500ms");
        } else {
            thread::sleep(Duration::from_millis(500));
        }

        // Build a map of workspace_id -> target_monitor from profile
        let explicit_mappings: std::collections::HashMap<u32, &str> = profile
            .workspaces
            .iter()
            .map(|w| (w.id, w.monitor_name.as_str()))
            .collect();

        // Get workspace IDs to manage: use persistent_workspace_ids if set, otherwise query Hyprland
        let workspace_ids: Vec<u32> = if let Some(ref ids) = profile.persistent_workspace_ids {
            ids.clone()
        } else {
            self.get_workspaces()
                .unwrap_or_default()
                .iter()
                .map(|w| w.id as u32)
                .collect()
        };

        // Move all workspaces to their target monitors
        for workspace_id in &workspace_ids {
            let target_monitor = if let Some(&monitor) = explicit_mappings.get(workspace_id) {
                monitor
            } else if let Some(ref fallback) = profile.workspaces_fallback_monitor_name {
                fallback.as_str()
            } else {
                continue; // No mapping and no fallback, skip
            };

            match self.run(&[
                "dispatch",
                "moveworkspacetomonitor",
                workspace_id.to_string().as_str(),
                target_monitor,
            ]) {
                Ok(output) if output.status.success() => {
                    if !self.dry_run {
                        println!("Successfully moved workspace {} to monitor {}",
                                 workspace_id, target_monitor);
                    }
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to move workspace {} to monitor {}: {}",
                              workspace_id, target_monitor, stderr);
                }
                Err(e) => {
                    eprintln!("Failed to execute moveworkspacetomonitor for workspace {}: {}",
                              workspace_id, e);
                }
            }
        }

        // Activate explicitly defined workspaces to make them visible on their monitors
        for workspace in &profile.workspaces {
            match self.run(&["dispatch", "workspace", workspace.id.to_string().as_str()]) {
                Ok(output) if output.status.success() => {
                    if !self.dry_run {
                        println!("Successfully activated workspace {} on monitor {}",
                                 workspace.id, workspace.monitor_name);
                    }
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to activate workspace {}: {}", workspace.id, stderr);
                }
                Err(e) => {
                    eprintln!("Failed to activate workspace {}: {}", workspace.id, e);
                }
            }
        }

        // Focus the final workspace if specified
        if let Some(focus_ws_id) = profile.focus_workspace_id {
            match self.run(&["dispatch", "workspace", focus_ws_id.to_string().as_str()]) {
                Ok(output) if output.status.success() => {
                    if !self.dry_run {
                        println!("Successfully focused workspace {}", focus_ws_id);
                    }
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to focus workspace {}: {}", focus_ws_id, stderr);
                }
                Err(e) => {
                    eprintln!("Failed to focus workspace {}: {}", focus_ws_id, e);
                }
            }
        }


        Ok(())
    }
}
