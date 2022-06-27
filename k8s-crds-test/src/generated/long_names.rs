pub mod stable_example_com {
    pub mod v1 {
        pub mod cron_tab {
            pub struct CronTab {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub a: A,
            }

            pub struct A {
                pub b: B,
            }

            pub struct B {
                pub c: C,
            }

            pub struct C {
                pub d: D,
                pub f: F,
            }

            pub struct D {
                pub e: i64,
            }

            pub struct F {
                pub g: String,
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
