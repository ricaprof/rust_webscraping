mod baixar;

fn main() {
    // Vetor de URLs para processar
    let urls = vec![
        "https://commons.wikimedia.org/wiki/List_of_dog_breeds",
        "https://pt.wikipedia.org/wiki/Lionel_Messi",
        "https://pt.wikipedia.org/wiki/Lewis_Hamilton",
    ];

    let output_dir = "imagens_baixadas";

    println!("Preparando para baixar {} páginas:", urls.len());

    // Iterar sobre as URLs e chamar a função baixar para cada uma
    for (i, url) in urls.iter().enumerate() {
        println!("Processando URL {} de {}: {}", i + 1, urls.len(), url);

        if let Err(err) = baixar::baixar(url, output_dir) {
            eprintln!("Erro ao processar {}: {}", url, err);
        }
    }
}
