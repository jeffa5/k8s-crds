#[cfg(feature = "cert-manager")]
pub use cert_manager_crds::*;

#[cfg(feature = "kube-prometheus-stack")]
pub use kube_prometheus_stack_crds::*;
