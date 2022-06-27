use std::fs::{create_dir, read_dir, remove_dir_all, File};
use std::io::Write;
use std::process::Command;

fn main() {
    let mut names = Vec::new();

    let _ = remove_dir_all("k8s-crds-test/src/generated");
    create_dir("k8s-crds-test/src/generated").unwrap();

    for file in read_dir("k8s-crds-test/resources").unwrap() {
        let file = file.unwrap();
        let path = file.path();
        let name = path.file_stem().unwrap();
        names.push(name.to_string_lossy().into_owned());
    }

    make_lib(&names);

    for name in names {
        let mut f = File::create(format!("k8s-crds-test/src/generated/{name}.rs")).unwrap();

        k8s_crds_codegen::build_from_path(&mut f, format!("k8s-crds-test/resources/{name}.yaml"))
            .unwrap();
    }

    fmt()
}

fn make_lib(names: &[String]) {
    let mut f = File::create("k8s-crds-test/src/generated/generated.rs").unwrap();
    for name in names {
        writeln!(f, "pub mod {name};").unwrap();
    }
}

fn fmt() {
    Command::new("cargo").arg("fmt").status().unwrap();
}
