#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct G6502App {
    label: String,
}

impl Default for G6502App {
    fn default() -> Self {
        Self {
            label: "Hello World!".to_owned(),
        }
    }
}

impl G6502App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for G6502App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(label.as_str());
            ui.add(egui::TextEdit::singleline(label));
        });
    }
}
