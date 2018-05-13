use std::io::Write;
use std::process::{Command, Stdio};
use std::env;
use std::str;
use std::string::String;
use std::fs;
use std::path::Path;

const IMAGE_FOLDER: &'static str = "/.natgeo-img";
const IMAGE_PATH: &'static str = "/photo-of-the-day.jpg";
const NATGEO_JSON: &'static str = "https://www.nationalgeographic.com/photography/photo-of-the-day/_jcr_content/.gallery.json";
const JQ_ARGUMENT: &'static str = ".items[0].url + .items[0].sizes[\"2048\"]";

fn main() {
    let mut image_path = env::var("HOME").expect("no se pudo leer");
    image_path.push_str(IMAGE_FOLDER);

    if !Path::new(&image_path).exists() {
        fs::create_dir(image_path.to_string()).expect("error creando el directorio"); 
    }

    image_path.push_str(IMAGE_PATH);
    let owned_image_path = image_path.to_string();
    download_image(&owned_image_path);
    println!("guardar en {}", image_path);

    Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri")
            .arg(image_path)
            .output()
            .expect("perro fallo");
    
}

fn download_image(image_path: &str) {
   let cmd_curl = Command::new("curl")
           .args(&["-s", NATGEO_JSON])
           .stdout(Stdio::piped())
           .output()
           .expect("error en curl");

   let mut cmd_jq = Command::new("jq")
           .stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .args(&["-r", JQ_ARGUMENT])
           .spawn()
           .expect("error en jq");
   
   {
       let stdin_jq = cmd_jq.stdin.as_mut().expect("perro");
       stdin_jq.write_all(&cmd_curl.stdout).ok();
   }

   let otro = cmd_jq.wait_with_output().expect("aaaa");
   let output_jq = String::from_utf8_lossy(&otro.stdout.as_slice());
    Command::new("wget")
           .args(&["-O", image_path, &output_jq])
           .output()
           .expect("error en wget");

    println!("output = {}", &output_jq);
}
