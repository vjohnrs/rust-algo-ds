use std::i64;

fn maximum_sub_array_cross(arr: &Vec<i64>, p: usize, q: usize, r: usize) -> (usize, usize, i64) {

    let mut left = q;
    let mut left_sum = i64::MIN; 

    let mut sum = 0;
    
    for i in (p..q+1).rev() {
        sum = arr[i] + sum; 
        if sum >= left_sum {
            left_sum = sum; 
            left = i;             
        }
    }

    let mut right = q;
    let mut right_sum = i64::MIN; 

    let mut sum = 0;
    
    for i in (q+1..r+1) {
        sum = arr[i] + sum; 
        if sum >= right_sum {
            right_sum = sum; 
            right = i;             
        }
    }
    
    return (left, right, left_sum + right_sum);
}
pub fn maximum_sub_array_recursive(arr: &Vec<i64>, p: usize, r: usize) -> (usize, usize, i64) {
    if p == r {
        return (p,r,arr[p]);
    }

    let q = (r + p) / 2; 

    let (left_i, left_j, left_max) = maximum_sub_array_recursive(arr, p, q);
    let (right_i, right_j, right_max) = maximum_sub_array_recursive(arr, q+1, r);
    let (cross_i, cross_j, cross_max) = maximum_sub_array_cross(arr, p, q, r);

    if left_max >= right_max && left_max >= cross_max {
        return (left_i, left_j, left_max);
    } else if right_max >= left_max && right_max >= cross_max {
        return (right_i, right_j, right_max);
    } else {
        return (cross_i, cross_j, cross_max);
    }  
        
}
#[cfg(test)]
mod tests {
    
    use super::*;
 
    
    #[test]
    fn run_recursive () {
        
        assert_eq!(maximum_sub_array_recursive(&vec![1,2,3,-4], 0, 3), (0, 2, 6));
        assert_eq!(maximum_sub_array_recursive(&vec![1,-4,22], 0, 2), (2, 2, 22));
        assert_eq!(maximum_sub_array_recursive(&vec![-12,-6,10,-4,22], 0, 4), (2, 4, 28));
        assert_eq!(maximum_sub_array_recursive(&vec![13, -3, -25, 20, -3, -16, -23, 18, 20, -7, 12, -5, -22, 15, -4, 7], 0, 15), (7, 10, 43));
    }

}
