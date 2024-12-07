// 5. Função com Lifetimes e Mut
// Crie uma função swap_refs que recebe duas referências mutáveis a variáveis i32.
// A função deve trocar os valores entre as duas variáveis usando lifetimes
// explicitamente, para evitar problemas de referências múltiplas.

pub fn main() {
    let mut a = 10;
    let mut b = 20;
    println!("Before: a = {}, b = {}", a, b);

    swap_refs(&mut a, &mut b);

    println!("After: a = {}, b = {}", a, b);
}

fn swap_refs<'a>(x: &'a mut i32, y: &'a mut i32) {
    let temp = *x;
    *x = *y;
    *y = temp;
}
