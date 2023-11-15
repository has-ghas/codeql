use clap::Args;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};

use codeql_extractor::{diagnostics, extractor, file_paths, node_types, trap};

#[derive(Args)]
pub struct Options {
    /// Sets a custom source achive folder
    #[arg(long)]
    source_archive_dir: String,

    /// Sets a custom trap folder
    #[arg(long)]
    output_dir: String,

    /// A text file containing the paths of the files to extract
    #[arg(long)]
    file_list: String,
}

pub fn run(options: Options) -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_level(true)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("php_extractor=warn")),
        )
        .init();
    let diagnostics = diagnostics::DiagnosticLoggers::new("php");
    let mut main_thread_logger = diagnostics.logger();
    let num_threads = match codeql_extractor::options::num_threads() {
        Ok(num) => num,
        Err(e) => {
            main_thread_logger.write(
                main_thread_logger
                    .new_entry("configuration-error", "Configuration error")
                    .message(
                        "{}; defaulting to 1 thread.",
                        &[diagnostics::MessageArg::Code(&e)],
                    )
                    .severity(diagnostics::Severity::Warning),
            );
            1
        }
    };
    tracing::info!(
        "Using {} {}",
        num_threads,
        if num_threads == 1 {
            "thread"
        } else {
            "threads"
        }
    );
    let trap_compression =
        match trap::Compression::from_env("CODEQL_EXTRACTOR_PHP_OPTION_TRAP_COMPRESSION") {
            Ok(x) => x,
            Err(e) => {
                main_thread_logger.write(
                    main_thread_logger
                        .new_entry("configuration-error", "Configuration error")
                        .message("{}; using gzip.", &[diagnostics::MessageArg::Code(&e)])
                        .severity(diagnostics::Severity::Warning),
                );
                trap::Compression::Gzip
            }
        };
    drop(main_thread_logger);
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let src_archive_dir = file_paths::path_from_string(&options.source_archive_dir);

    let trap_dir = file_paths::path_from_string(&options.output_dir);

    let file_list = fs::File::open(file_paths::path_from_string(&options.file_list))?;

    let language = tree_sitter_php::language();
    let schema = node_types::read_node_types_str("php", tree_sitter_php::NODE_TYPES)?;
    let lines: std::io::Result<Vec<String>> = std::io::BufReader::new(file_list).lines().collect();
    let lines = lines?;
    lines
        .par_iter()
        .try_for_each(|line| {
            let mut diagnostics_writer = diagnostics.logger();
            let path = PathBuf::from(line).canonicalize()?;
            let src_archive_file = file_paths::path_for(&src_archive_dir, &path, "");
            let source = std::fs::read(&path)?;
            let needs_conversion = false;
            let code_ranges;
            let mut trap_writer = trap::Writer::new();
            code_ranges = vec![];
            extractor::extract(
                language,
                "php",
                &schema,
                &mut diagnostics_writer,
                &mut trap_writer,
                &path,
                &source,
                &code_ranges,
            );
            std::fs::create_dir_all(src_archive_file.parent().unwrap())?;
            if needs_conversion {
                std::fs::write(&src_archive_file, &source)?;
            } else {
                std::fs::copy(&path, &src_archive_file)?;
            }
            write_trap(&trap_dir, path, &trap_writer, trap_compression)
        })
        .expect("failed to extract files");

    let path = PathBuf::from("extras");
    let mut trap_writer = trap::Writer::new();
    extractor::populate_empty_location(&mut trap_writer);
    write_trap(&trap_dir, path, &trap_writer, trap_compression)
}

lazy_static! {
    static ref CP_NUMBER: regex::Regex = regex::Regex::new("cp([0-9]+)").unwrap();
}

fn write_trap(
    trap_dir: &Path,
    path: PathBuf,
    trap_writer: &trap::Writer,
    trap_compression: trap::Compression,
) -> std::io::Result<()> {
    let trap_file = file_paths::path_for(trap_dir, &path, trap_compression.extension());
    std::fs::create_dir_all(trap_file.parent().unwrap())?;
    trap_writer.write_to_file(&trap_file, trap_compression)
}
