pub mod argoproj_io {
    pub mod v1alpha1 {
        pub mod app_project {
            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct AppProject {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub status: Status,
            }

            /// JWTToken holds the issuedAt and expiresAt values of a token
            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct ItemsItem {
                pub exp: i64,
                pub iat: i64,
                pub id: String,
            }

            /// JWTTokensByRole contains a list of JWT tokens issued for a given role
            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct JwtTokensByRole {
                /// JWTTokensByRole contains a list of JWT tokens issued for a given role
                pub properties: std::collections::HashMap<String, Value>,
            }

            /// AppProjectStatus contains status information for AppProject CRs
            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct Status {
                /// JWTTokensByRole contains a list of JWT tokens issued for a given role
                pub jwt_tokens_by_role: JwtTokensByRole,
            }

            /// JWTTokens represents a list of JWT tokens
            #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
            #[serde(rename_all = "camelCase")]
            pub struct Value {
                pub items: Vec<ItemsItem>,
            }

            impl k8s_openapi::Resource for AppProject {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "argoproj.io/v1alpha1";
                const GROUP: &'static str = "argoproj.io";
                const KIND: &'static str = "AppProject";
                const VERSION: &'static str = "v1alpha1";
                const URL_PATH_SEGMENT: &'static str = "TODO";
            }

            impl k8s_openapi::Metadata for AppProject {
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
