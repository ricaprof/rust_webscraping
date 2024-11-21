🖼️ Image Downloader 📥

Um programa poderoso para baixar todas as imagens de qualquer página da web! Ele é capaz de extrair URLs de imagens, processar links relativos, lidar com erros de rede e salvar as imagens em um diretório especificado. Simples, eficiente e personalizável! 🚀
🌟 Recursos

    Barra de Progresso Elegante: Acompanhe o progresso do download das imagens em tempo real com uma barra de progresso intuitiva.
    Resolução de URLs Relativas: Transforma URLs relativas em absolutas automaticamente.
    Mensagens de Erro Detalhadas: Diagnostique problemas facilmente com mensagens explicativas para cada tipo de erro.
    Configuração Flexível: Escolha o diretório de saída e o número de URLs a processar.
    Confiável: Verifica o status HTTP, evita falhas silenciosas e trata exceções com cuidado.

🚀 Como Usar
1️⃣ Pré-requisitos

Certifique-se de que o Rust está instalado no seu sistema. Se ainda não estiver, instale-o aqui.
2️⃣ Clone ou Faça o Download do Código

git clone https://github.com/seu-repositorio/image-downloader.git
cd image-downloader

3️⃣ Compile e Execute

Compile o programa:

cargo build --release

Execute o programa:

cargo run

4️⃣ Digite as URLs

O programa solicitará que você insira URLs das páginas para processar. Digite as URLs uma de cada vez e pressione Enter. Quando terminar, digite sair para iniciar o download.

Exemplo:

Digite as URLs que deseja processar. Digite 'sair' para finalizar a entrada.
Digite uma URL (ou 'sair' para terminar): https://commons.wikimedia.org/wiki/List_of_dog_breeds
URL adicionada: https://commons.wikimedia.org/wiki/List_of_dog_breeds
Digite uma URL (ou 'sair' para terminar): https://pt.wikipedia.org/wiki/Lionel_Messi
URL adicionada: https://pt.wikipedia.org/wiki/Lionel_Messi
Digite uma URL (ou 'sair' para terminar): sair

5️⃣ Acompanhe o Download

As imagens serão baixadas para o diretório configurado (padrão: imagens_baixadas). O progresso será exibido em uma barra dinâmica.
📂 Estrutura do Projeto

src/
├── baixar.rs       # Módulo principal para lógica de download
├── main.rs         # Entrada do programa
├── Cargo.toml      # Configurações do projeto
imagens_baixadas/   # Diretório para salvar as imagens baixadas

🛠️ Configuração Personalizada

Você pode ajustar os seguintes parâmetros:

    Diretório de saída: Mude o diretório onde as imagens serão salvas.
    URLs pré-definidas: Adicione URLs no vetor padrão para testar rapidamente.

🧪 Exemplos de Execução
Caso Sucesso

Acessando a URL: https://commons.wikimedia.org/wiki/List_of_dog_breeds
Conexão bem-sucedida! Lendo conteúdo...
Procurando imagens na página...
Foram encontradas 12 imagens. Iniciando downloads...
Baixando imagem 1 de 12
Imagem salva: imagens_baixadas/dog1.jpg
Baixando imagem 2 de 12
Imagem salva: imagens_baixadas/dog2.jpg
...
Download concluído!

Caso Erro

Acessando a URL: https://commons.wikimedia.org/wiki/invalid_url
Erro ao acessar a URL https://commons.wikimedia.org/wiki/invalid_url: erro de conexão.
