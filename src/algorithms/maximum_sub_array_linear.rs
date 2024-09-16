fn maximum_sub_array_linear (arr: & Vec<i64>) -> (usize, usize, i64) {

    let mut sum = 0;    
    
    let mut sum_max = 0;    
    let mut left_max: usize = 0; 
    let mut right_max: usize = 0; 

    for i in 0..arr.len() {
        sum = sum + arr[i];

        if sum > sum_max {
            sum_max = sum; 
            right_max = i; 
        } else if sum < 0 {
            left_max = i; 
            sum = 0; 
        }
    }   

    return (left_max,right_max,sum_max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run(){
        println!("{:?}", maximum_sub_array_linear(&vec![1,2,3]));
        println!("{:?}", maximum_sub_array_linear(&vec![1,2,3,-4]));
        println!("{:?}", maximum_sub_array_linear(&vec![1,-4,22]));
        println!("{:?}", maximum_sub_array_linear(&vec![-12,-6,10,-4,22]));
    }
}