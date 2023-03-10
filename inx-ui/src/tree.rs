use egui::{CollapsingHeader, Ui};

pub type CustomRender = Box<dyn Fn(&mut Ui)>;

pub enum Entry {
    Text(String),
    CustomRender(CustomRender),
    Tree(Box<Tree>),
}

#[derive(Default)]

pub struct Tree {
    pub entries: Vec<Entry>,
    pub name: String,
}

impl Tree {
    pub fn new(name: String, entries: Vec<Entry>) -> Tree {
        Tree { name, entries }
    }

    pub fn show(&self, ui: &mut Ui) -> () {
        CollapsingHeader::new(&self.name).show(ui, |ui| {
            for item in self.entries.iter() {
                match item {
                    Entry::Text(ref entry) => {
                        ui.label(entry);
                    }

                    Entry::CustomRender(e) => {
                        e(ui);
                    }

                    Entry::Tree(ref tree) => tree.show(ui),
                };
            }
        });
    }
}
