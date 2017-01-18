pub struct Screen(());

impl Drop for Screen {
    fn drop(&mut self) {
        // restore screen and cursor position, show cursor
        print!(concat!("\x1B[?1049l", "\x1B[?25h"));
    }
}

impl Screen {
    pub fn new() -> Self {
        // save screen and cursor position, hide cursor
        print!(concat!("\x1B[?1049h", "\x1B[?25l"));
        Screen(())
    }

    pub fn clear(&self) {
        // clear screen, move cursor to home position
        print!(concat!("\x1B[2J", "\x1B[H"));
    }
}
