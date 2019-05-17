use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub struct ProcSeed {
    hash: u64,
    pub skew: f64,
}

impl ProcSeed {
    pub fn new<T: Hash>(t: &T, skew: f64) -> ProcSeed {
        ProcSeed {
            hash: hash(t),
            skew: skew,
        }
    }

    pub fn get(&self) -> u64 {
        self.hash
    }

    pub fn get_skew(&self) -> f64 {
        self.skew
    }

    pub fn derive<T: Hash + Debug>(&self, t: &T) -> ProcSeed {
        //println!("Deriving seed {:?} from {:?} by hashing {:?} to {:?}", hash(&(self.get() ^ hash(t))), self.get(), t, hash(t));
        ProcSeed::new(&(self.get() ^ hash(t)), self.skew)
    }
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
