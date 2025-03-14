use eframe::{egui, App};
use egui::{Color32, RichText, Stroke, Vec2, IconData, FontId, FontFamily};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::Config;
use crate::organise_files;

/// Background color used in the default dark theme.
const BACKGROUND_COLOR: Color32 = Color32::from_rgb(16, 24, 38); // Dark navy blue
/// Accent color used in the UI.
const ACCENT_COLOR: Color32 = Color32::from_rgb(59, 130, 246);   // Blue
/// Text color for light text on dark backgrounds.
const TEXT_COLOR: Color32 = Color32::from_rgb(226, 232, 240);    // Light gray
/// A darker variant of the accent color.
const DARKER_ACCENT: Color32 = Color32::from_rgb(30, 58, 138);   // Darker blue

enum ProcessState {
    Idle,
    Running { progress: f32, message: String },
    Complete { success: bool, message: String },
}

pub struct FileOrganizerApp {
    directory: String,
    log_file: String,
    others_directory: String,
    copy_files: bool,
    include_hidden: bool,
    directory_browse_dialog_open: bool,
    log_file_browse_dialog_open: bool,
    theme_mode: ThemeMode,
    process_state: ProcessState,
    operation_result: Arc<Mutex<Option<Result<(), String>>>>,
}

enum ThemeMode {
    Dark,
    Darker,
    Light,
}

impl Default for FileOrganizerApp {
    fn default() -> Self {
        Self {
            directory: String::new(),
            log_file: "file_organizer.log".to_owned(),
            others_directory: "Others".to_owned(),
            copy_files: false,
            include_hidden: false,
            directory_browse_dialog_open: false,
            log_file_browse_dialog_open: false,
            theme_mode: ThemeMode::Dark,
            process_state: ProcessState::Idle,
            operation_result: Arc::new(Mutex::new(None)),
        }
    }
}

impl App for FileOrganizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update visuals based on theme_mode.
        match self.theme_mode {
            ThemeMode::Dark => {
                let mut visuals = egui::Visuals::dark();
                visuals.window_fill = BACKGROUND_COLOR;
                visuals.panel_fill = BACKGROUND_COLOR;
                visuals.widgets.noninteractive.bg_fill = BACKGROUND_COLOR;
                visuals.widgets.inactive.bg_fill = BACKGROUND_COLOR.linear_multiply(1.2);
                visuals.widgets.active.bg_fill = ACCENT_COLOR;
                visuals.widgets.hovered.bg_fill = ACCENT_COLOR.linear_multiply(0.8);
                visuals.override_text_color = Some(TEXT_COLOR);
                visuals.selection.stroke = Stroke::new(1.0, ACCENT_COLOR);
                visuals.selection.bg_fill = ACCENT_COLOR.linear_multiply(0.5);
                ctx.set_visuals(visuals);
            }
            ThemeMode::Darker => {
                let mut visuals = egui::Visuals::dark();
                visuals.window_fill = Color32::BLACK;
                visuals.panel_fill = Color32::BLACK;
                visuals.widgets.noninteractive.bg_fill = Color32::BLACK;
                visuals.widgets.inactive.bg_fill = Color32::from_rgb(20, 20, 20);
                visuals.widgets.active.bg_fill = DARKER_ACCENT;
                visuals.widgets.hovered.bg_fill = DARKER_ACCENT.linear_multiply(0.8);
                visuals.override_text_color = Some(TEXT_COLOR);
                visuals.selection.stroke = Stroke::new(1.0, DARKER_ACCENT);
                visuals.selection.bg_fill = DARKER_ACCENT.linear_multiply(0.5);
                ctx.set_visuals(visuals);
            }
            ThemeMode::Light => {
                let mut visuals = egui::Visuals::light();
                visuals.override_text_color = Some(Color32::BLACK);
                ctx.set_visuals(visuals);
            }
        }

        if let Some(result) = self.operation_result.lock().unwrap().take() {
            self.process_state = match result {
                Ok(()) => ProcessState::Complete {
                    success: true,
                    message: "Files organized successfully!".to_owned(),
                },
                Err(err) => ProcessState::Complete {
                    success: false,
                    message: format!("Error: {}", err),
                },
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.set_max_width(480.0);
                ui.add_space(20.0);

                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("FILE ORGANIZER")
                            .color(ACCENT_COLOR)
                            .size(32.0)
                            .strong(),
                    );
                    ui.add_space(10.0);
                    ui.label("Organize your files by file extensions");
                    ui.add_space(20.0);
                });

                ui.horizontal(|ui| {
                    ui.label("Theme:");
                    if ui
                        .selectable_label(matches!(self.theme_mode, ThemeMode::Dark), "Dark")
                        .clicked()
                    {
                        self.theme_mode = ThemeMode::Dark;
                    }
                    if ui
                        .selectable_label(matches!(self.theme_mode, ThemeMode::Darker), "Darker")
                        .clicked()
                    {
                        self.theme_mode = ThemeMode::Darker;
                    }
                    if ui
                        .selectable_label(matches!(self.theme_mode, ThemeMode::Light), "Light")
                        .clicked()
                    {
                        self.theme_mode = ThemeMode::Light;
                    }
                });
                ui.add_space(20.0);

                let text_width = 400.0;

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Directory to organize")
                                .size(16.0)
                                .strong(),
                        );
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.directory)
                                    .hint_text("Select a directory...")
                                    .desired_width(text_width),
                            );
                            // Use Noto Emoji for the folder icon.
                            if ui
                                .button(
                                    RichText::new("📁").font(
                                        FontId::new(14.0, FontFamily::Name("noto_emoji".into()))
                                    ),
                                )
                                .clicked()
                            {
                                self.directory_browse_dialog_open = true;
                            }
                            if self.directory_browse_dialog_open {
                                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                    self.directory = path.display().to_string();
                                    self.directory_browse_dialog_open = false;
                                } else {
                                    self.directory_browse_dialog_open = false;
                                }
                            }
                        });
                    });
                });
                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Log file path").size(16.0).strong());
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.log_file)
                                    .hint_text("Path to log file...")
                                    .desired_width(text_width),
                            );
                            // Use Noto Emoji for the log file icon.
                            if ui
                                .button(
                                    RichText::new("📄").font(
                                        FontId::new(14.0, FontFamily::Name("noto_emoji".into()))
                                    ),
                                )
                                .clicked()
                            {
                                self.log_file_browse_dialog_open = true;
                            }
                            if self.log_file_browse_dialog_open {
                                if let Some(path) = rfd::FileDialog::new()
                                    .add_filter("Log files", &["log", "txt"])
                                    .save_file()
                                {
                                    self.log_file = path.display().to_string();
                                    self.log_file_browse_dialog_open = false;
                                } else {
                                    self.log_file_browse_dialog_open = false;
                                }
                            }
                        });
                    });
                });
                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new("Directory for files without extension")
                                .size(16.0)
                                .strong(),
                        );
                        ui.add(
                            egui::TextEdit::singleline(&mut self.others_directory)
                                .hint_text("Name for 'Others' directory...")
                                .desired_width(text_width),
                        );
                    });
                });
                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Options").size(16.0).strong());
                        ui.checkbox(&mut self.copy_files, "Copy files instead of moving them");
                        ui.checkbox(&mut self.include_hidden, "Include hidden files");
                    });
                });
                ui.add_space(20.0);

                match &self.process_state {
                    ProcessState::Idle => {}
                    ProcessState::Running { progress, message } => {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.label(message);
                                ui.add(
                                    egui::ProgressBar::new(*progress)
                                        .animate(true)
                                        .show_percentage()
                                        .desired_width(text_width),
                                );
                            });
                        });
                    }
                    ProcessState::Complete { success, message } => {
                        ui.group(|ui| {
                            let color = if *success { Color32::GREEN } else { Color32::RED };
                            ui.colored_label(color, message);
                        });
                    }
                }
                ui.add_space(20.0);

                let can_execute = !self.directory.is_empty()
                    && !self.log_file.is_empty()
                    && !self.others_directory.is_empty()
                    && !matches!(self.process_state, ProcessState::Running { .. });
                if ui
                    .add_enabled(
                        can_execute,
                        egui::Button::new(
                            RichText::new("ORGANIZE FILES")
                                .size(18.0)
                                .color(TEXT_COLOR)
                                .strong(),
                        )
                        .min_size(Vec2::new(text_width, 50.0))
                        .fill(if can_execute {
                            ACCENT_COLOR
                        } else {
                            ACCENT_COLOR.linear_multiply(0.5)
                        })
                        .corner_radius(egui::CornerRadius::same(10)),
                    )
                    .clicked()
                {
                    let dir = self.directory.clone();
                    let config = Config {
                        copy: self.copy_files,
                        include_hidden: self.include_hidden,
                        others_directory: self.others_directory.clone(),
                        log_file: self.log_file.clone(),
                    };

                    self.process_state = ProcessState::Running {
                        progress: 0.0,
                        message: "Organizing files...".to_owned(),
                    };

                    let result_clone = Arc::clone(&self.operation_result);
                    thread::spawn(move || {
                        let result = match organise_files(&dir, &config) {
                            Ok(()) => Ok(()),
                            Err(e) => Err(e.to_string()),
                        };
                        let mut guard = result_clone.lock().unwrap();
                        *guard = Some(result);
                    });
                }
                ui.add_space(20.0);
            });
        });
    }
}

/// Runs the File Organizer GUI.
pub fn run_gui() -> Result<(), eframe::Error> {
    let icon_bytes = include_bytes!("../assets/images/logo.png");
    let icon_image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon image")
        .into_rgba8();
    let (width, height) = icon_image.dimensions();
    let icon_data = IconData {
        rgba: icon_image.into_raw(),
        width,
        height,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 720.0])
            .with_min_inner_size([400.0, 600.0])
            .with_resizable(true)
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "File Organizer",
        options,
        Box::new(|cc| {
            // Prepare custom font definitions.
            let mut fonts = egui::FontDefinitions::default();

            // Insert the NotoEmoji font with a String key and an Arc<FontData> value.
            fonts.font_data.insert(
                "noto_emoji".to_owned(),
                Arc::new(egui::FontData::from_static(include_bytes!(
                    "../assets/fonts/NotoEmoji-VariableFont_wght.ttf"
                ))),
            );

            // Create a new custom font family for "noto_emoji" by binding its name
            // to a vector containing our font key.
            fonts.families.insert(
                egui::FontFamily::Name("noto_emoji".into()),
                vec!["noto_emoji".to_owned()],
            );

            // Apply the updated font definitions.
            cc.egui_ctx.set_fonts(fonts);

            Ok(Box::new(FileOrganizerApp::default()))
        }),
    )
}
