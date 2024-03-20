use crate::pad_view::PadView;
use crate::preview::Preview;
use crate::{settings, SceneName, SceneResult, Settings};
use copypasta::{ClipboardContext, ClipboardProvider};
use pixels_graphics_lib::prelude::SceneUpdateResult::Pop;
use pixels_graphics_lib::prelude::TextSize::*;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::scenes::SceneUpdateResult::Nothing;
use pixels_graphics_lib::ui::prelude::*;
use pixels_graphics_lib::ui::styles::UiStyle;

const WIDTH_POS: Coord = Coord::new(24, 90);
const HEIGHT_POS: Coord = Coord::new(24, 124);

pub struct PadScene {
    bg_color: Color,
    result: SceneUpdateResult<SceneResult, SceneName>,
    pad_view: PadView,
    font_width_dec: Button,
    font_width_inc: Button,
    font_height_dec: Button,
    font_height_inc: Button,
    clear: Button,
    fill: Button,
    flip_h: Button,
    flip_v: Button,
    preview: Preview,
    infos: Vec<Text>,
    clipboard: ClipboardContext,
    settings: AppPrefs<Settings>,
    next_update: Timer,
}

impl PadScene {
    pub fn new(style: &UiStyle) -> Box<Self> {
        let settings = settings();
        Box::new(PadScene {
            bg_color: style.background,
            result: Nothing,
            clipboard: ClipboardContext::new().expect("Unable to access clipboard"),
            infos: vec![
                Text::new("W:", TextPos::px(coord!(4, 90)), (WHITE, Normal)),
                Text::new("H:", TextPos::px(coord!(4, 124)), (WHITE, Normal)),
            ],
            preview: Preview::new(coord!(5, 164), &settings),
            pad_view: PadView::new(coord!(60, 4), &settings),
            fill: Button::new(coord!(4, 4), "Fill", Some(50), &style.button),
            clear: Button::new(coord!(4, 24), "Clear", Some(50), &style.button),
            flip_h: Button::new(coord!(4, 44), "Flip H", Some(50), &style.button),
            flip_v: Button::new(coord!(4, 64), "Flip V", Some(50), &style.button),
            font_width_inc: Button::new(coord!(30, 100), "+", Some(20), &style.button),
            font_width_dec: Button::new(coord!(4, 100), "-", Some(20), &style.button),
            font_height_inc: Button::new(coord!(30, 134), "+", Some(20), &style.button),
            font_height_dec: Button::new(coord!(4, 134), "-", Some(20), &style.button),
            settings,
            next_update: Timer::new_once(0.2),
        })
    }
}

impl PadScene {
    fn copy(&mut self) {
        let output = self.pad_view.copy_str();
        self.clipboard
            .set_contents(output.clone())
            .unwrap_or_else(|err| panic!("Error copying: {output}: {err:?}"));
        self.settings.data.dots = self.pad_view.dots.clone();
        self.settings.data.width = self.pad_view.size.0;
        self.settings.data.height = self.pad_view.size.1;
        self.settings.save();
    }

    fn paste(&mut self) {
        match self.clipboard.get_contents() {
            Ok(contents) => {
                self.pad_view
                    .paste_str(contents.trim().trim_end_matches(','));
                self.preview.update(&self.pad_view);
            }
            Err(err) => eprintln!("{err:?}"),
        }
    }
}

impl Scene<SceneResult, SceneName> for PadScene {
    fn render(&self, graphics: &mut Graphics, mouse: &MouseData, _: &[KeyCode]) {
        graphics.clear(self.bg_color);
        self.pad_view.render(graphics, mouse);
        self.preview.render(graphics, mouse);
        self.fill.render(graphics, mouse);
        self.flip_h.render(graphics, mouse);
        self.flip_v.render(graphics, mouse);
        self.clear.render(graphics, mouse);
        self.font_height_dec.render(graphics, mouse);
        self.font_height_inc.render(graphics, mouse);
        self.font_width_dec.render(graphics, mouse);
        self.font_width_inc.render(graphics, mouse);
        self.infos.iter().for_each(|t| t.render(graphics));
        graphics.draw_text(
            &format!("{}", self.pad_view.size.0),
            TextPos::px(WIDTH_POS),
            (WHITE, Normal),
        );
        graphics.draw_text(
            &format!("{}", self.pad_view.size.1),
            TextPos::px(HEIGHT_POS),
            (WHITE, Normal),
        );
    }

    fn on_key_up(&mut self, key: KeyCode, _: &MouseData, held: &[KeyCode]) {
        let modifier_pressed = held.contains(&KeyCode::ControlLeft)
            || held.contains(&KeyCode::ControlRight)
            || held.contains(&KeyCode::SuperLeft)
            || held.contains(&KeyCode::SuperRight);
        match key {
            KeyCode::Escape => self.result = Pop(None),
            KeyCode::KeyC => {
                if modifier_pressed {
                    self.copy()
                }
            }
            KeyCode::KeyV => {
                if modifier_pressed {
                    self.paste()
                }
            }
            KeyCode::ArrowUp => {
                self.pad_view.move_up();
                self.preview.update(&self.pad_view);
            }
            KeyCode::ArrowDown => {
                self.pad_view.move_down();
                self.preview.update(&self.pad_view);
            }
            KeyCode::ArrowLeft => {
                self.pad_view.move_left();
                self.preview.update(&self.pad_view);
            }
            KeyCode::ArrowRight => {
                self.pad_view.move_right();
                self.preview.update(&self.pad_view);
            }
            _ => {}
        }
    }

    fn on_mouse_click(
        &mut self,
        down_at: Coord,
        mouse: &MouseData,
        mouse_button: MouseButton,
        _: &[KeyCode],
    ) {
        if mouse_button == MouseButton::Left {
            if self.font_height_inc.on_mouse_click(down_at, mouse.xy) {
                self.pad_view.change_height(1);
            }
            if self.font_height_dec.on_mouse_click(down_at, mouse.xy) {
                self.pad_view.change_height(-1);
            }
            if self.font_width_inc.on_mouse_click(down_at, mouse.xy) {
                self.pad_view.change_width(1);
            }
            if self.font_width_dec.on_mouse_click(down_at, mouse.xy) {
                self.pad_view.change_width(-1);
            }
            if self.clear.on_mouse_click(down_at, mouse.xy) {
                self.pad_view.clear();
            }
            if self.fill.on_mouse_click(down_at, mouse.xy) {
                self.pad_view.fill();
            }
            if self.flip_h.on_mouse_click(down_at, mouse.xy) {
                self.pad_view.flip_h();
            }
            if self.flip_v.on_mouse_click(down_at, mouse.xy) {
                self.pad_view.flip_v();
            }
            self.preview.update(&self.pad_view);
        }
    }

    fn update(
        &mut self,
        timing: &Timing,
        mouse: &MouseData,
        _: &[KeyCode],
    ) -> SceneUpdateResult<SceneResult, SceneName> {
        if mouse.is_down(MouseButton::Left).is_some() && self.next_update.update(timing) {
            self.pad_view.on_mouse_update(mouse.xy);
            self.preview.update(&self.pad_view);
            self.next_update.reset();
        }
        self.result.clone()
    }
}
