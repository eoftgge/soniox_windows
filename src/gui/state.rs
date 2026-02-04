use eframe::egui::{Context, ViewportCommand, WindowLevel};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    Config,
    Overlay,
}

impl AppState {
    pub fn apply_window_state(&self, ctx: &Context, enable_high_priority: bool) {
        match self {
            AppState::Config => {
                ctx.send_viewport_cmd(ViewportCommand::Decorations(true));
                ctx.send_viewport_cmd(ViewportCommand::Transparent(false));
                ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(false));
                ctx.send_viewport_cmd(ViewportCommand::Resizable(false));
                ctx.send_viewport_cmd(ViewportCommand::WindowLevel(WindowLevel::Normal));
            }
            AppState::Overlay => {
                ctx.send_viewport_cmd(ViewportCommand::Decorations(false));
                ctx.send_viewport_cmd(ViewportCommand::Transparent(true));
                ctx.send_viewport_cmd(ViewportCommand::MousePassthrough(true));
                ctx.send_viewport_cmd(ViewportCommand::Maximized(true));

                if enable_high_priority {
                    ctx.send_viewport_cmd(ViewportCommand::WindowLevel(WindowLevel::AlwaysOnTop));
                }
            }
        }
    }
}
