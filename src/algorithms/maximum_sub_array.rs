pub fn maximum_sub_array_nsquared(arr: &Vec<i64>) -> (usize, usize) {
    
    let mut max = i64::MIN; 
    let mut result: (usize, usize) = (0,0);

    for i in 0..arr.len() {
        for j in i+1..arr.len() {
           
            let diff = arr[j] - arr[i];
            
            if diff > max {
                max = diff;
                result = (i, j);
            }
        }
    }
    return result;
}


fn maximum_sub_array_cross (arr:&Vec<i64>, low: usize, mid: usize, high:usize) -> (usize, usize, i64) {
    // find left_index, max
    let mut left_sum = i64::MIN;
    let mut max_left:usize = mid;
    let mut sum = 0;
    
    for i in (low..mid+1).rev() {
        sum = sum + arr[i];

        if sum > left_sum {
            left_sum = sum;
            max_left = i;
        }
    }

    let mut right_sum = i64::MIN;
    let mut max_right:usize = mid+1;
    sum = 0;
    
    for i in mid+1..high+1 {
        sum = sum + arr[i];

        if sum > right_sum {
            right_sum = sum;
            max_right = i;
        }
    }

    return (max_left, max_right, left_sum + right_sum); 
}

fn maximum_sub_array_recursive(arr: &Vec<i64>, low: usize, high: usize) -> (usize, usize, i64) {    
    // base case
    if low == high {
        return (low, high, arr[low]);
    }

    let mid = (low + high) / 2;
    // find max in left
    let (left_low, left_high, left_sum) = maximum_sub_array_recursive(arr, low, mid);
    // find max in right
    let (right_low, right_high, right_sum) = maximum_sub_array_recursive(arr,mid+1, high);
    // find max across
    let (cross_low, cross_high, cross_sum) = maximum_sub_array_cross(arr, low, mid, high);
    // select highest of above three to return

    if left_sum >= right_sum && left_sum >= cross_sum {
        return (left_low, left_high, left_sum);
    } else if right_sum >= left_sum && right_sum >= cross_sum  {
        return (right_low, right_high, right_sum);
    } else {
        return (cross_low, cross_high, cross_sum);
    }    
}

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
    fn run_recursive () {
        let arr = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
        let len = arr.len();
        println!("{:?}", maximum_sub_array_recursive(&arr, 0 , len -1));
    }

    #[test]
    fn run_linear(){
        println!("{:?}", maximum_sub_array_linear(&vec![1,2,3]));
        println!("{:?}", maximum_sub_array_linear(&vec![1,2,3,-4]));
        println!("{:?}", maximum_sub_array_linear(&vec![1,-4,22]));
        println!("{:?}", maximum_sub_array_linear(&vec![-12,-6,10,-4,22]));
        println!("{:?}", maximum_sub_array_linear(&vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7]));

    }
}