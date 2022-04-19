use std::collections::HashMap;
extern crate rand;
use rand::Rng;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    array: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            map: HashMap::new(),
            array: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let None = self.map.get(&val) {
            self.array.push(val);
            self.map.insert(val, self.array.len() - 1);
            return true;
        }

        false
    }

    fn remove(&mut self, val: i32) -> bool {
        println!("remove {val}");
        println!("{:?}", self.map.get(&val));
        if let Some(&idx) = self.map.get(&val) {
            let last = self.array.len() - 1;
            if last != idx {
                self.array.swap(idx, last);
                let another = &self.array[idx];
                *self.map.get_mut(another).unwrap() = idx;
            }
            self.array.pop();
            self.map.remove(&val);
            return true;
        }
        false
    }

    fn get_random(&self) -> i32 {
        let idx: usize = rand::thread_rng().gen_range(0..self.array.len());
        *self.array.get(idx).unwrap()
    }
}

fn main() {
    let mut set = RandomizedSet::new();
    dbg!(set.insert(1));
    dbg!(set.remove(2));
    dbg!(set.insert(2));
    dbg!(set.get_random());
    dbg!(set.remove(1));
    dbg!(set.insert(2));
    dbg!(set.get_random());
}
