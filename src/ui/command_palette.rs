use gpui::*;
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
                "File: New".to_string(),
                "File: Open".to_string(),
                "File: Save".to_string(),
                "View: Toggle Sidebar".to_string(),
                "View: Toggle Bottom Panel".to_string(),
                "Editor: Format Document".to_string(),
                "Help: Documentation".to_string(),
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
        div()
            .absolute()
            .top(px(48.0))
            .left_1_2()
            .ml(px(-320.0))
            .w(px(640.0))
            .bg(theme.panel_bg)
            .rounded_xl()
            .border_1()
            .border_color(theme.border)
            .shadow_xl()
            .p_2()
            .child(
                div()
                    .px_4()
                    .py_3()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .flex()
                            .gap(px(8.0))
                            .child(div().text_color(theme.accent).child(">"))
                            .child(div().child("Search commands..."))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .mt_1()
                    .children(self.results.iter().enumerate().map(|(i, res)| {
                        let is_selected = i == self.selected_index;
                        div()
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .when(is_selected, |s| {
                                s.bg(theme.accent).text_color(theme.background)
                            })
                            .child(res.clone())
                    }))
            )
    }
}
