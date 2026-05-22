use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ID(pub Uuid);
