use uuid::Uuid;

pub trait UniquelyIdentifiedInGame {
    const ID: Uuid;
}
