mod pad_scene;
mod pad_view;
mod preview;

use crate::pad_scene::PadScene;
use anyhow::Result;
use pixels_graphics_lib::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    pub width: usize,
    pub height: usize,
    pub dots: Vec<bool>,
}

fn settings() -> AppPrefs<Settings> {
    AppPrefs::new("app", "emmabritton", "fontpad", || Settings {
        width: 5,
        height: 5,
        dots: vec![false; 25],
    })
    .expect("Unable to create prefs file")
}

fn main() -> Result<()> {
    let window_prefs = WindowPreferences::new("com", "emmabritton", "fontpad", 1)?;
    let options = Options::default();
    let switcher: SceneSwitcher<SceneResult, SceneName> = |_, _, _| {};
    let first_scene = PadScene::new(&options.style);
    run_scenes(
        300,
        250,
        "Font Pad",
        Some(window_prefs),
        switcher,
        first_scene,
        options,
        empty_pre_post(),
    )?;
    Ok(())
}

#[derive(Clone, Debug, PartialEq)]
enum SceneName {}

#[derive(Clone, Debug, PartialEq)]
enum SceneResult {}
