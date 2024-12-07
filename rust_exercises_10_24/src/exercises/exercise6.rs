// 6. Cálculo de Med́ ia com Erros em Entradas
// Implemente uma funçaõ que receba um vetor de strings representando números e calcule a média apenas dos números válidos. A função deve retornar um `Result<f64, String>` que contenha a média ou uma mensagem de erro se nenhum nuḿ ero for válido.

pub fn main() {
    let sample1 = vec![
        "1.0".to_string(),
        "2.5".to_string(),
        "invalid".to_string(),
        "3.7".to_string(),
    ];
    let sample2 = vec!["abc".to_string(), "def".to_string()];

    println!("Sample 1 result: {:?}", calculate_avarage(sample1));
    println!("Sample 2 result: {:?}", calculate_avarage(sample2));
}

pub fn calculate_avarage(numbers: Vec<String>) -> Result<f64, String> {
    let valid_numbers: Vec<f64> = numbers
        .iter()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();

    if valid_numbers.is_empty() {
        return Err("No valid numbers found".to_string());
    }

    let sum: f64 = valid_numbers.iter().sum();
    let count = valid_numbers.len() as f64;

    Ok(sum / count)
}
