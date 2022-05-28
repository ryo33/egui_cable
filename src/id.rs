use std::any::Any;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

// Low-cost to clone, so avoid use a reference of it for output.
#[derive(Clone)]
pub struct Id {
    id: Arc<dyn Any + Send + Sync + 'static>,
    hash: fn(&dyn Any, &mut &mut dyn Hasher) -> u64,
    eq: fn(&dyn Any, &dyn Any) -> bool,
    debug: fn(&dyn Any) -> &dyn Debug,
}

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug = self.debug;
        f.debug_struct("PortId")
            .field("id", debug(self.id.as_ref()))
            .finish_non_exhaustive()
    }
}

impl Hash for Id {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = &self.hash;
        hash(self.id.as_ref(), &mut (state as &mut dyn Hasher));
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        let eq = &self.eq;
        eq(self.id.as_ref(), other.id.as_ref())
    }
}

impl Eq for Id {}

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

impl Id {
    pub fn new<T: Hash + Eq + Debug + Send + Sync + 'static>(id: T) -> Self {
        Id {
            id: Arc::new(id),
            hash: hash::<T>,
            eq: eq::<T>,
            debug: debug::<T>,
        }
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.id.downcast_ref()
    }
}

impl<T: 'static> AsRef<T> for Id {
    fn as_ref(&self) -> &T {
        self.downcast_ref()
            .expect("Cannot downcast port ID. Check your type.")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn as_ref() {
        let id = Id::new(42_usize);
        assert_eq!(id.as_ref() as &usize, &42);
    }

    #[test]
    fn partial_eq() {
        let id1 = Id::new(42_usize);
        let id2 = Id::new(42_usize);
        let id3 = Id::new(43_usize);
        let id4 = Id::new(42_u8);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
        assert_ne!(id1, id4);
    }

    #[test]
    fn hash() {
        let id1 = Id::new(42_usize);
        let id2 = Id::new(42_usize);
        let id3 = Id::new(43_usize);
        let id4 = Id::new(42_u8);
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
