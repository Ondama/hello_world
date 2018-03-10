use std::process::Command;
use std::env;

fn main() {
    let mut image_path = env::var("HOME").expect("no se pudo leer");
    println!("{}", image_path);
    image_path.push_str("/Programming/natgeo-wallpaper/src/img/photonatgeo1.jpg");
    println!("{}", image_path);
    Command::new("gsettings")
            .arg("set")
            .arg("org.gnome.desktop.background")
            .arg("picture-uri")
            .arg(image_path)
            .output()
            .expect("perro fallo");
}
