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
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("Failed to disable monitor {}: {}", monitor.name, stderr);
                        return Err(DisplayError::CommandExecutionError(format!(
                            "Failed to disable monitor {}: {}",
                            monitor.name, stderr
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
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("Failed to configure monitor {}: {}", monitor.name, stderr);
                        return Err(DisplayError::CommandExecutionError(format!(
                            "Failed to configure monitor {}: {}",
                            monitor.name, stderr
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

        if !profile.workspaces.is_empty() {
            let assigned_workspace_ids: std::collections::HashSet<i32> = profile
                .workspaces
                .iter()
                .map(|w| w.id as i32)
                .collect();

            profile.workspaces.iter().for_each(|workspace| {
                match self.run(&[
                    "dispatch",
                    "moveworkspacetomonitor",
                    workspace.id.to_string().as_str(),
                    workspace.monitor_name.as_str(),
                ]) {
                    Ok(output) if output.status.success() => {
                        if !self.dry_run {
                            println!("Successfully moved workspace {} to monitor {}",
                                     workspace.id, workspace.monitor_name);
                        }
                    }
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        eprintln!("Failed to move workspace {} to monitor {}: {}",
                                  workspace.id, workspace.monitor_name, stderr);
                    }
                    Err(e) => {
                        eprintln!("Failed to execute moveworkspacetomonitor for workspace {}: {}",
                                  workspace.id, e);
                    }
                }
            });

            let fallback_monitor = profile
                .workspaces_fallback_monitor_name
                .as_deref()
                .or(profile.focus_monitor_name.as_deref())
                .or_else(|| {
                    profile
                        .monitors
                        .iter()
                        .find(|m| m.is_enabled)
                        .map(|m| m.name.as_str())
                });

            if let Some(fallback_monitor) = fallback_monitor {
                if let Ok(current_workspaces) = self.get_workspaces() {
                    for ws in current_workspaces {
                        if ws.windows > 0 && !assigned_workspace_ids.contains(&ws.id) {
                            match self.run(&[
                                "dispatch",
                                "moveworkspacetomonitor",
                                ws.id.to_string().as_str(),
                                fallback_monitor,
                            ]) {
                                Ok(output) if output.status.success() => {
                                    if !self.dry_run {
                                        println!("Successfully moved unassigned workspace {} to fallback monitor {}",
                                                 ws.id, fallback_monitor);
                                    }
                                }
                                Ok(output) => {
                                    let stderr = String::from_utf8_lossy(&output.stderr);
                                    eprintln!("Failed to move workspace {} to fallback monitor {}: {}",
                                              ws.id, fallback_monitor, stderr);
                                }
                                Err(e) => {
                                    eprintln!("Failed to execute moveworkspacetomonitor for workspace {}: {}",
                                              ws.id, e);
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(focus_monitor_name) = profile.focus_monitor_name.as_deref() {
            match self.run(&["dispatch", "focusmonitor", focus_monitor_name]) {
                Ok(output) if output.status.success() => {
                    if !self.dry_run {
                        println!("Successfully focused monitor: {}", focus_monitor_name);
                    }
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to focus monitor {}: {}", focus_monitor_name, stderr);
                }
                Err(e) => {
                    eprintln!("Failed to execute focusmonitor: {}", e);
                }
            }
        }

        if let Some(focus_workspace_id) = profile.focus_workspace_id {
            match self.run(&["dispatch", "workspace", focus_workspace_id.to_string().as_str()]) {
                Ok(output) if output.status.success() => {
                    if !self.dry_run {
                        println!("Successfully focused workspace: {}", focus_workspace_id);
                    }
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("Failed to focus workspace {}: {}", focus_workspace_id, stderr);
                }
                Err(e) => {
                    eprintln!("Failed to execute workspace focus: {}", e);
                }
            }
        }

        Ok(())
    }
}
