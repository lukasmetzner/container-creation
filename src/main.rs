use uuid::Uuid;
use std::fs;

mod container;

fn startup_init() {
    // Create root dir for CRP
    fs::create_dir_all("/tmp/crp").unwrap();
    // Create root dir for containers
    fs::create_dir_all("/tmp/crp/container").unwrap();
}

fn main() {
    startup_init();
    let container_name: String = Uuid::new_v4().to_string();
    let container_init_args: String = String::from("ls -alh");

    container::Container::create_container(container_name, container_init_args);
}
