pub mod stable_example_com {
    pub mod v1 {
        pub mod cron_tab {
            pub struct CronTab {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub obj1: Obj1,
                pub obj2: Obj2,
            }

            pub struct Obj1A {
                pub b: String,
            }

            pub struct Obj2A {
                pub c: i64,
            }

            pub struct Obj1 {
                pub a: Obj1A,
            }

            pub struct Obj2 {
                pub a: Obj2A,
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
        }
    }
}