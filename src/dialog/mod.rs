pub mod info;



pub mod utils {
    use eframe::egui;

    
    pub fn close_on_unfocus(ui: &mut egui::Ui){
        if ui.input(|i| {
            i.events.iter().any(|e| matches!(e, egui::Event::WindowFocused(false)))
        }){
            ui.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}