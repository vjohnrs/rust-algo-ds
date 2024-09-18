pub fn maximum_sub_array_linear (arr: & Vec<i64>) -> (usize, usize, i64) {

    let mut max_left: usize = 0;
    let mut max_right: usize = 0; 
    let mut max_sum: i64 = i64::MIN;

    let mut sum: i64 = 0; 


    for i in 0..arr.len() {
       
        sum = sum + arr[i];

        if sum >= max_sum {
            max_sum = sum;
            max_right = i;
        } else if sum < 0 {
            sum = 0;
            max_left = i +1;
        } 
    }
    return (max_left,max_right,max_sum);
}


#[cfg(test)]
mod tests {
    
    use super::*;
   
    #[test]
    fn run_linear(){
        assert_eq!(maximum_sub_array_linear(&vec![1,2,3]), (0, 2, 6));
        assert_eq!(maximum_sub_array_linear(&vec![1,2,3,-4]), (0, 2, 6));
        assert_eq!(maximum_sub_array_linear(&vec![1,-4,22]), (2, 2, 22));
        assert_eq!(maximum_sub_array_linear(&vec![-12,-6,10,-4,22]), (2, 4, 28));
        assert_eq!(maximum_sub_array_linear(&vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7]), (7, 10, 43));
    }
}
