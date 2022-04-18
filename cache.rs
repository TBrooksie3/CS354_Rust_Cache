
use std::collections::LinkedList;
use std::fmt::Display;

pub struct Cache<T> {
    list:LinkedList<T>,
    capacity: usize
}

impl<T:Eq+Copy+Display> Cache<T> {
    pub fn new(capacity:usize) -> Cache<T> {
        Cache {capacity: capacity, list:LinkedList::new()}
    }

    pub fn add_object(&mut self, object:T) {
        if self.list.len() == self.capacity {
            self.list.pop_back();
        }
        self.list.push_front(object);
    }

    pub fn get_object(&mut self, object:T) -> Option<T> {
        let mut iter = self.list.iter();
        match iter.next() {
            None => {return None;}
            Some(obj) => { 
                if *obj == object {
                    return Some(*obj); 
                } else {
                    return None;
                }
            }
        } 
    }

    pub fn clear_cache(&mut self) {
        self.list.clear();
    }

    pub fn move_to_top(&mut self, object:T) {
        match self.list.iter().position(|e| *e==object) {
                None => {},
                Some(idx) => {
                self.list.remove(idx);
                }
        }
        self.add_object(object);
    }
}