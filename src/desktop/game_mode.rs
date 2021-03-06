//! ```no_run
//! use ashpd::desktop::game_mode::{GameModeProxy, GameModeStatus};
//! use zbus::{self, fdo::Result};
//!
//! fn main() -> Result<()> {
//!     let connection = zbus::Connection::new_session()?;
//!     let proxy = GameModeProxy::new(&connection)?;
//!
//!     println!("{:#?}", proxy.register_game(246612)?);
//!
//!     println!("{:#?}", proxy.query_status(246612)?);
//!
//!     println!("{:#?}", proxy.unregister_game(246612)?);
//!
//!     println!("{:#?}", proxy.query_status(246612)?);
//!
//!     Ok(())
//! }
//! ```
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::{dbus_proxy, fdo::Result};
use zvariant::Fd;
use zvariant_derive::Type;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Type)]
#[repr(i32)]
/// The status of the game mode.
pub enum GameModeStatus {
    /// GameMode is inactive.
    Inactive = 0,
    /// GameMode is active.
    Active = 1,
    /// GameMode is active and `pid` is registered.
    Registered = 2,
    /// The query failed inside GameMode.
    Rejected = -1,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Type)]
#[repr(i32)]
/// The status of a register game mode request.
pub enum RegisterStatus {
    /// If the game was successfully registered.
    Success = 0,
    /// If the request was rejected by GameMode.
    Rejected = -1,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Type)]
#[repr(i32)]
/// The status of an un-register game mode request.
pub enum UnregisterStatus {
    /// If the game was successfully registered.
    Success = 0,
    /// If the request was rejected by GameMode.
    Rejected = -1,
}

#[dbus_proxy(
    interface = "org.freedesktop.portal.GameMode",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
/// The interface lets sandboxed applications access GameMode from within the sandbox.
///
/// It is analogous to the com.feralinteractive.GameMode interface and will proxy request there,
/// but with additional permission checking and pid mapping.
/// The latter is necessary in the case that sandbox has pid namespace isolation enabled.
/// See the man page for pid_namespaces(7) for more details, but briefly,
/// it means that the sandbox has its own process id namespace which is separated
/// from the one on the host. Thus there will be two separate process ids (pids) within
/// two different namespaces that both identify same process.
/// One id from the pid namespace inside the sandbox and one id from the host pid namespace.
/// Since GameMode expects pids from the host pid namespace but
/// programs inside the sandbox can only know pids from the sandbox namespace,
/// process ids need to be translated from the portal to the host namespace.
/// The portal will do that transparently for all calls where this is necessary.
///
/// Note: GameMode will monitor active clients, i.e. games and other programs that
/// have successfully called 'RegisterGame'. In the event that a client terminates
/// without a call to the 'UnregisterGame' method, GameMode will automatically
/// un-register the client. This might happen with a (small) delay.
trait GameMode {
    /// Query the GameMode status for a process.
    /// If the caller is running inside a sandbox with pid namespace isolation,
    /// the pid will be translated to the respective host pid.
    ///
    /// # Arguments
    ///
    /// * `pid` - Process id to query the GameMode status of.
    fn query_status(&self, pid: i32) -> Result<GameModeStatus>;

    /// Query the GameMode status for a process.
    ///
    /// # Arguments
    ///
    /// * `target` - Pid file descriptor to query the GameMode status of.
    /// * `requester` - Pid file descriptor of the process requesting the information.
    #[dbus_proxy(name = "QueryStatusByPIDFd")]
    fn query_status_by_pidfd(&self, target: Fd, requester: Fd) -> Result<GameModeStatus>;

    /// Query the GameMode status for a process.
    ///
    /// # Arguments
    ///
    /// * `target` - Process id to query the GameMode status of.
    /// * `requester` - Process id of the process requesting the information.
    fn query_status_by_pid(&self, target: i32, requester: i32) -> Result<GameModeStatus>;

    /// Register a game with GameMode and thus request GameMode to be activated.
    /// If the caller is running inside a sandbox with pid namespace isolation,
    /// the pid will be translated to the respective host pid. See the general introduction for details.
    /// If the GameMode has already been requested for pid before, this call will fail, i.e. result will be `RegisterStatus::Rejected`
    ///
    /// # Arguments
    ///
    /// * `pid` - Process id of the game to register.
    fn register_game(&self, pid: i32) -> Result<RegisterStatus>;

    /// Register a game with GameMode.
    ///
    /// # Arguments
    ///
    /// * `target` - Process file descriptor of the game to register.
    /// * `requester` - Process file descriptor of the process requesting the registration.
    #[dbus_proxy(name = "RegisterGameByPIDFd")]
    fn register_game_by_pidfd(&self, target: Fd, requester: Fd) -> Result<RegisterStatus>;

    /// Register a game with GameMode.
    ///
    /// # Arguments
    ///
    /// * `target` - Process id of the game to register.
    /// * `requester` - Process id of the process requesting the registration.
    fn register_game_by_pid(&self, target: i32, requester: i32) -> Result<RegisterStatus>;

    /// Un-register a game from GameMode.
    /// if the call is successful and there are no other games or clients registered, GameMode will be deactivated.
    /// If the caller is running inside a sandbox with pid namespace isolation,
    /// the pid will be translated to the respective host pid.
    ///
    /// # Arguments
    ///
    /// `pid` - Process id of the game to un-register.
    fn unregister_game(&self, pid: i32) -> Result<UnregisterStatus>;

    /// Un-register a game from GameMode.
    ///
    /// # Arguments
    ///
    /// * `target` - Pid file descriptor of the game to un-register.
    /// * `requester` - Pid file descriptor of the process requesting the un-registration.
    #[dbus_proxy(name = "UnregisterGameByPIDFd")]
    fn unregister_game_by_pidfd(&self, target: Fd, requester: Fd) -> Result<UnregisterStatus>;

    /// Un-register a game from GameMode.
    ///
    /// # Arguments
    ///
    /// * `target` - Process id of the game to un-register.
    /// * `requester` - Process id of the process requesting the un-registration.
    fn unregister_game_by_pid(&self, target: i32, requester: i32) -> Result<UnregisterStatus>;

    /// version property
    #[dbus_proxy(property, name = "version")]
    fn version(&self) -> Result<u32>;
}
