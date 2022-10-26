#[cfg(target_arch = "wasm32")]
use core::any::Any;
#[cfg(feature = "glow")]
use eframe::glow;
//use std::default::{Default, default, self};
use crate::apps::home_app;
use crate::apps::profile_app::ProfileApp;
use crate::wrap_app::home_app::HomeApp;

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct State {
    home: HomeApp,
    profile: ProfileApp,
    selected_anchor: String,
}
impl Default for State {
    fn default() -> Self {
        let state = State {
            home: HomeApp::default(),
            profile: ProfileApp::default(),
            selected_anchor: String::from("home"),
        };
        state
    }
}
pub struct WrapApp {
    state: State,
}

impl WrapApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        #[allow(unused_mut)]
        let mut slf = Self {
            state: State::default(),
        };

        slf
    }
    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        let mut vec = vec![
            ("Home", "home", &mut self.state.home as &mut dyn eframe::App),
            (
                "Profile",
                "profile",
                &mut self.state.profile as &mut dyn eframe::App,
            ),
        ];
        vec.into_iter()
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut found_anchor = false;
        let selected_anchor = self.state.selected_anchor.clone();
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory().everything_is_visible() {
                app.update(ctx, frame);
                found_anchor = true;
            }
        }
        if !found_anchor {
            self.state.selected_anchor = "demo".into();
        }
    }
    fn bar_contents(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::widgets::global_dark_light_mode_switch(ui);

        ui.separator();
        let mut selected_anchor = self.state.selected_anchor.clone();
        for (name, anchor, _app) in self.apps_iter_mut() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor.to_owned();
            }
        }
        self.state.selected_anchor = selected_anchor;
    }
}

impl eframe::App for WrapApp {
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.state);
    }
    fn clear_color(&self, visuals: &egui::Visuals) -> egui::Rgba {
        visuals.window_fill().into()
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        if let Some(anchor) = frame.info().web_info.location.hash.strip_prefix('#') {
            self.state.selected_anchor = anchor.to_owned();
        }

        if self.state.selected_anchor.is_empty() {
            let selected_anchor = self.apps_iter_mut().next().unwrap().0.to_owned();
            self.state.selected_anchor = selected_anchor;
        }

        egui::TopBottomPanel::top("wrap_app_top_bar").show(ctx, |ui| {
            egui::trace!(ui);
            ui.horizontal_wrapped(|ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });
        });
        self.show_selected_app(ctx, frame);
    }
}
