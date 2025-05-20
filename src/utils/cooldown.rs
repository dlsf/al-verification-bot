use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;
use std::time::{Duration, Instant};

pub struct Cooldown<T: Eq + Hash> {
    pub cooldown_duration: Duration,
    pub map: HashMap<T, Instant>
}

impl<T: Eq + Hash> Cooldown<T> {
    pub fn new(cooldown_duration: Duration) -> Cooldown<T> {
        Cooldown {
            cooldown_duration,
            map: HashMap::new()
        }
    }
    
    pub fn is_on_cooldown(&self, t: T) -> bool {
        if !self.map.contains_key(&t) {
            return false;
        }
        
        let now = Instant::now();
        self.map.get(&t).unwrap().add(self.cooldown_duration) > now
    }
    
    pub fn apply(&mut self, t: T) {
        self.map.insert(t, Instant::now());
    }
}
