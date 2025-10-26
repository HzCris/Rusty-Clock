#![cfg_attr(
    all(not(debug_assertions), any(target_os = "windows", target_os = "linux")),
    windows_subsystem = "windows"
)]

use {chrono::Local, 
    eframe::egui, 
    eframe::App as EframeApp,
    serde::Serialize,
    serde::Deserialize,
    std::fs,
};

#[derive(Default)]
struct ClockApp {
    default_theme: bool,
}

impl EframeApp for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Timer variable, centering, and themes
        egui::CentralPanel::default().show(ctx, |ui| {
            let time = Local::now().format("%I:%M:%S %p\n%b %e %a, %Y").to_string();

            // this fixes the ugly default text colors on launch
            if !self.default_theme {
                let mut visuals = egui::Visuals::dark();
                visuals.override_text_color = Some(egui::Color32::from_rgb(209, 209, 209));
                ctx.set_visuals(visuals);
                self.default_theme = true; // Mark the style as set
            }

            ui.vertical_centered(|ui| {
                ui.heading(&time);

                //Themes
                //Doc
                ui.label("Themes:");
                if ui
                    .add(egui::Button::new("Green and Purple").corner_radius(12.0))
                    .clicked()
                {
                    let mut visuals = egui::Visuals::dark();
                    visuals.override_text_color = Some(egui::Color32::from_rgb(51, 255, 0));
                    visuals.panel_fill = egui::Color32::from_rgb(75, 0, 130);
                    ctx.set_visuals(visuals);
                }
                //Light
                if ui
                    .add(egui::Button::new("Light Theme").corner_radius(12.0))
                    .clicked()
                {
                    let visuals = egui::Visuals::light();
                    ctx.set_visuals(visuals);
                }

                //Dark
                if ui
                    .add(egui::Button::new("Dark Theme").corner_radius(12.0))
                    .clicked()
                {
                    let mut visuals = egui::Visuals::dark();
                    visuals.override_text_color = Some(egui::Color32::from_rgb(209, 209, 209));
                    ctx.set_visuals(visuals);
                }

                //Dark Red
                if ui
                    .add(egui::Button::new("Dark Red").corner_radius(12.0))
                    .clicked()
                {
                    let mut visuals = egui::Visuals::dark();
                    visuals.override_text_color = Some(egui::Color32::from_rgb(255, 0, 0));
                    visuals.panel_fill = egui::Color32::from_rgb(27, 27, 27);
                    ctx.set_visuals(visuals);
                }

                //Terminal
                if ui
                    .add(egui::Button::new("Terminal").corner_radius(12.0))
                    .clicked()
                {
                    let mut visuals = egui::Visuals::dark();
                    visuals.override_text_color = Some(egui::Color32::from_rgb(51, 255, 0));
                    visuals.panel_fill = egui::Color32::from_rgb(0, 0, 0);
                    ctx.set_visuals(visuals);
                }
            });
        });

        // Change fonts and font sizes:
        let mut fonts = egui::FontDefinitions::default();
        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap();
        //.insert(0, "Hack".to_owned()); Maybe add a custom font one day

        ctx.set_fonts(fonts);

        let mut style = (*ctx.style()).clone();
        style
            .text_styles
            .insert(egui::TextStyle::Heading, egui::FontId::monospace(36.0));
        ctx.set_style(style);

        ctx.request_repaint_after(std::time::Duration::from_millis(200));
    }
}

fn main() -> eframe::Result<()> {
    // Window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([250.0, 300.0])
            .with_title("Rusty Clock"),
        ..Default::default()
    };

    eframe::run_native(
        "Rusty Clock",
        options,
        Box::new(|_cc| {
            let app = ClockApp::default();
            Ok(Box::new(app))
        }),
    )
}
