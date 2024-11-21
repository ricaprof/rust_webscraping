extern crate reqwest;
extern crate select;

use indicatif::{ProgressBar, ProgressStyle};
use select::document::Document;
use select::predicate::Name;
use std::fs;
use std::io::Write;
use url::Url;

pub fn baixar(base_url: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Cliente HTTP para fazer requisições
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(base_url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36",
        )
        .send()?;

    // Verificar se a requisição foi bem-sucedida
    if response.status().is_success() {
        let body = response.text()?;
        let document = Document::from(body.as_str());
        let base_url = Url::parse(base_url)?;

        // Criar diretório para salvar as imagens
        fs::create_dir_all(output_dir)?;

        // Coletar todas as URLs das imagens
        let image_links: Vec<_> = document
            .find(Name("img"))
            .filter_map(|img| img.attr("src"))
            .filter_map(|src| base_url.join(src).ok()) // Resolver URLs relativas usando a URL base
            .collect();

        // Configurar a barra de progresso
        let bar = ProgressBar::new(image_links.len() as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40.cyan/blue} {pos}/{len} [{elapsed_precise}] {msg}")
                .expect("Erro ao configurar o estilo da barra")
                .progress_chars("=>-"),
        );

        // Baixar as imagens
        for (index, image_url) in image_links.iter().enumerate() {
            bar.set_message(format!("Baixando imagem {} de {}", index + 1, image_links.len()));

            // Fazer o download da imagem
            let image_response = client.get(image_url.as_str()).send()?;
            if image_response.status().is_success() {
                // Gerar nome do arquivo com base na URL
                let filename = image_url
                    .path_segments()
                    .and_then(|segments| segments.last())
                    .unwrap_or("imagem_desconhecida")
                    .replace("?", "_");

                let filepath = format!("{}/{}", output_dir, filename);

                // Salvar a imagem no disco
                let mut file = fs::File::create(&filepath)?;
                file.write_all(&image_response.bytes()?)?;
            } else {
                println!("Erro ao baixar a imagem: {}", image_url);
            }

            bar.inc(1);
        }

        bar.finish_with_message("Download completo!");
    } else {
        eprintln!(
            "Falha ao acessar a página. Código de status: {}",
            response.status()
        );
    }

    Ok(())
}
