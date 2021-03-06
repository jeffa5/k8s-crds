pub mod stable_example_com {
    pub mod v1 {
        pub mod cron_tab {
            #[derive(serde::Deserialize, Debug, PartialEq)]
            pub struct CronTab {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
            }

            impl k8s_openapi::Resource for CronTab {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "stable.example.com/v1";
                const GROUP: &'static str = "stable.example.com";
                const KIND: &'static str = "CronTab";
                const VERSION: &'static str = "v1";
                const URL_PATH_SEGMENT: &'static str = "TODO";
            }

            impl k8s_openapi::Metadata for CronTab {
                type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

                fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                    &self.metadata
                }

                fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                    &mut self.metadata
                }
            }

            impl serde::Serialize for CronTab {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use serde::ser::SerializeStruct;
                    let mut state = serializer.serialize_struct("CronTab", 4)?;
                    state.serialize_field(
                        "apiVersion",
                        <Self as k8s_openapi::Resource>::API_VERSION,
                    )?;
                    state.serialize_field("kind", <Self as k8s_openapi::Resource>::KIND)?;
                    state.serialize_field("metadata", &self.metadata)?;
                    state.serialize_field("spec", &self.spec)?;
                    state.end()
                }
            }

            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct Spec {
                pub cron_spec: String,
                pub image: String,
                pub replicas: i64,
            }
        }
    }
}
