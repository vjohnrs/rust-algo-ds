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

fn maximum_sub_array(arr: &Vec<i64>, low: usize, high: usize) -> (usize, usize, i64) {    
    // base case
    if low == high {
        return (low, high, arr[low]);
    }

    let mid = (low + high) / 2;
    // find max in left
    let (left_low, left_high, left_sum) = maximum_sub_array(arr, low, mid);
    // find max in right
    let (right_low, right_high, right_sum) = maximum_sub_array(arr,mid+1, high);
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

#[cfg(test)]
mod tests {
    
    use super::*;
    
    #[test]
    fn run () {
        let arr = vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7];
        let len = arr.len();
        println!("{:?}", maximum_sub_array(&arr, 0, len - 1));
    }
}