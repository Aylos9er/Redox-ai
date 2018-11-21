use std::sync::Arc;

use orbclient::{Renderer as OrbRenderer, Window as OrbWindow};
use orbfont::Font;

use backend::Renderer;
use structs::{Point, Rect};
use theme::{Selector, Theme, ROBOTO_REGULAR_FONT};

struct OrbFontRenderer {
    font: Option<Font>,
}

impl OrbFontRenderer {
    fn render(
        &self,
        theme: &Theme,
        text: &str,
        bounds: &Rect,
        parent_bounds: &Rect,
        selector: &Selector,
        offset: &Point,
        global_position: &Point,
        renderer: &mut OrbWindow,
    ) {
        let borders = theme.border_dimensions(selector);
        let bwidth = borders.0 + borders.2;
        let bheight = borders.1 + borders.3;

        let mut bounds = *bounds;
        bounds.x += borders.0 as i32;
        bounds.y += borders.1 as i32;
        bounds.width += bwidth;
        bounds.height += bheight;

        let mut parent_bounds = *parent_bounds;
        parent_bounds.x += borders.0 as i32;
        parent_bounds.y += borders.1 as i32;
        parent_bounds.width += bwidth;
        parent_bounds.height += bheight;

        if let Some(font) = &self.font {
            let line = font.render(text, theme.uint("font-size", selector) as f32);
            // line.draw_clipped(
            //     renderer,
            //     bounds.x + offset.x + global_position.x,
            //     bounds.y + offset.y + global_position.y,
            //     theme.color("color", selector),
            //     (global_position.x + parent_bounds.x, global_position.y + parent_bounds.y, parent_bounds.width, parent_bounds.height)
            // );
            line.draw(
                renderer,
                bounds.x + offset.x + global_position.x,
                bounds.y + offset.y + global_position.y,
                theme.color("color", selector),
            );
        } else {
            let rect = Rect::new(bounds.x, bounds.y, bounds.width, bounds.height);
            let mut current_rect = Rect::new(
                bounds.x + offset.x,
                bounds.y + offset.y,
                bounds.width,
                bounds.height,
            );
            let x = rect.x;

            for c in text.chars() {
                if c == '\n' {
                    current_rect.x = x;
                    current_rect.y += 16;
                } else {
                    if current_rect.x + 8 >= parent_bounds.x
                        && current_rect.y + 16 >= parent_bounds.y
                        && current_rect.x + 8 < parent_bounds.x + parent_bounds.width as i32
                        && current_rect.y < parent_bounds.y + parent_bounds.height as i32
                    {
                        renderer.char(
                            current_rect.x + global_position.x,
                            current_rect.y + global_position.y,
                            c,
                            theme.color("color", selector),
                        );
                    }
                    current_rect.x += 8;
                }
            }
        }
    }
}

lazy_static! {
    static ref FONT_RENDERER: Arc<OrbFontRenderer> = {
        let font = {
            if let Ok(font) = Font::from_data(ROBOTO_REGULAR_FONT.to_vec().into_boxed_slice()) {
                Some(font)
            } else {
                None
            }
        };

        Arc::new(OrbFontRenderer { font })
    };
}

// lazy_static! {
//     static ref DEFAULT_FONT: Result<Font>> = {
//         if let Ok(font) = Font::from_data(ROBOTO_REGULAR_FONT.to_vec().into_boxed_slice()) {
//             Arc::new(Some(font))
//         } else {
//             Arc::new(None)
//         }
//     };
// }

impl Renderer for OrbWindow {
    fn render(&mut self, theme: &Theme) {
        // render window background
        let col = theme.color("background", &"window".into());
        let blub = col.data;
        let mut _color = format!("#{:x}", blub);
        _color.remove(0);
        _color.remove(0);
        self.set(theme.color("background", &"window".into()));
    }

    fn render_rectangle(
        &mut self,
        theme: &Theme,
        bounds: &Rect,
        parent_bounds: &Rect,
        selector: &Selector,
        offset: &Point,
        global_position: &Point,
    ) {
        let is_debug = selector.element.as_ref().map_or(false, |e| e == "debugborder");
        let b_r = theme.uint("border-radius", selector);
        let fill = theme.color("background", selector);


        let x = (bounds.x + global_position.x + offset.x).max(parent_bounds.x);
        let y = (bounds.y + global_position.y + offset.y).max(parent_bounds.y);
        let width = (bounds.width as i32 + offset.x).min(parent_bounds.width as i32) as u32;
        let height = (bounds.height as i32 + offset.y).min(parent_bounds.height as i32) as u32;

        let (border_left, border_top, border_right, border_bottom) =
            theme.border_dimensions(selector);

        let border_width = border_left + border_right;
        let border_height = border_bottom + border_top;

        if border_width > 0 {
            let border_color = theme.color("border-color", selector);
            self.rounded_rect(x, y, width + border_width, height + border_height, b_r, !is_debug, border_color);
        }

        self.rounded_rect(x + border_left as i32, y + border_top as i32, width, height, b_r, !is_debug, fill);
    }

    fn render_text(
        &mut self,
        theme: &Theme,
        text: &str,
        bounds: &Rect,
        parent_bounds: &Rect,
        selector: &Selector,
        offset: &Point,
        global_position: &Point,
    ) {
        FONT_RENDERER.render(theme, text, bounds, parent_bounds, selector, offset, global_position, self);
    }
}
