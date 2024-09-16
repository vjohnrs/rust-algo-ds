mod sorting;
mod algorithms;
/* 
use crate::sorting::merge_sort::merge_sort;
use crate::sorting::insertion_sort::insertion_sort;
*/

//use crate::algorithms::maximum_sub_array::maximum_sub_array;

fn main() {
    
   // println!("{:?}", insertion_sort(vec![21, 12, 4, 999, -99]));
    
    let mut arr = vec![21, 12, 4, 999, -99];
    let len = arr.len();

    //println!("{:?}", merge_sort(&mut arr, 0, len - 1));
    let mid = 0;
    let low = 0;

    for i in (low..mid+1).rev() {
        println!("{}", i);
    }
}
