use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::buffer_graphics_lib::scaling::Scaling;
use pixels_graphics_lib::MouseData;
use pixels_graphics_lib::prelude::{AppPrefs, BLACK, Coord, Rect, Shape, Timing, WHITE};
use pixels_graphics_lib::ui::{ElementState, UiElement};
use crate::pad_view::PadView;
use crate::Settings;

pub struct Preview {
    bounds: Rect,
    size: (usize, usize),
    dots: Vec<bool>,
}

impl Preview {
    pub fn new(pos: Coord, settings: &AppPrefs<Settings>) -> Preview {
        Preview {
            bounds: Rect::new_with_size(pos, 50, 80),
            size: (settings.data.width, settings.data.height),
            dots: settings.data.dots.clone(),
        }
    }
}

impl Preview {
    pub fn update(&mut self, pad_view: &PadView) {
        self.size = pad_view.size;
        self.dots = pad_view.dots.clone();
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
        let mut image_graphics = Graphics::new(&mut image_buffer, self.size.0, self.size.1).expect("Creating preview buffer");

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