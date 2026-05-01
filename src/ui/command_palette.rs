use gpui::*;
use gpui::prelude::*;
use crate::theme::Theme;

pub struct CommandPalette {
    pub query: String,
    pub results: Vec<String>,
    pub selected_index: usize,
}

impl CommandPalette {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            results: vec![
                "File: New File".to_string(),
                "File: Open File...".to_string(),
                "File: Save".to_string(),
                "View: Toggle Sidebar".to_string(),
                "View: Toggle Bottom Panel".to_string(),
                "Editor: Format Document".to_string(),
                "Project: New Window".to_string(),
                "Settings: Open Settings".to_string(),
                "Project: Run Tests".to_string(),
            ],
            selected_index: 0,
        }
    }
}

impl EventEmitter<CommandPaletteEvent> for CommandPalette {}
pub enum CommandPaletteEvent {
    Executed,
    Dismissed,
}

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        
        // Modal Container (Floating)
        div()
            .absolute()
            .top(px(64.0))
            .left_1_2()
            .ml(px(-340.0))
            .w(px(680.0))
            .bg(theme.panel_bg)
            .rounded_xl()
            .border_1()
            .border_color(theme.border)
            .shadow_xl()
            .flex()
            .flex_col()
            .overflow_hidden()
            .child(
                // Search Input Area
                div()
                    .px_4()
                    .py_3()
                    .bg(theme.toolbar_bg)
                    .border_b_1()
                    .border_color(theme.border)
                    .flex()
                    .items_center()
                    .gap(px(12.0))
                    .child(div().text_xl().text_color(theme.accent).child("󰭎"))
                    .child(
                        div()
                            .text_lg()
                            .text_color(theme.text)
                            .child("Search commands...")
                    )
            )
            .child(
                // Results List
                div()
                    .flex()
                    .flex_col()
                    .p_2()
                    .children(self.results.iter().enumerate().map(|(i, res)| {
                        let is_selected = i == self.selected_index;
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .px_4()
                            .py_2()
                            .rounded_lg()
                            .when(is_selected, |s| {
                                s.bg(theme.accent).text_color(hsla(0.0, 0.0, 1.0, 1.0))
                            })
                            .child(div().child(res.clone()))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(4.0))
                                    .children(if is_selected {
                                        vec![div().child("ENTER")]
                                    } else {
                                        vec![]
                                    })
                                    .text_xs()
                                    .text_color(if is_selected { hsla(0.0, 0.0, 1.0, 0.8) } else { theme.text_muted })
                            )
                    }))
            )
    }
}
