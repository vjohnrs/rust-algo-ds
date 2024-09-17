fn matrix_multiplication_n_cube (A: Vec<Vec<i64>>, B: Vec<Vec<i64>>) -> Vec<Vec<i64>> {

    let mut C: Vec<Vec<i64>> = vec![];

    for i in 0..A.len() {        
        let mut c: Vec<i64> = vec![]; 
        for j in 0..A[i].len() {                         
            let mut sum = 0;
           
            for k in 0..B.len() {                                
                sum = sum + (&A[i][k] * &B[k][j]);
            }

            c.push(sum);
        }
        C.push(c);
    }

    return C;
} 
 #[cfg(test)]
 mod tests { 
    use super::*; 
    #[test]
    fn invoke() {
        println!("{:?}", matrix_multiplication_n_cube(
            vec![ vec![1,2],  vec![3,4]],
            vec![ vec![5,6],  vec![7,8]]));
    }
}