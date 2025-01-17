#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Entity {
    id: u64,
}

impl Entity {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity({})", self.id)
    }
}
