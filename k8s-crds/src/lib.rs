pub mod monitoring_coreos_com {
    pub mod v1 {
        pub mod alertmanager {
        /// Alertmanager describes an Alertmanager cluster.
        pub struct Alertmanager {
            pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            pub spec: Spec,
            pub status: Status,
        }

        /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
        pub struct AccessModes {
        }

        /// Added capabilities
        pub struct Add {
        }

        /// AdditionalPeers allows injecting a set of additional Alertmanagers to peer with to form a highly available cluster.
        pub struct AdditionalPeers {
        }

        /// If specified, the pod's scheduling constraints.
        pub struct Affinity {
            /// Describes node affinity scheduling rules for the pod.
            pub node_affinity: NodeAffinity,
            /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
            pub pod_affinity: PodAffinity,
            /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
            pub pod_anti_affinity: PodAntiAffinity,
        }

        /// Namespaces to be selected for AlertmanagerConfig discovery. If nil, only check own namespace.
        pub struct AlertmanagerConfigNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// AlertmanagerConfigs to be selected for to merge and configure Alertmanager with.
        pub struct AlertmanagerConfigSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// EXPERIMENTAL: alertmanagerConfiguration specifies the global Alertmanager configuration. If defined, it takes precedence over the `configSecret` field. This field may change in future releases.
        pub struct AlertmanagerConfiguration {
            /// The name of the AlertmanagerConfig resource which is used to generate the global configuration. It must be defined in the same namespace as the Alertmanager object. The operator will not enforce a `namespace` label for routes and inhibition rules.
            pub name: String,
        }

        /// The storage resource within AllocatedResources tracks the capacity allocated to a PVC. It may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
        pub struct AllocatedResources {
        }

        /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
        pub struct Annotations {
        }

        /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
        pub struct Args {
        }

        /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
        pub struct AwsElasticBlockStore {
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
            pub partition: i32,
            /// Specify "true" to force and set the ReadOnly property in VolumeMounts to "true". If omitted, the default is "false". More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
            pub read_only: bool,
            /// Unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
            pub volume_i_d: String,
        }

        /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
        pub struct AzureDisk {
            /// Host Caching mode: None, Read Only, Read Write.
            pub caching_mode: String,
            /// The Name of the data disk in the blob storage
            pub disk_name: String,
            /// The URI the data disk in the blob storage
            pub disk_u_r_i: String,
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
            pub kind: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
        }

        /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
        pub struct AzureFile {
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// the name of secret that contains Azure Storage Account Name and Key
            pub secret_name: String,
            /// Share Name
            pub share_name: String,
        }

        /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
        pub struct Capabilities {
            /// Added capabilities
            pub add: Vec<String>,
            /// Removed capabilities
            pub drop: Vec<String>,
        }

        /// Represents the actual resources of the underlying volume.
        pub struct Capacity {
        }

        /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
        pub struct Cephfs {
            /// Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub monitors: Vec<String>,
            /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
            pub path: String,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub read_only: bool,
            /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub secret_file: String,
            /// Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub secret_ref: SecretRef,
            /// Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub user: String,
        }

        /// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
        pub struct Cinder {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub fs_type: String,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub read_only: bool,
            /// Optional: points to a secret object containing parameters used to connect to OpenStack.
            pub secret_ref: SecretRef,
            /// volume id used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub volume_i_d: String,
        }

        /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
        pub struct Command {
        }

        /// Current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'.
        pub struct Conditions {
        }

        /// PersistentVolumeClaimCondition contails details about state of pvc
        pub struct ConditionsItem {
            /// Last time we probed the condition.
            pub last_probe_time: String,
            /// Last time the condition transitioned from one status to another.
            pub last_transition_time: String,
            /// Human-readable message indicating details about last transition.
            pub message: String,
            /// Unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports "ResizeStarted" that means the underlying persistent volume is being resized.
            pub reason: String,
            pub status: String,
            /// PersistentVolumeClaimConditionType is a valid value of PersistentVolumeClaimCondition.Type
            pub r#type: String,
        }

        /// information about the configMap data to project
        pub struct ConfigMap {
            /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
            pub items: Vec<ItemsItem>,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its keys must be defined
            pub optional: bool,
        }

        /// Selects a key of a ConfigMap.
        pub struct ConfigMapKeyRef {
            /// The key to select.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its key must be defined
            pub optional: bool,
        }

        /// The ConfigMap to select from
        pub struct ConfigMapRef {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap must be defined
            pub optional: bool,
        }

        /// ConfigMaps is a list of ConfigMaps in the same namespace as the Alertmanager object, which shall be mounted into the Alertmanager Pods. The ConfigMaps are mounted into /etc/alertmanager/configmaps/<configmap-name>.
        pub struct ConfigMaps {
        }

        /// Containers allows injecting additional containers. This is meant to allow adding an authentication proxy to an Alertmanager pod. Containers described here modify an operator generated container if they share the same name and modifications are done via a strategic merge patch. The current container names are: `alertmanager` and `config-reloader`. Overriding containers is entirely outside the scope of what the maintainers will support and by doing so, you accept that this behaviour may break at any time without notice.
        pub struct Containers {
        }

        /// A single application container that you want to run within a pod.
        pub struct ContainersItem {
            /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub args: Vec<String>,
            /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub command: Vec<String>,
            /// List of environment variables to set in the container. Cannot be updated.
            pub env: Vec<EnvItem>,
            /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
            pub env_from: Vec<EnvFromItem>,
            /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
            pub image: String,
            /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
            pub image_pull_policy: String,
            /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
            pub lifecycle: Lifecycle,
            /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub liveness_probe: LivenessProbe,
            /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
            pub name: String,
            /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
            pub ports: Vec<PortsItem>,
            /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub readiness_probe: ReadinessProbe,
            /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub resources: Resources,
            /// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
            pub security_context: SecurityContext,
            /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub startup_probe: StartupProbe,
            /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
            pub stdin: bool,
            /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
            pub stdin_once: bool,
            /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
            pub termination_message_path: String,
            /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
            pub termination_message_policy: String,
            /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
            pub tty: bool,
            /// volumeDevices is the list of block devices to be used by the container.
            pub volume_devices: Vec<VolumeDevicesItem>,
            /// Pod volumes to mount into the container's filesystem. Cannot be updated.
            pub volume_mounts: Vec<VolumeMountsItem>,
            /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
            pub working_dir: String,
        }

        /// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
        pub struct Csi {
            /// Driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
            pub driver: String,
            /// Filesystem type to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
            pub fs_type: String,
            /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
            pub node_publish_secret_ref: NodePublishSecretRef,
            /// Specifies a read-only configuration for the volume. Defaults to false (read/write).
            pub read_only: bool,
            /// VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
            pub volume_attributes: VolumeAttributes,
        }

        /// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.
        pub struct DataSource {
            /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
            pub api_group: String,
            /// Kind is the type of resource being referenced
            pub kind: String,
            /// Name is the name of resource being referenced
            pub name: String,
        }

        /// Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef allows any non-core object, as well as PersistentVolumeClaim objects. * While DataSource ignores disallowed values (dropping them), DataSourceRef preserves all values, and generates an error if a disallowed value is specified. (Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
        pub struct DataSourceRef {
            /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
            pub api_group: String,
            /// Kind is the type of resource being referenced
            pub kind: String,
            /// Name is the name of resource being referenced
            pub name: String,
        }

        /// information about the downwardAPI data to project
        pub struct DownwardAPI {
            /// Items is a list of DownwardAPIVolume file
            pub items: Vec<ItemsItem>,
        }

        /// Removed capabilities
        pub struct Drop {
        }

        /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
        pub struct EmptyDir {
            /// What type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
            pub medium: String,
            /// Total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: http://kubernetes.io/docs/user-guide/volumes#emptydir
            pub size_limit: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        }

        /// List of environment variables to set in the container. Cannot be updated.
        pub struct Env {
        }

        /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
        pub struct EnvFrom {
        }

        /// EnvFromSource represents the source of a set of ConfigMaps
        pub struct EnvFromItem {
            /// The ConfigMap to select from
            pub config_map_ref: ConfigMapRef,
            /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
            pub prefix: String,
            /// The Secret to select from
            pub secret_ref: SecretRef,
        }

        /// EnvVar represents an environment variable present in a Container.
        pub struct EnvItem {
            /// Name of the environment variable. Must be a C_IDENTIFIER.
            pub name: String,
            /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
            pub value: String,
            /// Source for the environment variable's value. Cannot be used if value is not empty.
            pub value_from: ValueFrom,
        }

        /// Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed. 
        ///  Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity tracking are needed, c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through a PersistentVolumeClaim (see EphemeralVolumeSource for more information on the connection between this volume type and PersistentVolumeClaim). 
        ///  Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod. 
        ///  Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information. 
        ///  A pod can use both types of ephemeral volumes and persistent volumes at the same time.
        pub struct Ephemeral {
            /// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long). 
            ///  An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster. 
            ///  This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created. 
            ///  Required, must not be nil.
            pub volume_claim_template: VolumeClaimTemplate,
        }

        /// Exec specifies the action to take.
        pub struct Exec {
            /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
            pub command: Vec<String>,
        }

        /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
        pub struct Fc {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// Optional: FC target lun number
            pub lun: i32,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// Optional: FC target worldwide names (WWNs)
            pub target_w_w_ns: Vec<String>,
            /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
            pub wwids: Vec<String>,
        }

        /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
        pub struct FieldRef {
            /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
            pub api_version: String,
            /// Path of the field to select in the specified API version.
            pub field_path: String,
        }

        /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
        pub struct FlexVolume {
            /// Driver is the name of the driver to use for this volume.
            pub driver: String,
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
            pub fs_type: String,
            /// Optional: Extra command options if any.
            pub options: Options,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
            pub secret_ref: SecretRef,
        }

        /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
        pub struct Flocker {
            /// Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
            pub dataset_name: String,
            /// UUID of the dataset. This is unique identifier of a Flocker dataset
            pub dataset_u_u_i_d: String,
        }

        /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
        pub struct GcePersistentDisk {
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub partition: i32,
            /// Unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub pd_name: String,
            /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub read_only: bool,
        }

        /// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
        pub struct GitRepo {
            /// Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
            pub directory: String,
            /// Repository URL
            pub repository: String,
            /// Commit hash for the specified revision.
            pub revision: String,
        }

        /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
        pub struct Glusterfs {
            /// EndpointsName is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
            pub endpoints: String,
            /// Path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
            pub path: String,
            /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
            pub read_only: bool,
        }

        /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
        pub struct Grpc {
            /// Port number of the gRPC service. Number must be in the range 1 to 65535.
            pub port: i32,
            /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
            ///  If this is not specified, the default behavior is defined by gRPC.
            pub service: String,
        }

        /// Pods' hostAliases configuration
        pub struct HostAliases {
        }

        /// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
        pub struct HostAliasesItem {
            /// Hostnames for the above IP address.
            pub hostnames: Vec<String>,
            /// IP address of the host file entry.
            pub ip: String,
        }

        /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath --- TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not mount host directories as read/write.
        pub struct HostPath {
            /// Path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
            pub path: String,
            /// Type for HostPath Volume Defaults to "" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
            pub r#type: String,
        }

        /// Hostnames for the above IP address.
        pub struct Hostnames {
        }

        /// HTTPGet specifies the http request to perform.
        pub struct HttpGet {
            /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
            pub host: String,
            /// Custom headers to set in the request. HTTP allows repeated headers.
            pub http_headers: Vec<HttpHeadersItem>,
            /// Path to access on the HTTP server.
            pub path: String,
            /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
            pub port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            /// Scheme to use for connecting to the host. Defaults to HTTP.
            pub scheme: String,
        }

        /// Custom headers to set in the request. HTTP allows repeated headers.
        pub struct HttpHeaders {
        }

        /// HTTPHeader describes a custom header to be used in HTTP probes
        pub struct HttpHeadersItem {
            /// The header field name
            pub name: String,
            /// The header field value
            pub value: String,
        }

        /// An optional list of references to secrets in the same namespace to use for pulling prometheus and alertmanager images from registries see http://kubernetes.io/docs/user-guide/images#specifying-imagepullsecrets-on-a-pod
        pub struct ImagePullSecrets {
        }

        /// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
        pub struct ImagePullSecretsItem {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
        }

        /// InitContainers allows adding initContainers to the pod definition. Those can be used to e.g. fetch secrets for injection into the Alertmanager configuration from external sources. Any errors during the execution of an initContainer will lead to a restart of the Pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/ Using initContainers for any use case other then secret fetching is entirely outside the scope of what the maintainers will support and by doing so, you accept that this behaviour may break at any time without notice.
        pub struct InitContainers {
        }

        /// A single application container that you want to run within a pod.
        pub struct InitContainersItem {
            /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub args: Vec<String>,
            /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub command: Vec<String>,
            /// List of environment variables to set in the container. Cannot be updated.
            pub env: Vec<EnvItem>,
            /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
            pub env_from: Vec<EnvFromItem>,
            /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
            pub image: String,
            /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
            pub image_pull_policy: String,
            /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
            pub lifecycle: Lifecycle,
            /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub liveness_probe: LivenessProbe,
            /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
            pub name: String,
            /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
            pub ports: Vec<PortsItem>,
            /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub readiness_probe: ReadinessProbe,
            /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub resources: Resources,
            /// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
            pub security_context: SecurityContext,
            /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub startup_probe: StartupProbe,
            /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
            pub stdin: bool,
            /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
            pub stdin_once: bool,
            /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
            pub termination_message_path: String,
            /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
            pub termination_message_policy: String,
            /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
            pub tty: bool,
            /// volumeDevices is the list of block devices to be used by the container.
            pub volume_devices: Vec<VolumeDevicesItem>,
            /// Pod volumes to mount into the container's filesystem. Cannot be updated.
            pub volume_mounts: Vec<VolumeMountsItem>,
            /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
            pub working_dir: String,
        }

        /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
        pub struct Iscsi {
            /// whether support iSCSI Discovery CHAP authentication
            pub chap_auth_discovery: bool,
            /// whether support iSCSI Session CHAP authentication
            pub chap_auth_session: bool,
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// Custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
            pub initiator_name: String,
            /// Target iSCSI Qualified Name.
            pub iqn: String,
            /// iSCSI Interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
            pub iscsi_interface: String,
            /// iSCSI Target Lun number.
            pub lun: i32,
            /// iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
            pub portals: Vec<String>,
            /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
            pub read_only: bool,
            /// CHAP Secret for iSCSI target and initiator authentication
            pub secret_ref: SecretRef,
            /// iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
            pub target_portal: String,
        }

        /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
        pub struct Items {
        }

        /// Maps a string key to a path within a volume.
        pub struct ItemsItem {
            /// The key to project.
            pub key: String,
            /// Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
            pub mode: i32,
            /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
            pub path: String,
        }

        /// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
        pub struct LabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
        pub struct Labels {
        }

        /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
        pub struct Lifecycle {
            /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
            pub post_start: PostStart,
            /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
            pub pre_stop: PreStop,
        }

        /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
        pub struct Limits {
        }

        /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
        pub struct LivenessProbe {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
            pub failure_threshold: i32,
            /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
            pub grpc: Grpc,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub initial_delay_seconds: i32,
            /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
            pub period_seconds: i32,
            /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
            pub success_threshold: i32,
            /// TCPSocket specifies an action involving a TCP port.
            pub tcp_socket: TcpSocket,
            /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
            pub termination_grace_period_seconds: i64,
            /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub timeout_seconds: i32,
        }

        /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
        pub struct MatchExpressions {
        }

        /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
        pub struct MatchExpressionsItem {
            /// key is the label key that the selector applies to.
            pub key: String,
            /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
            pub operator: String,
            /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
            pub values: Vec<String>,
        }

        /// A list of node selector requirements by node's fields.
        pub struct MatchFields {
        }

        /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
        pub struct MatchFieldsItem {
            /// The label key that the selector applies to.
            pub key: String,
            /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
            pub operator: String,
            /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
            pub values: Vec<String>,
        }

        /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
        pub struct MatchLabels {
        }

        /// May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
        pub struct Metadata {
        }

        /// A collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
        pub struct Monitors {
        }

        /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
        pub struct NamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
        pub struct Namespaces {
        }

        /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
        pub struct Nfs {
            /// Path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub path: String,
            /// ReadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub read_only: bool,
            /// Server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub server: String,
        }

        /// Describes node affinity scheduling rules for the pod.
        pub struct NodeAffinity {
            /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
            pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredDuringSchedulingIgnoredDuringExecutionItem>,
            /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
            pub required_during_scheduling_ignored_during_execution: RequiredDuringSchedulingIgnoredDuringExecution,
        }

        /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
        pub struct NodePublishSecretRef {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
        }

        /// Define which Nodes the Pods are scheduled on.
        pub struct NodeSelector {
        }

        /// Required. A list of node selector terms. The terms are ORed.
        pub struct NodeSelectorTerms {
        }

        /// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
        pub struct NodeSelectorTermsItem {
            /// A list of node selector requirements by node's labels.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// A list of node selector requirements by node's fields.
            pub match_fields: Vec<MatchFieldsItem>,
        }

        /// Optional: Extra command options if any.
        pub struct Options {
        }

        /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
        pub struct PersistentVolumeClaim {
            /// ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
            pub claim_name: String,
            /// Will force the ReadOnly setting in VolumeMounts. Default false.
            pub read_only: bool,
        }

        /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
        pub struct PhotonPersistentDisk {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// ID that identifies Photon Controller persistent disk
            pub pd_i_d: String,
        }

        /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
        pub struct PodAffinity {
            /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
            pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredDuringSchedulingIgnoredDuringExecutionItem>,
            /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
            pub required_during_scheduling_ignored_during_execution: Vec<RequiredDuringSchedulingIgnoredDuringExecutionItem>,
        }

        /// Required. A pod affinity term, associated with the corresponding weight.
        pub struct PodAffinityTerm {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: LabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: NamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

        /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
        pub struct PodAntiAffinity {
            /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
            pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredDuringSchedulingIgnoredDuringExecutionItem>,
            /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
            pub required_during_scheduling_ignored_during_execution: Vec<RequiredDuringSchedulingIgnoredDuringExecutionItem>,
        }

        /// PodMetadata configures Labels and Annotations which are propagated to the alertmanager pods.
        pub struct PodMetadata {
            /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
            pub annotations: Annotations,
            /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
            pub labels: Labels,
            /// Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names
            pub name: String,
        }

        /// iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
        pub struct Portals {
        }

        /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
        pub struct Ports {
        }

        /// ContainerPort represents a network port in a single container.
        pub struct PortsItem {
            /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
            pub container_port: i32,
            /// What host IP to bind the external port to.
            pub host_i_p: String,
            /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
            pub host_port: i32,
            /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
            pub name: String,
            /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
            pub protocol: String,
        }

        /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
        pub struct PortworxVolume {
            /// FSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// VolumeID uniquely identifies a Portworx volume
            pub volume_i_d: String,
        }

        /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
        pub struct PostStart {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
            pub tcp_socket: TcpSocket,
        }

        /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
        pub struct PreStop {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
            pub tcp_socket: TcpSocket,
        }

        /// A node selector term, associated with the corresponding weight.
        pub struct Preference {
            /// A list of node selector requirements by node's labels.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// A list of node selector requirements by node's fields.
            pub match_fields: Vec<MatchFieldsItem>,
        }

        /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
        pub struct PreferredDuringSchedulingIgnoredDuringExecution {
        }

        /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
        pub struct PreferredDuringSchedulingIgnoredDuringExecutionItem {
            /// Required. A pod affinity term, associated with the corresponding weight.
            pub pod_affinity_term: PodAffinityTerm,
            /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
            pub weight: i32,
        }

        /// Items for all in one resources secrets, configmaps, and downward API
        pub struct Projected {
            /// Mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
            pub default_mode: i32,
            /// list of volume projections
            pub sources: Vec<SourcesItem>,
        }

        /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
        pub struct Quobyte {
            /// Group to map volume access to Default is no group
            pub group: String,
            /// ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
            pub read_only: bool,
            /// Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
            pub registry: String,
            /// Tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin
            pub tenant: String,
            /// User to map volume access to Defaults to serivceaccount user
            pub user: String,
            /// Volume is a string that references an already created Quobyte volume by name.
            pub volume: String,
        }

        /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
        pub struct Rbd {
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// The rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub image: String,
            /// Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub keyring: String,
            /// A collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub monitors: Vec<String>,
            /// The rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub pool: String,
            /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub read_only: bool,
            /// SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub secret_ref: SecretRef,
            /// The rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub user: String,
        }

        /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
        pub struct ReadinessProbe {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
            pub failure_threshold: i32,
            /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
            pub grpc: Grpc,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub initial_delay_seconds: i32,
            /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
            pub period_seconds: i32,
            /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
            pub success_threshold: i32,
            /// TCPSocket specifies an action involving a TCP port.
            pub tcp_socket: TcpSocket,
            /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
            pub termination_grace_period_seconds: i64,
            /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub timeout_seconds: i32,
        }

        /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
        pub struct Requests {
        }

        /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
        pub struct RequiredDuringSchedulingIgnoredDuringExecution {
        }

        /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
        pub struct RequiredDuringSchedulingIgnoredDuringExecutionItem {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: LabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: NamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

        /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
        pub struct ResourceFieldRef {
            /// Container name: required for volumes, optional for env vars
            pub container_name: String,
            /// Specifies the output format of the exposed resources, defaults to "1"
            pub divisor: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            /// Required: resource to select
            pub resource: String,
        }

        /// Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
        pub struct Resources {
            /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub limits: Limits,
            /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub requests: Requests,
        }

        /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
        pub struct ScaleIO {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
            pub fs_type: String,
            /// The host address of the ScaleIO API Gateway.
            pub gateway: String,
            /// The name of the ScaleIO Protection Domain for the configured storage.
            pub protection_domain: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
            pub secret_ref: SecretRef,
            /// Flag to enable/disable SSL communication with Gateway, default false
            pub ssl_enabled: bool,
            /// Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
            pub storage_mode: String,
            /// The ScaleIO Storage Pool associated with the protection domain.
            pub storage_pool: String,
            /// The name of the storage system as configured in ScaleIO.
            pub system: String,
            /// The name of a volume already created in the ScaleIO system that is associated with this volume source.
            pub volume_name: String,
        }

        /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
        pub struct SeLinuxOptions {
            /// Level is SELinux level label that applies to the container.
            pub level: String,
            /// Role is a SELinux role label that applies to the container.
            pub role: String,
            /// Type is a SELinux type label that applies to the container.
            pub r#type: String,
            /// User is a SELinux user label that applies to the container.
            pub user: String,
        }

        /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
        pub struct SeccompProfile {
            /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
            pub localhost_profile: String,
            /// type indicates which kind of seccomp profile will be applied. Valid options are: 
            ///  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
            pub r#type: String,
        }

        /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
        pub struct Secret {
            /// Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
            pub default_mode: i32,
            /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
            pub items: Vec<ItemsItem>,
            /// Specify whether the Secret or its keys must be defined
            pub optional: bool,
            /// Name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
            pub secret_name: String,
        }

        /// Selects a key of a secret in the pod's namespace
        pub struct SecretKeyRef {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
        pub struct SecretRef {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
        }

        /// Secrets is a list of Secrets in the same namespace as the Alertmanager object, which shall be mounted into the Alertmanager Pods. The Secrets are mounted into /etc/alertmanager/secrets/<secret-name>.
        pub struct Secrets {
        }

        /// SecurityContext holds pod-level security attributes and common container settings. This defaults to the default PodSecurityContext.
        pub struct SecurityContext {
            /// A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod: 
            ///  1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw---- 
            ///  If unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows.
            pub fs_group: i64,
            /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used. Note that this field cannot be set when spec.os.name is windows.
            pub fs_group_change_policy: String,
            /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
            pub run_as_group: i64,
            /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
            pub run_as_non_root: bool,
            /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
            pub run_as_user: i64,
            /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
            pub se_linux_options: SeLinuxOptions,
            /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
            pub seccomp_profile: SeccompProfile,
            /// A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container. Note that this field cannot be set when spec.os.name is windows.
            pub supplemental_groups: Vec<i64>,
            /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
            pub sysctls: Vec<SysctlsItem>,
            /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
            pub windows_options: WindowsOptions,
        }

        /// A label query over volumes to consider for binding.
        pub struct Selector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// information about the serviceAccountToken data to project
        pub struct ServiceAccountToken {
            /// Audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.
            pub audience: String,
            /// ExpirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.
            pub expiration_seconds: i64,
            /// Path is the path relative to the mount point of the file to project the token into.
            pub path: String,
        }

        /// list of volume projections
        pub struct Sources {
        }

        /// Projection that may be projected along with other supported volume types
        pub struct SourcesItem {
            /// information about the configMap data to project
            pub config_map: ConfigMap,
            /// information about the downwardAPI data to project
            pub downward_a_p_i: DownwardAPI,
            /// information about the secret data to project
            pub secret: Secret,
            /// information about the serviceAccountToken data to project
            pub service_account_token: ServiceAccountToken,
        }

        /// The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
        pub struct Spec {
            /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
            pub access_modes: Vec<String>,
            /// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.
            pub data_source: DataSource,
            /// Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef allows any non-core object, as well as PersistentVolumeClaim objects. * While DataSource ignores disallowed values (dropping them), DataSourceRef preserves all values, and generates an error if a disallowed value is specified. (Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
            pub data_source_ref: DataSourceRef,
            /// Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
            pub resources: Resources,
            /// A label query over volumes to consider for binding.
            pub selector: Selector,
            /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
            pub storage_class_name: String,
            /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
            pub volume_mode: String,
            /// VolumeName is the binding reference to the PersistentVolume backing this claim.
            pub volume_name: String,
        }

        /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
        pub struct StartupProbe {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
            pub failure_threshold: i32,
            /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
            pub grpc: Grpc,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub initial_delay_seconds: i32,
            /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
            pub period_seconds: i32,
            /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
            pub success_threshold: i32,
            /// TCPSocket specifies an action involving a TCP port.
            pub tcp_socket: TcpSocket,
            /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
            pub termination_grace_period_seconds: i64,
            /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub timeout_seconds: i32,
        }

        /// Most recent observed status of the Alertmanager cluster. Read-only. Not included when requesting from the apiserver, only from the Prometheus Operator API itself. More info: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
        pub struct Status {
            /// Total number of available pods (ready for at least minReadySeconds) targeted by this Alertmanager cluster.
            pub available_replicas: i32,
            /// Represents whether any actions on the underlying managed objects are being performed. Only delete actions will be performed.
            pub paused: bool,
            /// Total number of non-terminated pods targeted by this Alertmanager cluster (their labels match the selector).
            pub replicas: i32,
            /// Total number of unavailable pods targeted by this Alertmanager cluster.
            pub unavailable_replicas: i32,
            /// Total number of non-terminated pods targeted by this Alertmanager cluster that have the desired version spec.
            pub updated_replicas: i32,
        }

        /// Storage is the definition of how storage will be used by the Alertmanager instances.
        pub struct Storage {
            /// Deprecated: subPath usage will be disabled by default in a future release, this option will become unnecessary. DisableMountSubPath allows to remove any subPath usage in volume mounts.
            pub disable_mount_sub_path: bool,
            /// EmptyDirVolumeSource to be used by the Prometheus StatefulSets. If specified, used in place of any volumeClaimTemplate. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir
            pub empty_dir: EmptyDir,
            /// EphemeralVolumeSource to be used by the Prometheus StatefulSets. This is a beta field in k8s 1.21, for lower versions, starting with k8s 1.19, it requires enabling the GenericEphemeralVolume feature gate. More info: https://kubernetes.io/docs/concepts/storage/ephemeral-volumes/#generic-ephemeral-volumes
            pub ephemeral: Ephemeral,
            /// A PVC spec to be used by the Prometheus StatefulSets.
            pub volume_claim_template: VolumeClaimTemplate,
        }

        /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
        pub struct Storageos {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
            pub secret_ref: SecretRef,
            /// VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
            pub volume_name: String,
            /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
            pub volume_namespace: String,
        }

        /// A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container. Note that this field cannot be set when spec.os.name is windows.
        pub struct SupplementalGroups {
        }

        /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
        pub struct Sysctls {
        }

        /// Sysctl defines a kernel parameter to be set
        pub struct SysctlsItem {
            /// Name of a property to set
            pub name: String,
            /// Value of a property to set
            pub value: String,
        }

        /// Optional: FC target worldwide names (WWNs)
        pub struct TargetWWNs {
        }

        /// TCPSocket specifies an action involving a TCP port.
        pub struct TcpSocket {
            /// Optional: Host name to connect to, defaults to the pod IP.
            pub host: String,
            /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
            pub port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        }

        /// If specified, the pod's tolerations.
        pub struct Tolerations {
        }

        /// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
        pub struct TolerationsItem {
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

        /// If specified, the pod's topology spread constraints.
        pub struct TopologySpreadConstraints {
        }

        /// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
        pub struct TopologySpreadConstraintsItem {
            /// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
            pub label_selector: LabelSelector,
            /// MaxSkew describes the degree to which pods may be unevenly distributed. When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference between the number of matching pods in the target topology and the global minimum. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 1/1/0: | zone1 | zone2 | zone3 | |   P   |   P   |       | - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 1/1/1; scheduling it onto zone1(zone2) would make the ActualSkew(2-0) on zone1(zone2) violate MaxSkew(1). - if MaxSkew is 2, incoming pod can be scheduled onto any zone. When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence to topologies that satisfy it. It's a required field. Default value is 1 and 0 is not allowed.
            pub max_skew: i32,
            /// TopologyKey is the key of node labels. Nodes that have a label with this key and identical values are considered to be in the same topology. We consider each <key, value> as a "bucket", and try to put balanced number of pods into each bucket. It's a required field.
            pub topology_key: String,
            /// WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint. - DoNotSchedule (default) tells the scheduler not to schedule it. - ScheduleAnyway tells the scheduler to schedule the pod in any location, but giving higher precedence to topologies that would help reduce the skew. A constraint is considered "Unsatisfiable" for an incoming pod if and only if every possible node assignment for that pod would violate "MaxSkew" on some topology. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 3/1/1: | zone1 | zone2 | zone3 | | P P P |   P   |   P   | If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler won't make it *more* imbalanced. It's a required field.
            pub when_unsatisfiable: String,
        }

        /// Source for the environment variable's value. Cannot be used if value is not empty.
        pub struct ValueFrom {
            /// Selects a key of a ConfigMap.
            pub config_map_key_ref: ConfigMapKeyRef,
            /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
            pub field_ref: FieldRef,
            /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
            pub resource_field_ref: ResourceFieldRef,
            /// Selects a key of a secret in the pod's namespace
            pub secret_key_ref: SecretKeyRef,
        }

        /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
        pub struct Values {
        }

        /// VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
        pub struct VolumeAttributes {
        }

        /// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long). 
        ///  An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster. 
        ///  This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created. 
        ///  Required, must not be nil.
        pub struct VolumeClaimTemplate {
            /// May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
            pub metadata: Metadata,
            /// The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
            pub spec: Spec,
        }

        /// volumeDevices is the list of block devices to be used by the container.
        pub struct VolumeDevices {
        }

        /// volumeDevice describes a mapping of a raw block device within a container.
        pub struct VolumeDevicesItem {
            /// devicePath is the path inside of the container that the device will be mapped to.
            pub device_path: String,
            /// name must match the name of a persistentVolumeClaim in the pod
            pub name: String,
        }

        /// VolumeMounts allows configuration of additional VolumeMounts on the output StatefulSet definition. VolumeMounts specified will be appended to other VolumeMounts in the alertmanager container, that are generated as a result of StorageSpec objects.
        pub struct VolumeMounts {
        }

        /// VolumeMount describes a mounting of a Volume within a container.
        pub struct VolumeMountsItem {
            /// Path within the container at which the volume should be mounted.  Must not contain ':'.
            pub mount_path: String,
            /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
            pub mount_propagation: String,
            /// This must match the Name of a Volume.
            pub name: String,
            /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
            pub read_only: bool,
            /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
            pub sub_path: String,
            /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
            pub sub_path_expr: String,
        }

        /// Volumes allows configuration of additional volumes on the output StatefulSet definition. Volumes specified will be appended to other volumes that are generated as a result of StorageSpec objects.
        pub struct Volumes {
        }

        /// Volume represents a named volume in a pod that may be accessed by any container in the pod.
        pub struct VolumesItem {
            /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
            pub aws_elastic_block_store: AwsElasticBlockStore,
            /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
            pub azure_disk: AzureDisk,
            /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
            pub azure_file: AzureFile,
            /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
            pub cephfs: Cephfs,
            /// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub cinder: Cinder,
            /// ConfigMap represents a configMap that should populate this volume
            pub config_map: ConfigMap,
            /// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
            pub csi: Csi,
            /// DownwardAPI represents downward API about the pod that should populate this volume
            pub downward_a_p_i: DownwardAPI,
            /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
            pub empty_dir: EmptyDir,
            /// Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed. 
            ///  Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity tracking are needed, c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through a PersistentVolumeClaim (see EphemeralVolumeSource for more information on the connection between this volume type and PersistentVolumeClaim). 
            ///  Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod. 
            ///  Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information. 
            ///  A pod can use both types of ephemeral volumes and persistent volumes at the same time.
            pub ephemeral: Ephemeral,
            /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
            pub fc: Fc,
            /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
            pub flex_volume: FlexVolume,
            /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
            pub flocker: Flocker,
            /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub gce_persistent_disk: GcePersistentDisk,
            /// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
            pub git_repo: GitRepo,
            /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
            pub glusterfs: Glusterfs,
            /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath --- TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not mount host directories as read/write.
            pub host_path: HostPath,
            /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
            pub iscsi: Iscsi,
            /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
            pub name: String,
            /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub nfs: Nfs,
            /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
            pub persistent_volume_claim: PersistentVolumeClaim,
            /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
            pub photon_persistent_disk: PhotonPersistentDisk,
            /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
            pub portworx_volume: PortworxVolume,
            /// Items for all in one resources secrets, configmaps, and downward API
            pub projected: Projected,
            /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
            pub quobyte: Quobyte,
            /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
            pub rbd: Rbd,
            /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
            pub scale_i_o: ScaleIO,
            /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
            pub secret: Secret,
            /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
            pub storageos: Storageos,
            /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
            pub vsphere_volume: VsphereVolume,
        }

        /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
        pub struct VsphereVolume {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
            pub storage_policy_i_d: String,
            /// Storage Policy Based Management (SPBM) profile name.
            pub storage_policy_name: String,
            /// Path that identifies vSphere volume vmdk
            pub volume_path: String,
        }

        /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
        pub struct WindowsOptions {
            /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
            pub gmsa_credential_spec: String,
            /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
            pub gmsa_credential_spec_name: String,
            /// HostProcess determines if a container should be run as a 'Host Process' container. This field is alpha-level and will only be honored by components that enable the WindowsHostProcessContainers feature flag. Setting this field without the feature flag will result in errors when validating the Pod. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).  In addition, if HostProcess is true then HostNetwork must also be set to true.
            pub host_process: bool,
            /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
            pub run_as_user_name: String,
        }

        /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
        pub struct Wwids {
        }

        impl k8s_openapi::Resource for Alertmanager {
            type Scope = k8s_openapi::ClusterResourceScope;

            const API_VERSION : &'static str = "monitoring.coreos.com/v1";
            const GROUP : &'static str = "monitoring.coreos.com";
            const KIND : &'static str = "Alertmanager";
            const VERSION : &'static str = "v1";
            const URL_PATH_SEGMENT : &'static str = "TODO";
        }

        impl k8s_openapi::Metadata for Alertmanager {
            type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

            fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                &self.metadata
            }

            fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                &mut self.metadata
            }
        }

}
        pub mod pod_monitor {
        /// PodMonitor defines monitoring for a set of pods.
        pub struct PodMonitor {
            pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            pub spec: Spec,
        }

        /// Attaches node metadata to discovered targets. Only valid for role: pod. Only valid in Prometheus versions 2.35.0 and newer.
        pub struct AttachMetadata {
            /// When set to true, Prometheus must have permissions to get Nodes.
            pub node: bool,
        }

        /// Authorization section for this endpoint
        pub struct Authorization {
            /// The secret's key that contains the credentials of the request
            pub credentials: Credentials,
            /// Set the authentication type. Defaults to Bearer, Basic will cause an error
            pub r#type: String,
        }

        /// BasicAuth allow an endpoint to authenticate over basic authentication. More info: https://prometheus.io/docs/operating/configuration/#endpoint
        pub struct BasicAuth {
            /// The secret in the service monitor namespace that contains the password for authentication.
            pub password: Password,
            /// The secret in the service monitor namespace that contains the username for authentication.
            pub username: Username,
        }

        /// Secret to mount to read bearer token for scraping targets. The secret needs to be in the same namespace as the pod monitor and accessible by the Prometheus Operator.
        pub struct BearerTokenSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Struct containing the CA cert to use for the targets.
        pub struct Ca {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// Struct containing the client cert file for the targets.
        pub struct Cert {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret or configmap containing the OAuth2 client id
        pub struct ClientId {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret containing the OAuth2 client secret
        pub struct ClientSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// ConfigMap containing data to use for the targets.
        pub struct ConfigMap {
            /// The key to select.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its key must be defined
            pub optional: bool,
        }

        /// The secret's key that contains the credentials of the request
        pub struct Credentials {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Parameters to append to the token URL
        pub struct EndpointParams {
        }

        /// Secret containing the client key file for the targets.
        pub struct KeySecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
        pub struct MatchExpressions {
        }

        /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
        pub struct MatchExpressionsItem {
            /// key is the label key that the selector applies to.
            pub key: String,
            /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
            pub operator: String,
            /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
            pub values: Vec<String>,
        }

        /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
        pub struct MatchLabels {
        }

        /// List of namespace names to select from.
        pub struct MatchNames {
        }

        pub struct Metadata {
        }

        /// MetricRelabelConfigs to apply to samples before ingestion.
        pub struct MetricRelabelings {
        }

        /// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion. It defines `<metric_relabel_configs>`-section of Prometheus configuration. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
        pub struct MetricRelabelingsItem {
            /// Action to perform based on regex matching. Default is 'replace'
            pub action: String,
            /// Modulus to take of the hash of the source label values.
            pub modulus: i64,
            /// Regular expression against which the extracted value is matched. Default is '(.*)'
            pub regex: String,
            /// Replacement value against which a regex replace is performed if the regular expression matches. Regex capture groups are available. Default is '$1'
            pub replacement: String,
            /// Separator placed between concatenated source label values. default is ';'.
            pub separator: String,
            /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
            pub source_labels: Vec<String>,
            /// Label to which the resulting value is written in a replace action. It is mandatory for replace actions. Regex capture groups are available.
            pub target_label: String,
        }

        /// Selector to select which namespaces the Endpoints objects are discovered from.
        pub struct NamespaceSelector {
            /// Boolean describing whether all namespaces are selected in contrast to a list restricting them.
            pub any: bool,
            /// List of namespace names to select from.
            pub match_names: Vec<String>,
        }

        /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
        pub struct Oauth2 {
            /// The secret or configmap containing the OAuth2 client id
            pub client_id: ClientId,
            /// The secret containing the OAuth2 client secret
            pub client_secret: ClientSecret,
            /// Parameters to append to the token URL
            pub endpoint_params: EndpointParams,
            /// OAuth2 scopes used for the token request
            pub scopes: Vec<String>,
            /// The URL to fetch the token from
            pub token_url: String,
        }

        /// Optional HTTP URL parameters
        pub struct Params {
        }

        /// The secret in the service monitor namespace that contains the password for authentication.
        pub struct Password {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// A list of endpoints allowed as part of this PodMonitor.
        pub struct PodMetricsEndpoints {
        }

        /// PodMetricsEndpoint defines a scrapeable endpoint of a Kubernetes Pod serving Prometheus metrics.
        pub struct PodMetricsEndpointsItem {
            /// Authorization section for this endpoint
            pub authorization: Authorization,
            /// BasicAuth allow an endpoint to authenticate over basic authentication. More info: https://prometheus.io/docs/operating/configuration/#endpoint
            pub basic_auth: BasicAuth,
            /// Secret to mount to read bearer token for scraping targets. The secret needs to be in the same namespace as the pod monitor and accessible by the Prometheus Operator.
            pub bearer_token_secret: BearerTokenSecret,
            /// FollowRedirects configures whether scrape requests follow HTTP 3xx redirects.
            pub follow_redirects: bool,
            /// HonorLabels chooses the metric's labels on collisions with target labels.
            pub honor_labels: bool,
            /// HonorTimestamps controls whether Prometheus respects the timestamps present in scraped data.
            pub honor_timestamps: bool,
            /// Interval at which metrics should be scraped If not specified Prometheus' global scrape interval is used.
            pub interval: String,
            /// MetricRelabelConfigs to apply to samples before ingestion.
            pub metric_relabelings: Vec<MetricRelabelingsItem>,
            /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
            pub oauth2: Oauth2,
            /// Optional HTTP URL parameters
            pub params: Params,
            /// HTTP path to scrape for metrics.
            pub path: String,
            /// Name of the pod port this endpoint refers to. Mutually exclusive with targetPort.
            pub port: String,
            /// ProxyURL eg http://proxyserver:2195 Directs scrapes to proxy through this endpoint.
            pub proxy_url: String,
            /// RelabelConfigs to apply to samples before scraping. Prometheus Operator automatically adds relabelings for a few standard Kubernetes fields. The original scrape job's name is available via the `__tmp_prometheus_job_name` label. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
            pub relabelings: Vec<RelabelingsItem>,
            /// HTTP scheme to use for scraping.
            pub scheme: String,
            /// Timeout after which the scrape is ended If not specified, the Prometheus global scrape interval is used.
            pub scrape_timeout: String,
            /// Deprecated: Use 'port' instead.
            pub target_port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            /// TLS configuration to use when scraping the endpoint.
            pub tls_config: TlsConfig,
        }

        /// PodTargetLabels transfers labels on the Kubernetes Pod onto the target.
        pub struct PodTargetLabels {
        }

        /// RelabelConfigs to apply to samples before scraping. Prometheus Operator automatically adds relabelings for a few standard Kubernetes fields. The original scrape job's name is available via the `__tmp_prometheus_job_name` label. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
        pub struct Relabelings {
        }

        /// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion. It defines `<metric_relabel_configs>`-section of Prometheus configuration. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
        pub struct RelabelingsItem {
            /// Action to perform based on regex matching. Default is 'replace'
            pub action: String,
            /// Modulus to take of the hash of the source label values.
            pub modulus: i64,
            /// Regular expression against which the extracted value is matched. Default is '(.*)'
            pub regex: String,
            /// Replacement value against which a regex replace is performed if the regular expression matches. Regex capture groups are available. Default is '$1'
            pub replacement: String,
            /// Separator placed between concatenated source label values. default is ';'.
            pub separator: String,
            /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
            pub source_labels: Vec<String>,
            /// Label to which the resulting value is written in a replace action. It is mandatory for replace actions. Regex capture groups are available.
            pub target_label: String,
        }

        /// OAuth2 scopes used for the token request
        pub struct Scopes {
        }

        /// Secret containing data to use for the targets.
        pub struct Secret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Selector to select Pod objects.
        pub struct Selector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
        pub struct SourceLabels {
        }

        /// Specification of desired Pod selection for target discovery by Prometheus.
        pub struct Spec {
            /// Attaches node metadata to discovered targets. Only valid for role: pod. Only valid in Prometheus versions 2.35.0 and newer.
            pub attach_metadata: AttachMetadata,
            /// The label to use to retrieve the job name from.
            pub job_label: String,
            /// Per-scrape limit on number of labels that will be accepted for a sample. Only valid in Prometheus versions 2.27.0 and newer.
            pub label_limit: i64,
            /// Per-scrape limit on length of labels name that will be accepted for a sample. Only valid in Prometheus versions 2.27.0 and newer.
            pub label_name_length_limit: i64,
            /// Per-scrape limit on length of labels value that will be accepted for a sample. Only valid in Prometheus versions 2.27.0 and newer.
            pub label_value_length_limit: i64,
            /// Selector to select which namespaces the Endpoints objects are discovered from.
            pub namespace_selector: NamespaceSelector,
            /// A list of endpoints allowed as part of this PodMonitor.
            pub pod_metrics_endpoints: Vec<PodMetricsEndpointsItem>,
            /// PodTargetLabels transfers labels on the Kubernetes Pod onto the target.
            pub pod_target_labels: Vec<String>,
            /// SampleLimit defines per-scrape limit on number of scraped samples that will be accepted.
            pub sample_limit: i64,
            /// Selector to select Pod objects.
            pub selector: Selector,
            /// TargetLimit defines a limit on the number of scraped targets that will be accepted.
            pub target_limit: i64,
        }

        /// TLS configuration to use when scraping the endpoint.
        pub struct TlsConfig {
            /// Struct containing the CA cert to use for the targets.
            pub ca: Ca,
            /// Struct containing the client cert file for the targets.
            pub cert: Cert,
            /// Disable target certificate validation.
            pub insecure_skip_verify: bool,
            /// Secret containing the client key file for the targets.
            pub key_secret: KeySecret,
            /// Used to verify the hostname for the targets.
            pub server_name: String,
        }

        /// The secret in the service monitor namespace that contains the username for authentication.
        pub struct Username {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
        pub struct Values {
        }

        impl k8s_openapi::Resource for PodMonitor {
            type Scope = k8s_openapi::ClusterResourceScope;

            const API_VERSION : &'static str = "monitoring.coreos.com/v1";
            const GROUP : &'static str = "monitoring.coreos.com";
            const KIND : &'static str = "PodMonitor";
            const VERSION : &'static str = "v1";
            const URL_PATH_SEGMENT : &'static str = "TODO";
        }

        impl k8s_openapi::Metadata for PodMonitor {
            type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

            fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                &self.metadata
            }

            fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                &mut self.metadata
            }
        }

}
        pub mod probe {
        /// Probe defines monitoring for a set of static targets or ingresses.
        pub struct Probe {
            pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            pub spec: Spec,
        }

        /// Authorization section for this endpoint
        pub struct Authorization {
            /// The secret's key that contains the credentials of the request
            pub credentials: Credentials,
            /// Set the authentication type. Defaults to Bearer, Basic will cause an error
            pub r#type: String,
        }

        /// BasicAuth allow an endpoint to authenticate over basic authentication. More info: https://prometheus.io/docs/operating/configuration/#endpoint
        pub struct BasicAuth {
            /// The secret in the service monitor namespace that contains the password for authentication.
            pub password: Password,
            /// The secret in the service monitor namespace that contains the username for authentication.
            pub username: Username,
        }

        /// Secret to mount to read bearer token for scraping targets. The secret needs to be in the same namespace as the probe and accessible by the Prometheus Operator.
        pub struct BearerTokenSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Struct containing the CA cert to use for the targets.
        pub struct Ca {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// Struct containing the client cert file for the targets.
        pub struct Cert {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret or configmap containing the OAuth2 client id
        pub struct ClientId {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret containing the OAuth2 client secret
        pub struct ClientSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// ConfigMap containing data to use for the targets.
        pub struct ConfigMap {
            /// The key to select.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its key must be defined
            pub optional: bool,
        }

        /// The secret's key that contains the credentials of the request
        pub struct Credentials {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Parameters to append to the token URL
        pub struct EndpointParams {
        }

        /// ingress defines the Ingress objects to probe and the relabeling configuration. If `staticConfig` is also defined, `staticConfig` takes precedence.
        pub struct Ingress {
            /// From which namespaces to select Ingress objects.
            pub namespace_selector: NamespaceSelector,
            /// RelabelConfigs to apply to the label set of the target before it gets scraped. The original ingress address is available via the `__tmp_prometheus_ingress_address` label. It can be used to customize the probed URL. The original scrape job's name is available via the `__tmp_prometheus_job_name` label. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
            pub relabeling_configs: Vec<RelabelingConfigsItem>,
            /// Selector to select the Ingress objects.
            pub selector: Selector,
        }

        /// Secret containing the client key file for the targets.
        pub struct KeySecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Labels assigned to all metrics scraped from the targets.
        pub struct Labels {
        }

        /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
        pub struct MatchExpressions {
        }

        /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
        pub struct MatchExpressionsItem {
            /// key is the label key that the selector applies to.
            pub key: String,
            /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
            pub operator: String,
            /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
            pub values: Vec<String>,
        }

        /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
        pub struct MatchLabels {
        }

        /// List of namespace names to select from.
        pub struct MatchNames {
        }

        pub struct Metadata {
        }

        /// MetricRelabelConfigs to apply to samples before ingestion.
        pub struct MetricRelabelings {
        }

        /// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion. It defines `<metric_relabel_configs>`-section of Prometheus configuration. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
        pub struct MetricRelabelingsItem {
            /// Action to perform based on regex matching. Default is 'replace'
            pub action: String,
            /// Modulus to take of the hash of the source label values.
            pub modulus: i64,
            /// Regular expression against which the extracted value is matched. Default is '(.*)'
            pub regex: String,
            /// Replacement value against which a regex replace is performed if the regular expression matches. Regex capture groups are available. Default is '$1'
            pub replacement: String,
            /// Separator placed between concatenated source label values. default is ';'.
            pub separator: String,
            /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
            pub source_labels: Vec<String>,
            /// Label to which the resulting value is written in a replace action. It is mandatory for replace actions. Regex capture groups are available.
            pub target_label: String,
        }

        /// From which namespaces to select Ingress objects.
        pub struct NamespaceSelector {
            /// Boolean describing whether all namespaces are selected in contrast to a list restricting them.
            pub any: bool,
            /// List of namespace names to select from.
            pub match_names: Vec<String>,
        }

        /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
        pub struct Oauth2 {
            /// The secret or configmap containing the OAuth2 client id
            pub client_id: ClientId,
            /// The secret containing the OAuth2 client secret
            pub client_secret: ClientSecret,
            /// Parameters to append to the token URL
            pub endpoint_params: EndpointParams,
            /// OAuth2 scopes used for the token request
            pub scopes: Vec<String>,
            /// The URL to fetch the token from
            pub token_url: String,
        }

        /// The secret in the service monitor namespace that contains the password for authentication.
        pub struct Password {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Specification for the prober to use for probing targets. The prober.URL parameter is required. Targets cannot be probed if left empty.
        pub struct Prober {
            /// Path to collect metrics from. Defaults to `/probe`.
            pub path: String,
            /// Optional ProxyURL.
            pub proxy_url: String,
            /// HTTP scheme to use for scraping. Defaults to `http`.
            pub scheme: String,
            /// Mandatory URL of the prober.
            pub url: String,
        }

        /// RelabelConfigs to apply to the label set of the targets before it gets scraped. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
        pub struct RelabelingConfigs {
        }

        /// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion. It defines `<metric_relabel_configs>`-section of Prometheus configuration. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
        pub struct RelabelingConfigsItem {
            /// Action to perform based on regex matching. Default is 'replace'
            pub action: String,
            /// Modulus to take of the hash of the source label values.
            pub modulus: i64,
            /// Regular expression against which the extracted value is matched. Default is '(.*)'
            pub regex: String,
            /// Replacement value against which a regex replace is performed if the regular expression matches. Regex capture groups are available. Default is '$1'
            pub replacement: String,
            /// Separator placed between concatenated source label values. default is ';'.
            pub separator: String,
            /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
            pub source_labels: Vec<String>,
            /// Label to which the resulting value is written in a replace action. It is mandatory for replace actions. Regex capture groups are available.
            pub target_label: String,
        }

        /// OAuth2 scopes used for the token request
        pub struct Scopes {
        }

        /// Secret containing data to use for the targets.
        pub struct Secret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Selector to select the Ingress objects.
        pub struct Selector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
        pub struct SourceLabels {
        }

        /// Specification of desired Ingress selection for target discovery by Prometheus.
        pub struct Spec {
            /// Authorization section for this endpoint
            pub authorization: Authorization,
            /// BasicAuth allow an endpoint to authenticate over basic authentication. More info: https://prometheus.io/docs/operating/configuration/#endpoint
            pub basic_auth: BasicAuth,
            /// Secret to mount to read bearer token for scraping targets. The secret needs to be in the same namespace as the probe and accessible by the Prometheus Operator.
            pub bearer_token_secret: BearerTokenSecret,
            /// Interval at which targets are probed using the configured prober. If not specified Prometheus' global scrape interval is used.
            pub interval: String,
            /// The job name assigned to scraped metrics by default.
            pub job_name: String,
            /// Per-scrape limit on number of labels that will be accepted for a sample. Only valid in Prometheus versions 2.27.0 and newer.
            pub label_limit: i64,
            /// Per-scrape limit on length of labels name that will be accepted for a sample. Only valid in Prometheus versions 2.27.0 and newer.
            pub label_name_length_limit: i64,
            /// Per-scrape limit on length of labels value that will be accepted for a sample. Only valid in Prometheus versions 2.27.0 and newer.
            pub label_value_length_limit: i64,
            /// MetricRelabelConfigs to apply to samples before ingestion.
            pub metric_relabelings: Vec<MetricRelabelingsItem>,
            /// The module to use for probing specifying how to probe the target. Example module configuring in the blackbox exporter: https://github.com/prometheus/blackbox_exporter/blob/master/example.yml
            pub module: String,
            /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
            pub oauth2: Oauth2,
            /// Specification for the prober to use for probing targets. The prober.URL parameter is required. Targets cannot be probed if left empty.
            pub prober: Prober,
            /// SampleLimit defines per-scrape limit on number of scraped samples that will be accepted.
            pub sample_limit: i64,
            /// Timeout for scraping metrics from the Prometheus exporter. If not specified, the Prometheus global scrape interval is used.
            pub scrape_timeout: String,
            /// TargetLimit defines a limit on the number of scraped targets that will be accepted.
            pub target_limit: i64,
            /// Targets defines a set of static or dynamically discovered targets to probe.
            pub targets: Targets,
            /// TLS configuration to use when scraping the endpoint.
            pub tls_config: TlsConfig,
        }

        /// The list of hosts to probe.
        pub struct Static {
        }

        /// staticConfig defines the static list of targets to probe and the relabeling configuration. If `ingress` is also defined, `staticConfig` takes precedence. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#static_config.
        pub struct StaticConfig {
            /// Labels assigned to all metrics scraped from the targets.
            pub labels: Labels,
            /// RelabelConfigs to apply to the label set of the targets before it gets scraped. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
            pub relabeling_configs: Vec<RelabelingConfigsItem>,
            /// The list of hosts to probe.
            pub r#static: Vec<String>,
        }

        /// Targets defines a set of static or dynamically discovered targets to probe.
        pub struct Targets {
            /// ingress defines the Ingress objects to probe and the relabeling configuration. If `staticConfig` is also defined, `staticConfig` takes precedence.
            pub ingress: Ingress,
            /// staticConfig defines the static list of targets to probe and the relabeling configuration. If `ingress` is also defined, `staticConfig` takes precedence. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#static_config.
            pub static_config: StaticConfig,
        }

        /// TLS configuration to use when scraping the endpoint.
        pub struct TlsConfig {
            /// Struct containing the CA cert to use for the targets.
            pub ca: Ca,
            /// Struct containing the client cert file for the targets.
            pub cert: Cert,
            /// Disable target certificate validation.
            pub insecure_skip_verify: bool,
            /// Secret containing the client key file for the targets.
            pub key_secret: KeySecret,
            /// Used to verify the hostname for the targets.
            pub server_name: String,
        }

        /// The secret in the service monitor namespace that contains the username for authentication.
        pub struct Username {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
        pub struct Values {
        }

        impl k8s_openapi::Resource for Probe {
            type Scope = k8s_openapi::ClusterResourceScope;

            const API_VERSION : &'static str = "monitoring.coreos.com/v1";
            const GROUP : &'static str = "monitoring.coreos.com";
            const KIND : &'static str = "Probe";
            const VERSION : &'static str = "v1";
            const URL_PATH_SEGMENT : &'static str = "TODO";
        }

        impl k8s_openapi::Metadata for Probe {
            type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

            fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                &self.metadata
            }

            fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                &mut self.metadata
            }
        }

}
        pub mod prometheus {
        /// Prometheus defines a Prometheus deployment.
        pub struct Prometheus {
            pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            pub spec: Spec,
            pub status: Status,
        }

        /// AccessKey is the AWS API key. If blank, the environment variable `AWS_ACCESS_KEY_ID` is used.
        pub struct AccessKey {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
        pub struct AccessModes {
        }

        /// Added capabilities
        pub struct Add {
        }

        /// AdditionalAlertManagerConfigs allows specifying a key of a Secret containing additional Prometheus AlertManager configurations. AlertManager configurations specified are appended to the configurations generated by the Prometheus Operator. Job configurations specified must have the form as specified in the official Prometheus documentation: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#alertmanager_config. As AlertManager configs are appended, the user is responsible to make sure it is valid. Note that using this feature may expose the possibility to break upgrades of Prometheus. It is advised to review Prometheus release notes to ensure that no incompatible AlertManager configs are going to break Prometheus after the upgrade.
        pub struct AdditionalAlertManagerConfigs {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// AdditionalAlertRelabelConfigs allows specifying a key of a Secret containing additional Prometheus alert relabel configurations. Alert relabel configurations specified are appended to the configurations generated by the Prometheus Operator. Alert relabel configurations specified must have the form as specified in the official Prometheus documentation: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#alert_relabel_configs. As alert relabel configs are appended, the user is responsible to make sure it is valid. Note that using this feature may expose the possibility to break upgrades of Prometheus. It is advised to review Prometheus release notes to ensure that no incompatible alert relabel configs are going to break Prometheus after the upgrade.
        pub struct AdditionalAlertRelabelConfigs {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// AdditionalScrapeConfigs allows specifying a key of a Secret containing additional Prometheus scrape configurations. Scrape configurations specified are appended to the configurations generated by the Prometheus Operator. Job configurations specified must have the form as specified in the official Prometheus documentation: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#scrape_config. As scrape configs are appended, the user is responsible to make sure it is valid. Note that using this feature may expose the possibility to break upgrades of Prometheus. It is advised to review Prometheus release notes to ensure that no incompatible scrape configs are going to break Prometheus after the upgrade.
        pub struct AdditionalScrapeConfigs {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// If specified, the pod's scheduling constraints.
        pub struct Affinity {
            /// Describes node affinity scheduling rules for the pod.
            pub node_affinity: NodeAffinity,
            /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
            pub pod_affinity: PodAffinity,
            /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
            pub pod_anti_affinity: PodAntiAffinity,
        }

        /// /--rules.alert.*/ command-line arguments
        pub struct Alert {
            /// Minimum duration between alert and restored 'for' state. This is maintained only for alerts with configured 'for' time greater than grace period.
            pub for_grace_period: String,
            /// Max time to tolerate prometheus outage for restoring 'for' state of alert.
            pub for_outage_tolerance: String,
            /// Minimum amount of time to wait before resending an alert to Alertmanager.
            pub resend_delay: String,
        }

        /// Define details regarding alerting.
        pub struct Alerting {
            /// AlertmanagerEndpoints Prometheus should fire alerts against.
            pub alertmanagers: Vec<AlertmanagersItem>,
        }

        /// AlertmanagerEndpoints Prometheus should fire alerts against.
        pub struct Alertmanagers {
        }

        /// AlertmanagerEndpoints defines a selection of a single Endpoints object containing alertmanager IPs to fire alerts against.
        pub struct AlertmanagersItem {
            /// Version of the Alertmanager API that Prometheus uses to send alerts. It can be "v1" or "v2".
            pub api_version: String,
            /// Authorization section for this alertmanager endpoint
            pub authorization: Authorization,
            /// BearerTokenFile to read from filesystem to use when authenticating to Alertmanager.
            pub bearer_token_file: String,
            /// Name of Endpoints object in Namespace.
            pub name: String,
            /// Namespace of Endpoints object.
            pub namespace: String,
            /// Prefix for the HTTP path alerts are pushed to.
            pub path_prefix: String,
            /// Port the Alertmanager API is exposed on.
            pub port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            /// Scheme to use when firing alerts.
            pub scheme: String,
            /// Timeout is a per-target Alertmanager timeout when pushing alerts.
            pub timeout: String,
            /// TLS Config to use for alertmanager connection.
            pub tls_config: TlsConfig,
        }

        /// The storage resource within AllocatedResources tracks the capacity allocated to a PVC. It may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
        pub struct AllocatedResources {
        }

        /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
        pub struct Annotations {
        }

        /// APIServerConfig allows specifying a host and auth methods to access apiserver. If left empty, Prometheus is assumed to run inside of the cluster and will discover API servers automatically and use the pod's CA certificate and bearer token file at /var/run/secrets/kubernetes.io/serviceaccount/.
        pub struct ApiserverConfig {
            /// Authorization section for accessing apiserver
            pub authorization: Authorization,
            /// BasicAuth allow an endpoint to authenticate over basic authentication
            pub basic_auth: BasicAuth,
            /// Bearer token for accessing apiserver.
            pub bearer_token: String,
            /// File to read bearer token for accessing apiserver.
            pub bearer_token_file: String,
            /// Host of apiserver. A valid string consisting of a hostname or IP followed by an optional port number
            pub host: String,
            /// TLS Config to use for accessing apiserver.
            pub tls_config: TlsConfig,
        }

        /// ArbitraryFSAccessThroughSMs configures whether configuration based on a service monitor can access arbitrary files on the file system of the Prometheus container e.g. bearer token files.
        pub struct ArbitraryFSAccessThroughSMs {
            pub deny: bool,
        }

        /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
        pub struct Args {
        }

        /// Authorization section for remote write
        pub struct Authorization {
            /// The secret's key that contains the credentials of the request
            pub credentials: Credentials,
            /// File to read a secret from, mutually exclusive with Credentials (from SafeAuthorization)
            pub credentials_file: String,
            /// Set the authentication type. Defaults to Bearer, Basic will cause an error
            pub r#type: String,
        }

        /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
        pub struct AwsElasticBlockStore {
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
            pub partition: i32,
            /// Specify "true" to force and set the ReadOnly property in VolumeMounts to "true". If omitted, the default is "false". More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
            pub read_only: bool,
            /// Unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
            pub volume_i_d: String,
        }

        /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
        pub struct AzureDisk {
            /// Host Caching mode: None, Read Only, Read Write.
            pub caching_mode: String,
            /// The Name of the data disk in the blob storage
            pub disk_name: String,
            /// The URI the data disk in the blob storage
            pub disk_u_r_i: String,
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
            pub kind: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
        }

        /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
        pub struct AzureFile {
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// the name of secret that contains Azure Storage Account Name and Key
            pub secret_name: String,
            /// Share Name
            pub share_name: String,
        }

        /// BasicAuth for the URL.
        pub struct BasicAuth {
            /// The secret in the service monitor namespace that contains the password for authentication.
            pub password: Password,
            /// The secret in the service monitor namespace that contains the username for authentication.
            pub username: Username,
        }

        /// Struct containing the CA cert to use for the targets.
        pub struct Ca {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
        pub struct Capabilities {
            /// Added capabilities
            pub add: Vec<String>,
            /// Removed capabilities
            pub drop: Vec<String>,
        }

        /// Represents the actual resources of the underlying volume.
        pub struct Capacity {
        }

        /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
        pub struct Cephfs {
            /// Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub monitors: Vec<String>,
            /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
            pub path: String,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub read_only: bool,
            /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub secret_file: String,
            /// Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub secret_ref: SecretRef,
            /// Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub user: String,
        }

        /// Contains the TLS certificate for the server.
        pub struct Cert {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
        pub struct Cinder {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub fs_type: String,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub read_only: bool,
            /// Optional: points to a secret object containing parameters used to connect to OpenStack.
            pub secret_ref: SecretRef,
            /// volume id used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub volume_i_d: String,
        }

        /// List of supported cipher suites for TLS versions up to TLS 1.2. If empty, Go default cipher suites are used. Available cipher suites are documented in the go documentation: https://golang.org/pkg/crypto/tls/#pkg-constants
        pub struct CipherSuites {
        }

        /// Contains the CA certificate for client certificate authentication to the server.
        pub struct ClientCa {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret or configmap containing the OAuth2 client id
        pub struct ClientId {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret containing the OAuth2 client secret
        pub struct ClientSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
        pub struct Command {
        }

        /// The current state of the Prometheus deployment.
        pub struct Conditions {
        }

        /// PrometheusCondition represents the state of the resources associated with the Prometheus resource.
        pub struct ConditionsItem {
            /// lastTransitionTime is the time of the last update to the current status property.
            pub last_transition_time: String,
            /// Human-readable message indicating details for the condition's last transition.
            pub message: String,
            /// Reason for the condition's last transition.
            pub reason: String,
            /// status of the condition.
            pub status: String,
            /// Type of the condition being reported.
            pub r#type: String,
        }

        /// ConfigMap containing data to use for the targets.
        pub struct ConfigMap {
            /// The key to select.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its key must be defined
            pub optional: bool,
        }

        /// Selects a key of a ConfigMap.
        pub struct ConfigMapKeyRef {
            /// The key to select.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its key must be defined
            pub optional: bool,
        }

        /// The ConfigMap to select from
        pub struct ConfigMapRef {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap must be defined
            pub optional: bool,
        }

        /// ConfigMaps is a list of ConfigMaps in the same namespace as the Prometheus object, which shall be mounted into the Prometheus Pods. The ConfigMaps are mounted into /etc/prometheus/configmaps/<configmap-name>.
        pub struct ConfigMaps {
        }

        /// Containers allows injecting additional containers or modifying operator generated containers. This can be used to allow adding an authentication proxy to a Prometheus pod or to change the behavior of an operator generated container. Containers described here modify an operator generated container if they share the same name and modifications are done via a strategic merge patch. The current container names are: `prometheus`, `config-reloader`, and `thanos-sidecar`. Overriding containers is entirely outside the scope of what the maintainers will support and by doing so, you accept that this behaviour may break at any time without notice.
        pub struct Containers {
        }

        /// A single application container that you want to run within a pod.
        pub struct ContainersItem {
            /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub args: Vec<String>,
            /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub command: Vec<String>,
            /// List of environment variables to set in the container. Cannot be updated.
            pub env: Vec<EnvItem>,
            /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
            pub env_from: Vec<EnvFromItem>,
            /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
            pub image: String,
            /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
            pub image_pull_policy: String,
            /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
            pub lifecycle: Lifecycle,
            /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub liveness_probe: LivenessProbe,
            /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
            pub name: String,
            /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
            pub ports: Vec<PortsItem>,
            /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub readiness_probe: ReadinessProbe,
            /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub resources: Resources,
            /// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
            pub security_context: SecurityContext,
            /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub startup_probe: StartupProbe,
            /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
            pub stdin: bool,
            /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
            pub stdin_once: bool,
            /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
            pub termination_message_path: String,
            /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
            pub termination_message_policy: String,
            /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
            pub tty: bool,
            /// volumeDevices is the list of block devices to be used by the container.
            pub volume_devices: Vec<VolumeDevicesItem>,
            /// Pod volumes to mount into the container's filesystem. Cannot be updated.
            pub volume_mounts: Vec<VolumeMountsItem>,
            /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
            pub working_dir: String,
        }

        /// The secret's key that contains the credentials of the request
        pub struct Credentials {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
        pub struct Csi {
            /// Driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
            pub driver: String,
            /// Filesystem type to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
            pub fs_type: String,
            /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
            pub node_publish_secret_ref: NodePublishSecretRef,
            /// Specifies a read-only configuration for the volume. Defaults to false (read/write).
            pub read_only: bool,
            /// VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
            pub volume_attributes: VolumeAttributes,
        }

        /// Elliptic curves that will be used in an ECDHE handshake, in preference order. Available curves are documented in the go documentation: https://golang.org/pkg/crypto/tls/#CurveID
        pub struct CurvePreferences {
        }

        /// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.
        pub struct DataSource {
            /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
            pub api_group: String,
            /// Kind is the type of resource being referenced
            pub kind: String,
            /// Name is the name of resource being referenced
            pub name: String,
        }

        /// Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef allows any non-core object, as well as PersistentVolumeClaim objects. * While DataSource ignores disallowed values (dropping them), DataSourceRef preserves all values, and generates an error if a disallowed value is specified. (Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
        pub struct DataSourceRef {
            /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
            pub api_group: String,
            /// Kind is the type of resource being referenced
            pub kind: String,
            /// Name is the name of resource being referenced
            pub name: String,
        }

        /// information about the downwardAPI data to project
        pub struct DownwardAPI {
            /// Items is a list of DownwardAPIVolume file
            pub items: Vec<ItemsItem>,
        }

        /// Removed capabilities
        pub struct Drop {
        }

        /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
        pub struct EmptyDir {
            /// What type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
            pub medium: String,
            /// Total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: http://kubernetes.io/docs/user-guide/volumes#emptydir
            pub size_limit: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        }

        /// Enable access to Prometheus disabled features. By default, no features are enabled. Enabling disabled features is entirely outside the scope of what the maintainers will support and by doing so, you accept that this behaviour may break at any time without notice. For more information see https://prometheus.io/docs/prometheus/latest/disabled_features/
        pub struct EnableFeatures {
        }

        /// Parameters to append to the token URL
        pub struct EndpointParams {
        }

        /// List of environment variables to set in the container. Cannot be updated.
        pub struct Env {
        }

        /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
        pub struct EnvFrom {
        }

        /// EnvFromSource represents the source of a set of ConfigMaps
        pub struct EnvFromItem {
            /// The ConfigMap to select from
            pub config_map_ref: ConfigMapRef,
            /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
            pub prefix: String,
            /// The Secret to select from
            pub secret_ref: SecretRef,
        }

        /// EnvVar represents an environment variable present in a Container.
        pub struct EnvItem {
            /// Name of the environment variable. Must be a C_IDENTIFIER.
            pub name: String,
            /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
            pub value: String,
            /// Source for the environment variable's value. Cannot be used if value is not empty.
            pub value_from: ValueFrom,
        }

        /// Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed. 
        ///  Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity tracking are needed, c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through a PersistentVolumeClaim (see EphemeralVolumeSource for more information on the connection between this volume type and PersistentVolumeClaim). 
        ///  Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod. 
        ///  Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information. 
        ///  A pod can use both types of ephemeral volumes and persistent volumes at the same time.
        pub struct Ephemeral {
            /// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long). 
            ///  An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster. 
            ///  This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created. 
            ///  Required, must not be nil.
            pub volume_claim_template: VolumeClaimTemplate,
        }

        /// List of references to PodMonitor, ServiceMonitor, Probe and PrometheusRule objects to be excluded from enforcing a namespace label of origin. Applies only if enforcedNamespaceLabel set to true.
        pub struct ExcludedFromEnforcement {
        }

        /// ObjectReference references a PodMonitor, ServiceMonitor, Probe or PrometheusRule object.
        pub struct ExcludedFromEnforcementItem {
            /// Group of the referent. When not specified, it defaults to `monitoring.coreos.com`
            pub group: String,
            /// Name of the referent. When not set, all resources are matched.
            pub name: String,
            /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
            pub namespace: String,
            /// Resource of the referent.
            pub resource: String,
        }

        /// Exec specifies the action to take.
        pub struct Exec {
            /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
            pub command: Vec<String>,
        }

        /// The labels to add to any time series or alerts when communicating with external systems (federation, remote storage, Alertmanager).
        pub struct ExternalLabels {
        }

        /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
        pub struct Fc {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// Optional: FC target lun number
            pub lun: i32,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// Optional: FC target worldwide names (WWNs)
            pub target_w_w_ns: Vec<String>,
            /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
            pub wwids: Vec<String>,
        }

        /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
        pub struct FieldRef {
            /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
            pub api_version: String,
            /// Path of the field to select in the specified API version.
            pub field_path: String,
        }

        /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
        pub struct FlexVolume {
            /// Driver is the name of the driver to use for this volume.
            pub driver: String,
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
            pub fs_type: String,
            /// Optional: Extra command options if any.
            pub options: Options,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
            pub secret_ref: SecretRef,
        }

        /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
        pub struct Flocker {
            /// Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
            pub dataset_name: String,
            /// UUID of the dataset. This is unique identifier of a Flocker dataset
            pub dataset_u_u_i_d: String,
        }

        /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
        pub struct GcePersistentDisk {
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub partition: i32,
            /// Unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub pd_name: String,
            /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub read_only: bool,
        }

        /// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
        pub struct GitRepo {
            /// Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
            pub directory: String,
            /// Repository URL
            pub repository: String,
            /// Commit hash for the specified revision.
            pub revision: String,
        }

        /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
        pub struct Glusterfs {
            /// EndpointsName is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
            pub endpoints: String,
            /// Path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
            pub path: String,
            /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
            pub read_only: bool,
        }

        /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
        pub struct Grpc {
            /// Port number of the gRPC service. Number must be in the range 1 to 65535.
            pub port: i32,
            /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
            ///  If this is not specified, the default behavior is defined by gRPC.
            pub service: String,
        }

        /// GRPCServerTLSConfig configures the gRPC server from which Thanos Querier reads recorded rule data. Note: Currently only the CAFile, CertFile, and KeyFile fields are supported. Maps to the '--grpc-server-tls-*' CLI args.
        pub struct GrpcServerTlsConfig {
            /// Struct containing the CA cert to use for the targets.
            pub ca: Ca,
            /// Path to the CA cert in the Prometheus container to use for the targets.
            pub ca_file: String,
            /// Struct containing the client cert file for the targets.
            pub cert: Cert,
            /// Path to the client cert file in the Prometheus container for the targets.
            pub cert_file: String,
            /// Disable target certificate validation.
            pub insecure_skip_verify: bool,
            /// Path to the client key file in the Prometheus container for the targets.
            pub key_file: String,
            /// Secret containing the client key file for the targets.
            pub key_secret: KeySecret,
            /// Used to verify the hostname for the targets.
            pub server_name: String,
        }

        /// Custom HTTP headers to be sent along with each remote write request. Be aware that headers that are set by Prometheus itself can't be overwritten. Only valid in Prometheus versions 2.25.0 and newer.
        pub struct Headers {
        }

        /// Pods' hostAliases configuration
        pub struct HostAliases {
        }

        /// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
        pub struct HostAliasesItem {
            /// Hostnames for the above IP address.
            pub hostnames: Vec<String>,
            /// IP address of the host file entry.
            pub ip: String,
        }

        /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath --- TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not mount host directories as read/write.
        pub struct HostPath {
            /// Path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
            pub path: String,
            /// Type for HostPath Volume Defaults to "" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
            pub r#type: String,
        }

        /// Hostnames for the above IP address.
        pub struct Hostnames {
        }

        /// HTTPGet specifies the http request to perform.
        pub struct HttpGet {
            /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
            pub host: String,
            /// Custom headers to set in the request. HTTP allows repeated headers.
            pub http_headers: Vec<HttpHeadersItem>,
            /// Path to access on the HTTP server.
            pub path: String,
            /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
            pub port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            /// Scheme to use for connecting to the host. Defaults to HTTP.
            pub scheme: String,
        }

        /// Custom headers to set in the request. HTTP allows repeated headers.
        pub struct HttpHeaders {
        }

        /// HTTPHeader describes a custom header to be used in HTTP probes
        pub struct HttpHeadersItem {
            /// The header field name
            pub name: String,
            /// The header field value
            pub value: String,
        }

        /// An optional list of references to secrets in the same namespace to use for pulling prometheus and alertmanager images from registries see http://kubernetes.io/docs/user-guide/images#specifying-imagepullsecrets-on-a-pod
        pub struct ImagePullSecrets {
        }

        /// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
        pub struct ImagePullSecretsItem {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
        }

        /// InitContainers allows adding initContainers to the pod definition. Those can be used to e.g. fetch secrets for injection into the Prometheus configuration from external sources. Any errors during the execution of an initContainer will lead to a restart of the Pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/ InitContainers described here modify an operator generated init containers if they share the same name and modifications are done via a strategic merge patch. The current init container name is: `init-config-reloader`. Overriding init containers is entirely outside the scope of what the maintainers will support and by doing so, you accept that this behaviour may break at any time without notice.
        pub struct InitContainers {
        }

        /// A single application container that you want to run within a pod.
        pub struct InitContainersItem {
            /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub args: Vec<String>,
            /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub command: Vec<String>,
            /// List of environment variables to set in the container. Cannot be updated.
            pub env: Vec<EnvItem>,
            /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
            pub env_from: Vec<EnvFromItem>,
            /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
            pub image: String,
            /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
            pub image_pull_policy: String,
            /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
            pub lifecycle: Lifecycle,
            /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub liveness_probe: LivenessProbe,
            /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
            pub name: String,
            /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
            pub ports: Vec<PortsItem>,
            /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub readiness_probe: ReadinessProbe,
            /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub resources: Resources,
            /// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
            pub security_context: SecurityContext,
            /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub startup_probe: StartupProbe,
            /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
            pub stdin: bool,
            /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
            pub stdin_once: bool,
            /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
            pub termination_message_path: String,
            /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
            pub termination_message_policy: String,
            /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
            pub tty: bool,
            /// volumeDevices is the list of block devices to be used by the container.
            pub volume_devices: Vec<VolumeDevicesItem>,
            /// Pod volumes to mount into the container's filesystem. Cannot be updated.
            pub volume_mounts: Vec<VolumeMountsItem>,
            /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
            pub working_dir: String,
        }

        /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
        pub struct Iscsi {
            /// whether support iSCSI Discovery CHAP authentication
            pub chap_auth_discovery: bool,
            /// whether support iSCSI Session CHAP authentication
            pub chap_auth_session: bool,
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// Custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
            pub initiator_name: String,
            /// Target iSCSI Qualified Name.
            pub iqn: String,
            /// iSCSI Interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
            pub iscsi_interface: String,
            /// iSCSI Target Lun number.
            pub lun: i32,
            /// iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
            pub portals: Vec<String>,
            /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
            pub read_only: bool,
            /// CHAP Secret for iSCSI target and initiator authentication
            pub secret_ref: SecretRef,
            /// iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
            pub target_portal: String,
        }

        /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
        pub struct Items {
        }

        /// Maps a string key to a path within a volume.
        pub struct ItemsItem {
            /// The key to project.
            pub key: String,
            /// Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
            pub mode: i32,
            /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
            pub path: String,
        }

        /// Secret containing the TLS key for the server.
        pub struct KeySecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
        pub struct LabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
        pub struct Labels {
        }

        /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
        pub struct Lifecycle {
            /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
            pub post_start: PostStart,
            /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
            pub pre_stop: PreStop,
        }

        /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
        pub struct Limits {
        }

        /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
        pub struct LivenessProbe {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
            pub failure_threshold: i32,
            /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
            pub grpc: Grpc,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub initial_delay_seconds: i32,
            /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
            pub period_seconds: i32,
            /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
            pub success_threshold: i32,
            /// TCPSocket specifies an action involving a TCP port.
            pub tcp_socket: TcpSocket,
            /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
            pub termination_grace_period_seconds: i64,
            /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub timeout_seconds: i32,
        }

        /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
        pub struct MatchExpressions {
        }

        /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
        pub struct MatchExpressionsItem {
            /// key is the label key that the selector applies to.
            pub key: String,
            /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
            pub operator: String,
            /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
            pub values: Vec<String>,
        }

        /// A list of node selector requirements by node's fields.
        pub struct MatchFields {
        }

        /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
        pub struct MatchFieldsItem {
            /// The label key that the selector applies to.
            pub key: String,
            /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
            pub operator: String,
            /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
            pub values: Vec<String>,
        }

        /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
        pub struct MatchLabels {
        }

        /// May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
        pub struct Metadata {
        }

        /// MetadataConfig configures the sending of series metadata to the remote storage.
        pub struct MetadataConfig {
            /// Whether metric metadata is sent to the remote storage or not.
            pub send: bool,
            /// How frequently metric metadata is sent to the remote storage.
            pub send_interval: String,
        }

        /// A collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
        pub struct Monitors {
        }

        /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
        pub struct NamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
        pub struct Namespaces {
        }

        /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
        pub struct Nfs {
            /// Path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub path: String,
            /// ReadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub read_only: bool,
            /// Server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub server: String,
        }

        /// Describes node affinity scheduling rules for the pod.
        pub struct NodeAffinity {
            /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
            pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredDuringSchedulingIgnoredDuringExecutionItem>,
            /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
            pub required_during_scheduling_ignored_during_execution: RequiredDuringSchedulingIgnoredDuringExecution,
        }

        /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
        pub struct NodePublishSecretRef {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
        }

        /// Define which Nodes the Pods are scheduled on.
        pub struct NodeSelector {
        }

        /// Required. A list of node selector terms. The terms are ORed.
        pub struct NodeSelectorTerms {
        }

        /// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
        pub struct NodeSelectorTermsItem {
            /// A list of node selector requirements by node's labels.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// A list of node selector requirements by node's fields.
            pub match_fields: Vec<MatchFieldsItem>,
        }

        /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
        pub struct Oauth2 {
            /// The secret or configmap containing the OAuth2 client id
            pub client_id: ClientId,
            /// The secret containing the OAuth2 client secret
            pub client_secret: ClientSecret,
            /// Parameters to append to the token URL
            pub endpoint_params: EndpointParams,
            /// OAuth2 scopes used for the token request
            pub scopes: Vec<String>,
            /// The URL to fetch the token from
            pub token_url: String,
        }

        /// ObjectStorageConfig configures object storage in Thanos. Alternative to ObjectStorageConfigFile, and lower order priority.
        pub struct ObjectStorageConfig {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Optional: Extra command options if any.
        pub struct Options {
        }

        /// The secret in the service monitor namespace that contains the password for authentication.
        pub struct Password {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
        pub struct PersistentVolumeClaim {
            /// ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
            pub claim_name: String,
            /// Will force the ReadOnly setting in VolumeMounts. Default false.
            pub read_only: bool,
        }

        /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
        pub struct PhotonPersistentDisk {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// ID that identifies Photon Controller persistent disk
            pub pd_i_d: String,
        }

        /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
        pub struct PodAffinity {
            /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
            pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredDuringSchedulingIgnoredDuringExecutionItem>,
            /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
            pub required_during_scheduling_ignored_during_execution: Vec<RequiredDuringSchedulingIgnoredDuringExecutionItem>,
        }

        /// Required. A pod affinity term, associated with the corresponding weight.
        pub struct PodAffinityTerm {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: LabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: NamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

        /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
        pub struct PodAntiAffinity {
            /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
            pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredDuringSchedulingIgnoredDuringExecutionItem>,
            /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
            pub required_during_scheduling_ignored_during_execution: Vec<RequiredDuringSchedulingIgnoredDuringExecutionItem>,
        }

        /// PodMetadata configures Labels and Annotations which are propagated to the prometheus pods.
        pub struct PodMetadata {
            /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
            pub annotations: Annotations,
            /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
            pub labels: Labels,
            /// Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names
            pub name: String,
        }

        /// Namespace's labels to match for PodMonitor discovery. If nil, only check own namespace.
        pub struct PodMonitorNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// *Experimental* PodMonitors to be selected for target discovery. *Deprecated:* if neither this nor serviceMonitorSelector are specified, configuration is unmanaged.
        pub struct PodMonitorSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
        pub struct Portals {
        }

        /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
        pub struct Ports {
        }

        /// ContainerPort represents a network port in a single container.
        pub struct PortsItem {
            /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
            pub container_port: i32,
            /// What host IP to bind the external port to.
            pub host_i_p: String,
            /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
            pub host_port: i32,
            /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
            pub name: String,
            /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
            pub protocol: String,
        }

        /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
        pub struct PortworxVolume {
            /// FSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// VolumeID uniquely identifies a Portworx volume
            pub volume_i_d: String,
        }

        /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
        pub struct PostStart {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
            pub tcp_socket: TcpSocket,
        }

        /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
        pub struct PreStop {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
            pub tcp_socket: TcpSocket,
        }

        /// A node selector term, associated with the corresponding weight.
        pub struct Preference {
            /// A list of node selector requirements by node's labels.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// A list of node selector requirements by node's fields.
            pub match_fields: Vec<MatchFieldsItem>,
        }

        /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
        pub struct PreferredDuringSchedulingIgnoredDuringExecution {
        }

        /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
        pub struct PreferredDuringSchedulingIgnoredDuringExecutionItem {
            /// Required. A pod affinity term, associated with the corresponding weight.
            pub pod_affinity_term: PodAffinityTerm,
            /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
            pub weight: i32,
        }

        /// *Experimental* Namespaces to be selected for Probe discovery. If nil, only check own namespace.
        pub struct ProbeNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// *Experimental* Probes to be selected for target discovery.
        pub struct ProbeSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// Items for all in one resources secrets, configmaps, and downward API
        pub struct Projected {
            /// Mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
            pub default_mode: i32,
            /// list of volume projections
            pub sources: Vec<SourcesItem>,
        }

        /// PrometheusRulesExcludedFromEnforce - list of prometheus rules to be excluded from enforcing of adding namespace labels. Works only if enforcedNamespaceLabel set to true. Make sure both ruleNamespace and ruleName are set for each pair. Deprecated: use excludedFromEnforcement instead.
        pub struct PrometheusRulesExcludedFromEnforce {
        }

        /// PrometheusRuleExcludeConfig enables users to configure excluded PrometheusRule names and their namespaces to be ignored while enforcing namespace label for alerts and metrics.
        pub struct PrometheusRulesExcludedFromEnforceItem {
            /// RuleNamespace - name of excluded rule
            pub rule_name: String,
            /// RuleNamespace - namespace of excluded rule
            pub rule_namespace: String,
        }

        /// QuerySpec defines the query command line flags when starting Prometheus.
        pub struct Query {
            /// The delta difference allowed for retrieving metrics during expression evaluations.
            pub lookback_delta: String,
            /// Number of concurrent queries that can be run at once.
            pub max_concurrency: i32,
            /// Maximum number of samples a single query can load into memory. Note that queries will fail if they would load more samples than this into memory, so this also limits the number of samples a query can return.
            pub max_samples: i32,
            /// Maximum time a query may take before being aborted.
            pub timeout: String,
        }

        /// QueueConfig allows tuning of the remote write queue parameters.
        pub struct QueueConfig {
            /// BatchSendDeadline is the maximum time a sample will wait in buffer.
            pub batch_send_deadline: String,
            /// Capacity is the number of samples to buffer per shard before we start dropping them.
            pub capacity: i64,
            /// MaxBackoff is the maximum retry delay.
            pub max_backoff: String,
            /// MaxRetries is the maximum number of times to retry a batch on recoverable errors.
            pub max_retries: i64,
            /// MaxSamplesPerSend is the maximum number of samples per send.
            pub max_samples_per_send: i64,
            /// MaxShards is the maximum number of shards, i.e. amount of concurrency.
            pub max_shards: i64,
            /// MinBackoff is the initial retry delay. Gets doubled for every retry.
            pub min_backoff: String,
            /// MinShards is the minimum number of shards, i.e. amount of concurrency.
            pub min_shards: i64,
            /// Retry upon receiving a 429 status code from the remote-write storage. This is experimental feature and might change in the future.
            pub retry_on_rate_limit: bool,
        }

        /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
        pub struct Quobyte {
            /// Group to map volume access to Default is no group
            pub group: String,
            /// ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
            pub read_only: bool,
            /// Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
            pub registry: String,
            /// Tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin
            pub tenant: String,
            /// User to map volume access to Defaults to serivceaccount user
            pub user: String,
            /// Volume is a string that references an already created Quobyte volume by name.
            pub volume: String,
        }

        /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
        pub struct Rbd {
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// The rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub image: String,
            /// Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub keyring: String,
            /// A collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub monitors: Vec<String>,
            /// The rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub pool: String,
            /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub read_only: bool,
            /// SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub secret_ref: SecretRef,
            /// The rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub user: String,
        }

        /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
        pub struct ReadinessProbe {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
            pub failure_threshold: i32,
            /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
            pub grpc: Grpc,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub initial_delay_seconds: i32,
            /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
            pub period_seconds: i32,
            /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
            pub success_threshold: i32,
            /// TCPSocket specifies an action involving a TCP port.
            pub tcp_socket: TcpSocket,
            /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
            pub termination_grace_period_seconds: i64,
            /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub timeout_seconds: i32,
        }

        /// remoteRead is the list of remote read configurations.
        pub struct RemoteRead {
        }

        /// RemoteReadSpec defines the configuration for Prometheus to read back samples from a remote endpoint.
        pub struct RemoteReadItem {
            /// Authorization section for remote read
            pub authorization: Authorization,
            /// BasicAuth for the URL.
            pub basic_auth: BasicAuth,
            /// Bearer token for remote read.
            pub bearer_token: String,
            /// File to read bearer token for remote read.
            pub bearer_token_file: String,
            /// Custom HTTP headers to be sent along with each remote read request. Be aware that headers that are set by Prometheus itself can't be overwritten. Only valid in Prometheus versions 2.26.0 and newer.
            pub headers: Headers,
            /// The name of the remote read queue, it must be unique if specified. The name is used in metrics and logging in order to differentiate read configurations.  Only valid in Prometheus versions 2.15.0 and newer.
            pub name: String,
            /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
            pub oauth2: Oauth2,
            /// Optional ProxyURL.
            pub proxy_url: String,
            /// Whether reads should be made for queries for time ranges that the local storage should have complete data for.
            pub read_recent: bool,
            /// Timeout for requests to the remote read endpoint.
            pub remote_timeout: String,
            /// An optional list of equality matchers which have to be present in a selector to query the remote read endpoint.
            pub required_matchers: RequiredMatchers,
            /// TLS Config to use for remote read.
            pub tls_config: TlsConfig,
            /// The URL of the endpoint to query from.
            pub url: String,
        }

        /// remoteWrite is the list of remote write configurations.
        pub struct RemoteWrite {
        }

        /// RemoteWriteSpec defines the configuration to write samples from Prometheus to a remote endpoint.
        pub struct RemoteWriteItem {
            /// Authorization section for remote write
            pub authorization: Authorization,
            /// BasicAuth for the URL.
            pub basic_auth: BasicAuth,
            /// Bearer token for remote write.
            pub bearer_token: String,
            /// File to read bearer token for remote write.
            pub bearer_token_file: String,
            /// Custom HTTP headers to be sent along with each remote write request. Be aware that headers that are set by Prometheus itself can't be overwritten. Only valid in Prometheus versions 2.25.0 and newer.
            pub headers: Headers,
            /// MetadataConfig configures the sending of series metadata to the remote storage.
            pub metadata_config: MetadataConfig,
            /// The name of the remote write queue, it must be unique if specified. The name is used in metrics and logging in order to differentiate queues. Only valid in Prometheus versions 2.15.0 and newer.
            pub name: String,
            /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
            pub oauth2: Oauth2,
            /// Optional ProxyURL.
            pub proxy_url: String,
            /// QueueConfig allows tuning of the remote write queue parameters.
            pub queue_config: QueueConfig,
            /// Timeout for requests to the remote write endpoint.
            pub remote_timeout: String,
            /// Enables sending of exemplars over remote write. Note that exemplar-storage itself must be enabled using the enableFeature option for exemplars to be scraped in the first place.  Only valid in Prometheus versions 2.27.0 and newer.
            pub send_exemplars: bool,
            /// Sigv4 allows to configures AWS's Signature Verification 4
            pub sigv4: Sigv4,
            /// TLS Config to use for remote write.
            pub tls_config: TlsConfig,
            /// The URL of the endpoint to send samples to.
            pub url: String,
            /// The list of remote write relabel configurations.
            pub write_relabel_configs: Vec<WriteRelabelConfigsItem>,
        }

        /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
        pub struct Requests {
        }

        /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
        pub struct RequiredDuringSchedulingIgnoredDuringExecution {
        }

        /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
        pub struct RequiredDuringSchedulingIgnoredDuringExecutionItem {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: LabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: NamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

        /// An optional list of equality matchers which have to be present in a selector to query the remote read endpoint.
        pub struct RequiredMatchers {
        }

        /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
        pub struct ResourceFieldRef {
            /// Container name: required for volumes, optional for env vars
            pub container_name: String,
            /// Specifies the output format of the exposed resources, defaults to "1"
            pub divisor: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            /// Required: resource to select
            pub resource: String,
        }

        /// Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
        pub struct Resources {
            /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub limits: Limits,
            /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub requests: Requests,
        }

        /// Namespaces to be selected for PrometheusRules discovery. If unspecified, only the same namespace as the Prometheus object is in is used.
        pub struct RuleNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// A selector to select which PrometheusRules to mount for loading alerting/recording rules from. Until (excluding) Prometheus Operator v0.24.0 Prometheus Operator will migrate any legacy rule ConfigMaps to PrometheusRule custom resources selected by RuleSelector. Make sure it does not match any config maps that you do not want to be migrated.
        pub struct RuleSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// /--rules.*/ command-line arguments.
        pub struct Rules {
            /// /--rules.alert.*/ command-line arguments
            pub alert: Alert,
        }

        /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
        pub struct ScaleIO {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
            pub fs_type: String,
            /// The host address of the ScaleIO API Gateway.
            pub gateway: String,
            /// The name of the ScaleIO Protection Domain for the configured storage.
            pub protection_domain: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
            pub secret_ref: SecretRef,
            /// Flag to enable/disable SSL communication with Gateway, default false
            pub ssl_enabled: bool,
            /// Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
            pub storage_mode: String,
            /// The ScaleIO Storage Pool associated with the protection domain.
            pub storage_pool: String,
            /// The name of the storage system as configured in ScaleIO.
            pub system: String,
            /// The name of a volume already created in the ScaleIO system that is associated with this volume source.
            pub volume_name: String,
        }

        /// OAuth2 scopes used for the token request
        pub struct Scopes {
        }

        /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
        pub struct SeLinuxOptions {
            /// Level is SELinux level label that applies to the container.
            pub level: String,
            /// Role is a SELinux role label that applies to the container.
            pub role: String,
            /// Type is a SELinux type label that applies to the container.
            pub r#type: String,
            /// User is a SELinux user label that applies to the container.
            pub user: String,
        }

        /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
        pub struct SeccompProfile {
            /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
            pub localhost_profile: String,
            /// type indicates which kind of seccomp profile will be applied. Valid options are: 
            ///  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
            pub r#type: String,
        }

        /// Secret containing data to use for the targets.
        pub struct Secret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// SecretKey is the AWS API secret. If blank, the environment variable `AWS_SECRET_ACCESS_KEY` is used.
        pub struct SecretKey {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Selects a key of a secret in the pod's namespace
        pub struct SecretKeyRef {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
        pub struct SecretRef {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
        }

        /// Secrets is a list of Secrets in the same namespace as the Prometheus object, which shall be mounted into the Prometheus Pods. The Secrets are mounted into /etc/prometheus/secrets/<secret-name>.
        pub struct Secrets {
        }

        /// SecurityContext holds pod-level security attributes and common container settings. This defaults to the default PodSecurityContext.
        pub struct SecurityContext {
            /// A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod: 
            ///  1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw---- 
            ///  If unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows.
            pub fs_group: i64,
            /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used. Note that this field cannot be set when spec.os.name is windows.
            pub fs_group_change_policy: String,
            /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
            pub run_as_group: i64,
            /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
            pub run_as_non_root: bool,
            /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
            pub run_as_user: i64,
            /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
            pub se_linux_options: SeLinuxOptions,
            /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
            pub seccomp_profile: SeccompProfile,
            /// A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container. Note that this field cannot be set when spec.os.name is windows.
            pub supplemental_groups: Vec<i64>,
            /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
            pub sysctls: Vec<SysctlsItem>,
            /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
            pub windows_options: WindowsOptions,
        }

        /// A label query over volumes to consider for binding.
        pub struct Selector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// information about the serviceAccountToken data to project
        pub struct ServiceAccountToken {
            /// Audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.
            pub audience: String,
            /// ExpirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.
            pub expiration_seconds: i64,
            /// Path is the path relative to the mount point of the file to project the token into.
            pub path: String,
        }

        /// Namespace's labels to match for ServiceMonitor discovery. If nil, only check own namespace.
        pub struct ServiceMonitorNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// ServiceMonitors to be selected for target discovery. *Deprecated:* if neither this nor podMonitorSelector are specified, configuration is unmanaged.
        pub struct ServiceMonitorSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// The list has one entry per shard. Each entry provides a summary of the shard status.
        pub struct ShardStatuses {
        }

        pub struct ShardStatusesItem {
            /// Total number of available pods (ready for at least minReadySeconds) targeted by this shard.
            pub available_replicas: i32,
            /// Total number of pods targeted by this shard.
            pub replicas: i32,
            /// Identifier of the shard.
            pub shard_i_d: String,
            /// Total number of unavailable pods targeted by this shard.
            pub unavailable_replicas: i32,
            /// Total number of non-terminated pods targeted by this shard that have the desired spec.
            pub updated_replicas: i32,
        }

        /// Sigv4 allows to configures AWS's Signature Verification 4
        pub struct Sigv4 {
            /// AccessKey is the AWS API key. If blank, the environment variable `AWS_ACCESS_KEY_ID` is used.
            pub access_key: AccessKey,
            /// Profile is the named AWS profile used to authenticate.
            pub profile: String,
            /// Region is the AWS region. If blank, the region from the default credentials chain used.
            pub region: String,
            /// RoleArn is the named AWS profile used to authenticate.
            pub role_arn: String,
            /// SecretKey is the AWS API secret. If blank, the environment variable `AWS_SECRET_ACCESS_KEY` is used.
            pub secret_key: SecretKey,
        }

        /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
        pub struct SourceLabels {
        }

        /// list of volume projections
        pub struct Sources {
        }

        /// Projection that may be projected along with other supported volume types
        pub struct SourcesItem {
            /// information about the configMap data to project
            pub config_map: ConfigMap,
            /// information about the downwardAPI data to project
            pub downward_a_p_i: DownwardAPI,
            /// information about the secret data to project
            pub secret: Secret,
            /// information about the serviceAccountToken data to project
            pub service_account_token: ServiceAccountToken,
        }

        /// The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
        pub struct Spec {
            /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
            pub access_modes: Vec<String>,
            /// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.
            pub data_source: DataSource,
            /// Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef allows any non-core object, as well as PersistentVolumeClaim objects. * While DataSource ignores disallowed values (dropping them), DataSourceRef preserves all values, and generates an error if a disallowed value is specified. (Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
            pub data_source_ref: DataSourceRef,
            /// Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
            pub resources: Resources,
            /// A label query over volumes to consider for binding.
            pub selector: Selector,
            /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
            pub storage_class_name: String,
            /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
            pub volume_mode: String,
            /// VolumeName is the binding reference to the PersistentVolume backing this claim.
            pub volume_name: String,
        }

        /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
        pub struct StartupProbe {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
            pub failure_threshold: i32,
            /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
            pub grpc: Grpc,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub initial_delay_seconds: i32,
            /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
            pub period_seconds: i32,
            /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
            pub success_threshold: i32,
            /// TCPSocket specifies an action involving a TCP port.
            pub tcp_socket: TcpSocket,
            /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
            pub termination_grace_period_seconds: i64,
            /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub timeout_seconds: i32,
        }

        /// Most recent observed status of the Prometheus cluster. Read-only. More info: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
        pub struct Status {
            /// Total number of available pods (ready for at least minReadySeconds) targeted by this Prometheus deployment.
            pub available_replicas: i32,
            /// The current state of the Prometheus deployment.
            pub conditions: Vec<ConditionsItem>,
            /// Represents whether any actions on the underlying managed objects are being performed. Only delete actions will be performed.
            pub paused: bool,
            /// Total number of non-terminated pods targeted by this Prometheus deployment (their labels match the selector).
            pub replicas: i32,
            /// The list has one entry per shard. Each entry provides a summary of the shard status.
            pub shard_statuses: Vec<ShardStatusesItem>,
            /// Total number of unavailable pods targeted by this Prometheus deployment.
            pub unavailable_replicas: i32,
            /// Total number of non-terminated pods targeted by this Prometheus deployment that have the desired version spec.
            pub updated_replicas: i32,
        }

        /// Storage spec to specify how storage shall be used.
        pub struct Storage {
            /// Deprecated: subPath usage will be disabled by default in a future release, this option will become unnecessary. DisableMountSubPath allows to remove any subPath usage in volume mounts.
            pub disable_mount_sub_path: bool,
            /// EmptyDirVolumeSource to be used by the Prometheus StatefulSets. If specified, used in place of any volumeClaimTemplate. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir
            pub empty_dir: EmptyDir,
            /// EphemeralVolumeSource to be used by the Prometheus StatefulSets. This is a beta field in k8s 1.21, for lower versions, starting with k8s 1.19, it requires enabling the GenericEphemeralVolume feature gate. More info: https://kubernetes.io/docs/concepts/storage/ephemeral-volumes/#generic-ephemeral-volumes
            pub ephemeral: Ephemeral,
            /// A PVC spec to be used by the Prometheus StatefulSets.
            pub volume_claim_template: VolumeClaimTemplate,
        }

        /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
        pub struct Storageos {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
            pub secret_ref: SecretRef,
            /// VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
            pub volume_name: String,
            /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
            pub volume_namespace: String,
        }

        /// A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container. Note that this field cannot be set when spec.os.name is windows.
        pub struct SupplementalGroups {
        }

        /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
        pub struct Sysctls {
        }

        /// Sysctl defines a kernel parameter to be set
        pub struct SysctlsItem {
            /// Name of a property to set
            pub name: String,
            /// Value of a property to set
            pub value: String,
        }

        /// Optional: FC target worldwide names (WWNs)
        pub struct TargetWWNs {
        }

        /// TCPSocket specifies an action involving a TCP port.
        pub struct TcpSocket {
            /// Optional: Host name to connect to, defaults to the pod IP.
            pub host: String,
            /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
            pub port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        }

        /// Thanos configuration allows configuring various aspects of a Prometheus server in a Thanos environment. 
        ///  This section is experimental, it may change significantly without deprecation notice in any release. 
        ///  This is experimental and may change significantly without backward compatibility in any release.
        pub struct Thanos {
            /// Thanos base image if other than default. Deprecated: use 'image' instead
            pub base_image: String,
            /// GRPCServerTLSConfig configures the gRPC server from which Thanos Querier reads recorded rule data. Note: Currently only the CAFile, CertFile, and KeyFile fields are supported. Maps to the '--grpc-server-tls-*' CLI args.
            pub grpc_server_tls_config: GrpcServerTlsConfig,
            /// Image if specified has precedence over baseImage, tag and sha combinations. Specifying the version is still necessary to ensure the Prometheus Operator knows what version of Thanos is being configured.
            pub image: String,
            /// ListenLocal makes the Thanos sidecar listen on loopback, so that it does not bind against the Pod IP.
            pub listen_local: bool,
            /// LogFormat for Thanos sidecar to be configured with.
            pub log_format: String,
            /// LogLevel for Thanos sidecar to be configured with.
            pub log_level: String,
            /// MinTime for Thanos sidecar to be configured with. Option can be a constant time in RFC3339 format or time duration relative to current time, such as -1d or 2h45m. Valid duration units are ms, s, m, h, d, w, y.
            pub min_time: String,
            /// ObjectStorageConfig configures object storage in Thanos. Alternative to ObjectStorageConfigFile, and lower order priority.
            pub object_storage_config: ObjectStorageConfig,
            /// ObjectStorageConfigFile specifies the path of the object storage configuration file. When used alongside with ObjectStorageConfig, ObjectStorageConfigFile takes precedence.
            pub object_storage_config_file: String,
            /// ReadyTimeout is the maximum time Thanos sidecar will wait for Prometheus to start. Eg 10m
            pub ready_timeout: String,
            /// Resources defines the resource requirements for the Thanos sidecar. If not provided, no requests/limits will be set
            pub resources: Resources,
            /// SHA of Thanos container image to be deployed. Defaults to the value of `version`. Similar to a tag, but the SHA explicitly deploys an immutable container image. Version and Tag are ignored if SHA is set. Deprecated: use 'image' instead.  The image digest can be specified as part of the image URL.
            pub sha: String,
            /// Tag of Thanos sidecar container image to be deployed. Defaults to the value of `version`. Version is ignored if Tag is set. Deprecated: use 'image' instead.  The image tag can be specified as part of the image URL.
            pub tag: String,
            /// TracingConfig configures tracing in Thanos. This is an experimental feature, it may change in any upcoming release in a breaking way.
            pub tracing_config: TracingConfig,
            /// TracingConfig specifies the path of the tracing configuration file. When used alongside with TracingConfig, TracingConfigFile takes precedence.
            pub tracing_config_file: String,
            /// Version describes the version of Thanos to use.
            pub version: String,
            /// VolumeMounts allows configuration of additional VolumeMounts on the output StatefulSet definition. VolumeMounts specified will be appended to other VolumeMounts in the thanos-sidecar container.
            pub volume_mounts: Vec<VolumeMountsItem>,
        }

        /// WebTLSConfig defines the TLS parameters for HTTPS.
        pub struct TlsConfig {
            /// Contains the TLS certificate for the server.
            pub cert: Cert,
            /// List of supported cipher suites for TLS versions up to TLS 1.2. If empty, Go default cipher suites are used. Available cipher suites are documented in the go documentation: https://golang.org/pkg/crypto/tls/#pkg-constants
            pub cipher_suites: Vec<String>,
            /// Server policy for client authentication. Maps to ClientAuth Policies. For more detail on clientAuth options: https://golang.org/pkg/crypto/tls/#ClientAuthType
            pub client_auth_type: String,
            /// Contains the CA certificate for client certificate authentication to the server.
            pub client_ca: ClientCa,
            /// Elliptic curves that will be used in an ECDHE handshake, in preference order. Available curves are documented in the go documentation: https://golang.org/pkg/crypto/tls/#CurveID
            pub curve_preferences: Vec<String>,
            /// Secret containing the TLS key for the server.
            pub key_secret: KeySecret,
            /// Maximum TLS version that is acceptable. Defaults to TLS13.
            pub max_version: String,
            /// Minimum TLS version that is acceptable. Defaults to TLS12.
            pub min_version: String,
            /// Controls whether the server selects the client's most preferred cipher suite, or the server's most preferred cipher suite. If true then the server's preference, as expressed in the order of elements in cipherSuites, is used.
            pub prefer_server_cipher_suites: bool,
        }

        /// If specified, the pod's tolerations.
        pub struct Tolerations {
        }

        /// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
        pub struct TolerationsItem {
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

        /// If specified, the pod's topology spread constraints.
        pub struct TopologySpreadConstraints {
        }

        /// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
        pub struct TopologySpreadConstraintsItem {
            /// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
            pub label_selector: LabelSelector,
            /// MaxSkew describes the degree to which pods may be unevenly distributed. When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference between the number of matching pods in the target topology and the global minimum. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 1/1/0: | zone1 | zone2 | zone3 | |   P   |   P   |       | - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 1/1/1; scheduling it onto zone1(zone2) would make the ActualSkew(2-0) on zone1(zone2) violate MaxSkew(1). - if MaxSkew is 2, incoming pod can be scheduled onto any zone. When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence to topologies that satisfy it. It's a required field. Default value is 1 and 0 is not allowed.
            pub max_skew: i32,
            /// TopologyKey is the key of node labels. Nodes that have a label with this key and identical values are considered to be in the same topology. We consider each <key, value> as a "bucket", and try to put balanced number of pods into each bucket. It's a required field.
            pub topology_key: String,
            /// WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint. - DoNotSchedule (default) tells the scheduler not to schedule it. - ScheduleAnyway tells the scheduler to schedule the pod in any location, but giving higher precedence to topologies that would help reduce the skew. A constraint is considered "Unsatisfiable" for an incoming pod if and only if every possible node assignment for that pod would violate "MaxSkew" on some topology. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 3/1/1: | zone1 | zone2 | zone3 | | P P P |   P   |   P   | If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler won't make it *more* imbalanced. It's a required field.
            pub when_unsatisfiable: String,
        }

        /// TracingConfig configures tracing in Thanos. This is an experimental feature, it may change in any upcoming release in a breaking way.
        pub struct TracingConfig {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// The secret in the service monitor namespace that contains the username for authentication.
        pub struct Username {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Source for the environment variable's value. Cannot be used if value is not empty.
        pub struct ValueFrom {
            /// Selects a key of a ConfigMap.
            pub config_map_key_ref: ConfigMapKeyRef,
            /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
            pub field_ref: FieldRef,
            /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
            pub resource_field_ref: ResourceFieldRef,
            /// Selects a key of a secret in the pod's namespace
            pub secret_key_ref: SecretKeyRef,
        }

        /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
        pub struct Values {
        }

        /// VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
        pub struct VolumeAttributes {
        }

        /// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long). 
        ///  An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster. 
        ///  This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created. 
        ///  Required, must not be nil.
        pub struct VolumeClaimTemplate {
            /// May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
            pub metadata: Metadata,
            /// The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
            pub spec: Spec,
        }

        /// volumeDevices is the list of block devices to be used by the container.
        pub struct VolumeDevices {
        }

        /// volumeDevice describes a mapping of a raw block device within a container.
        pub struct VolumeDevicesItem {
            /// devicePath is the path inside of the container that the device will be mapped to.
            pub device_path: String,
            /// name must match the name of a persistentVolumeClaim in the pod
            pub name: String,
        }

        /// VolumeMounts allows configuration of additional VolumeMounts on the output StatefulSet definition. VolumeMounts specified will be appended to other VolumeMounts in the prometheus container, that are generated as a result of StorageSpec objects.
        pub struct VolumeMounts {
        }

        /// VolumeMount describes a mounting of a Volume within a container.
        pub struct VolumeMountsItem {
            /// Path within the container at which the volume should be mounted.  Must not contain ':'.
            pub mount_path: String,
            /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
            pub mount_propagation: String,
            /// This must match the Name of a Volume.
            pub name: String,
            /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
            pub read_only: bool,
            /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
            pub sub_path: String,
            /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
            pub sub_path_expr: String,
        }

        /// Volumes allows configuration of additional volumes on the output StatefulSet definition. Volumes specified will be appended to other volumes that are generated as a result of StorageSpec objects.
        pub struct Volumes {
        }

        /// Volume represents a named volume in a pod that may be accessed by any container in the pod.
        pub struct VolumesItem {
            /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
            pub aws_elastic_block_store: AwsElasticBlockStore,
            /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
            pub azure_disk: AzureDisk,
            /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
            pub azure_file: AzureFile,
            /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
            pub cephfs: Cephfs,
            /// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub cinder: Cinder,
            /// ConfigMap represents a configMap that should populate this volume
            pub config_map: ConfigMap,
            /// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
            pub csi: Csi,
            /// DownwardAPI represents downward API about the pod that should populate this volume
            pub downward_a_p_i: DownwardAPI,
            /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
            pub empty_dir: EmptyDir,
            /// Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed. 
            ///  Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity tracking are needed, c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through a PersistentVolumeClaim (see EphemeralVolumeSource for more information on the connection between this volume type and PersistentVolumeClaim). 
            ///  Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod. 
            ///  Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information. 
            ///  A pod can use both types of ephemeral volumes and persistent volumes at the same time.
            pub ephemeral: Ephemeral,
            /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
            pub fc: Fc,
            /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
            pub flex_volume: FlexVolume,
            /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
            pub flocker: Flocker,
            /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub gce_persistent_disk: GcePersistentDisk,
            /// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
            pub git_repo: GitRepo,
            /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
            pub glusterfs: Glusterfs,
            /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath --- TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not mount host directories as read/write.
            pub host_path: HostPath,
            /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
            pub iscsi: Iscsi,
            /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
            pub name: String,
            /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub nfs: Nfs,
            /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
            pub persistent_volume_claim: PersistentVolumeClaim,
            /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
            pub photon_persistent_disk: PhotonPersistentDisk,
            /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
            pub portworx_volume: PortworxVolume,
            /// Items for all in one resources secrets, configmaps, and downward API
            pub projected: Projected,
            /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
            pub quobyte: Quobyte,
            /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
            pub rbd: Rbd,
            /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
            pub scale_i_o: ScaleIO,
            /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
            pub secret: Secret,
            /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
            pub storageos: Storageos,
            /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
            pub vsphere_volume: VsphereVolume,
        }

        /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
        pub struct VsphereVolume {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
            pub storage_policy_i_d: String,
            /// Storage Policy Based Management (SPBM) profile name.
            pub storage_policy_name: String,
            /// Path that identifies vSphere volume vmdk
            pub volume_path: String,
        }

        /// WebSpec defines the web command line flags when starting Prometheus.
        pub struct Web {
            /// The prometheus web page title
            pub page_title: String,
            /// WebTLSConfig defines the TLS parameters for HTTPS.
            pub tls_config: TlsConfig,
        }

        /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
        pub struct WindowsOptions {
            /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
            pub gmsa_credential_spec: String,
            /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
            pub gmsa_credential_spec_name: String,
            /// HostProcess determines if a container should be run as a 'Host Process' container. This field is alpha-level and will only be honored by components that enable the WindowsHostProcessContainers feature flag. Setting this field without the feature flag will result in errors when validating the Pod. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).  In addition, if HostProcess is true then HostNetwork must also be set to true.
            pub host_process: bool,
            /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
            pub run_as_user_name: String,
        }

        /// The list of remote write relabel configurations.
        pub struct WriteRelabelConfigs {
        }

        /// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion. It defines `<metric_relabel_configs>`-section of Prometheus configuration. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
        pub struct WriteRelabelConfigsItem {
            /// Action to perform based on regex matching. Default is 'replace'
            pub action: String,
            /// Modulus to take of the hash of the source label values.
            pub modulus: i64,
            /// Regular expression against which the extracted value is matched. Default is '(.*)'
            pub regex: String,
            /// Replacement value against which a regex replace is performed if the regular expression matches. Regex capture groups are available. Default is '$1'
            pub replacement: String,
            /// Separator placed between concatenated source label values. default is ';'.
            pub separator: String,
            /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
            pub source_labels: Vec<String>,
            /// Label to which the resulting value is written in a replace action. It is mandatory for replace actions. Regex capture groups are available.
            pub target_label: String,
        }

        /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
        pub struct Wwids {
        }

        impl k8s_openapi::Resource for Prometheus {
            type Scope = k8s_openapi::ClusterResourceScope;

            const API_VERSION : &'static str = "monitoring.coreos.com/v1";
            const GROUP : &'static str = "monitoring.coreos.com";
            const KIND : &'static str = "Prometheus";
            const VERSION : &'static str = "v1";
            const URL_PATH_SEGMENT : &'static str = "TODO";
        }

        impl k8s_openapi::Metadata for Prometheus {
            type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

            fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                &self.metadata
            }

            fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                &mut self.metadata
            }
        }

}
        pub mod prometheus_rule {
        /// PrometheusRule defines recording and alerting rules for a Prometheus instance
        pub struct PrometheusRule {
            pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            pub spec: Spec,
        }

        pub struct Annotations {
        }

        /// Content of Prometheus rule file
        pub struct Groups {
        }

        /// RuleGroup is a list of sequentially evaluated recording and alerting rules. Note: PartialResponseStrategy is only used by ThanosRuler and will be ignored by Prometheus instances.  Valid values for this field are 'warn' or 'abort'.  More info: https://github.com/thanos-io/thanos/blob/main/docs/components/rule.md#partial-response
        pub struct GroupsItem {
            pub interval: String,
            pub name: String,
            pub partial_response_strategy: String,
            pub rules: Vec<RulesItem>,
        }

        pub struct Labels {
        }

        pub struct Metadata {
        }

        pub struct Rules {
        }

        /// Rule describes an alerting or recording rule See Prometheus documentation: [alerting](https://www.prometheus.io/docs/prometheus/latest/configuration/alerting_rules/) or [recording](https://www.prometheus.io/docs/prometheus/latest/configuration/recording_rules/#recording-rules) rule
        pub struct RulesItem {
            pub alert: String,
            pub annotations: Annotations,
            pub expr: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            pub r#for: String,
            pub labels: Labels,
            pub record: String,
        }

        /// Specification of desired alerting rule definitions for Prometheus.
        pub struct Spec {
            /// Content of Prometheus rule file
            pub groups: Vec<GroupsItem>,
        }

        impl k8s_openapi::Resource for PrometheusRule {
            type Scope = k8s_openapi::ClusterResourceScope;

            const API_VERSION : &'static str = "monitoring.coreos.com/v1";
            const GROUP : &'static str = "monitoring.coreos.com";
            const KIND : &'static str = "PrometheusRule";
            const VERSION : &'static str = "v1";
            const URL_PATH_SEGMENT : &'static str = "TODO";
        }

        impl k8s_openapi::Metadata for PrometheusRule {
            type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

            fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                &self.metadata
            }

            fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                &mut self.metadata
            }
        }

}
        pub mod service_monitor {
        /// ServiceMonitor defines monitoring for a set of services.
        pub struct ServiceMonitor {
            pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            pub spec: Spec,
        }

        /// Authorization section for this endpoint
        pub struct Authorization {
            /// The secret's key that contains the credentials of the request
            pub credentials: Credentials,
            /// Set the authentication type. Defaults to Bearer, Basic will cause an error
            pub r#type: String,
        }

        /// BasicAuth allow an endpoint to authenticate over basic authentication More info: https://prometheus.io/docs/operating/configuration/#endpoints
        pub struct BasicAuth {
            /// The secret in the service monitor namespace that contains the password for authentication.
            pub password: Password,
            /// The secret in the service monitor namespace that contains the username for authentication.
            pub username: Username,
        }

        /// Secret to mount to read bearer token for scraping targets. The secret needs to be in the same namespace as the service monitor and accessible by the Prometheus Operator.
        pub struct BearerTokenSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Struct containing the CA cert to use for the targets.
        pub struct Ca {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// Struct containing the client cert file for the targets.
        pub struct Cert {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret or configmap containing the OAuth2 client id
        pub struct ClientId {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret containing the OAuth2 client secret
        pub struct ClientSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// ConfigMap containing data to use for the targets.
        pub struct ConfigMap {
            /// The key to select.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its key must be defined
            pub optional: bool,
        }

        /// The secret's key that contains the credentials of the request
        pub struct Credentials {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Parameters to append to the token URL
        pub struct EndpointParams {
        }

        /// A list of endpoints allowed as part of this ServiceMonitor.
        pub struct Endpoints {
        }

        /// Endpoint defines a scrapeable endpoint serving Prometheus metrics.
        pub struct EndpointsItem {
            /// Authorization section for this endpoint
            pub authorization: Authorization,
            /// BasicAuth allow an endpoint to authenticate over basic authentication More info: https://prometheus.io/docs/operating/configuration/#endpoints
            pub basic_auth: BasicAuth,
            /// File to read bearer token for scraping targets.
            pub bearer_token_file: String,
            /// Secret to mount to read bearer token for scraping targets. The secret needs to be in the same namespace as the service monitor and accessible by the Prometheus Operator.
            pub bearer_token_secret: BearerTokenSecret,
            /// FollowRedirects configures whether scrape requests follow HTTP 3xx redirects.
            pub follow_redirects: bool,
            /// HonorLabels chooses the metric's labels on collisions with target labels.
            pub honor_labels: bool,
            /// HonorTimestamps controls whether Prometheus respects the timestamps present in scraped data.
            pub honor_timestamps: bool,
            /// Interval at which metrics should be scraped If not specified Prometheus' global scrape interval is used.
            pub interval: String,
            /// MetricRelabelConfigs to apply to samples before ingestion.
            pub metric_relabelings: Vec<MetricRelabelingsItem>,
            /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
            pub oauth2: Oauth2,
            /// Optional HTTP URL parameters
            pub params: Params,
            /// HTTP path to scrape for metrics.
            pub path: String,
            /// Name of the service port this endpoint refers to. Mutually exclusive with targetPort.
            pub port: String,
            /// ProxyURL eg http://proxyserver:2195 Directs scrapes to proxy through this endpoint.
            pub proxy_url: String,
            /// RelabelConfigs to apply to samples before scraping. Prometheus Operator automatically adds relabelings for a few standard Kubernetes fields. The original scrape job's name is available via the `__tmp_prometheus_job_name` label. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
            pub relabelings: Vec<RelabelingsItem>,
            /// HTTP scheme to use for scraping.
            pub scheme: String,
            /// Timeout after which the scrape is ended If not specified, the Prometheus global scrape timeout is used unless it is less than `Interval` in which the latter is used.
            pub scrape_timeout: String,
            /// Name or number of the target port of the Pod behind the Service, the port must be specified with container port property. Mutually exclusive with port.
            pub target_port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            /// TLS configuration to use when scraping the endpoint
            pub tls_config: TlsConfig,
        }

        /// Secret containing the client key file for the targets.
        pub struct KeySecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
        pub struct MatchExpressions {
        }

        /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
        pub struct MatchExpressionsItem {
            /// key is the label key that the selector applies to.
            pub key: String,
            /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
            pub operator: String,
            /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
            pub values: Vec<String>,
        }

        /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
        pub struct MatchLabels {
        }

        /// List of namespace names to select from.
        pub struct MatchNames {
        }

        pub struct Metadata {
        }

        /// MetricRelabelConfigs to apply to samples before ingestion.
        pub struct MetricRelabelings {
        }

        /// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion. It defines `<metric_relabel_configs>`-section of Prometheus configuration. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
        pub struct MetricRelabelingsItem {
            /// Action to perform based on regex matching. Default is 'replace'
            pub action: String,
            /// Modulus to take of the hash of the source label values.
            pub modulus: i64,
            /// Regular expression against which the extracted value is matched. Default is '(.*)'
            pub regex: String,
            /// Replacement value against which a regex replace is performed if the regular expression matches. Regex capture groups are available. Default is '$1'
            pub replacement: String,
            /// Separator placed between concatenated source label values. default is ';'.
            pub separator: String,
            /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
            pub source_labels: Vec<String>,
            /// Label to which the resulting value is written in a replace action. It is mandatory for replace actions. Regex capture groups are available.
            pub target_label: String,
        }

        /// Selector to select which namespaces the Kubernetes Endpoints objects are discovered from.
        pub struct NamespaceSelector {
            /// Boolean describing whether all namespaces are selected in contrast to a list restricting them.
            pub any: bool,
            /// List of namespace names to select from.
            pub match_names: Vec<String>,
        }

        /// OAuth2 for the URL. Only valid in Prometheus versions 2.27.0 and newer.
        pub struct Oauth2 {
            /// The secret or configmap containing the OAuth2 client id
            pub client_id: ClientId,
            /// The secret containing the OAuth2 client secret
            pub client_secret: ClientSecret,
            /// Parameters to append to the token URL
            pub endpoint_params: EndpointParams,
            /// OAuth2 scopes used for the token request
            pub scopes: Vec<String>,
            /// The URL to fetch the token from
            pub token_url: String,
        }

        /// Optional HTTP URL parameters
        pub struct Params {
        }

        /// The secret in the service monitor namespace that contains the password for authentication.
        pub struct Password {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// PodTargetLabels transfers labels on the Kubernetes `Pod` onto the created metrics.
        pub struct PodTargetLabels {
        }

        /// RelabelConfigs to apply to samples before scraping. Prometheus Operator automatically adds relabelings for a few standard Kubernetes fields. The original scrape job's name is available via the `__tmp_prometheus_job_name` label. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#relabel_config
        pub struct Relabelings {
        }

        /// RelabelConfig allows dynamic rewriting of the label set, being applied to samples before ingestion. It defines `<metric_relabel_configs>`-section of Prometheus configuration. More info: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#metric_relabel_configs
        pub struct RelabelingsItem {
            /// Action to perform based on regex matching. Default is 'replace'
            pub action: String,
            /// Modulus to take of the hash of the source label values.
            pub modulus: i64,
            /// Regular expression against which the extracted value is matched. Default is '(.*)'
            pub regex: String,
            /// Replacement value against which a regex replace is performed if the regular expression matches. Regex capture groups are available. Default is '$1'
            pub replacement: String,
            /// Separator placed between concatenated source label values. default is ';'.
            pub separator: String,
            /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
            pub source_labels: Vec<String>,
            /// Label to which the resulting value is written in a replace action. It is mandatory for replace actions. Regex capture groups are available.
            pub target_label: String,
        }

        /// OAuth2 scopes used for the token request
        pub struct Scopes {
        }

        /// Secret containing data to use for the targets.
        pub struct Secret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Selector to select Endpoints objects.
        pub struct Selector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// The source labels select values from existing labels. Their content is concatenated using the configured separator and matched against the configured regular expression for the replace, keep, and drop actions.
        pub struct SourceLabels {
        }

        /// Specification of desired Service selection for target discovery by Prometheus.
        pub struct Spec {
            /// A list of endpoints allowed as part of this ServiceMonitor.
            pub endpoints: Vec<EndpointsItem>,
            /// Chooses the label of the Kubernetes `Endpoints`. Its value will be used for the `job`-label's value of the created metrics. 
            ///  Default & fallback value: the name of the respective Kubernetes `Endpoint`.
            pub job_label: String,
            /// Per-scrape limit on number of labels that will be accepted for a sample. Only valid in Prometheus versions 2.27.0 and newer.
            pub label_limit: i64,
            /// Per-scrape limit on length of labels name that will be accepted for a sample. Only valid in Prometheus versions 2.27.0 and newer.
            pub label_name_length_limit: i64,
            /// Per-scrape limit on length of labels value that will be accepted for a sample. Only valid in Prometheus versions 2.27.0 and newer.
            pub label_value_length_limit: i64,
            /// Selector to select which namespaces the Kubernetes Endpoints objects are discovered from.
            pub namespace_selector: NamespaceSelector,
            /// PodTargetLabels transfers labels on the Kubernetes `Pod` onto the created metrics.
            pub pod_target_labels: Vec<String>,
            /// SampleLimit defines per-scrape limit on number of scraped samples that will be accepted.
            pub sample_limit: i64,
            /// Selector to select Endpoints objects.
            pub selector: Selector,
            /// TargetLabels transfers labels from the Kubernetes `Service` onto the created metrics.
            pub target_labels: Vec<String>,
            /// TargetLimit defines a limit on the number of scraped targets that will be accepted.
            pub target_limit: i64,
        }

        /// TargetLabels transfers labels from the Kubernetes `Service` onto the created metrics.
        pub struct TargetLabels {
        }

        /// TLS configuration to use when scraping the endpoint
        pub struct TlsConfig {
            /// Struct containing the CA cert to use for the targets.
            pub ca: Ca,
            /// Path to the CA cert in the Prometheus container to use for the targets.
            pub ca_file: String,
            /// Struct containing the client cert file for the targets.
            pub cert: Cert,
            /// Path to the client cert file in the Prometheus container for the targets.
            pub cert_file: String,
            /// Disable target certificate validation.
            pub insecure_skip_verify: bool,
            /// Path to the client key file in the Prometheus container for the targets.
            pub key_file: String,
            /// Secret containing the client key file for the targets.
            pub key_secret: KeySecret,
            /// Used to verify the hostname for the targets.
            pub server_name: String,
        }

        /// The secret in the service monitor namespace that contains the username for authentication.
        pub struct Username {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
        pub struct Values {
        }

        impl k8s_openapi::Resource for ServiceMonitor {
            type Scope = k8s_openapi::ClusterResourceScope;

            const API_VERSION : &'static str = "monitoring.coreos.com/v1";
            const GROUP : &'static str = "monitoring.coreos.com";
            const KIND : &'static str = "ServiceMonitor";
            const VERSION : &'static str = "v1";
            const URL_PATH_SEGMENT : &'static str = "TODO";
        }

        impl k8s_openapi::Metadata for ServiceMonitor {
            type Ty = k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

            fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
                &self.metadata
            }

            fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
                &mut self.metadata
            }
        }

}
        pub mod thanos_ruler {
        /// ThanosRuler defines a ThanosRuler deployment.
        pub struct ThanosRuler {
            pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            pub spec: Spec,
            pub status: Status,
        }

        /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
        pub struct AccessModes {
        }

        /// Added capabilities
        pub struct Add {
        }

        /// If specified, the pod's scheduling constraints.
        pub struct Affinity {
            /// Describes node affinity scheduling rules for the pod.
            pub node_affinity: NodeAffinity,
            /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
            pub pod_affinity: PodAffinity,
            /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
            pub pod_anti_affinity: PodAntiAffinity,
        }

        /// AlertDropLabels configure the label names which should be dropped in ThanosRuler alerts. The replica label `thanos_ruler_replica` will always be dropped in alerts.
        pub struct AlertDropLabels {
        }

        /// AlertRelabelConfigs configures alert relabeling in ThanosRuler. Alert relabel configurations must have the form as specified in the official Prometheus documentation: https://prometheus.io/docs/prometheus/latest/configuration/configuration/#alert_relabel_configs Alternative to AlertRelabelConfigFile, and lower order priority.
        pub struct AlertRelabelConfigs {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Define configuration for connecting to alertmanager.  Only available with thanos v0.10.0 and higher.  Maps to the `alertmanagers.config` arg.
        pub struct AlertmanagersConfig {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Define URLs to send alerts to Alertmanager.  For Thanos v0.10.0 and higher, AlertManagersConfig should be used instead.  Note: this field will be ignored if AlertManagersConfig is specified. Maps to the `alertmanagers.url` arg.
        pub struct AlertmanagersUrl {
        }

        /// The storage resource within AllocatedResources tracks the capacity allocated to a PVC. It may be larger than the actual capacity when a volume expansion operation is requested. For storage quota, the larger value from allocatedResources and PVC.spec.resources is used. If allocatedResources is not set, PVC.spec.resources alone is used for quota calculation. If a volume expansion capacity request is lowered, allocatedResources is only lowered if there are no expansion operations in progress and if the actual volume capacity is equal or lower than the requested capacity. This is an alpha field and requires enabling RecoverVolumeExpansionFailure feature.
        pub struct AllocatedResources {
        }

        /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
        pub struct Annotations {
        }

        /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
        pub struct Args {
        }

        /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
        pub struct AwsElasticBlockStore {
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
            pub partition: i32,
            /// Specify "true" to force and set the ReadOnly property in VolumeMounts to "true". If omitted, the default is "false". More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
            pub read_only: bool,
            /// Unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
            pub volume_i_d: String,
        }

        /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
        pub struct AzureDisk {
            /// Host Caching mode: None, Read Only, Read Write.
            pub caching_mode: String,
            /// The Name of the data disk in the blob storage
            pub disk_name: String,
            /// The URI the data disk in the blob storage
            pub disk_u_r_i: String,
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
            pub kind: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
        }

        /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
        pub struct AzureFile {
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// the name of secret that contains Azure Storage Account Name and Key
            pub secret_name: String,
            /// Share Name
            pub share_name: String,
        }

        /// Struct containing the CA cert to use for the targets.
        pub struct Ca {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime. Note that this field cannot be set when spec.os.name is windows.
        pub struct Capabilities {
            /// Added capabilities
            pub add: Vec<String>,
            /// Removed capabilities
            pub drop: Vec<String>,
        }

        /// Represents the actual resources of the underlying volume.
        pub struct Capacity {
        }

        /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
        pub struct Cephfs {
            /// Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub monitors: Vec<String>,
            /// Optional: Used as the mounted root, rather than the full Ceph tree, default is /
            pub path: String,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub read_only: bool,
            /// Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub secret_file: String,
            /// Optional: SecretRef is reference to the authentication secret for User, default is empty. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub secret_ref: SecretRef,
            /// Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
            pub user: String,
        }

        /// Struct containing the client cert file for the targets.
        pub struct Cert {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
        pub struct Cinder {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub fs_type: String,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub read_only: bool,
            /// Optional: points to a secret object containing parameters used to connect to OpenStack.
            pub secret_ref: SecretRef,
            /// volume id used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub volume_i_d: String,
        }

        /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
        pub struct Command {
        }

        /// Current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'.
        pub struct Conditions {
        }

        /// PersistentVolumeClaimCondition contails details about state of pvc
        pub struct ConditionsItem {
            /// Last time we probed the condition.
            pub last_probe_time: String,
            /// Last time the condition transitioned from one status to another.
            pub last_transition_time: String,
            /// Human-readable message indicating details about last transition.
            pub message: String,
            /// Unique, this should be a short, machine understandable string that gives the reason for condition's last transition. If it reports "ResizeStarted" that means the underlying persistent volume is being resized.
            pub reason: String,
            pub status: String,
            /// PersistentVolumeClaimConditionType is a valid value of PersistentVolumeClaimCondition.Type
            pub r#type: String,
        }

        /// information about the configMap data to project
        pub struct ConfigMap {
            /// If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
            pub items: Vec<ItemsItem>,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its keys must be defined
            pub optional: bool,
        }

        /// Selects a key of a ConfigMap.
        pub struct ConfigMapKeyRef {
            /// The key to select.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its key must be defined
            pub optional: bool,
        }

        /// The ConfigMap to select from
        pub struct ConfigMapRef {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap must be defined
            pub optional: bool,
        }

        /// Containers allows injecting additional containers or modifying operator generated containers. This can be used to allow adding an authentication proxy to a ThanosRuler pod or to change the behavior of an operator generated container. Containers described here modify an operator generated container if they share the same name and modifications are done via a strategic merge patch. The current container names are: `thanos-ruler` and `config-reloader`. Overriding containers is entirely outside the scope of what the maintainers will support and by doing so, you accept that this behaviour may break at any time without notice.
        pub struct Containers {
        }

        /// A single application container that you want to run within a pod.
        pub struct ContainersItem {
            /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub args: Vec<String>,
            /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub command: Vec<String>,
            /// List of environment variables to set in the container. Cannot be updated.
            pub env: Vec<EnvItem>,
            /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
            pub env_from: Vec<EnvFromItem>,
            /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
            pub image: String,
            /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
            pub image_pull_policy: String,
            /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
            pub lifecycle: Lifecycle,
            /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub liveness_probe: LivenessProbe,
            /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
            pub name: String,
            /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
            pub ports: Vec<PortsItem>,
            /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub readiness_probe: ReadinessProbe,
            /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub resources: Resources,
            /// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
            pub security_context: SecurityContext,
            /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub startup_probe: StartupProbe,
            /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
            pub stdin: bool,
            /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
            pub stdin_once: bool,
            /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
            pub termination_message_path: String,
            /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
            pub termination_message_policy: String,
            /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
            pub tty: bool,
            /// volumeDevices is the list of block devices to be used by the container.
            pub volume_devices: Vec<VolumeDevicesItem>,
            /// Pod volumes to mount into the container's filesystem. Cannot be updated.
            pub volume_mounts: Vec<VolumeMountsItem>,
            /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
            pub working_dir: String,
        }

        /// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
        pub struct Csi {
            /// Driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
            pub driver: String,
            /// Filesystem type to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
            pub fs_type: String,
            /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
            pub node_publish_secret_ref: NodePublishSecretRef,
            /// Specifies a read-only configuration for the volume. Defaults to false (read/write).
            pub read_only: bool,
            /// VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
            pub volume_attributes: VolumeAttributes,
        }

        /// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.
        pub struct DataSource {
            /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
            pub api_group: String,
            /// Kind is the type of resource being referenced
            pub kind: String,
            /// Name is the name of resource being referenced
            pub name: String,
        }

        /// Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef allows any non-core object, as well as PersistentVolumeClaim objects. * While DataSource ignores disallowed values (dropping them), DataSourceRef preserves all values, and generates an error if a disallowed value is specified. (Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
        pub struct DataSourceRef {
            /// APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
            pub api_group: String,
            /// Kind is the type of resource being referenced
            pub kind: String,
            /// Name is the name of resource being referenced
            pub name: String,
        }

        /// information about the downwardAPI data to project
        pub struct DownwardAPI {
            /// Items is a list of DownwardAPIVolume file
            pub items: Vec<ItemsItem>,
        }

        /// Removed capabilities
        pub struct Drop {
        }

        /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
        pub struct EmptyDir {
            /// What type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
            pub medium: String,
            /// Total amount of local storage required for this EmptyDir volume. The size limit is also applicable for memory medium. The maximum usage on memory medium EmptyDir would be the minimum value between the SizeLimit specified here and the sum of memory limits of all containers in a pod. The default is nil which means that the limit is undefined. More info: http://kubernetes.io/docs/user-guide/volumes#emptydir
            pub size_limit: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        }

        /// List of environment variables to set in the container. Cannot be updated.
        pub struct Env {
        }

        /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
        pub struct EnvFrom {
        }

        /// EnvFromSource represents the source of a set of ConfigMaps
        pub struct EnvFromItem {
            /// The ConfigMap to select from
            pub config_map_ref: ConfigMapRef,
            /// An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
            pub prefix: String,
            /// The Secret to select from
            pub secret_ref: SecretRef,
        }

        /// EnvVar represents an environment variable present in a Container.
        pub struct EnvItem {
            /// Name of the environment variable. Must be a C_IDENTIFIER.
            pub name: String,
            /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
            pub value: String,
            /// Source for the environment variable's value. Cannot be used if value is not empty.
            pub value_from: ValueFrom,
        }

        /// Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed. 
        ///  Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity tracking are needed, c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through a PersistentVolumeClaim (see EphemeralVolumeSource for more information on the connection between this volume type and PersistentVolumeClaim). 
        ///  Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod. 
        ///  Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information. 
        ///  A pod can use both types of ephemeral volumes and persistent volumes at the same time.
        pub struct Ephemeral {
            /// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long). 
            ///  An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster. 
            ///  This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created. 
            ///  Required, must not be nil.
            pub volume_claim_template: VolumeClaimTemplate,
        }

        /// List of references to PrometheusRule objects to be excluded from enforcing a namespace label of origin. Applies only if enforcedNamespaceLabel set to true.
        pub struct ExcludedFromEnforcement {
        }

        /// ObjectReference references a PodMonitor, ServiceMonitor, Probe or PrometheusRule object.
        pub struct ExcludedFromEnforcementItem {
            /// Group of the referent. When not specified, it defaults to `monitoring.coreos.com`
            pub group: String,
            /// Name of the referent. When not set, all resources are matched.
            pub name: String,
            /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
            pub namespace: String,
            /// Resource of the referent.
            pub resource: String,
        }

        /// Exec specifies the action to take.
        pub struct Exec {
            /// Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
            pub command: Vec<String>,
        }

        /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
        pub struct Fc {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// Optional: FC target lun number
            pub lun: i32,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// Optional: FC target worldwide names (WWNs)
            pub target_w_w_ns: Vec<String>,
            /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
            pub wwids: Vec<String>,
        }

        /// Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
        pub struct FieldRef {
            /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
            pub api_version: String,
            /// Path of the field to select in the specified API version.
            pub field_path: String,
        }

        /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
        pub struct FlexVolume {
            /// Driver is the name of the driver to use for this volume.
            pub driver: String,
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
            pub fs_type: String,
            /// Optional: Extra command options if any.
            pub options: Options,
            /// Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// Optional: SecretRef is reference to the secret object containing sensitive information to pass to the plugin scripts. This may be empty if no secret object is specified. If the secret object contains more than one secret, all secrets are passed to the plugin scripts.
            pub secret_ref: SecretRef,
        }

        /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
        pub struct Flocker {
            /// Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
            pub dataset_name: String,
            /// UUID of the dataset. This is unique identifier of a Flocker dataset
            pub dataset_u_u_i_d: String,
        }

        /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
        pub struct GcePersistentDisk {
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub partition: i32,
            /// Unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub pd_name: String,
            /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub read_only: bool,
        }

        /// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
        pub struct GitRepo {
            /// Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
            pub directory: String,
            /// Repository URL
            pub repository: String,
            /// Commit hash for the specified revision.
            pub revision: String,
        }

        /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
        pub struct Glusterfs {
            /// EndpointsName is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
            pub endpoints: String,
            /// Path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
            pub path: String,
            /// ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
            pub read_only: bool,
        }

        /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
        pub struct Grpc {
            /// Port number of the gRPC service. Number must be in the range 1 to 65535.
            pub port: i32,
            /// Service is the name of the service to place in the gRPC HealthCheckRequest (see https://github.com/grpc/grpc/blob/master/doc/health-checking.md). 
            ///  If this is not specified, the default behavior is defined by gRPC.
            pub service: String,
        }

        /// GRPCServerTLSConfig configures the gRPC server from which Thanos Querier reads recorded rule data. Note: Currently only the CAFile, CertFile, and KeyFile fields are supported. Maps to the '--grpc-server-tls-*' CLI args.
        pub struct GrpcServerTlsConfig {
            /// Struct containing the CA cert to use for the targets.
            pub ca: Ca,
            /// Path to the CA cert in the Prometheus container to use for the targets.
            pub ca_file: String,
            /// Struct containing the client cert file for the targets.
            pub cert: Cert,
            /// Path to the client cert file in the Prometheus container for the targets.
            pub cert_file: String,
            /// Disable target certificate validation.
            pub insecure_skip_verify: bool,
            /// Path to the client key file in the Prometheus container for the targets.
            pub key_file: String,
            /// Secret containing the client key file for the targets.
            pub key_secret: KeySecret,
            /// Used to verify the hostname for the targets.
            pub server_name: String,
        }

        /// Pods' hostAliases configuration
        pub struct HostAliases {
        }

        /// HostAlias holds the mapping between IP and hostnames that will be injected as an entry in the pod's hosts file.
        pub struct HostAliasesItem {
            /// Hostnames for the above IP address.
            pub hostnames: Vec<String>,
            /// IP address of the host file entry.
            pub ip: String,
        }

        /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath --- TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not mount host directories as read/write.
        pub struct HostPath {
            /// Path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
            pub path: String,
            /// Type for HostPath Volume Defaults to "" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
            pub r#type: String,
        }

        /// Hostnames for the above IP address.
        pub struct Hostnames {
        }

        /// HTTPGet specifies the http request to perform.
        pub struct HttpGet {
            /// Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
            pub host: String,
            /// Custom headers to set in the request. HTTP allows repeated headers.
            pub http_headers: Vec<HttpHeadersItem>,
            /// Path to access on the HTTP server.
            pub path: String,
            /// Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
            pub port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            /// Scheme to use for connecting to the host. Defaults to HTTP.
            pub scheme: String,
        }

        /// Custom headers to set in the request. HTTP allows repeated headers.
        pub struct HttpHeaders {
        }

        /// HTTPHeader describes a custom header to be used in HTTP probes
        pub struct HttpHeadersItem {
            /// The header field name
            pub name: String,
            /// The header field value
            pub value: String,
        }

        /// An optional list of references to secrets in the same namespace to use for pulling thanos images from registries see http://kubernetes.io/docs/user-guide/images#specifying-imagepullsecrets-on-a-pod
        pub struct ImagePullSecrets {
        }

        /// LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
        pub struct ImagePullSecretsItem {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
        }

        /// InitContainers allows adding initContainers to the pod definition. Those can be used to e.g. fetch secrets for injection into the ThanosRuler configuration from external sources. Any errors during the execution of an initContainer will lead to a restart of the Pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/init-containers/ Using initContainers for any use case other then secret fetching is entirely outside the scope of what the maintainers will support and by doing so, you accept that this behaviour may break at any time without notice.
        pub struct InitContainers {
        }

        /// A single application container that you want to run within a pod.
        pub struct InitContainersItem {
            /// Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub args: Vec<String>,
            /// Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
            pub command: Vec<String>,
            /// List of environment variables to set in the container. Cannot be updated.
            pub env: Vec<EnvItem>,
            /// List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
            pub env_from: Vec<EnvFromItem>,
            /// Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
            pub image: String,
            /// Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
            pub image_pull_policy: String,
            /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
            pub lifecycle: Lifecycle,
            /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub liveness_probe: LivenessProbe,
            /// Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
            pub name: String,
            /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
            pub ports: Vec<PortsItem>,
            /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub readiness_probe: ReadinessProbe,
            /// Compute Resources required by this container. Cannot be updated. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub resources: Resources,
            /// SecurityContext defines the security options the container should be run with. If set, the fields of SecurityContext override the equivalent fields of PodSecurityContext. More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
            pub security_context: SecurityContext,
            /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub startup_probe: StartupProbe,
            /// Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
            pub stdin: bool,
            /// Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
            pub stdin_once: bool,
            /// Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
            pub termination_message_path: String,
            /// Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
            pub termination_message_policy: String,
            /// Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
            pub tty: bool,
            /// volumeDevices is the list of block devices to be used by the container.
            pub volume_devices: Vec<VolumeDevicesItem>,
            /// Pod volumes to mount into the container's filesystem. Cannot be updated.
            pub volume_mounts: Vec<VolumeMountsItem>,
            /// Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
            pub working_dir: String,
        }

        /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
        pub struct Iscsi {
            /// whether support iSCSI Discovery CHAP authentication
            pub chap_auth_discovery: bool,
            /// whether support iSCSI Session CHAP authentication
            pub chap_auth_session: bool,
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// Custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
            pub initiator_name: String,
            /// Target iSCSI Qualified Name.
            pub iqn: String,
            /// iSCSI Interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
            pub iscsi_interface: String,
            /// iSCSI Target Lun number.
            pub lun: i32,
            /// iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
            pub portals: Vec<String>,
            /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
            pub read_only: bool,
            /// CHAP Secret for iSCSI target and initiator authentication
            pub secret_ref: SecretRef,
            /// iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
            pub target_portal: String,
        }

        /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
        pub struct Items {
        }

        /// Maps a string key to a path within a volume.
        pub struct ItemsItem {
            /// The key to project.
            pub key: String,
            /// Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
            pub mode: i32,
            /// The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
            pub path: String,
        }

        /// Secret containing the client key file for the targets.
        pub struct KeySecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
        pub struct LabelSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
        pub struct Labels {
        }

        /// Actions that the management system should take in response to container lifecycle events. Cannot be updated.
        pub struct Lifecycle {
            /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
            pub post_start: PostStart,
            /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
            pub pre_stop: PreStop,
        }

        /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
        pub struct Limits {
        }

        /// Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
        pub struct LivenessProbe {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
            pub failure_threshold: i32,
            /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
            pub grpc: Grpc,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub initial_delay_seconds: i32,
            /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
            pub period_seconds: i32,
            /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
            pub success_threshold: i32,
            /// TCPSocket specifies an action involving a TCP port.
            pub tcp_socket: TcpSocket,
            /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
            pub termination_grace_period_seconds: i64,
            /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub timeout_seconds: i32,
        }

        /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
        pub struct MatchExpressions {
        }

        /// A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
        pub struct MatchExpressionsItem {
            /// key is the label key that the selector applies to.
            pub key: String,
            /// operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
            pub operator: String,
            /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
            pub values: Vec<String>,
        }

        /// A list of node selector requirements by node's fields.
        pub struct MatchFields {
        }

        /// A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
        pub struct MatchFieldsItem {
            /// The label key that the selector applies to.
            pub key: String,
            /// Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
            pub operator: String,
            /// An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
            pub values: Vec<String>,
        }

        /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
        pub struct MatchLabels {
        }

        /// May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
        pub struct Metadata {
        }

        /// A collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
        pub struct Monitors {
        }

        /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
        pub struct NamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
        pub struct Namespaces {
        }

        /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
        pub struct Nfs {
            /// Path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub path: String,
            /// ReadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub read_only: bool,
            /// Server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub server: String,
        }

        /// Describes node affinity scheduling rules for the pod.
        pub struct NodeAffinity {
            /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
            pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredDuringSchedulingIgnoredDuringExecutionItem>,
            /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
            pub required_during_scheduling_ignored_during_execution: RequiredDuringSchedulingIgnoredDuringExecution,
        }

        /// NodePublishSecretRef is a reference to the secret object containing sensitive information to pass to the CSI driver to complete the CSI NodePublishVolume and NodeUnpublishVolume calls. This field is optional, and  may be empty if no secret is required. If the secret object contains more than one secret, all secret references are passed.
        pub struct NodePublishSecretRef {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
        }

        /// Define which Nodes the Pods are scheduled on.
        pub struct NodeSelector {
        }

        /// Required. A list of node selector terms. The terms are ORed.
        pub struct NodeSelectorTerms {
        }

        /// A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
        pub struct NodeSelectorTermsItem {
            /// A list of node selector requirements by node's labels.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// A list of node selector requirements by node's fields.
            pub match_fields: Vec<MatchFieldsItem>,
        }

        /// ObjectStorageConfig configures object storage in Thanos. Alternative to ObjectStorageConfigFile, and lower order priority.
        pub struct ObjectStorageConfig {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Optional: Extra command options if any.
        pub struct Options {
        }

        /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
        pub struct PersistentVolumeClaim {
            /// ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
            pub claim_name: String,
            /// Will force the ReadOnly setting in VolumeMounts. Default false.
            pub read_only: bool,
        }

        /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
        pub struct PhotonPersistentDisk {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// ID that identifies Photon Controller persistent disk
            pub pd_i_d: String,
        }

        /// Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
        pub struct PodAffinity {
            /// The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
            pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredDuringSchedulingIgnoredDuringExecutionItem>,
            /// If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
            pub required_during_scheduling_ignored_during_execution: Vec<RequiredDuringSchedulingIgnoredDuringExecutionItem>,
        }

        /// Required. A pod affinity term, associated with the corresponding weight.
        pub struct PodAffinityTerm {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: LabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: NamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

        /// Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
        pub struct PodAntiAffinity {
            /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
            pub preferred_during_scheduling_ignored_during_execution: Vec<PreferredDuringSchedulingIgnoredDuringExecutionItem>,
            /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
            pub required_during_scheduling_ignored_during_execution: Vec<RequiredDuringSchedulingIgnoredDuringExecutionItem>,
        }

        /// PodMetadata contains Labels and Annotations gets propagated to the thanos ruler pods.
        pub struct PodMetadata {
            /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
            pub annotations: Annotations,
            /// Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
            pub labels: Labels,
            /// Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names
            pub name: String,
        }

        /// iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
        pub struct Portals {
        }

        /// List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
        pub struct Ports {
        }

        /// ContainerPort represents a network port in a single container.
        pub struct PortsItem {
            /// Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
            pub container_port: i32,
            /// What host IP to bind the external port to.
            pub host_i_p: String,
            /// Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
            pub host_port: i32,
            /// If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
            pub name: String,
            /// Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
            pub protocol: String,
        }

        /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
        pub struct PortworxVolume {
            /// FSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// VolumeID uniquely identifies a Portworx volume
            pub volume_i_d: String,
        }

        /// PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
        pub struct PostStart {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
            pub tcp_socket: TcpSocket,
        }

        /// PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The Pod's termination grace period countdown begins before the PreStop hook is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period (unless delayed by finalizers). Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
        pub struct PreStop {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Deprecated. TCPSocket is NOT supported as a LifecycleHandler and kept for the backward compatibility. There are no validation of this field and lifecycle hooks will fail in runtime when tcp handler is specified.
            pub tcp_socket: TcpSocket,
        }

        /// A node selector term, associated with the corresponding weight.
        pub struct Preference {
            /// A list of node selector requirements by node's labels.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// A list of node selector requirements by node's fields.
            pub match_fields: Vec<MatchFieldsItem>,
        }

        /// The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
        pub struct PreferredDuringSchedulingIgnoredDuringExecution {
        }

        /// The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
        pub struct PreferredDuringSchedulingIgnoredDuringExecutionItem {
            /// Required. A pod affinity term, associated with the corresponding weight.
            pub pod_affinity_term: PodAffinityTerm,
            /// weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
            pub weight: i32,
        }

        /// Items for all in one resources secrets, configmaps, and downward API
        pub struct Projected {
            /// Mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
            pub default_mode: i32,
            /// list of volume projections
            pub sources: Vec<SourcesItem>,
        }

        /// PrometheusRulesExcludedFromEnforce - list of Prometheus rules to be excluded from enforcing of adding namespace labels. Works only if enforcedNamespaceLabel set to true. Make sure both ruleNamespace and ruleName are set for each pair Deprecated: use excludedFromEnforcement instead.
        pub struct PrometheusRulesExcludedFromEnforce {
        }

        /// PrometheusRuleExcludeConfig enables users to configure excluded PrometheusRule names and their namespaces to be ignored while enforcing namespace label for alerts and metrics.
        pub struct PrometheusRulesExcludedFromEnforceItem {
            /// RuleNamespace - name of excluded rule
            pub rule_name: String,
            /// RuleNamespace - namespace of excluded rule
            pub rule_namespace: String,
        }

        /// Define configuration for connecting to thanos query instances. If this is defined, the QueryEndpoints field will be ignored. Maps to the `query.config` CLI argument. Only available with thanos v0.11.0 and higher.
        pub struct QueryConfig {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// QueryEndpoints defines Thanos querier endpoints from which to query metrics. Maps to the --query flag of thanos ruler.
        pub struct QueryEndpoints {
        }

        /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
        pub struct Quobyte {
            /// Group to map volume access to Default is no group
            pub group: String,
            /// ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
            pub read_only: bool,
            /// Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
            pub registry: String,
            /// Tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin
            pub tenant: String,
            /// User to map volume access to Defaults to serivceaccount user
            pub user: String,
            /// Volume is a string that references an already created Quobyte volume by name.
            pub volume: String,
        }

        /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
        pub struct Rbd {
            /// Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd TODO: how do we prevent errors in the filesystem from compromising the machine
            pub fs_type: String,
            /// The rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub image: String,
            /// Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub keyring: String,
            /// A collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub monitors: Vec<String>,
            /// The rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub pool: String,
            /// ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub read_only: bool,
            /// SecretRef is name of the authentication secret for RBDUser. If provided overrides keyring. Default is nil. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub secret_ref: SecretRef,
            /// The rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
            pub user: String,
        }

        /// Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
        pub struct ReadinessProbe {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
            pub failure_threshold: i32,
            /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
            pub grpc: Grpc,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub initial_delay_seconds: i32,
            /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
            pub period_seconds: i32,
            /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
            pub success_threshold: i32,
            /// TCPSocket specifies an action involving a TCP port.
            pub tcp_socket: TcpSocket,
            /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
            pub termination_grace_period_seconds: i64,
            /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub timeout_seconds: i32,
        }

        /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
        pub struct Requests {
        }

        /// If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
        pub struct RequiredDuringSchedulingIgnoredDuringExecution {
        }

        /// Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
        pub struct RequiredDuringSchedulingIgnoredDuringExecutionItem {
            /// A label query over a set of resources, in this case pods.
            pub label_selector: LabelSelector,
            /// A label query over the set of namespaces that the term applies to. The term is applied to the union of the namespaces selected by this field and the ones listed in the namespaces field. null selector and null or empty namespaces list means "this pod's namespace". An empty selector ({}) matches all namespaces. This field is beta-level and is only honored when PodAffinityNamespaceSelector feature is enabled.
            pub namespace_selector: NamespaceSelector,
            /// namespaces specifies a static list of namespace names that the term applies to. The term is applied to the union of the namespaces listed in this field and the ones selected by namespaceSelector. null or empty namespaces list and null namespaceSelector means "this pod's namespace"
            pub namespaces: Vec<String>,
            /// This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
            pub topology_key: String,
        }

        /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
        pub struct ResourceFieldRef {
            /// Container name: required for volumes, optional for env vars
            pub container_name: String,
            /// Specifies the output format of the exposed resources, defaults to "1"
            pub divisor: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
            /// Required: resource to select
            pub resource: String,
        }

        /// Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
        pub struct Resources {
            /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub limits: Limits,
            /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
            pub requests: Requests,
        }

        /// Namespaces to be selected for Rules discovery. If unspecified, only the same namespace as the ThanosRuler object is in is used.
        pub struct RuleNamespaceSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// A label selector to select which PrometheusRules to mount for alerting and recording.
        pub struct RuleSelector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
        pub struct ScaleIO {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
            pub fs_type: String,
            /// The host address of the ScaleIO API Gateway.
            pub gateway: String,
            /// The name of the ScaleIO Protection Domain for the configured storage.
            pub protection_domain: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// SecretRef references to the secret for ScaleIO user and other sensitive information. If this is not provided, Login operation will fail.
            pub secret_ref: SecretRef,
            /// Flag to enable/disable SSL communication with Gateway, default false
            pub ssl_enabled: bool,
            /// Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
            pub storage_mode: String,
            /// The ScaleIO Storage Pool associated with the protection domain.
            pub storage_pool: String,
            /// The name of the storage system as configured in ScaleIO.
            pub system: String,
            /// The name of a volume already created in the ScaleIO system that is associated with this volume source.
            pub volume_name: String,
        }

        /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
        pub struct SeLinuxOptions {
            /// Level is SELinux level label that applies to the container.
            pub level: String,
            /// Role is a SELinux role label that applies to the container.
            pub role: String,
            /// Type is a SELinux type label that applies to the container.
            pub r#type: String,
            /// User is a SELinux user label that applies to the container.
            pub user: String,
        }

        /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
        pub struct SeccompProfile {
            /// localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
            pub localhost_profile: String,
            /// type indicates which kind of seccomp profile will be applied. Valid options are: 
            ///  Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
            pub r#type: String,
        }

        /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
        pub struct Secret {
            /// Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
            pub default_mode: i32,
            /// If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
            pub items: Vec<ItemsItem>,
            /// Specify whether the Secret or its keys must be defined
            pub optional: bool,
            /// Name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
            pub secret_name: String,
        }

        /// Selects a key of a secret in the pod's namespace
        pub struct SecretKeyRef {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
        pub struct SecretRef {
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
        }

        /// SecurityContext holds pod-level security attributes and common container settings. This defaults to the default PodSecurityContext.
        pub struct SecurityContext {
            /// A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod: 
            ///  1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw---- 
            ///  If unset, the Kubelet will not modify the ownership and permissions of any volume. Note that this field cannot be set when spec.os.name is windows.
            pub fs_group: i64,
            /// fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used. Note that this field cannot be set when spec.os.name is windows.
            pub fs_group_change_policy: String,
            /// The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
            pub run_as_group: i64,
            /// Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
            pub run_as_non_root: bool,
            /// The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
            pub run_as_user: i64,
            /// The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container. Note that this field cannot be set when spec.os.name is windows.
            pub se_linux_options: SeLinuxOptions,
            /// The seccomp options to use by the containers in this pod. Note that this field cannot be set when spec.os.name is windows.
            pub seccomp_profile: SeccompProfile,
            /// A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container. Note that this field cannot be set when spec.os.name is windows.
            pub supplemental_groups: Vec<i64>,
            /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
            pub sysctls: Vec<SysctlsItem>,
            /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
            pub windows_options: WindowsOptions,
        }

        /// A label query over volumes to consider for binding.
        pub struct Selector {
            /// matchExpressions is a list of label selector requirements. The requirements are ANDed.
            pub match_expressions: Vec<MatchExpressionsItem>,
            /// matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
            pub match_labels: MatchLabels,
        }

        /// information about the serviceAccountToken data to project
        pub struct ServiceAccountToken {
            /// Audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.
            pub audience: String,
            /// ExpirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.
            pub expiration_seconds: i64,
            /// Path is the path relative to the mount point of the file to project the token into.
            pub path: String,
        }

        /// list of volume projections
        pub struct Sources {
        }

        /// Projection that may be projected along with other supported volume types
        pub struct SourcesItem {
            /// information about the configMap data to project
            pub config_map: ConfigMap,
            /// information about the downwardAPI data to project
            pub downward_a_p_i: DownwardAPI,
            /// information about the secret data to project
            pub secret: Secret,
            /// information about the serviceAccountToken data to project
            pub service_account_token: ServiceAccountToken,
        }

        /// The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
        pub struct Spec {
            /// AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
            pub access_modes: Vec<String>,
            /// This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source. If the AnyVolumeDataSource feature gate is enabled, this field will always have the same contents as the DataSourceRef field.
            pub data_source: DataSource,
            /// Specifies the object from which to populate the volume with data, if a non-empty volume is desired. This may be any local object from a non-empty API group (non core object) or a PersistentVolumeClaim object. When this field is specified, volume binding will only succeed if the type of the specified object matches some installed volume populator or dynamic provisioner. This field will replace the functionality of the DataSource field and as such if both fields are non-empty, they must have the same value. For backwards compatibility, both fields (DataSource and DataSourceRef) will be set to the same value automatically if one of them is empty and the other is non-empty. There are two important differences between DataSource and DataSourceRef: * While DataSource only allows two specific types of objects, DataSourceRef allows any non-core object, as well as PersistentVolumeClaim objects. * While DataSource ignores disallowed values (dropping them), DataSourceRef preserves all values, and generates an error if a disallowed value is specified. (Alpha) Using this field requires the AnyVolumeDataSource feature gate to be enabled.
            pub data_source_ref: DataSourceRef,
            /// Resources represents the minimum resources the volume should have. If RecoverVolumeExpansionFailure feature is enabled users are allowed to specify resource requirements that are lower than previous value but must still be higher than capacity recorded in the status field of the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources
            pub resources: Resources,
            /// A label query over volumes to consider for binding.
            pub selector: Selector,
            /// Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
            pub storage_class_name: String,
            /// volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
            pub volume_mode: String,
            /// VolumeName is the binding reference to the PersistentVolume backing this claim.
            pub volume_name: String,
        }

        /// StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
        pub struct StartupProbe {
            /// Exec specifies the action to take.
            pub exec: Exec,
            /// Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
            pub failure_threshold: i32,
            /// GRPC specifies an action involving a GRPC port. This is an alpha field and requires enabling GRPCContainerProbe feature gate.
            pub grpc: Grpc,
            /// HTTPGet specifies the http request to perform.
            pub http_get: HttpGet,
            /// Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub initial_delay_seconds: i32,
            /// How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
            pub period_seconds: i32,
            /// Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
            pub success_threshold: i32,
            /// TCPSocket specifies an action involving a TCP port.
            pub tcp_socket: TcpSocket,
            /// Optional duration in seconds the pod needs to terminate gracefully upon probe failure. The grace period is the duration in seconds after the processes running in the pod are sent a termination signal and the time when the processes are forcibly halted with a kill signal. Set this value longer than the expected cleanup time for your process. If this value is nil, the pod's terminationGracePeriodSeconds will be used. Otherwise, this value overrides the value provided by the pod spec. Value must be non-negative integer. The value zero indicates stop immediately via the kill signal (no opportunity to shut down). This is a beta field and requires enabling ProbeTerminationGracePeriod feature gate. Minimum value is 1. spec.terminationGracePeriodSeconds is used if unset.
            pub termination_grace_period_seconds: i64,
            /// Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
            pub timeout_seconds: i32,
        }

        /// Most recent observed status of the ThanosRuler cluster. Read-only. Not included when requesting from the apiserver, only from the ThanosRuler Operator API itself. More info: https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
        pub struct Status {
            /// Total number of available pods (ready for at least minReadySeconds) targeted by this ThanosRuler deployment.
            pub available_replicas: i32,
            /// Represents whether any actions on the underlying managed objects are being performed. Only delete actions will be performed.
            pub paused: bool,
            /// Total number of non-terminated pods targeted by this ThanosRuler deployment (their labels match the selector).
            pub replicas: i32,
            /// Total number of unavailable pods targeted by this ThanosRuler deployment.
            pub unavailable_replicas: i32,
            /// Total number of non-terminated pods targeted by this ThanosRuler deployment that have the desired version spec.
            pub updated_replicas: i32,
        }

        /// Storage spec to specify how storage shall be used.
        pub struct Storage {
            /// Deprecated: subPath usage will be disabled by default in a future release, this option will become unnecessary. DisableMountSubPath allows to remove any subPath usage in volume mounts.
            pub disable_mount_sub_path: bool,
            /// EmptyDirVolumeSource to be used by the Prometheus StatefulSets. If specified, used in place of any volumeClaimTemplate. More info: https://kubernetes.io/docs/concepts/storage/volumes/#emptydir
            pub empty_dir: EmptyDir,
            /// EphemeralVolumeSource to be used by the Prometheus StatefulSets. This is a beta field in k8s 1.21, for lower versions, starting with k8s 1.19, it requires enabling the GenericEphemeralVolume feature gate. More info: https://kubernetes.io/docs/concepts/storage/ephemeral-volumes/#generic-ephemeral-volumes
            pub ephemeral: Ephemeral,
            /// A PVC spec to be used by the Prometheus StatefulSets.
            pub volume_claim_template: VolumeClaimTemplate,
        }

        /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
        pub struct Storageos {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
            pub read_only: bool,
            /// SecretRef specifies the secret to use for obtaining the StorageOS API credentials.  If not specified, default values will be attempted.
            pub secret_ref: SecretRef,
            /// VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
            pub volume_name: String,
            /// VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
            pub volume_namespace: String,
        }

        /// A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container. Note that this field cannot be set when spec.os.name is windows.
        pub struct SupplementalGroups {
        }

        /// Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch. Note that this field cannot be set when spec.os.name is windows.
        pub struct Sysctls {
        }

        /// Sysctl defines a kernel parameter to be set
        pub struct SysctlsItem {
            /// Name of a property to set
            pub name: String,
            /// Value of a property to set
            pub value: String,
        }

        /// Optional: FC target worldwide names (WWNs)
        pub struct TargetWWNs {
        }

        /// TCPSocket specifies an action involving a TCP port.
        pub struct TcpSocket {
            /// Optional: Host name to connect to, defaults to the pod IP.
            pub host: String,
            /// Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
            pub port: k8s_openapi::apimachinery::pkg::util::intstr::IntOrString,
        }

        /// If specified, the pod's tolerations.
        pub struct Tolerations {
        }

        /// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
        pub struct TolerationsItem {
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

        /// If specified, the pod's topology spread constraints.
        pub struct TopologySpreadConstraints {
        }

        /// TopologySpreadConstraint specifies how to spread matching pods among the given topology.
        pub struct TopologySpreadConstraintsItem {
            /// LabelSelector is used to find matching pods. Pods that match this label selector are counted to determine the number of pods in their corresponding topology domain.
            pub label_selector: LabelSelector,
            /// MaxSkew describes the degree to which pods may be unevenly distributed. When `whenUnsatisfiable=DoNotSchedule`, it is the maximum permitted difference between the number of matching pods in the target topology and the global minimum. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 1/1/0: | zone1 | zone2 | zone3 | |   P   |   P   |       | - if MaxSkew is 1, incoming pod can only be scheduled to zone3 to become 1/1/1; scheduling it onto zone1(zone2) would make the ActualSkew(2-0) on zone1(zone2) violate MaxSkew(1). - if MaxSkew is 2, incoming pod can be scheduled onto any zone. When `whenUnsatisfiable=ScheduleAnyway`, it is used to give higher precedence to topologies that satisfy it. It's a required field. Default value is 1 and 0 is not allowed.
            pub max_skew: i32,
            /// TopologyKey is the key of node labels. Nodes that have a label with this key and identical values are considered to be in the same topology. We consider each <key, value> as a "bucket", and try to put balanced number of pods into each bucket. It's a required field.
            pub topology_key: String,
            /// WhenUnsatisfiable indicates how to deal with a pod if it doesn't satisfy the spread constraint. - DoNotSchedule (default) tells the scheduler not to schedule it. - ScheduleAnyway tells the scheduler to schedule the pod in any location, but giving higher precedence to topologies that would help reduce the skew. A constraint is considered "Unsatisfiable" for an incoming pod if and only if every possible node assignment for that pod would violate "MaxSkew" on some topology. For example, in a 3-zone cluster, MaxSkew is set to 1, and pods with the same labelSelector spread as 3/1/1: | zone1 | zone2 | zone3 | | P P P |   P   |   P   | If WhenUnsatisfiable is set to DoNotSchedule, incoming pod can only be scheduled to zone2(zone3) to become 3/2/1(3/1/2) as ActualSkew(2-1) on zone2(zone3) satisfies MaxSkew(1). In other words, the cluster can still be imbalanced, but scheduler won't make it *more* imbalanced. It's a required field.
            pub when_unsatisfiable: String,
        }

        /// TracingConfig configures tracing in Thanos. This is an experimental feature, it may change in any upcoming release in a breaking way.
        pub struct TracingConfig {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Source for the environment variable's value. Cannot be used if value is not empty.
        pub struct ValueFrom {
            /// Selects a key of a ConfigMap.
            pub config_map_key_ref: ConfigMapKeyRef,
            /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
            pub field_ref: FieldRef,
            /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
            pub resource_field_ref: ResourceFieldRef,
            /// Selects a key of a secret in the pod's namespace
            pub secret_key_ref: SecretKeyRef,
        }

        /// values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
        pub struct Values {
        }

        /// VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
        pub struct VolumeAttributes {
        }

        /// Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long). 
        ///  An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster. 
        ///  This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created. 
        ///  Required, must not be nil.
        pub struct VolumeClaimTemplate {
            /// May contain labels and annotations that will be copied into the PVC when creating it. No other fields are allowed and will be rejected during validation.
            pub metadata: Metadata,
            /// The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
            pub spec: Spec,
        }

        /// volumeDevices is the list of block devices to be used by the container.
        pub struct VolumeDevices {
        }

        /// volumeDevice describes a mapping of a raw block device within a container.
        pub struct VolumeDevicesItem {
            /// devicePath is the path inside of the container that the device will be mapped to.
            pub device_path: String,
            /// name must match the name of a persistentVolumeClaim in the pod
            pub name: String,
        }

        /// Pod volumes to mount into the container's filesystem. Cannot be updated.
        pub struct VolumeMounts {
        }

        /// VolumeMount describes a mounting of a Volume within a container.
        pub struct VolumeMountsItem {
            /// Path within the container at which the volume should be mounted.  Must not contain ':'.
            pub mount_path: String,
            /// mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
            pub mount_propagation: String,
            /// This must match the Name of a Volume.
            pub name: String,
            /// Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
            pub read_only: bool,
            /// Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
            pub sub_path: String,
            /// Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
            pub sub_path_expr: String,
        }

        /// Volumes allows configuration of additional volumes on the output StatefulSet definition. Volumes specified will be appended to other volumes that are generated as a result of StorageSpec objects.
        pub struct Volumes {
        }

        /// Volume represents a named volume in a pod that may be accessed by any container in the pod.
        pub struct VolumesItem {
            /// AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
            pub aws_elastic_block_store: AwsElasticBlockStore,
            /// AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
            pub azure_disk: AzureDisk,
            /// AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
            pub azure_file: AzureFile,
            /// CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
            pub cephfs: Cephfs,
            /// Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
            pub cinder: Cinder,
            /// ConfigMap represents a configMap that should populate this volume
            pub config_map: ConfigMap,
            /// CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
            pub csi: Csi,
            /// DownwardAPI represents downward API about the pod that should populate this volume
            pub downward_a_p_i: DownwardAPI,
            /// EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
            pub empty_dir: EmptyDir,
            /// Ephemeral represents a volume that is handled by a cluster storage driver. The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed. 
            ///  Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity tracking are needed, c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through a PersistentVolumeClaim (see EphemeralVolumeSource for more information on the connection between this volume type and PersistentVolumeClaim). 
            ///  Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod. 
            ///  Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information. 
            ///  A pod can use both types of ephemeral volumes and persistent volumes at the same time.
            pub ephemeral: Ephemeral,
            /// FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
            pub fc: Fc,
            /// FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
            pub flex_volume: FlexVolume,
            /// Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
            pub flocker: Flocker,
            /// GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
            pub gce_persistent_disk: GcePersistentDisk,
            /// GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
            pub git_repo: GitRepo,
            /// Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
            pub glusterfs: Glusterfs,
            /// HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath --- TODO(jonesdl) We need to restrict who can use host directory mounts and who can/can not mount host directories as read/write.
            pub host_path: HostPath,
            /// ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
            pub iscsi: Iscsi,
            /// Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
            pub name: String,
            /// NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
            pub nfs: Nfs,
            /// PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
            pub persistent_volume_claim: PersistentVolumeClaim,
            /// PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
            pub photon_persistent_disk: PhotonPersistentDisk,
            /// PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
            pub portworx_volume: PortworxVolume,
            /// Items for all in one resources secrets, configmaps, and downward API
            pub projected: Projected,
            /// Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
            pub quobyte: Quobyte,
            /// RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
            pub rbd: Rbd,
            /// ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
            pub scale_i_o: ScaleIO,
            /// Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
            pub secret: Secret,
            /// StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
            pub storageos: Storageos,
            /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
            pub vsphere_volume: VsphereVolume,
        }

        /// VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
        pub struct VsphereVolume {
            /// Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
            pub fs_type: String,
            /// Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
            pub storage_policy_i_d: String,
            /// Storage Policy Based Management (SPBM) profile name.
            pub storage_policy_name: String,
            /// Path that identifies vSphere volume vmdk
            pub volume_path: String,
        }

        /// The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence. Note that this field cannot be set when spec.os.name is linux.
        pub struct WindowsOptions {
            /// GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
            pub gmsa_credential_spec: String,
            /// GMSACredentialSpecName is the name of the GMSA credential spec to use.
            pub gmsa_credential_spec_name: String,
            /// HostProcess determines if a container should be run as a 'Host Process' container. This field is alpha-level and will only be honored by components that enable the WindowsHostProcessContainers feature flag. Setting this field without the feature flag will result in errors when validating the Pod. All of a Pod's containers must have the same effective HostProcess value (it is not allowed to have a mix of HostProcess containers and non-HostProcess containers).  In addition, if HostProcess is true then HostNetwork must also be set to true.
            pub host_process: bool,
            /// The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
            pub run_as_user_name: String,
        }

        /// Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
        pub struct Wwids {
        }

        impl k8s_openapi::Resource for ThanosRuler {
            type Scope = k8s_openapi::ClusterResourceScope;

            const API_VERSION : &'static str = "monitoring.coreos.com/v1";
            const GROUP : &'static str = "monitoring.coreos.com";
            const KIND : &'static str = "ThanosRuler";
            const VERSION : &'static str = "v1";
            const URL_PATH_SEGMENT : &'static str = "TODO";
        }

        impl k8s_openapi::Metadata for ThanosRuler {
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
    pub mod v1alpha1 {
        pub mod alertmanager_config {
        /// AlertmanagerConfig defines a namespaced AlertmanagerConfig to be aggregated across multiple namespaces configuring one Alertmanager cluster.
        pub struct AlertmanagerConfig {
            pub metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta,
            pub spec: Spec,
        }

        /// AccessKey is the AWS API key. If blank, the environment variable `AWS_ACCESS_KEY_ID` is used.
        pub struct AccessKey {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// A list of Slack actions that are sent with each notification.
        pub struct Actions {
        }

        /// SlackAction configures a single Slack action that is sent with each notification. See https://api.slack.com/docs/message-attachments#action_fields and https://api.slack.com/docs/message-buttons for more information.
        pub struct ActionsItem {
            /// SlackConfirmationField protect users from destructive actions or particularly distinguished decisions by asking them to confirm their button click one more time. See https://api.slack.com/docs/interactive-message-field-guide#confirmation_fields for more information.
            pub confirm: Confirm,
            pub name: String,
            pub style: String,
            pub text: String,
            pub r#type: String,
            pub url: String,
            pub value: String,
        }

        /// The secret's key that contains the API key to use when talking to the VictorOps API. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct ApiKey {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// The secret's key that contains the WeChat API key. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct ApiSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// The secret's key that contains the Slack webhook URL. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct ApiURL {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// SNS message attributes.
        pub struct Attributes {
        }

        /// The secret's key that contains the password to use for authentication. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct AuthPassword {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// The secret's key that contains the CRAM-MD5 secret. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct AuthSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Authorization header configuration for the client. This is mutually exclusive with BasicAuth and is only available starting from Alertmanager v0.22+.
        pub struct Authorization {
            /// The secret's key that contains the credentials of the request
            pub credentials: Credentials,
            /// Set the authentication type. Defaults to Bearer, Basic will cause an error
            pub r#type: String,
        }

        /// BasicAuth for the client. This is mutually exclusive with Authorization. If both are defined, BasicAuth takes precedence.
        pub struct BasicAuth {
            /// The secret in the service monitor namespace that contains the password for authentication.
            pub password: Password,
            /// The secret in the service monitor namespace that contains the username for authentication.
            pub username: Username,
        }

        /// The secret's key that contains the bearer token to be used by the client for authentication. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct BearerTokenSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Telegram bot token The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct BotToken {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Struct containing the CA cert to use for the targets.
        pub struct Ca {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// Struct containing the client cert file for the targets.
        pub struct Cert {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret or configmap containing the OAuth2 client id
        pub struct ClientId {
            /// ConfigMap containing data to use for the targets.
            pub config_map: ConfigMap,
            /// Secret containing data to use for the targets.
            pub secret: Secret,
        }

        /// The secret containing the OAuth2 client secret
        pub struct ClientSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// ConfigMap containing data to use for the targets.
        pub struct ConfigMap {
            /// The key to select.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the ConfigMap or its key must be defined
            pub optional: bool,
        }

        /// SlackConfirmationField protect users from destructive actions or particularly distinguished decisions by asking them to confirm their button click one more time. See https://api.slack.com/docs/interactive-message-field-guide#confirmation_fields for more information.
        pub struct Confirm {
            pub dismiss_text: String,
            pub ok_text: String,
            pub text: String,
            pub title: String,
        }

        /// The secret's key that contains the credentials of the request
        pub struct Credentials {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Additional custom fields for notification.
        pub struct CustomFields {
        }

        /// KeyValue defines a (key, value) tuple.
        pub struct CustomFieldsItem {
            /// Key of the tuple.
            pub key: String,
            /// Value of the tuple.
            pub value: String,
        }

        /// DaysOfMonth is a list of DayOfMonthRange
        pub struct DaysOfMonth {
        }

        /// DayOfMonthRange is an inclusive range of days of the month beginning at 1
        pub struct DaysOfMonthItem {
            /// End of the inclusive range
            pub end: i64,
            /// Start of the inclusive range
            pub start: i64,
        }

        /// Arbitrary key/value pairs that provide further detail about the incident.
        pub struct Details {
        }

        /// KeyValue defines a (key, value) tuple.
        pub struct DetailsItem {
            /// Key of the tuple.
            pub key: String,
            /// Value of the tuple.
            pub value: String,
        }

        /// List of Email configurations.
        pub struct EmailConfigs {
        }

        /// EmailConfig configures notifications via Email.
        pub struct EmailConfigsItem {
            /// The identity to use for authentication.
            pub auth_identity: String,
            /// The secret's key that contains the password to use for authentication. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub auth_password: AuthPassword,
            /// The secret's key that contains the CRAM-MD5 secret. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub auth_secret: AuthSecret,
            /// The username to use for authentication.
            pub auth_username: String,
            /// The sender address.
            pub from: String,
            /// Further headers email header key/value pairs. Overrides any headers previously set by the notification implementation.
            pub headers: Vec<HeadersItem>,
            /// The hostname to identify to the SMTP server.
            pub hello: String,
            /// The HTML body of the email notification.
            pub html: String,
            /// The SMTP TLS requirement. Note that Go does not support unencrypted connections to remote SMTP endpoints.
            pub require_t_l_s: bool,
            /// Whether or not to notify about resolved alerts.
            pub send_resolved: bool,
            /// The SMTP host and port through which emails are sent. E.g. example.com:25
            pub smarthost: String,
            /// The text body of the email notification.
            pub text: String,
            /// TLS configuration
            pub tls_config: TlsConfig,
            /// The email address to send notifications to.
            pub to: String,
        }

        /// Parameters to append to the token URL
        pub struct EndpointParams {
        }

        /// Labels that must have an equal value in the source and target alert for the inhibition to take effect.
        pub struct Equal {
        }

        /// A list of Slack fields that are sent with each notification.
        pub struct Fields {
        }

        /// SlackField configures a single Slack field that is sent with each notification. Each field must contain a title, value, and optionally, a boolean value to indicate if the field is short enough to be displayed next to other fields designated as short. See https://api.slack.com/docs/message-attachments#fields for more information.
        pub struct FieldsItem {
            pub short: bool,
            pub title: String,
            pub value: String,
        }

        /// List of labels to group by. Labels must not be repeated (unique list). Special label "..." (aggregate by all possible labels), if provided, must be the only element in the list.
        pub struct GroupBy {
        }

        /// Further headers email header key/value pairs. Overrides any headers previously set by the notification implementation.
        pub struct Headers {
        }

        /// KeyValue defines a (key, value) tuple.
        pub struct HeadersItem {
            /// Key of the tuple.
            pub key: String,
            /// Value of the tuple.
            pub value: String,
        }

        /// HTTP client configuration.
        pub struct HttpConfig {
            /// Authorization header configuration for the client. This is mutually exclusive with BasicAuth and is only available starting from Alertmanager v0.22+.
            pub authorization: Authorization,
            /// BasicAuth for the client. This is mutually exclusive with Authorization. If both are defined, BasicAuth takes precedence.
            pub basic_auth: BasicAuth,
            /// The secret's key that contains the bearer token to be used by the client for authentication. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub bearer_token_secret: BearerTokenSecret,
            /// FollowRedirects specifies whether the client should follow HTTP 3xx redirects.
            pub follow_redirects: bool,
            /// OAuth2 client credentials used to fetch a token for the targets.
            pub oauth2: Oauth2,
            /// Optional proxy URL.
            pub proxy_u_r_l: String,
            /// TLS configuration for the client.
            pub tls_config: TlsConfig,
        }

        /// List of inhibition rules. The rules will only apply to alerts matching the resource’s namespace.
        pub struct InhibitRules {
        }

        /// InhibitRule defines an inhibition rule that allows to mute alerts when other alerts are already firing. See https://prometheus.io/docs/alerting/latest/configuration/#inhibit_rule
        pub struct InhibitRulesItem {
            /// Labels that must have an equal value in the source and target alert for the inhibition to take effect.
            pub equal: Vec<String>,
            /// Matchers for which one or more alerts have to exist for the inhibition to take effect. The operator enforces that the alert matches the resource’s namespace.
            pub source_match: Vec<SourceMatchItem>,
            /// Matchers that have to be fulfilled in the alerts to be muted. The operator enforces that the alert matches the resource’s namespace.
            pub target_match: Vec<TargetMatchItem>,
        }

        /// Secret containing the client key file for the targets.
        pub struct KeySecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// List of matchers that the alert’s labels should match. For the first level route, the operator removes any existing equality and regexp matcher on the `namespace` label and adds a `namespace: <object namespace>` matcher.
        pub struct Matchers {
        }

        /// Matcher defines how to match on alert's labels.
        pub struct MatchersItem {
            /// Match operation available with AlertManager >= v0.22.0 and takes precedence over Regex (deprecated) if non-empty.
            pub match_type: String,
            /// Label to match.
            pub name: String,
            /// Whether to match on equality (false) or regular-expression (true). Deprecated as of AlertManager >= v0.22.0 where a user should use MatchType instead.
            pub regex: bool,
            /// Label value to match.
            pub value: String,
        }

        pub struct Metadata {
        }

        /// Months is a list of MonthRange
        pub struct Months {
        }

        pub struct MrkdwnIn {
        }

        /// Note: this comment applies to the field definition above but appears below otherwise it gets included in the generated manifest. CRD schema doesn't support self-referential types for now (see https://github.com/kubernetes/kubernetes/issues/62872). We have to use an alternative type to circumvent the limitation. The downside is that the Kube API can't validate the data beyond the fact that it is a valid JSON representation. MuteTimeIntervals is a list of MuteTimeInterval names that will mute this route when matched,
        pub struct MuteTimeIntervals {
        }

        /// MuteTimeInterval specifies the periods in time when notifications will be muted
        pub struct MuteTimeIntervalsItem {
            /// Name of the time interval
            pub name: String,
            /// TimeIntervals is a list of TimeInterval
            pub time_intervals: Vec<TimeIntervalsItem>,
        }

        /// OAuth2 client credentials used to fetch a token for the targets.
        pub struct Oauth2 {
            /// The secret or configmap containing the OAuth2 client id
            pub client_id: ClientId,
            /// The secret containing the OAuth2 client secret
            pub client_secret: ClientSecret,
            /// Parameters to append to the token URL
            pub endpoint_params: EndpointParams,
            /// OAuth2 scopes used for the token request
            pub scopes: Vec<String>,
            /// The URL to fetch the token from
            pub token_url: String,
        }

        /// List of OpsGenie configurations.
        pub struct OpsgenieConfigs {
        }

        /// OpsGenieConfig configures notifications via OpsGenie. See https://prometheus.io/docs/alerting/latest/configuration/#opsgenie_config
        pub struct OpsgenieConfigsItem {
            /// Comma separated list of actions that will be available for the alert.
            pub actions: String,
            /// The secret's key that contains the OpsGenie API key. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub api_key: ApiKey,
            /// The URL to send OpsGenie API requests to.
            pub api_u_r_l: String,
            /// Description of the incident.
            pub description: String,
            /// A set of arbitrary key/value pairs that provide further detail about the incident.
            pub details: Vec<DetailsItem>,
            /// Optional field that can be used to specify which domain alert is related to.
            pub entity: String,
            /// HTTP client configuration.
            pub http_config: HttpConfig,
            /// Alert text limited to 130 characters.
            pub message: String,
            /// Additional alert note.
            pub note: String,
            /// Priority level of alert. Possible values are P1, P2, P3, P4, and P5.
            pub priority: String,
            /// List of responders responsible for notifications.
            pub responders: Vec<RespondersItem>,
            /// Whether or not to notify about resolved alerts.
            pub send_resolved: bool,
            /// Backlink to the sender of the notification.
            pub source: String,
            /// Comma separated list of tags attached to the notifications.
            pub tags: String,
            /// Whether to update message and description of the alert in OpsGenie if it already exists By default, the alert is never updated in OpsGenie, the new message only appears in activity log.
            pub update_alerts: bool,
        }

        /// A list of image details to attach that provide further detail about an incident.
        pub struct PagerDutyImageConfigs {
        }

        /// PagerDutyImageConfig attaches images to an incident
        pub struct PagerDutyImageConfigsItem {
            /// Alt is the optional alternative text for the image.
            pub alt: String,
            /// Optional URL; makes the image a clickable link.
            pub href: String,
            /// Src of the image being attached to the incident
            pub src: String,
        }

        /// A list of link details to attach that provide further detail about an incident.
        pub struct PagerDutyLinkConfigs {
        }

        /// PagerDutyLinkConfig attaches text links to an incident
        pub struct PagerDutyLinkConfigsItem {
            /// Text that describes the purpose of the link, and can be used as the link's text.
            pub alt: String,
            /// Href is the URL of the link to be attached
            pub href: String,
        }

        /// List of PagerDuty configurations.
        pub struct PagerdutyConfigs {
        }

        /// PagerDutyConfig configures notifications via PagerDuty. See https://prometheus.io/docs/alerting/latest/configuration/#pagerduty_config
        pub struct PagerdutyConfigsItem {
            /// The class/type of the event.
            pub class: String,
            /// Client identification.
            pub client: String,
            /// Backlink to the sender of notification.
            pub client_u_r_l: String,
            /// The part or component of the affected system that is broken.
            pub component: String,
            /// Description of the incident.
            pub description: String,
            /// Arbitrary key/value pairs that provide further detail about the incident.
            pub details: Vec<DetailsItem>,
            /// A cluster or grouping of sources.
            pub group: String,
            /// HTTP client configuration.
            pub http_config: HttpConfig,
            /// A list of image details to attach that provide further detail about an incident.
            pub pager_duty_image_configs: Vec<PagerDutyImageConfigsItem>,
            /// A list of link details to attach that provide further detail about an incident.
            pub pager_duty_link_configs: Vec<PagerDutyLinkConfigsItem>,
            /// The secret's key that contains the PagerDuty integration key (when using Events API v2). Either this field or `serviceKey` needs to be defined. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub routing_key: RoutingKey,
            /// Whether or not to notify about resolved alerts.
            pub send_resolved: bool,
            /// The secret's key that contains the PagerDuty service key (when using integration type "Prometheus"). Either this field or `routingKey` needs to be defined. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub service_key: ServiceKey,
            /// Severity of the incident.
            pub severity: String,
            /// The URL to send requests to.
            pub url: String,
        }

        /// The secret in the service monitor namespace that contains the password for authentication.
        pub struct Password {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// List of Pushover configurations.
        pub struct PushoverConfigs {
        }

        /// PushoverConfig configures notifications via Pushover. See https://prometheus.io/docs/alerting/latest/configuration/#pushover_config
        pub struct PushoverConfigsItem {
            /// How long your notification will continue to be retried for, unless the user acknowledges the notification.
            pub expire: String,
            /// Whether notification message is HTML or plain text.
            pub html: bool,
            /// HTTP client configuration.
            pub http_config: HttpConfig,
            /// Notification message.
            pub message: String,
            /// Priority, see https://pushover.net/api#priority
            pub priority: String,
            /// How often the Pushover servers will send the same notification to the user. Must be at least 30 seconds.
            pub retry: String,
            /// Whether or not to notify about resolved alerts.
            pub send_resolved: bool,
            /// The name of one of the sounds supported by device clients to override the user's default sound choice
            pub sound: String,
            /// Notification title.
            pub title: String,
            /// The secret's key that contains the registered application’s API token, see https://pushover.net/apps. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub token: Token,
            /// A supplementary URL shown alongside the message.
            pub url: String,
            /// A title for supplementary URL, otherwise just the URL is shown
            pub url_title: String,
            /// The secret's key that contains the recipient user’s user key. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub user_key: UserKey,
        }

        /// List of receivers.
        pub struct Receivers {
        }

        /// Receiver defines one or more notification integrations.
        pub struct ReceiversItem {
            /// List of Email configurations.
            pub email_configs: Vec<EmailConfigsItem>,
            /// Name of the receiver. Must be unique across all items from the list.
            pub name: String,
            /// List of OpsGenie configurations.
            pub opsgenie_configs: Vec<OpsgenieConfigsItem>,
            /// List of PagerDuty configurations.
            pub pagerduty_configs: Vec<PagerdutyConfigsItem>,
            /// List of Pushover configurations.
            pub pushover_configs: Vec<PushoverConfigsItem>,
            /// List of Slack configurations.
            pub slack_configs: Vec<SlackConfigsItem>,
            /// List of SNS configurations
            pub sns_configs: Vec<SnsConfigsItem>,
            /// List of Telegram configurations.
            pub telegram_configs: Vec<TelegramConfigsItem>,
            /// List of VictorOps configurations.
            pub victorops_configs: Vec<VictoropsConfigsItem>,
            /// List of webhook configurations.
            pub webhook_configs: Vec<WebhookConfigsItem>,
            /// List of WeChat configurations.
            pub wechat_configs: Vec<WechatConfigsItem>,
        }

        /// List of responders responsible for notifications.
        pub struct Responders {
        }

        /// OpsGenieConfigResponder defines a responder to an incident. One of `id`, `name` or `username` has to be defined.
        pub struct RespondersItem {
            /// ID of the responder.
            pub id: String,
            /// Name of the responder.
            pub name: String,
            /// Type of responder.
            pub r#type: String,
            /// Username of the responder.
            pub username: String,
        }

        /// The Alertmanager route definition for alerts matching the resource’s namespace. If present, it will be added to the generated Alertmanager configuration as a first-level route.
        pub struct Route {
            /// Boolean indicating whether an alert should continue matching subsequent sibling nodes. It will always be overridden to true for the first-level route by the Prometheus operator.
            pub r#continue: bool,
            /// List of labels to group by. Labels must not be repeated (unique list). Special label "..." (aggregate by all possible labels), if provided, must be the only element in the list.
            pub group_by: Vec<String>,
            /// How long to wait before sending an updated notification. Must match the regular expression`^(([0-9]+)y)?(([0-9]+)w)?(([0-9]+)d)?(([0-9]+)h)?(([0-9]+)m)?(([0-9]+)s)?(([0-9]+)ms)?$` Example: "5m"
            pub group_interval: String,
            /// How long to wait before sending the initial notification. Must match the regular expression`^(([0-9]+)y)?(([0-9]+)w)?(([0-9]+)d)?(([0-9]+)h)?(([0-9]+)m)?(([0-9]+)s)?(([0-9]+)ms)?$` Example: "30s"
            pub group_wait: String,
            /// List of matchers that the alert’s labels should match. For the first level route, the operator removes any existing equality and regexp matcher on the `namespace` label and adds a `namespace: <object namespace>` matcher.
            pub matchers: Vec<MatchersItem>,
            /// Note: this comment applies to the field definition above but appears below otherwise it gets included in the generated manifest. CRD schema doesn't support self-referential types for now (see https://github.com/kubernetes/kubernetes/issues/62872). We have to use an alternative type to circumvent the limitation. The downside is that the Kube API can't validate the data beyond the fact that it is a valid JSON representation. MuteTimeIntervals is a list of MuteTimeInterval names that will mute this route when matched,
            pub mute_time_intervals: Vec<String>,
            /// Name of the receiver for this route. If not empty, it should be listed in the `receivers` field.
            pub receiver: String,
            /// How long to wait before repeating the last notification. Must match the regular expression`^(([0-9]+)y)?(([0-9]+)w)?(([0-9]+)d)?(([0-9]+)h)?(([0-9]+)m)?(([0-9]+)s)?(([0-9]+)ms)?$` Example: "4h"
            pub repeat_interval: String,
            /// Child routes.
        }

        /// Child routes.
        pub struct Routes {
        }

        /// The secret's key that contains the PagerDuty integration key (when using Events API v2). Either this field or `serviceKey` needs to be defined. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct RoutingKey {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// OAuth2 scopes used for the token request
        pub struct Scopes {
        }

        /// Secret containing data to use for the targets.
        pub struct Secret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// SecretKey is the AWS API secret. If blank, the environment variable `AWS_SECRET_ACCESS_KEY` is used.
        pub struct SecretKey {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// The secret's key that contains the PagerDuty service key (when using integration type "Prometheus"). Either this field or `routingKey` needs to be defined. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct ServiceKey {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// Configures AWS's Signature Verification 4 signing process to sign requests.
        pub struct Sigv4 {
            /// AccessKey is the AWS API key. If blank, the environment variable `AWS_ACCESS_KEY_ID` is used.
            pub access_key: AccessKey,
            /// Profile is the named AWS profile used to authenticate.
            pub profile: String,
            /// Region is the AWS region. If blank, the region from the default credentials chain used.
            pub region: String,
            /// RoleArn is the named AWS profile used to authenticate.
            pub role_arn: String,
            /// SecretKey is the AWS API secret. If blank, the environment variable `AWS_SECRET_ACCESS_KEY` is used.
            pub secret_key: SecretKey,
        }

        /// List of Slack configurations.
        pub struct SlackConfigs {
        }

        /// SlackConfig configures notifications via Slack. See https://prometheus.io/docs/alerting/latest/configuration/#slack_config
        pub struct SlackConfigsItem {
            /// A list of Slack actions that are sent with each notification.
            pub actions: Vec<ActionsItem>,
            /// The secret's key that contains the Slack webhook URL. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub api_u_r_l: ApiURL,
            pub callback_id: String,
            /// The channel or user to send notifications to.
            pub channel: String,
            pub color: String,
            pub fallback: String,
            /// A list of Slack fields that are sent with each notification.
            pub fields: Vec<FieldsItem>,
            pub footer: String,
            /// HTTP client configuration.
            pub http_config: HttpConfig,
            pub icon_emoji: String,
            pub icon_u_r_l: String,
            pub image_u_r_l: String,
            pub link_names: bool,
            pub mrkdwn_in: Vec<String>,
            pub pretext: String,
            /// Whether or not to notify about resolved alerts.
            pub send_resolved: bool,
            pub short_fields: bool,
            pub text: String,
            pub thumb_u_r_l: String,
            pub title: String,
            pub title_link: String,
            pub username: String,
        }

        /// List of SNS configurations
        pub struct SnsConfigs {
        }

        /// SNSConfig configures notifications via AWS SNS. See https://prometheus.io/docs/alerting/latest/configuration/#sns_configs
        pub struct SnsConfigsItem {
            /// The SNS API URL i.e. https://sns.us-east-2.amazonaws.com. If not specified, the SNS API URL from the SNS SDK will be used.
            pub api_u_r_l: String,
            /// SNS message attributes.
            pub attributes: Attributes,
            /// HTTP client configuration.
            pub http_config: HttpConfig,
            /// The message content of the SNS notification.
            pub message: String,
            /// Phone number if message is delivered via SMS in E.164 format. If you don't specify this value, you must specify a value for the TopicARN or TargetARN.
            pub phone_number: String,
            /// Whether or not to notify about resolved alerts.
            pub send_resolved: bool,
            /// Configures AWS's Signature Verification 4 signing process to sign requests.
            pub sigv4: Sigv4,
            /// Subject line when the message is delivered to email endpoints.
            pub subject: String,
            /// The  mobile platform endpoint ARN if message is delivered via mobile notifications. If you don't specify this value, you must specify a value for the topic_arn or PhoneNumber.
            pub target_a_r_n: String,
            /// SNS topic ARN, i.e. arn:aws:sns:us-east-2:698519295917:My-Topic If you don't specify this value, you must specify a value for the PhoneNumber or TargetARN.
            pub topic_a_r_n: String,
        }

        /// Matchers for which one or more alerts have to exist for the inhibition to take effect. The operator enforces that the alert matches the resource’s namespace.
        pub struct SourceMatch {
        }

        /// Matcher defines how to match on alert's labels.
        pub struct SourceMatchItem {
            /// Match operation available with AlertManager >= v0.22.0 and takes precedence over Regex (deprecated) if non-empty.
            pub match_type: String,
            /// Label to match.
            pub name: String,
            /// Whether to match on equality (false) or regular-expression (true). Deprecated as of AlertManager >= v0.22.0 where a user should use MatchType instead.
            pub regex: bool,
            /// Label value to match.
            pub value: String,
        }

        /// AlertmanagerConfigSpec is a specification of the desired behavior of the Alertmanager configuration. By definition, the Alertmanager configuration only applies to alerts for which the `namespace` label is equal to the namespace of the AlertmanagerConfig resource.
        pub struct Spec {
            /// List of inhibition rules. The rules will only apply to alerts matching the resource’s namespace.
            pub inhibit_rules: Vec<InhibitRulesItem>,
            /// List of MuteTimeInterval specifying when the routes should be muted.
            pub mute_time_intervals: Vec<MuteTimeIntervalsItem>,
            /// List of receivers.
            pub receivers: Vec<ReceiversItem>,
            /// The Alertmanager route definition for alerts matching the resource’s namespace. If present, it will be added to the generated Alertmanager configuration as a first-level route.
            pub route: Route,
        }

        /// Matchers that have to be fulfilled in the alerts to be muted. The operator enforces that the alert matches the resource’s namespace.
        pub struct TargetMatch {
        }

        /// Matcher defines how to match on alert's labels.
        pub struct TargetMatchItem {
            /// Match operation available with AlertManager >= v0.22.0 and takes precedence over Regex (deprecated) if non-empty.
            pub match_type: String,
            /// Label to match.
            pub name: String,
            /// Whether to match on equality (false) or regular-expression (true). Deprecated as of AlertManager >= v0.22.0 where a user should use MatchType instead.
            pub regex: bool,
            /// Label value to match.
            pub value: String,
        }

        /// List of Telegram configurations.
        pub struct TelegramConfigs {
        }

        /// TelegramConfig configures notifications via Telegram. See https://prometheus.io/docs/alerting/latest/configuration/#telegram_config
        pub struct TelegramConfigsItem {
            /// The Telegram API URL i.e. https://api.telegram.org. If not specified, default API URL will be used.
            pub api_u_r_l: String,
            /// Telegram bot token The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub bot_token: BotToken,
            /// The Telegram chat ID.
            pub chat_i_d: i64,
            /// Disable telegram notifications
            pub disable_notifications: bool,
            /// HTTP client configuration.
            pub http_config: HttpConfig,
            /// Message template
            pub message: String,
            /// Parse mode for telegram message
            pub parse_mode: String,
            /// Whether to notify about resolved alerts.
            pub send_resolved: bool,
        }

        /// TimeIntervals is a list of TimeInterval
        pub struct TimeIntervals {
        }

        /// TimeInterval describes intervals of time
        pub struct TimeIntervalsItem {
            /// DaysOfMonth is a list of DayOfMonthRange
            pub days_of_month: Vec<DaysOfMonthItem>,
            /// Months is a list of MonthRange
            pub months: Vec<String>,
            /// Times is a list of TimeRange
            pub times: Vec<TimesItem>,
            /// Weekdays is a list of WeekdayRange
            pub weekdays: Vec<String>,
            /// Years is a list of YearRange
            pub years: Vec<String>,
        }

        /// Times is a list of TimeRange
        pub struct Times {
        }

        /// TimeRange defines a start and end time in 24hr format
        pub struct TimesItem {
            /// EndTime is the end time in 24hr format.
            pub end_time: String,
            /// StartTime is the start time in 24hr format.
            pub start_time: String,
        }

        /// TLS configuration for the client.
        pub struct TlsConfig {
            /// Struct containing the CA cert to use for the targets.
            pub ca: Ca,
            /// Struct containing the client cert file for the targets.
            pub cert: Cert,
            /// Disable target certificate validation.
            pub insecure_skip_verify: bool,
            /// Secret containing the client key file for the targets.
            pub key_secret: KeySecret,
            /// Used to verify the hostname for the targets.
            pub server_name: String,
        }

        /// The secret's key that contains the registered application’s API token, see https://pushover.net/apps. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct Token {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// The secret's key that contains the webhook URL to send HTTP requests to. `urlSecret` takes precedence over `url`. One of `urlSecret` and `url` should be defined. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct UrlSecret {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// The secret's key that contains the recipient user’s user key. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
        pub struct UserKey {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// The secret in the service monitor namespace that contains the username for authentication.
        pub struct Username {
            /// The key of the secret to select from.  Must be a valid secret key.
            pub key: String,
            /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
            pub name: String,
            /// Specify whether the Secret or its key must be defined
            pub optional: bool,
        }

        /// List of VictorOps configurations.
        pub struct VictoropsConfigs {
        }

        /// VictorOpsConfig configures notifications via VictorOps. See https://prometheus.io/docs/alerting/latest/configuration/#victorops_config
        pub struct VictoropsConfigsItem {
            /// The secret's key that contains the API key to use when talking to the VictorOps API. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub api_key: ApiKey,
            /// The VictorOps API URL.
            pub api_url: String,
            /// Additional custom fields for notification.
            pub custom_fields: Vec<CustomFieldsItem>,
            /// Contains summary of the alerted problem.
            pub entity_display_name: String,
            /// The HTTP client's configuration.
            pub http_config: HttpConfig,
            /// Describes the behavior of the alert (CRITICAL, WARNING, INFO).
            pub message_type: String,
            /// The monitoring tool the state message is from.
            pub monitoring_tool: String,
            /// A key used to map the alert to a team.
            pub routing_key: String,
            /// Whether or not to notify about resolved alerts.
            pub send_resolved: bool,
            /// Contains long explanation of the alerted problem.
            pub state_message: String,
        }

        /// List of webhook configurations.
        pub struct WebhookConfigs {
        }

        /// WebhookConfig configures notifications via a generic receiver supporting the webhook payload. See https://prometheus.io/docs/alerting/latest/configuration/#webhook_config
        pub struct WebhookConfigsItem {
            /// HTTP client configuration.
            pub http_config: HttpConfig,
            /// Maximum number of alerts to be sent per webhook message. When 0, all alerts are included.
            pub max_alerts: i32,
            /// Whether or not to notify about resolved alerts.
            pub send_resolved: bool,
            /// The URL to send HTTP POST requests to. `urlSecret` takes precedence over `url`. One of `urlSecret` and `url` should be defined.
            pub url: String,
            /// The secret's key that contains the webhook URL to send HTTP requests to. `urlSecret` takes precedence over `url`. One of `urlSecret` and `url` should be defined. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub url_secret: UrlSecret,
        }

        /// List of WeChat configurations.
        pub struct WechatConfigs {
        }

        /// WeChatConfig configures notifications via WeChat. See https://prometheus.io/docs/alerting/latest/configuration/#wechat_config
        pub struct WechatConfigsItem {
            pub agent_i_d: String,
            /// The secret's key that contains the WeChat API key. The secret needs to be in the same namespace as the AlertmanagerConfig object and accessible by the Prometheus Operator.
            pub api_secret: ApiSecret,
            /// The WeChat API URL.
            pub api_u_r_l: String,
            /// The corp id for authentication.
            pub corp_i_d: String,
            /// HTTP client configuration.
            pub http_config: HttpConfig,
            /// API request data as defined by the WeChat API.
            pub message: String,
            pub message_type: String,
            /// Whether or not to notify about resolved alerts.
            pub send_resolved: bool,
            pub to_party: String,
            pub to_tag: String,
            pub to_user: String,
        }

        /// Weekdays is a list of WeekdayRange
        pub struct Weekdays {
        }

        /// Years is a list of YearRange
        pub struct Years {
        }

        impl k8s_openapi::Resource for AlertmanagerConfig {
            type Scope = k8s_openapi::ClusterResourceScope;

            const API_VERSION : &'static str = "monitoring.coreos.com/v1alpha1";
            const GROUP : &'static str = "monitoring.coreos.com";
            const KIND : &'static str = "AlertmanagerConfig";
            const VERSION : &'static str = "v1alpha1";
            const URL_PATH_SEGMENT : &'static str = "TODO";
        }

        impl k8s_openapi::Metadata for AlertmanagerConfig {
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
