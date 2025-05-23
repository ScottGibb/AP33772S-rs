use arbitrary_int::u1;
use bitbybit::{bitenum, bitfield};

use crate::{impl_one_byte_read_command, impl_one_byte_write_command};

use super::command_map::Command;

/// Command: SYSTEM
#[bitfield(u8, default = 0x10)]
#[derive(Debug, PartialEq)]
pub struct SystemControl{
    /// VOUTCTL
    #[bits(0..=1, rw)]
    pub v_out_control: VOutControl, 
    /// Reserved
    #[bit(2, rw)]
    pub reserved: u1,
    /// Reserved
    #[bit(3, rw)]
    pub reserved2: u1,
    /// CMDVER
    #[bits(4..=5, r)]
    pub command_version: Option<CommandVersion>,
    /// Reserved
    #[bit(6, rw)]
    pub reserved3: u1,
    /// Reserved
    #[bit(7, rw)]
    pub reserved4: u1,
}


#[bitenum(u2, exhaustive=false)]
pub enum CommandVersion {
    V1_0 = 0,   
}

#[bitenum(u2, exhaustive=true)]
#[derive(Debug, PartialEq, Default)]
pub enum VOutControl {
    #[default]
    Auto =0,
    ForceOff=1,
    ForceOn=2,
    Reserved=3,
}
impl_one_byte_read_command!(SystemControl, Command::SystemControl);
impl_one_byte_write_command!(SystemControl, Command::SystemControl);