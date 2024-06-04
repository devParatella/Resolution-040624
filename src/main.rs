// Esta é a função principal do programa.
// Ela demonstra o uso de variáveis mutáveis, literais de string e a macro println!.
fn main() {
    // Uma referência mutável a um literal de string representando o nome.
    // Ela é inicializada com o valor "Marcos" e depois atualizada para "Paratella".
    let mut _name: &str = "Marcos";
    _name = "Paratella";

    // Um inteiro sem sinal de 32 bits representando a idade.
    // Ele é inicializado com o valor 25.
    let _age: u32 = 25;

    // Imprime uma mensagem de saudação no console, usando o valor da variável `_name`.
    println!("Hello, {}!", _name);
}

