use uuid::Uuid;
use std::path::Path;
use std::fs;
use tar::Archive;

const UBUNTU_LAYER: &str = "ce1d66bff996cbccd4d52b704e94469aef49119d98c2e7dea6b279166f6789ce";


fn start_namespace(id: String, init_args: String) {
    unsafe {
        start_namespace_c(
            id.as_bytes().as_ptr(),
            init_args.as_bytes().as_ptr(),
        );
    }
}


pub struct Container {
    name: String,
    init_args: String
}

impl Container {
    pub fn create_container(name: String, init_args: String) -> Container {
        // Create container dir
        let path: std::path::PathBuf = Path::new("/tmp/crp/container").join(&name);
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

        start_namespace(String::from(container_path.to_str().unwrap()), init_args.clone());

        Container {
            name: name,
            init_args: init_args
        }
    }
}