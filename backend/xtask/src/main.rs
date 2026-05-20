use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::time::Uptime;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{:?}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .with_timer(Uptime::default())
        .with_file(false)
        .with_line_number(false)
        .init();

    let task = env::args().nth(1);
    match task.as_deref() {
        Some("db:up") => db_up()?,
        Some("db:down") => db_down()?,
        Some("run") => run()?,
        Some("generate-dict") => generate_dict()?,
        Some("generate-api") => generate_api()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

db:up             Start development database
db:down           Stop development database
run               Run server crate
generate-dict     Generate dictionary file
generate-api      Generate OpenAPI specification
    "
    )
}

const DB_NAME: &'static str = "jpquiz-db";

fn db_up() -> Result<()> {
    run_docker(&[
        "run",
        "-e",
        "POSTGRES_USER=postgres",
        "-e",
        "POSTGRES_PASSWORD=password",
        "-p",
        "127.0.0.1:5432:5432",
        "--name",
        DB_NAME,
        "-d",
        "--rm",
        "postgres",
        "-N",
        "1000",
    ])
}

fn db_down() -> Result<()> {
    run_docker(&["stop", DB_NAME])
}

fn run_docker(cmd: &[&str]) -> Result<()> {
    let mut out = Command::new("docker").args(cmd).spawn().context("spawn")?;

    let exit = out.wait()?;

    if !exit.success() {
        bail!("non 0 exit code: {exit}");
    }

    Ok(())
}

fn run() -> Result<()> {
    run_cargo(&["run", "-p", "server"])
}

fn run_cargo(cmd: &[&str]) -> Result<()> {
    let mut out = Command::new("cargo").args(cmd).spawn().context("spawn")?;

    let exit = out.wait()?;

    if !exit.success() {
        bail!("non 0 exit code: {exit}");
    }

    Ok(())
}

fn generate_dict() -> Result<()> {
    let jmdict_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../static/jmdict-eng-common-3.6.2.json");
    let jmdict_rdr = File::open(jmdict_path).context("failed to open JMDict file")?;

    let jlpt_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../static/JLPTWords-1.4.json");
    let jlpt_rdr = File::open(jlpt_path).context("failed to open JLPT file")?;

    let output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../static/dictionary.json");
    let output = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_path)
        .context("failed to open output file")?;

    dictionary::generate_dictionary_file(jmdict_rdr, jlpt_rdr, output)
        .context("generate dictionary file")
}

fn generate_api() -> Result<()> {
    let api_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/openapi.json");
    let mut api_file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(api_path)?;

    let config = server::configuration::get()?;

    let (_router, api) = server::application::router(&config.application).split_for_parts();

    let api_json = api.to_pretty_json()?;

    Ok(api_file.write(api_json.as_bytes()).map(|_| ())?)
}
