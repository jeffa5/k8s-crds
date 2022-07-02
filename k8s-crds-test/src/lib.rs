#[path = "generated/generated.rs"]
pub mod generated;

#[cfg(test)]
mod tests {
    use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

    use super::*;

    #[test]
    fn serde_basic() {
        use generated::basic_object::stable_example_com::v1::cron_tab::*;

        let ct = CronTab {
            metadata: ObjectMeta {
                name: Some("test".into()),
                ..Default::default()
            },
            spec: Spec {
                cron_spec: "* * * * *".into(),
                image: "nginx".into(),
                replicas: 2,
            },
        };

        let s = serde_yaml::to_string(&ct).unwrap();
        insta::assert_snapshot!(s);

        let ct_d = serde_yaml::from_str(&s).unwrap();

        assert_eq!(ct, ct_d);
    }

    #[test]
    fn serde_double() {
        use generated::double_object::stable_example_com::v1::cron_tab::*;

        let ct = CronTab {
            metadata: ObjectMeta {
                name: Some("test".into()),
                ..Default::default()
            },
            obj1: Obj1 {
                a: Obj1A { b: "BB".into() },
            },
            obj2: Obj2 { a: Obj2A { c: 4 } },
        };

        let s = serde_yaml::to_string(&ct).unwrap();

        insta::assert_snapshot!(s);

        let ct_d = serde_yaml::from_str(&s).unwrap();

        assert_eq!(ct, ct_d);
    }

    #[test]
    fn serde_long() {
        use generated::long_names::stable_example_com::v1::cron_tab::*;

        let ct = CronTab {
            metadata: ObjectMeta {
                name: Some("test".into()),
                ..Default::default()
            },
            a: A {
                b: B {
                    c: C {
                        d: D { e: 2 },
                        f: F { g: "gg".into() },
                    },
                },
            },
        };

        let s = serde_yaml::to_string(&ct).unwrap();

        insta::assert_snapshot!(s);

        let ct_d = serde_yaml::from_str(&s).unwrap();

        assert_eq!(ct, ct_d);
    }
}
