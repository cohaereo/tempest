/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    hasher: usize,
    input: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            hasher: 0,
            input: "".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            macro_rules! hashers {
                ($($name:ident),*) => {
                    const HASHES: &[(&str, fn(&[u8]) -> String)] = &[
                        $((stringify!($name), |b| format!("0x{:X}", crate::process::hash::fnv::$name(b))),)*
                    ];
                };
            }

            #[rustfmt::skip]
            hashers!(
                fnv0_32, fnv1_32, fnv1a_32,
                fnv0_64, fnv1_64, fnv1a_64,
                fnv0_128, fnv1_128, fnv1a_128,
                fnv0_256, fnv1_256, fnv1a_256,
                fnv0_512, fnv1_512, fnv1a_512,
                fnv0_1024, fnv1_1024, fnv1a_1024
            );

            egui::ComboBox::from_label("Hasher")
                .selected_text(HASHES[self.hasher].0)
                .show_ui(ui, |ui| {
                    for (i, (name, _)) in HASHES.iter().enumerate() {
                        if ui.selectable_label(self.hasher == i, *name).clicked() {
                            self.hasher = i;
                        }
                    }
                });

            ui.strong("Input:");
            ui.text_edit_singleline(&mut self.input);
            ui.strong(format!(
                "Output: {}",
                HASHES[self.hasher].1(self.input.as_bytes())
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                egui::warn_if_debug_build(ui);
            });
        });
    }
}
