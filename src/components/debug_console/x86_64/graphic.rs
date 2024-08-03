use x86::io::inb;

use crate::{
    components::debug_console::DebugConsole,
    utils::MutexNoIrq,
};

use super::font::BIT_FONTS;

/// TIPS: This should always be a multiple of 2, or 1, But not 0.
const SCALE: usize = 1;

const SCAN_CODE_TO_ASCII: [u8; 58] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'+', 08, b'\t', b'q',
    b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', 10, 0, b'a', b's', b'd',
    b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0, b'\\', b'z', b'x', b'c', b'v', b'b',
    b'n', b'm', b',', b'.', b'/', 0, b'*', 0, b' ',
];

pub struct GraphicConsole {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    pitch: usize,
    ptr: usize,
    color: u32,
}

impl GraphicConsole {
    /// The width of the font in the console.
    const F_WIDTH: usize = 8 * SCALE;
    /// The height of the font in the console.
    const F_HEIGHT: usize = 16 * SCALE;

    /// Create a new graphic console.
    const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            ptr: 0xfd000000,
            width: 1366,
            height: 768,
            pitch: 1366,
            color: 0xffffffff,
        }
    }

    /// Scroll the screen to the specified position.
    fn scroll_up(&mut self, line: usize) {
        assert_eq!(self.x, 0);
        let ptr = self.ptr as *mut u64;
        let offset = line * Self::F_HEIGHT * self.pitch / 8;
        let blank_offset = (self.y - line) * self.pitch / 8;
        unsafe {
            ptr.copy_from_nonoverlapping(ptr.add(offset), blank_offset);
            core::slice::from_raw_parts_mut(ptr.add(blank_offset), offset).fill(0);
        }
        self.y -= line * Self::F_HEIGHT;
    }

    /// Put a character to the Screen.
    fn put_char(&mut self, c: u8) {
        match c {
            b'\n' => {
                self.y += Self::F_HEIGHT;
                self.x = 0;
            }
            b'\r' => {
                self.x = 0;
            }
            _ => {
                let bit_offset = match c as usize * 0x10 < BIT_FONTS.len() {
                    true => c as usize * 0x10,
                    _ => 0,
                };

                let ptr = self.current_ptr();

                for y in 0..16 {
                    let word = BIT_FONTS[bit_offset + y];

                    for x in 0..8 {
                        let color = match word & (1 << (7 - x)) != 0 {
                            true => self.color,
                            false => 0,
                        };
                        unsafe {
                            ptr.add(self.line_offset(y, x)).write_volatile(color);
                        }
                    }
                }

                self.x += Self::F_WIDTH;
            }
        }

        // If the last space is not enough for a character, then newline.
        if self.x > self.width - Self::F_WIDTH {
            self.x = 0;
            self.y += Self::F_HEIGHT;
        }
        // If the last line is not enough for a character, scroll up 1 line.
        if self.y > self.height - Self::F_HEIGHT {
            self.scroll_up(1);
        }
    }

    #[inline]
    fn clear(&self) {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr as *mut u64, self.height * self.pitch / 4 / 2)
                .fill(0);
        }
    }

    /// Get the current pointer of the current.
    const fn current_ptr(&self) -> *mut u32 {
        (self.ptr + self.y * self.pitch + self.x * 4) as *mut u32
    }

    /// Get the offset of the given line number.
    const fn line_offset(&self, line: usize, x: usize) -> usize {
        line * self.pitch / 4 + x
    }
}

static GRAPHIC_CONSOLE: MutexNoIrq<GraphicConsole> = MutexNoIrq::new(GraphicConsole::new());

impl DebugConsole {
    #[inline]
    pub fn putchar(c: u8) {
        GRAPHIC_CONSOLE.lock().put_char(c);
    }

    #[inline]
    pub fn getchar() -> Option<u8> {
        let c = unsafe { inb(0x60) };
        SCAN_CODE_TO_ASCII.get(c as usize).cloned()
    }


    /// Set the color of the current state.
    #[inline]
    pub(crate) fn set_color(color: u32) {
        GRAPHIC_CONSOLE.lock().color = color;
    }
}

/// Init the graphics console's information, includes frame buffer addresse, width and height.
pub(crate) fn init(addr: usize, width: usize, height: usize, pitch: usize) {
    let mut g_console = GRAPHIC_CONSOLE.lock();
    g_console.ptr = addr;
    g_console.width = width;
    g_console.height = height;
    g_console.pitch = pitch;
    g_console.clear();
    drop(g_console);
}
