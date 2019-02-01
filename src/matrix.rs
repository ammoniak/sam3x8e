#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register"]
    pub matrix_mcfg: [MATRIX_MCFG; 6],
    _reserved0: [u8; 40usize],
    #[doc = "0x40 - Slave Configuration Register"]
    pub matrix_scfg: [MATRIX_SCFG; 9],
    _reserved1: [u8; 28usize],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: MATRIX_PRAS0,
    _reserved2: [u8; 4usize],
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: MATRIX_PRAS1,
    _reserved3: [u8; 4usize],
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: MATRIX_PRAS2,
    _reserved4: [u8; 4usize],
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: MATRIX_PRAS3,
    _reserved5: [u8; 4usize],
    #[doc = "0xa0 - Priority Register A for Slave 4"]
    pub matrix_pras4: MATRIX_PRAS4,
    _reserved6: [u8; 4usize],
    #[doc = "0xa8 - Priority Register A for Slave 5"]
    pub matrix_pras5: MATRIX_PRAS5,
    _reserved7: [u8; 4usize],
    #[doc = "0xb0 - Priority Register A for Slave 6"]
    pub matrix_pras6: MATRIX_PRAS6,
    _reserved8: [u8; 4usize],
    #[doc = "0xb8 - Priority Register A for Slave 7"]
    pub matrix_pras7: MATRIX_PRAS7,
    _reserved9: [u8; 4usize],
    #[doc = "0xc0 - Priority Register A for Slave 8"]
    pub matrix_pras8: MATRIX_PRAS8,
    _reserved10: [u8; 60usize],
    #[doc = "0x100 - Master Remap Control Register"]
    pub matrix_mrcr: MATRIX_MRCR,
    _reserved11: [u8; 16usize],
    #[doc = "0x114 - System I/O Configuration register"]
    pub ccfg_sysio: CCFG_SYSIO,
    _reserved12: [u8; 204usize],
    #[doc = "0x1e4 - Write Protect Mode Register"]
    pub matrix_wpmr: MATRIX_WPMR,
    #[doc = "0x1e8 - Write Protect Status Register"]
    pub matrix_wpsr: MATRIX_WPSR,
}
#[doc = "Master Configuration Register"]
pub struct MATRIX_MCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "Slave Configuration Register"]
pub struct MATRIX_SCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "Priority Register A for Slave 0"]
pub struct MATRIX_PRAS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "Priority Register A for Slave 1"]
pub struct MATRIX_PRAS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "Priority Register A for Slave 2"]
pub struct MATRIX_PRAS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "Priority Register A for Slave 3"]
pub struct MATRIX_PRAS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "Priority Register A for Slave 4"]
pub struct MATRIX_PRAS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 4"]
pub mod matrix_pras4;
#[doc = "Priority Register A for Slave 5"]
pub struct MATRIX_PRAS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 5"]
pub mod matrix_pras5;
#[doc = "Priority Register A for Slave 6"]
pub struct MATRIX_PRAS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 6"]
pub mod matrix_pras6;
#[doc = "Priority Register A for Slave 7"]
pub struct MATRIX_PRAS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 7"]
pub mod matrix_pras7;
#[doc = "Priority Register A for Slave 8"]
pub struct MATRIX_PRAS8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Priority Register A for Slave 8"]
pub mod matrix_pras8;
#[doc = "Master Remap Control Register"]
pub struct MATRIX_MRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Remap Control Register"]
pub mod matrix_mrcr;
#[doc = "System I/O Configuration register"]
pub struct CCFG_SYSIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System I/O Configuration register"]
pub mod ccfg_sysio;
#[doc = "Write Protect Mode Register"]
pub struct MATRIX_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode Register"]
pub mod matrix_wpmr;
#[doc = "Write Protect Status Register"]
pub struct MATRIX_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Status Register"]
pub mod matrix_wpsr;
