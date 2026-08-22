use std::env;
use std::process;

// creates a symlink from  ~/.local/bin/suono to target/release/suono
fn main() {
    let binary_path = env::current_dir().unwrap();
    let mut binary_path = binary_path.to_str().unwrap().to_owned();
    binary_path += "/target/release/suono";

    let dot_local_path =
        "/home/".to_owned() + env::var_os("USER").unwrap().to_str().unwrap() + "/.local/bin";

    let export_command = r#"PATH="$PATH:"#.to_owned() + &dot_local_path + r#"""#;

    process::Command::new("sh")
        .arg("-c")
        .arg(&export_command)
        .spawn()
        .expect("failed to export path");

    let symlink_path = dot_local_path + "/suono";

    process::Command::new("ln")
        .args(["-s", &binary_path, &symlink_path])
        .spawn()
        .expect("failed to create symlink");
}
