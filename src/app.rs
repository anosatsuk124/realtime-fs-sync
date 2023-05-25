use eframe::egui;
use rust_i18n::t;

const APP_NAME: &str = "Realtime FS Sync";

pub fn app() -> anyhow::Result<()> {
    let options = eframe::NativeOptions::default();
    anyhow::bail!(
        "Couldn't run gui: {:?}",
        eframe::run_native(&APP_NAME, options, Box::new(|cc| Box::new(App::new(cc))))
    );
}

#[derive(Debug, Default)]
pub struct App {}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = egui::FontDefinitions::default();

        const NOTO_SANS: &[u8] = include_bytes!("../fonts/NotoSansJP-Regular.ttf");
        const DEFAULT_FONT_NAME: &str = "Noto Sans JP";

        let noto_sans = egui::FontData::from_static(NOTO_SANS);
        fonts
            .font_data
            .insert(DEFAULT_FONT_NAME.to_owned(), noto_sans);

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, DEFAULT_FONT_NAME.to_owned());

        cc.egui_ctx.set_fonts(fonts);

        if let Some(_storage) = cc.storage {
            // TODO: load preference from storage
        }

        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button(t!("start-sync")).clicked() {
                todo!()
            }
        });
    }
}
