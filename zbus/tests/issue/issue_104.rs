use ntest::timeout;
use test_log::test;
use tracing::trace;

use zbus::{OwnedObjectPath, OwnedVariant};

use zbus::{
    conn::{self, Connection},
    names::UniqueName,
};

#[test]
#[timeout(15000)]
fn issue104() {
    // Tests the fix for https://github.com/z-galaxy/zbus/issues/104
    //
    // The issue is caused by `proxy` macro adding `()` around the return value of methods
    // with multiple out arguments, ending up with double parenthesis around the signature of
    // the return type and zbus only removing the outer `()` only and then it not matching the
    // signature we receive on the reply message.
    zbus::block_on(issue104_async());
}

async fn issue104_async() {
    use zbus::{ObjectPath, Variant};

    struct Secret;
    #[zbus::interface(name = "org.freedesktop.Secret.Service")]
    impl Secret {
        fn open_session(
            &self,
            _algorithm: &str,
            input: Variant<'_>,
        ) -> zbus::fdo::Result<(OwnedVariant, OwnedObjectPath)> {
            Ok((
                OwnedVariant::try_from(input).unwrap(),
                ObjectPath::try_from("/org/freedesktop/secrets/Blah")
                    .unwrap()
                    .into(),
            ))
        }
    }

    let secret = Secret;
    let conn = conn::Builder::session()
        .serve_at("/org/freedesktop/secrets", secret)
        .build()
        .await
        .unwrap();
    let service_name = conn.unique_name().unwrap().clone();

    {
        let conn = Connection::session().await.unwrap();
        #[zbus::proxy(interface = "org.freedesktop.Secret.Service", assume_defaults = true)]
        trait Secret {
            fn open_session(
                &self,
                algorithm: &str,
                input: &zbus::Variant<'_>,
            ) -> zbus::Result<(OwnedVariant, OwnedObjectPath)>;
        }

        let proxy = SecretProxy::builder(&conn)
            .destination(UniqueName::from(service_name))
            .path("/org/freedesktop/secrets")
            .build()
            .await
            .unwrap();

        trace!("Calling open_session");
        proxy
            .open_session("plain", &Variant::from(""))
            .await
            .unwrap();
        trace!("Called open_session");
    };
}
