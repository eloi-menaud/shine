#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![expect(rustdoc::missing_crate_level_docs)] // it's an example


/******* CLI *******/
use clap::Parser;
use egui_alignments::{center_horizontal, center_vertical};
use crate::{dialog::utils::close_on_unfocus, theme::injecting::Heading, types::WindowSize};

#[derive(Parser,Debug,Default,Clone)]
pub struct InfoArgs {
    #[arg(long, default_value = "Info")]
    title: String,
    
    #[arg(long, default_value = "")]
    sub_title: String,
    
    #[arg(long, default_value = "ok")]
    validation_bttn: String,
    
    #[arg(long, default_value = "Oyez - Info")]
    window_title: String,
    
    #[arg(long, default_value = "Oyez - Info")]
    window_id: String,
    
    #[arg(long, default_value = "320x320")] // default_value = "700x400"
    window_size: WindowSize,
    
    #[arg(long)]
    close_on_unfocus: bool
}




/******* UI *******/
use eframe::egui::{self, Button, Color32, Vec2, Widget};


#[derive(Default)]
struct InfoBox{
    args: InfoArgs
}


pub fn render(args: InfoArgs) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
                    .with_title(&args.window_title)
                    .with_app_id(&args.window_id) 
                    .with_decorations(false)
                    .with_inner_size([args.window_size.width as f32, args.window_size.height as f32])
                    .with_resizable(false)
                    .with_always_on_top(),
                ..Default::default()
    };
    eframe::run_native(
        args.window_title.clone().as_str(),
        options,
        Box::new(|_cc| Ok(Box::new(InfoBox { args }))),
    )
}



impl eframe::App for InfoBox {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        // if self.args.close_on_unfocus{
        //     close_on_unfocus(ui);
        // }
        
        egui::CentralPanel::default().show(ui, |ui| {
            center_vertical(ui, |ui| {
                ui.label("Click a button");
                
                center_horizontal(ui, |ui| {
                    for i in 1..=10 {
                        if Button::new(format!("Button {}", i))
                            .ui(ui)
                            .clicked()
                        {
                            // clicked_button = Some(i);
                        }
                    }
                });
            })

        });

        
        // egui::CentralPanel::default()
        //     .frame(egui::Frame::new()
        //         .fill(Color32::from_rgb(0, 0, 0))
        //         .inner_margin(40.0)
        //     )
        //     .show(ui, |ui| {
                
        //         ui.centered_and_justified(|ui| {
        //             ui.vertical(|ui| {
        //                 ui.themed_heading("Are you sure ?");
                        
        //                 let frame = egui::Frame::canvas(ui.style())
        //                         .fill(egui::Color32::from_rgb(255, 0, 0));
                                
        //                 frame.show(ui, |ui| {
        //                     ui.horizontal(|ui| {
        //                         // Ici, on ne force plus available_size(), on laisse egui calculer
        //                         let button_width = 80.0; // Donne une largeur fixe ou calculée
        //                         ui.add_sized([button_width, 30.0], egui::Button::new("Yes"));
        //                         ui.add_sized([button_width, 30.0], egui::Button::new("No"));
        //                     });
        //                 });
                                
        //                 // frame.show(ui, |ui| {
        //                 //     ui.horizontal(|ui| {
        //                 //         let layout = egui::Layout::left_to_right(egui::Align::Center); //.with_main_justify(true);
                                        
        //                 //         let resp = ui.allocate_ui_with_layout(ui.available_size(), layout, |ui| {
                
        //                 //             let button_width = (ui.available_width() - ui.spacing().item_spacing.x) / 2.0;
                                    
        //                 //             ui.add_sized([button_width, 20.0], egui::Button::new("Yes"));
        //                 //             ui.add_sized([button_width, 20.0], egui::Button::new("No"))
                                    
        //                 //         });
        //                 //     });
        //                 // });
        //             });
        //         });
                
                
        //         let content_size = ui.min_size();
        //         println!("{content_size}");
        // });
    }
}