// Generated by:
// target/debug/k8s-crds-codegen

pub mod argoproj_io {
    pub mod v1alpha1 {
        pub mod event_bus {
            #[derive(serde::Deserialize, Debug, PartialEq)]
            pub struct EventBus {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
                pub status: Status,
            }

            impl k8s_openapi::Resource for EventBus {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "argoproj.io/v1alpha1";
                const GROUP: &'static str = "argoproj.io";
                const KIND: &'static str = "EventBus";
                const VERSION: &'static str = "v1alpha1";
                const URL_PATH_SEGMENT: &'static str = "";
            }

            impl k8s_openapi::Metadata for EventBus {
                type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

                fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                    &self.metadata
                }

                fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                    &mut self.metadata
                }
            }

            impl serde::Serialize for EventBus {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut state = serializer.serialize_struct("EventBus", 5)?;
                    state.serialize_field(
                        "apiVersion",
                        <Self as k8s_openapi::Resource>::API_VERSION,
                    )?;
                    state.serialize_field("kind", <Self as k8s_openapi::Resource>::KIND)?;
                    state.serialize_field("metadata", &self.metadata)?;
                    state.serialize_field("spec", &self.spec)?;
                    state.serialize_field("status", &self.status)?;
                    state.end()
                }
            }

            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct Spec {
                pub properties: serde_json::Map<String, serde_json::Value>,
            }

            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct Status {
                pub properties: serde_json::Map<String, serde_json::Value>,
            }
        }
        pub mod event_source {
            #[derive(serde::Deserialize, Debug, PartialEq)]
            pub struct EventSource {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
                pub status: Status,
            }

            impl k8s_openapi::Resource for EventSource {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "argoproj.io/v1alpha1";
                const GROUP: &'static str = "argoproj.io";
                const KIND: &'static str = "EventSource";
                const VERSION: &'static str = "v1alpha1";
                const URL_PATH_SEGMENT: &'static str = "";
            }

            impl k8s_openapi::Metadata for EventSource {
                type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

                fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                    &self.metadata
                }

                fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                    &mut self.metadata
                }
            }

            impl serde::Serialize for EventSource {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut state = serializer.serialize_struct("EventSource", 5)?;
                    state.serialize_field(
                        "apiVersion",
                        <Self as k8s_openapi::Resource>::API_VERSION,
                    )?;
                    state.serialize_field("kind", <Self as k8s_openapi::Resource>::KIND)?;
                    state.serialize_field("metadata", &self.metadata)?;
                    state.serialize_field("spec", &self.spec)?;
                    state.serialize_field("status", &self.status)?;
                    state.end()
                }
            }

            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct Spec {
                pub properties: serde_json::Map<String, serde_json::Value>,
            }

            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct Status {
                pub properties: serde_json::Map<String, serde_json::Value>,
            }
        }
        pub mod sensor {
            #[derive(serde::Deserialize, Debug, PartialEq)]
            pub struct Sensor {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
                pub status: Status,
            }

            impl k8s_openapi::Resource for Sensor {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "argoproj.io/v1alpha1";
                const GROUP: &'static str = "argoproj.io";
                const KIND: &'static str = "Sensor";
                const VERSION: &'static str = "v1alpha1";
                const URL_PATH_SEGMENT: &'static str = "";
            }

            impl k8s_openapi::Metadata for Sensor {
                type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

                fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                    &self.metadata
                }

                fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                    &mut self.metadata
                }
            }

            impl serde::Serialize for Sensor {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut state = serializer.serialize_struct("Sensor", 5)?;
                    state.serialize_field(
                        "apiVersion",
                        <Self as k8s_openapi::Resource>::API_VERSION,
                    )?;
                    state.serialize_field("kind", <Self as k8s_openapi::Resource>::KIND)?;
                    state.serialize_field("metadata", &self.metadata)?;
                    state.serialize_field("spec", &self.spec)?;
                    state.serialize_field("status", &self.status)?;
                    state.end()
                }
            }

            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct Spec {
                pub properties: serde_json::Map<String, serde_json::Value>,
            }

            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct Status {
                pub properties: serde_json::Map<String, serde_json::Value>,
            }
        }
    }
}
