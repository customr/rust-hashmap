pub const INITIAL_SIZE: usize = 17;

pub struct HashMap<T, U> 
where T: Hashable + PartialEq,
{
    pub occupied_count: usize,
    pub cells: Vec<HashCell<T, U>>
}

#[derive(Default, Clone)]
pub struct HashCell<T, U> 
where T: Hashable + PartialEq,
{
    pub key: T,
    pub value: U,
    pub occupied: bool
}

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for usize {
    fn hash(&self) -> usize {
        *self
    }
}

impl Hashable for String {
    fn hash(&self) -> usize {
        let mut hash: usize = 5381;
        for c in self.chars(){
            hash = (hash << 5) + hash + (c as usize);
        }
        return hash;
    }
}

impl<T, U> HashMap<T, U> 
where 
    T: Hashable + Default + Clone + PartialEq,
    U: Default + Clone,
{
    pub fn new() -> Self{ 
        Self {
            occupied_count: 0,
            cells: vec![HashCell::<T, U>::default(); INITIAL_SIZE]
        }
    }

    fn extend(&mut self){ 
        let mut new_self = Self {
            occupied_count: 0,
            cells: vec![HashCell::<T, U>::default(); self.cells.len() * 2 + 1]
        };

        for bucket in self.cells.iter(){
            if bucket.occupied{
                new_self.insert(bucket.key.clone(), bucket.value.clone())
            }
        }

        *self = new_self;
    }

    pub fn insert(&mut self, key: T, value: U){
        if let Some(old_cell) = self.get_mut(&key){
            *old_cell = value;
        } else {
            let mut hash = key.hash() % self.cells.len();
            while self.cells[hash].occupied{
                hash = (hash + 1) % self.cells.len();
            }
            self.cells[hash].key = key;
            self.cells[hash].value = value;
            self.cells[hash].occupied = true;
            self.occupied_count = self.occupied_count + 1;
            if self.occupied_count as f32 / self.cells.len() as f32 > 0.75{
                self.extend()
            }
        }
    }

    fn get_index(&self, key: &T) -> Option<usize> {
        let mut hash = key.hash() % self.cells.len();
        for _ in 0..self.cells.len(){
            if self.cells[hash].key == *key || !self.cells[hash].occupied {
                break
            }

            hash = (hash + 1) % self.cells.len();
        }

        if self.cells[hash].occupied && self.cells[hash].key == *key{
            return Some(hash);
        } else {
            return None;
        }
    }

    pub fn get(&self, key: &T) -> Option<&U>{
        if let Some(hash) = self.get_index(key) {
            Some(&self.cells[hash].value)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut U>{
        if let Some(hash) = self.get_index(key) {
            Some(&mut self.cells[hash].value)
        } else {
            None
        }
    }
}
