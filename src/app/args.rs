#[derive(Debug, StructOpt)]
struct Args {
    src: PathBuf,
    dst: PathBuf,

    #[structopt(short, long)]
    watch: bool,
}
