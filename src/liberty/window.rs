use winit::{
    dpi::{Position, Size},
    event_loop::EventLoopWindowTarget,
    window::{Fullscreen, Icon, Window},
};

use crate::liberty::EngineBuilder;

impl EngineBuilder {
    pub fn window<T: 'static>(&self, target: &EventLoopWindowTarget<T>) -> Window {
        self.window.clone().build(target).unwrap()
    }
    pub fn title(mut self, title: &str) -> Self {
        self.window = self.window.with_title(title);
        self
    }

    pub fn inner_size<G: Into<Size>>(mut self, size: G) -> Self {
        self.window = self.window.with_inner_size(size);
        self
    }

    pub fn min_inner_size<G: Into<Size>>(mut self, min_size: G) -> Self {
        self.window = self.window.with_min_inner_size(min_size);
        self
    }

    pub fn max_inner_size<G: Into<Size>>(mut self, max_size: G) -> Self {
        self.window = self.window.with_max_inner_size(max_size);
        self
    }

    pub fn position<G: Into<Position>>(mut self, position: G) -> Self {
        self.window = self.window.with_position(position);
        self
    }

    pub fn unresizable(mut self) -> Self {
        self.window = self.window.with_resizable(false);
        self
    }

    pub fn fullscreen(mut self) -> Self {
        self.window = self
            .window
            .with_fullscreen(Some(Fullscreen::Borderless(None)));
        self
    }

    pub fn maximized(mut self) -> Self {
        self.window = self.window.with_maximized(true);
        self
    }

    pub fn invisible(mut self) -> Self {
        self.window = self.window.with_visible(false);
        self
    }

    // Reminder: Remember that when updating to winit 0.27, there's a function for checking if transparent is supported
    pub fn transparent(mut self) -> Self {
        self.window = self.window.with_transparent(true);
        self
    }

    pub fn no_decorations(mut self) -> Self {
        self.window = self.window.with_decorations(false);
        self
    }

    pub fn always_on_top(mut self) -> Self {
        self.window = self.window.with_always_on_top(true);
        self
    }

    pub fn window_icon(mut self, window_icon: Option<Icon>) -> Self {
        self.window = self.window.with_window_icon(window_icon);
        self
    }
}
