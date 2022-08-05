use winit::{
    dpi::{Position, Size},
    window::{Fullscreen, Icon},
};

use crate::liberty::EngineBuilder;

impl EngineBuilder {
    pub fn title<G: Into<String>>(mut self, title: G) -> Self {
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

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.window = self.window.with_resizable(resizable);
        self
    }

    pub fn fullscreen(mut self, fullscreen: Option<Fullscreen>) -> Self {
        self.window = self.window.with_fullscreen(fullscreen);
        self
    }

    pub fn maximized(mut self, maximized: bool) -> Self {
        self.window = self.window.with_maximized(maximized);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.window = self.window.with_visible(visible);
        self
    }

    pub fn transparent(mut self, transparent: bool) -> Self {
        self.window = self.window.with_transparent(transparent);
        self
    }

    pub fn decorations(mut self, decorations: bool) -> Self {
        self.window = self.window.with_decorations(decorations);
        self
    }

    pub fn always_on_top(mut self, always_on_top: bool) -> Self {
        self.window = self.window.with_always_on_top(always_on_top);
        self
    }

    pub fn window_icon(mut self, window_icon: Option<Icon>) -> Self {
        self.window = self.window.with_window_icon(window_icon);
        self
    }
}
