pub struct Heap {
    pub arr: Vec<i64>,
    pub heapsize: usize
}

impl Heap {
    pub fn parent (i: usize) -> usize {
        (i - 1)/ 2
    }

    fn left (i: usize) -> usize {
        (2 * i) + 1      
    }

    fn right (i: usize) -> usize {
        (2 * i) + 2
    }

    pub fn new(arr: Vec<i64>) -> Self {
        
        let len = arr.len();

        return Self {
            arr: arr,
            heapsize: len - 1
        }
    }

    pub fn swap (&mut self, i:usize, j:usize) {
        let mut arr = &mut self.arr;
        let temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }

    pub fn max_heapify (&mut self, i:usize) {

        let mut arr = &mut self.arr;

        let mut largest = i; 
        let left = Self::left(i);
        let right = Self::right(i);

        if left <= self.heapsize && arr[left] >= arr[largest] {
            largest = left; 
        }

        if right <= self.heapsize && arr[right] >= arr[largest] {
            largest = right; 
        }

        if i != largest {
            self.swap(i, largest);
            self.max_heapify(largest);
        }
    }

    pub fn build_max_heap (&mut self) {
        for i in (0..(&self.arr.len() / 2)).rev() {
            &self.max_heapify(i);
        }
    }

    fn heap_sort (&mut self) {
                
        self.build_max_heap();

        for i in (1..self.arr.len()).rev(){       
            self.swap(0, i);
            self.heapsize -= 1;
            self.max_heapify(0);
        }

    }
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
        assert_eq!(heap.arr, vec![3,2,1]);
    }

    #[test]
    fn max_heapify_at_index_1(){
        let mut heap = Heap::new(vec![16,4,10,14,7,9,3,2,8,1]);
        heap.max_heapify(1);
        assert_eq!(heap.arr, vec![16,14,10,8,7,9,3,2,4,1]);
    }

    #[test]
    fn build_max_heap(){
        let mut heap = Heap::new(vec![4,1,3,2,16,9,10,14,8,7]);
        heap.build_max_heap();
        assert_eq!(heap.arr, vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);        
    }

    #[test]
    fn run_heap_sort(){
        let mut heap = Heap::new(vec![4,1,3,2,16,9,10,14,8,7]);
        heap.heap_sort();
        assert_eq!(heap.arr, vec![1, 2, 3, 4, 7, 8, 9, 10, 14, 16]);
        
        let mut heap = Heap::new(vec![1]);
        heap.heap_sort();
        assert_eq!(heap.arr, vec![1]);

        let mut heap = Heap::new(vec![21,1]);
        heap.heap_sort();
        assert_eq!(heap.arr, vec![1,21]);
    }

}


