use specs::Entity;

#[derive(Debug, Default)]
pub struct Events {
    pub added: Vec<Entity>,
    pub removed: Vec<Entity>,
}

impl Events {
    pub fn take(&mut self) -> Events {
        let added = std::mem::replace(&mut self.added, Vec::new());
        let removed = std::mem::replace(&mut self.removed, Vec::new());
        Events { added, removed }
    }
}
