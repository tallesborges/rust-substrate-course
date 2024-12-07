fn main() {
    // exercise 1
    exercise_1("Hello".to_string(), "World".to_string());

    // exercise 2
    let exercise2 = vec![1, 1, 1, 1, 1];
    exercise_2(&exercise2);
    println!("exercise2 = {:?}", exercise2);

    // exercise 3
    exercise_3();

    // exercise 4
    let mut exercise4 = "World".to_string();
    exercise_4(&mut exercise4, "Hello ");
    println!("exercise4 = {}", exercise4);

    // exercise 5
    println!(
        "exercise5 = {:?}",
        exercise_5(&vec![2, 2, 2], &vec![3, 3, 3])
    );

    // exercise 6
    let exercise6 = "Hello".to_string();
    println!("exercise6 = {}", exercise_6(&exercise6));

    // exercise 10
    let mut exercise10 = vec![1, 2, 3, 4, 5];
    println!("exercise10 = {:?}", exercise10);
    exercise_10(&mut exercise10);
    println!("exercise10 = {:?}", exercise10);
}

// 1. Clonando e Movendo Strings
// Escreva um programa que recebe duas strings como entrada. A primeira string
// deve ser movida para uma nova variável e a segunda string deve ser clonada para
// uma nova variável. Mostre os valores das strings originais e das novas após o
// movimento e clonagem. Explique a diferença entre mover e clonar.
fn exercise_1(s1: String, s2: String) {
    let s3 = s1;
    let s4 = s2.clone();

    // println!("s1 = {}, s2 = {}, s3 = {}, s4 = {}", s1, s2, s3, s4); // we can't use the s1 as it moved
    println!("s2 = {}, s3 = {}, s4 = {}", &s2, &s3, &s4);

    // when moving a value, the original value is no longer valid
    // when cloning a value, the value is cloned and allocated in a new location
}

// 2. Função de Soma com Empréstimo Imutável
// Crie uma função soma que recebe uma slice de números inteiros como uma
// referência imutável. A função deve calcular e retornar a soma dos números. No
// programa principal, crie um vetor de inteiros e passe uma slice para a função.
// Após chamar a função, mostre o valor do vetor original. Dica: procure por vec!.
fn exercise_2(numbers: &[i32]) {
    let sum: i32 = numbers.iter().sum();

    println!("sum = {}", sum);
}

// 3. Reatribuição e Ownership
// Crie um programa que tenha duas variáveis que armazenam strings. Reatribua
// uma das strings para a outra variável e mostre o que acontece com o valor da
// variável original após a reatribuição.
fn exercise_3() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");

    println!("s1 = {}, s2 = {}", s1, s2);

    let s2 = s1;
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // we can't use the s1 as it moved
}

// 4. Empréstimo Mutável
// Escreva uma função adicionar_prefixo que recebe uma string mutável e adiciona o
// prefixo "Prefixo: " a essa string. No programa principal, crie uma string e passe-a
// para a função como referência mutável. Mostre o valor da string após a
// modificação.
fn exercise_4(s: &mut String, prefix: &str) {
    s.insert_str(0, prefix);

    println!("s = {}", s);
}

// 5. Multiplicação de Vetores com Empréstimo Imutável
// Crie uma função que recebe dois vetores de números inteiros como referências
// imutáveis e retorna um novo vetor contendo os produtos dos elementos
// correspondentes. O programa principal deve criar dois vetores, passar ambos
// para a função, e depois imprimir o vetor resultante. Dica: procure por Vec<> e vec!
fn exercise_5(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    v1.iter().zip(v2.iter()).map(|(&x, &y)| x * y).collect()
}

// 6. Número de Caracteres com Empréstimo Imutável
// Implemente uma função chamada contar_caracteres que recebe uma referência
// imutável para uma string e retorna o número de caracteres nela. No programa
// principal, crie uma string e chame a função, mostrando o resultado. Dica: procure
// por chars e count.
fn exercise_6(s: &str) -> usize {
    s.chars().count()
}

// 7. Split de String com Empréstimo Imutável
// Escreva uma função que recebe uma referência imutável para uma string e retorna
// um vetor de palavras que são divididas por espaço. No programa principal, crie
// uma string, passe-a para a função e mostre o vetor resultante. Dica: procure por
// split_whitspace e collect.

// 8. Função com Várias Referências Imutáveis
// Escreva uma função concatenar que recebe duas referências imutáveis para
// strings e retorna uma nova string que é a concatenação das duas. No programa
// principal, passe duas strings para a função e mostre o resultado.

// 9. Função com Referência Mutável e Imutável
// Implemente uma função adicionar_elemento que recebe uma referência mutável
// para um vetor de inteiros e uma referência imutável para um número inteiro. A
// função deve adicionar o número ao vetor. No programa principal, crie um vetor e
// um número, e depois passe-os para a função. Dica: procure por Vec<>

// 10. Ciclo de Empréstimo Mutável
// Crie um programa que recebe um vetor de números inteiros. O programa deve
// usar uma função que altera o vetor dobrando o valor de cada elemento. Use
// referências mutáveis para modificar o vetor, sem perder o controle do ownership.
// Mostre o vetor após a modificação. Dica: procure por Vec<>
fn exercise_10(v: &mut Vec<i32>) {
    v.iter_mut().for_each(|x| *x *= 2);
}
