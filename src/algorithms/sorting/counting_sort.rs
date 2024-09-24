pub fn counting_sort (arr: Vec<usize>) -> Vec<usize> {

    let mut k = 0; 
    let mut hold_arr: Vec<usize> = vec![];
    let mut r_arr: Vec<usize> = vec![0; arr.len()];
    
    // find the largest interger. 
    for i in 0..arr.len() {
        if arr[i] > k {
            k = arr[i];
        }
    }
    
    // default all index to 0; since upon seeing the number the ith location will be incremented
    for i in 0..k+1 {
        hold_arr.push(0);
    }

    // increment ith location
    for i in 0..arr.len() {
        hold_arr[arr[i]] = hold_arr[arr[i]] + 1;
    }

    // each location replaced with number of elements lower.
    for i in 1..k+1 {
        hold_arr[i] = hold_arr[i] + hold_arr[i - 1];
    }

    // from last elem keep locating its spot in return array
    for i in (0..arr.len()).rev() {
        r_arr[hold_arr[arr[i]]  - 1] = arr[i];
        hold_arr[arr[i]] = hold_arr[arr[i]] - 1;
    }

    return r_arr;

} 
 #[cfg(test)]
 mod tests { 
    use super::*; 
    
    #[test] 
    fn invoke(){
        assert_eq!(counting_sort(vec![2,5,3,0,2,3,0,3]), vec![0,0,2,2,3,3,3,5]);

        assert_eq!(counting_sort(vec![0]), vec![0]);

        assert_eq!(counting_sort(vec![0,1]), vec![0,1]);
        assert_eq!(counting_sort(vec![1,0]), vec![0,1]);
    }
}