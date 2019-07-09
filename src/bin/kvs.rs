use clap::App;
use std::process::exit;

fn main() {
    let yaml = clap::load_yaml!("../../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("set") {
        let key = matches.value_of("KEY").unwrap();
        let value = matches.value_of("VALUE").unwrap();
        eprintln!("unimplemented");
        exit(1)
    } else if let Some(matches) = matches.subcommand_matches("get") {
        let key = matches.value_of("KEY").unwrap();
        eprintln!("unimplemented");
        exit(1)
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        let key = matches.value_of("KEY").unwrap();
        eprintln!("unimplemented");
        exit(1)
    } else if matches.is_present("version") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    } else {
        println!("Error!");
        exit(1);
    }
}
