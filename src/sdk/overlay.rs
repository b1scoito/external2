use active_win_pos_rs::get_active_window;
use color_eyre::eyre::{self, Result};
use egui::{Pos2, Rect, Vec2};
use egui_overlay::{egui_render_three_d, egui_window_glfw_passthrough::{self}, EguiOverlay};
use windows::{core::w, Win32::{Foundation::RECT, UI::WindowsAndMessaging::{FindWindowW, GetWindowRect}}};

#[derive(Clone)]
pub struct SdkOverlay {
    pub frame: u64,
    pub size: [f32; 2],
    pub pos: [f32; 2],
    pub painter: Option<egui::Painter>,
}

pub enum DrawType {
    Box,
}

// Implement organized cross-game rendering functions here.
impl SdkOverlay {
    pub fn new() -> Self {
        Self {
            frame: 0,
            size: [1280.0, 800.0],
            pos: [0.0, 0.0],
            painter: None,
        }
    }

    fn update_to_window_size(&mut self) -> Result<()> {
        // Check if cs2 is focused
        let active_window = get_active_window();
        match active_window {
            std::result::Result::Ok(window) => {
                if window.title != "Counter-Strike 2" && window.title != "external2" {
                    self.size = [1.0, 1.0];
                    self.pos = [1.0, 1.0];
                    return Ok(());
                }
            },
            Err(_e) => {},
        }


        // Get cs2 window HWND with FindWindowW
        let hwnd = unsafe { FindWindowW(w!("SDL_app"), w!("Counter-Strike 2")) };
        if hwnd.0 == 0 {
            return Err(eyre::eyre!("cs2 window not found"));
        }

        // Get window rect
        let mut rect: RECT = Default::default();
        unsafe { GetWindowRect(hwnd, &mut rect)? };

        rect.bottom -= 1;

        self.size = [rect.right as f32 - rect.left as f32, rect.bottom as f32 - rect.top as f32];
        self.pos = [rect.left as f32, rect.top as f32];

        Ok(())
    }

    fn set_painter(&mut self, egui_context: &egui::Context) {
        let painter = egui_context.layer_painter(egui::LayerId::background());
        self.painter = Some(painter);
    }

    pub fn draw(&self, draw_type: DrawType, pos: egui::Pos2, width: f32, height: f32, fill: egui::Color32, outline: egui::Stroke) {
        if let Some(ref painter) = self.painter {
            match draw_type {
                DrawType::Box => {
                    let rect = Rect::from_center_size(pos, Vec2::new(width, height));
                    painter.rect_filled(rect, 0.0, fill);
                    painter.rect_stroke(rect, 0.0, outline);
                },
            }
        } else {
            log::error!("painter is None");
        }
    }
}

impl EguiOverlay for SdkOverlay {
    fn gui_run(
        &mut self,
        ctx: &egui::Context,
        _default_gfx_backend: &mut egui_render_three_d::ThreeDBackend,
        glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
    ) {
        if self.painter.is_none() {
            self.set_painter(&ctx);
        }

        // just some controls to show how you can use glfw_backend
        egui::Window::new("external2").default_width(500.0).default_height(500.0).show(ctx, |ui| {
            ui.label("cheats");
            ui.separator();
            ui.horizontal(|ui| {
                ui.checkbox(&mut true, "bhop");
                ui.checkbox(&mut true, "esp");
            });
            if ui.button("exit").clicked() {
                glfw_backend.window.set_should_close(true);
            }

            self.update_to_window_size().unwrap();

            glfw_backend.set_window_size(self.size);
            glfw_backend.window.set_pos(self.pos[0] as i32, self.pos[1] as i32);
            
        });

        // here you decide if you want to be passthrough or not.
        if ctx.wants_pointer_input() || ctx.wants_keyboard_input() {
            glfw_backend.set_passthrough(false);
        } else {
            glfw_backend.set_passthrough(true)
        }

        
        ctx.request_repaint();
    }
}
