use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::{
    AppPrefs, Color, Coord, Rect, Scaling, Shape, Timing, BLACK, MID_GRAY, WHITE,
};
use pixels_graphics_lib::ui::{ElementState, UiElement};
use pixels_graphics_lib::MouseData;

use crate::pad_view::PadView;
use crate::Settings;

const PX_COLOR: Color = WHITE;
const GUIDE_COLOR: Color = MID_GRAY.with_alpha(128);

pub struct Preview {
    bounds: Rect,
    size: (usize, usize),
    dots: Vec<bool>,
    guides: Vec<bool>,
    history: Vec<Vec<bool>>,
}

impl Preview {
    pub fn new(pos: Coord, settings: &AppPrefs<Settings>) -> Preview {
        Preview {
            bounds: Rect::new_with_size(pos, 56, 100),
            size: (settings.data.width, settings.data.height),
            dots: settings.data.dots.clone(),
            guides: settings.data.guides.clone(),
            history: vec![],
        }
    }
}

impl Preview {
    pub fn update(&mut self, pad_view: &PadView) {
        if self.size != pad_view.size {
            println!("history cleared");
            self.history.clear();
        }
        self.guides = pad_view.guides.clone();
        self.dots = pad_view.dots.clone();
        self.size = pad_view.size;
    }

    pub fn add_to_history(&mut self) {
        println!("add_to_history");
        if self.history.last() != Some(&self.dots) {
            self.history.push(self.dots.clone());
            if self.size.0 * (self.history.len().saturating_sub(1)) > self.bounds.width() {
                println!("  outside of bounds, removing first");
                self.history.remove(0);
            }
        } else {
            println!("  current same as history");
        }
    }
}

impl UiElement for Preview {
    fn set_position(&mut self, top_left: Coord) {
        self.bounds = self.bounds.move_to(top_left);
    }

    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, _: &MouseData) {
        graphics.clip_mut().set_valid_rect(self.bounds.clone());

        graphics.clear_aware(BLACK);

        let mut image_buffer = vec![0; self.size.0 * self.size.1 * 4];
        let mut image_graphics = Graphics::new(&mut image_buffer, self.size.0, self.size.1)
            .expect("Creating preview buffer");

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let i = x + y * self.size.0;
                if self.dots[i] {
                    image_graphics.set_pixel(x as isize, y as isize, WHITE);
                }
            }
        }

        let offset = self.bounds.top_left() + ((self.bounds.width() / 2) - (self.size.0 / 2), 2);
        let image = image_graphics.copy_to_image();

        graphics.draw_image(offset, &image);

        let offset = self.bounds.top_left() + ((self.bounds.width() / 2) - self.size.0 + 1, 32);

        let scaled = image.scale(Scaling::nn_double());

        graphics.draw_image(offset, &scaled);

        let history_width = ((self.size.0) * self.history.len()) as isize;
        let start_x = if history_width > self.bounds.width() as isize {
            self.bounds.width() as isize - history_width
        } else {
            self.bounds.center().x - (history_width / 2)
        };
        let y = self.bounds.height() - self.size.1 - 1;
        for (i, dots) in self.history.iter().enumerate() {
            let start = self.bounds.top_left() + (start_x + (self.size.0 * i) as isize, y as isize);
            for x in 0..self.size.0 {
                for y in 0..self.size.1 {
                    let px = x + y * self.size.0;
                    if dots[px] {
                        graphics.set_pixel(start.x + x as isize, start.y + y as isize, PX_COLOR);
                    }
                    if self.guides[px] {
                        graphics.set_pixel(start.x + x as isize, start.y + y as isize, GUIDE_COLOR);
                    }
                }
            }
        }

        graphics.clip_mut().set_all_valid();
    }

    fn update(&mut self, _: &Timing) {}

    fn set_state(&mut self, _: ElementState) {
        unimplemented!()
    }

    fn get_state(&self) -> ElementState {
        ElementState::Normal
    }
}
