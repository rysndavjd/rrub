use alloc::vec::Vec;
use uefi::fs::Path;
use uefi::proto::console::text::{Input, Key, ScanCode};
use uefi::{boot, println, ResultExt};
use embedded_graphics::{
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    prelude::*,
    text::Text,
};
use crate::display::Display;
use crate::error::RrubError;
use embedded_graphics::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use alloc::format;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::Drawable;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::primitives::PrimitiveStyle;
use crate::alloc::string::ToString;
use crate::usb::Usb2HostController;

pub struct Terminal<'a, DT: DrawTarget<Color = Rgb888>> {
    pub draw_target: &'a mut DT,
    pub line_buffer: Vec<char>,
    pub style: MonoTextStyle<'static, Rgb888>,
    pub cursor: Point,
    pub char_size: Size,
    pub screen_size: Size,
    pub line_spacing: u32,
    pub root: Option<&'a Path>
}

#[cfg(feature = "uefi")]
impl core::fmt::Debug for Terminal<'_, Display> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Terminal")
            .field("line_buffer", &self.line_buffer)
            .field("cursor", &self.cursor)
            .field("char_size", &self.char_size)
            .field("screen_size", &self.screen_size)
            .field("line_spacing", &self.line_spacing)            
            .field("root", &self.root)            
            .finish()
    }
}

impl<'a, DT: DrawTarget<Color = Rgb888>> Terminal<'a, DT> { 
    pub fn clear_screen<D: DrawTarget<Color = Rgb888>>(
            &mut self, 
        ) -> Result<(), RrubError> 
    {
        self.line_buffer.clear();
        self.cursor = Point::zero();

        let bg_rect = Rectangle::new(Point::zero(), self.screen_size);
        bg_rect
            .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
            .draw(self.draw_target)
            .map_err(|_| RrubError::DrawingError("Unable to clear screen"))?;

        return Ok(());
    }

    pub fn clear_line<D: DrawTarget<Color = Rgb888>>(
            &mut self, 
            y: i32
        ) -> Result<(), RrubError> 
    {
        let rect = Rectangle::new(
            Point::new(0, y),
            Size::new(self.screen_size.width, self.char_size.height + self.line_spacing),
        );
        rect.into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
            .draw(self.draw_target)
            .map_err(|_| RrubError::DrawingError("Unable to clear line"))?;

        return Ok(());
    }

    fn scroll_up<D: DrawTarget<Color = Rgb888>>(
            &mut self,
        ) -> Result<(), RrubError> 
    {
        self.clear_screen::<D>()?;
        self.cursor = Point::zero();

        return Ok(());
    }

    fn newline<D: DrawTarget<Color = Rgb888>>(
            &mut self,
        ) -> Result<(), RrubError> 
    {   
        self.line_buffer.clear();
        self.cursor.x = 0;
        self.cursor.y += (self.char_size.height + self.line_spacing) as i32;

        if self.cursor.y + self.char_size.height as i32 > self.screen_size.height as i32 {
            self.scroll_up::<D>()?;
            self.cursor.y -= (self.char_size.height + self.line_spacing) as i32;
        }

        return Ok(());
    }

    fn advance_cursor<D: DrawTarget<Color = Rgb888>>(
            &mut self,
        ) -> Result<(), RrubError> 
    {
        self.cursor.x += self.char_size.width as i32;
        if self.cursor.x + self.char_size.width as i32 > self.screen_size.width as i32 {
            self.newline::<D>()?;
        }

        return Ok(());
    }

    pub fn backspace<D: DrawTarget<Color = Rgb888>>(
            &mut self,
        ) -> Result<(), RrubError> 
    {
        if self.cursor.x >= self.char_size.width as i32 {
            self.cursor.x -= self.char_size.width as i32;

            let top_left = Point::new(self.cursor.x, 
                self.cursor.y - self.char_size.height as i32);
            
            let char_size = Size::new(self.char_size.width, self.char_size.height + self.line_spacing);

            let erase_rect = Rectangle::new(
                top_left,
                char_size,
            );

            erase_rect
                .into_styled(PrimitiveStyle::with_fill(Rgb888::BLACK))
                .draw(self.draw_target)
                .map_err(|_| RrubError::DrawingError("Unable to draw to backspace charater"))?;
        }

        return Ok(());
    }

    pub fn write_char<D: DrawTarget<Color = Rgb888>>(
            &mut self,
            c: char
        ) -> Result<(), RrubError> 
    {
        match c {
            '\n' => self.newline::<D>()?,
            '\r' => self.cursor.x = 0,
            _ => {
                self.line_buffer.push(c);
                Text::new(&c.to_string(), self.cursor, self.style)
                    .draw(self.draw_target)
                    .map_err(|_| RrubError::DrawingError("Unable to write char"))?;

                self.advance_cursor::<D>()?;
            }
        }
        return Ok(());
    }

    pub fn write_str<D: DrawTarget<Color = Rgb888>>(
            &mut self,
            s: &str
        ) -> Result<(), RrubError> 
    {
        for c in s.chars() {
            self.write_char::<D>(c)?;
        }

        return Ok(());
    }
}

pub fn start_shell(input: &mut Input) -> Result<(), RrubError> {
    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>()?;
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle)?;

    let mode = gop.current_mode_info();
    let mut display = Display::new(gop.frame_buffer(), mode)?;

    println!("display: {:?}", display);

    //let text_style = MonoTextStyle::new(&FONT_10X20, Rgb888::WHITE);

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Rgb888::WHITE)
        .background_color(Rgb888::BLUE)
        .build();

    let stringe = format!("\ndisplay: {:?}", display);

    let mut terminal = Terminal {
        draw_target: &mut display,
        line_buffer: Vec::new(),
        style: text_style,
        cursor: Point::new(0, 20),
        char_size: Size { width: 10, height: 20 },
        screen_size: Size { width: mode.resolution().0 as u32, height: mode.resolution().1 as u32 },
        line_spacing: 5,
        root: None
    };

    terminal.clear_screen::<Display>()?;

    loop {
        let mut events = [input.wait_for_key_event().unwrap()];
        
        boot::wait_for_event(&mut events).discard_errdata()?;

        match input.read_key()? {
            Some(Key::Printable(key)) => { 
                //let tesrt = format!("\nkey: {:?}", key);
                //terminal.write_str::<UefiDisplay>(&tesrt);
                let char = char::from(key);
                match char {
                    '\r' => terminal.newline::<Display>()?,
                    '\u{8}' => {
                        terminal.backspace::<Display>()?;
                    }, 
                    _ => terminal.write_char::<Display>(char)?,
                }    
            }

            /* 
            Some(Key::Special(ScanCode::LEFT)) => {
                terminal.cursor.x -= terminal.char_size.width as i32;
            }

            Some(Key::Special(ScanCode::RIGHT)) => {
                terminal.cursor.x += terminal.char_size.width as i32;
            }
            */
            Some(Key::Special(ScanCode::ESCAPE)) => {
                terminal.clear_screen::<Display>()?;
                terminal.write_str::<Display>("Escaped")?;

                break;
            },
            _ => {}
        }

        println!("{:?}", &terminal);
    }

    Ok(())
}
