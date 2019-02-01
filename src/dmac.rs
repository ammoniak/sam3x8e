#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAC Global Configuration Register"]
    pub gcfg: GCFG,
    #[doc = "0x04 - DMAC Enable Register"]
    pub en: EN,
    #[doc = "0x08 - DMAC Software Single Request Register"]
    pub sreq: SREQ,
    #[doc = "0x0c - DMAC Software Chunk Transfer Request Register"]
    pub creq: CREQ,
    #[doc = "0x10 - DMAC Software Last Transfer Flag Register"]
    pub last: LAST,
    _reserved0: [u8; 4usize],
    #[doc = "0x18 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
    pub ebcier: EBCIER,
    #[doc = "0x1c - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
    pub ebcidr: EBCIDR,
    #[doc = "0x20 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
    pub ebcimr: EBCIMR,
    #[doc = "0x24 - DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
    pub ebcisr: EBCISR,
    #[doc = "0x28 - DMAC Channel Handler Enable Register"]
    pub cher: CHER,
    #[doc = "0x2c - DMAC Channel Handler Disable Register"]
    pub chdr: CHDR,
    #[doc = "0x30 - DMAC Channel Handler Status Register"]
    pub chsr: CHSR,
    _reserved1: [u8; 8usize],
    #[doc = "0x3c - DMAC Channel Source Address Register (ch_num = 0)"]
    pub saddr0: SADDR0,
    #[doc = "0x40 - DMAC Channel Destination Address Register (ch_num = 0)"]
    pub daddr0: DADDR0,
    #[doc = "0x44 - DMAC Channel Descriptor Address Register (ch_num = 0)"]
    pub dscr0: DSCR0,
    #[doc = "0x48 - DMAC Channel Control A Register (ch_num = 0)"]
    pub ctrla0: CTRLA0,
    #[doc = "0x4c - DMAC Channel Control B Register (ch_num = 0)"]
    pub ctrlb0: CTRLB0,
    #[doc = "0x50 - DMAC Channel Configuration Register (ch_num = 0)"]
    pub cfg0: CFG0,
    _reserved2: [u8; 16usize],
    #[doc = "0x64 - DMAC Channel Source Address Register (ch_num = 1)"]
    pub saddr1: SADDR1,
    #[doc = "0x68 - DMAC Channel Destination Address Register (ch_num = 1)"]
    pub daddr1: DADDR1,
    #[doc = "0x6c - DMAC Channel Descriptor Address Register (ch_num = 1)"]
    pub dscr1: DSCR1,
    #[doc = "0x70 - DMAC Channel Control A Register (ch_num = 1)"]
    pub ctrla1: CTRLA1,
    #[doc = "0x74 - DMAC Channel Control B Register (ch_num = 1)"]
    pub ctrlb1: CTRLB1,
    #[doc = "0x78 - DMAC Channel Configuration Register (ch_num = 1)"]
    pub cfg1: CFG1,
    _reserved3: [u8; 16usize],
    #[doc = "0x8c - DMAC Channel Source Address Register (ch_num = 2)"]
    pub saddr2: SADDR2,
    #[doc = "0x90 - DMAC Channel Destination Address Register (ch_num = 2)"]
    pub daddr2: DADDR2,
    #[doc = "0x94 - DMAC Channel Descriptor Address Register (ch_num = 2)"]
    pub dscr2: DSCR2,
    #[doc = "0x98 - DMAC Channel Control A Register (ch_num = 2)"]
    pub ctrla2: CTRLA2,
    #[doc = "0x9c - DMAC Channel Control B Register (ch_num = 2)"]
    pub ctrlb2: CTRLB2,
    #[doc = "0xa0 - DMAC Channel Configuration Register (ch_num = 2)"]
    pub cfg2: CFG2,
    _reserved4: [u8; 16usize],
    #[doc = "0xb4 - DMAC Channel Source Address Register (ch_num = 3)"]
    pub saddr3: SADDR3,
    #[doc = "0xb8 - DMAC Channel Destination Address Register (ch_num = 3)"]
    pub daddr3: DADDR3,
    #[doc = "0xbc - DMAC Channel Descriptor Address Register (ch_num = 3)"]
    pub dscr3: DSCR3,
    #[doc = "0xc0 - DMAC Channel Control A Register (ch_num = 3)"]
    pub ctrla3: CTRLA3,
    #[doc = "0xc4 - DMAC Channel Control B Register (ch_num = 3)"]
    pub ctrlb3: CTRLB3,
    #[doc = "0xc8 - DMAC Channel Configuration Register (ch_num = 3)"]
    pub cfg3: CFG3,
    _reserved5: [u8; 16usize],
    #[doc = "0xdc - DMAC Channel Source Address Register (ch_num = 4)"]
    pub saddr4: SADDR4,
    #[doc = "0xe0 - DMAC Channel Destination Address Register (ch_num = 4)"]
    pub daddr4: DADDR4,
    #[doc = "0xe4 - DMAC Channel Descriptor Address Register (ch_num = 4)"]
    pub dscr4: DSCR4,
    #[doc = "0xe8 - DMAC Channel Control A Register (ch_num = 4)"]
    pub ctrla4: CTRLA4,
    #[doc = "0xec - DMAC Channel Control B Register (ch_num = 4)"]
    pub ctrlb4: CTRLB4,
    #[doc = "0xf0 - DMAC Channel Configuration Register (ch_num = 4)"]
    pub cfg4: CFG4,
    _reserved6: [u8; 16usize],
    #[doc = "0x104 - DMAC Channel Source Address Register (ch_num = 5)"]
    pub saddr5: SADDR5,
    #[doc = "0x108 - DMAC Channel Destination Address Register (ch_num = 5)"]
    pub daddr5: DADDR5,
    #[doc = "0x10c - DMAC Channel Descriptor Address Register (ch_num = 5)"]
    pub dscr5: DSCR5,
    #[doc = "0x110 - DMAC Channel Control A Register (ch_num = 5)"]
    pub ctrla5: CTRLA5,
    #[doc = "0x114 - DMAC Channel Control B Register (ch_num = 5)"]
    pub ctrlb5: CTRLB5,
    #[doc = "0x118 - DMAC Channel Configuration Register (ch_num = 5)"]
    pub cfg5: CFG5,
    _reserved7: [u8; 200usize],
    #[doc = "0x1e4 - DMAC Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0x1e8 - DMAC Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "DMAC Global Configuration Register"]
pub struct GCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Global Configuration Register"]
pub mod gcfg;
#[doc = "DMAC Enable Register"]
pub struct EN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Enable Register"]
pub mod en;
#[doc = "DMAC Software Single Request Register"]
pub struct SREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Software Single Request Register"]
pub mod sreq;
#[doc = "DMAC Software Chunk Transfer Request Register"]
pub struct CREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Software Chunk Transfer Request Register"]
pub mod creq;
#[doc = "DMAC Software Last Transfer Flag Register"]
pub struct LAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Software Last Transfer Flag Register"]
pub mod last;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
pub struct EBCIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
pub mod ebcier;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
pub struct EBCIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
pub mod ebcidr;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
pub struct EBCIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
pub mod ebcimr;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
pub struct EBCISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
pub mod ebcisr;
#[doc = "DMAC Channel Handler Enable Register"]
pub struct CHER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Handler Enable Register"]
pub mod cher;
#[doc = "DMAC Channel Handler Disable Register"]
pub struct CHDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Handler Disable Register"]
pub mod chdr;
#[doc = "DMAC Channel Handler Status Register"]
pub struct CHSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Handler Status Register"]
pub mod chsr;
#[doc = "DMAC Channel Source Address Register (ch_num = 0)"]
pub struct SADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Source Address Register (ch_num = 0)"]
pub mod saddr0;
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)"]
pub struct DADDR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)"]
pub mod daddr0;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 0)"]
pub struct DSCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 0)"]
pub mod dscr0;
#[doc = "DMAC Channel Control A Register (ch_num = 0)"]
pub struct CTRLA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control A Register (ch_num = 0)"]
pub mod ctrla0;
#[doc = "DMAC Channel Control B Register (ch_num = 0)"]
pub struct CTRLB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control B Register (ch_num = 0)"]
pub mod ctrlb0;
#[doc = "DMAC Channel Configuration Register (ch_num = 0)"]
pub struct CFG0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Configuration Register (ch_num = 0)"]
pub mod cfg0;
#[doc = "DMAC Channel Source Address Register (ch_num = 1)"]
pub struct SADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Source Address Register (ch_num = 1)"]
pub mod saddr1;
#[doc = "DMAC Channel Destination Address Register (ch_num = 1)"]
pub struct DADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 1)"]
pub mod daddr1;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 1)"]
pub struct DSCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 1)"]
pub mod dscr1;
#[doc = "DMAC Channel Control A Register (ch_num = 1)"]
pub struct CTRLA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control A Register (ch_num = 1)"]
pub mod ctrla1;
#[doc = "DMAC Channel Control B Register (ch_num = 1)"]
pub struct CTRLB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control B Register (ch_num = 1)"]
pub mod ctrlb1;
#[doc = "DMAC Channel Configuration Register (ch_num = 1)"]
pub struct CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Configuration Register (ch_num = 1)"]
pub mod cfg1;
#[doc = "DMAC Channel Source Address Register (ch_num = 2)"]
pub struct SADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Source Address Register (ch_num = 2)"]
pub mod saddr2;
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)"]
pub struct DADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)"]
pub mod daddr2;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 2)"]
pub struct DSCR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 2)"]
pub mod dscr2;
#[doc = "DMAC Channel Control A Register (ch_num = 2)"]
pub struct CTRLA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control A Register (ch_num = 2)"]
pub mod ctrla2;
#[doc = "DMAC Channel Control B Register (ch_num = 2)"]
pub struct CTRLB2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control B Register (ch_num = 2)"]
pub mod ctrlb2;
#[doc = "DMAC Channel Configuration Register (ch_num = 2)"]
pub struct CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Configuration Register (ch_num = 2)"]
pub mod cfg2;
#[doc = "DMAC Channel Source Address Register (ch_num = 3)"]
pub struct SADDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Source Address Register (ch_num = 3)"]
pub mod saddr3;
#[doc = "DMAC Channel Destination Address Register (ch_num = 3)"]
pub struct DADDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 3)"]
pub mod daddr3;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)"]
pub struct DSCR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)"]
pub mod dscr3;
#[doc = "DMAC Channel Control A Register (ch_num = 3)"]
pub struct CTRLA3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control A Register (ch_num = 3)"]
pub mod ctrla3;
#[doc = "DMAC Channel Control B Register (ch_num = 3)"]
pub struct CTRLB3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control B Register (ch_num = 3)"]
pub mod ctrlb3;
#[doc = "DMAC Channel Configuration Register (ch_num = 3)"]
pub struct CFG3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Configuration Register (ch_num = 3)"]
pub mod cfg3;
#[doc = "DMAC Channel Source Address Register (ch_num = 4)"]
pub struct SADDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Source Address Register (ch_num = 4)"]
pub mod saddr4;
#[doc = "DMAC Channel Destination Address Register (ch_num = 4)"]
pub struct DADDR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 4)"]
pub mod daddr4;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 4)"]
pub struct DSCR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 4)"]
pub mod dscr4;
#[doc = "DMAC Channel Control A Register (ch_num = 4)"]
pub struct CTRLA4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control A Register (ch_num = 4)"]
pub mod ctrla4;
#[doc = "DMAC Channel Control B Register (ch_num = 4)"]
pub struct CTRLB4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control B Register (ch_num = 4)"]
pub mod ctrlb4;
#[doc = "DMAC Channel Configuration Register (ch_num = 4)"]
pub struct CFG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Configuration Register (ch_num = 4)"]
pub mod cfg4;
#[doc = "DMAC Channel Source Address Register (ch_num = 5)"]
pub struct SADDR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Source Address Register (ch_num = 5)"]
pub mod saddr5;
#[doc = "DMAC Channel Destination Address Register (ch_num = 5)"]
pub struct DADDR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Destination Address Register (ch_num = 5)"]
pub mod daddr5;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 5)"]
pub struct DSCR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 5)"]
pub mod dscr5;
#[doc = "DMAC Channel Control A Register (ch_num = 5)"]
pub struct CTRLA5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control A Register (ch_num = 5)"]
pub mod ctrla5;
#[doc = "DMAC Channel Control B Register (ch_num = 5)"]
pub struct CTRLB5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Control B Register (ch_num = 5)"]
pub mod ctrlb5;
#[doc = "DMAC Channel Configuration Register (ch_num = 5)"]
pub struct CFG5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Channel Configuration Register (ch_num = 5)"]
pub mod cfg5;
#[doc = "DMAC Write Protect Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "DMAC Write Protect Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAC Write Protect Status Register"]
pub mod wpsr;
