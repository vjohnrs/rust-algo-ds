use crate::Heap;

struct PQ {
    heap: Heap
}

impl PQ {
    fn new (arr: Vec<i64>) -> Self {        
        PQ {
            heap: Heap::new(arr),            
        }
    }    

    // fn heap_maximum(&self) -> i64 {
    //     self.heap.arr[0]
    // }

    // fn heap_extract_maximum(&mut self) -> i64 {

    //     let heapsize = self.heap.heapsize;        
    //     let mut arr = &mut self.heap.arr;

    //     if heapsize < 1 {
    //         panic!("no more elements in priority queue");        
    //     }

    //     let mut max = arr[0];

    //     arr[0] = arr[heapsize - 1];
    //     self.heap.heapsize -= 1;
    //     self.heap.max_heapify(0);
        
    //     return max 
    // }

    // fn heap_increase_key(&mut self, mut i:usize, key: i64) {

    //     let heapsize = self.heap.heapsize;                

    //     if key < self.heap.arr[i] {
    //         panic!("new key smaller than current key");
    //     }

    //     self.heap.arr[i] = key;
    //     let parent = Heap::parent(i);

    //     while i > 0 && self.heap.arr[i] > self.heap.arr[parent] {
    //         self.heap.swap(i, parent);
    //         i = parent;
    //     }
    // }

    // fn max_heap_insert(key: i64) {
        
    // }
}

 #[cfg(test)]
mod tests { 
    use super::*; 
    #[test] 
    fn invoke(){
        let mut pq = PQ::new(vec![4,12,29]);  
        pq.heap.build_max_heap();   
        println!("{:?}", pq.heap.arr);   
        // pq.heap_extract_maximum();     
    }
}