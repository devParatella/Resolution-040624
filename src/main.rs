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

    //Em Rust, o uso de underscores (_) em variáveis tem algumas convenções e
    // significados específicos. Aqui estão algumas das principais utilizações:
    let (_x, _) = (5, 10);
    // Aqui, estamos ignorando o segundo valor (10).

    //Prefixo de Variáveis Não Utilizadas: Se você declarar uma variável mas 
    //não for utilizá-la, você pode prefixá-la com _ para evitar warnings de 
    //variáveis não utilizadas.
    let _unused_variable: i32 = 42;
    // Isto informa ao compilador que você sabe que a variável não será usada.

    //...
    let some_value: i32 = 5; // Define the variable `some_value`
    
    //Subdivisão de Padrões em match: Dentro de um bloco match, _ pode ser 
    //usado para corresponder a qualquer valor, funcionando como um 
    //"correspondente padrão".
    match some_value {
        1 => println!("Um"),
        2 => println!("Dois"),
        _ => println!("Outro valor"),
    }
    // O "_" aqui corresponderá a qualquer valor que não seja 1 ou 2.

    }


