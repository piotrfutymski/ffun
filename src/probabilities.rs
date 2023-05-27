pub fn prob_matrix_from_density_matrix(mat: & Vec<Vec<f64>>) ->Vec<Vec<f64>>{
    mat.iter()
        .map(|v|{
            let mut res = v.clone();
            let mut sum = v[0];
            for i in 1..v.len() {
                res[i] = res[i] + sum;
                sum = res[i]
            }
            res
        }).collect()
}


pub fn count_cycles(mat: & Vec<Vec<f64>>, tokens: &Vec<char>) -> Vec<(String, f64)>{
    generate_permutations(mat.len())
        .iter()
        .map(|v|(v.iter().map(|e|tokens[*e]).collect(), 
            {
                let mut res = mat[v[v.len()-1]][v[0]];
                for i in 1..v.len() {
                    res *= mat[v[i-1]][v[i]]
                }
                res
            }
            
        ))
        .collect()
}


fn generate_permutations(n: usize) -> Vec<Vec<usize>>{
    if(n == 1){ 
        return vec![vec![0]];
    }else{
        let mut res = generate_permutations(n-1);
        let mut new_perm = vec![vec![n-1]];
        for perm in res.iter(){
            for i in 1..perm.len()+1{
                let mut to_add = perm[..i].to_vec();
                to_add.push(n-1);
                to_add.append(&mut perm[i..].to_vec());
                new_perm.push(to_add)
            }
        }
        res.append(&mut new_perm);
        res
    }
}