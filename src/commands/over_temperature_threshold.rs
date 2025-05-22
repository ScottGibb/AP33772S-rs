#[bitfield(u8, default = 0x78)]
#[derive(Debug, PartialEq)]
struct OverTemperatureThreshold{
    #[bits(0..=8, rw)]
    threshold: u8
}