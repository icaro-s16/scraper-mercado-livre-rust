# 🦀 Rust Web Scraper - Mercado Livre (Educational)

Este é um projeto de **Web Scraping** desenvolvido em **Rust** para extrair dados de produtos (Notebooks) do Mercado Livre e salvá-los em um arquivo JSON.

> ⚠️ **AVISO LEGAL / DISCLAIMER**
>
> Este software foi desenvolvido estritamente para **fins educacionais e de aprendizado** sobre a linguagem Rust, automação de navegadores e manipulação de DOM.
> Este projeto **não** tem fins comerciais, não deve ser utilizado para sobrecarregar os servidores do Mercado Livre e não é afiliado à plataforma. O uso deste código é de total responsabilidade do usuário.

## 🛠️ Tecnologias Utilizadas

* **[Rust](https://www.rust-lang.org/)**: Linguagem principal.
* **[Thirtyfour](https://crates.io/crates/thirtyfour)**: Biblioteca de automação de navegador (WebDriver) para Rust.
* **[Tokio](https://tokio.rs/)**: Runtime assíncrono.
* **[Serde](https://serde.rs/)**: Framework para serialização/deserialização de dados (JSON).
* **[clearscreen](https://crates.io/crates/clearscreen)**: Utilit para apagar a tela do terminal quando o usuário achar nescessário

## ⚙️ Funcionalidades

* Navegação automatizada com **Firefox (Geckodriver)**.
* Busca automática por termos (ex: "notebooks").
* Tratamento de paginação (scroll automático e clique no botão "Seguinte").
* Contorno de "Sticky Headers" e carregamento dinâmico (Lazy Loading).
* Extração de Nome e Preço dos produtos.
* Exportação dos dados para `products.json`.

## 🚀 Pré-requisitos

Para rodar este projeto, você precisa ter instalado:

1.  **Rust & Cargo**: [Instalação oficial](https://www.rust-lang.org/tools/install).
2.  **Mozilla Firefox**: O navegador.
3.  **Geckodriver**: O driver que permite controlar o Firefox.
    * *Linux/Mac*: Instale via gerenciador de pacotes ou baixe o binário e coloque no PATH.
    * *Windows*: Baixe o binário e adicione ao PATH do sistema.

## ▶️ Como Rodar

1.  Clone este repositório:
    ```bash
    git clone https://github.com/icaro-s16/scraper-mercado-livre-rust
    cd scraper-mercado-livre-rust
    ```

2.  Inicie o servidor do Geckodriver em um terminal separado (se não estiver configurado para auto-start):
    ```bash
    geckodriver
    # Deve aparecer "Listening on 127.0.0.1:4444"
    ```

3.  Rode o projeto Rust:
    ```bash
    cargo run
    ```
4. Utilize a interface do usuário para inciar o scrap com o produto desejado
    ```bash
    scrap <product_name>
    ```

4.  Aguarde o navegador abrir e realizar a tarefa. Ao final, verifique o arquivo `products.json` gerado na raiz.

## 📝 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.