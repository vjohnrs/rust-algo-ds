/*

 - intilization: starts with single element as sorted  
 - maintenance: every for iteration leaves the array sorted until the iteration point
 - termination: completes relying on above two properties

i_exit variabe to work around while loop using "-1" to exit, since rust usizes can't be negative.  

*/





fn insertion_sort (mut arr: Vec<i64>) -> Vec<i64> {

    for j in 1..arr.len() {
        
        let key = arr[j];
        
        let mut i = j - 1;        
        let mut i_exit: i64 = i as i64;

        while  i_exit >= 0 && arr[i] >= key {            
           
            arr[i+1] = arr[i];          
            i_exit -= 1;            
            
            if i_exit >= 0 {
                i -= 1;
            }            
        }

        arr[(i_exit + 1) as usize] = key;        
    }

    return arr;
}


fn main() {
    println!("Insertion Sort");

    let arr = vec![5,4,3,2,1];
    assert_eq!(insertion_sort(arr), [1,2,3,4,5]);

    let arr2 = vec![6,4,3,2,1];
    assert_eq!(insertion_sort(arr2), [1,2,3,4,6]);
}
