use std::fs::File;

fn main() {
    let urls = [
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-alertmanagerconfigs.yaml",
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-alertmanagers.yaml",
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-podmonitors.yaml",
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-probes.yaml",
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-prometheuses.yaml",
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-prometheusrules.yaml",
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-servicemonitors.yaml",
        "https://raw.githubusercontent.com/prometheus-community/helm-charts/main/charts/kube-prometheus-stack/crds/crd-thanosrulers.yaml",
    ];

    let mut out = File::create("k8s-crds/src/lib.rs").unwrap();
    k8s_crds_codegen::build_from_urls(&mut out, &urls).unwrap();
}
