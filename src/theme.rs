use gpui::*;

pub struct Theme {
    pub background: Hsla,
    pub sidebar_bg: Hsla,
    pub editor_bg: Hsla,
    pub status_bg: Hsla,
    pub panel_bg: Hsla,
    pub toolbar_bg: Hsla,
    pub activity_bar_bg: Hsla,
    pub border: Hsla,
    pub border_variant: Hsla,
    pub border_focused: Hsla,
    pub text: Hsla,
    pub text_muted: Hsla,
    pub text_accent: Hsla,
    pub accent: Hsla,
    pub accent_hover: Hsla,
    pub selection: Hsla,
    pub cursor: Hsla,
    pub line_number: Hsla,
    pub shadow: Hsla,
    pub tab_active_bg: Hsla,
    pub tab_inactive_bg: Hsla,
    pub tab_active_border: Hsla,
    pub terminal_bg: Hsla,
    pub terminal_prompt: Hsla,
    pub git_added: Hsla,
    pub git_modified: Hsla,
    pub git_deleted: Hsla,
    pub scrollbar: Hsla,
    pub scrollbar_hover: Hsla,
}

impl Theme {
    pub fn zed_ultra_dark() -> Self {
        Self {
            background: hsla(220.0 / 360.0, 0.14, 0.08, 1.0),      // #111214
            sidebar_bg: hsla(220.0 / 360.0, 0.14, 0.07, 1.0),      // #0e0f11
            editor_bg: hsla(220.0 / 360.0, 0.14, 0.08, 1.0),       
            status_bg: hsla(220.0 / 360.0, 0.14, 0.05, 1.0),       
            panel_bg: hsla(220.0 / 360.0, 0.14, 0.12, 1.0),        
            toolbar_bg: hsla(220.0 / 360.0, 0.14, 0.10, 1.0),      
            activity_bar_bg: hsla(220.0 / 360.0, 0.14, 0.04, 1.0), 
            border: hsla(220.0 / 360.0, 0.12, 0.18, 1.0),          
            border_variant: hsla(220.0 / 360.0, 0.12, 0.12, 1.0),  
            border_focused: hsla(210.0 / 360.0, 0.80, 0.50, 1.0),  
            text: hsla(220.0 / 360.0, 0.15, 0.95, 1.0),            
            text_muted: hsla(220.0 / 360.0, 0.12, 0.55, 1.0),      
            text_accent: hsla(210.0 / 360.0, 0.90, 0.75, 1.0),     
            accent: hsla(210.0 / 360.0, 0.85, 0.60, 1.0),          
            accent_hover: hsla(210.0 / 360.0, 0.85, 0.70, 1.0),    
            selection: hsla(210.0 / 360.0, 0.60, 0.40, 0.3),       
            cursor: hsla(210.0 / 360.0, 0.90, 0.65, 1.0),          
            line_number: hsla(220.0 / 360.0, 0.10, 0.35, 1.0),     
            shadow: hsla(0.0, 0.0, 0.0, 0.8),                      
            tab_active_bg: hsla(220.0 / 360.0, 0.14, 0.08, 1.0),   
            tab_inactive_bg: hsla(220.0 / 360.0, 0.14, 0.06, 1.0), 
            tab_active_border: hsla(210.0 / 360.0, 0.85, 0.60, 1.0),
            terminal_bg: hsla(220.0 / 360.0, 0.14, 0.04, 1.0),
            terminal_prompt: hsla(140.0 / 360.0, 0.70, 0.60, 1.0),
            git_added: hsla(145.0 / 360.0, 0.65, 0.55, 1.0),       
            git_modified: hsla(45.0 / 360.0, 0.85, 0.60, 1.0),      
            git_deleted: hsla(0.0 / 360.0, 0.75, 0.60, 1.0),       
            scrollbar: hsla(220.0 / 360.0, 0.10, 0.30, 0.4),
            scrollbar_hover: hsla(220.0 / 360.0, 0.10, 0.40, 0.6),
        }
    }
}

impl Global for Theme {}
