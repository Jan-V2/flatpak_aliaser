use std::fs::{OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use confy::ConfyError;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Config {
    do_not_alias: Vec<String>,
    special_alias:Vec<(String, String, String)>,
    decapitalize:bool,
    destination_path:String,
}

const APP_NAME: &str = "flatpak_aliaser";
const CONF_NAME: &str = "conf";
const OUT_FILE_NAME: &str = ".flatpak_aliases";

impl Default for  Config{
    fn default()->Self{
        Config{
            do_not_alias: vec!["app.example.org".to_string()],
            special_alias: vec![("app.example.org".to_string(), "example".to_string()
                                 , "--some --options".to_string())],
            decapitalize: true,
            destination_path: get_default_config_path() ,
        }
    }
}

fn get_default_config_path()-> String{
    let mut config_path = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    config_path.push_str("/");
    config_path.push_str(OUT_FILE_NAME);
    config_path
}

fn main() {
/*    let matches = App::new("flatpak aliaser")
        .version("0.0.1")
       // .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("A cool file "))
        .arg(Arg::with_name("num")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("Five less than your favorite number"))
        .get_matches();*/

    let stdout =String::from_utf8( Command::new("flatpak")
        .arg("list")
        .arg("--app")
        .arg("--columns=application")
        .output()
        .expect("flat command failed to start").stdout).unwrap();

    let config_exists = Path::new(&get_default_config_path()).exists();

    let c :Result<Config,ConfyError> = confy::load(APP_NAME,  CONF_NAME);
    let conf:Config;

    if c.is_ok(){
        conf  = c.unwrap();
        if config_exists{
            println!("loaded config from file");
        }else {
            println!("loaded default config")
        }
    }else {
        let err = c.err().unwrap();
        print!("could not load conf ");
        match err {
            ConfyError::BadTomlData(_) => {println!("bad toml data, there might be a typo")}
            ConfyError::DirectoryCreationFailed(_) => {println!("directory creation failed")}
            ConfyError::GeneralLoadError(_) => {println!("general load error")}
            ConfyError::BadConfigDirectory(_) => {println!("config directory not found")}
            ConfyError::SerializeTomlError(_) => {println!("serialisation failed, there might be a typo")}
            ConfyError::WriteConfigurationFileError(_) => {println!("write configuration error")}
            ConfyError::ReadConfigurationFileError(_) => {println!("read configuration error")}
            ConfyError::OpenConfigurationFileError(_) => {println!("open configuration error")}
            #[allow(unreachable_patterns)]
            _ => {println!("unknown error")}
        }
        println!("failed to load conf file, loading defaults");
        conf = Config::default();
    }

    let mut lines:Vec<_> =  stdout.split("\n").map(|e| e.to_string()).collect();
    lines.remove(lines.len()-1);
    let mut aliases:Vec<String> = vec![];
    for line in lines{
        let opt = standard_alias(&line, &conf);
        if opt.is_some(){
            aliases.push(opt.unwrap());
        }
    }

    let mut out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(conf.destination_path)
        .unwrap();

    println!("writing to {}", OUT_FILE_NAME);
    for line in aliases{
        out_file.write(format!("{}\n", line).as_bytes()).unwrap();
    }
    println!("done")
}


fn standard_alias(app_id:&String, conf:&Config)-> Option<String>{
    if conf.do_not_alias.contains(app_id){
         return None
    }
    for special_tuple in &conf.special_alias{
        if special_tuple.0 == app_id.clone(){
            return Some(format!("alias {}='flatpak run {} {}'", special_tuple.1, app_id, special_tuple.2))
        }
    }
    let segments: Vec<_> =  app_id.split(".").map(|e| e.to_string()).collect();
    let mut flatpak_app_id = segments[segments.len()-1].clone();
    if conf.decapitalize{
        flatpak_app_id = flatpak_app_id.to_lowercase();
    }
    return Some(format!("alias {}='flatpak run {}'", flatpak_app_id, app_id))
}