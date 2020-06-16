use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::BufRead;
use std::result::Result;

use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus_derive::dbus_proxy;
use zvariant::{OwnedValue, Value};
use zvariant_derive::Type;

use crate::Error;

/// Flags used in the CheckAuthorization() method.
#[repr(u32)]
#[derive(Type, BitFlags, Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum CheckAuthorizationFlags {
    /// If the Subject can obtain the authorization through authentication, and an authentication
    /// agent is available, then attempt to do so. Note, this means that the CheckAuthorization()
    /// method will block while the user is being asked to authenticate.
    AllowUserInteraction = 0x01,
}

/// An enumeration for granting implicit authorizations.
#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Type, Debug, PartialEq)]
pub enum ImplicitAuthorization {
    /// The Subject is not authorized.
    NotAuthorized = 0,
    /// Authentication is required.
    AuthenticationRequired = 1,
    /// Authentication as an administrator is required.
    AdministratorAuthenticationRequired = 2,
    /// Authentication is required. If the authorization is obtained, it is retained.
    AuthenticationRequiredRetained = 3,
    /// Authentication as an administrator is required. If the authorization is obtained, it is retained.
    AdministratorAuthenticationRequiredRetained = 4,
    /// The subject is authorized.
    Authorized = 5,
}

/// Flags describing features supported by the Authority implementation.
#[repr(u32)]
#[derive(Type, BitFlags, Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum AuthorityFeatures {
    /// The authority supports temporary authorizations that can be obtained through authentication.
    TemporaryAuthorization = 0x01,
}

impl TryFrom<OwnedValue> for AuthorityFeatures {
    type Error = <u32 as TryFrom<OwnedValue>>::Error;

    fn try_from(v: OwnedValue) -> Result<Self, Self::Error> {
        // safe because AuthorityFeatures has repr u32
        Ok(unsafe { std::mem::transmute(<u32>::try_from(v)?) })
    }
}

#[derive(Debug, Type, Deserialize)]
pub struct TemporaryAuthorization {
    /// An opaque identifier for the temporary authorization.
    pub id: String,

    /// The action the temporary authorization is for.
    pub action_id: String,

    /// The subject the temporary authorization is for.
    pub subject: Subject,

    /// When the temporary authorization was obtained, in seconds since the Epoch Jan 1, 1970 0:00
    /// UTC. Note that the PolicyKit daemon is using monotonic time internally so the returned value
    /// may change if system time changes.
    pub time_obtained: u64,

    /// When the temporary authorization is set to expire, in seconds since the Epoch Jan 1, 1970
    /// 0:00 UTC. Note that the PolicyKit daemon is using monotonic time internally so the returned
    /// value may change if system time changes.
    pub time_expires: u64,
}

/// This struct describes identities such as UNIX users and UNIX groups. It is typically used to
/// check if a given process is authorized for an action.
///
/// The following kinds of identities are known:
///
/// * Unix User. `identity_kind` should be set to `unix-user` with key uid (of type uint32).
///
/// * Unix Group. `identity_kind` should be set to `unix-group` with key gid (of type uint32).
#[derive(Debug, Type, Serialize)]
pub struct Identity<'a> {
    pub identity_kind: &'a str,

    pub identity_details: &'a HashMap<&'a str, Value<'a>>,
}

fn pid_start_time(pid: u32) -> Result<u64, Error> {
    let fname = format!("/proc/{}/stat", pid);
    let content = std::fs::read_to_string(fname)?;

    if let Some(i) = content.rfind(')') {
        if let Some(start_time) = content[i..].split(' ').nth(20) {
            return Ok(start_time.parse()?);
        }
    }

    Err(std::io::Error::from(std::io::ErrorKind::NotFound).into())
}

// Return the "current" UID.  Note that this is inherently racy, and the value may already be
// obsolete by the time this function returns; this function only guarantees that the UID was valid
// at some point during its execution.
fn pid_uid_racy(pid: u32) -> Result<u32, Error> {
    let fname = format!("/proc/{}/status", pid);
    let file = std::fs::File::open(fname)?;
    let lines = std::io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(l) = line {
            if l.starts_with("Uid:") {
                if let Some(uid) = l.split('\t').nth(1) {
                    return Ok(uid.parse()?);
                }
            }
        }
    }

    Err(std::io::Error::from(std::io::ErrorKind::NotFound).into())
}

/// This struct describes subjects such as UNIX processes. It is typically used to check if a given
/// process is authorized for an action.
///
/// The following kinds of subjects are known:
///
/// * Unix Process. `subject_kind` should be set to `unix-process` with keys `pid` (of type
/// `uint32`) and `start-time` (of type `uint64`).
///
/// * Unix Session. `subject_kind` should be set to `unix-session` with the key `session-id` (of
/// type `string`).
///
/// * System Bus Name. `subject_kind` should be set to `system-bus-name` with the key `name` (of
/// type `string`).
#[derive(Debug, Type, Serialize, Deserialize)]
pub struct Subject {
    /// The type of the subject.
    pub subject_kind: String,

    /// Details about the subject. Depending of the value of `subject_kind`, a set of well-defined
    /// key/value pairs are guaranteed to be available.
    pub subject_details: HashMap<String, OwnedValue>,
}

impl Subject {
    /// Create a `Subject` for `pid`, `start_time` & `uid`.
    ///
    /// # Arguments
    ///
    /// * `pid` - The process ID
    ///
    /// * `start_time` - The start time for `pid` or `None` to look it up in e.g. `/proc`
    ///
    /// * `uid` - The (real, not effective) uid of the owner of `pid` or `None` to look it up in
    /// e.g. `/proc`
    pub fn new_for_owner(
        pid: u32,
        start_time: Option<u64>,
        uid: Option<u32>,
    ) -> Result<Self, Error> {
        let start_time = match start_time {
            Some(s) => s,
            None => pid_start_time(pid)?,
        };
        let uid = match uid {
            Some(u) => u,
            None => pid_uid_racy(pid)?,
        };
        let mut hashmap = HashMap::new();
        hashmap.insert("pid".to_string(), Value::from(pid).into());
        hashmap.insert("start-time".to_string(), Value::from(start_time).into());
        hashmap.insert("uid".to_string(), Value::from(uid).into());

        Ok(Self {
            subject_kind: "unix-process".into(),
            subject_details: hashmap,
        })
    }
}

/// This struct describes actions registered with the PolicyKit daemon.
#[derive(Debug, Type, Serialize, Deserialize)]
pub struct ActionDescription {
    /// Action Identifier.
    pub action_id: String,

    /// Localized description of the action.
    pub description: String,

    /// Localized message to be displayed when making the user authenticate for an action.
    pub message: String,

    /// Name of the provider of the action or the empty string.
    pub vendor_name: String,

    /// A URL pointing to a place with more information about the action or the empty string.
    pub vendor_url: String,

    /// The themed icon describing the action or the empty string if no icon is set.
    pub icon_name: String,

    /// A value from the ImplicitAuthorization. enumeration for implicit authorizations that apply
    /// to any Subject.
    pub implicit_any: ImplicitAuthorization,

    /// A value from the ImplicitAuthorization. enumeration for implicit authorizations that apply
    /// any Subject in an inactive user session on the local console.
    pub implicit_inactive: ImplicitAuthorization,

    /// A value from the ImplicitAuthorization. enumeration for implicit authorizations that apply
    /// any Subject in an active user session on the local console.
    pub implicit_active: ImplicitAuthorization,

    /// Annotations for the action.
    pub annotations: HashMap<String, String>,
}

/// Describes the result of calling `CheckAuthorization()`
#[derive(Debug, Type, Serialize, Deserialize)]
pub struct AuthorizationResult {
    /// TRUE if the given `Subject` is authorized for the given action.
    pub is_authorized: bool,

    /// TRUE if the given `Subject` could be authorized if more information was provided, and
    /// `CheckAuthorizationFlags::AllowUserInteraction` wasn't passed or no suitable authentication
    /// agent was available.
    pub is_challenge: bool,

    /// Details for the result. Known key/value-pairs include `polkit.temporary_authorization_id`
    /// (if the authorization is temporary, this is set to the opaque temporary authorization id),
    /// `polkit.retains_authorization_after_challenge` (Set to a non-empty string if the
    /// authorization will be retained after authentication (if is_challenge is TRUE)),
    /// `polkit.dismissed` (Set to a non-empty string if the authentication dialog was dismissed by
    /// the user).
    pub details: std::collections::HashMap<String, String>,
}

/// This D-Bus interface is implemented by the /org/freedesktop/PolicyKit1/Authority object on the
/// well-known name org.freedesktop.PolicyKit1 on the system message bus.
#[dbus_proxy(
    interface = "org.freedesktop.PolicyKit1.Authority",
    default_service = "org.freedesktop.PolicyKit1",
    default_path = "/org/freedesktop/PolicyKit1/Authority"
)]
trait Authority {
    /// Method for authentication agents to invoke on successful authentication, intended only for
    /// use by a privileged helper process internal to polkit. This method will fail unless a
    /// sufficiently privileged +caller invokes it. Deprecated in favor of
    /// `AuthenticationAgentResponse2()`.
    fn authentication_agent_response(&self, cookie: &str, identity: &Identity) -> zbus::Result<()>;

    /// Method for authentication agents to invoke on successful authentication, intended only for
    /// use by a privileged helper process internal to polkit. This method will fail unless a
    /// sufficiently privileged caller invokes it. Note this method was introduced in 0.114 and
    /// should be preferred over `AuthenticationAgentResponse()` as it fixes a security issue.
    fn authentication_agent_response2(
        &self,
        uid: u32,
        cookie: &str,
        identity: &Identity,
    ) -> zbus::Result<()>;

    /// Cancels an authorization check.
    ///
    /// # Arguments
    ///
    /// * `cancellation_id` - The cancellation_id passed to `CheckAuthorization()`.
    fn cancel_check_authorization(&self, cancellation_id: &str) -> zbus::Result<()>;

    /// Checks if subject is authorized to perform the action with identifier `action_id`
    ///
    /// If `cancellation_id` is non-empty and already in use for the caller, the
    /// `org.freedesktop.PolicyKit1.Error.CancellationIdNotUnique` error is returned.
    ///
    /// Note that `CheckAuthorizationFlags::AllowUserInteraction` SHOULD be passed ONLY if the event
    /// that triggered the authorization check is stemming from an user action, e.g. the user
    /// pressing a button or attaching a device.
    ///
    /// # Arguments
    ///
    /// * `subject` - A Subject struct.
    ///
    /// * `action_id` - Identifier for the action that subject is attempting to do.
    ///
    /// * `details` - Details describing the action. Keys starting with `polkit.` can only be set
    /// if defined in this document.
    ///
    /// Known keys include `polkit.message` and `polkit.gettext_domain` that can be used to override
    /// the message shown to the user. This latter is needed because the user could be running an
    /// authentication agent in another locale than the calling process.
    ///
    /// The (translated version of) `polkit.message` may include references to other keys that are
    /// expanded with their respective values. For example if the key `device_file` has the value
    /// `/dev/sda` then the message "Authenticate to format $(device_file)" is expanded to
    /// "Authenticate to format /dev/sda".
    ///
    /// The key `polkit.icon_name` is used to override the icon shown in the authentication dialog.
    ///
    /// If non-empty, then the request will fail with `org.freedesktop.PolicyKit1.Error.Failed`
    /// unless the process doing the check itsef is sufficiently authorized (e.g. running as uid 0).
    ///
    /// * `flags` - A set of `CheckAuthorizationFlags`.
    ///
    /// * `cancellation_id` - A unique id used to cancel the the authentication check via
    /// `CancelCheckAuthorization()` or the empty string if cancellation is not needed.
    ///
    /// Returns: An `AuthorizationResult` structure.
    fn check_authorization(
        &self,
        subject: &Subject,
        action_id: &str,
        details: std::collections::HashMap<&str, &str>,
        flags: BitFlags<CheckAuthorizationFlags>,
        cancellation_id: &str,
    ) -> zbus::Result<AuthorizationResult>;

    /// Enumerates all registered PolicyKit actions.
    ///
    /// # Arguments:
    ///
    /// * `locale` - The locale to get descriptions in or the blank string to use the system locale.
    fn enumerate_actions(&self, locale: &str) -> zbus::Result<Vec<ActionDescription>>;

    /// Retrieves all temporary authorizations that applies to subject.
    fn enumerate_temporary_authorizations(
        &self,
        subject: &Subject,
    ) -> zbus::Result<Vec<TemporaryAuthorization>>;

    /// Register an authentication agent.
    ///
    /// Note that this should be called by same effective UID which will be passed to
    /// `AuthenticationAgentResponse2()`.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject to register the authentication agent for, typically a session
    /// subject.
    ///
    /// * `locale` - The locale of the authentication agent.
    ///
    /// * `object_path` - The object path of authentication agent object on the unique name of the
    /// caller.
    fn register_authentication_agent(
        &self,
        subject: &Subject,
        locale: &str,
        object_path: &str,
    ) -> zbus::Result<()>;

    /// Like `RegisterAuthenticationAgent` but takes additional options. If the option fallback (of
    /// type Boolean) is TRUE, then the authentcation agent will only be used as a fallback, e.g. if
    /// another agent (without the fallback option set TRUE) is available, it will be used instead.
    fn register_authentication_agent_with_options(
        &self,
        subject: &Subject,
        locale: &str,
        object_path: &str,
        options: std::collections::HashMap<&str, zvariant::Value>,
    ) -> zbus::Result<()>;

    /// Revokes all temporary authorizations that applies to `id`.
    fn revoke_temporary_authorization_by_id(&self, id: &str) -> zbus::Result<()>;

    /// Revokes all temporary authorizations that applies to `subject`.
    fn revoke_temporary_authorizations(&self, subject: &Subject) -> zbus::Result<()>;

    /// Unregister an authentication agent.
    ///
    /// # Arguments
    ///
    /// * `subject` - The subject passed to `RegisterAuthenticationAgent()`.
    ///
    /// * `object_path` - The object_path passed to `RegisterAuthenticationAgent()`.
    fn unregister_authentication_agent(
        &self,
        subject: &Subject,
        object_path: &str,
    ) -> zbus::Result<()>;

    /// This signal is emitted when actions and/or authorizations change
    // fn Changed()

    /// The features supported by the currently used Authority backend.
    #[dbus_proxy(property)]
    fn backend_features(&self) -> zbus::Result<AuthorityFeatures>;

    /// The name of the currently used Authority backend.
    #[dbus_proxy(property)]
    fn backend_name(&self) -> zbus::Result<String>;

    /// The version of the currently used Authority backend.
    #[dbus_proxy(property)]
    fn backend_version(&self) -> zbus::Result<String>;
}
