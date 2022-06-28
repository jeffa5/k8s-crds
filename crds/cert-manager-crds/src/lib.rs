pub mod acme_cert_manager_io {
    pub mod v1 {
        pub mod challenge {
            /// Challenge is a type to represent a Challenge request with an ACME server
            pub struct Challenge {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
                pub status: Status,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct AccessTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct AccountSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Use the 'ACME DNS' (https://github.com/joohoi/acme-dns) API to manage DNS01 challenge records.
            pub struct AcmeDNS {
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub account_secret_ref: AccountSecretRef,
                pub host: String,
            }

            /// If specified, the pod's scheduling constraints
            pub struct Affinity {
                /// Describes node affinity scheduling rules for the pod.
                pub node_affinity: NodeAffinity,
                /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
                pub pod_affinity: PodAffinity,
                /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
                pub pod_anti_affinity: PodAntiAffinity,
            }

            /// Use the Akamai DNS zone management API to manage DNS01 challenge records.
            pub struct Akamai {
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub access_token_secret_ref: AccessTokenSecretRef,
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub client_secret_secret_ref: AkamaiClientSecretSecretRef,
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub client_token_secret_ref: ClientTokenSecretRef,
                pub service_consumer_domain: String,
            }

            /// Annotations that should be added to the created ACME HTTP01 solver ingress.
            pub struct IngressTemplateMetadataAnnotations {
                /// Annotations that should be added to the created ACME HTTP01 solver ingress.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// Annotations that should be added to the create ACME HTTP01 solver pods.
            pub struct PodTemplateMetadataAnnotations {
                /// Annotations that should be added to the create ACME HTTP01 solver pods.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// API key to use to authenticate with Cloudflare. Note: using an API token to authenticate is now the recommended method as it allows greater control of permissions.
            pub struct ApiKeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// API token used to authenticate with Cloudflare.
            pub struct ApiTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Use the Microsoft Azure DNS API to manage DNS01 challenge records.
            pub struct AzureDNS {
                /// if both this and ClientSecret are left unset MSI will be used
                pub client_i_d: String,
                /// if both this and ClientID are left unset MSI will be used
                pub client_secret_secret_ref: AzureDNSClientSecretSecretRef,
                /// name of the Azure environment (default AzurePublicCloud)
                pub environment: String,
                /// name of the DNS zone that should be used
                pub hosted_zone_name: String,
                /// managed identity configuration, can not be used at the same time as clientID, clientSecretSecretRef or tenantID
                pub managed_identity: ManagedIdentity,
                /// resource group the DNS zone is located in
                pub resource_group_name: String,
                /// ID of the Azure subscription
                pub subscription_i_d: String,
                /// when specifying ClientID and ClientSecret then this field is also needed
                pub tenant_i_d: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct AkamaiClientSecretSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// if both this and ClientID are left unset MSI will be used
            pub struct AzureDNSClientSecretSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct ClientTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Use the Google Cloud DNS API to manage DNS01 challenge records.
            pub struct CloudDNS {
                /// HostedZoneName is an optional field that tells cert-manager in which Cloud DNS zone the challenge record has to be created. If left empty cert-manager will automatically choose a zone.
                pub hosted_zone_name: String,
                pub project: String,
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub service_account_secret_ref: ServiceAccountSecretRef,
            }

            /// Use the Cloudflare API to manage DNS01 challenge records.
            pub struct Cloudflare {
                /// API key to use to authenticate with Cloudflare. Note: using an API token to authenticate is now the recommended method as it allows greater control of permissions.
                pub api_key_secret_ref: ApiKeySecretRef,
                /// API token used to authenticate with Cloudflare.
                pub api_token_secret_ref: ApiTokenSecretRef,
                /// Email of the account, only required when using API key based authentication.
                pub email: String,
            }

            /// Use the DigitalOcean DNS API to manage DNS01 challenge records.
            pub struct Digitalocean {
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub token_secret_ref: TokenSecretRef,
            }

            /// Configures cert-manager to attempt to complete authorizations by performing the DNS01 challenge flow.
            pub struct Dns01 {
                /// Use the 'ACME DNS' (https://github.com/joohoi/acme-dns) API to manage DNS01 challenge records.
                pub acme_d_n_s: AcmeDNS,
                /// Use the Akamai DNS zone management API to manage DNS01 challenge records.
                pub akamai: Akamai,
                /// Use the Microsoft Azure DNS API to manage DNS01 challenge records.
                pub azure_d_n_s: AzureDNS,
                /// Use the Google Cloud DNS API to manage DNS01 challenge records.
                pub cloud_d_n_s: CloudDNS,
                /// Use the Cloudflare API to manage DNS01 challenge records.
                pub cloudflare: Cloudflare,
                /// CNAMEStrategy configures how the DNS01 provider should handle CNAME records when found in DNS zones.
                pub cname_strategy: String,
                /// Use the DigitalOcean DNS API to manage DNS01 challenge records.
                pub digitalocean: Digitalocean,
                /// Use RFC2136 ("Dynamic Updates in the Domain Name System") (https://datatracker.ietf.org/doc/rfc2136/) to manage DNS01 challenge records.
                pub rfc2136: Rfc2136,
                /// Use the AWS Route53 API to manage DNS01 challenge records.
                pub route53: Route53,
                /// Configure an external webhook based DNS01 challenge solver to manage DNS01 challenge records.
                pub webhook: Webhook,
            }

            /// The Gateway API is a sig-network community API that models service networking in Kubernetes (https://gateway-api.sigs.k8s.io/). The Gateway solver will create HTTPRoutes with the specified labels in the same namespace as the challenge. This solver is experimental, and fields / behaviour may change in the future.
            pub struct GatewayHTTPRoute {
                /// Custom labels that will be applied to HTTPRoutes created by cert-manager while solving HTTP-01 challenges.
                pub labels: GatewayHTTPRouteLabels,
                /// When solving an HTTP-01 challenge, cert-manager creates an HTTPRoute. cert-manager needs to know which parentRefs should be used when creating the HTTPRoute. Usually, the parentRef references a Gateway. See: https://gateway-api.sigs.k8s.io/v1alpha2/api-types/httproute/#attaching-to-gateways
                pub parent_refs: Vec<ParentRef>,
                /// Optional service type for Kubernetes solver service. Supported values are NodePort or ClusterIP. If unset, defaults to NodePort.
                pub service_type: String,
            }

            /// Configures cert-manager to attempt to complete authorizations by performing the HTTP01 challenge flow. It is not possible to obtain certificates for wildcard domain names (e.g. `*.example.com`) using the HTTP01 challenge mechanism.
            pub struct Http01 {
                /// The Gateway API is a sig-network community API that models service networking in Kubernetes (https://gateway-api.sigs.k8s.io/). The Gateway solver will create HTTPRoutes with the specified labels in the same namespace as the challenge. This solver is experimental, and fields / behaviour may change in the future.
                pub gateway_h_t_t_p_route: GatewayHTTPRoute,
                /// The ingress based HTTP01 challenge solver will solve challenges by creating or modifying Ingress resources in order to route requests for '/.well-known/acme-challenge/XYZ' to 'challenge solver' pods that are provisioned by cert-manager for each Challenge to be completed.
                pub ingress: Ingress,
            }

            /// The ingress based HTTP01 challenge solver will solve challenges by creating or modifying Ingress resources in order to route requests for '/.well-known/acme-challenge/XYZ' to 'challenge solver' pods that are provisioned by cert-manager for each Challenge to be completed.
            pub struct Ingress {
                /// The ingress class to use when creating Ingress resources to solve ACME challenges that use this challenge solver. Only one of 'class' or 'name' may be specified.
                pub class: String,
                /// Optional ingress template used to configure the ACME challenge solver ingress used for HTTP01 challenges.
                pub ingress_template: IngressTemplate,
                /// The name of the ingress resource that should have ACME challenge solving routes inserted into it in order to solve HTTP01 challenges. This is typically used in conjunction with ingress controllers like ingress-gce, which maintains a 1:1 mapping between external IPs and ingress resources.
                pub name: String,
                /// Optional pod template used to configure the ACME challenge solver pods used for HTTP01 challenges.
                pub pod_template: PodTemplate,
                /// Optional service type for Kubernetes solver service. Supported values are NodePort or ClusterIP. If unset, defaults to NodePort.
                pub service_type: String,
            }

            /// Optional ingress template used to configure the ACME challenge solver ingress used for HTTP01 challenges.
            pub struct IngressTemplate {
                /// ObjectMeta overrides for the ingress used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
                pub metadata: IngressTemplateMetadata,
            }

            /// References a properly configured ACME-type Issuer which should be used to create this Challenge. If the Issuer does not exist, processing will be retried. If the Issuer is not an 'ACME' Issuer, an error will be returned and the Challenge will be marked as failed.
            pub struct IssuerRef {
                /// Group of the resource being referred to.
                pub group: String,
                /// Kind of the resource being referred to.
                pub kind: String,
                /// Name of the resource being referred to.
                pub name: String,
            }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels,
        }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels,
        }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels,
        }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels,
        }

            /// Custom labels that will be applied to HTTPRoutes created by cert-manager while solving HTTP-01 challenges.
            pub struct GatewayHTTPRouteLabels {
                /// Custom labels that will be applied to HTTPRoutes created by cert-manager while solving HTTP-01 challenges.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// Labels that should be added to the created ACME HTTP01 solver ingress.
            pub struct IngressTemplateMetadataLabels {
                /// Labels that should be added to the created ACME HTTP01 solver ingress.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// Labels that should be added to the created ACME HTTP01 solver pods.
            pub struct PodTemplateMetadataLabels {
                /// Labels that should be added to the created ACME HTTP01 solver pods.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// managed identity configuration, can not be used at the same time as clientID, clientSecretSecretRef or tenantID
            pub struct ManagedIdentity {
                /// client ID of the managed identity, can not be used at the same time as resourceID
                pub client_i_d: String,
                /// resource ID of the managed identity, can not be used at the same time as clientID
                pub resource_i_d: String,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PreferenceMatchExpression {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct NodeSelectorTermMatchExpression {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PreferenceMatchField {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct NodeSelectorTermMatchField {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// A label selector that is used to refine the set of certificate's that this challenge solver will apply to.
            pub struct SelectorMatchLabels {
                /// A label selector that is used to refine the set of certificate's that this challenge solver will apply to.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// ObjectMeta overrides for the ingress used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
            pub struct IngressTemplateMetadata {
                /// Annotations that should be added to the created ACME HTTP01 solver ingress.
                pub annotations: IngressTemplateMetadataAnnotations,
                /// Labels that should be added to the created ACME HTTP01 solver ingress.
                pub labels: IngressTemplateMetadataLabels,
            }

            /// ObjectMeta overrides for the pod used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
            pub struct PodTemplateMetadata {
                /// Annotations that should be added to the create ACME HTTP01 solver pods.
                pub annotations: PodTemplateMetadataAnnotations,
                /// Labels that should be added to the created ACME HTTP01 solver pods.
                pub labels: PodTemplateMetadataLabels,
            }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels,
        }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels,
        }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels,
        }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels,
        }

            /// Describes node affinity scheduling rules for the pod.
            pub struct NodeAffinity {
                /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
                pub preferred_during_scheduling_ignored_during_execution:
                    Vec<NodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem>,
                /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
                pub required_during_scheduling_ignored_during_execution:
                    RequiredDuringSchedulingIgnoredDuringExecution,
            }

            /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
            pub struct NodeSelector {
                /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
                pub properties: std::collections::HashMap<String, String>,
            }

            /// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
            pub struct NodeSelectorTerm {
                /// A list of node selector requirements by node's labels.
                pub match_expressions: Vec<NodeSelectorTermMatchExpression>,
                /// A list of node selector requirements by node's fields.
                pub match_fields: Vec<NodeSelectorTermMatchField>,
            }

            /// ParentRef identifies an API object (usually a Gateway) that can be considered a parent of this resource (usually a route). The only kind of parent resource with "Core" support is Gateway. This API may be extended in the future to support additional kinds of parent resources, such as HTTPRoute.
            ///  The API object must be valid in the cluster; the Group and Kind must be registered in the cluster for this reference to be valid.
            ///  References to objects with invalid Group and Kind are not valid, and must be rejected by the implementation, with appropriate Conditions set on the containing object.
            pub struct ParentRef {
                /// Group is the group of the referent.
                ///  Support: Core
                pub group: String,
                /// Kind is kind of the referent.
                ///  Support: Core (Gateway) Support: Custom (Other Resources)
                pub kind: String,
                /// Name is the name of the referent.
                ///  Support: Core
                pub name: String,
                /// Namespace is the namespace of the referent. When unspecified (or empty string), this refers to the local namespace of the Route.
                ///  Support: Core
                pub namespace: String,
                /// SectionName is the name of a section within the target resource. In the following resources, SectionName is interpreted as the following:
                ///  * Gateway: Listener Name
                ///  Implementations MAY choose to support attaching Routes to other resources. If that is the case, they MUST clearly document how SectionName is interpreted.
                ///  When unspecified (empty string), this will reference the entire resource. For the purpose of status, an attachment is considered successful if at least one section in the parent resource accepts it. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway.
                ///  Support: Core
                pub section_name: String,
            }

            /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
            pub struct PodAffinity {
                /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                pub preferred_during_scheduling_ignored_during_execution:
                    Vec<PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem>,
                /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                pub required_during_scheduling_ignored_during_execution:
                    Vec<PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem>,
            }

            /// Required. A pod affinity term, associated with the corresponding weight.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

            /// Required. A pod affinity term, associated with the corresponding weight.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

            /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
            pub struct PodAntiAffinity {
                /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                pub preferred_during_scheduling_ignored_during_execution:
                    Vec<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem>,
                /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                pub required_during_scheduling_ignored_during_execution:
                    Vec<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem>,
            }

            /// Optional pod template used to configure the ACME challenge solver pods used for HTTP01 challenges.
            pub struct PodTemplate {
                /// ObjectMeta overrides for the pod used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
                pub metadata: PodTemplateMetadata,
                /// PodSpec defines overrides for the HTTP01 challenge solver pod. Only the 'priorityClassName', 'nodeSelector', 'affinity', 'serviceAccountName' and 'tolerations' fields are supported currently. All other fields will be ignored.
                pub spec: PodTemplateSpec,
            }

            /// A node selector term, associated with the corresponding weight.
            pub struct Preference {
                /// A list of node selector requirements by node's labels.
                pub match_expressions: Vec<PreferenceMatchExpression>,
                /// A list of node selector requirements by node's fields.
                pub match_fields: Vec<PreferenceMatchField>,
            }

            /// An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
            pub struct NodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem {
                /// A node selector term, associated with the corresponding weight.
                pub preference: Preference,
                /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
                pub weight: i32,
            }

            /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem {
                /// Required. A pod affinity term, associated with the corresponding weight.
                pub pod_affinity_term:
                    PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm,
                /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
                pub weight: i32,
            }

            /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem {
            /// Required. A pod affinity term, associated with the corresponding weight.
            pub pod_affinity_term: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm,
            /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
            pub weight: i32,
        }

            /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
            pub struct RequiredDuringSchedulingIgnoredDuringExecution {
                /// Required. A list of node selector terms. The terms are ORed.
                pub node_selector_terms: Vec<NodeSelectorTerm>,
            }

            /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem {
                /// A label query over a set of resources, in this case pods.
                pub label_selector:
                    PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector,
                /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
                pub namespace_selector:
                    PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector,
                /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
                pub namespaces: Vec<String>,
                /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
                pub topology_key: String,
            }

            /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

            /// Use RFC2136 ("Dynamic Updates in the Domain Name System") (https://datatracker.ietf.org/doc/rfc2136/) to manage DNS01 challenge records.
            pub struct Rfc2136 {
                /// The IP address or hostname of an authoritative DNS server supporting RFC2136 in the form host:port. If the host is an IPv6 address it must be enclosed in square brackets (e.g [2001:db8::1]) ; port is optional. This field is required.
                pub nameserver: String,
                /// The TSIG Algorithm configured in the DNS supporting RFC2136. Used only when ``tsigSecretSecretRef`` and ``tsigKeyName`` are defined. Supported values are (case-insensitive): ``HMACMD5`` (default), ``HMACSHA1``, ``HMACSHA256`` or ``HMACSHA512``.
                pub tsig_algorithm: String,
                /// The TSIG Key name configured in the DNS. If ``tsigSecretSecretRef`` is defined, this field is required.
                pub tsig_key_name: String,
                /// The name of the secret containing the TSIG value. If ``tsigKeyName`` is defined, this field is required.
                pub tsig_secret_secret_ref: TsigSecretSecretRef,
            }

            /// Use the AWS Route53 API to manage DNS01 challenge records.
            pub struct Route53 {
                /// The AccessKeyID is used for authentication. If not set we fall-back to using env vars, shared credentials file or AWS Instance metadata see: https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html#specifying-credentials
                pub access_key_i_d: String,
                /// If set, the provider will manage only this zone in Route53 and will not do an lookup using the route53:ListHostedZonesByName api call.
                pub hosted_zone_i_d: String,
                /// Always set the region when using AccessKeyID and SecretAccessKey
                pub region: String,
                /// Role is a Role ARN which the Route53 provider will assume using either the explicit credentials AccessKeyID/SecretAccessKey or the inferred credentials from environment variables, shared credentials file or AWS Instance metadata
                pub role: String,
                /// The SecretAccessKey is used for authentication. If not set we fall-back to using env vars, shared credentials file or AWS Instance metadata https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html#specifying-credentials
                pub secret_access_key_secret_ref: SecretAccessKeySecretRef,
            }

            /// The SecretAccessKey is used for authentication. If not set we fall-back to using env vars, shared credentials file or AWS Instance metadata https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html#specifying-credentials
            pub struct SecretAccessKeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Selector selects a set of DNSNames on the Certificate resource that should be solved using this challenge solver. If not specified, the solver will be treated as the 'default' solver with the lowest priority, i.e. if any other solver has a more specific match, it will be used instead.
            pub struct Selector {
                /// List of DNSNames that this solver will be used to solve. If specified and a match is found, a dnsNames selector will take precedence over a dnsZones selector. If multiple solvers match with the same dnsNames value, the solver with the most matching labels in matchLabels will be selected. If neither has more matches, the solver defined earlier in the list will be selected.
                pub dns_names: Vec<String>,
                /// List of DNSZones that this solver will be used to solve. The most specific DNS zone match specified here will take precedence over other DNS zone matches, so a solver specifying sys.example.com will be selected over one specifying example.com for the domain www.sys.example.com. If multiple solvers match with the same dnsZones value, the solver with the most matching labels in matchLabels will be selected. If neither has more matches, the solver defined earlier in the list will be selected.
                pub dns_zones: Vec<String>,
                /// A label selector that is used to refine the set of certificate's that this challenge solver will apply to.
                pub match_labels: SelectorMatchLabels,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct ServiceAccountSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Contains the domain solving configuration that should be used to solve this challenge resource.
            pub struct Solver {
                /// Configures cert-manager to attempt to complete authorizations by performing the DNS01 challenge flow.
                pub dns01: Dns01,
                /// Configures cert-manager to attempt to complete authorizations by performing the HTTP01 challenge flow. It is not possible to obtain certificates for wildcard domain names (e.g. `*.example.com`) using the HTTP01 challenge mechanism.
                pub http01: Http01,
                /// Selector selects a set of DNSNames on the Certificate resource that should be solved using this challenge solver. If not specified, the solver will be treated as the 'default' solver with the lowest priority, i.e. if any other solver has a more specific match, it will be used instead.
                pub selector: Selector,
            }

            pub struct Spec {
                /// The URL to the ACME Authorization resource that this challenge is a part of.
                pub authorization_u_r_l: String,
                /// dnsName is the identifier that this challenge is for, e.g. example.com. If the requested DNSName is a 'wildcard', this field MUST be set to the non-wildcard domain, e.g. for `*.example.com`, it must be `example.com`.
                pub dns_name: String,
                /// References a properly configured ACME-type Issuer which should be used to create this Challenge. If the Issuer does not exist, processing will be retried. If the Issuer is not an 'ACME' Issuer, an error will be returned and the Challenge will be marked as failed.
                pub issuer_ref: IssuerRef,
                /// The ACME challenge key for this challenge For HTTP01 challenges, this is the value that must be responded with to complete the HTTP01 challenge in the format: `<private key JWK thumbprint>.<key from acme server for challenge>`. For DNS01 challenges, this is the base64 encoded SHA256 sum of the `<private key JWK thumbprint>.<key from acme server for challenge>` text that must be set as the TXT record content.
                pub key: String,
                /// Contains the domain solving configuration that should be used to solve this challenge resource.
                pub solver: Solver,
                /// The ACME challenge token for this challenge. This is the raw value returned from the ACME server.
                pub token: String,
                /// The type of ACME challenge this resource represents. One of "HTTP-01" or "DNS-01".
                pub r#type: String,
                /// The URL of the ACME Challenge resource for this challenge. This can be used to lookup details about the status of this challenge.
                pub url: String,
                /// wildcard will be true if this challenge is for a wildcard identifier, for example '*.example.com'.
                pub wildcard: bool,
            }

            /// PodSpec defines overrides for the HTTP01 challenge solver pod. Only the 'priorityClassName', 'nodeSelector', 'affinity', 'serviceAccountName' and 'tolerations' fields are supported currently. All other fields will be ignored.
            pub struct PodTemplateSpec {
                /// If specified, the pod's scheduling constraints
                pub affinity: Affinity,
                /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
                pub node_selector: NodeSelector,
                /// If specified, the pod's priorityClassName.
                pub priority_class_name: String,
                /// If specified, the pod's service account
                pub service_account_name: String,
                /// If specified, the pod's tolerations.
                pub tolerations: Vec<Toleration>,
            }

            pub struct Status {
                /// presented will be set to true if the challenge values for this challenge are currently 'presented'. This *does not* imply the self check is passing. Only that the values have been 'submitted' for the appropriate challenge mechanism (i.e. the DNS01 TXT record has been presented, or the HTTP01 configuration has been configured).
                pub presented: bool,
                /// Used to denote whether this challenge should be processed or not. This field will only be set to true by the 'scheduling' component. It will only be set to false by the 'challenges' controller, after the challenge has reached a final state or timed out. If this field is set to false, the challenge controller will not take any more action.
                pub processing: bool,
                /// Contains human readable information on why the Challenge is in the current state.
                pub reason: String,
                /// Contains the current 'state' of the challenge. If not set, the state of the challenge is unknown.
                pub state: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct TokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
            pub struct Toleration {
                /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
                pub effect: String,
                /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
                pub key: String,
                /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
                pub operator: String,
                /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
                pub toleration_seconds: i64,
                /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
                pub value: String,
            }

            /// The name of the secret containing the TSIG value. If ``tsigKeyName`` is defined, this field is required.
            pub struct TsigSecretSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Configure an external webhook based DNS01 challenge solver to manage DNS01 challenge records.
            pub struct Webhook {
                /// Additional configuration that should be passed to the webhook apiserver when challenges are processed. This can contain arbitrary JSON data. Secret values should not be specified in this stanza. If secret values are needed (e.g. credentials for a DNS service), you should use a SecretKeySelector to reference a Secret resource. For details on the schema of this field, consult the webhook provider implementation's documentation.
                pub config: std::collections::HashMap<String, String>,
                /// The API group name that should be used when POSTing ChallengePayload resources to the webhook apiserver. This should be the same as the GroupName specified in the webhook provider implementation.
                pub group_name: String,
                /// The name of the solver to use, as defined in the webhook provider implementation. This will typically be the name of the provider, e.g. 'cloudflare'.
                pub solver_name: String,
            }

            impl k8s_openapi::Resource for Challenge {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "acme.cert-manager.io/v1";
                const GROUP: &'static str = "acme.cert-manager.io";
                const KIND: &'static str = "Challenge";
                const VERSION: &'static str = "v1";
                const URL_PATH_SEGMENT: &'static str = "TODO";
            }

            impl k8s_openapi::Metadata for Challenge {
                type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

                fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                    &self.metadata
                }

                fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                    &mut self.metadata
                }
            }
        }
        pub mod order {
            /// Order is a type to represent an Order with an ACME server
            pub struct Order {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
                pub status: Status,
            }

            /// ACMEAuthorization contains data returned from the ACME server on an authorization that must be completed in order validate a DNS name on an ACME Order resource.
            pub struct Authorization {
                /// Challenges specifies the challenge types offered by the ACME server. One of these challenge types will be selected when validating the DNS name and an appropriate Challenge resource will be created to perform the ACME challenge process.
                pub challenges: Vec<Challenge>,
                /// Identifier is the DNS name to be validated as part of this authorization
                pub identifier: String,
                /// InitialState is the initial state of the ACME authorization when first fetched from the ACME server. If an Authorization is already 'valid', the Order controller will not create a Challenge resource for the authorization. This will occur when working with an ACME server that enables 'authz reuse' (such as Let's Encrypt's production endpoint). If not set and 'identifier' is set, the state is assumed to be pending and a Challenge will be created.
                pub initial_state: String,
                /// URL is the URL of the Authorization that must be completed
                pub url: String,
                /// Wildcard will be true if this authorization is for a wildcard DNS name. If this is true, the identifier will be the *non-wildcard* version of the DNS name. For example, if '*.example.com' is the DNS name being validated, this field will be 'true' and the 'identifier' field will be 'example.com'.
                pub wildcard: bool,
            }

            /// Challenge specifies a challenge offered by the ACME server for an Order. An appropriate Challenge resource can be created to perform the ACME challenge process.
            pub struct Challenge {
                /// Token is the token that must be presented for this challenge. This is used to compute the 'key' that must also be presented.
                pub token: String,
                /// Type is the type of challenge being offered, e.g. 'http-01', 'dns-01', 'tls-sni-01', etc. This is the raw value retrieved from the ACME server. Only 'http-01' and 'dns-01' are supported by cert-manager, other values will be ignored.
                pub r#type: String,
                /// URL is the URL of this challenge. It can be used to retrieve additional metadata about the Challenge from the ACME server.
                pub url: String,
            }

            /// IssuerRef references a properly configured ACME-type Issuer which should be used to create this Order. If the Issuer does not exist, processing will be retried. If the Issuer is not an 'ACME' Issuer, an error will be returned and the Order will be marked as failed.
            pub struct IssuerRef {
                /// Group of the resource being referred to.
                pub group: String,
                /// Kind of the resource being referred to.
                pub kind: String,
                /// Name of the resource being referred to.
                pub name: String,
            }

            pub struct Spec {
                /// CommonName is the common name as specified on the DER encoded CSR. If specified, this value must also be present in `dnsNames` or `ipAddresses`. This field must match the corresponding field on the DER encoded CSR.
                pub common_name: String,
                /// DNSNames is a list of DNS names that should be included as part of the Order validation process. This field must match the corresponding field on the DER encoded CSR.
                pub dns_names: Vec<String>,
                /// Duration is the duration for the not after date for the requested certificate. this is set on order creation as pe the ACME spec.
                pub duration: String,
                /// IPAddresses is a list of IP addresses that should be included as part of the Order validation process. This field must match the corresponding field on the DER encoded CSR.
                pub ip_addresses: Vec<String>,
                /// IssuerRef references a properly configured ACME-type Issuer which should be used to create this Order. If the Issuer does not exist, processing will be retried. If the Issuer is not an 'ACME' Issuer, an error will be returned and the Order will be marked as failed.
                pub issuer_ref: IssuerRef,
                /// Certificate signing request bytes in DER encoding. This will be used when finalizing the order. This field must be set on the order.
                pub request: String,
            }

            pub struct Status {
                /// Authorizations contains data returned from the ACME server on what authorizations must be completed in order to validate the DNS names specified on the Order.
                pub authorizations: Vec<Authorization>,
                /// Certificate is a copy of the PEM encoded certificate for this Order. This field will be populated after the order has been successfully finalized with the ACME server, and the order has transitioned to the 'valid' state.
                pub certificate: String,
                /// FailureTime stores the time that this order failed. This is used to influence garbage collection and back-off.
                pub failure_time: String,
                /// FinalizeURL of the Order. This is used to obtain certificates for this order once it has been completed.
                pub finalize_u_r_l: String,
                /// Reason optionally provides more information about a why the order is in the current state.
                pub reason: String,
                /// State contains the current state of this Order resource. States 'success' and 'expired' are 'final'
                pub state: String,
                /// URL of the Order. This will initially be empty when the resource is first created. The Order controller will populate this field when the Order is first processed. This field will be immutable after it is initially set.
                pub url: String,
            }

            impl k8s_openapi::Resource for Order {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "acme.cert-manager.io/v1";
                const GROUP: &'static str = "acme.cert-manager.io";
                const KIND: &'static str = "Order";
                const VERSION: &'static str = "v1";
                const URL_PATH_SEGMENT: &'static str = "TODO";
            }

            impl k8s_openapi::Metadata for Order {
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
pub mod cert_manager_io {
    pub mod v1 {
        pub mod certificate {
            /// A Certificate resource should be created to ensure an up to date and signed x509 certificate is stored in the Kubernetes Secret resource named in `spec.secretName`.
            ///  The stored certificate will be renewed before it expires (as configured by `spec.renewBefore`).
            pub struct Certificate {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
                pub status: Status,
            }

            /// CertificateAdditionalOutputFormat defines an additional output format of a Certificate resource. These contain supplementary data formats of the signed certificate chain and paired private key.
            pub struct AdditionalOutputFormat {
                /// Type is the name of the format type that should be written to the Certificate's target Secret.
                pub r#type: String,
            }

            /// Annotations is a key value map to be copied to the target Kubernetes Secret.
            pub struct Annotations {
                /// Annotations is a key value map to be copied to the target Kubernetes Secret.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// CertificateCondition contains condition information for an Certificate.
            pub struct Condition {
                /// LastTransitionTime is the timestamp corresponding to the last status change of this condition.
                pub last_transition_time: String,
                /// Message is a human readable description of the details of the last transition, complementing reason.
                pub message: String,
                /// If set, this represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.condition[x].observedGeneration is 9, the condition is out of date with respect to the current state of the Certificate.
                pub observed_generation: i64,
                /// Reason is a brief machine readable explanation for the condition's last transition.
                pub reason: String,
                /// Status of the condition, one of (`True`, `False`, `Unknown`).
                pub status: String,
                /// Type of the condition, known values are (`Ready`, `Issuing`).
                pub r#type: String,
            }

            /// IssuerRef is a reference to the issuer for this certificate. If the `kind` field is not set, or set to `Issuer`, an Issuer resource with the given name in the same namespace as the Certificate will be used. If the `kind` field is set to `ClusterIssuer`, a ClusterIssuer with the provided name will be used. The `name` field in this stanza is required at all times.
            pub struct IssuerRef {
                /// Group of the resource being referred to.
                pub group: String,
                /// Kind of the resource being referred to.
                pub kind: String,
                /// Name of the resource being referred to.
                pub name: String,
            }

            /// JKS configures options for storing a JKS keystore in the `spec.secretName` Secret resource.
            pub struct Jks {
                /// Create enables JKS keystore creation for the Certificate. If true, a file named `keystore.jks` will be created in the target Secret resource, encrypted using the password stored in `passwordSecretRef`. The keystore file will only be updated upon re-issuance. A file named `truststore.jks` will also be created in the target Secret resource, encrypted using the password stored in `passwordSecretRef` containing the issuing Certificate Authority
                pub create: bool,
                /// PasswordSecretRef is a reference to a key in a Secret resource containing the password used to encrypt the JKS keystore.
                pub password_secret_ref: JksPasswordSecretRef,
            }

            /// Keystores configures additional keystore output formats stored in the `secretName` Secret resource.
            pub struct Keystores {
                /// JKS configures options for storing a JKS keystore in the `spec.secretName` Secret resource.
                pub jks: Jks,
                /// PKCS12 configures options for storing a PKCS12 keystore in the `spec.secretName` Secret resource.
                pub pkcs12: Pkcs12,
            }

            /// Labels is a key value map to be copied to the target Kubernetes Secret.
            pub struct Labels {
                /// Labels is a key value map to be copied to the target Kubernetes Secret.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// PasswordSecretRef is a reference to a key in a Secret resource containing the password used to encrypt the JKS keystore.
            pub struct JksPasswordSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// PasswordSecretRef is a reference to a key in a Secret resource containing the password used to encrypt the PKCS12 keystore.
            pub struct Pkcs12PasswordSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// PKCS12 configures options for storing a PKCS12 keystore in the `spec.secretName` Secret resource.
            pub struct Pkcs12 {
                /// Create enables PKCS12 keystore creation for the Certificate. If true, a file named `keystore.p12` will be created in the target Secret resource, encrypted using the password stored in `passwordSecretRef`. The keystore file will only be updated upon re-issuance. A file named `truststore.p12` will also be created in the target Secret resource, encrypted using the password stored in `passwordSecretRef` containing the issuing Certificate Authority
                pub create: bool,
                /// PasswordSecretRef is a reference to a key in a Secret resource containing the password used to encrypt the PKCS12 keystore.
                pub password_secret_ref: Pkcs12PasswordSecretRef,
            }

            /// Options to control private keys used for the Certificate.
            pub struct PrivateKey {
                /// Algorithm is the private key algorithm of the corresponding private key for this certificate. If provided, allowed values are either `RSA`,`Ed25519` or `ECDSA` If `algorithm` is specified and `size` is not provided, key size of 256 will be used for `ECDSA` key algorithm and key size of 2048 will be used for `RSA` key algorithm. key size is ignored when using the `Ed25519` key algorithm.
                pub algorithm: String,
                /// The private key cryptography standards (PKCS) encoding for this certificate's private key to be encoded in. If provided, allowed values are `PKCS1` and `PKCS8` standing for PKCS#1 and PKCS#8, respectively. Defaults to `PKCS1` if not specified.
                pub encoding: String,
                /// RotationPolicy controls how private keys should be regenerated when a re-issuance is being processed. If set to Never, a private key will only be generated if one does not already exist in the target `spec.secretName`. If one does exists but it does not have the correct algorithm or size, a warning will be raised to await user intervention. If set to Always, a private key matching the specified requirements will be generated whenever a re-issuance occurs. Default is 'Never' for backward compatibility.
                pub rotation_policy: String,
                /// Size is the key bit size of the corresponding private key for this certificate. If `algorithm` is set to `RSA`, valid values are `2048`, `4096` or `8192`, and will default to `2048` if not specified. If `algorithm` is set to `ECDSA`, valid values are `256`, `384` or `521`, and will default to `256` if not specified. If `algorithm` is set to `Ed25519`, Size is ignored. No other values are allowed.
                pub size: i64,
            }

            /// SecretTemplate defines annotations and labels to be copied to the Certificate's Secret. Labels and annotations on the Secret will be changed as they appear on the SecretTemplate when added or removed. SecretTemplate annotations are added in conjunction with, and cannot overwrite, the base set of annotations cert-manager sets on the Certificate's Secret.
            pub struct SecretTemplate {
                /// Annotations is a key value map to be copied to the target Kubernetes Secret.
                pub annotations: Annotations,
                /// Labels is a key value map to be copied to the target Kubernetes Secret.
                pub labels: Labels,
            }

            /// Desired state of the Certificate resource.
            pub struct Spec {
                /// AdditionalOutputFormats defines extra output formats of the private key and signed certificate chain to be written to this Certificate's target Secret. This is an Alpha Feature and is only enabled with the `--feature-gates=AdditionalCertificateOutputFormats=true` option on both the controller and webhook components.
                pub additional_output_formats: Vec<AdditionalOutputFormat>,
                /// CommonName is a common name to be used on the Certificate. The CommonName should have a length of 64 characters or fewer to avoid generating invalid CSRs. This value is ignored by TLS clients when any subject alt name is set. This is x509 behaviour: https://tools.ietf.org/html/rfc6125#section-6.4.4
                pub common_name: String,
                /// DNSNames is a list of DNS subjectAltNames to be set on the Certificate.
                pub dns_names: Vec<String>,
                /// The requested 'duration' (i.e. lifetime) of the Certificate. This option may be ignored/overridden by some issuer types. If unset this defaults to 90 days. Certificate will be renewed either 2/3 through its duration or `renewBefore` period before its expiry, whichever is later. Minimum accepted duration is 1 hour. Value must be in units accepted by Go time.ParseDuration https://golang.org/pkg/time/#ParseDuration
                pub duration: String,
                /// EmailAddresses is a list of email subjectAltNames to be set on the Certificate.
                pub email_addresses: Vec<String>,
                /// EncodeUsagesInRequest controls whether key usages should be present in the CertificateRequest
                pub encode_usages_in_request: bool,
                /// IPAddresses is a list of IP address subjectAltNames to be set on the Certificate.
                pub ip_addresses: Vec<String>,
                /// IsCA will mark this Certificate as valid for certificate signing. This will automatically add the `cert sign` usage to the list of `usages`.
                pub is_c_a: bool,
                /// IssuerRef is a reference to the issuer for this certificate. If the `kind` field is not set, or set to `Issuer`, an Issuer resource with the given name in the same namespace as the Certificate will be used. If the `kind` field is set to `ClusterIssuer`, a ClusterIssuer with the provided name will be used. The `name` field in this stanza is required at all times.
                pub issuer_ref: IssuerRef,
                /// Keystores configures additional keystore output formats stored in the `secretName` Secret resource.
                pub keystores: Keystores,
                /// Options to control private keys used for the Certificate.
                pub private_key: PrivateKey,
                /// How long before the currently issued certificate's expiry cert-manager should renew the certificate. The default is 2/3 of the issued certificate's duration. Minimum accepted value is 5 minutes. Value must be in units accepted by Go time.ParseDuration https://golang.org/pkg/time/#ParseDuration
                pub renew_before: String,
                /// revisionHistoryLimit is the maximum number of CertificateRequest revisions that are maintained in the Certificate's history. Each revision represents a single `CertificateRequest` created by this Certificate, either when it was created, renewed, or Spec was changed. Revisions will be removed by oldest first if the number of revisions exceeds this number. If set, revisionHistoryLimit must be a value of `1` or greater. If unset (`nil`), revisions will not be garbage collected. Default value is `nil`.
                pub revision_history_limit: i32,
                /// SecretName is the name of the secret resource that will be automatically created and managed by this Certificate resource. It will be populated with a private key and certificate, signed by the denoted issuer.
                pub secret_name: String,
                /// SecretTemplate defines annotations and labels to be copied to the Certificate's Secret. Labels and annotations on the Secret will be changed as they appear on the SecretTemplate when added or removed. SecretTemplate annotations are added in conjunction with, and cannot overwrite, the base set of annotations cert-manager sets on the Certificate's Secret.
                pub secret_template: SecretTemplate,
                /// Full X509 name specification (https://golang.org/pkg/crypto/x509/pkix/#Name).
                pub subject: Subject,
                /// URIs is a list of URI subjectAltNames to be set on the Certificate.
                pub uris: Vec<String>,
                /// Usages is the set of x509 usages that are requested for the certificate. Defaults to `digital signature` and `key encipherment` if not specified.
                pub usages: Vec<String>,
            }

            /// Status of the Certificate. This is set and managed automatically.
            pub struct Status {
                /// List of status conditions to indicate the status of certificates. Known condition types are `Ready` and `Issuing`.
                pub conditions: Vec<Condition>,
                /// The number of continuous failed issuance attempts up till now. This field gets removed (if set) on a successful issuance and gets set to 1 if unset and an issuance has failed. If an issuance has failed, the delay till the next issuance will be calculated using formula time.Hour * 2 ^ (failedIssuanceAttempts - 1).
                pub failed_issuance_attempts: i64,
                /// LastFailureTime is the time as recorded by the Certificate controller of the most recent failure to complete a CertificateRequest for this Certificate resource. If set, cert-manager will not re-request another Certificate until 1 hour has elapsed from this time.
                pub last_failure_time: String,
                /// The name of the Secret resource containing the private key to be used for the next certificate iteration. The keymanager controller will automatically set this field if the `Issuing` condition is set to `True`. It will automatically unset this field when the Issuing condition is not set or False.
                pub next_private_key_secret_name: String,
                /// The expiration time of the certificate stored in the secret named by this resource in `spec.secretName`.
                pub not_after: String,
                /// The time after which the certificate stored in the secret named by this resource in spec.secretName is valid.
                pub not_before: String,
                /// RenewalTime is the time at which the certificate will be next renewed. If not set, no upcoming renewal is scheduled.
                pub renewal_time: String,
                /// The current 'revision' of the certificate as issued.
                ///  When a CertificateRequest resource is created, it will have the `cert-manager.io/certificate-revision` set to one greater than the current value of this field.
                ///  Upon issuance, this field will be set to the value of the annotation on the CertificateRequest resource used to issue the certificate.
                ///  Persisting the value on the CertificateRequest resource allows the certificates controller to know whether a request is part of an old issuance or if it is part of the ongoing revision's issuance by checking if the revision value in the annotation is greater than this field.
                pub revision: i64,
            }

            /// Full X509 name specification (https://golang.org/pkg/crypto/x509/pkix/#Name).
            pub struct Subject {
                /// Countries to be used on the Certificate.
                pub countries: Vec<String>,
                /// Cities to be used on the Certificate.
                pub localities: Vec<String>,
                /// Organizational Units to be used on the Certificate.
                pub organizational_units: Vec<String>,
                /// Organizations to be used on the Certificate.
                pub organizations: Vec<String>,
                /// Postal codes to be used on the Certificate.
                pub postal_codes: Vec<String>,
                /// State/Provinces to be used on the Certificate.
                pub provinces: Vec<String>,
                /// Serial number to be used on the Certificate.
                pub serial_number: String,
                /// Street addresses to be used on the Certificate.
                pub street_addresses: Vec<String>,
            }

            impl k8s_openapi::Resource for Certificate {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "cert-manager.io/v1";
                const GROUP: &'static str = "cert-manager.io";
                const KIND: &'static str = "Certificate";
                const VERSION: &'static str = "v1";
                const URL_PATH_SEGMENT: &'static str = "TODO";
            }

            impl k8s_openapi::Metadata for Certificate {
                type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

                fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                    &self.metadata
                }

                fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                    &mut self.metadata
                }
            }
        }
        pub mod certificate_request {
            /// A CertificateRequest is used to request a signed certificate from one of the configured issuers.
            ///  All fields within the CertificateRequest's `spec` are immutable after creation. A CertificateRequest will either succeed or fail, as denoted by its `status.state` field.
            ///  A CertificateRequest is a one-shot resource, meaning it represents a single point in time request for a certificate and cannot be re-used.
            pub struct CertificateRequest {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
                pub status: Status,
            }

            /// CertificateRequestCondition contains condition information for a CertificateRequest.
            pub struct Condition {
                /// LastTransitionTime is the timestamp corresponding to the last status change of this condition.
                pub last_transition_time: String,
                /// Message is a human readable description of the details of the last transition, complementing reason.
                pub message: String,
                /// Reason is a brief machine readable explanation for the condition's last transition.
                pub reason: String,
                /// Status of the condition, one of (`True`, `False`, `Unknown`).
                pub status: String,
                /// Type of the condition, known values are (`Ready`, `InvalidRequest`, `Approved`, `Denied`).
                pub r#type: String,
            }

            /// Extra contains extra attributes of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
            pub struct Extra {
                /// Extra contains extra attributes of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
                pub properties: std::collections::HashMap<String, Vec<String>>,
            }

            /// IssuerRef is a reference to the issuer for this CertificateRequest.  If the `kind` field is not set, or set to `Issuer`, an Issuer resource with the given name in the same namespace as the CertificateRequest will be used.  If the `kind` field is set to `ClusterIssuer`, a ClusterIssuer with the provided name will be used. The `name` field in this stanza is required at all times. The group field refers to the API group of the issuer which defaults to `cert-manager.io` if empty.
            pub struct IssuerRef {
                /// Group of the resource being referred to.
                pub group: String,
                /// Kind of the resource being referred to.
                pub kind: String,
                /// Name of the resource being referred to.
                pub name: String,
            }

            /// Desired state of the CertificateRequest resource.
            pub struct Spec {
                /// The requested 'duration' (i.e. lifetime) of the Certificate. This option may be ignored/overridden by some issuer types.
                pub duration: String,
                /// Extra contains extra attributes of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
                pub extra: Extra,
                /// Groups contains group membership of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
                pub groups: Vec<String>,
                /// IsCA will request to mark the certificate as valid for certificate signing when submitting to the issuer. This will automatically add the `cert sign` usage to the list of `usages`.
                pub is_c_a: bool,
                /// IssuerRef is a reference to the issuer for this CertificateRequest.  If the `kind` field is not set, or set to `Issuer`, an Issuer resource with the given name in the same namespace as the CertificateRequest will be used.  If the `kind` field is set to `ClusterIssuer`, a ClusterIssuer with the provided name will be used. The `name` field in this stanza is required at all times. The group field refers to the API group of the issuer which defaults to `cert-manager.io` if empty.
                pub issuer_ref: IssuerRef,
                /// The PEM-encoded x509 certificate signing request to be submitted to the CA for signing.
                pub request: String,
                /// UID contains the uid of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
                pub uid: String,
                /// Usages is the set of x509 usages that are requested for the certificate. If usages are set they SHOULD be encoded inside the CSR spec Defaults to `digital signature` and `key encipherment` if not specified.
                pub usages: Vec<String>,
                /// Username contains the name of the user that created the CertificateRequest. Populated by the cert-manager webhook on creation and immutable.
                pub username: String,
            }

            /// Status of the CertificateRequest. This is set and managed automatically.
            pub struct Status {
                /// The PEM encoded x509 certificate of the signer, also known as the CA (Certificate Authority). This is set on a best-effort basis by different issuers. If not set, the CA is assumed to be unknown/not available.
                pub ca: String,
                /// The PEM encoded x509 certificate resulting from the certificate signing request. If not set, the CertificateRequest has either not been completed or has failed. More information on failure can be found by checking the `conditions` field.
                pub certificate: String,
                /// List of status conditions to indicate the status of a CertificateRequest. Known condition types are `Ready` and `InvalidRequest`.
                pub conditions: Vec<Condition>,
                /// FailureTime stores the time that this CertificateRequest failed. This is used to influence garbage collection and back-off.
                pub failure_time: String,
            }

            impl k8s_openapi::Resource for CertificateRequest {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "cert-manager.io/v1";
                const GROUP: &'static str = "cert-manager.io";
                const KIND: &'static str = "CertificateRequest";
                const VERSION: &'static str = "v1";
                const URL_PATH_SEGMENT: &'static str = "TODO";
            }

            impl k8s_openapi::Metadata for CertificateRequest {
                type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

                fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                    &self.metadata
                }

                fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                    &mut self.metadata
                }
            }
        }
        pub mod cluster_issuer {
            /// A ClusterIssuer represents a certificate issuing authority which can be referenced as part of `issuerRef` fields. It is similar to an Issuer, however it is cluster-scoped and therefore can be referenced by resources that exist in *any* namespace, not just the same namespace as the referent.
            pub struct ClusterIssuer {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
                pub status: Status,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct AccessTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct AccountSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// ACME configures this issuer to communicate with a RFC8555 (ACME) server to obtain signed x509 certificates.
            pub struct SpecAcme {
                /// Enables or disables generating a new ACME account key. If true, the Issuer resource will *not* request a new account but will expect the account key to be supplied via an existing secret. If false, the cert-manager system will generate a new ACME account key for the Issuer. Defaults to false.
                pub disable_account_key_generation: bool,
                /// Email is the email address to be associated with the ACME account. This field is optional, but it is strongly recommended to be set. It will be used to contact you in case of issues with your account or certificates, including expiry notification emails. This field may be updated after the account is initially registered.
                pub email: String,
                /// Enables requesting a Not After date on certificates that matches the duration of the certificate. This is not supported by all ACME servers like Let's Encrypt. If set to true when the ACME server does not support it it will create an error on the Order. Defaults to false.
                pub enable_duration_feature: bool,
                /// ExternalAccountBinding is a reference to a CA external account of the ACME server. If set, upon registration cert-manager will attempt to associate the given external account credentials with the registered ACME account.
                pub external_account_binding: ExternalAccountBinding,
                /// PreferredChain is the chain to use if the ACME server outputs multiple. PreferredChain is no guarantee that this one gets delivered by the ACME endpoint. For example, for Let's Encrypt's DST crosssign you would use: "DST Root CA X3" or "ISRG Root X1" for the newer Let's Encrypt root CA. This value picks the first certificate bundle in the ACME alternative chains that has a certificate with this value as its issuer's CN
                pub preferred_chain: String,
                /// PrivateKey is the name of a Kubernetes Secret resource that will be used to store the automatically generated ACME account private key. Optionally, a `key` may be specified to select a specific entry within the named Secret resource. If `key` is not specified, a default of `tls.key` will be used.
                pub private_key_secret_ref: PrivateKeySecretRef,
                /// Server is the URL used to access the ACME server's 'directory' endpoint. For example, for Let's Encrypt's staging endpoint, you would use: "https://acme-staging-v02.api.letsencrypt.org/directory". Only ACME v2 endpoints (i.e. RFC 8555) are supported.
                pub server: String,
                /// Enables or disables validation of the ACME server TLS certificate. If true, requests to the ACME server will not have their TLS certificate validated (i.e. insecure connections will be allowed). Only enable this option in development environments. The cert-manager system installed roots will be used to verify connections to the ACME server if this is false. Defaults to false.
                pub skip_t_l_s_verify: bool,
                /// Solvers is a list of challenge solvers that will be used to solve ACME challenges for the matching domains. Solver configurations must be provided in order to obtain certificates from an ACME server. For more information, see: https://cert-manager.io/docs/configuration/acme/
                pub solvers: Vec<Solver>,
            }

            /// ACME specific status options. This field should only be set if the Issuer is configured to use an ACME server to issue certificates.
            pub struct StatusAcme {
                /// LastRegisteredEmail is the email associated with the latest registered ACME account, in order to track changes made to registered account associated with the  Issuer
                pub last_registered_email: String,
                /// URI is the unique account identifier, which can also be used to retrieve account details from the CA
                pub uri: String,
            }

            /// Use the 'ACME DNS' (https://github.com/joohoi/acme-dns) API to manage DNS01 challenge records.
            pub struct AcmeDNS {
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub account_secret_ref: AccountSecretRef,
                pub host: String,
            }

            /// If specified, the pod's scheduling constraints
            pub struct Affinity {
                /// Describes node affinity scheduling rules for the pod.
                pub node_affinity: NodeAffinity,
                /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
                pub pod_affinity: PodAffinity,
                /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
                pub pod_anti_affinity: PodAntiAffinity,
            }

            /// Use the Akamai DNS zone management API to manage DNS01 challenge records.
            pub struct Akamai {
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub access_token_secret_ref: AccessTokenSecretRef,
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub client_secret_secret_ref: AkamaiClientSecretSecretRef,
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub client_token_secret_ref: ClientTokenSecretRef,
                pub service_consumer_domain: String,
            }

            /// Annotations that should be added to the created ACME HTTP01 solver ingress.
            pub struct IngressTemplateMetadataAnnotations {
                /// Annotations that should be added to the created ACME HTTP01 solver ingress.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// Annotations that should be added to the create ACME HTTP01 solver pods.
            pub struct PodTemplateMetadataAnnotations {
                /// Annotations that should be added to the create ACME HTTP01 solver pods.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// API key to use to authenticate with Cloudflare. Note: using an API token to authenticate is now the recommended method as it allows greater control of permissions.
            pub struct ApiKeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// API token used to authenticate with Cloudflare.
            pub struct CloudflareApiTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// APITokenSecretRef is a secret key selector for the Venafi Cloud API token.
            pub struct CloudApiTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// AppRole authenticates with Vault using the App Role auth mechanism, with the role and secret stored in a Kubernetes Secret resource.
            pub struct AppRole {
                /// Path where the App Role authentication backend is mounted in Vault, e.g: "approle"
                pub path: String,
                /// RoleID configured in the App Role authentication backend when setting up the authentication backend in Vault.
                pub role_id: String,
                /// Reference to a key in a Secret that contains the App Role secret used to authenticate with Vault. The `key` field must be specified and denotes which entry within the Secret resource is used as the app role secret.
                pub secret_ref: AppRoleSecretRef,
            }

            /// Auth configures how cert-manager authenticates with the Vault server.
            pub struct Auth {
                /// AppRole authenticates with Vault using the App Role auth mechanism, with the role and secret stored in a Kubernetes Secret resource.
                pub app_role: AppRole,
                /// Kubernetes authenticates with Vault by passing the ServiceAccount token stored in the named Secret resource to the Vault server.
                pub kubernetes: Kubernetes,
                /// TokenSecretRef authenticates with Vault by presenting a token.
                pub token_secret_ref: AuthTokenSecretRef,
            }

            /// Use the Microsoft Azure DNS API to manage DNS01 challenge records.
            pub struct AzureDNS {
                /// if both this and ClientSecret are left unset MSI will be used
                pub client_i_d: String,
                /// if both this and ClientID are left unset MSI will be used
                pub client_secret_secret_ref: AzureDNSClientSecretSecretRef,
                /// name of the Azure environment (default AzurePublicCloud)
                pub environment: String,
                /// name of the DNS zone that should be used
                pub hosted_zone_name: String,
                /// managed identity configuration, can not be used at the same time as clientID, clientSecretSecretRef or tenantID
                pub managed_identity: ManagedIdentity,
                /// resource group the DNS zone is located in
                pub resource_group_name: String,
                /// ID of the Azure subscription
                pub subscription_i_d: String,
                /// when specifying ClientID and ClientSecret then this field is also needed
                pub tenant_i_d: String,
            }

            /// CA configures this issuer to sign certificates using a signing CA keypair stored in a Secret resource. This is used to build internal PKIs that are managed by cert-manager.
            pub struct Ca {
                /// The CRL distribution points is an X.509 v3 certificate extension which identifies the location of the CRL from which the revocation of this certificate can be checked. If not set, certificates will be issued without distribution points set.
                pub crl_distribution_points: Vec<String>,
                /// The OCSP server list is an X.509 v3 extension that defines a list of URLs of OCSP responders. The OCSP responders can be queried for the revocation status of an issued certificate. If not set, the certificate will be issued with no OCSP servers set. For example, an OCSP server URL could be "http://ocsp.int-x3.letsencrypt.org".
                pub ocsp_servers: Vec<String>,
                /// SecretName is the name of the secret used to sign Certificates issued by this Issuer.
                pub secret_name: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct AkamaiClientSecretSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// if both this and ClientID are left unset MSI will be used
            pub struct AzureDNSClientSecretSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct ClientTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Cloud specifies the Venafi cloud configuration settings. Only one of TPP or Cloud may be specified.
            pub struct Cloud {
                /// APITokenSecretRef is a secret key selector for the Venafi Cloud API token.
                pub api_token_secret_ref: CloudApiTokenSecretRef,
                /// URL is the base URL for Venafi Cloud. Defaults to "https://api.venafi.cloud/v1".
                pub url: String,
            }

            /// Use the Google Cloud DNS API to manage DNS01 challenge records.
            pub struct CloudDNS {
                /// HostedZoneName is an optional field that tells cert-manager in which Cloud DNS zone the challenge record has to be created. If left empty cert-manager will automatically choose a zone.
                pub hosted_zone_name: String,
                pub project: String,
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub service_account_secret_ref: ServiceAccountSecretRef,
            }

            /// Use the Cloudflare API to manage DNS01 challenge records.
            pub struct Cloudflare {
                /// API key to use to authenticate with Cloudflare. Note: using an API token to authenticate is now the recommended method as it allows greater control of permissions.
                pub api_key_secret_ref: ApiKeySecretRef,
                /// API token used to authenticate with Cloudflare.
                pub api_token_secret_ref: CloudflareApiTokenSecretRef,
                /// Email of the account, only required when using API key based authentication.
                pub email: String,
            }

            /// IssuerCondition contains condition information for an Issuer.
            pub struct Condition {
                /// LastTransitionTime is the timestamp corresponding to the last status change of this condition.
                pub last_transition_time: String,
                /// Message is a human readable description of the details of the last transition, complementing reason.
                pub message: String,
                /// If set, this represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.condition[x].observedGeneration is 9, the condition is out of date with respect to the current state of the Issuer.
                pub observed_generation: i64,
                /// Reason is a brief machine readable explanation for the condition's last transition.
                pub reason: String,
                /// Status of the condition, one of (`True`, `False`, `Unknown`).
                pub status: String,
                /// Type of the condition, known values are (`Ready`).
                pub r#type: String,
            }

            /// CredentialsRef is a reference to a Secret containing the username and password for the TPP server. The secret must contain two keys, 'username' and 'password'.
            pub struct CredentialsRef {
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Use the DigitalOcean DNS API to manage DNS01 challenge records.
            pub struct Digitalocean {
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub token_secret_ref: DigitaloceanTokenSecretRef,
            }

            /// Configures cert-manager to attempt to complete authorizations by performing the DNS01 challenge flow.
            pub struct Dns01 {
                /// Use the 'ACME DNS' (https://github.com/joohoi/acme-dns) API to manage DNS01 challenge records.
                pub acme_d_n_s: AcmeDNS,
                /// Use the Akamai DNS zone management API to manage DNS01 challenge records.
                pub akamai: Akamai,
                /// Use the Microsoft Azure DNS API to manage DNS01 challenge records.
                pub azure_d_n_s: AzureDNS,
                /// Use the Google Cloud DNS API to manage DNS01 challenge records.
                pub cloud_d_n_s: CloudDNS,
                /// Use the Cloudflare API to manage DNS01 challenge records.
                pub cloudflare: Cloudflare,
                /// CNAMEStrategy configures how the DNS01 provider should handle CNAME records when found in DNS zones.
                pub cname_strategy: String,
                /// Use the DigitalOcean DNS API to manage DNS01 challenge records.
                pub digitalocean: Digitalocean,
                /// Use RFC2136 ("Dynamic Updates in the Domain Name System") (https://datatracker.ietf.org/doc/rfc2136/) to manage DNS01 challenge records.
                pub rfc2136: Rfc2136,
                /// Use the AWS Route53 API to manage DNS01 challenge records.
                pub route53: Route53,
                /// Configure an external webhook based DNS01 challenge solver to manage DNS01 challenge records.
                pub webhook: Webhook,
            }

            /// ExternalAccountBinding is a reference to a CA external account of the ACME server. If set, upon registration cert-manager will attempt to associate the given external account credentials with the registered ACME account.
            pub struct ExternalAccountBinding {
                /// Deprecated: keyAlgorithm field exists for historical compatibility reasons and should not be used. The algorithm is now hardcoded to HS256 in golang/x/crypto/acme.
                pub key_algorithm: String,
                /// keyID is the ID of the CA key that the External Account is bound to.
                pub key_i_d: String,
                /// keySecretRef is a Secret Key Selector referencing a data item in a Kubernetes Secret which holds the symmetric MAC key of the External Account Binding. The `key` is the index string that is paired with the key data in the Secret and should not be confused with the key data itself, or indeed with the External Account Binding keyID above. The secret key stored in the Secret **must** be un-padded, base64 URL encoded data.
                pub key_secret_ref: KeySecretRef,
            }

            /// The Gateway API is a sig-network community API that models service networking in Kubernetes (https://gateway-api.sigs.k8s.io/). The Gateway solver will create HTTPRoutes with the specified labels in the same namespace as the challenge. This solver is experimental, and fields / behaviour may change in the future.
            pub struct GatewayHTTPRoute {
                /// Custom labels that will be applied to HTTPRoutes created by cert-manager while solving HTTP-01 challenges.
                pub labels: GatewayHTTPRouteLabels,
                /// When solving an HTTP-01 challenge, cert-manager creates an HTTPRoute. cert-manager needs to know which parentRefs should be used when creating the HTTPRoute. Usually, the parentRef references a Gateway. See: https://gateway-api.sigs.k8s.io/v1alpha2/api-types/httproute/#attaching-to-gateways
                pub parent_refs: Vec<ParentRef>,
                /// Optional service type for Kubernetes solver service. Supported values are NodePort or ClusterIP. If unset, defaults to NodePort.
                pub service_type: String,
            }

            /// Configures cert-manager to attempt to complete authorizations by performing the HTTP01 challenge flow. It is not possible to obtain certificates for wildcard domain names (e.g. `*.example.com`) using the HTTP01 challenge mechanism.
            pub struct Http01 {
                /// The Gateway API is a sig-network community API that models service networking in Kubernetes (https://gateway-api.sigs.k8s.io/). The Gateway solver will create HTTPRoutes with the specified labels in the same namespace as the challenge. This solver is experimental, and fields / behaviour may change in the future.
                pub gateway_h_t_t_p_route: GatewayHTTPRoute,
                /// The ingress based HTTP01 challenge solver will solve challenges by creating or modifying Ingress resources in order to route requests for '/.well-known/acme-challenge/XYZ' to 'challenge solver' pods that are provisioned by cert-manager for each Challenge to be completed.
                pub ingress: Ingress,
            }

            /// The ingress based HTTP01 challenge solver will solve challenges by creating or modifying Ingress resources in order to route requests for '/.well-known/acme-challenge/XYZ' to 'challenge solver' pods that are provisioned by cert-manager for each Challenge to be completed.
            pub struct Ingress {
                /// The ingress class to use when creating Ingress resources to solve ACME challenges that use this challenge solver. Only one of 'class' or 'name' may be specified.
                pub class: String,
                /// Optional ingress template used to configure the ACME challenge solver ingress used for HTTP01 challenges.
                pub ingress_template: IngressTemplate,
                /// The name of the ingress resource that should have ACME challenge solving routes inserted into it in order to solve HTTP01 challenges. This is typically used in conjunction with ingress controllers like ingress-gce, which maintains a 1:1 mapping between external IPs and ingress resources.
                pub name: String,
                /// Optional pod template used to configure the ACME challenge solver pods used for HTTP01 challenges.
                pub pod_template: PodTemplate,
                /// Optional service type for Kubernetes solver service. Supported values are NodePort or ClusterIP. If unset, defaults to NodePort.
                pub service_type: String,
            }

            /// Optional ingress template used to configure the ACME challenge solver ingress used for HTTP01 challenges.
            pub struct IngressTemplate {
                /// ObjectMeta overrides for the ingress used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
                pub metadata: IngressTemplateMetadata,
            }

            /// keySecretRef is a Secret Key Selector referencing a data item in a Kubernetes Secret which holds the symmetric MAC key of the External Account Binding. The `key` is the index string that is paired with the key data in the Secret and should not be confused with the key data itself, or indeed with the External Account Binding keyID above. The secret key stored in the Secret **must** be un-padded, base64 URL encoded data.
            pub struct KeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Kubernetes authenticates with Vault by passing the ServiceAccount token stored in the named Secret resource to the Vault server.
            pub struct Kubernetes {
                /// The Vault mountPath here is the mount path to use when authenticating with Vault. For example, setting a value to `/v1/auth/foo`, will use the path `/v1/auth/foo/login` to authenticate with Vault. If unspecified, the default value "/v1/auth/kubernetes" will be used.
                pub mount_path: String,
                /// A required field containing the Vault Role to assume. A Role binds a Kubernetes ServiceAccount with a set of Vault policies.
                pub role: String,
                /// The required Secret field containing a Kubernetes ServiceAccount JWT used for authenticating with Vault. Use of 'ambient credentials' is not supported.
                pub secret_ref: KubernetesSecretRef,
            }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels,
        }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels,
        }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels,
        }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels,
        }

            /// Custom labels that will be applied to HTTPRoutes created by cert-manager while solving HTTP-01 challenges.
            pub struct GatewayHTTPRouteLabels {
                /// Custom labels that will be applied to HTTPRoutes created by cert-manager while solving HTTP-01 challenges.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// Labels that should be added to the created ACME HTTP01 solver ingress.
            pub struct IngressTemplateMetadataLabels {
                /// Labels that should be added to the created ACME HTTP01 solver ingress.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// Labels that should be added to the created ACME HTTP01 solver pods.
            pub struct PodTemplateMetadataLabels {
                /// Labels that should be added to the created ACME HTTP01 solver pods.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// managed identity configuration, can not be used at the same time as clientID, clientSecretSecretRef or tenantID
            pub struct ManagedIdentity {
                /// client ID of the managed identity, can not be used at the same time as resourceID
                pub client_i_d: String,
                /// resource ID of the managed identity, can not be used at the same time as clientID
                pub resource_i_d: String,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PreferenceMatchExpression {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct NodeSelectorTermMatchExpression {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PreferenceMatchField {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct NodeSelectorTermMatchField {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// A label selector that is used to refine the set of certificate's that this challenge solver will apply to.
            pub struct SelectorMatchLabels {
                /// A label selector that is used to refine the set of certificate's that this challenge solver will apply to.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// ObjectMeta overrides for the ingress used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
            pub struct IngressTemplateMetadata {
                /// Annotations that should be added to the created ACME HTTP01 solver ingress.
                pub annotations: IngressTemplateMetadataAnnotations,
                /// Labels that should be added to the created ACME HTTP01 solver ingress.
                pub labels: IngressTemplateMetadataLabels,
            }

            /// ObjectMeta overrides for the pod used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
            pub struct PodTemplateMetadata {
                /// Annotations that should be added to the create ACME HTTP01 solver pods.
                pub annotations: PodTemplateMetadataAnnotations,
                /// Labels that should be added to the created ACME HTTP01 solver pods.
                pub labels: PodTemplateMetadataLabels,
            }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels,
        }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels,
        }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels,
        }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels,
        }

            /// Describes node affinity scheduling rules for the pod.
            pub struct NodeAffinity {
                /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
                pub preferred_during_scheduling_ignored_during_execution:
                    Vec<NodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem>,
                /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
                pub required_during_scheduling_ignored_during_execution:
                    RequiredDuringSchedulingIgnoredDuringExecution,
            }

            /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
            pub struct NodeSelector {
                /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
                pub properties: std::collections::HashMap<String, String>,
            }

            /// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
            pub struct NodeSelectorTerm {
                /// A list of node selector requirements by node's labels.
                pub match_expressions: Vec<NodeSelectorTermMatchExpression>,
                /// A list of node selector requirements by node's fields.
                pub match_fields: Vec<NodeSelectorTermMatchField>,
            }

            /// ParentRef identifies an API object (usually a Gateway) that can be considered a parent of this resource (usually a route). The only kind of parent resource with "Core" support is Gateway. This API may be extended in the future to support additional kinds of parent resources, such as HTTPRoute.
            ///  The API object must be valid in the cluster; the Group and Kind must be registered in the cluster for this reference to be valid.
            ///  References to objects with invalid Group and Kind are not valid, and must be rejected by the implementation, with appropriate Conditions set on the containing object.
            pub struct ParentRef {
                /// Group is the group of the referent.
                ///  Support: Core
                pub group: String,
                /// Kind is kind of the referent.
                ///  Support: Core (Gateway) Support: Custom (Other Resources)
                pub kind: String,
                /// Name is the name of the referent.
                ///  Support: Core
                pub name: String,
                /// Namespace is the namespace of the referent. When unspecified (or empty string), this refers to the local namespace of the Route.
                ///  Support: Core
                pub namespace: String,
                /// SectionName is the name of a section within the target resource. In the following resources, SectionName is interpreted as the following:
                ///  * Gateway: Listener Name
                ///  Implementations MAY choose to support attaching Routes to other resources. If that is the case, they MUST clearly document how SectionName is interpreted.
                ///  When unspecified (empty string), this will reference the entire resource. For the purpose of status, an attachment is considered successful if at least one section in the parent resource accepts it. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway.
                ///  Support: Core
                pub section_name: String,
            }

            /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
            pub struct PodAffinity {
                /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                pub preferred_during_scheduling_ignored_during_execution:
                    Vec<PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem>,
                /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                pub required_during_scheduling_ignored_during_execution:
                    Vec<PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem>,
            }

            /// Required. A pod affinity term, associated with the corresponding weight.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

            /// Required. A pod affinity term, associated with the corresponding weight.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

            /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
            pub struct PodAntiAffinity {
                /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                pub preferred_during_scheduling_ignored_during_execution:
                    Vec<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem>,
                /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                pub required_during_scheduling_ignored_during_execution:
                    Vec<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem>,
            }

            /// Optional pod template used to configure the ACME challenge solver pods used for HTTP01 challenges.
            pub struct PodTemplate {
                /// ObjectMeta overrides for the pod used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
                pub metadata: PodTemplateMetadata,
                /// PodSpec defines overrides for the HTTP01 challenge solver pod. Only the 'priorityClassName', 'nodeSelector', 'affinity', 'serviceAccountName' and 'tolerations' fields are supported currently. All other fields will be ignored.
                pub spec: PodTemplateSpec,
            }

            /// A node selector term, associated with the corresponding weight.
            pub struct Preference {
                /// A list of node selector requirements by node's labels.
                pub match_expressions: Vec<PreferenceMatchExpression>,
                /// A list of node selector requirements by node's fields.
                pub match_fields: Vec<PreferenceMatchField>,
            }

            /// An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
            pub struct NodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem {
                /// A node selector term, associated with the corresponding weight.
                pub preference: Preference,
                /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
                pub weight: i32,
            }

            /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem {
                /// Required. A pod affinity term, associated with the corresponding weight.
                pub pod_affinity_term:
                    PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm,
                /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
                pub weight: i32,
            }

            /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem {
            /// Required. A pod affinity term, associated with the corresponding weight.
            pub pod_affinity_term: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm,
            /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
            pub weight: i32,
        }

            /// PrivateKey is the name of a Kubernetes Secret resource that will be used to store the automatically generated ACME account private key. Optionally, a `key` may be specified to select a specific entry within the named Secret resource. If `key` is not specified, a default of `tls.key` will be used.
            pub struct PrivateKeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
            pub struct RequiredDuringSchedulingIgnoredDuringExecution {
                /// Required. A list of node selector terms. The terms are ORed.
                pub node_selector_terms: Vec<NodeSelectorTerm>,
            }

            /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem {
                /// A label query over a set of resources, in this case pods.
                pub label_selector:
                    PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector,
                /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
                pub namespace_selector:
                    PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector,
                /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
                pub namespaces: Vec<String>,
                /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
                pub topology_key: String,
            }

            /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

            /// Use RFC2136 ("Dynamic Updates in the Domain Name System") (https://datatracker.ietf.org/doc/rfc2136/) to manage DNS01 challenge records.
            pub struct Rfc2136 {
                /// The IP address or hostname of an authoritative DNS server supporting RFC2136 in the form host:port. If the host is an IPv6 address it must be enclosed in square brackets (e.g [2001:db8::1]) ; port is optional. This field is required.
                pub nameserver: String,
                /// The TSIG Algorithm configured in the DNS supporting RFC2136. Used only when ``tsigSecretSecretRef`` and ``tsigKeyName`` are defined. Supported values are (case-insensitive): ``HMACMD5`` (default), ``HMACSHA1``, ``HMACSHA256`` or ``HMACSHA512``.
                pub tsig_algorithm: String,
                /// The TSIG Key name configured in the DNS. If ``tsigSecretSecretRef`` is defined, this field is required.
                pub tsig_key_name: String,
                /// The name of the secret containing the TSIG value. If ``tsigKeyName`` is defined, this field is required.
                pub tsig_secret_secret_ref: TsigSecretSecretRef,
            }

            /// Use the AWS Route53 API to manage DNS01 challenge records.
            pub struct Route53 {
                /// The AccessKeyID is used for authentication. If not set we fall-back to using env vars, shared credentials file or AWS Instance metadata see: https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html#specifying-credentials
                pub access_key_i_d: String,
                /// If set, the provider will manage only this zone in Route53 and will not do an lookup using the route53:ListHostedZonesByName api call.
                pub hosted_zone_i_d: String,
                /// Always set the region when using AccessKeyID and SecretAccessKey
                pub region: String,
                /// Role is a Role ARN which the Route53 provider will assume using either the explicit credentials AccessKeyID/SecretAccessKey or the inferred credentials from environment variables, shared credentials file or AWS Instance metadata
                pub role: String,
                /// The SecretAccessKey is used for authentication. If not set we fall-back to using env vars, shared credentials file or AWS Instance metadata https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html#specifying-credentials
                pub secret_access_key_secret_ref: SecretAccessKeySecretRef,
            }

            /// The SecretAccessKey is used for authentication. If not set we fall-back to using env vars, shared credentials file or AWS Instance metadata https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html#specifying-credentials
            pub struct SecretAccessKeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Reference to a key in a Secret that contains the App Role secret used to authenticate with Vault. The `key` field must be specified and denotes which entry within the Secret resource is used as the app role secret.
            pub struct AppRoleSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// The required Secret field containing a Kubernetes ServiceAccount JWT used for authenticating with Vault. Use of 'ambient credentials' is not supported.
            pub struct KubernetesSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Selector selects a set of DNSNames on the Certificate resource that should be solved using this challenge solver. If not specified, the solver will be treated as the 'default' solver with the lowest priority, i.e. if any other solver has a more specific match, it will be used instead.
            pub struct Selector {
                /// List of DNSNames that this solver will be used to solve. If specified and a match is found, a dnsNames selector will take precedence over a dnsZones selector. If multiple solvers match with the same dnsNames value, the solver with the most matching labels in matchLabels will be selected. If neither has more matches, the solver defined earlier in the list will be selected.
                pub dns_names: Vec<String>,
                /// List of DNSZones that this solver will be used to solve. The most specific DNS zone match specified here will take precedence over other DNS zone matches, so a solver specifying sys.example.com will be selected over one specifying example.com for the domain www.sys.example.com. If multiple solvers match with the same dnsZones value, the solver with the most matching labels in matchLabels will be selected. If neither has more matches, the solver defined earlier in the list will be selected.
                pub dns_zones: Vec<String>,
                /// A label selector that is used to refine the set of certificate's that this challenge solver will apply to.
                pub match_labels: SelectorMatchLabels,
            }

            /// SelfSigned configures this issuer to 'self sign' certificates using the private key used to create the CertificateRequest object.
            pub struct SelfSigned {
                /// The CRL distribution points is an X.509 v3 certificate extension which identifies the location of the CRL from which the revocation of this certificate can be checked. If not set certificate will be issued without CDP. Values are strings.
                pub crl_distribution_points: Vec<String>,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct ServiceAccountSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// An ACMEChallengeSolver describes how to solve ACME challenges for the issuer it is part of. A selector may be provided to use different solving strategies for different DNS names. Only one of HTTP01 or DNS01 must be provided.
            pub struct Solver {
                /// Configures cert-manager to attempt to complete authorizations by performing the DNS01 challenge flow.
                pub dns01: Dns01,
                /// Configures cert-manager to attempt to complete authorizations by performing the HTTP01 challenge flow. It is not possible to obtain certificates for wildcard domain names (e.g. `*.example.com`) using the HTTP01 challenge mechanism.
                pub http01: Http01,
                /// Selector selects a set of DNSNames on the Certificate resource that should be solved using this challenge solver. If not specified, the solver will be treated as the 'default' solver with the lowest priority, i.e. if any other solver has a more specific match, it will be used instead.
                pub selector: Selector,
            }

            /// Desired state of the ClusterIssuer resource.
            pub struct Spec {
                /// ACME configures this issuer to communicate with a RFC8555 (ACME) server to obtain signed x509 certificates.
                pub acme: SpecAcme,
                /// CA configures this issuer to sign certificates using a signing CA keypair stored in a Secret resource. This is used to build internal PKIs that are managed by cert-manager.
                pub ca: Ca,
                /// SelfSigned configures this issuer to 'self sign' certificates using the private key used to create the CertificateRequest object.
                pub self_signed: SelfSigned,
                /// Vault configures this issuer to sign certificates using a HashiCorp Vault PKI backend.
                pub vault: Vault,
                /// Venafi configures this issuer to sign certificates using a Venafi TPP or Venafi Cloud policy zone.
                pub venafi: Venafi,
            }

            /// PodSpec defines overrides for the HTTP01 challenge solver pod. Only the 'priorityClassName', 'nodeSelector', 'affinity', 'serviceAccountName' and 'tolerations' fields are supported currently. All other fields will be ignored.
            pub struct PodTemplateSpec {
                /// If specified, the pod's scheduling constraints
                pub affinity: Affinity,
                /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
                pub node_selector: NodeSelector,
                /// If specified, the pod's priorityClassName.
                pub priority_class_name: String,
                /// If specified, the pod's service account
                pub service_account_name: String,
                /// If specified, the pod's tolerations.
                pub tolerations: Vec<Toleration>,
            }

            /// Status of the ClusterIssuer. This is set and managed automatically.
            pub struct Status {
                /// ACME specific status options. This field should only be set if the Issuer is configured to use an ACME server to issue certificates.
                pub acme: StatusAcme,
                /// List of status conditions to indicate the status of a CertificateRequest. Known condition types are `Ready`.
                pub conditions: Vec<Condition>,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct DigitaloceanTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// TokenSecretRef authenticates with Vault by presenting a token.
            pub struct AuthTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
            pub struct Toleration {
                /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
                pub effect: String,
                /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
                pub key: String,
                /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
                pub operator: String,
                /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
                pub toleration_seconds: i64,
                /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
                pub value: String,
            }

            /// TPP specifies Trust Protection Platform configuration settings. Only one of TPP or Cloud may be specified.
            pub struct Tpp {
                /// CABundle is a PEM encoded TLS certificate to use to verify connections to the TPP instance. If specified, system roots will not be used and the issuing CA for the TPP instance must be verifiable using the provided root. If not specified, the connection will be verified using the cert-manager system root certificates.
                pub ca_bundle: String,
                /// CredentialsRef is a reference to a Secret containing the username and password for the TPP server. The secret must contain two keys, 'username' and 'password'.
                pub credentials_ref: CredentialsRef,
                /// URL is the base URL for the vedsdk endpoint of the Venafi TPP instance, for example: "https://tpp.example.com/vedsdk".
                pub url: String,
            }

            /// The name of the secret containing the TSIG value. If ``tsigKeyName`` is defined, this field is required.
            pub struct TsigSecretSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Vault configures this issuer to sign certificates using a HashiCorp Vault PKI backend.
            pub struct Vault {
                /// Auth configures how cert-manager authenticates with the Vault server.
                pub auth: Auth,
                /// PEM-encoded CA bundle (base64-encoded) used to validate Vault server certificate. Only used if the Server URL is using HTTPS protocol. This parameter is ignored for plain HTTP protocol connection. If not set the system root certificates are used to validate the TLS connection.
                pub ca_bundle: String,
                /// Name of the vault namespace. Namespaces is a set of features within Vault Enterprise that allows Vault environments to support Secure Multi-tenancy. e.g: "ns1" More about namespaces can be found here https://www.vaultproject.io/docs/enterprise/namespaces
                pub namespace: String,
                /// Path is the mount path of the Vault PKI backend's `sign` endpoint, e.g: "my_pki_mount/sign/my-role-name".
                pub path: String,
                /// Server is the connection address for the Vault server, e.g: "https://vault.example.com:8200".
                pub server: String,
            }

            /// Venafi configures this issuer to sign certificates using a Venafi TPP or Venafi Cloud policy zone.
            pub struct Venafi {
                /// Cloud specifies the Venafi cloud configuration settings. Only one of TPP or Cloud may be specified.
                pub cloud: Cloud,
                /// TPP specifies Trust Protection Platform configuration settings. Only one of TPP or Cloud may be specified.
                pub tpp: Tpp,
                /// Zone is the Venafi Policy Zone to use for this issuer. All requests made to the Venafi platform will be restricted by the named zone policy. This field is required.
                pub zone: String,
            }

            /// Configure an external webhook based DNS01 challenge solver to manage DNS01 challenge records.
            pub struct Webhook {
                /// Additional configuration that should be passed to the webhook apiserver when challenges are processed. This can contain arbitrary JSON data. Secret values should not be specified in this stanza. If secret values are needed (e.g. credentials for a DNS service), you should use a SecretKeySelector to reference a Secret resource. For details on the schema of this field, consult the webhook provider implementation's documentation.
                pub config: std::collections::HashMap<String, String>,
                /// The API group name that should be used when POSTing ChallengePayload resources to the webhook apiserver. This should be the same as the GroupName specified in the webhook provider implementation.
                pub group_name: String,
                /// The name of the solver to use, as defined in the webhook provider implementation. This will typically be the name of the provider, e.g. 'cloudflare'.
                pub solver_name: String,
            }

            impl k8s_openapi::Resource for ClusterIssuer {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "cert-manager.io/v1";
                const GROUP: &'static str = "cert-manager.io";
                const KIND: &'static str = "ClusterIssuer";
                const VERSION: &'static str = "v1";
                const URL_PATH_SEGMENT: &'static str = "TODO";
            }

            impl k8s_openapi::Metadata for ClusterIssuer {
                type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

                fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                    &self.metadata
                }

                fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                    &mut self.metadata
                }
            }
        }
        pub mod issuer {
            /// An Issuer represents a certificate issuing authority which can be referenced as part of `issuerRef` fields. It is scoped to a single namespace and can therefore only be referenced by resources within the same namespace.
            pub struct Issuer {
                pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
                pub spec: Spec,
                pub status: Status,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct AccessTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct AccountSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// ACME configures this issuer to communicate with a RFC8555 (ACME) server to obtain signed x509 certificates.
            pub struct SpecAcme {
                /// Enables or disables generating a new ACME account key. If true, the Issuer resource will *not* request a new account but will expect the account key to be supplied via an existing secret. If false, the cert-manager system will generate a new ACME account key for the Issuer. Defaults to false.
                pub disable_account_key_generation: bool,
                /// Email is the email address to be associated with the ACME account. This field is optional, but it is strongly recommended to be set. It will be used to contact you in case of issues with your account or certificates, including expiry notification emails. This field may be updated after the account is initially registered.
                pub email: String,
                /// Enables requesting a Not After date on certificates that matches the duration of the certificate. This is not supported by all ACME servers like Let's Encrypt. If set to true when the ACME server does not support it it will create an error on the Order. Defaults to false.
                pub enable_duration_feature: bool,
                /// ExternalAccountBinding is a reference to a CA external account of the ACME server. If set, upon registration cert-manager will attempt to associate the given external account credentials with the registered ACME account.
                pub external_account_binding: ExternalAccountBinding,
                /// PreferredChain is the chain to use if the ACME server outputs multiple. PreferredChain is no guarantee that this one gets delivered by the ACME endpoint. For example, for Let's Encrypt's DST crosssign you would use: "DST Root CA X3" or "ISRG Root X1" for the newer Let's Encrypt root CA. This value picks the first certificate bundle in the ACME alternative chains that has a certificate with this value as its issuer's CN
                pub preferred_chain: String,
                /// PrivateKey is the name of a Kubernetes Secret resource that will be used to store the automatically generated ACME account private key. Optionally, a `key` may be specified to select a specific entry within the named Secret resource. If `key` is not specified, a default of `tls.key` will be used.
                pub private_key_secret_ref: PrivateKeySecretRef,
                /// Server is the URL used to access the ACME server's 'directory' endpoint. For example, for Let's Encrypt's staging endpoint, you would use: "https://acme-staging-v02.api.letsencrypt.org/directory". Only ACME v2 endpoints (i.e. RFC 8555) are supported.
                pub server: String,
                /// Enables or disables validation of the ACME server TLS certificate. If true, requests to the ACME server will not have their TLS certificate validated (i.e. insecure connections will be allowed). Only enable this option in development environments. The cert-manager system installed roots will be used to verify connections to the ACME server if this is false. Defaults to false.
                pub skip_t_l_s_verify: bool,
                /// Solvers is a list of challenge solvers that will be used to solve ACME challenges for the matching domains. Solver configurations must be provided in order to obtain certificates from an ACME server. For more information, see: https://cert-manager.io/docs/configuration/acme/
                pub solvers: Vec<Solver>,
            }

            /// ACME specific status options. This field should only be set if the Issuer is configured to use an ACME server to issue certificates.
            pub struct StatusAcme {
                /// LastRegisteredEmail is the email associated with the latest registered ACME account, in order to track changes made to registered account associated with the  Issuer
                pub last_registered_email: String,
                /// URI is the unique account identifier, which can also be used to retrieve account details from the CA
                pub uri: String,
            }

            /// Use the 'ACME DNS' (https://github.com/joohoi/acme-dns) API to manage DNS01 challenge records.
            pub struct AcmeDNS {
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub account_secret_ref: AccountSecretRef,
                pub host: String,
            }

            /// If specified, the pod's scheduling constraints
            pub struct Affinity {
                /// Describes node affinity scheduling rules for the pod.
                pub node_affinity: NodeAffinity,
                /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
                pub pod_affinity: PodAffinity,
                /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
                pub pod_anti_affinity: PodAntiAffinity,
            }

            /// Use the Akamai DNS zone management API to manage DNS01 challenge records.
            pub struct Akamai {
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub access_token_secret_ref: AccessTokenSecretRef,
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub client_secret_secret_ref: AkamaiClientSecretSecretRef,
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub client_token_secret_ref: ClientTokenSecretRef,
                pub service_consumer_domain: String,
            }

            /// Annotations that should be added to the created ACME HTTP01 solver ingress.
            pub struct IngressTemplateMetadataAnnotations {
                /// Annotations that should be added to the created ACME HTTP01 solver ingress.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// Annotations that should be added to the create ACME HTTP01 solver pods.
            pub struct PodTemplateMetadataAnnotations {
                /// Annotations that should be added to the create ACME HTTP01 solver pods.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// API key to use to authenticate with Cloudflare. Note: using an API token to authenticate is now the recommended method as it allows greater control of permissions.
            pub struct ApiKeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// API token used to authenticate with Cloudflare.
            pub struct CloudflareApiTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// APITokenSecretRef is a secret key selector for the Venafi Cloud API token.
            pub struct CloudApiTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// AppRole authenticates with Vault using the App Role auth mechanism, with the role and secret stored in a Kubernetes Secret resource.
            pub struct AppRole {
                /// Path where the App Role authentication backend is mounted in Vault, e.g: "approle"
                pub path: String,
                /// RoleID configured in the App Role authentication backend when setting up the authentication backend in Vault.
                pub role_id: String,
                /// Reference to a key in a Secret that contains the App Role secret used to authenticate with Vault. The `key` field must be specified and denotes which entry within the Secret resource is used as the app role secret.
                pub secret_ref: AppRoleSecretRef,
            }

            /// Auth configures how cert-manager authenticates with the Vault server.
            pub struct Auth {
                /// AppRole authenticates with Vault using the App Role auth mechanism, with the role and secret stored in a Kubernetes Secret resource.
                pub app_role: AppRole,
                /// Kubernetes authenticates with Vault by passing the ServiceAccount token stored in the named Secret resource to the Vault server.
                pub kubernetes: Kubernetes,
                /// TokenSecretRef authenticates with Vault by presenting a token.
                pub token_secret_ref: AuthTokenSecretRef,
            }

            /// Use the Microsoft Azure DNS API to manage DNS01 challenge records.
            pub struct AzureDNS {
                /// if both this and ClientSecret are left unset MSI will be used
                pub client_i_d: String,
                /// if both this and ClientID are left unset MSI will be used
                pub client_secret_secret_ref: AzureDNSClientSecretSecretRef,
                /// name of the Azure environment (default AzurePublicCloud)
                pub environment: String,
                /// name of the DNS zone that should be used
                pub hosted_zone_name: String,
                /// managed identity configuration, can not be used at the same time as clientID, clientSecretSecretRef or tenantID
                pub managed_identity: ManagedIdentity,
                /// resource group the DNS zone is located in
                pub resource_group_name: String,
                /// ID of the Azure subscription
                pub subscription_i_d: String,
                /// when specifying ClientID and ClientSecret then this field is also needed
                pub tenant_i_d: String,
            }

            /// CA configures this issuer to sign certificates using a signing CA keypair stored in a Secret resource. This is used to build internal PKIs that are managed by cert-manager.
            pub struct Ca {
                /// The CRL distribution points is an X.509 v3 certificate extension which identifies the location of the CRL from which the revocation of this certificate can be checked. If not set, certificates will be issued without distribution points set.
                pub crl_distribution_points: Vec<String>,
                /// The OCSP server list is an X.509 v3 extension that defines a list of URLs of OCSP responders. The OCSP responders can be queried for the revocation status of an issued certificate. If not set, the certificate will be issued with no OCSP servers set. For example, an OCSP server URL could be "http://ocsp.int-x3.letsencrypt.org".
                pub ocsp_servers: Vec<String>,
                /// SecretName is the name of the secret used to sign Certificates issued by this Issuer.
                pub secret_name: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct AkamaiClientSecretSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// if both this and ClientID are left unset MSI will be used
            pub struct AzureDNSClientSecretSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct ClientTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Cloud specifies the Venafi cloud configuration settings. Only one of TPP or Cloud may be specified.
            pub struct Cloud {
                /// APITokenSecretRef is a secret key selector for the Venafi Cloud API token.
                pub api_token_secret_ref: CloudApiTokenSecretRef,
                /// URL is the base URL for Venafi Cloud. Defaults to "https://api.venafi.cloud/v1".
                pub url: String,
            }

            /// Use the Google Cloud DNS API to manage DNS01 challenge records.
            pub struct CloudDNS {
                /// HostedZoneName is an optional field that tells cert-manager in which Cloud DNS zone the challenge record has to be created. If left empty cert-manager will automatically choose a zone.
                pub hosted_zone_name: String,
                pub project: String,
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub service_account_secret_ref: ServiceAccountSecretRef,
            }

            /// Use the Cloudflare API to manage DNS01 challenge records.
            pub struct Cloudflare {
                /// API key to use to authenticate with Cloudflare. Note: using an API token to authenticate is now the recommended method as it allows greater control of permissions.
                pub api_key_secret_ref: ApiKeySecretRef,
                /// API token used to authenticate with Cloudflare.
                pub api_token_secret_ref: CloudflareApiTokenSecretRef,
                /// Email of the account, only required when using API key based authentication.
                pub email: String,
            }

            /// IssuerCondition contains condition information for an Issuer.
            pub struct Condition {
                /// LastTransitionTime is the timestamp corresponding to the last status change of this condition.
                pub last_transition_time: String,
                /// Message is a human readable description of the details of the last transition, complementing reason.
                pub message: String,
                /// If set, this represents the .metadata.generation that the condition was set based upon. For instance, if .metadata.generation is currently 12, but the .status.condition[x].observedGeneration is 9, the condition is out of date with respect to the current state of the Issuer.
                pub observed_generation: i64,
                /// Reason is a brief machine readable explanation for the condition's last transition.
                pub reason: String,
                /// Status of the condition, one of (`True`, `False`, `Unknown`).
                pub status: String,
                /// Type of the condition, known values are (`Ready`).
                pub r#type: String,
            }

            /// CredentialsRef is a reference to a Secret containing the username and password for the TPP server. The secret must contain two keys, 'username' and 'password'.
            pub struct CredentialsRef {
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Use the DigitalOcean DNS API to manage DNS01 challenge records.
            pub struct Digitalocean {
                /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
                pub token_secret_ref: DigitaloceanTokenSecretRef,
            }

            /// Configures cert-manager to attempt to complete authorizations by performing the DNS01 challenge flow.
            pub struct Dns01 {
                /// Use the 'ACME DNS' (https://github.com/joohoi/acme-dns) API to manage DNS01 challenge records.
                pub acme_d_n_s: AcmeDNS,
                /// Use the Akamai DNS zone management API to manage DNS01 challenge records.
                pub akamai: Akamai,
                /// Use the Microsoft Azure DNS API to manage DNS01 challenge records.
                pub azure_d_n_s: AzureDNS,
                /// Use the Google Cloud DNS API to manage DNS01 challenge records.
                pub cloud_d_n_s: CloudDNS,
                /// Use the Cloudflare API to manage DNS01 challenge records.
                pub cloudflare: Cloudflare,
                /// CNAMEStrategy configures how the DNS01 provider should handle CNAME records when found in DNS zones.
                pub cname_strategy: String,
                /// Use the DigitalOcean DNS API to manage DNS01 challenge records.
                pub digitalocean: Digitalocean,
                /// Use RFC2136 ("Dynamic Updates in the Domain Name System") (https://datatracker.ietf.org/doc/rfc2136/) to manage DNS01 challenge records.
                pub rfc2136: Rfc2136,
                /// Use the AWS Route53 API to manage DNS01 challenge records.
                pub route53: Route53,
                /// Configure an external webhook based DNS01 challenge solver to manage DNS01 challenge records.
                pub webhook: Webhook,
            }

            /// ExternalAccountBinding is a reference to a CA external account of the ACME server. If set, upon registration cert-manager will attempt to associate the given external account credentials with the registered ACME account.
            pub struct ExternalAccountBinding {
                /// Deprecated: keyAlgorithm field exists for historical compatibility reasons and should not be used. The algorithm is now hardcoded to HS256 in golang/x/crypto/acme.
                pub key_algorithm: String,
                /// keyID is the ID of the CA key that the External Account is bound to.
                pub key_i_d: String,
                /// keySecretRef is a Secret Key Selector referencing a data item in a Kubernetes Secret which holds the symmetric MAC key of the External Account Binding. The `key` is the index string that is paired with the key data in the Secret and should not be confused with the key data itself, or indeed with the External Account Binding keyID above. The secret key stored in the Secret **must** be un-padded, base64 URL encoded data.
                pub key_secret_ref: KeySecretRef,
            }

            /// The Gateway API is a sig-network community API that models service networking in Kubernetes (https://gateway-api.sigs.k8s.io/). The Gateway solver will create HTTPRoutes with the specified labels in the same namespace as the challenge. This solver is experimental, and fields / behaviour may change in the future.
            pub struct GatewayHTTPRoute {
                /// Custom labels that will be applied to HTTPRoutes created by cert-manager while solving HTTP-01 challenges.
                pub labels: GatewayHTTPRouteLabels,
                /// When solving an HTTP-01 challenge, cert-manager creates an HTTPRoute. cert-manager needs to know which parentRefs should be used when creating the HTTPRoute. Usually, the parentRef references a Gateway. See: https://gateway-api.sigs.k8s.io/v1alpha2/api-types/httproute/#attaching-to-gateways
                pub parent_refs: Vec<ParentRef>,
                /// Optional service type for Kubernetes solver service. Supported values are NodePort or ClusterIP. If unset, defaults to NodePort.
                pub service_type: String,
            }

            /// Configures cert-manager to attempt to complete authorizations by performing the HTTP01 challenge flow. It is not possible to obtain certificates for wildcard domain names (e.g. `*.example.com`) using the HTTP01 challenge mechanism.
            pub struct Http01 {
                /// The Gateway API is a sig-network community API that models service networking in Kubernetes (https://gateway-api.sigs.k8s.io/). The Gateway solver will create HTTPRoutes with the specified labels in the same namespace as the challenge. This solver is experimental, and fields / behaviour may change in the future.
                pub gateway_h_t_t_p_route: GatewayHTTPRoute,
                /// The ingress based HTTP01 challenge solver will solve challenges by creating or modifying Ingress resources in order to route requests for '/.well-known/acme-challenge/XYZ' to 'challenge solver' pods that are provisioned by cert-manager for each Challenge to be completed.
                pub ingress: Ingress,
            }

            /// The ingress based HTTP01 challenge solver will solve challenges by creating or modifying Ingress resources in order to route requests for '/.well-known/acme-challenge/XYZ' to 'challenge solver' pods that are provisioned by cert-manager for each Challenge to be completed.
            pub struct Ingress {
                /// The ingress class to use when creating Ingress resources to solve ACME challenges that use this challenge solver. Only one of 'class' or 'name' may be specified.
                pub class: String,
                /// Optional ingress template used to configure the ACME challenge solver ingress used for HTTP01 challenges.
                pub ingress_template: IngressTemplate,
                /// The name of the ingress resource that should have ACME challenge solving routes inserted into it in order to solve HTTP01 challenges. This is typically used in conjunction with ingress controllers like ingress-gce, which maintains a 1:1 mapping between external IPs and ingress resources.
                pub name: String,
                /// Optional pod template used to configure the ACME challenge solver pods used for HTTP01 challenges.
                pub pod_template: PodTemplate,
                /// Optional service type for Kubernetes solver service. Supported values are NodePort or ClusterIP. If unset, defaults to NodePort.
                pub service_type: String,
            }

            /// Optional ingress template used to configure the ACME challenge solver ingress used for HTTP01 challenges.
            pub struct IngressTemplate {
                /// ObjectMeta overrides for the ingress used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
                pub metadata: IngressTemplateMetadata,
            }

            /// keySecretRef is a Secret Key Selector referencing a data item in a Kubernetes Secret which holds the symmetric MAC key of the External Account Binding. The `key` is the index string that is paired with the key data in the Secret and should not be confused with the key data itself, or indeed with the External Account Binding keyID above. The secret key stored in the Secret **must** be un-padded, base64 URL encoded data.
            pub struct KeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Kubernetes authenticates with Vault by passing the ServiceAccount token stored in the named Secret resource to the Vault server.
            pub struct Kubernetes {
                /// The Vault mountPath here is the mount path to use when authenticating with Vault. For example, setting a value to `/v1/auth/foo`, will use the path `/v1/auth/foo/login` to authenticate with Vault. If unspecified, the default value "/v1/auth/kubernetes" will be used.
                pub mount_path: String,
                /// A required field containing the Vault Role to assume. A Role binds a Kubernetes ServiceAccount with a set of Vault policies.
                pub role: String,
                /// The required Secret field containing a Kubernetes ServiceAccount JWT used for authenticating with Vault. Use of 'ambient credentials' is not supported.
                pub secret_ref: KubernetesSecretRef,
            }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels,
        }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels,
        }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels,
        }

            /// A label query over a set of resources, in this case pods.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels,
        }

            /// Custom labels that will be applied to HTTPRoutes created by cert-manager while solving HTTP-01 challenges.
            pub struct GatewayHTTPRouteLabels {
                /// Custom labels that will be applied to HTTPRoutes created by cert-manager while solving HTTP-01 challenges.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// Labels that should be added to the created ACME HTTP01 solver ingress.
            pub struct IngressTemplateMetadataLabels {
                /// Labels that should be added to the created ACME HTTP01 solver ingress.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// Labels that should be added to the created ACME HTTP01 solver pods.
            pub struct PodTemplateMetadataLabels {
                /// Labels that should be added to the created ACME HTTP01 solver pods.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// managed identity configuration, can not be used at the same time as clientID, clientSecretSecretRef or tenantID
            pub struct ManagedIdentity {
                /// client ID of the managed identity, can not be used at the same time as resourceID
                pub client_i_d: String,
                /// resource ID of the managed identity, can not be used at the same time as clientID
                pub resource_i_d: String,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PreferenceMatchExpression {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct NodeSelectorTermMatchExpression {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression
            {
                /// key is the label key that the selector applies to.
                pub key: String,
                /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
                pub operator: String,
                /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct PreferenceMatchField {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
            pub struct NodeSelectorTermMatchField {
                /// The label key that the selector applies to.
                pub key: String,
                /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
                pub operator: String,
                /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
                pub values: Vec<String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels
            {
                /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// A label selector that is used to refine the set of certificate's that this challenge solver will apply to.
            pub struct SelectorMatchLabels {
                /// A label selector that is used to refine the set of certificate's that this challenge solver will apply to.
                pub properties: std::collections::HashMap<String, String>,
            }

            /// ObjectMeta overrides for the ingress used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
            pub struct IngressTemplateMetadata {
                /// Annotations that should be added to the created ACME HTTP01 solver ingress.
                pub annotations: IngressTemplateMetadataAnnotations,
                /// Labels that should be added to the created ACME HTTP01 solver ingress.
                pub labels: IngressTemplateMetadataLabels,
            }

            /// ObjectMeta overrides for the pod used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
            pub struct PodTemplateMetadata {
                /// Annotations that should be added to the create ACME HTTP01 solver pods.
                pub annotations: PodTemplateMetadataAnnotations,
                /// Labels that should be added to the created ACME HTTP01 solver pods.
                pub labels: PodTemplateMetadataLabels,
            }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels,
        }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels,
        }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelectorMatchLabels,
        }

            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchExpression>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelectorMatchLabels,
        }

            /// Describes node affinity scheduling rules for the pod.
            pub struct NodeAffinity {
                /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
                pub preferred_during_scheduling_ignored_during_execution:
                    Vec<NodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem>,
                /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
                pub required_during_scheduling_ignored_during_execution:
                    RequiredDuringSchedulingIgnoredDuringExecution,
            }

            /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
            pub struct NodeSelector {
                /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
                pub properties: std::collections::HashMap<String, String>,
            }

            /// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
            pub struct NodeSelectorTerm {
                /// A list of node selector requirements by node's labels.
                pub match_expressions: Vec<NodeSelectorTermMatchExpression>,
                /// A list of node selector requirements by node's fields.
                pub match_fields: Vec<NodeSelectorTermMatchField>,
            }

            /// ParentRef identifies an API object (usually a Gateway) that can be considered a parent of this resource (usually a route). The only kind of parent resource with "Core" support is Gateway. This API may be extended in the future to support additional kinds of parent resources, such as HTTPRoute.
            ///  The API object must be valid in the cluster; the Group and Kind must be registered in the cluster for this reference to be valid.
            ///  References to objects with invalid Group and Kind are not valid, and must be rejected by the implementation, with appropriate Conditions set on the containing object.
            pub struct ParentRef {
                /// Group is the group of the referent.
                ///  Support: Core
                pub group: String,
                /// Kind is kind of the referent.
                ///  Support: Core (Gateway) Support: Custom (Other Resources)
                pub kind: String,
                /// Name is the name of the referent.
                ///  Support: Core
                pub name: String,
                /// Namespace is the namespace of the referent. When unspecified (or empty string), this refers to the local namespace of the Route.
                ///  Support: Core
                pub namespace: String,
                /// SectionName is the name of a section within the target resource. In the following resources, SectionName is interpreted as the following:
                ///  * Gateway: Listener Name
                ///  Implementations MAY choose to support attaching Routes to other resources. If that is the case, they MUST clearly document how SectionName is interpreted.
                ///  When unspecified (empty string), this will reference the entire resource. For the purpose of status, an attachment is considered successful if at least one section in the parent resource accepts it. For example, Gateway listeners can restrict which Routes can attach to them by Route kind, namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from the referencing Route, the Route MUST be considered successfully attached. If no Gateway listeners accept attachment from this Route, the Route MUST be considered detached from the Gateway.
                ///  Support: Core
                pub section_name: String,
            }

            /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
            pub struct PodAffinity {
                /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                pub preferred_during_scheduling_ignored_during_execution:
                    Vec<PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem>,
                /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                pub required_during_scheduling_ignored_during_execution:
                    Vec<PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem>,
            }

            /// Required. A pod affinity term, associated with the corresponding weight.
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

            /// Required. A pod affinity term, associated with the corresponding weight.
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermLabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTermNamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

            /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
            pub struct PodAntiAffinity {
                /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                pub preferred_during_scheduling_ignored_during_execution:
                    Vec<PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem>,
                /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                pub required_during_scheduling_ignored_during_execution:
                    Vec<PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem>,
            }

            /// Optional pod template used to configure the ACME challenge solver pods used for HTTP01 challenges.
            pub struct PodTemplate {
                /// ObjectMeta overrides for the pod used to solve HTTP01 challenges. Only the 'labels' and 'annotations' fields may be set. If labels or annotations overlap with in-built values, the values here will override the in-built values.
                pub metadata: PodTemplateMetadata,
                /// PodSpec defines overrides for the HTTP01 challenge solver pod. Only the 'priorityClassName', 'nodeSelector', 'affinity', 'serviceAccountName' and 'tolerations' fields are supported currently. All other fields will be ignored.
                pub spec: PodTemplateSpec,
            }

            /// A node selector term, associated with the corresponding weight.
            pub struct Preference {
                /// A list of node selector requirements by node's labels.
                pub match_expressions: Vec<PreferenceMatchExpression>,
                /// A list of node selector requirements by node's fields.
                pub match_fields: Vec<PreferenceMatchField>,
            }

            /// An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
            pub struct NodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem {
                /// A node selector term, associated with the corresponding weight.
                pub preference: Preference,
                /// Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
                pub weight: i32,
            }

            /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
            pub struct PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem {
                /// Required. A pod affinity term, associated with the corresponding weight.
                pub pod_affinity_term:
                    PodAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm,
                /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
                pub weight: i32,
            }

            /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
            pub struct PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItem {
            /// Required. A pod affinity term, associated with the corresponding weight.
            pub pod_affinity_term: PodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionItemPodAffinityTerm,
            /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
            pub weight: i32,
        }

            /// PrivateKey is the name of a Kubernetes Secret resource that will be used to store the automatically generated ACME account private key. Optionally, a `key` may be specified to select a specific entry within the named Secret resource. If `key` is not specified, a default of `tls.key` will be used.
            pub struct PrivateKeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
            pub struct RequiredDuringSchedulingIgnoredDuringExecution {
                /// Required. A list of node selector terms. The terms are ORed.
                pub node_selector_terms: Vec<NodeSelectorTerm>,
            }

            /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
            pub struct PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem {
                /// A label query over a set of resources, in this case pods.
                pub label_selector:
                    PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector,
                /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
                pub namespace_selector:
                    PodAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector,
                /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
                pub namespaces: Vec<String>,
                /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
                pub topology_key: String,
            }

            /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
            pub struct PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItem {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemLabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: PodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionItemNamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

            /// Use RFC2136 ("Dynamic Updates in the Domain Name System") (https://datatracker.ietf.org/doc/rfc2136/) to manage DNS01 challenge records.
            pub struct Rfc2136 {
                /// The IP address or hostname of an authoritative DNS server supporting RFC2136 in the form host:port. If the host is an IPv6 address it must be enclosed in square brackets (e.g [2001:db8::1]) ; port is optional. This field is required.
                pub nameserver: String,
                /// The TSIG Algorithm configured in the DNS supporting RFC2136. Used only when ``tsigSecretSecretRef`` and ``tsigKeyName`` are defined. Supported values are (case-insensitive): ``HMACMD5`` (default), ``HMACSHA1``, ``HMACSHA256`` or ``HMACSHA512``.
                pub tsig_algorithm: String,
                /// The TSIG Key name configured in the DNS. If ``tsigSecretSecretRef`` is defined, this field is required.
                pub tsig_key_name: String,
                /// The name of the secret containing the TSIG value. If ``tsigKeyName`` is defined, this field is required.
                pub tsig_secret_secret_ref: TsigSecretSecretRef,
            }

            /// Use the AWS Route53 API to manage DNS01 challenge records.
            pub struct Route53 {
                /// The AccessKeyID is used for authentication. If not set we fall-back to using env vars, shared credentials file or AWS Instance metadata see: https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html#specifying-credentials
                pub access_key_i_d: String,
                /// If set, the provider will manage only this zone in Route53 and will not do an lookup using the route53:ListHostedZonesByName api call.
                pub hosted_zone_i_d: String,
                /// Always set the region when using AccessKeyID and SecretAccessKey
                pub region: String,
                /// Role is a Role ARN which the Route53 provider will assume using either the explicit credentials AccessKeyID/SecretAccessKey or the inferred credentials from environment variables, shared credentials file or AWS Instance metadata
                pub role: String,
                /// The SecretAccessKey is used for authentication. If not set we fall-back to using env vars, shared credentials file or AWS Instance metadata https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html#specifying-credentials
                pub secret_access_key_secret_ref: SecretAccessKeySecretRef,
            }

            /// The SecretAccessKey is used for authentication. If not set we fall-back to using env vars, shared credentials file or AWS Instance metadata https://docs.aws.amazon.com/sdk-for-go/v1/developer-guide/configuring-sdk.html#specifying-credentials
            pub struct SecretAccessKeySecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Reference to a key in a Secret that contains the App Role secret used to authenticate with Vault. The `key` field must be specified and denotes which entry within the Secret resource is used as the app role secret.
            pub struct AppRoleSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// The required Secret field containing a Kubernetes ServiceAccount JWT used for authenticating with Vault. Use of 'ambient credentials' is not supported.
            pub struct KubernetesSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Selector selects a set of DNSNames on the Certificate resource that should be solved using this challenge solver. If not specified, the solver will be treated as the 'default' solver with the lowest priority, i.e. if any other solver has a more specific match, it will be used instead.
            pub struct Selector {
                /// List of DNSNames that this solver will be used to solve. If specified and a match is found, a dnsNames selector will take precedence over a dnsZones selector. If multiple solvers match with the same dnsNames value, the solver with the most matching labels in matchLabels will be selected. If neither has more matches, the solver defined earlier in the list will be selected.
                pub dns_names: Vec<String>,
                /// List of DNSZones that this solver will be used to solve. The most specific DNS zone match specified here will take precedence over other DNS zone matches, so a solver specifying sys.example.com will be selected over one specifying example.com for the domain www.sys.example.com. If multiple solvers match with the same dnsZones value, the solver with the most matching labels in matchLabels will be selected. If neither has more matches, the solver defined earlier in the list will be selected.
                pub dns_zones: Vec<String>,
                /// A label selector that is used to refine the set of certificate's that this challenge solver will apply to.
                pub match_labels: SelectorMatchLabels,
            }

            /// SelfSigned configures this issuer to 'self sign' certificates using the private key used to create the CertificateRequest object.
            pub struct SelfSigned {
                /// The CRL distribution points is an X.509 v3 certificate extension which identifies the location of the CRL from which the revocation of this certificate can be checked. If not set certificate will be issued without CDP. Values are strings.
                pub crl_distribution_points: Vec<String>,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct ServiceAccountSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// An ACMEChallengeSolver describes how to solve ACME challenges for the issuer it is part of. A selector may be provided to use different solving strategies for different DNS names. Only one of HTTP01 or DNS01 must be provided.
            pub struct Solver {
                /// Configures cert-manager to attempt to complete authorizations by performing the DNS01 challenge flow.
                pub dns01: Dns01,
                /// Configures cert-manager to attempt to complete authorizations by performing the HTTP01 challenge flow. It is not possible to obtain certificates for wildcard domain names (e.g. `*.example.com`) using the HTTP01 challenge mechanism.
                pub http01: Http01,
                /// Selector selects a set of DNSNames on the Certificate resource that should be solved using this challenge solver. If not specified, the solver will be treated as the 'default' solver with the lowest priority, i.e. if any other solver has a more specific match, it will be used instead.
                pub selector: Selector,
            }

            /// Desired state of the Issuer resource.
            pub struct Spec {
                /// ACME configures this issuer to communicate with a RFC8555 (ACME) server to obtain signed x509 certificates.
                pub acme: SpecAcme,
                /// CA configures this issuer to sign certificates using a signing CA keypair stored in a Secret resource. This is used to build internal PKIs that are managed by cert-manager.
                pub ca: Ca,
                /// SelfSigned configures this issuer to 'self sign' certificates using the private key used to create the CertificateRequest object.
                pub self_signed: SelfSigned,
                /// Vault configures this issuer to sign certificates using a HashiCorp Vault PKI backend.
                pub vault: Vault,
                /// Venafi configures this issuer to sign certificates using a Venafi TPP or Venafi Cloud policy zone.
                pub venafi: Venafi,
            }

            /// PodSpec defines overrides for the HTTP01 challenge solver pod. Only the 'priorityClassName', 'nodeSelector', 'affinity', 'serviceAccountName' and 'tolerations' fields are supported currently. All other fields will be ignored.
            pub struct PodTemplateSpec {
                /// If specified, the pod's scheduling constraints
                pub affinity: Affinity,
                /// NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
                pub node_selector: NodeSelector,
                /// If specified, the pod's priorityClassName.
                pub priority_class_name: String,
                /// If specified, the pod's service account
                pub service_account_name: String,
                /// If specified, the pod's tolerations.
                pub tolerations: Vec<Toleration>,
            }

            /// Status of the Issuer. This is set and managed automatically.
            pub struct Status {
                /// ACME specific status options. This field should only be set if the Issuer is configured to use an ACME server to issue certificates.
                pub acme: StatusAcme,
                /// List of status conditions to indicate the status of a CertificateRequest. Known condition types are `Ready`.
                pub conditions: Vec<Condition>,
            }

            /// A reference to a specific 'key' within a Secret resource. In some instances, `key` is a required field.
            pub struct DigitaloceanTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// TokenSecretRef authenticates with Vault by presenting a token.
            pub struct AuthTokenSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
            pub struct Toleration {
                /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
                pub effect: String,
                /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
                pub key: String,
                /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
                pub operator: String,
                /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
                pub toleration_seconds: i64,
                /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
                pub value: String,
            }

            /// TPP specifies Trust Protection Platform configuration settings. Only one of TPP or Cloud may be specified.
            pub struct Tpp {
                /// CABundle is a PEM encoded TLS certificate to use to verify connections to the TPP instance. If specified, system roots will not be used and the issuing CA for the TPP instance must be verifiable using the provided root. If not specified, the connection will be verified using the cert-manager system root certificates.
                pub ca_bundle: String,
                /// CredentialsRef is a reference to a Secret containing the username and password for the TPP server. The secret must contain two keys, 'username' and 'password'.
                pub credentials_ref: CredentialsRef,
                /// URL is the base URL for the vedsdk endpoint of the Venafi TPP instance, for example: "https://tpp.example.com/vedsdk".
                pub url: String,
            }

            /// The name of the secret containing the TSIG value. If ``tsigKeyName`` is defined, this field is required.
            pub struct TsigSecretSecretRef {
                /// The key of the entry in the Secret resource's `data` field to be used. Some instances of this field may be defaulted, in others it may be required.
                pub key: String,
                /// Name of the resource being referred to. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                pub name: String,
            }

            /// Vault configures this issuer to sign certificates using a HashiCorp Vault PKI backend.
            pub struct Vault {
                /// Auth configures how cert-manager authenticates with the Vault server.
                pub auth: Auth,
                /// PEM-encoded CA bundle (base64-encoded) used to validate Vault server certificate. Only used if the Server URL is using HTTPS protocol. This parameter is ignored for plain HTTP protocol connection. If not set the system root certificates are used to validate the TLS connection.
                pub ca_bundle: String,
                /// Name of the vault namespace. Namespaces is a set of features within Vault Enterprise that allows Vault environments to support Secure Multi-tenancy. e.g: "ns1" More about namespaces can be found here https://www.vaultproject.io/docs/enterprise/namespaces
                pub namespace: String,
                /// Path is the mount path of the Vault PKI backend's `sign` endpoint, e.g: "my_pki_mount/sign/my-role-name".
                pub path: String,
                /// Server is the connection address for the Vault server, e.g: "https://vault.example.com:8200".
                pub server: String,
            }

            /// Venafi configures this issuer to sign certificates using a Venafi TPP or Venafi Cloud policy zone.
            pub struct Venafi {
                /// Cloud specifies the Venafi cloud configuration settings. Only one of TPP or Cloud may be specified.
                pub cloud: Cloud,
                /// TPP specifies Trust Protection Platform configuration settings. Only one of TPP or Cloud may be specified.
                pub tpp: Tpp,
                /// Zone is the Venafi Policy Zone to use for this issuer. All requests made to the Venafi platform will be restricted by the named zone policy. This field is required.
                pub zone: String,
            }

            /// Configure an external webhook based DNS01 challenge solver to manage DNS01 challenge records.
            pub struct Webhook {
                /// Additional configuration that should be passed to the webhook apiserver when challenges are processed. This can contain arbitrary JSON data. Secret values should not be specified in this stanza. If secret values are needed (e.g. credentials for a DNS service), you should use a SecretKeySelector to reference a Secret resource. For details on the schema of this field, consult the webhook provider implementation's documentation.
                pub config: std::collections::HashMap<String, String>,
                /// The API group name that should be used when POSTing ChallengePayload resources to the webhook apiserver. This should be the same as the GroupName specified in the webhook provider implementation.
                pub group_name: String,
                /// The name of the solver to use, as defined in the webhook provider implementation. This will typically be the name of the provider, e.g. 'cloudflare'.
                pub solver_name: String,
            }

            impl k8s_openapi::Resource for Issuer {
                type Scope = k8s_openapi::ClusterResourceScope;

                const API_VERSION: &'static str = "cert-manager.io/v1";
                const GROUP: &'static str = "cert-manager.io";
                const KIND: &'static str = "Issuer";
                const VERSION: &'static str = "v1";
                const URL_PATH_SEGMENT: &'static str = "TODO";
            }

            impl k8s_openapi::Metadata for Issuer {
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
