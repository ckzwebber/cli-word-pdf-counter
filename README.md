# WordCount CLI - Contagem de Palavras em PDF

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/) [![Cargo](https://img.shields.io/badge/Cargo-f5F5F5?style=for-the-badge&logo=cargo&logoColor=black)](https://crates.io/) [![clap](https://img.shields.io/crates/v/clap?style=for-the-badge)](https://crates.io/crates/clap) [![pdf_extract](https://img.shields.io/crates/v/pdf_extract?style=for-the-badge)](https://crates.io/crates/pdf_extract) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)

Seja bem-vindo! Este repositório contém o **WordCount CLI**, uma ferramenta de linha de comando desenvolvida em **Rust** para extrair texto de arquivos PDF e exibir as palavras mais frequentes de maneira rápida e eficiente.

---

## 📌 Visão Geral

O **WordCount CLI** permite:

- Verificar se o arquivo PDF informado existe.
- Extrair texto do PDF utilizando a crate `pdf_extract`.
- Limpar o texto (converter para minúsculas e remover caracteres não alfabéticos).
- Contar a frequência de cada palavra.
- Exibir as **10** palavras mais frequentes por padrão.

---

## 🛠️ Tecnologias Utilizadas

- **Rust**
- **clap** para parsing de argumentos de linha de comando
- **pdf_extract** para extração de texto de arquivos PDF
- **std** (`std::path`, `std::collections::HashMap`, entre outros)

---

## 🚀 Funcionalidades

- Checagem de existência do arquivo PDF.
- Tratamento de erros com mensagens claras.
- Extração e limpeza de texto para análise precisa.
- Contagem e ordenação das palavras mais frequentes.
- Saída no terminal com formato legível.

---

## 📦 Como Compilar e Executar

### Pré-requisitos

- [Rust e Cargo](https://www.rust-lang.org/tools/install) instalados em sua máquina.

### Compilando

No diretório raiz do projeto, execute:

```bash
cargo build --release
```

O binário compilado ficará disponível em `target/release/wordcount`.

### Instalando via Cargo

Para instalar globalmente:

```bash
cargo install --path .
```

### Executando

```bash
wordcount --file path/para/seu_arquivo.pdf
```

Você verá no terminal algo como:

```
Processing file: seu_arquivo.pdf
Top 10 most frequent words:
exemplo - 42x
rust - 37x
...
```

---

## 🤝 Contribuições

Contribuições são bem-vindas! Para contribuir:

1. Fork este repositório.
2. Crie uma branch com sua feature ou correção: `git checkout -b minha-feature`.
3. Faça commit das suas alterações: `git commit -m "Minha contribuição"`.
4. Envie para a branch original: `git push origin minha-feature`.
5. Abra um Pull Request.

---

## 📞 Contato

Fique à vontade para entrar em contato comigo pelo meu [LinkedIn](https://www.linkedin.com/in/cmiguelwm/) para dúvidas ou sugestões.

---

## 📄 Licença

Este projeto está licenciado sob a licença **MIT**. Consulte o arquivo [LICENSE](LICENSE) para mais detalhes.
