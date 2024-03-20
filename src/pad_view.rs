use crate::Settings;
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::{
    fill, AppPrefs, Coord, Rect, Shape, Timing, BLACK, LIGHT_GRAY, WHITE,
};
use pixels_graphics_lib::ui::{ElementState, UiElement};
use pixels_graphics_lib::MouseData;
use std::ptr::swap_nonoverlapping;

pub struct PadView {
    bounds: Rect,
    pub dots: Vec<bool>,
    pub size: (usize, usize),
    last_cell_changed: usize,
}

impl PadView {
    pub fn new(pos: Coord, settings: &AppPrefs<Settings>) -> PadView {
        PadView {
            bounds: Rect::new_with_size(pos, 240, 240),
            size: (settings.data.width, settings.data.height),
            dots: settings.data.dots.clone(),
            last_cell_changed: usize::MAX,
        }
    }
}

impl PadView {
    pub fn change_width(&mut self, value: isize) {
        if value < 0 && self.size.0 > 4 {
            self.size.0 -= 1;
        }
        if value > 0 && self.size.0 < 24 {
            self.size.0 += 1;
        }
        self.dots = vec![false; self.size.0 * self.size.1];
    }

    pub fn change_height(&mut self, value: isize) {
        if value < 0 && self.size.1 > 4 {
            self.size.1 -= 1;
        }
        if value > 0 && self.size.1 < 24 {
            self.size.1 += 1;
        }
        self.dots = vec![false; self.size.0 * self.size.1];
    }

    pub fn on_mouse_update(&mut self, down_at: Coord) {
        if let Some(cell) = self.cell_for(down_at) {
            if cell < self.dots.len() && self.last_cell_changed != cell {
                self.dots[cell] = !self.dots[cell];
                self.last_cell_changed = cell;
            }
        }
    }

    pub fn copy_str(&self) -> String {
        let mut output = String::new();
        let mut i = 1;
        for value in &self.dots {
            output.push_str(if *value { "true" } else { "false" });
            output.push(',');
            if i >= self.size.0 {
                output.push('\n');
                i = 0;
            }
            i += 1;
        }
        output.trim().to_string()
    }

    pub fn paste_str(&mut self, value: &str) {
        let parts: Vec<&str> = value.split(',').collect();
        if parts.len() != self.dots.len() {
            eprintln!(
                "Invalid length, expected {} found {}",
                self.dots.len(),
                parts.len()
            );
            return;
        }
        if !parts.iter().all(|s| matches!(s.trim(), "true" | "false")) {
            eprintln!("Invalid string pasted must be ([true|false],){{16,576}}");
            return;
        }
        parts
            .iter()
            .enumerate()
            .for_each(|(i, &value)| self.dots[i] = value.trim() == "true")
    }

    pub fn clear(&mut self) {
        self.dots.fill(false);
    }

    pub fn fill(&mut self) {
        self.dots.fill(true);
    }

    pub fn flip_h(&mut self) {
        self.dots = Self::horz_swapper(self.size.0, self.size.1, &self.dots);
    }

    pub fn flip_v(&mut self) {
        self.dots = Self::vert_swapper(self.size.0, self.size.1, &self.dots);
    }

    fn horz_swapper(width: usize, height: usize, dots: &[bool]) -> Vec<bool> {
        let mut output = dots.to_vec();
        let half_width = (width as f32 / 2.).floor() as usize;
        for y in 0..height {
            for x in 0..half_width {
                let target_right_i = (width - 1 - x) + y * width;
                let target_left_i = x + y * width;
                unsafe {
                    swap_nonoverlapping(&mut output[target_left_i], &mut output[target_right_i], 1);
                }
            }
        }
        output
    }

    fn vert_swapper(width: usize, height: usize, dots: &[bool]) -> Vec<bool> {
        let mut output = dots.to_vec();
        let half_height = (height as f32 / 2.).floor() as usize;
        for y in 0..half_height {
            unsafe {
                swap_nonoverlapping(
                    &mut output[y * width],
                    &mut output[(height - 1 - y) * width],
                    width,
                );
            }
        }
        output
    }

    fn cell_for(&self, pos: Coord) -> Option<usize> {
        let area = self.drawing_area();
        let cell_size = self.square_size();
        if area.contains(pos) {
            let moved = pos - area.top_left();
            let x = moved.x as usize / cell_size;
            let y = moved.y as usize / cell_size;
            return Some(x + y * self.size.0);
        }
        None
    }

    fn square_size(&self) -> usize {
        let size = self.size.0.max(self.size.1);
        let area = ((self.bounds.width().min(self.bounds.height()) as f32) * 0.98).round() as usize;
        (area / size).min(20)
    }

    fn drawing_area(&self) -> Rect {
        let square_size = self.square_size();
        let drawing_area =
            Rect::new_with_size((0, 0), square_size * self.size.0, square_size * self.size.1);
        drawing_area.move_center_to(self.bounds.center())
    }

    pub fn move_up(&mut self) {
        let mut removed = vec![];
        for _ in 0..self.size.0 {
            removed.push(self.dots.remove(0));
        }
        self.dots.extend_from_slice(&removed);
    }

    pub fn move_down(&mut self) {
        let mut removed = vec![];
        for _ in 0..self.size.0 {
            removed.push(self.dots.remove(self.dots.len() - 1));
        }
        for value in removed.into_iter() {
            self.dots.insert(0, value);
        }
    }

    pub fn move_left(&mut self) {
        for i in 0..self.size.1 {
            let take = i * self.size.0;
            let insert = take + 3;
            let value = self.dots.remove(take);
            self.dots.insert(insert, value);
        }
    }

    pub fn move_right(&mut self) {
        for i in 0..self.size.1 {
            let insert = i * self.size.0;
            let take = insert + 3;
            let value = self.dots.remove(take);
            self.dots.insert(insert, value);
        }
    }
}

impl UiElement for PadView {
    fn set_position(&mut self, top_left: Coord) {
        self.bounds = self.bounds.move_to(top_left);
    }

    fn bounds(&self) -> &Rect {
        &self.bounds
    }

    fn render(&self, graphics: &mut Graphics, _: &MouseData) {
        graphics.clip_mut().set_valid_rect(self.bounds.clone());

        graphics.clear_aware(BLACK);

        let size = self.square_size();
        let area = self.drawing_area();

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let i = x + y * self.size.0;
                if self.dots[i] {
                    let cell =
                        Rect::new_with_size(area.top_left() + (x * size, y * size), size, size);
                    graphics.draw_rect(cell, fill(WHITE));
                }
            }
        }

        for x in 0..=self.size.0 {
            graphics.draw_line(
                area.top_left() + (x * size, 0),
                area.bottom_left() + (x * size, 0),
                LIGHT_GRAY,
            );
        }
        for y in 0..=self.size.1 {
            graphics.draw_line(
                area.top_left() + (0, y * size),
                area.top_right() + (0, y * size),
                LIGHT_GRAY,
            );
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
