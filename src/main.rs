mod baixar;

use std::io::{stdin, stdout, Write};

fn main() {
    //URLS testadas: https://commons.wikimedia.org/w/index.php?search=dog+breeds&title=Special:MediaSearch&type=image
    //https://pt.wikipedia.org/wiki/Lewis_Hamilton

    let mut urls = Vec::new();

    println!("Digite as URLs que deseja processar. Digite 'sair' para finalizar a entrada.");

    loop {
        print!("Digite uma URL (ou 'sair' para terminar): ");
        stdout().flush().unwrap(); // Garantir que o texto seja exibido antes da entrada

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Erro ao ler a entrada.");
        let input = input.trim(); // Remover espaços e quebras de linha

        if input.eq_ignore_ascii_case("sair") {
            break;
        }

        if !input.is_empty() {
            urls.push(input.to_string());
            println!("URL adicionada: {}", input);
        } else {
            println!("Entrada vazia. Tente novamente.");
        }
    }

    if urls.is_empty() {
        println!("Nenhuma URL foi fornecida. Encerrando o programa.");
        return;
    }

    let output_dir = "imagens_baixadas";

    println!("\nPreparando para baixar {} páginas:", urls.len());

    // Iterar sobre as URLs e chamar a função baixar para cada uma
    for (i, url) in urls.iter().enumerate() {
        println!("Processando URL {} de {}: {}", i + 1, urls.len(), url);

        if let Err(err) = baixar::baixar(url, output_dir) {
            eprintln!("Erro ao processar {}: {}", url, err);
        }
    }

    println!("Processamento concluído. As imagens estão na pasta '{}'.", output_dir);
}
