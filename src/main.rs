mod algorithms;
mod data_structures;
/* 
use crate::sorting::merge_sort::merge_sort;
use crate::algorithms::maximum_sub_array::maximum_sub_array;
*/
use crate::algorithms::sorting::insertion_sort::insertion_sort;
use crate::data_structures::heap::Heap;

fn main() {    
   println!("{:?}", insertion_sort(vec![21, 12, 4, 999, -99]));
   
}
