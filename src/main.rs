use eframe::egui;

use std::{cell::RefCell, format as f};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct GUI {
    macros: RefCell<Vec<RefCell<Macro>>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Macro {
    key: String,
    actions: Vec<ActionEnum>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
enum TimeScale {
    Ms,
    S,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq, Eq)]
enum ActionEnum {
    KeyDown {
        key: String,
    },
    KeyUp {
        key: String,
    },
    Press {
        key: String,
        // duration: std::time::Duration,
        duration: u64,
        time_scale: TimeScale,
    },
    Sleep {
        // duration: std::time::Duration,
        duration: u64,
        time_scale: TimeScale,
    },
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { macros } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                    if ui.button("New").clicked() {
                        println!("New");
                    }
                });
            });
        });

        for (i, item_ref) in macros.borrow_mut().iter_mut().enumerate() {
            let mut _window = egui::Window::new("Macro")
                .resizable(false)
                .collapsible(false)
                .enabled(true)
                .show(ctx, |ui| {
                    // ui.heading(f!("When {} is pressed...", item.key.clone()));
                    ui.horizontal(|ui| {
                        ui.heading("When ");
                        let mut temp_borrow = item_ref.borrow_mut();
                        if ui.text_edit_singleline(&mut temp_borrow.key).changed() {
                            while item_ref.borrow().key.len() > 1 {
                                temp_borrow.key.remove(0);
                            }
                        }
                        ui.heading(" is pressed...");
                        drop(temp_borrow);
                    });
                    ui.separator();
                    let mut temp_borrow = item_ref.borrow_mut();
                    for (i_2, action) in temp_borrow.actions.iter_mut().enumerate() {
                        match action {
                            ActionEnum::KeyDown { key } => {
                                ui.label(f!("Press {key}"));
                            }
                            ActionEnum::KeyUp { key } => {
                                ui.label(f!("Release {key}"));
                            }
                            ActionEnum::Press {
                                key,
                                duration,
                                time_scale,
                            } => {
                                ui.horizontal(|ui| {
                                    ui.label(f!("Hold {} for", key));
                                    ui.add(egui::DragValue::new(duration));
                                    ui.label(f!("{:?}", time_scale));
                                });
                                if ui.button("-").clicked() {
                                    temp_borrow.actions.remove(i_2);
                                }
                            }
                            ActionEnum::Sleep {
                                duration,
                                time_scale,
                            } => {
                                ui.horizontal(|ui| {
                                    ui.label("Sleep for");
                                    ui.add(egui::DragValue::new(duration));
                                    ui.label(f!("{:?}", time_scale));
                                });
                            }
                        }
                    }
                    let action = ui.menu_button("+", self::add_menu).inner;

                    if action.clone().is_some() {
                        if action.clone().unwrap().is_some() {
                            item_ref.borrow_mut().actions.push(action.unwrap().unwrap());
                        }
                    }
                });
        }
    }
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            macros: RefCell::new(vec![RefCell::new(Macro {
                key: "A".to_string(),
                actions: vec![
                    ActionEnum::Press {
                        key: "ZL".to_string(),
                        duration: 10,
                        time_scale: TimeScale::Ms,
                    },
                    ActionEnum::KeyDown {
                        key: "ZL".to_string(),
                    },
                    ActionEnum::Sleep {
                        duration: 50,
                        time_scale: TimeScale::Ms,
                    },
                ],
            })]),
        }
    }
}

impl GUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "LSwitch Macro Editor",
        native_options,
        Box::new(|cc| Box::new(self::GUI::new(cc))),
    )
}

fn add_menu(ui: &mut egui::Ui) -> Option<ActionEnum> {
    if ui.button("Hold").clicked() {
        return Some(ActionEnum::Press {
            key: "A".to_string(),
            duration: 10,
            time_scale: TimeScale::Ms,
        });
    }
    if ui.button("Press").clicked() {
        return Some(ActionEnum::KeyDown {
            key: "A".to_string(),
        });
    }
    if ui.button("Release").clicked() {
        return Some(ActionEnum::KeyUp {
            key: "A".to_string(),
        });
    }
    if ui.button("Sleep").clicked() {
        return Some(ActionEnum::Sleep {
            duration: 10,
            time_scale: TimeScale::Ms,
        });
    }

    None
}
