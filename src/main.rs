use uuid::Uuid;
use std::path::Path;
use std::fs;
use tar::Archive;

const UBUNTU_LAYER: &str = "ce1d66bff996cbccd4d52b704e94469aef49119d98c2e7dea6b279166f6789ce";

extern "C" {
    fn start_namespace(container_id: *const u8, init_args: *const u8);
}

struct ContainerArgs
{
  init_args: String,
  container_path: String
}

fn start_namespace_(container_args: ContainerArgs) {
    unsafe {
        start_namespace(
            container_args.container_path.as_bytes().as_ptr(),
            container_args.init_args.as_bytes().as_ptr(),
        );
    }
}

fn main() {
    // Create root dir for containers
    fs::create_dir_all("/tmp/asdf").unwrap();

    // Create container dir
    let path: std::path::PathBuf = Path::new("/tmp/asdf").join(Uuid::new_v4().to_string());
    fs::create_dir(&path).unwrap();
    
    // Unpack ubuntu tar
    let image_path = path.join("ubuntu.tar");
    fs::copy("./images/ubuntu.tar", &image_path).unwrap();
    let image_file = fs::File::open(image_path).unwrap();
    let mut archive = Archive::new(image_file);
    archive.unpack(&path).unwrap();

    // Build final path for chroot in container
    let container_path = path.join(UBUNTU_LAYER);
    let layer_path = container_path.join("layer.tar");
    let layer_file = fs::File::open(&layer_path).unwrap();
    let mut layer_archive = Archive::new(layer_file);
    layer_archive.unpack(&container_path).unwrap();

    println!("{}", container_path.to_str().unwrap());

    let container_args = ContainerArgs {
        container_path: String::from(container_path.to_str().unwrap()),
        init_args: String::from("ls -alh")
    };

    start_namespace_(container_args);
    println!("Hello, world!");
}
