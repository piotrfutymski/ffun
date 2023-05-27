pub mod words_1;
pub mod probabilities;

fn main() {
    let tokens = vec!['a','b','c','d','e','f'];
    let density_matrix = vec![
        vec![0.,0.62,0.18,0.18,0.01,0.01],
        vec![0.62,0.,0.18,0.18,0.01,0.01],
        vec![0.18,0.18,0.,0.62,0.01,0.01],
        vec![0.18,0.18,0.62,0.,0.01,0.01],
        vec![0.01,0.01,0.01,0.01,0.,0.96],
        vec![0.01,0.01,0.01,0.01,0.96,0.0],
    ];
    let probability_matrix = probabilities::prob_matrix_from_density_matrix(&density_matrix);
    println!("{:?}",&probability_matrix);

    let mut cycles = probabilities::count_cycles(&density_matrix, &tokens);
    cycles.sort_by(|a,b|b.1.total_cmp(&a.1));

    println!("{:?}", cycles);
    
    let word = words_1::generate_colored_word(&tokens, &probability_matrix, 2000);
    word.iter().for_each(|w|print!("{}",w));
}
