use std::process::Command; // for executing commands

fn check_fs(path: String) -> bool {
    if path == "/usr" {
        let tmp = Command::new("lsattr").arg("/usr")
            .output()
            .expect("failed to execute process");

        let tmp = String::from_utf8_lossy(&tmp.stdout);
        let tmp = tmp.trim();
        let tmp = tmp.split_whitespace().collect::<Vec<&str>>();
        let tmp = tmp[0];
        let tmp = tmp.to_string();

        return tmp.contains("i");
    }

    let true_path = Command::new("ls").arg("-l").arg(path).output().unwrap();
    let true_path = String::from_utf8_lossy(&true_path.stdout);
    let true_path = true_path.trim();
    let true_path = true_path.split_whitespace().collect::<Vec<&str>>();
    let true_path = true_path[10];
    let true_path = true_path.to_string();

    let true_path = "/".to_owned() + &true_path;

    let tmp = Command::new("lsattr").arg(true_path)
        .output()
        .expect("failed to execute process");

    let tmp = String::from_utf8_lossy(&tmp.stdout);
    let tmp = tmp.trim();
    let tmp = tmp.split_whitespace().collect::<Vec<&str>>();
    let tmp = tmp[0];
    let tmp = tmp.to_string();

    return tmp.contains("i");
}

pub(crate) fn getmode() -> bool {
    let usr = check_fs("/usr".to_string());
    let sbin = check_fs("/sbin".to_string());
    let bin = check_fs("/bin".to_string());
    let lib = check_fs("/lib".to_string());

    return usr && sbin && bin && lib;
}



pub(crate) fn enterro() {

}

pub(crate) fn enterrw() {

}