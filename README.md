# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do projeto

Este projeto implementa um sistema de busca e recomendação para catálogos de produtos da MegaStore.
A solução combina:

- Indexação com tabelas hash para consultas rápidas por nome, marca, categoria e tags.
- Recomendação por grafo de similaridade usando travessia BFS com profundidade configurável.

## Tecnologias utilizadas

- Linguagem: Rust (edition 2021)
- Crates: `serde`, `serde_json`, `clap`, `anyhow`, `unicode-normalization`, `petgraph`
- Testes: `cargo test` (unitários e integração)

## Como executar o sistema de busca

1. Instale o Rust com `rustup`.
2. No diretório do projeto, compile e execute:

```bash
cargo run --release -- --input products.json --query "smartphone novatech"
```

### Parâmetros de CLI

- `--input <arquivo>`: arquivo JSON com produtos.
- `--query <texto>`: consulta textual (nome, marca, categoria, tags).
- `--id <n>`: id do produto base para recomendações.
- `--depth <n>`: profundidade de travessia no grafo (padrão: 1).
- `--k <n>`: quantidade máxima de recomendações (padrão: 5).

## Como executar os testes

```bash
cargo test
```

## Exemplos de uso

- Busca por nome e marca:

```bash
cargo run -- --input products.json --query "laptop novatech"
```

- Recomendação direta (vizinhos de 1 passo):

```bash
cargo run -- --input products.json --id 1 --depth 1 --k 3
```

- Recomendação expandida (2 passos no grafo):

```bash
cargo run -- --input products.json --id 1 --depth 2 --k 5
```

- Busca + recomendação na mesma execução:

```bash
cargo run -- --input products.json --query "wireless" --id 4 --depth 1 --k 3
```

## Arquitetura do sistema

- `src/models.rs`: modelo `Product` (id, name, brand, category, tags).
- `src/normalizer.rs`: normalização e tokenização para consultas robustas.
- `src/indexer.rs`: índices hash por nome, marca, categoria e tags.
- `src/search.rs`: motor de busca com score ponderado por tipo de match.
- `src/graph.rs`: grafo bipartido produto-feature e recomendação por BFS.
- `src/catalogue.rs`: coordenação de carga, indexação, busca e recomendação.
- `src/main.rs`: CLI e fluxo de execução.

## Algoritmos e estruturas de dados utilizados

- `HashMap<String, Vec<ProductId>>` para índices invertidos por atributo.
- Grafo bipartido não direcionado (`Product <-> Feature`) para similaridade.
- BFS limitada por profundidade para expandir recomendações sem explorar o grafo inteiro.

## Considerações sobre desempenho e escalabilidade

- Busca textual: acesso médio próximo de `O(1)` por token nos índices hash.
- Construção do grafo: custo linear no número de produtos e features conectadas, evitando comparação global produto-a-produto.
- Recomendação: custo proporcional ao subgrafo visitado até `depth`, controlado por `--depth`.

### Medição recomendada

- Execute em modo release:

```bash
cargo run --release -- --input products.json --query "gaming keyboard"
```

- Para comparar tempos em lote, use `hyperfine` (opcional).

## Contribuições

1. Abra uma issue descrevendo o problema ou melhoria.
2. Crie branch de feature e adicione testes para novas regras.
3. Rode `cargo test` antes de abrir PR.

## Licença

Este projeto está sob a licença MIT. Veja o arquivo `LICENSE`.

## Estrutura esperada do repositório

- `src/`: código fonte Rust
- `tests/`: testes de integração
- `Cargo.toml`: configuração do projeto
- `README.md`: documentação de uso e arquitetura
