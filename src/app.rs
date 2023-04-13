use egui_extras::{image, RetainedImage};
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use zmq::{Context, Socket};
use log::{Level, info, warn};


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct RoverGUI {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    given_ip: String,
    given_port: u16,

    #[serde(skip)]
    zmq_socket: Arc<Mutex<Socket>>,

    #[serde(skip)]
    current_frame: Arc<Mutex<Option<RetainedImage>>>,
}
impl Default for RoverGUI {
    fn default() -> Self {
        // Initialize zmq without connecting.
        let context = Context::new();
        let socket = context
            .socket(zmq::SUB)
            .expect("Failed to create initial zmq socket."); // why can this fail?
        socket
            .set_subscribe(b"")
            .expect("Failed to set inital socket type to subscriber.");
        socket
            .set_rcvtimeo(0)
            .expect("Failed to disable blocking on initial zmq socket."); // disable waiting on a message

        // Initialize placeholder image.
        let placeholder_image = Arc::new(Mutex::new(Some(
            RetainedImage::from_image_bytes(
                "no_image_yet.png",
                include_bytes!("../assets/no_image_yet.png"),
            )
            .expect("Placeholder image initialization failed."),
        )));


        // Initialize logging

        // on desktop, print to the terminal
        #[cfg(not(target_arch = "wasm32"))]
        env_logger::init();

        // on wasm, print to the console
        #[cfg(target_arch = "wasm32")]
        console_log::init_with_level();



        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            current_frame: placeholder_image,
            zmq_socket: Arc::new(Mutex::new(socket)),
            given_ip: "127.0.0.1".to_owned(),
            given_port: 5570,
        }
    }
}

impl RoverGUI {
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

    fn connect(&mut self) {
        let guarded_socket = self
            .zmq_socket
            .as_ref()
            .lock();

        let socket = match guarded_socket {
            Ok(good_socket) => good_socket,
            Err(error) => {
                warn!("[connect]: Unable to access the ZMQ socket.");
                return;
            }
        };
            
        let result = socket.connect(format!("{}:{}", self.given_ip, self.given_port).as_str());

        match result {
            Ok(_) => {
                info!("[connect]: ZMQ connection successful!");
            }
            Err(error) => {
                warn!("[connect]: ZMQ connection error!");
                warn!("{}", error.message());
            }
        }
    }
}

impl eframe::App for RoverGUI {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            label,
            value,
            zmq_socket,
            current_frame,
            given_ip,
            given_port,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}
