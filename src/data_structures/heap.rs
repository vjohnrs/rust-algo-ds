pub struct Heap {
    arr: Vec<i64>,
    heapsize: usize
}

impl Heap {    
    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }
    
    fn left(i: usize) -> usize {
        (2 * i) + 1
    }

    fn right(i: usize) -> usize {
        (2 * i) + 2
    }

    fn get_arr (&self) -> &Vec<i64> {
        return &self.arr;
    }

    fn max_heapify(&mut self, i: usize){

        let mut arr = &mut self.arr;

        let l = Self::left(i);
        let r = Self::right(i);

        let mut largest:usize = i;

        if l < self.heapsize && arr[l] > arr[largest] {
            largest = l; 
        } 
        
        if r < self.heapsize && arr[r] > arr[largest] {
            largest = r; 
        } 

        if largest != i {
            self.swap(i, largest);            
            self.max_heapify(largest);
        }
    }
    

    fn swap (&mut self, i: usize, j: usize){
        let mut arr = &mut self.arr;
        let mut temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }

    fn build_max_heap(&mut self) {                        
        for i in (0..(&self.arr.len() / 2)).rev() {
            self.max_heapify(i);
        }
    }
    
    pub fn new (input_arr: Vec<i64>) -> Self {
        let len = input_arr.len();
        return Heap {
            arr: input_arr,
            heapsize: len
        }
    }

    pub fn insert (&mut self, val: i64) {
        self.arr.push(val);
    }  
  // Todo
  /* pub fn heap_sort (&mut self) { 
        
        self.build_max_heap();        
        for i in (1..self.arr.len()).rev(){
            self.swap(0, i);
            self.heapsize = self.heapsize - 1;
            self.max_heapify(1);
        }
    } */ 
}

 #[cfg(test)]
 mod tests { 
     use super::*; 
     #[test] 
fn invoke(){        
}

#[test]    
fn test_parent_child_for_0(){
    assert_eq!(Heap::parent(1), 0);
    assert_eq!(Heap::parent(2), 0);
    assert_ne!(Heap::parent(3), 0);
    assert_ne!(Heap::parent(4), 0);
    assert_eq!(Heap::left(0), 1);
    assert_eq!(Heap::right(0), 2);    
}

#[test]
fn test_partent_child_for_1(){
    assert_eq!(Heap::left(1), 3);
    assert_eq!(Heap::right(1), 4);
    assert_eq!(Heap::parent(3), 1);
    assert_eq!(Heap::parent(4), 1);
}


#[test]
fn max_heapify_at_index_0(){
    let mut heap = Heap::new(vec![1,2,3]);
    heap.max_heapify(0);
    assert_eq!(heap.get_arr(), &vec![3,2,1]);
}

#[test]
fn max_heapify_at_index_1(){
    let mut heap = Heap::new(vec![16,4,10,14,7,9,3,2,8,1]);
    heap.max_heapify(1);
    assert_eq!(heap.get_arr(), &vec![16,14,10,8,7,9,3,2,4,1]);
}
#[test]
fn build_max_heap(){
    let mut heap = Heap::new(vec![4,1,3,2,16,9,10,14,8,7]);
    heap.build_max_heap();
    println!("{:?}", heap.get_arr());    
}
 // Todo
 /* 
#[test]
fn run_heap_sort(){
    let mut heap = Heap::new(vec![4,1,3,2,16,9,10,14,8,7]);
    heap.heap_sort();
    println!("sorted data {:?}", heap.get_arr());    
}
*/

}


