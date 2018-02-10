use std;
use contracts::*;

pub trait BookKeeper {
    /// Find or create an id associated with the given type and id
    fn resolve_id(&self, etype: EntityType, id: Id, authoritative: bool) -> Option<Id>;

    /// Reset the BookKeeper's internal state
    fn reset(&mut self);
}