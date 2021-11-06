use eframe::{egui, epi};

pub struct TxtJobApp {

}
impl Default for TxtJobApp {
    fn default() -> Self {
        Self {}
    }
}

impl epi::App for TxtJobApp {
    fn name(&self) -> &str {        
        "TxtJobApp"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {

    }
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eframe TxtJobApp");
            {
                use ::eframe::egui::text::LayoutJob;
                let mut job = LayoutJob::default();
                let first_row_indentation = 10.0;

                let (default_color, strong_color) = if ui.visuals().dark_mode {
                    (egui::Color32::LIGHT_GRAY, egui::Color32::WHITE)
                } else {
                    (egui::Color32::DARK_GRAY, egui::Color32::BLACK)
                };
                
                job.append(
                    "This is a demonstration of TxtJobApp .",
                    first_row_indentation,
                    egui::TextFormat {
                        style: egui::TextStyle::Body,
                        color: default_color,
                        ..Default::default()
                    },
                );

                /*
                *  error message 
                *  ^^^ `LayoutJob` cannot be formatted with the default formatter
                *      required by a bound introduced by this call
                */
                ui.label(job);
            }
            egui::warn_if_debug_build(ui);
        });
    }
}