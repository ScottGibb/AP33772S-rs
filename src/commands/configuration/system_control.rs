use bitbybit::{bitenum, bitfield};

use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

use crate::commands::command_map::Command;

/// The System Control register defines the system control options.
/// It includes the [VOutControl](crate::commands::configuration::system_control::VOutControl) which control the output voltage.
/// The command version is also included in this register.
///
/// Datasheet Name: SYSTEM
#[bitfield(u8, default = 0x10)]
#[derive(Debug, PartialEq)]
pub struct SystemControl {
    /// VOUT Control Switch
    /// See [VOutControl](crate::commands::configuration::system_control::VOutControl) for more details.
    ///
    /// Datasheet Name: VOUTCTL
    #[bits(0..=1, rw)]
    pub v_out_control: Option<VoltageOutputControl>,
    // /// Reserved
    // #[bit(2, rw)]
    // reserved: u1,
    // /// Reserved
    // #[bit(3, rw)]
    // reserved2: u1,
    /// Command Version
    /// The command version is used to indicate the version of the command set.
    /// See [CommandVersion](crate::commands::configuration::system_control::CommandVersion) for more details.
    /// Datasheet Name: CMDVER
    #[bits(4..=5, r)]
    pub command_version: Option<CommandVersion>,
    // /// Reserved
    // #[bit(6, rw)]
    // reserved3: u1,
    // /// Reserved
    // #[bit(7, rw)]
    // reserved4: u1,
}

/// The AP33772S supports a command version.
/// The command version is used to indicate the version of the command set.
/// The current version is `V1_0`. The rest are reserved for future use.
///
/// Datasheet Name: CMDVER
#[bitenum(u2, exhaustive = false)]
pub enum CommandVersion {
    V1_0 = 0,
}

/// The AP33772S supports four VOUT Control modes.
/// - `Auto`: The output voltage is automatically controlled by the AP33772S.
/// - `ForceOff`: The output voltage is forced off.
/// - `ForceOn`: The output voltage is forced on.
/// - `Reserved`: This mode is reserved for future use.
///
/// Datasheet Name: VOUTCTL
#[bitenum(u2, exhaustive = false)]
#[derive(Debug, PartialEq, Default)]
pub enum VoltageOutputControl {
    #[default]
    Auto = 0,
    ForceOff = 1,
    ForceOn = 2,
}
impl_one_byte_read_command!(SystemControl, Command::SystemControl);
impl_one_byte_write_command!(SystemControl, Command::SystemControl);
