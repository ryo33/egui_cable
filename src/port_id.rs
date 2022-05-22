use std::any::Any;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

#[derive(Clone)]
pub struct PortId {
    id: Arc<dyn Any + Send + Sync + 'static>,
    hash: fn(&dyn Any, &mut &mut dyn Hasher) -> u64,
    eq: fn(&dyn Any, &dyn Any) -> bool,
    debug: fn(&dyn Any) -> &dyn Debug,
}

impl Debug for PortId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug = self.debug;
        f.debug_struct("PortId")
            .field("id", debug(self.id.as_ref()))
            .finish_non_exhaustive()
    }
}

impl Hash for PortId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = &self.hash;
        hash(self.id.as_ref(), &mut (state as &mut dyn Hasher));
    }
}

impl PartialEq for PortId {
    fn eq(&self, other: &Self) -> bool {
        let eq = &self.eq;
        eq(self.id.as_ref(), other.id.as_ref())
    }
}

impl Eq for PortId {}

fn hash<T: Hash + 'static>(id: &dyn Any, hasher: &mut &mut dyn Hasher) -> u64 {
    id.downcast_ref::<T>().unwrap().hash(hasher);
    hasher.finish()
}

fn eq<T: Eq + 'static>(id: &dyn Any, other: &dyn Any) -> bool {
    if let Some(other) = other.downcast_ref::<T>() {
        id.downcast_ref::<T>().unwrap().eq(other)
    } else {
        false
    }
}

fn debug<T: Debug + 'static>(id: &dyn Any) -> &dyn Debug {
    id.downcast_ref::<T>().unwrap()
}

impl PortId {
    pub fn new<T: Hash + Eq + Debug + Send + Sync + 'static>(id: T) -> Self {
        PortId {
            id: Arc::new(id),
            hash: hash::<T>,
            eq: eq::<T>,
            debug: debug::<T>,
        }
    }
}

impl<T: 'static> AsRef<T> for PortId {
    fn as_ref(&self) -> &T {
        self.id
            .downcast_ref()
            .expect("Cannot downcast port ID. Check your type.")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn as_ref() {
        let id = PortId::new(42 as usize);
        assert_eq!(id.as_ref() as &usize, &42);
    }

    #[test]
    fn partial_eq() {
        let id1 = PortId::new(42 as usize);
        let id2 = PortId::new(42 as usize);
        let id3 = PortId::new(43 as usize);
        let id4 = PortId::new(42 as u8);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id1, id4);
    }

    #[test]
    fn hash() {
        let id1 = PortId::new(42 as usize);
        let id2 = PortId::new(42 as usize);
        let id3 = PortId::new(43 as usize);
        let id4 = PortId::new(42 as u8);
        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);
        assert_eq!(set.len(), 1);
        set.insert(id3);
        assert_eq!(set.len(), 2);
        set.insert(id4);
        assert_eq!(set.len(), 3);
    }
}
