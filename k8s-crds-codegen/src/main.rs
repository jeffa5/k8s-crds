use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::debug;
use tracing::info;

const CRDS_ROOT: &str = "crds";

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    Crate {
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
        name: "cert-manager".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
        "https://github.com/cert-manager/cert-manager/releases/download/v1.8.2/cert-manager.crds.yaml".to_owned(),
    ],
    }.make()?;

    Crate {
        name: "argo-cd".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
        "https://raw.githubusercontent.com/argoproj/argo-cd/master/manifests/crds/application-crd.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-cd/master/manifests/crds/applicationset-crd.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-cd/master/manifests/crds/appproject-crd.yaml".to_owned(),
    ],
    }.make()?;

    Crate {
        name: "flux".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
        "https://github.com/fluxcd/source-controller/releases/download/v0.25.9/source-controller.crds.yaml".to_owned(),
        "https://github.com/fluxcd/kustomize-controller/releases/download/v0.26.2/kustomize-controller.crds.yaml".to_owned(),
        "https://github.com/fluxcd/helm-controller/releases/download/v0.22.1/helm-controller.crds.yaml".to_owned(),
        "https://github.com/fluxcd/notification-controller/releases/download/v0.24.0/notification-controller.crds.yaml".to_owned(),
        "https://github.com/fluxcd/image-reflector-controller/releases/download/v0.19.2/image-reflector-controller.crds.yaml".to_owned(),
        "https://github.com/fluxcd/image-automation-controller/releases/download/v0.23.4/image-automation-controller.crds.yaml".to_owned(),
    ],
    }.make()?;

    Crate {
        name: "istio".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
        "https://raw.githubusercontent.com/istio/istio/master/manifests/charts/base/crds/crd-all.gen.yaml".to_owned(),
        "https://raw.githubusercontent.com/istio/istio/master/manifests/charts/base/crds/crd-operator.yaml".to_owned(),
        ],
    }.make()?;

    Crate {
        name: "argo-rollouts".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
        "https://raw.githubusercontent.com/argoproj/argo-rollouts/master/manifests/crds/analysis-run-crd.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-rollouts/master/manifests/crds/analysis-template-crd.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-rollouts/master/manifests/crds/cluster-analysis-template-crd.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-rollouts/master/manifests/crds/experiment-crd.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-rollouts/master/manifests/crds/rollout-crd.yaml".to_owned(),
    ],
    }.make()?;

    Crate {
        name: "argo-events".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
        "https://raw.githubusercontent.com/argoproj/argo-events/master/manifests/base/crds/argoproj.io_eventbus.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-events/master/manifests/base/crds/argoproj.io_eventsources.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-events/master/manifests/base/crds/argoproj.io_sensors.yaml".to_owned(),
    ],
    }.make()?;

    Crate {
        name: "argo-workflows".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
        "https://raw.githubusercontent.com/argoproj/argo-workflows/master/manifests/base/crds/full/argoproj.io_clusterworkflowtemplates.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-workflows/master/manifests/base/crds/full/argoproj.io_cronworkflows.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-workflows/master/manifests/base/crds/full/argoproj.io_workfloweventbindings.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-workflows/master/manifests/base/crds/full/argoproj.io_workflows.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-workflows/master/manifests/base/crds/full/argoproj.io_workflowtasksets.yaml".to_owned(),
        "https://raw.githubusercontent.com/argoproj/argo-workflows/master/manifests/base/crds/full/argoproj.io_workflowtemplates.yaml".to_owned(),
    ],
    }.make()?;

    Crate {
        name: "longhorn".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
            "https://raw.githubusercontent.com/longhorn/longhorn/41f1d0a5a69c83b9ff63714cce7385663baef31d/deploy/longhorn.yaml".to_owned(),
        ],
    }
    .make()?;

    Crate {
        name: "zalando-postgres-operator".to_owned(),
        version: "0.1.0".to_owned(),
        urls: vec![
            "https://raw.githubusercontent.com/zalando/postgres-operator/master/manifests/operatorconfiguration.crd.yaml".to_owned(),
            "https://raw.githubusercontent.com/zalando/postgres-operator/master/manifests/postgresql.crd.yaml".to_owned(),
            "https://raw.githubusercontent.com/zalando/postgres-operator/master/manifests/postgresteam.crd.yaml".to_owned(),
        ],
    }
    .make()?;

    Ok(())
}

struct Crate {
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

        Self::write_creator(&mut out)?;

        let paths = self.fetch_resources()?;

        k8s_crds_codegen::build_from_paths(&mut out, paths)?;

        self.fmt();

        Ok(())
    }

    fn fmt(&self) {
        Command::new("cargo")
            .arg("fmt")
            .current_dir(self.crate_root())
            .status()
            .unwrap();
    }

    fn write_creator<W: Write>(w: &mut W) -> anyhow::Result<()> {
        let creator = std::env::args().collect::<Vec<_>>().join(" ");
        writeln!(w, "// Generated by:")?;
        writeln!(w, "// {}", creator)?;
        writeln!(w)?;
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
        let name = url.split('/').last().unwrap();
        let path = resources_dir.join(name);
        if path.exists() {
            debug!(?url, ?path, "Skipping fetch");
            return Ok(path);
        }

        info!(?url, "Fetching resource");
        let bytes = reqwest::blocking::get(url)?.bytes()?.to_vec();

        let mut f = File::create(&path)?;

        writeln!(f, "# fetched from {}", url)?;
        f.write_all(&bytes)?;

        Ok(path)
    }

    fn write_cargo_toml(&self) -> anyhow::Result<()> {
        let mut ct = File::create(self.crate_root().join("Cargo.toml"))?;

        // ensure it doesn't look up for workspaces
        writeln!(ct, "[workspace]")?;
        writeln!(ct, "[package]")?;
        writeln!(ct, "name = \"{}\"", self.package_name())?;
        writeln!(ct, "version = \"{}\"", self.version)?;
        writeln!(ct, "edition = \"2021\"")?;
        writeln!(ct, "authors = [\"Andrew Jeffery <dev@jeffas.io>\"]")?;
        let description = format!("CRDs for the {} deployment", self.name);
        writeln!(ct, "description = \"{}\"", description)?;
        writeln!(ct, "repository = \"https://github.com/jeffa5/k8s-crds/\"")?;
        writeln!(ct, "license = \"MIT\"")?;
        writeln!(
            ct,
            "keywords = [\"kubernetes\", \"k8s\", \"crd\", \"openapi\"]"
        )?;
        writeln!(
            ct,
            "categories = [\"api-bindings\", \"config\", \"virtualization\"]"
        )?;
        writeln!(ct)?;
        writeln!(ct, "[dependencies]")?;
        writeln!(
            ct,
            r#"k8s-openapi = {{ version = "0.19", features = ["v1_26"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1""#
        )?;
        Ok(())
    }

    fn crate_root(&self) -> PathBuf {
        PathBuf::from(CRDS_ROOT).join(format!("{}-crds", self.name))
    }

    fn package_name(&self) -> String {
        format!("{}-crds", self.name)
    }
}
