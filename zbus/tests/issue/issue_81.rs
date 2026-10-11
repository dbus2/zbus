use test_log::test;

use zbus::OwnedObjectPath;

#[test]
#[ignore]
fn issue_81() {
    use zbus::{OwnedVariant, Type, proxy};

    #[derive(
        Debug, PartialEq, Eq, Clone, Type, OwnedVariant, serde::Serialize, serde::Deserialize,
    )]
    pub struct DbusPath {
        id: String,
        path: OwnedObjectPath,
    }

    #[proxy(assume_defaults = true)]
    trait Session {
        #[zbus(property)]
        fn sessions_tuple(&self) -> zbus::Result<(String, String)>;

        #[zbus(property)]
        fn sessions_struct(&self) -> zbus::Result<DbusPath>;
    }
}
