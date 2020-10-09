mod jwt_operation;

use std::{env,fs};
use yaml_rust::YamlLoader;
use std::process::exit;

fn main() {
    let args : Vec<String> = env::args().collect();
    let path_yaml_file = &args[1];
    let yaml_content = fs::read_to_string(path_yaml_file).unwrap();

    let jwt_informations = YamlLoader::load_from_str(&yaml_content).unwrap();

    let jwt_information = &jwt_informations[0];

    for base in jwt_information["jwt"].to_owned() {
        let sub = base["sub"].as_str().unwrap();
        let keys = base["key"].to_owned().into_vec().unwrap();
        let roles_yaml_format = base["roles"].to_owned().into_vec().unwrap();
        let mut roles: Vec<String> = vec![];

        for role_yaml_format in roles_yaml_format {
            let role = role_yaml_format.as_str().unwrap();
            roles.push(role.to_owned())
        }
        println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("+++++++++++++++++++++++++{}+++++++++++++++++++++++++++++++", sub);
        for key in keys  {
            let signing_key = key.as_str().unwrap().as_bytes();
            let jwt_signed_key = jwt_operation::generate_key(&signing_key, sub.to_string(), &roles);

            println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
            println!("{}", jwt_signed_key);
        }
    }
}