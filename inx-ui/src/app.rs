use eframe::Theme;
use egui::{Align, Color32, DragValue, Grid, Layout, ScrollArea};
use inx::{human::Couple, Simulator, World};
use poll_promise::Promise;

use crate::{
    render,
    tree::{Entry, Tree},
};

pub struct App {
    simulator: Option<Promise<Simulator>>,
    max_generations: u64,
    population: usize,
    is_running: bool,
    interactions: usize,
    tree: Tree,
}

impl App {
    pub fn new() -> Self {
        Self {
            simulator: None,
            is_running: false,
            max_generations: 10,
            population: 10,
            interactions: 14000,
            tree: Tree::new("Pessoas".to_string(), Default::default()),
        }
    }
}

fn render_couple(root: &mut Vec<Entry>, couple: &Couple) {
    let mut child_tree: Vec<Entry> = couple
        .children
        .iter()
        .map(render::human)
        .map(Entry::CustomRender)
        .collect();

    if let Some(ref descedent) = couple.descedent {
        render_couple(&mut child_tree, descedent)
    }

    let new_tree = Tree::new(
        format!(
            "Mae: {}, Pai: {}",
            render::human_text(&couple.mother),
            render::human_text(&couple.father),
        ),
        child_tree,
    );

    let mut found = false;
    for it in root.iter() {
        match it {
            Entry::Tree(t) => {
                if t.name == new_tree.name {
                    found = true;
                }
            }

            _ => {}
        }
    }

    if !found {
        root.push(Entry::Tree(Box::new(new_tree)));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            max_generations,
            population,
            simulator,
            is_running,
            interactions,
            tree,
        } = self;

        let enabled = !*is_running;

        ctx.set_visuals(Theme::Dark.egui_visuals());

        if let Some(simulator) = simulator {
            if simulator.ready().is_some() {
                *is_running = false;
            }
        }

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.add_enabled_ui(enabled, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Simulacao");

                    if ui.button("Ligar").clicked() {
                        *is_running = !*is_running;

                        let it = *interactions;
                        let population = *population;

                        let max_generations = *max_generations;
                        let promise = Promise::spawn_thread("run_simulator", move || {
                            let mut sim = Simulator::new(World::new(population), max_generations);

                            for _ in 0..=it {
                                sim.step();
                            }

                            sim
                        });

                        *simulator = Some(promise);
                    }

                    ui.separator();
                    ui.heading("Informacoes");

                    Grid::new("info_grid")
                        .num_columns(2)
                        .spacing([65.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Ano: ");

                            if let Some(simulator_prom) = simulator {
                                if let Some(simulator) = simulator_prom.ready() {
                                    ui.label(simulator.world.year.to_string());
                                }
                            } else {
                                ui.colored_label(Color32::LIGHT_RED, "Não executando");
                            }
                            ui.end_row();
                        });

                    ui.separator();
                    ui.heading("Configuracao Inicial");

                    Grid::new("conf_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Máximo de geracoes");
                            ui.add(DragValue::new(max_generations));

                            ui.end_row();

                            ui.label("Populacao");
                            ui.add(DragValue::new(population));

                            ui.end_row();

                            ui.label("Interacoes");
                            ui.add(DragValue::new(interactions));
                        })
                });
            });

            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                ui.label("by: yxqsnz");
                ui.label(concat!("version: ", env!("CARGO_PKG_VERSION")));
                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                if *is_running {
                    ui.spinner();
                    return;
                }

                if let Some(sim) = simulator {
                    if let Some(sim) = sim.ready() {
                        let mut entries = Vec::new();

                        for people in &sim.world.peoples {
                            entries.push(Entry::CustomRender(render::human(people)));
                        }

                        for couple in &sim.world.couples {
                            render_couple(&mut entries, &couple);
                        }

                        tree.entries = entries;

                        ScrollArea::new([false, true]).show(ui, |ui| {
                            tree.show(ui);
                        });
                    }
                }
            });
        });
    }
}
