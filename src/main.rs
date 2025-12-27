extern crate ini;
extern crate keepass;
extern crate newline_converter;

use ini::Ini;
use keepass::{Database, DatabaseKey};
use keepass::db::NodeRef;
use newline_converter::dos2unix;

use std::env;
use std::fs::File;
use std::process;
use std::io::{self, Write};


fn main() -> std::io::Result<()> {
    let stdout = io::stdout();
    let mut out_handle = stdout.lock();
    let stderr = io::stderr();
    let mut err_handle = stderr.lock();

    let args: Vec<_> = env::args_os().collect();

    // Handle version flag
    if args.len() > 1 {
        let arg = args[1].to_str().unwrap();
        if arg == "-V" || arg == "--version" {
            out_handle.write(format!("{}\n", env!("CARGO_PKG_VERSION")).as_bytes()).unwrap();
            out_handle.flush().unwrap();
            process::exit(0);
        }
    }

    if args.len() <= 1 {
        err_handle.write(b"no variable was provided").unwrap();
        err_handle.flush().unwrap();
        process::exit(1);
    }

    let config_path = format!("{}/.summon-keepass.ini", env::var("HOME").unwrap());

    let config = Ini::load_from_file(config_path.as_str()).unwrap();
    let keepass_db = config.section(Some("keepass_db".to_owned())).unwrap();
    let keepass_db_path = keepass_db.get("path").unwrap();
    let keepass_db_pass = keepass_db.get("pass").unwrap();

    let db_path = std::path::Path::new(keepass_db_path);
    let key = DatabaseKey::new().with_password(keepass_db_pass);
    let db = Database::open(&mut File::open(db_path)?, key)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    
    let secret_path = args[1].to_str().unwrap();
    let secret_vec = secret_path.split("|").collect::<Vec<&str>>();

    let field;
    if secret_vec.len() == 2 {
        field = secret_vec[1];
    }
    else if secret_vec.len() == 1 {
        field = "Password";
    }
    else {
        err_handle.write(format!("{} is no valid secret path", secret_path).as_bytes()).unwrap();
        err_handle.flush().unwrap();
        process::exit(2);

    }
    let entry_path = secret_vec[0].split("/").collect::<Vec<&str>>();
    if let Some(NodeRef::Entry(e)) = db.root.get(&entry_path) {
        // Check if the field exists
        if let Some(field_value) = e.get(field) {
            out_handle.write(dos2unix(field_value).as_bytes()).unwrap();
            out_handle.flush().unwrap();
            process::exit(0);
        } else {
            err_handle.write(format!("{} could not be retrieved", secret_path).as_bytes()).unwrap();
            err_handle.flush().unwrap();
            process::exit(1);
        }
    }

    err_handle.write(format!("{} could not be retrieved", secret_path).as_bytes()).unwrap();
    err_handle.flush().unwrap();
    process::exit(1);
}
