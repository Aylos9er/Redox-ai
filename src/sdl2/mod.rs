use super::{Color, Event, Point, Rect, Renderer, Widget};

use std::path::Path;
use std::sync::Arc;

extern crate sdl2;
extern crate sdl2_ttf;

pub struct SdlRenderer<'a> {
    font: &'a mut sdl2_ttf::Font,
    inner: &'a mut sdl2::render::Renderer<'static>,
}

impl<'a> SdlRenderer<'a> {
    pub fn new(font: &'a mut sdl2_ttf::Font,
               inner: &'a mut sdl2::render::Renderer<'static>)
               -> SdlRenderer<'a> {
        SdlRenderer {
            font: font,
            inner: inner,
        }
    }
}

impl<'a> Renderer for SdlRenderer<'a> {
    fn clear(&mut self, color: Color) {
        self.inner.set_draw_color(sdl2::pixels::Color::RGBA((color.data >> 16) as u8,
                                                            (color.data >> 8) as u8,
                                                            color.data as u8,
                                                            (color.data >> 24) as u8));
        self.inner.clear();
    }

    fn char(&mut self, pos: Point, c: char, color: Color) {
        let surface =
            self.font
                .render(c,
                        sdl2_ttf::blended(sdl2::pixels::Color::RGBA((color.data >> 16) as u8,
                                                                    (color.data >> 8) as u8,
                                                                    color.data as u8,
                                                                    (color.data >> 24) as u8)))
                .unwrap();
        let mut texture = self.inner.create_texture_from_surface(&surface).unwrap();
        let sdl2::render::TextureQuery { width, height, .. } = texture.query();
        if let Some(rect) = sdl2::rect::Rect::new(pos.x as i32, pos.y as i32, width, height)
                                .unwrap() {
            self.inner.copy(&mut texture, None, Some(rect));
        }
    }

    fn rect(&mut self, rect: Rect, color: Color) {
        if let Some(sdl_rect) = sdl2::rect::Rect::new(rect.x as i32,
                                                      rect.y as i32,
                                                      rect.width as u32,
                                                      rect.height as u32)
                                    .unwrap() {
            self.inner.set_draw_color(sdl2::pixels::Color::RGBA((color.data >> 16) as u8,
                                                                (color.data >> 8) as u8,
                                                                color.data as u8,
                                                                (color.data >> 24) as u8));
            self.inner.fill_rect(sdl_rect);
        }
    }
}

impl<'a> Drop for SdlRenderer<'a> {
    fn drop(&mut self) {
        self.inner.present();
    }
}

pub struct Window {
    _ctx: sdl2::Sdl,
    _video_ctx: sdl2::VideoSubsystem,
    _ttf_context: sdl2_ttf::Sdl2TtfContext,
    events: sdl2::EventPump,
    font: sdl2_ttf::Font,
    inner: sdl2::render::Renderer<'static>,
    pub widgets: Vec<Arc<Widget>>,
    pub widget_focus: usize,
    pub bg: Color,
}

impl Window {
    pub fn new(rect: Rect, title: &str) -> Box<Self> {
        let ctx = sdl2::init().unwrap();
        let video_ctx = ctx.video().unwrap();
        let ttf_context = sdl2_ttf::init().unwrap();

        let mut window = video_ctx.window(title, rect.width as u32, rect.height as u32)
                                  .position(rect.x as i32, rect.y as i32)
                                  .opengl()
                                  .resizable()
                                  .build()
                                  .unwrap();
        window.show();

        video_ctx.text_input().start();

        let events = ctx.event_pump().unwrap();

        Box::new(Window {
            _ctx: ctx,
            _video_ctx: video_ctx,
            _ttf_context: ttf_context,
            events: events,
            font: sdl2_ttf::Font::from_file(&Path::new("res/Unifont.ttf"), 16).unwrap(),
            inner: window.renderer().build().unwrap(),
            widgets: Vec::new(),
            widget_focus: 0,
            bg: Color::rgb(237, 233, 227),
        })
    }

    pub fn draw(&mut self) {
        let mut renderer = SdlRenderer::new(&mut self.font, &mut self.inner);
        renderer.clear(self.bg);
        for widget in self.widgets.iter() {
            widget.draw(&mut renderer);
        }
    }

    pub fn exec(&mut self) {
        // Keep track of mouse state
        let mut left_button = false;
        let mut middle_button = false;
        let mut right_button = false;

        self.draw();
        'event: loop {
            let mut events: Vec<Event> = Vec::new();

            match self.events.wait_event() {
                sdl2::event::Event::MouseMotion { mousestate, x, y, .. } => {
                    left_button = mousestate.left();
                    middle_button = mousestate.middle();
                    right_button = mousestate.right();

                    events.push(Event::Mouse {
                        point: Point::new(x, y),
                        left_button: left_button,
                        middle_button: middle_button,
                        right_button: right_button,
                    });
                }
                sdl2::event::Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    match mouse_btn {
                        sdl2::mouse::Mouse::Left => left_button = true,
                        sdl2::mouse::Mouse::Middle => middle_button = true,
                        sdl2::mouse::Mouse::Right => right_button = true,
                        _ => (),
                    }

                    events.push(Event::Mouse {
                        point: Point::new(x, y),
                        left_button: left_button,
                        middle_button: middle_button,
                        right_button: right_button,
                    });
                }
                sdl2::event::Event::MouseButtonUp { mouse_btn, x, y, .. } => {
                    match mouse_btn {
                        sdl2::mouse::Mouse::Left => left_button = false,
                        sdl2::mouse::Mouse::Middle => middle_button = false,
                        sdl2::mouse::Mouse::Right => right_button = false,
                        _ => (),
                    }

                    events.push(Event::Mouse {
                        point: Point::new(x, y),
                        left_button: left_button,
                        middle_button: middle_button,
                        right_button: right_button,
                    });
                }
                sdl2::event::Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        match key {
                            sdl2::keyboard::Keycode::Return => events.push(Event::Enter),
                            sdl2::keyboard::Keycode::Backspace => events.push(Event::Backspace),
                            sdl2::keyboard::Keycode::Delete => events.push(Event::Delete),
                            sdl2::keyboard::Keycode::Home => events.push(Event::Home),
                            sdl2::keyboard::Keycode::End => events.push(Event::End),
                            sdl2::keyboard::Keycode::Up => events.push(Event::UpArrow),
                            sdl2::keyboard::Keycode::Down => events.push(Event::DownArrow),
                            sdl2::keyboard::Keycode::Left => events.push(Event::LeftArrow),
                            sdl2::keyboard::Keycode::Right => events.push(Event::RightArrow),
                            _ => (),
                        }
                    }
                }
                sdl2::event::Event::TextInput { text, .. } => {
                    for c in text.chars() {
                        events.push(Event::Text { c: c });
                    }
                }
                sdl2::event::Event::Quit {..} => break 'event,
                _ => (),
            };

            for event in events.iter() {
                for i in 0..self.widgets.len() {
                    if let Some(widget) = self.widgets.get(i) {
                        if widget.event(*event, self.widget_focus == i) {
                            self.widget_focus = i;
                        }
                    }
                }
            }

            self.draw();
        }
    }
}
