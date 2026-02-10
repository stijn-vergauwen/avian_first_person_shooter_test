use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow, WindowFocused};

pub struct CursorLockPlugin;

impl Plugin for CursorLockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lock_mouse_cursor_on_window_focused);
    }
}

fn lock_mouse_cursor_on_window_focused(
    mut on_window_focused: MessageReader<WindowFocused>,
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
) {
    for message in on_window_focused.read() {
        cursor_options.visible = !message.focused;
        cursor_options.grab_mode = if message.focused {
            CursorGrabMode::Locked
        } else {
            CursorGrabMode::None
        };
    }
}
