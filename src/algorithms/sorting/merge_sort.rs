/*

Tips: 
- Common overlooked detail is instead of comparing between L[i] and R[j], ending up comparing between arr[i] and arr[j].
*/

fn merge(arr: &mut Vec<i64>, p: usize, q: usize, r: usize) {

    let mut L = vec![];
    let mut R = vec![];

    for i in p..q+1 {
        L.push(arr[i]);
    }

    for j in q+1..r+1 {
        R.push(arr[j]);
    }

    L.push(i64::MAX);
    R.push(i64::MAX);

    let mut i:usize = 0;
    let mut j:usize = 0;

    for k in p..r+1 {
        if L[i] <= R[j] {
            arr[k] = L[i];
            i += 1; 
        } else {
            arr[k] = R[j];
            j += 1;
        }
    }

}

fn merge_sort (arr: &mut Vec<i64>, p: usize, r: usize) -> &mut Vec<i64> {

    if p < r {
        let q = (r + p) / 2;
        merge_sort(arr, p, q);
        merge_sort(arr, q+1, r);
        merge(arr, p, q, r);
    }

    return arr;
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn run () {
        let mut arr = vec![21, 12, 4, 999, -99];
        let len = arr.len();
        assert_eq!(merge_sort(&mut arr, 0, len - 1), &mut vec![-99, 4, 12, 21, 999]);

        let mut arr = vec![21];
        let len = arr.len();
        assert_eq!(merge_sort(&mut arr, 0, len - 1), &mut vec![21]);

        let mut arr = vec![4, 12, 21];
        let len = arr.len();
        assert_eq!(merge_sort(&mut arr, 0, len - 1), &mut vec![4, 12, 21]);

        let mut arr = vec![5, 5, 5, 5];
        let len = arr.len();
        assert_eq!(merge_sort(&mut arr, 0, len - 1), &mut vec![5,5,5,5]);

        // let mut arr = vec![];
        // let len = arr.len();
        // println!("{}", len);
        // assert_eq!(merge_sort(&mut arr, 0, len - 1), &mut vec![]);
    }
}