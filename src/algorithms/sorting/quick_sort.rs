/*

Todo: update ability to handle non-distinct elements. 

*/

use rand::Rng;

fn swap (arr: &mut Vec<i64>, i: usize, j:usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn partition (arr: &mut Vec<i64>, p: usize , r: usize) -> usize {

    let key = arr[r];
    let mut i: i64 = (p as i64 - 1); 

    for j in p..r{
        if arr[j] <= key {
            i += 1 ;
            swap(arr, i as usize, j);
        }        
    }

    swap(arr, (i+1) as usize, r);
    return (i + 1) as usize;
}

fn random (arr: &mut Vec<i64>, p: usize , r: usize) {
    let mut rng = rand::thread_rng();
    let i = rng.gen_range(p..r);
    swap(arr, i, r);
}

fn quick_sort (arr: &mut Vec<i64>, p: usize , r: usize) -> &mut Vec<i64> {
    if p < r {
        random(arr, p, r);
        let q = partition(arr, p, r);
        if p < q {
            quick_sort(arr, p, q - 1);
        }
        if q+1 < r {
            quick_sort(arr, q + 1, r);
        }
    }

    return arr;
    
}

 #[cfg(test)]
 mod tests { 
     use super::*; 
     #[test] 
    fn invoke(){
        assert_eq!(quick_sort(&mut vec![5,4,3,2,1], 0, 4), &mut vec![1,2,3,4,5]);
        assert_eq!(quick_sort(&mut vec![1], 0, 0), &mut vec![1]);
        assert_eq!(quick_sort(&mut vec![33,55], 0, 1), &mut vec![33,55]);
        assert_eq!(quick_sort(&mut vec![55, 33], 0, 1), &mut vec![33,55]);
        assert_eq!(quick_sort(&mut vec![12, 1, 33], 0, 2), &mut vec![1, 12, 33]);
        assert_eq!(quick_sort(&mut vec![33, 1, 12], 0, 2), &mut vec![1, 12, 33]);
        assert_eq!(quick_sort(&mut vec![33, 12, 1], 0, 2), &mut vec![1, 12, 33]);
        assert_eq!(quick_sort(&mut vec![1, 33, 12], 0, 2), &mut vec![1, 12, 33]);

        assert_eq!(quick_sort(&mut vec![1, 33, 12, 77], 0, 3), &mut vec![1, 12, 33, 77]);
        assert_eq!(quick_sort(&mut vec![1, 33, 77, 12], 0, 3), &mut vec![1, 12, 33, 77]);
        assert_eq!(quick_sort(&mut vec![1, 77, 12, 33], 0, 3), &mut vec![1, 12, 33, 77]);
        assert_eq!(quick_sort(&mut vec![77, 33, 12, 1], 0, 3), &mut vec![1, 12, 33, 77]);

        //add update to handle non-distinct elements. 
        assert_ne!(quick_sort(&mut vec![77, 33, 77, 12, 1], 0, 3), &mut vec![1, 12, 33, 77, 77]);

    }
}