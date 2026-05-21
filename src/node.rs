use uuid::Uuid;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct ID(pub Uuid);
