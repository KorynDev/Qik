mod theme;
mod actions;
mod ui;

use gpui::*;
use gpui_platform::application;
use crate::theme::Theme;
use crate::ui::Workspace;

fn main() {
    application().run(|cx: &mut App| {
        cx.set_global(Theme::zed_ultra_dark());
        
        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|cx| Workspace::new(window, cx))
        }).expect("failed to open window");
        
        cx.activate(true);
    });
}
