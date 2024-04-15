use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    library: Vec<LibraryConfig>,
}

#[derive(Deserialize, Debug)]
pub struct LibraryConfig {
    name: String,
    pre_install_script: String,
    install_script: String,
    post_install_script: String,
}

pub fn install(data: Data) {
    println!("{:?}", data);
}
