use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let root = "crds";

    Crate {
        root: root.to_owned(),
        name: "kube-prometheus-stack".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-alertmanagerconfigs.yaml".to_owned(),
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-alertmanagers.yaml".to_owned(),
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-podmonitors.yaml".to_owned(),
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-probes.yaml".to_owned(),
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-prometheuses.yaml".to_owned(),
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-prometheusrules.yaml".to_owned(),
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-servicemonitors.yaml".to_owned(),
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-thanosrulers.yaml".to_owned(),
    ],
    }.make()?;

    Crate {
        root: root.to_owned(),
        name: "cert-manager".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
        "https://github.com/cert-manager/cert-manager/releases/download/v1.8.2/cert-manager.crds.yaml".to_owned(),
    ],
    }.make()?;

    fmt();

    Ok(())
}

struct Crate {
    root: String,
    name: String,
    version: String,
    urls: Vec<String>,
}

impl Crate {
    fn make(&self) -> anyhow::Result<()> {
        create_dir_all(self.crate_root())?;

        self.write_cargo_toml()?;

        create_dir_all(self.crate_root().join("src"))?;
        let mut out = File::create(self.crate_root().join("src").join("lib.rs"))?;

        let paths = self.fetch_resources()?;

        k8s_crds_codegen::build_from_paths(&mut out, paths).unwrap();

        Ok(())
    }

    fn fetch_resources(&self) -> anyhow::Result<Vec<PathBuf>> {
        let resources_dir = self.crate_root().join("resources");
        create_dir_all(&resources_dir)?;
        let mut paths = Vec::new();
        for url in &self.urls {
            let path = self.fetch_resource(url, &resources_dir)?;
            paths.push(path);
        }
        Ok(paths)
    }

    fn fetch_resource(&self, url: &str, resources_dir: &Path) -> anyhow::Result<PathBuf> {
        let bytes = reqwest::blocking::get(url)?.bytes()?.to_vec();

        let name = url.split('/').last().unwrap();
        let path = resources_dir.join(name);

        let mut f = File::create(&path)?;

        writeln!(f, "# fetched from {}", url)?;
        f.write_all(&bytes)?;

        Ok(path)
    }

    fn write_cargo_toml(&self) -> anyhow::Result<()> {
        let mut ct = File::create(self.crate_root().join("Cargo.toml"))?;

        writeln!(ct, "[package]")?;
        writeln!(ct, "name = \"{}\"", self.package_name())?;
        writeln!(ct, "version = \"{}\"", self.version)?;
        writeln!(ct, "edition = \"2021\"")?;
        writeln!(ct)?;
        writeln!(ct, "[dependencies]")?;
        writeln!(
            ct,
            "k8s-openapi = {{ version = \"0.15.0\", features = [\"v1_20\"] }}"
        )?;
        Ok(())
    }

    fn crate_root(&self) -> PathBuf {
        PathBuf::from(&self.root).join(format!("{}-crds", self.name))
    }

    fn package_name(&self) -> String {
        format!("{}-crds", self.name)
    }
}

fn fmt() {
    Command::new("cargo").arg("fmt").status().unwrap();
}
