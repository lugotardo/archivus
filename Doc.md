# FileUtils - Biblioteca de Utilitários de Arquivo em Rust

`FileUtils` é uma biblioteca abrangente para manipulação de arquivos e diretórios em Rust, projetada para simplificar operações comuns de I/O, validação, listagem e busca de arquivos. Esta documentação fornece uma visão geral das funcionalidades, exemplos de uso e detalhes técnicos da biblioteca.

## Índice

1. [Visão Geral](#visão-geral)
2. [Estruturas e Enums](#estruturas-e-enums)
    - [FileUtilsError](#fileutilserror)
    - [FileInfo](#fileinfo)
    - [FileFilter](#filefilter)
    - [DirectoryStats](#directorystats)
3. [Funcionalidades Principais](#funcionalidades-principais)
    - [Validação de Arquivos e Diretórios](#validação-de-arquivos-e-diretórios)
    - [Listagem de Arquivos e Diretórios](#listagem-de-arquivos-e-diretórios)
    - [Busca de Arquivos](#busca-de-arquivos)
    - [Operações de Leitura e Escrita](#operações-de-leitura-e-escrita)
    - [Operações de Diretório](#operações-de-diretório)
    - [Utilitários Convenientes](#utilitários-convenientes)
4. [Funções Auxiliares](#funções-auxiliares)
5. [Exemplos de Uso](#exemplos-de-uso)
6. [Testes Unitários](#testes-unitários)
7. [Notas de Performance](#notas-de-performance)
8. [Dependências](#dependências)

## Visão Geral

A biblioteca `FileUtils` fornece uma interface simplificada para operações com arquivos e diretórios, incluindo validação, listagem, busca e manipulação de conteúdo. Todas as operações retornam resultados encapsulados em `Result` com o tipo de erro personalizado `FileUtilsError`, garantindo tratamento robusto de erros.

A biblioteca é projetada para ser:
- **Segura**: Manipulação de erros detalhada com enums personalizados.
- **Flexível**: Suporta filtros personalizados e busca recursiva.
- **Performática**: Otimizada para operações comuns com uso eficiente de recursos.
- **Fácil de usar**: API intuitiva com documentação detalhada e exemplos.

## Estruturas e Enums

### FileUtilsError

Enum que representa diferentes tipos de erro que podem ocorrer durante operações de arquivo.

```rust
pub enum FileUtilsError {
    NotFound(String),
    PermissionDenied(String),
    IoError(String),
    InvalidExtension(String),
    InvalidPath(String),
}
```

- Implementa `std::fmt::Display` e `std::error::Error` para formatação e integração com o sistema de erros do Rust.
- Converte automaticamente erros de `std::io::Error` para o tipo apropriado.

### FileInfo

Estrutura que contém informações detalhadas sobre um arquivo ou diretório.

```rust
pub struct FileInfo {
    pub path: PathBuf,
    pub name: String,
    pub extension: Option<String>,
    pub size: u64,
    pub is_directory: bool,
    pub is_file: bool,
    pub modified: Option<u64>,
}
```

- Criada com `FileInfo::new(path)` a partir de um caminho.
- Fornece metadados como nome, extensão, tamanho e timestamp de modificação.

### FileFilter

Estrutura para configurar filtros de busca de arquivos.

```rust
pub struct FileFilter {
    pub extensions: Option<Vec<String>>,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub include_directories: bool,
    pub include_files: bool,
    pub recursive: bool,
}
```

- Suporta filtros por extensão, tamanho mínimo/máximo, tipo (arquivo/diretório) e busca recursiva.
- Implementa `Default` para valores padrão.

### DirectoryStats

Estrutura para estatísticas de um diretório.

```rust
pub struct DirectoryStats {
    pub file_count: usize,
    pub directory_count: usize,
    pub total_size: u64,
    pub extensions: HashMap<String, usize>,
    pub largest_file_size: u64,
    pub largest_file_name: Option<String>,
}
```

- Inclui contagem de arquivos e diretórios, tamanho total, contagem por extensão e informações sobre o maior arquivo.

## Funcionalidades Principais

### Validação de Arquivos e Diretórios

- `file_exists(path)`: Verifica se um caminho é um arquivo.
- `directory_exists(path)`: Verifica se um caminho é um diretório.
- `path_exists(path)`: Verifica se um caminho existe (arquivo ou diretório).
- `has_extension(path, extension)`: Verifica se um arquivo tem a extensão especificada.
- `is_empty(path)`: Verifica se um arquivo está vazio (0 bytes).

### Listagem de Arquivos e Diretórios

- `list_files(dir_path)`: Lista apenas arquivos em um diretório.
- `list_directories(dir_path)`: Lista apenas diretórios em um diretório.
- `list_all(dir_path)`: Lista todos os itens (arquivos e diretórios).
- `list_with_filter(dir_path, filter)`: Lista itens com base em um filtro personalizado, com suporte a busca recursiva.

### Busca de Arquivos

- `find_by_name(dir_path, pattern, recursive)`: Busca arquivos por nome com suporte a wildcards (`*` e `?`).
- `find_by_extension(dir_path, extension, recursive)`: Busca arquivos por extensão.
- `find_by_size(dir_path, min_size, max_size, recursive)`: Busca arquivos por tamanho.

### Operações de Leitura e Escrita

- `read_to_string(path)`: Lê um arquivo como string UTF-8.
- `read_to_bytes(path)`: Lê um arquivo como vetor de bytes.
- `write_string(path, content)`: Escreve uma string em um arquivo.
- `write_bytes(path, content)`: Escreve bytes em um arquivo.
- `append_string(path, content)`: Anexa uma string ao final de um arquivo.

### Operações de Diretório

- `create_directory(path)`: Cria um diretório (e diretórios pais, se necessário).
- `remove_file(path)`: Remove um arquivo.
- `remove_directory(path)`: Remove um diretório vazio.
- `remove_directory_recursive(path)`: Remove um diretório e todo seu conteúdo.
- `copy_file(from, to)`: Copia um arquivoAE3 arquivo
- `move_item(from, to)`: Move ou renomeia um arquivo ou diretório.

### Utilitários Convenientes

- `directory_size(path)`: Calcula o tamanho total de um diretório.
- `count_files(path, recursive)`: Conta arquivos em um diretório.
- `count_directories(path, recursive)`: Conta diretórios em um diretório.
- `directory_stats(path)`: Gera estatísticas detalhadas de um diretório.
- `files_to_hashmap(files)`: Converte uma lista de `FileInfo` em um `HashMap` por nome.
- `group_by_extension(files)`: Agrupa arquivos por extensão.

## Funções Auxiliares

- `format_bytes(bytes)`: Formata um valor em bytes para um formato legível (B, KB, MB, GB, TB).
- `matches_pattern(name, pattern)`: Verifica se um nome corresponde a um padrão com wildcards.
- `wildcard_match(text, pattern)`: Implementação interna para correspondência de wildcards.

## Exemplos de Uso

### Verificar Existência de Arquivo

```rust
let utils = FileUtils::new();
if utils.file_exists("config.json") {
    println!("Arquivo de configuração encontrado!");
}
```

### Listar e Processar Arquivos

```rust
let utils = FileUtils::new();
match utils.list_files(".") {
    Ok(arquivos) => {
        for arquivo in arquivos {
            println!("📄 {} ({} bytes)", arquivo.name, arquivo.size);
        }
    }
    Err(e) => eprintln!("Erro ao listar arquivos: {}", e),
}
```

### Busca com Filtro

```rust
let utils = FileUtils::new();
let filter = FileFilter {
    extensions: Some(vec!["jpg".to_string(), "png".to_string()]),
    min_size: Some(1024),
    recursive: true,
    ..Default::default()
};
let imagens = utils.list_with_filter("./imagens", &filter)?;
for imagem in imagens {
    println!("Imagem encontrada: {} ({} bytes)", imagem.name, imagem.size);
}
```

### Leitura e Escrita

```rust
let utils = FileUtils::new();
if let Ok(conteudo) = utils.read_to_string("config.txt") {
    let novo_conteudo = conteudo + "\nNovo dado";
    utils.write_string("config.txt", &novo_conteudo)?;
}
```

## Testes Unitários

A biblioteca inclui testes unitários para validar funcionalidades críticas:

- `test_file_exists`: Verifica a funcionalidade de `file_exists`.
- `test_format_bytes`: Testa a formatação de tamanhos de arquivo.
- `test_wildcard_match`: Valida a correspondência de padrões com wildcards.

Para executar os testes:

```bash
cargo test
```

## Notas de Performance

- **Leitura de Arquivos Grandes**: `read_to_string` e `read_to_bytes` carregam todo o arquivo na memória. Para arquivos grandes, considere usar `std::io::BufReader`.
- **Busca Recursiva**: A busca recursiva pode ser intensiva em diretórios com muitos arquivos. Use filtros para limitar o escopo.
- **Metadados**: Funções como `is_empty` e `directory_size` são otimizadas para usar apenas metadados, minimizando I/O.

## Dependências

A biblioteca utiliza apenas a biblioteca padrão do Rust (`std`), com os seguintes módulos:

- `std::fs`: Operações de sistema de arquivos.
- `std::path`: Manipulação de caminhos.
- `std::io`: Operações de entrada/saída.
- `std::collections`: Estruturas de dados como `HashMap`.
- `std::time`: Manipulação de timestamps.

Nenhuma dependência externa é necessária, garantindo portabilidade e facilidade de integração.