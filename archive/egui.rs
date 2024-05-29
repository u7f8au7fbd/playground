#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use std::{process::Command, vec};

#[macro_use]
mod macros;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf - 8); // utf-8コマンドを実行する
    cmd!(line) // lineコマンドを実行する
}

use eframe::egui;
use egui_extras::*;
use std::ops::*;

fn main() -> Result<(), eframe::Error> {
    setup();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<Data>::default()
        }),
    )
}

struct Data {
    url: Vec<String>,
    score: Vec<i32>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            url: vec!["a".to_string()],
            score: vec![1],
        }
    }
}

impl eframe::App for Data {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
            // Here we add the table
            TableBuilder::new(ui)
                .column(
                    Column::auto()
                        .clip(false)
                        .resizable(true)
                        .range(RangeInclusive::new(50.0, 100.0)),
                )
                .column(
                    Column::auto()
                        .clip(false)
                        .resizable(true)
                        .range(RangeInclusive::new(50.0, 100.0)),
                )
                .column(
                    Column::auto()
                        .clip(false)
                        .resizable(true)
                        .range(RangeInclusive::new(50.0, 100.0)),
                )
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Rank");
                    });
                    header.col(|ui| {
                        ui.heading("URL");
                    });
                    header.col(|ui| {
                        ui.heading("Score");
                    });
                })
                .body(|mut body| {
                    body.row(30.0, |mut row| {
                        row.col(|ui| {
                            for i in 0..99 {
                                ui.label(i.to_string());
                            }
                        });
                        row.col(|ui| {
                            for _ in 0..49 {
                                if ui.button("world!").clicked() {
                                    println!("Hello world!");
                                };
                            }
                        });
                        row.col(|ui| {
                            for _ in 0..49 {
                                if ui.button("world!").clicked() {
                                    println!("Hello world!");
                                };
                            }
                        });
                    });
                });
        });
    }
}
