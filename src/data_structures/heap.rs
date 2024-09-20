pub struct Heap {
    arr: Vec<i64>
}

impl Heap {    

    fn parent(i: usize) -> usize {
        i / 2
    }
    
    fn left(i: usize) -> usize {
        2 * i
    }

    fn right(i: usize) -> usize {
        (2 * i) + 1
    }
    
   pub fn new () -> Self {
        return Heap {
            arr: vec![]
        }
    }

    pub fn insert (&mut self, val: i64) {
        self.arr.push(val);
    }
    fn get_arr (&self) -> &Vec<i64> {
        return &self.arr;
    }
}

 #[cfg(test)]
 mod tests { 
     use super::*; 
     #[test] 
fn invoke(){
        let mut heap = Heap::new();
        heap.insert(24);
        heap.insert(48);
        println!("{:?}", heap.get_arr());
    }
}


