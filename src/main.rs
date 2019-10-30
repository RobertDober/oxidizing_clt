use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    subcommand: String,
    prefix: Option<String>

}
fn main() {
    let args = Cli::from_args();
    match args.subcommand.as_str() {
        "randomize"                => println!("Not yet implemented"),
        "rename-for-unix"          => println!("Coming soon"),
        "mv-with-prefix-and-count" => println!("moving"),
        subcommand                 => panic!("unknown subcommand {}", subcommand)
    }
}
