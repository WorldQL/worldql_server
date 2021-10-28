use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// PostgreSQL Connection String
    #[clap(short, long = "psql", env = "WQL_POSTGRES_CONNECTION_STRING")]
    postgres_connection_string: String,
}

fn main() {
    let args = Args::parse();
    dbg!(&args);
}
