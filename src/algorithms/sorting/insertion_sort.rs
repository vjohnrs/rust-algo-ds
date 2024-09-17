/*

 - intilization: starts with single element as sorted  
 - maintenance: every for iteration leaves the array sorted until the iteration point
 - termination: completes relying on above two properties

i_exit variabe to work around while loop using "-1" to exit, since rust usizes can't be negative.  

*/

pub fn insertion_sort( mut arr: Vec<i64> ) -> Vec<i64> {        
    
    for j in 1..arr.len() {
       
        let key = arr[j];
        let mut i = (j - 1);        

        while arr[i] >= key {            
            
            arr[(i + 1)] = arr[i];
            
            if i == 0 {
                break
            }

            i = i - 1;                        
        }
        
        if arr[i] >= key{
            arr[i] = key; 
        }
        
        
    }
    println!("return {:?}", arr);
    return arr;

}

#[cfg(test)]
mod tests{
    
    use super::*;
    
    #[test]
    fn invoke(){
        assert_eq!(insertion_sort(vec![21, 12]), vec![12,21]);
        assert_eq!(insertion_sort(vec![21, 12, 4]), vec![4, 12,21]);
        assert_eq!(insertion_sort(vec![21, 12, 4, -99]), vec![-99, 4, 12, 21]);
       assert_eq!(insertion_sort(vec![21, 12, 4, 999, -99]), vec![-99, 4, 12, 21, 999]);
       insertion_sort(vec![21, 12, 4, 999, -99]);
       
    }
}