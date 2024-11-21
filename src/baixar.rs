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

    println!("Acessando a URL: {}", base_url);

    let response = client
        .get(base_url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36",
        )
        .send();

    // Verificar se a requisição foi bem-sucedida
    let response = match response {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Erro ao acessar a URL {}: {}", base_url, err);
            return Err(Box::new(err));
        }
    };

    if response.status().is_success() {
        println!("Conexão bem-sucedida! Lendo conteúdo...");

        let body = match response.text() {
            Ok(text) => text,
            Err(err) => {
                eprintln!("Erro ao ler o conteúdo da página: {}", err);
                return Err(Box::new(err));
            }
        };

        let document = Document::from(body.as_str());
        let base_url = Url::parse(base_url).map_err(|err| {
            eprintln!("Erro ao interpretar a URL base '{}': {}", base_url, err);
            err
        })?;

        // Criar diretório para salvar as imagens
        if let Err(err) = fs::create_dir_all(output_dir) {
            eprintln!(
                "Erro ao criar o diretório '{}': {}. Verifique as permissões do sistema.",
                output_dir, err
            );
            return Err(Box::new(err));
        }

        println!("Procurando imagens na página...");
        // Coletar todas as URLs das imagens
        let image_links: Vec<_> = document
            .find(Name("img"))
            .filter_map(|img| img.attr("src"))
            .filter_map(|src| {
                base_url.join(src).map_err(|err| {
                    eprintln!("Erro ao resolver URL relativa '{}': {}", src, err);
                    err
                }).ok()
            })
            .collect();

        if image_links.is_empty() {
            eprintln!("Nenhuma imagem foi encontrada na página.");
            return Ok(());
        }

        println!("Foram encontradas {} imagens. Iniciando downloads...", image_links.len());

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

            let image_response = client.get(image_url.as_str()).send();
            let image_response = match image_response {
                Ok(resp) if resp.status().is_success() => resp,
                Ok(resp) => {
                    eprintln!(
                        "Erro ao baixar a imagem '{}': Código de status {}",
                        image_url,
                        resp.status()
                    );
                    continue;
                }
                Err(err) => {
                    eprintln!("Erro ao baixar a imagem '{}': {}", image_url, err);
                    continue;
                }
            };

            // Gerar nome do arquivo com base na URL
            let filename = image_url
                .path_segments()
                .and_then(|segments| segments.last())
                .unwrap_or("imagem_desconhecida")
                .replace("?", "_");

            let filepath = format!("{}/{}", output_dir, filename);

            // Salvar a imagem no disco
            let bytes = match image_response.bytes() {
                Ok(bytes) => bytes,
                Err(err) => {
                    eprintln!("Erro ao ler os dados da imagem '{}': {}", image_url, err);
                    continue;
                }
            };

            if let Err(err) = fs::File::create(&filepath).and_then(|mut file| file.write_all(&bytes)) {
                eprintln!("Erro ao salvar a imagem '{}': {}", filepath, err);
                continue;
            }

            println!("Imagem salva: {}", filepath);
            bar.inc(1);
        }

        bar.finish_with_message("Download concluído!");
    } else {
        eprintln!(
            "Erro: Falha ao acessar a página. Código de status: {}",
            response.status()
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Falha ao acessar a página."
        )));
    }

    Ok(())
}
