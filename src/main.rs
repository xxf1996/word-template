#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;
use docx::{Docx, DocxFile};
use eframe::egui;
use rfd::FileDialog;

fn main() -> Result<(), eframe::Error> {
  // Log to stdout (if you run with `RUST_LOG=debug`).
  tracing_subscriber::fmt::init();

  let options = eframe::NativeOptions {
    initial_window_size: Some(egui::vec2(640.0, 360.0)),
    ..Default::default()
  };
  eframe::run_native(
    "Word Template Replacer",
    options,
    Box::new(|_cc| Box::new(MyApp::default())),
  )
}

struct TemplateItem {
  name: String,
  value: Option<String>
}

struct MyApp {
  template_path: Option<PathBuf>,
  template_items: Vec<TemplateItem>
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      template_path: None,
      template_items: vec![],
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("created by xxf");
      ui.horizontal(|ui| {
        let name_label = ui.label("template file: ");
        if ui.button("choose").labelled_by(name_label.id).clicked() {
          // ui.label("click");
          let file = FileDialog::new()
            .add_filter("word", &["docx", "doc"])
            .set_directory("/")
            .pick_file();
          self.template_path = file.map(|file_path| file_path);
        }
        if let Some(file_path) = &self.template_path {
          ui.label(file_path.to_str().unwrap());
        }
      });
      if self.template_path.is_none() {
        ui.label("is empty");
      } else {
        let mut doc = DocxFile::from_file(&self.template_path.unwrap()).unwrap().parse().unwrap();
        println!("{:#?}", doc.document.body.content);
      }
    });
  }
}
