use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct Inventory {
    pub slots: [Option<InventoryItem>; 8],
    pub selected: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InventoryItem {
    Key,
    Screw,
    Controller,
    CoverPicked,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: [None; 8],
            selected: 0,
        }
    }
}

impl Inventory {
    pub fn has(&self, item: InventoryItem) -> bool {
        self.slots.iter().any(|s| *s == Some(item))
    }

    pub fn selected_item(&self) -> Option<InventoryItem> {
        self.slots[self.selected]
    }

    pub fn add(&mut self, item: InventoryItem) -> bool {
        if self.has(item) {
            return true;
        }
        for slot in self.slots.iter_mut() {
            if slot.is_none() {
                *slot = Some(item);
                return true;
            }
        }
        false
    }

    pub fn clear(&mut self) {
        self.slots = [None; 8];
        self.selected = 0;
    }
}
