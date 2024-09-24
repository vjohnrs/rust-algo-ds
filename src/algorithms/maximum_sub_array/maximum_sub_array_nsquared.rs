pub fn maximum_sub_array_nsquared(arr: &Vec<i64>) -> (usize, usize) {

    let mut max = i64::MIN;
    let mut result: (usize, usize) = (0,0);

    for i in 0..arr.len() {
        for j in i+1..arr.len(){

            let diff = arr[j] - arr[i];
            if diff > max {
                max = diff;
                result = (i,j);
            }         
        }
    }

    return result;
    
}


#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn run_nsquared_on_the_prices_not_difference_like_others () {
        let arr = vec![10,11,7,10,6];
        let len = arr.len(); 
        println!("{:?}", maximum_sub_array_nsquared(&arr));

    }
   
}