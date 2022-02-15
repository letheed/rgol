use std::sync::atomic::{AtomicBool, Ordering};

use anyhow::bail;

use super::AnyResult;

static SCREEN_INSTANTIATED: AtomicBool = AtomicBool::new(false);

/// `Screen` handles the alternate terminal screen buffer.
///
/// Upon initialization, it saves the cursor position, switches to the alternate
/// screen buffer, clears it and hides the cursor.
/// Changes are reversed when the handle is dropped.
///
/// There can only be one instance of `Screen` alive at any time.
#[derive(Debug)]
pub struct Screen(());

impl Drop for Screen {
    /// Switches back to the normal screen buffer, restores the cursor position
    /// and shows the cursor.
    fn drop(&mut self) {
        print!(concat!("\x1B[?1049l", "\x1B[?25h"));
        SCREEN_INSTANTIATED.store(false, Ordering::SeqCst);
    }
}

impl Screen {
    /// Initializes the screen and returns the `Screen` handle or an error if
    /// one already exists.
    pub fn init() -> AnyResult<Self> {
        let instantiated = SCREEN_INSTANTIATED.swap(true, Ordering::SeqCst);
        if instantiated {
            bail!("tried to instantiate a `Screen` but one already exists");
        };
        print!(concat!("\x1B[?1049h", "\x1B[?25l"));
        Ok(Self(()))
    }

    /// Clears the screen buffer and moves the cursor to the home position
    /// (1, 1).
    #[allow(clippy::unused_self)]
    pub fn clear(&self) {
        print!(concat!("\x1B[2J", "\x1B[H"));
    }
}
