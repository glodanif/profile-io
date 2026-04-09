pub mod display_manager;
pub mod display_error;
pub mod hyprland_display_manager;
pub mod mode;
pub mod monitor;
pub mod size;
pub mod transformation;

use display_manager::DisplayManager;
use hyprland_display_manager::HyprlandManager;

pub fn get_display_manager(dry_run: bool) -> Box<dyn DisplayManager> {
    Box::new(HyprlandManager { dry_run })
}
