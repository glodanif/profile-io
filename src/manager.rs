pub mod display_manager;
pub mod error;
pub mod hyprland_display_manager;
pub mod mode;
pub mod monitor;
pub mod size;
pub mod transformation;

use display_manager::DisplayManager;
use hyprland_display_manager::HyprlandManager;

pub fn get_display_manager() -> Box<dyn DisplayManager> {
    Box::new(HyprlandManager {})
}
