use serde::{Deserialize, Serialize};

use super::save::PlayerSave;

#[derive(Default, Deserialize, Serialize)]
pub struct PlayerSaves {

    #[serde(skip)]
    pub selected: Option<usize>,
    
    pub saves: Vec<PlayerSave>,

}

impl PlayerSaves {

    pub fn select(&mut self, index: usize) {
        if index <= self.saves.len() {
            self.selected = Some(index);
        }
    }

    pub fn select_new(&mut self, name: &String) {
        let index = self.saves.len();
        self.saves.push(PlayerSave::new(name));
        self.select(index);
    }

    pub fn get(&self) -> &PlayerSave {
        &self.saves[self.selected.unwrap()]
    }

    pub fn get_mut(&mut self) -> &mut PlayerSave {
        &mut self.saves[self.selected.unwrap()]
    }

    // pub fn load_or_create_new(&self) {
    //     self
    // }

    pub fn name_list(&self) -> Vec<&String> {
        self.saves.iter().map(|data| &data.name).collect()
    }

}