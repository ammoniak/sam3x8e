#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network Control Register"]
    pub ncr: NCR,
    #[doc = "0x04 - Network Configuration Register"]
    pub ncfgr: NCFGR,
    #[doc = "0x08 - Network Status Register"]
    pub nsr: NSR,
    _reserved3: [u8; 8usize],
    #[doc = "0x14 - Transmit Status Register"]
    pub tsr: TSR,
    #[doc = "0x18 - Receive Buffer Queue Pointer Register"]
    pub rbqp: RBQP,
    #[doc = "0x1c - Transmit Buffer Queue Pointer Register"]
    pub tbqp: TBQP,
    #[doc = "0x20 - Receive Status Register"]
    pub rsr: RSR,
    #[doc = "0x24 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x30 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x34 - Phy Maintenance Register"]
    pub man: MAN,
    #[doc = "0x38 - Pause Time Register"]
    pub ptr: PTR,
    #[doc = "0x3c - Pause Frames Received Register"]
    pub pfr: PFR,
    #[doc = "0x40 - Frames Transmitted Ok Register"]
    pub fto: FTO,
    #[doc = "0x44 - Single Collision Frames Register"]
    pub scf: SCF,
    #[doc = "0x48 - Multiple Collision Frames Register"]
    pub mcf: MCF,
    #[doc = "0x4c - Frames Received Ok Register"]
    pub fro: FRO,
    #[doc = "0x50 - Frame Check Sequence Errors Register"]
    pub fcse: FCSE,
    #[doc = "0x54 - Alignment Errors Register"]
    pub ale: ALE,
    #[doc = "0x58 - Deferred Transmission Frames Register"]
    pub dtf: DTF,
    #[doc = "0x5c - Late Collisions Register"]
    pub lcol: LCOL,
    #[doc = "0x60 - Excessive Collisions Register"]
    pub ecol: ECOL,
    #[doc = "0x64 - Transmit Underrun Errors Register"]
    pub tund: TUND,
    #[doc = "0x68 - Carrier Sense Errors Register"]
    pub cse: CSE,
    #[doc = "0x6c - Receive Resource Errors Register"]
    pub rre: RRE,
    #[doc = "0x70 - Receive Overrun Errors Register"]
    pub rov: ROV,
    #[doc = "0x74 - Receive Symbol Errors Register"]
    pub rse: RSE,
    #[doc = "0x78 - Excessive Length Errors Register"]
    pub ele: ELE,
    #[doc = "0x7c - Receive Jabbers Register"]
    pub rja: RJA,
    #[doc = "0x80 - Undersize Frames Register"]
    pub usf: USF,
    #[doc = "0x84 - SQE Test Errors Register"]
    pub ste: STE,
    #[doc = "0x88 - Received Length Field Mismatch Register"]
    pub rle: RLE,
    _reserved33: [u8; 4usize],
    #[doc = "0x90 - Hash Register Bottom \\[31:0\\] Register"]
    pub hrb: HRB,
    #[doc = "0x94 - Hash Register Top \\[63:32\\] Register"]
    pub hrt: HRT,
    #[doc = "0x98 - Specific Address 1 Bottom Register"]
    pub sa1b: SA1B,
    #[doc = "0x9c - Specific Address 1 Top Register"]
    pub sa1t: SA1T,
    #[doc = "0xa0 - Specific Address 2 Bottom Register"]
    pub sa2b: SA2B,
    #[doc = "0xa4 - Specific Address 2 Top Register"]
    pub sa2t: SA2T,
    #[doc = "0xa8 - Specific Address 3 Bottom Register"]
    pub sa3b: SA3B,
    #[doc = "0xac - Specific Address 3 Top Register"]
    pub sa3t: SA3T,
    #[doc = "0xb0 - Specific Address 4 Bottom Register"]
    pub sa4b: SA4B,
    #[doc = "0xb4 - Specific Address 4 Top Register"]
    pub sa4t: SA4T,
    #[doc = "0xb8 - Type ID Checking Register"]
    pub tid: TID,
    _reserved44: [u8; 4usize],
    #[doc = "0xc0 - User Input/Output Register"]
    pub usrio: USRIO,
}
#[doc = "Network Control Register"]
pub struct NCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "Network Configuration Register"]
pub struct NCFGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "Network Status Register"]
pub struct NSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "Transmit Status Register"]
pub struct TSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "Receive Buffer Queue Pointer Register"]
pub struct RBQP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Buffer Queue Pointer Register"]
pub mod rbqp;
#[doc = "Transmit Buffer Queue Pointer Register"]
pub struct TBQP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Buffer Queue Pointer Register"]
pub mod tbqp;
#[doc = "Receive Status Register"]
pub struct RSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Phy Maintenance Register"]
pub struct MAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Phy Maintenance Register"]
pub mod man;
#[doc = "Pause Time Register"]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pause Time Register"]
pub mod ptr;
#[doc = "Pause Frames Received Register"]
pub struct PFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "Frames Transmitted Ok Register"]
pub struct FTO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted Ok Register"]
pub mod fto;
#[doc = "Single Collision Frames Register"]
pub struct SCF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "Multiple Collision Frames Register"]
pub struct MCF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "Frames Received Ok Register"]
pub struct FRO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Received Ok Register"]
pub mod fro;
#[doc = "Frame Check Sequence Errors Register"]
pub struct FCSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "Alignment Errors Register"]
pub struct ALE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alignment Errors Register"]
pub mod ale;
#[doc = "Deferred Transmission Frames Register"]
pub struct DTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "Late Collisions Register"]
pub struct LCOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Late Collisions Register"]
pub mod lcol;
#[doc = "Excessive Collisions Register"]
pub struct ECOL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Excessive Collisions Register"]
pub mod ecol;
#[doc = "Transmit Underrun Errors Register"]
pub struct TUND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Underrun Errors Register"]
pub mod tund;
#[doc = "Carrier Sense Errors Register"]
pub struct CSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "Receive Resource Errors Register"]
pub struct RRE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "Receive Overrun Errors Register"]
pub struct ROV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Overrun Errors Register"]
pub mod rov;
#[doc = "Receive Symbol Errors Register"]
pub struct RSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "Excessive Length Errors Register"]
pub struct ELE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Excessive Length Errors Register"]
pub mod ele;
#[doc = "Receive Jabbers Register"]
pub struct RJA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Jabbers Register"]
pub mod rja;
#[doc = "Undersize Frames Register"]
pub struct USF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Undersize Frames Register"]
pub mod usf;
#[doc = "SQE Test Errors Register"]
pub struct STE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SQE Test Errors Register"]
pub mod ste;
#[doc = "Received Length Field Mismatch Register"]
pub struct RLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received Length Field Mismatch Register"]
pub mod rle;
#[doc = "Hash Register Bottom \\[31:0\\] Register"]
pub struct HRB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash Register Bottom \\[31:0\\] Register"]
pub mod hrb;
#[doc = "Hash Register Top \\[63:32\\] Register"]
pub struct HRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash Register Top \\[63:32\\] Register"]
pub mod hrt;
#[doc = "Specific Address 1 Bottom Register"]
pub struct SA1B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 1 Bottom Register"]
pub mod sa1b;
#[doc = "Specific Address 1 Top Register"]
pub struct SA1T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 1 Top Register"]
pub mod sa1t;
#[doc = "Specific Address 2 Bottom Register"]
pub struct SA2B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 2 Bottom Register"]
pub mod sa2b;
#[doc = "Specific Address 2 Top Register"]
pub struct SA2T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 2 Top Register"]
pub mod sa2t;
#[doc = "Specific Address 3 Bottom Register"]
pub struct SA3B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 3 Bottom Register"]
pub mod sa3b;
#[doc = "Specific Address 3 Top Register"]
pub struct SA3T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 3 Top Register"]
pub mod sa3t;
#[doc = "Specific Address 4 Bottom Register"]
pub struct SA4B {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 4 Bottom Register"]
pub mod sa4b;
#[doc = "Specific Address 4 Top Register"]
pub struct SA4T {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 4 Top Register"]
pub mod sa4t;
#[doc = "Type ID Checking Register"]
pub struct TID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Type ID Checking Register"]
pub mod tid;
#[doc = "User Input/Output Register"]
pub struct USRIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Input/Output Register"]
pub mod usrio;
