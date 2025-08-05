// ====================================================================
// FILE_UTILS - BIBLIOTECA DE UTILITÁRIOS DE ARQUIVO EM RUST
// ====================================================================
// Uma biblioteca completa para manipulação de arquivos e diretórios
// Inclui validação, listagem, busca e operações básicas de I/O

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self};
use std::collections::HashMap;

// ====================================================================
// ESTRUTURAS DE DADOS E ENUMS
// ====================================================================

/// Enum para representar diferentes tipos de erro que podem ocorrer
#[derive(Debug, Clone)]
pub enum FileUtilsError {
    /// Arquivo ou diretório não encontrado
    NotFound(String),
    /// Permissão negada para acessar arquivo/diretório
    PermissionDenied(String),
    /// Erro de I/O genérico
    IoError(String),
    /// Extensão de arquivo inválida
    InvalidExtension(String),
    /// Caminho inválido
    InvalidPath(String),
}

impl std::fmt::Display for FileUtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileUtilsError::NotFound(msg) => write!(f, "Não encontrado: {}", msg),
            FileUtilsError::PermissionDenied(msg) => write!(f, "Permissão negada: {}", msg),
            FileUtilsError::IoError(msg) => write!(f, "Erro de I/O: {}", msg),
            FileUtilsError::InvalidExtension(msg) => write!(f, "Extensão inválida: {}", msg),
            FileUtilsError::InvalidPath(msg) => write!(f, "Caminho inválido: {}", msg),
        }
    }
}

impl std::error::Error for FileUtilsError {}

impl From<io::Error> for FileUtilsError {
    fn from(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => FileUtilsError::NotFound(error.to_string()),
            io::ErrorKind::PermissionDenied => FileUtilsError::PermissionDenied(error.to_string()),
            _ => FileUtilsError::IoError(error.to_string()),
        }
    }
}

/// Informações detalhadas sobre um arquivo
#[derive(Debug, Clone)]
pub struct FileInfo {
    /// Caminho completo do arquivo
    pub path: PathBuf,
    /// Nome do arquivo
    pub name: String,
    /// Extensão do arquivo (sem o ponto)
    pub extension: Option<String>,
    /// Tamanho do arquivo em bytes
    pub size: u64,
    /// Se é um diretório
    pub is_directory: bool,
    /// Se é um arquivo
    pub is_file: bool,
    /// Última modificação (timestamp Unix)
    pub modified: Option<u64>,
}

impl FileInfo {
    /// Cria um novo FileInfo a partir de um caminho
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, FileUtilsError> {
        let path = path.as_ref();
        let metadata = fs::metadata(path)?;

        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase());

        let modified = metadata.modified()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs());

        Ok(FileInfo {
            path: path.to_path_buf(),
            name,
            extension,
            size: metadata.len(),
            is_directory: metadata.is_dir(),
            is_file: metadata.is_file(),
            modified,
        })
    }
}

/// Filtros para busca de arquivos
#[derive(Debug, Clone)]
pub struct FileFilter {
    /// Extensões permitidas (None = todas)
    pub extensions: Option<Vec<String>>,
    /// Tamanho mínimo em bytes
    pub min_size: Option<u64>,
    /// Tamanho máximo em bytes
    pub max_size: Option<u64>,
    /// Incluir diretórios
    pub include_directories: bool,
    /// Incluir arquivos
    pub include_files: bool,
    /// Busca recursiva
    pub recursive: bool,
}

impl Default for FileFilter {
    fn default() -> Self {
        Self {
            extensions: None,
            min_size: None,
            max_size: None,
            include_directories: true,
            include_files: true,
            recursive: false,
        }
    }
}

// ====================================================================
// ESTRUTURA PRINCIPAL DA BIBLIOTECA
// ====================================================================

/// Estrutura principal que contém todos os utilitários de arquivo
pub struct FileUtils;

impl FileUtils {
    /// Cria uma nova instância de FileUtils
    pub fn new() -> Self {
        Self
    }

    // ================================================================
    // VALIDAÇÃO DE ARQUIVOS E DIRETÓRIOS
    // ================================================================

    /// Verifica se um arquivo específico existe no sistema de arquivos
    ///
    /// Esta função verifica apenas arquivos, não diretórios. Para verificar
    /// diretórios, use `directory_exists()`. Para verificar qualquer caminho,
    /// use `path_exists()`.
    ///
    /// # Parâmetros
    /// * `path` - O caminho para o arquivo a ser verificado. Pode ser relativo ou absoluto.
    ///
    /// # Retorna
    /// * `true` - Se o caminho existe E é um arquivo
    /// * `false` - Se o caminho não existe OU não é um arquivo (ex: é um diretório)
    ///
    /// # Exemplos
    /// ```rust
    /// use archivus::FileUtils;
    ///
    /// let utils = FileUtils::new();
    ///
    /// // Verifica um arquivo específico
    /// if utils.file_exists("config.json") {
    ///     println!("Arquivo de configuração encontrado!");
    /// }
    ///
    /// // Funciona com caminhos absolutos e relativos
    /// let existe_readme = utils.file_exists("./README.md");
    /// let existe_sistema = utils.file_exists("/etc/hosts");
    ///
    /// println!("README existe: {}", existe_readme);
    /// println!("Hosts existe: {}", existe_sistema);
    /// ```
    ///
    /// # Casos de Uso Comuns
    /// - Verificar se arquivos de configuração existem antes de carregá-los
    /// - Validar entrada do usuário (caminhos de arquivo)
    /// - Verificar dependências antes de executar operações
    pub fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_file()
    }

    /// Verifica se um diretório específico existe no sistema de arquivos
    ///
    /// Esta função verifica apenas diretórios, não arquivos. É útil para
    /// validar se uma pasta existe antes de tentar listar seu conteúdo
    /// ou criar arquivos dentro dela.
    ///
    /// # Parâmetros
    /// * `path` - O caminho para o diretório a ser verificado
    ///
    /// # Retorna
    /// * `true` - Se o caminho existe E é um diretório
    /// * `false` - Se o caminho não existe OU é um arquivo
    ///
    /// # Exemplos
    /// ```rust
    /// use archivus::FileUtils;
    ///
    /// let utils = FileUtils::new();
    ///
    /// // Verifica diretórios do projeto
    /// if utils.directory_exists("src") {
    ///     println!("Diretório fonte encontrado!");
    /// }
    ///
    /// // Verifica antes de criar arquivos
    /// if utils.directory_exists("output") {
    ///     // Diretório existe, pode criar arquivos
    ///     utils.write_string("output/result.txt", "dados")?;
    /// } else {
    ///     // Precisa criar o diretório primeiro
    ///     utils.create_directory("output")?;
    /// }
    /// ```
    ///
    /// # Dica
    /// Combine com `create_directory()` para garantir que um diretório existe:
    /// ```rust
    /// if !utils.directory_exists("backup") {
    ///     utils.create_directory("backup")?;
    /// }
    /// ```
    pub fn directory_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().is_dir()
    }

    /// Verifica se um caminho existe (arquivo ou diretório)
    ///
    /// Esta é a função mais geral para verificar existência. Retorna true
    /// se o caminho existe, independentemente de ser arquivo ou diretório.
    /// Use esta função quando não importa o tipo do item.
    ///
    /// # Parâmetros
    /// * `path` - O caminho a ser verificado
    ///
    /// # Retorna
    /// * `true` - Se o caminho existe (arquivo OU diretório)
    /// * `false` - Se o caminho não existe
    ///
    /// # Exemplos
    /// ```rust
    /// use archivus::FileUtils;
    ///
    /// let utils = FileUtils::new();
    ///
    /// // Verifica qualquer tipo de caminho
    /// let items = vec!["README.md", "src", "config.json", "build"];
    ///
    /// for item in items {
    ///     if utils.path_exists(item) {
    ///         println!("✅ {} existe", item);
    ///     } else {
    ///         println!("❌ {} não encontrado", item);
    ///     }
    /// }
    ///
    /// // Útil para validação geral
    /// let user_path = "/caminho/fornecido/pelo/usuario";
    /// if utils.path_exists(user_path) {
    ///     println!("Caminho válido!");
    /// } else {
    ///     eprintln!("Erro: Caminho '{}' não existe", user_path);
    /// }
    /// ```
    pub fn path_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    /// Valida se um arquivo possui uma extensão específica
    ///
    /// Esta função é útil para filtrar arquivos por tipo antes de processá-los.
    /// A comparação é feita ignorando maiúsculas/minúsculas, então "TXT" é
    /// igual a "txt".
    ///
    /// # Parâmetros
    /// * `path` - O caminho do arquivo a ser verificado
    /// * `extension` - A extensão esperada (sem o ponto inicial)
    ///
    /// # Retorna
    /// * `true` - Se o arquivo tem a extensão especificada
    /// * `false` - Se o arquivo não tem extensão ou tem extensão diferente
    ///
    /// # Exemplos
    /// ```rust
    /// use archivus::FileUtils;
    ///
    /// let utils = FileUtils::new();
    ///
    /// // Verificação simples
    /// if utils.has_extension("documento.pdf", "pdf") {
    ///     println!("É um arquivo PDF!");
    /// }
    ///
    /// // Case insensitive
    /// assert!(utils.has_extension("IMAGEM.JPG", "jpg"));
    /// assert!(utils.has_extension("arquivo.TXT", "txt"));
    ///
    /// // Uso prático: processar apenas arquivos de imagem
    /// let arquivos = vec!["foto.jpg", "documento.txt", "imagem.png"];
    ///
    /// for arquivo in arquivos {
    ///     if utils.has_extension(arquivo, "jpg") || 
    ///        utils.has_extension(arquivo, "png") {
    ///         println!("Processando imagem: {}", arquivo);
    ///     }
    /// }
    /// ```
    ///
    /// # Notas
    /// - A extensão é especificada SEM o ponto (use "txt", não ".txt")
    /// - Funciona com arquivos que têm múltiplos pontos: "arquivo.backup.txt"
    /// - Retorna false para arquivos sem extensão
    pub fn has_extension<P: AsRef<Path>>(&self, path: P, extension: &str) -> bool {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase() == extension.to_lowercase())
            .unwrap_or(false)
    }

    /// Verifica se um arquivo está vazio (tem 0 bytes)
    ///
    /// Esta função é útil para validar arquivos antes de processá-los.
    /// Arquivos vazios podem indicar erro na criação ou podem precisar
    /// de tratamento especial.
    ///
    /// # Parâmetros
    /// * `path` - O caminho do arquivo a ser verificado
    ///
    /// # Retorna
    /// * `Ok(true)` - Se o arquivo existe e está vazio (0 bytes)
    /// * `Ok(false)` - Se o arquivo existe e contém dados
    /// * `Err(FileUtilsError)` - Se houve erro ao acessar o arquivo
    ///
    /// # Erros
    /// Esta função pode retornar erro se:
    /// - O arquivo não existe
    /// - Não há permissão para ler o arquivo
    /// - Erro de I/O do sistema
    ///
    /// # Exemplos
    /// ```rust
    /// use archivus::FileUtils;
    ///
    /// let utils = FileUtils::new();
    ///
    /// // Verificação básica
    /// match utils.is_empty("log.txt") {
    ///     Ok(true) => println!("Arquivo de log está vazio"),
    ///     Ok(false) => println!("Arquivo de log contém dados"),
    ///     Err(e) => eprintln!("Erro ao verificar arquivo: {}", e),
    /// }
    ///
    /// // Uso prático: limpar arquivos vazios
    /// let arquivos = utils.list_files("./temp")?;
    ///
    /// for arquivo in arquivos {
    ///     if utils.is_empty(&arquivo.path)? {
    ///         println!("Removendo arquivo vazio: {}", arquivo.name);
    ///         utils.remove_file(&arquivo.path)?;
    ///     }
    /// }
    ///
    /// // Validação antes de processar
    /// if !utils.is_empty("dados.csv")? {
    ///     // Arquivo tem conteúdo, pode processar
    ///     let dados = utils.read_to_string("dados.csv")?;
    ///     // ... processar dados
    /// } else {
    ///     println!("Aviso: Arquivo de dados está vazio");
    /// }
    /// ```
    ///
    /// # Dica de Performance
    /// Esta função apenas verifica metadados (não lê o arquivo inteiro),
    /// então é muito rápida mesmo para arquivos grandes.
    pub fn is_empty<P: AsRef<Path>>(&self, path: P) -> Result<bool, FileUtilsError> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.len() == 0)
    }

    // ================================================================
    // LISTAGEM DE ARQUIVOS E DIRETÓRIOS
    // ================================================================

    /// Lista todos os arquivos de um diretório (não inclui subdiretórios)
    ///
    /// Esta função retorna informações detalhadas sobre todos os arquivos
    /// encontrados no diretório especificado. Não inclui diretórios nem
    /// faz busca recursiva - apenas o nível atual.
    ///
    /// # Parâmetros
    /// * `dir_path` - O caminho do diretório a ser listado
    ///
    /// # Retorna
    /// * `Ok(Vec<FileInfo>)` - Lista com informações de todos os arquivos
    /// * `Err(FileUtilsError)` - Se houve erro ao acessar o diretório
    ///
    /// # Estrutura FileInfo
    /// Cada arquivo retorna as seguintes informações:
    /// - `path`: Caminho completo do arquivo
    /// - `name`: Nome do arquivo (sem o caminho)
    /// - `extension`: Extensão do arquivo (sem o ponto)
    /// - `size`: Tamanho em bytes
    /// - `modified`: Timestamp da última modificação
    ///
    /// # Exemplos
    /// ```rust
    /// use archivus::FileUtils;
    ///
    /// let utils = FileUtils::new();
    ///
    /// // Listar arquivos do diretório atual
    /// match utils.list_files(".") {
    ///     Ok(arquivos) => {
    ///         println!("Encontrados {} arquivos:", arquivos.len());
    ///         
    ///         for arquivo in arquivos {
    ///             println!("📄 {} ({} bytes)", arquivo.name, arquivo.size);
    ///             
    ///             if let Some(ext) = arquivo.extension {
    ///                 println!("   Extensão: {}", ext);
    ///             }
    ///         }
    ///     }
    ///     Err(e) => eprintln!("Erro ao listar arquivos: {}", e),
    /// }
    ///
    /// // Processar apenas arquivos específicos
    /// let arquivos_src = utils.list_files("src")?;
    ///
    /// for arquivo in arquivos_src {
    ///     match arquivo.extension.as_deref() {
    ///         Some("rs") => println!("Arquivo Rust: {}", arquivo.name),
    ///         Some("toml") => println!("Arquivo TOML: {}", arquivo.name),
    ///         _ => println!("Outro arquivo: {}", arquivo.name),
    ///     }
    /// }
    ///
    /// // Calcular tamanho total dos arquivos
    /// let arquivos = utils.list_files("documents")?;
    /// let tamanho_total: u64 = arquivos.iter().map(|f| f.size).sum();
    /// println!("Tamanho total: {} bytes", tamanho_total);
    /// ```
    ///
    /// # Casos de Uso Comuns
    /// - Inventário de arquivos em um diretório
    /// - Processamento em lote de arquivos
    /// - Cálculo de estatísticas de uso
    /// - Validação de conteúdo de diretórios
    ///
    /// # Veja Também
    /// - `list_directories()` - Para listar apenas diretórios
    /// - `list_all()` - Para listar arquivos E diretórios
    /// - `list_with_filter()` - Para busca com critérios específicos
    pub fn list_files<P: AsRef<Path>>(&self, dir_path: P) -> Result<Vec<FileInfo>, FileUtilsError> {
        let mut files = Vec::new();

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let file_info = FileInfo::new(entry.path())?;

            if file_info.is_file {
                files.push(file_info);
            }
        }

        Ok(files)
    }

    /// Lista todos os diretórios dentro de um diretório (não inclui arquivos)
    ///
    /// Esta função retorna apenas os subdiretórios encontrados no diretório
    /// especificado. Útil para navegação em estruturas de pastas ou para
    /// operações que precisam processar apenas diretórios.
    ///
    /// # Parâmetros
    /// * `dir_path` - O caminho do diretório pai a ser examinado
    ///
    /// # Retorna
    /// * `Ok(Vec<FileInfo>)` - Lista com informações de todos os subdiretórios
    /// * `Err(FileUtilsError)` - Se houve erro ao acessar o diretório
    ///
    /// # Exemplos
    /// ```rust
    /// use archivus::FileUtils;
    ///
    /// let utils = FileUtils::new();
    ///
    /// // Listar subdiretórios do projeto
    /// match utils.list_directories(".") {
    ///     Ok(diretorios) => {
    ///         println!("Subdiretórios encontrados:");
    ///         
    ///         for dir in diretorios {
    ///             println!("📁 {}", dir.name);
    ///             
    ///             // Contar arquivos em cada subdiretório
    ///             match utils.count_files(&dir.path, false) {
    ///                 Ok(count) => println!("   {} arquivos", count),
    ///                 Err(_) => println!("   (não foi possível contar)"),
    ///             }
    ///         }
    ///     }
    ///     Err(e) => eprintln!("Erro: {}", e),
    /// }
    ///
    /// // Processar cada subdiretório
    /// let diretorios = utils.list_directories("projetos")?;
    ///
    /// for diretorio in diretorios {
    ///     println!("Processando: {}", diretorio.name);
    ///     
    ///     // Fazer backup de cada subdiretório
    ///     let backup_name = format!("{}_backup", diretorio.name);
    ///     // ... lógica de backup
    /// }
    ///
    /// // Navegação interativa
    /// fn navegar_diretorios(caminho: &str) -> Result<(), Box<dyn std::error::Error>> {
    ///     let utils = FileUtils::new();
    ///     let dirs = utils.list_directories(caminho)?;
    ///     
    ///     println!("Diretórios em '{}':", caminho);
    ///     for (i, dir) in dirs.iter().enumerate() {
    ///         println!("{}. {}", i + 1, dir.name);
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Casos de Uso Comuns
    /// - Navegação em estruturas de diretórios
    /// - Backup seletivo de pastas
    /// - Análise de organização de projetos
    /// - Criação de menus de navegação
    ///
    /// # Nota Importante
    /// Esta função NÃO é recursiva. Para listar subdiretórios de forma
    /// recursiva, use `list_with_filter()` com `recursive: true`.
    pub fn list_directories<P: AsRef<Path>>(&self, dir_path: P) -> Result<Vec<FileInfo>, FileUtilsError> {
        let mut directories = Vec::new();

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let file_info = FileInfo::new(entry.path())?;

            if file_info.is_directory {
                directories.push(file_info);
            }
        }

        Ok(directories)
    }

    /// Lista todos os itens (arquivos e diretórios) de um diretório
    pub fn list_all<P: AsRef<Path>>(&self, dir_path: P) -> Result<Vec<FileInfo>, FileUtilsError> {
        let mut items = Vec::new();

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let file_info = FileInfo::new(entry.path())?;
            items.push(file_info);
        }

        Ok(items)
    }

    /// Lista arquivos com filtro personalizado
    pub fn list_with_filter<P: AsRef<Path>>(&self, dir_path: P, filter: &FileFilter) -> Result<Vec<FileInfo>, FileUtilsError> {
        if filter.recursive {
            self.list_with_filter_recursive(dir_path, filter)
        } else {
            self.list_with_filter_simple(dir_path, filter)
        }
    }

    // Implementação não-recursiva
    fn list_with_filter_simple<P: AsRef<Path>>(&self, dir_path: P, filter: &FileFilter) -> Result<Vec<FileInfo>, FileUtilsError> {
        let mut filtered_items = Vec::new();

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let file_info = FileInfo::new(entry.path())?;

            if self.matches_filter(&file_info, filter) {
                filtered_items.push(file_info);
            }
        }

        Ok(filtered_items)
    }

    // Implementação recursiva
    fn list_with_filter_recursive<P: AsRef<Path>>(&self, dir_path: P, filter: &FileFilter) -> Result<Vec<FileInfo>, FileUtilsError> {
        let mut filtered_items = Vec::new();

        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let file_info = FileInfo::new(entry.path())?;

            if self.matches_filter(&file_info, filter) {
                filtered_items.push(file_info.clone());
            }

            // Recursão em diretórios
            if file_info.is_directory {
                let mut sub_items = self.list_with_filter_recursive(&file_info.path, filter)?;
                filtered_items.append(&mut sub_items);
            }
        }

        Ok(filtered_items)
    }

    // Verifica se um FileInfo corresponde ao filtro
    fn matches_filter(&self, file_info: &FileInfo, filter: &FileFilter) -> bool {
        // Verifica tipo (arquivo/diretório)
        if file_info.is_file && !filter.include_files {
            return false;
        }
        if file_info.is_directory && !filter.include_directories {
            return false;
        }

        // Verifica extensão
        if let Some(ref allowed_extensions) = filter.extensions {
            if file_info.is_file {
                match &file_info.extension {
                    Some(ext) => {
                        if !allowed_extensions.iter().any(|allowed| allowed.to_lowercase() == ext.to_lowercase()) {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }

        // Verifica tamanho mínimo
        if let Some(min_size) = filter.min_size {
            if file_info.size < min_size {
                return false;
            }
        }

        // Verifica tamanho máximo
        if let Some(max_size) = filter.max_size {
            if file_info.size > max_size {
                return false;
            }
        }

        true
    }

    // ================================================================
    // BUSCA DE ARQUIVOS
    // ================================================================

    /// Busca arquivos por nome (com wildcards simples)
    pub fn find_by_name<P: AsRef<Path>>(&self, dir_path: P, pattern: &str, recursive: bool) -> Result<Vec<FileInfo>, FileUtilsError> {
        let items = if recursive {
            self.list_with_filter(dir_path, &FileFilter {
                recursive: true,
                ..Default::default()
            })?
        } else {
            self.list_all(dir_path)?
        };

        let filtered: Vec<FileInfo> = items.into_iter()
            .filter(|item| self.matches_pattern(&item.name, pattern))
            .collect();

        Ok(filtered)
    }

    /// Busca arquivos por extensão
    pub fn find_by_extension<P: AsRef<Path>>(&self, dir_path: P, extension: &str, recursive: bool) -> Result<Vec<FileInfo>, FileUtilsError> {
        let filter = FileFilter {
            extensions: Some(vec![extension.to_string()]),
            include_directories: false,
            include_files: true,
            recursive,
            ..Default::default()
        };

        self.list_with_filter(dir_path, &filter)
    }

    /// Busca arquivos por tamanho
    pub fn find_by_size<P: AsRef<Path>>(&self, dir_path: P, min_size: Option<u64>, max_size: Option<u64>, recursive: bool) -> Result<Vec<FileInfo>, FileUtilsError> {
        let filter = FileFilter {
            min_size,
            max_size,
            include_directories: false,
            include_files: true,
            recursive,
            ..Default::default()
        };

        self.list_with_filter(dir_path, &filter)
    }

    // Verifica se um nome corresponde a um padrão simples (* e ?)
    fn matches_pattern(&self, name: &str, pattern: &str) -> bool {
        // Implementação simples de wildcard
        // * = qualquer sequência de caracteres
        // ? = qualquer caractere único

        if pattern == "*" {
            return true;
        }

        // Se não tem wildcards, comparação direta
        if !pattern.contains('*') && !pattern.contains('?') {
            return name.to_lowercase() == pattern.to_lowercase();
        }

        // Implementação básica de wildcards
        self.wildcard_match(name, pattern)
    }

    fn wildcard_match(&self, text: &str, pattern: &str) -> bool {
        let text = text.to_lowercase();
        let pattern = pattern.to_lowercase();

        self.wildcard_match_recursive(&text, &pattern, 0, 0)
    }

    fn wildcard_match_recursive(&self, text: &str, pattern: &str, text_idx: usize, pattern_idx: usize) -> bool {
        if pattern_idx == pattern.len() {
            return text_idx == text.len();
        }

        let pattern_chars: Vec<char> = pattern.chars().collect();
        let text_chars: Vec<char> = text.chars().collect();

        if pattern_chars[pattern_idx] == '*' {
            // Tenta todos os possíveis matches para *
            for i in text_idx..=text_chars.len() {
                if self.wildcard_match_recursive(text, pattern, i, pattern_idx + 1) {
                    return true;
                }
            }
            false
        } else if pattern_chars[pattern_idx] == '?' {
            // ? corresponde a exatamente um caractere
            if text_idx < text_chars.len() {
                self.wildcard_match_recursive(text, pattern, text_idx + 1, pattern_idx + 1)
            } else {
                false
            }
        } else {
            // Caractere literal
            if text_idx < text_chars.len() && text_chars[text_idx] == pattern_chars[pattern_idx] {
                self.wildcard_match_recursive(text, pattern, text_idx + 1, pattern_idx + 1)
            } else {
                false
            }
        }
    }

    // ================================================================
    // OPERAÇÕES DE LEITURA E ESCRITA
    // ================================================================

    /// Lê todo o conteúdo de um arquivo como String UTF-8
    ///
    /// Esta é a forma mais simples de ler um arquivo de texto. A função
    /// carrega todo o conteúdo do arquivo para a memória de uma vez.
    /// Use com cuidado para arquivos muito grandes.
    ///
    /// # Parâmetros
    /// * `path` - O caminho do arquivo a ser lido
    ///
    /// # Retorna
    /// * `Ok(String)` - O conteúdo completo do arquivo como String
    /// * `Err(FileUtilsError)` - Se houve erro ao ler o arquivo
    ///
    /// # Erros Possíveis
    /// - Arquivo não existe
    /// - Sem permissão de leitura
    /// - Arquivo não é UTF-8 válido
    /// - Erro de I/O do sistema
    /// - Arquivo muito grande para a memória disponível
    ///
    /// # Exemplos
    /// ```rust
    /// use archivus::FileUtils;
    ///
    /// let utils = FileUtils::new();
    ///
    /// // Leitura básica de arquivo
    /// match utils.read_to_string("config.json") {
    ///     Ok(conteudo) => {
    ///         println!("Arquivo lido com sucesso!");
    ///         println!("Tamanho: {} caracteres", conteudo.len());
    ///         
    ///         // Processar o conteúdo
    ///         for linha in conteudo.lines() {
    ///             println!("Linha: {}", linha);
    ///         }
    ///     }
    ///     Err(e) => eprintln!("Erro ao ler arquivo: {}", e),
    /// }
    ///
    /// // Leitura com processamento de erro específico
    /// fn ler_configuracao() -> Result<String, Box<dyn std::error::Error>> {
    ///     let utils = FileUtils::new();
    ///     
    ///     if !utils.file_exists("config.txt") {
    ///         return Err("Arquivo de configuração não encontrado".into());
    ///     }
    ///     
    ///     if utils.is_empty("config.txt")? {
    ///         return Err("Arquivo de configuração está vazio".into());
    ///     }
    ///     
    ///     let conteudo = utils.read_to_string("config.txt")?;
    ///     Ok(conteudo)
    /// }
    ///
    /// // Processamento de múltiplos arquivos
    /// let arquivos = vec!["dados1.txt", "dados2.txt", "dados3.txt"];
    /// let mut conteudo_completo = String::new();
    ///
    /// for arquivo in arquivos {
    ///     match utils.read_to_string(arquivo) {
    ///         Ok(conteudo) => {
    ///             conteudo_completo.push_str(&conteudo);
    ///             conteudo_completo.push('\n');
    ///         }
    ///         Err(e) => eprintln!("Aviso: Não foi possível ler {}: {}", arquivo, e),
    ///     }
    /// }
    /// ```
    ///
    /// # Limitações e Considerações
    /// - **Memória**: Carrega todo o arquivo para RAM - cuidado com arquivos grandes
    /// - **Encoding**: Assume que o arquivo é UTF-8 válido
    /// - **Performance**: Para arquivos grandes, considere leitura em chunks
    ///
    /// # Alternativas
    /// - `read_to_bytes()` - Para arquivos binários ou não-UTF-8
    /// - Para arquivos grandes, considere usar `std::io::BufReader` diretamente
    ///
    /// # Casos de Uso Comuns
    /// - Leitura de arquivos de configuração (JSON, TOML, XML)
    /// - Processamento de arquivos de texto pequenos/médios
    /// - Leitura de templates HTML/CSS
    /// - Carregamento de dados CSV simples
    pub fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String, FileUtilsError> {
        Ok(fs::read_to_string(path)?)
    }

    /// Lê todo o conteúdo de um arquivo como array de bytes
    ///
    /// Esta função é ideal para arquivos binários ou quando você precisa
    /// do controle total sobre os dados. Diferente de `read_to_string()`,
    /// não faz nenhuma validação de encoding UTF-8.
    ///
    /// # Parâmetros
    /// * `path` - O caminho do arquivo a ser lido
    ///
    /// # Retorna
    /// * `Ok(Vec<u8>)` - Os bytes do arquivo
    /// * `Err(FileUtilsError)` - Se houve erro ao ler o arquivo
    ///
    /// # Exemplos
    /// ```rust
    /// use archivus::FileUtils;
    ///
    /// let utils = FileUtils::new();
    ///
    /// // Leitura de arquivo binário (imagem)
    /// match utils.read_to_bytes("logo.png") {
    ///     Ok(bytes) => {
    ///         println!("Imagem carregada: {} bytes", bytes.len());
    ///         
    ///         // Verificar assinatura PNG
    ///         if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
    ///             println!("✅ Arquivo PNG válido");
    ///         }
    ///     }
    ///     Err(e) => eprintln!("Erro: {}", e),
    /// }
    ///
    /// // Processar arquivo de dados binário
    /// let dados = utils.read_to_bytes("dados.bin")?;
    ///
    /// // Converter para diferentes tipos
    /// let como_string = String::from_utf8_lossy(&dados);
    /// let primeiro_u32 = u32::from_le_bytes([dados[0], dados[1], dados[2], dados[3]]);
    ///
    /// // Análise de arquivo
    /// fn analisar_arquivo(caminho: &str) -> Result<(), Box<dyn std::error::Error>> {
    ///     let utils = FileUtils::new();
    ///     let bytes = utils.read_to_bytes(caminho)?;
    ///     
    ///     println!("Análise do arquivo '{}':", caminho);
    ///     println!("- Tamanho: {} bytes", bytes.len());
    ///     
    ///     if bytes.is_empty() {
    ///         println!("- Arquivo vazio");
    ///         return Ok(());
    ///     }
    ///     
    ///     // Verificar se é texto ou binário
    ///     let mut texto_valido = 0;
    ///     let mut controle = 0;
    ///     
    ///     for &byte in bytes.iter().take(1000) { // Amostra dos primeiros 1000 bytes
    ///         if byte.is_ascii_graphic() || byte.is_ascii_whitespace() {
    ///             texto_valido += 1;
    ///         } else if byte < 32 && byte != 9 && byte != 10 && byte != 13 {
    ///             controle += 1;
    ///         }
    ///     }
    ///     
    ///     if controle > texto_valido / 10 {
    ///         println!("- Tipo: Binário");
    ///     } else {
    ///         println!("- Tipo: Texto");
    ///     }
    ///     
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Casos de Uso Comuns
    /// - Leitura de imagens, vídeos, áudio
    /// - Processamento de arquivos executáveis
    /// - Leitura de arquivos com encoding específico
    /// - Análise forense de arquivos
    /// - Implementação de parsers binários
    ///
    /// # Vantagens sobre read_to_string()
    /// - Funciona com qualquer tipo de arquivo
    /// - Não falha com dados não-UTF-8
    /// - Preserva dados binários exatos
    /// - Útil para verificação de checksums
    pub fn read_to_bytes<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, FileUtilsError> {
        Ok(fs::read(path)?)
    }

    /// Escreve uma string para um arquivo
    pub fn write_string<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<(), FileUtilsError> {
        Ok(fs::write(path, content)?)
    }

    /// Escreve bytes para um arquivo
    pub fn write_bytes<P: AsRef<Path>>(&self, path: P, content: &[u8]) -> Result<(), FileUtilsError> {
        Ok(fs::write(path, content)?)
    }

    /// Anexa conteúdo ao final de um arquivo
    pub fn append_string<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<(), FileUtilsError> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }

    // ================================================================
    // OPERAÇÕES DE DIRETÓRIO
    // ================================================================

    /// Cria um diretório (e todos os diretórios pais se necessário)
    pub fn create_directory<P: AsRef<Path>>(&self, path: P) -> Result<(), FileUtilsError> {
        Ok(fs::create_dir_all(path)?)
    }

    /// Remove um arquivo
    pub fn remove_file<P: AsRef<Path>>(&self, path: P) -> Result<(), FileUtilsError> {
        Ok(fs::remove_file(path)?)
    }

    /// Remove um diretório (deve estar vazio)
    pub fn remove_directory<P: AsRef<Path>>(&self, path: P) -> Result<(), FileUtilsError> {
        Ok(fs::remove_dir(path)?)
    }

    /// Remove um diretório e todo seu conteúdo recursivamente
    pub fn remove_directory_recursive<P: AsRef<Path>>(&self, path: P) -> Result<(), FileUtilsError> {
        Ok(fs::remove_dir_all(path)?)
    }

    /// Copia um arquivo
    pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<u64, FileUtilsError> {
        Ok(fs::copy(from, to)?)
    }

    /// Move/renomeia um arquivo ou diretório
    pub fn move_item<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<(), FileUtilsError> {
        Ok(fs::rename(from, to)?)
    }

    // ================================================================
    // UTILITÁRIOS CONVENIENTES
    // ================================================================

    /// Obtém o tamanho total de um diretório (recursivamente)
    pub fn directory_size<P: AsRef<Path>>(&self, path: P) -> Result<u64, FileUtilsError> {
        let files = self.list_with_filter(path, &FileFilter {
            include_files: true,
            include_directories: false,
            recursive: true,
            ..Default::default()
        })?;

        Ok(files.iter().map(|f| f.size).sum())
    }

    /// Conta quantos arquivos existem em um diretório
    pub fn count_files<P: AsRef<Path>>(&self, path: P, recursive: bool) -> Result<usize, FileUtilsError> {
        let files = self.list_with_filter(path, &FileFilter {
            include_files: true,
            include_directories: false,
            recursive,
            ..Default::default()
        })?;

        Ok(files.len())
    }

    /// Conta quantos diretórios existem em um diretório
    pub fn count_directories<P: AsRef<Path>>(&self, path: P, recursive: bool) -> Result<usize, FileUtilsError> {
        let directories = self.list_with_filter(path, &FileFilter {
            include_files: false,
            include_directories: true,
            recursive,
            ..Default::default()
        })?;

        Ok(directories.len())
    }

    /// Obtém estatísticas de um diretório
    pub fn directory_stats<P: AsRef<Path>>(&self, path: P) -> Result<DirectoryStats, FileUtilsError> {
        let all_items = self.list_with_filter(path, &FileFilter {
            recursive: true,
            ..Default::default()
        })?;

        let mut stats = DirectoryStats::default();

        for item in all_items {
            if item.is_file {
                stats.file_count += 1;
                stats.total_size += item.size;

                if let Some(ext) = item.extension {
                    *stats.extensions.entry(ext).or_insert(0) += 1;
                }

                if item.size > stats.largest_file_size {
                    stats.largest_file_size = item.size;
                    stats.largest_file_name = Some(item.name);
                }
            } else {
                stats.directory_count += 1;
            }
        }

        Ok(stats)
    }

    /// Converte um vetor de FileInfo em um HashMap para acesso rápido
    pub fn files_to_hashmap(&self, files: Vec<FileInfo>) -> HashMap<String, FileInfo> {
        files.into_iter()
            .map(|file| (file.name.clone(), file))
            .collect()
    }

    /// Agrupa arquivos por extensão
    pub fn group_by_extension(&self, files: Vec<FileInfo>) -> HashMap<String, Vec<FileInfo>> {
        let mut groups: HashMap<String, Vec<FileInfo>> = HashMap::new();

        for file in files {
            let ext = file.extension.clone().unwrap_or_else(|| "sem_extensao".to_string());
            groups.entry(ext).or_insert_with(Vec::new).push(file);
        }

        groups
    }
}

// ================================================================
// ESTRUTURAS AUXILIARES
// ================================================================

/// Estatísticas de um diretório
#[derive(Debug, Default)]
pub struct DirectoryStats {
    /// Número de arquivos
    pub file_count: usize,
    /// Número de diretórios
    pub directory_count: usize,
    /// Tamanho total em bytes
    pub total_size: u64,
    /// Contagem por extensão
    pub extensions: HashMap<String, usize>,
    /// Maior arquivo (tamanho)
    pub largest_file_size: u64,
    /// Nome do maior arquivo
    pub largest_file_name: Option<String>,
}

impl DirectoryStats {
    /// Formata o tamanho total de forma legível
    pub fn formatted_size(&self) -> String {
        format_bytes(self.total_size)
    }

    /// Formata o tamanho do maior arquivo
    pub fn formatted_largest_file_size(&self) -> String {
        format_bytes(self.largest_file_size)
    }
}

// ================================================================
// FUNÇÕES UTILITÁRIAS
// ================================================================

/// Formata bytes em formato legível (KB, MB, GB, etc.)
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];

    if bytes == 0 {
        return "0 B".to_string();
    }

    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

// ================================================================
// IMPLEMENTAÇÃO DEFAULT
// ================================================================

impl Default for FileUtils {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================
// TESTES UNITÁRIOS
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_file_exists() {
        let utils = FileUtils::new();

        // Cria um arquivo temporário para teste
        let test_file = "test_file.txt";
        fs::write(test_file, "conteúdo de teste").unwrap();

        assert!(utils.file_exists(test_file));
        assert!(!utils.file_exists("arquivo_inexistente.txt"));

        // Limpa
        fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1536), "1.50 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
    }

    #[test]
    fn test_wildcard_match() {
        let utils = FileUtils::new();

        assert!(utils.matches_pattern("test.txt", "*.txt"));
        assert!(utils.matches_pattern("arquivo.rs", "*.rs"));
        assert!(utils.matches_pattern("test", "t?st"));
        assert!(!utils.matches_pattern("test.txt", "*.rs"));
    }
}