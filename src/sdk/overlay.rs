use active_win_pos_rs::get_active_window;
use color_eyre::eyre::{self, Result};
use egui_overlay::{egui_render_three_d, egui_window_glfw_passthrough, EguiOverlay};
use windows::{core::w, Win32::{Foundation::RECT, UI::WindowsAndMessaging::{FindWindowW, GetWindowRect}}};
pub struct SdkOverlay {
    pub frame: u64,
    pub size: [f32; 2],
    pub pos: [f32; 2],
}


impl SdkOverlay {
    fn update_to_window_size(&mut self) -> Result<()> {

        // Check if cs2 is focused
        let active_window = get_active_window();
        match active_window {
            std::result::Result::Ok(window) => {
                if window.title != "Counter-Strike 2" && window.title != "glfw window" {
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

        self.size = [rect.right as f32 - rect.left as f32, rect.bottom as f32 - rect.top as f32];
        self.pos = [rect.left as f32, rect.top as f32];

        Ok(())
    }
}

impl EguiOverlay for SdkOverlay {
    fn gui_run(
        &mut self,
        egui_context: &egui::Context,
        _default_gfx_backend: &mut egui_render_three_d::ThreeDBackend,
        glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
    ) {
        // just some controls to show how you can use glfw_backend
        egui::Window::new("controls").show(egui_context, |ui| {
            // Default GUI size is 400x400, but you can change it.
            ui.set_width(400.0);
            ui.set_height(200.0);

            // sometimes, you want to see the borders to understand where the overlay is.
            let mut borders = glfw_backend.window.is_decorated();
            if ui.checkbox(&mut borders, "window borders").changed() {
                glfw_backend.window.set_decorated(borders);
            }

            ui.label(format!(
                "passthrough: {}",
                glfw_backend.window.is_mouse_passthrough()
            ));

            self.update_to_window_size().unwrap();
            glfw_backend.set_window_size(self.size);
            glfw_backend.window.set_pos(self.pos[0] as i32, self.pos[1] as i32);
        });

        // here you decide if you want to be passthrough or not.
        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            // we need input, so we need the window to be NOT passthrough
            glfw_backend.set_passthrough(false);
        } else {
            // we don't care about input, so the window can be passthrough now
            glfw_backend.set_passthrough(true)
        }
        
        egui_context.request_repaint();
    }
}
