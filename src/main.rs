use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use rayon::prelude::*;
use walkdir::WalkDir;
use clap::{Arg, Command};

#[derive(Debug, Clone)]
struct LanguageConfig {
    name: String,
    extensions: Vec<String>,
    line_comment: Vec<String>,
    block_comment_start: Vec<String>,
    block_comment_end: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct FileStats {
    files: u64,
    blank_lines: u64,
    comment_lines: u64,
    code_lines: u64,
}

impl std::ops::Add for FileStats {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            files: self.files + other.files,
            blank_lines: self.blank_lines + other.blank_lines,
            comment_lines: self.comment_lines + other.comment_lines,
            code_lines: self.code_lines + other.code_lines,
        }
    }
}

struct LanguageDatabase {
    languages: HashMap<String, LanguageConfig>,
    ext_to_lang: HashMap<String, String>,
}

impl LanguageDatabase {
    fn new() -> Self {
        let mut db = LanguageDatabase {
            languages: HashMap::new(),
            ext_to_lang: HashMap::new(),
        };
        
        db.add_languages();
        db
    }
    
    fn add_language(&mut self, config: LanguageConfig) {
        for ext in &config.extensions {
            self.ext_to_lang.insert(ext.clone(), config.name.clone());
        }
        self.languages.insert(config.name.clone(), config);
    }
    
    fn add_languages(&mut self) {
        // Rust
        self.add_language(LanguageConfig {
            name: "Rust".to_string(),
            extensions: vec!["rs".to_string()],
            line_comment: vec!["//".to_string()],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        // C/C++
        self.add_language(LanguageConfig {
            name: "C/C++".to_string(),
            extensions: vec!["c".to_string(), "cpp".to_string(), "cc".to_string(), "cxx".to_string(), "h".to_string(), "hpp".to_string()],
            line_comment: vec!["//".to_string()],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        // Python
        self.add_language(LanguageConfig {
            name: "Python".to_string(),
            extensions: vec!["py".to_string(), "pyw".to_string()],
            line_comment: vec!["#".to_string()],
            block_comment_start: vec!["\"\"\"".to_string(), "'''".to_string()],
            block_comment_end: vec!["\"\"\"".to_string(), "'''".to_string()],
        });
        
        // JavaScript/TypeScript
        self.add_language(LanguageConfig {
            name: "JavaScript".to_string(),
            extensions: vec!["js".to_string(), "jsx".to_string(), "mjs".to_string()],
            line_comment: vec!["//".to_string()],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        self.add_language(LanguageConfig {
            name: "TypeScript".to_string(),
            extensions: vec!["ts".to_string(), "tsx".to_string()],
            line_comment: vec!["//".to_string()],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        // Java
        self.add_language(LanguageConfig {
            name: "Java".to_string(),
            extensions: vec!["java".to_string()],
            line_comment: vec!["//".to_string()],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        // C#
        self.add_language(LanguageConfig {
            name: "C#".to_string(),
            extensions: vec!["cs".to_string()],
            line_comment: vec!["//".to_string()],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        // Go
        self.add_language(LanguageConfig {
            name: "Go".to_string(),
            extensions: vec!["go".to_string()],
            line_comment: vec!["//".to_string()],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        // Shell scripts
        self.add_language(LanguageConfig {
            name: "Shell".to_string(),
            extensions: vec!["sh".to_string(), "bash".to_string(), "zsh".to_string()],
            line_comment: vec!["#".to_string()],
            block_comment_start: vec![],
            block_comment_end: vec![],
        });
        
        // PowerShell
        self.add_language(LanguageConfig {
            name: "PowerShell".to_string(),
            extensions: vec!["ps1".to_string(), "psm1".to_string(), "psd1".to_string()],
            line_comment: vec!["#".to_string()],
            block_comment_start: vec!["<#".to_string()],
            block_comment_end: vec!["#>".to_string()],
        });
        
        // HTML/XML
        self.add_language(LanguageConfig {
            name: "HTML".to_string(),
            extensions: vec!["html".to_string(), "htm".to_string(), "xml".to_string()],
            line_comment: vec![],
            block_comment_start: vec!["<!--".to_string()],
            block_comment_end: vec!["-->".to_string()],
        });
        
        // CSS
        self.add_language(LanguageConfig {
            name: "CSS".to_string(),
            extensions: vec!["css".to_string()],
            line_comment: vec![],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        // SQL
        self.add_language(LanguageConfig {
            name: "SQL".to_string(),
            extensions: vec!["sql".to_string()],
            line_comment: vec!["--".to_string()],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        // Ruby
        self.add_language(LanguageConfig {
            name: "Ruby".to_string(),
            extensions: vec!["rb".to_string()],
            line_comment: vec!["#".to_string()],
            block_comment_start: vec!["=begin".to_string()],
            block_comment_end: vec!["=end".to_string()],
        });
        
        // PHP
        self.add_language(LanguageConfig {
            name: "PHP".to_string(),
            extensions: vec!["php".to_string()],
            line_comment: vec!["//".to_string(), "#".to_string()],
            block_comment_start: vec!["/*".to_string()],
            block_comment_end: vec!["*/".to_string()],
        });
        
        // YAML/JSON
        self.add_language(LanguageConfig {
            name: "YAML".to_string(),
            extensions: vec!["yaml".to_string(), "yml".to_string()],
            line_comment: vec!["#".to_string()],
            block_comment_start: vec![],
            block_comment_end: vec![],
        });
        
        self.add_language(LanguageConfig {
            name: "JSON".to_string(),
            extensions: vec!["json".to_string()],
            line_comment: vec![],
            block_comment_start: vec![],
            block_comment_end: vec![],
        });
        
        // Markdown
        self.add_language(LanguageConfig {
            name: "Markdown".to_string(),
            extensions: vec!["md".to_string(), "markdown".to_string()],
            line_comment: vec![],
            block_comment_start: vec!["<!--".to_string()],
            block_comment_end: vec!["-->".to_string()],
        });
    }
    
    fn get_language(&self, path: &Path) -> Option<&LanguageConfig> {
        let ext = path.extension()?.to_str()?.to_lowercase();
        let lang_name = self.ext_to_lang.get(&ext)?;
        self.languages.get(lang_name)
    }
}

#[derive(Debug)]
enum LineType {
    Blank,
    Comment,
    Code,
}

struct FileAnalyzer {
    lang_config: LanguageConfig,
}

impl FileAnalyzer {
    fn new(lang_config: LanguageConfig) -> Self {
        Self { lang_config }
    }
    
    fn analyze_file(&self, path: &Path) -> Result<FileStats> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut stats = FileStats {
            files: 1,
            ..Default::default()
        };
        
        let mut in_block_comment = false;
        let mut current_block_end = String::new();
        
        for line_result in reader.lines() {
            let line = line_result?;
            let trimmed = line.trim();
            
            if trimmed.is_empty() {
                stats.blank_lines += 1;
                continue;
            }
            
            let line_type = self.classify_line(trimmed, &mut in_block_comment, &mut current_block_end);
            
            match line_type {
                LineType::Blank => stats.blank_lines += 1,
                LineType::Comment => stats.comment_lines += 1,
                LineType::Code => stats.code_lines += 1,
            }
        }
        
        Ok(stats)
    }
    
    fn classify_line(&self, line: &str, in_block_comment: &mut bool, current_block_end: &mut String) -> LineType {
        let mut remaining = line;
        let mut has_code = false;
        
        loop {
            if *in_block_comment {
                    if let Some(end_pos) = remaining.find(current_block_end.as_str()) {
                        remaining = &remaining[end_pos + current_block_end.len()..];
                        *in_block_comment = false;
                        current_block_end.clear();
                        continue;
                    } else {
                        return if has_code { LineType::Code } else { LineType::Comment };
                    }
                }
            
            // Check for start of block comment
            let mut block_start_pos = None;
            let mut block_start_len = 0;
            let mut matching_end = String::new();
            
            for (i, start) in self.lang_config.block_comment_start.iter().enumerate() {
                if let Some(pos) = remaining.find(start) {
                    if block_start_pos.is_none() || pos < block_start_pos.unwrap() {
                        block_start_pos = Some(pos);
                        block_start_len = start.len();
                        matching_end = self.lang_config.block_comment_end.get(i)
                            .unwrap_or(&String::new()).clone();
                    }
                }
            }
            
            // Check for line comment
            let mut line_comment_pos = None;
            for comment in &self.lang_config.line_comment {
                if let Some(pos) = remaining.find(comment) {
                    if line_comment_pos.is_none() || pos < line_comment_pos.unwrap() {
                        line_comment_pos = Some(pos);
                    }
                }
            }
            
            // Determine what comes first
            match (block_start_pos, line_comment_pos) {
                (Some(block_pos), Some(line_pos)) if block_pos <= line_pos => {
                    // Block comment starts first
                    if block_pos > 0 && !remaining[..block_pos].trim().is_empty() {
                        has_code = true;
                    }
                    remaining = &remaining[block_pos + block_start_len..];
                    *in_block_comment = true;
                    *current_block_end = matching_end;
                }
                (Some(block_pos), None) => {
                    // Only block comment
                    if block_pos > 0 && !remaining[..block_pos].trim().is_empty() {
                        has_code = true;
                    }
                    remaining = &remaining[block_pos + block_start_len..];
                    *in_block_comment = true;
                    *current_block_end = matching_end;
                }
                (_, Some(line_pos)) => {
                    // Line comment (possibly after block comment check)
                    if line_pos > 0 && !remaining[..line_pos].trim().is_empty() {
                        has_code = true;
                    }
                    return if has_code { LineType::Code } else { LineType::Comment };
                }
                (None, None) => {
                    // No comments found
                    if !remaining.trim().is_empty() {
                        has_code = true;
                    }
                    break;
                }
            }
        }
        
        if has_code {
            LineType::Code
        } else if remaining.trim().is_empty() {
            LineType::Blank
        } else {
            LineType::Code
        }
    }
}

fn should_skip_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    
    // Skip common build/cache directories
    let skip_dirs = [
        "target", "node_modules", ".git", ".svn", ".hg", 
        "build", "dist", "out", "bin", "obj", ".vs", ".vscode",
        "__pycache__", ".pytest_cache", ".mypy_cache",
        "vendor", "deps", ".idea", ".gradle"
    ];
    
    for component in path.components() {
        let component_str = component.as_os_str().to_string_lossy().to_lowercase();
        if skip_dirs.contains(&component_str.as_str()) {
            return true;
        }
    }
    
    // Skip hidden files and directories (starting with .)
    if let Some(filename) = path.file_name() {
        let filename_str = filename.to_string_lossy();
        if filename_str.starts_with('.') && filename_str.len() > 1 {
            return true;
        }
    }
    
    false
}

fn collect_files(path: &Path, lang_db: &LanguageDatabase) -> Vec<(PathBuf, LanguageConfig)> {
    let processed_files = Arc::new(AtomicU64::new(0));
    let processed_files_clone = processed_files.clone();
    
    let files: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| !should_skip_path(entry.path()))
        .filter_map(|entry| {
            let count = processed_files_clone.fetch_add(1, Ordering::Relaxed);
            if count % 1000 == 0 {
                eprintln!("Scanned {} files...", count);
            }
            
            let path = entry.path();
            lang_db.get_language(path).map(|lang| (path.to_path_buf(), lang.clone()))
        })
        .collect();
    
    eprintln!("Found {} files to analyze", files.len());
    files
}

fn analyze_files(files: Vec<(PathBuf, LanguageConfig)>) -> HashMap<String, FileStats> {
    let processed = Arc::new(AtomicU64::new(0));
    let total = files.len() as u64;
    
    files
        .into_par_iter()
        .filter_map(|(path, lang_config)| {
            let count = processed.fetch_add(1, Ordering::Relaxed);
            if count % 100 == 0 {
                eprintln!("Analyzed {}/{} files ({:.1}%)", count, total, (count as f64 / total as f64) * 100.0);
            }
            
            let analyzer = FileAnalyzer::new(lang_config.clone());
            match analyzer.analyze_file(&path) {
                Ok(stats) => Some((lang_config.name, stats)),
                Err(_) => None, // Skip files that can't be read
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .fold(HashMap::new(), |mut acc, (lang, stats)| {
            *acc.entry(lang).or_default() = acc.get(&lang).cloned().unwrap_or_default() + stats;
            acc
        })
}

fn print_results(results: HashMap<String, FileStats>) {
    let mut total_stats = FileStats::default();
    let mut sorted_results: Vec<_> = results.iter().collect();
    sorted_results.sort_by(|a, b| b.1.code_lines.cmp(&a.1.code_lines));
    
    println!("{:<20} {:>10} {:>10} {:>10} {:>10}", "Language", "Files", "Blank", "Comment", "Code");
    println!("{}", "-".repeat(70));
    
    for (lang, stats) in &sorted_results {
        println!("{:<20} {:>10} {:>10} {:>10} {:>10}", 
                 lang, stats.files, stats.blank_lines, stats.comment_lines, stats.code_lines);
        total_stats = total_stats.clone() + stats.clone().clone();
    }
    
    println!("{}", "-".repeat(70));
    println!("{:<20} {:>10} {:>10} {:>10} {:>10}", 
             "SUM", total_stats.files, total_stats.blank_lines, total_stats.comment_lines, total_stats.code_lines);
}

fn main() {
    let matches = Command::new("rcloc")
        .version("1.0.0")
        .about("A fast clone of cloc (Count Lines of Code) written in Rust")
        .arg(
            Arg::new("path")
                .help("Directory or file to analyze")
                .value_name("PATH")
                .default_value(".")
                .index(1)
        )
        .arg(
            Arg::new("exclude-dirs")
                .long("exclude-dirs")
                .help("Exclude additional directories (comma-separated)")
                .value_name("DIRS")
        )
        .get_matches();
    
    let path = matches.get_one::<String>("path").unwrap();
    let start_time = Instant::now();
    
    eprintln!("Analyzing directory: {}", path);
    
    let lang_db = LanguageDatabase::new();
    let files = collect_files(Path::new(path), &lang_db);
    
    if files.is_empty() {
        eprintln!("No supported files found!");
        return;
    }
    
    let results = analyze_files(files);
    let duration = start_time.elapsed();
    
    println!();
    print_results(results);
    println!();
    eprintln!("Analysis completed in {:.2} seconds", duration.as_secs_f64());
}