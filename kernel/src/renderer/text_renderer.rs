use alloc::format;
use core::convert::Infallible;
use core::fmt::Write;

use bootloader_api::info::FrameBuffer;
use conquer_once::spin::OnceCell;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::mono_font::ascii::FONT_7X14;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use embedded_graphics::primitives::{PrimitiveStyle, Rectangle, StyledDrawable};
use embedded_graphics::text::Text;
use embedded_graphics::Drawable;
use spin::Mutex;

use crate::renderer::Display;

const CURSOR_HEIGHT: i32 = FONT_7X14.character_size.height as i32;
const LETTER_WIDTH: i32 = FONT_7X14.character_size.width as i32;
const LINE_SPACING: i32 = 4;

pub static TEXT_RENDERER: OnceCell<Mutex<TextRenderer<'static>>> = OnceCell::uninit();

pub fn init_text_renderer(framebuffer: &'static mut FrameBuffer) {
    let display = Display::new(framebuffer);
    let renderer = TextRenderer::new(display);
    TEXT_RENDERER.get_or_init(move || Mutex::new(renderer));
}

pub struct TextRenderer<'f> {
    buffer: [u8; 128 * 25],
    buffer_index: usize,
    position: Point,
    style: MonoTextStyle<'static, Rgb888>,
    background_color: Rgb888,
    display: Display<'f>,
}

impl<'f> TextRenderer<'f> {
    pub fn new(display: Display<'f>) -> TextRenderer<'f> {
        TextRenderer {
            buffer: [0; 128 * 25],
            buffer_index: 0,
            position: Point::zero(),
            style: MonoTextStyle::new(&FONT_7X14, Rgb888::WHITE),
            background_color: Rgb888::BLACK,
            display,
        }
    }

    fn width(&self) -> usize {
        let info = self.display.framebuffer.info();
        info.width
    }

    fn height(&self) -> usize {
        let info = self.display.framebuffer.info();
        info.height
    }

    fn bytes_per_text_line(&self) -> usize {
        let info = self.display.framebuffer.info();
        info.stride * info.bytes_per_pixel * CURSOR_HEIGHT as usize
    }

    fn shift_framebuffer_down(&mut self) {
        let info = self.display.framebuffer.info();
        let bytes_per_text_line = self.bytes_per_text_line();
        let framebuffer_size = info.stride * info.height * info.bytes_per_pixel;
        let framebuffer = self.display.framebuffer.buffer_mut();
        let framebuffer_start = framebuffer.as_mut_ptr();
        let framebuffer_end = unsafe { framebuffer_start.add(framebuffer_size) };
        let framebuffer_line_start = unsafe { framebuffer_start.add(bytes_per_text_line) };
        let framebuffer_line_end = unsafe { framebuffer_end.sub(bytes_per_text_line) };
        unsafe {
            core::ptr::copy(
                framebuffer_line_start,
                framebuffer_start,
                framebuffer_size - bytes_per_text_line,
            );
            core::ptr::write_bytes(framebuffer_line_end, 0, bytes_per_text_line);
        }
        self.position.y -= 2 * CURSOR_HEIGHT + LINE_SPACING;
    }

    pub fn shift_framebuffer_up(&mut self) {
        let info = self.display.framebuffer.info();
        let bytes_per_text_line = self.bytes_per_text_line();
        let framebuffer_size = info.stride * info.height * info.bytes_per_pixel;
        let framebuffer = self.display.framebuffer.buffer_mut();
        let framebuffer_start = framebuffer.as_mut_ptr();
        let framebuffer_line_start = unsafe { framebuffer_start.add(bytes_per_text_line) };
        unsafe {
            core::ptr::copy(
                framebuffer_start,
                framebuffer_line_start,
                framebuffer_size - bytes_per_text_line,
            );
            core::ptr::write_bytes(framebuffer_start, 0, bytes_per_text_line);
        }
        self.position.y += 2 * CURSOR_HEIGHT + LINE_SPACING;
    }

    fn render_cursor(&mut self) {
        let style = PrimitiveStyle::with_fill(Rgb888::WHITE);
        Rectangle::new(self.position, Size::new(10, 20))
            .draw_styled(&style, &mut self.display)
            .unwrap_or_else(infallible);
    }

    fn clear_cursor(&mut self) {
        let style = PrimitiveStyle::with_fill(self.background_color);
        Rectangle::new(self.position, Size::new(10, 20))
            .draw_styled(&style, &mut self.display)
            .unwrap_or_else(infallible);
    }

    fn draw_char_at(&mut self, c: char, x: i32, y: i32) {
        let mut tmp = [0u8; 4];
        let c_str = c.encode_utf8(&mut tmp);
        let text = Text::new(c_str, Point::new(x, y + CURSOR_HEIGHT), self.style);
        text.draw(&mut self.display).unwrap_or_else(infallible);
    }

    pub fn cursor_left(&mut self) {
        self.clear_cursor();
        let num_letters = self.display.framebuffer.info().width as i32 / LETTER_WIDTH;
        let row = self.position.y / (CURSOR_HEIGHT + LINE_SPACING);
        let col = self.position.x / LETTER_WIDTH;
        // let c = self.buffer_map.get(&(row as usize, col as usize)).unwrap_or(&0);

        // format string of row and col number and last buffer value
        let s = format!(
            "{} {} {} {} {}",
            row,
            col,
            (row * num_letters + col) as usize,
            0,
            self.buffer_index
        );
        let text = Text::new(
            s.as_str(),
            Point::new(self.position.x, self.position.y + CURSOR_HEIGHT),
            self.style,
        );
        text.draw(&mut self.display).unwrap_or_else(infallible);

        self.position.x -= LETTER_WIDTH + 1;
        if self.position.x < 0 {
            self.position.x = self.width() as i32 - LETTER_WIDTH;
            self.position.y -= CURSOR_HEIGHT + LINE_SPACING;
        }
        self.render_cursor();
    }

    pub fn cursor_right(&mut self) {
        self.clear_cursor();
        let c = self.buffer[(self.position.y as usize / CURSOR_HEIGHT as usize) * 128
            + (self.position.x as usize / LETTER_WIDTH as usize)];
        let pos_x = self.position.x;
        let pos_y = self.position.y;

        self.position.x += LETTER_WIDTH + 1;
        if self.position.x as usize >= self.width() {
            self.position.x = 0;
            self.position.y += CURSOR_HEIGHT + LINE_SPACING;
        }
        self.render_cursor();

        if c != 0 {
            self.draw_char_at(c as char, pos_x, pos_y);
        }
    }

    pub fn cursor_up(&mut self) {
        self.clear_cursor();
        let c = self.buffer[(self.position.y as usize / CURSOR_HEIGHT as usize) * 128
            + (self.position.x as usize / LETTER_WIDTH as usize)];
        let pos_x = self.position.x;
        let pos_y = self.position.y;

        self.position.y -= CURSOR_HEIGHT + LINE_SPACING;
        if self.position.y < 0 {
            self.shift_framebuffer_up();
        }
        self.render_cursor();

        if c != 0 {
            self.draw_char_at(c as char, pos_x, pos_y);
        }
    }

    pub fn cursor_down(&mut self) {
        self.clear_cursor();
        let c = self.buffer[(self.position.y as usize / CURSOR_HEIGHT as usize) * 128
            + (self.position.x as usize / LETTER_WIDTH as usize)];
        let pos_x = self.position.x;
        let pos_y = self.position.y;

        self.position.y += CURSOR_HEIGHT + LINE_SPACING;
        if self.position.y as usize >= self.height() {
            self.shift_framebuffer_down();
        }
        self.render_cursor();

        if c != 0 {
            self.draw_char_at(c as char, pos_x, pos_y);
        }
    }

    pub fn clear(&mut self) {
        self.display.framebuffer.buffer_mut().fill(0);
        self.position = Point::zero();
    }

    pub fn add_to_buffer(&mut self, c: char) {
        if self.buffer_index == 128 * 25 {
            for i in 0..128 * 24 {
                self.buffer[i] = self.buffer[i + 128];
            }
            self.buffer[128 * 24] = c as u8;
        } else {
            self.buffer[self.buffer_index] = c as u8;
            self.buffer_index += 1;
        }
    }

    pub fn draw_char(&mut self, c: char) {
        self.clear_cursor();
        self.add_to_buffer(c);

        if c == '\r' {
            self.position.x = 0;
            return;
        }

        if c == '\n' {
            self.position.x = 0;
            self.position.y += CURSOR_HEIGHT + LINE_SPACING;
            return;
        }

        if (self.position.x + LETTER_WIDTH) as usize > self.width() {
            self.position.x = 0;
            self.position.y += CURSOR_HEIGHT + LINE_SPACING;
        }

        if (self.position.y + CURSOR_HEIGHT) as usize > self.height() {
            self.shift_framebuffer_down();
        }
        let mut tmp = [0u8; 4];
        let c_str = c.encode_utf8(&mut tmp);
        let text = Text::new(
            c_str,
            Point::new(self.position.x, self.position.y + CURSOR_HEIGHT),
            self.style,
        );
        text.draw(&mut self.display).unwrap_or_else(infallible);
        self.position.x += LETTER_WIDTH + 1;

        self.render_cursor();
    }

    pub fn remove_char(&mut self) {
        self.clear_cursor();
        if self.buffer.is_empty() {
            return;
        }
        self.buffer[self.buffer.len() - 1] = 0;
        self.position.x -= LETTER_WIDTH + 1;
        if self.position.x < 0 {
            self.position.x = self.width() as i32 - LETTER_WIDTH;
            self.position.y -= CURSOR_HEIGHT + LINE_SPACING;
        }
        self.render_cursor();
    }

    pub fn set_color(&mut self, color: Rgb888) {
        self.style = MonoTextStyle::new(&FONT_7X14, color);
    }
}

fn infallible<T>(v: Infallible) -> T {
    match v {}
}

unsafe impl<'f> Send for TextRenderer<'f> {}
unsafe impl<'f> Sync for TextRenderer<'f> {}

impl Write for TextRenderer<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.draw_char(c);
        }
        self.render_cursor();
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use x86_64::instructions::interrupts::without_interrupts;

    if let Some(renderer) = TEXT_RENDERER.get() {
        without_interrupts(|| {
            renderer.lock().write_fmt(args).expect("Error writing to renderer");
        });
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::renderer::text_renderer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
