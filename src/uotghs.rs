#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub devctrl: DEVCTRL,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub devisr: DEVISR,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub devicr: DEVICR,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub devifr: DEVIFR,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub devimr: DEVIMR,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub devidr: DEVIDR,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub devier: DEVIER,
    #[doc = "0x1c - Device Endpoint Register"]
    pub devept: DEVEPT,
    #[doc = "0x20 - Device Frame Number Register"]
    pub devfnum: DEVFNUM,
    _reserved9: [u8; 220usize],
    #[doc = "0x100 - Device Endpoint Configuration Register (n = 0)"]
    pub deveptcfg: [DEVEPTCFG; 10],
    _reserved10: [u8; 8usize],
    #[doc = "Device Endpoint Status Register (n = 0)"]
    pub deveptisr: DEVEPTISR_UNION,
    _reserved11: [u8; 8usize],
    #[doc = "Device Endpoint Clear Register (n = 0)"]
    pub devepticr: DEVEPTICR_UNION,
    _reserved12: [u8; 8usize],
    #[doc = "Device Endpoint Set Register (n = 0)"]
    pub deveptifr: DEVEPTIFR_UNION,
    _reserved13: [u8; 8usize],
    #[doc = "Device Endpoint Mask Register (n = 0)"]
    pub deveptimr: DEVEPTIMR_UNION,
    _reserved14: [u8; 8usize],
    #[doc = "Device Endpoint Enable Register (n = 0)"]
    pub deveptier: DEVEPTIER_UNION,
    _reserved15: [u8; 8usize],
    #[doc = "Device Endpoint Disable Register (n = 0)"]
    pub deveptidr: DEVEPTIDR_UNION,
    _reserved16: [u8; 200usize],
    #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub devdmanxtdsc1: DEVDMANXTDSC1,
    #[doc = "0x314 - Device DMA Channel Address Register (n = 1)"]
    pub devdmaaddress1: DEVDMAADDRESS1,
    #[doc = "0x318 - Device DMA Channel Control Register (n = 1)"]
    pub devdmacontrol1: DEVDMACONTROL1,
    #[doc = "0x31c - Device DMA Channel Status Register (n = 1)"]
    pub devdmastatus1: DEVDMASTATUS1,
    #[doc = "0x320 - Device DMA Channel Next Descriptor Address Register (n = 2)"]
    pub devdmanxtdsc2: DEVDMANXTDSC2,
    #[doc = "0x324 - Device DMA Channel Address Register (n = 2)"]
    pub devdmaaddress2: DEVDMAADDRESS2,
    #[doc = "0x328 - Device DMA Channel Control Register (n = 2)"]
    pub devdmacontrol2: DEVDMACONTROL2,
    #[doc = "0x32c - Device DMA Channel Status Register (n = 2)"]
    pub devdmastatus2: DEVDMASTATUS2,
    #[doc = "0x330 - Device DMA Channel Next Descriptor Address Register (n = 3)"]
    pub devdmanxtdsc3: DEVDMANXTDSC3,
    #[doc = "0x334 - Device DMA Channel Address Register (n = 3)"]
    pub devdmaaddress3: DEVDMAADDRESS3,
    #[doc = "0x338 - Device DMA Channel Control Register (n = 3)"]
    pub devdmacontrol3: DEVDMACONTROL3,
    #[doc = "0x33c - Device DMA Channel Status Register (n = 3)"]
    pub devdmastatus3: DEVDMASTATUS3,
    #[doc = "0x340 - Device DMA Channel Next Descriptor Address Register (n = 4)"]
    pub devdmanxtdsc4: DEVDMANXTDSC4,
    #[doc = "0x344 - Device DMA Channel Address Register (n = 4)"]
    pub devdmaaddress4: DEVDMAADDRESS4,
    #[doc = "0x348 - Device DMA Channel Control Register (n = 4)"]
    pub devdmacontrol4: DEVDMACONTROL4,
    #[doc = "0x34c - Device DMA Channel Status Register (n = 4)"]
    pub devdmastatus4: DEVDMASTATUS4,
    #[doc = "0x350 - Device DMA Channel Next Descriptor Address Register (n = 5)"]
    pub devdmanxtdsc5: DEVDMANXTDSC5,
    #[doc = "0x354 - Device DMA Channel Address Register (n = 5)"]
    pub devdmaaddress5: DEVDMAADDRESS5,
    #[doc = "0x358 - Device DMA Channel Control Register (n = 5)"]
    pub devdmacontrol5: DEVDMACONTROL5,
    #[doc = "0x35c - Device DMA Channel Status Register (n = 5)"]
    pub devdmastatus5: DEVDMASTATUS5,
    #[doc = "0x360 - Device DMA Channel Next Descriptor Address Register (n = 6)"]
    pub devdmanxtdsc6: DEVDMANXTDSC6,
    #[doc = "0x364 - Device DMA Channel Address Register (n = 6)"]
    pub devdmaaddress6: DEVDMAADDRESS6,
    #[doc = "0x368 - Device DMA Channel Control Register (n = 6)"]
    pub devdmacontrol6: DEVDMACONTROL6,
    #[doc = "0x36c - Device DMA Channel Status Register (n = 6)"]
    pub devdmastatus6: DEVDMASTATUS6,
    #[doc = "0x370 - Device DMA Channel Next Descriptor Address Register (n = 7)"]
    pub devdmanxtdsc7: DEVDMANXTDSC7,
    #[doc = "0x374 - Device DMA Channel Address Register (n = 7)"]
    pub devdmaaddress7: DEVDMAADDRESS7,
    #[doc = "0x378 - Device DMA Channel Control Register (n = 7)"]
    pub devdmacontrol7: DEVDMACONTROL7,
    #[doc = "0x37c - Device DMA Channel Status Register (n = 7)"]
    pub devdmastatus7: DEVDMASTATUS7,
    _reserved44: [u8; 128usize],
    #[doc = "0x400 - Host General Control Register"]
    pub hstctrl: HSTCTRL,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub hstisr: HSTISR,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub hsticr: HSTICR,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub hstifr: HSTIFR,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub hstimr: HSTIMR,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub hstidr: HSTIDR,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub hstier: HSTIER,
    #[doc = "0x41c - Host Pipe Register"]
    pub hstpip: HSTPIP,
    #[doc = "0x420 - Host Frame Number Register"]
    pub hstfnum: HSTFNUM,
    #[doc = "0x424 - Host Address 1 Register"]
    pub hstaddr1: HSTADDR1,
    #[doc = "0x428 - Host Address 2 Register"]
    pub hstaddr2: HSTADDR2,
    #[doc = "0x42c - Host Address 3 Register"]
    pub hstaddr3: HSTADDR3,
    _reserved56: [u8; 208usize],
    #[doc = "Host Pipe Configuration Register (n = 0)"]
    pub hstpipcfg: HSTPIPCFG_UNION,
    _reserved57: [u8; 8usize],
    #[doc = "Host Pipe Status Register (n = 0)"]
    pub hstpipisr: HSTPIPISR_UNION,
    _reserved58: [u8; 8usize],
    #[doc = "Host Pipe Clear Register (n = 0)"]
    pub hstpipicr: HSTPIPICR_UNION,
    _reserved59: [u8; 8usize],
    #[doc = "Host Pipe Set Register (n = 0)"]
    pub hstpipifr: HSTPIPIFR_UNION,
    _reserved60: [u8; 8usize],
    #[doc = "Host Pipe Mask Register (n = 0)"]
    pub hstpipimr: HSTPIPIMR_UNION,
    _reserved61: [u8; 8usize],
    #[doc = "Host Pipe Enable Register (n = 0)"]
    pub hstpipier: HSTPIPIER_UNION,
    _reserved62: [u8; 8usize],
    #[doc = "Host Pipe Disable Register (n = 0)"]
    pub hstpipidr: HSTPIPIDR_UNION,
    _reserved63: [u8; 8usize],
    #[doc = "0x650 - Host Pipe IN Request Register (n = 0)"]
    pub hstpipinrq: [HSTPIPINRQ; 10],
    _reserved64: [u8; 8usize],
    #[doc = "0x680 - Host Pipe Error Register (n = 0)"]
    pub hstpiperr: [HSTPIPERR; 10],
    _reserved65: [u8; 104usize],
    #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub hstdmanxtdsc1: HSTDMANXTDSC1,
    #[doc = "0x714 - Host DMA Channel Address Register (n = 1)"]
    pub hstdmaaddress1: HSTDMAADDRESS1,
    #[doc = "0x718 - Host DMA Channel Control Register (n = 1)"]
    pub hstdmacontrol1: HSTDMACONTROL1,
    #[doc = "0x71c - Host DMA Channel Status Register (n = 1)"]
    pub hstdmastatus1: HSTDMASTATUS1,
    #[doc = "0x720 - Host DMA Channel Next Descriptor Address Register (n = 2)"]
    pub hstdmanxtdsc2: HSTDMANXTDSC2,
    #[doc = "0x724 - Host DMA Channel Address Register (n = 2)"]
    pub hstdmaaddress2: HSTDMAADDRESS2,
    #[doc = "0x728 - Host DMA Channel Control Register (n = 2)"]
    pub hstdmacontrol2: HSTDMACONTROL2,
    #[doc = "0x72c - Host DMA Channel Status Register (n = 2)"]
    pub hstdmastatus2: HSTDMASTATUS2,
    #[doc = "0x730 - Host DMA Channel Next Descriptor Address Register (n = 3)"]
    pub hstdmanxtdsc3: HSTDMANXTDSC3,
    #[doc = "0x734 - Host DMA Channel Address Register (n = 3)"]
    pub hstdmaaddress3: HSTDMAADDRESS3,
    #[doc = "0x738 - Host DMA Channel Control Register (n = 3)"]
    pub hstdmacontrol3: HSTDMACONTROL3,
    #[doc = "0x73c - Host DMA Channel Status Register (n = 3)"]
    pub hstdmastatus3: HSTDMASTATUS3,
    #[doc = "0x740 - Host DMA Channel Next Descriptor Address Register (n = 4)"]
    pub hstdmanxtdsc4: HSTDMANXTDSC4,
    #[doc = "0x744 - Host DMA Channel Address Register (n = 4)"]
    pub hstdmaaddress4: HSTDMAADDRESS4,
    #[doc = "0x748 - Host DMA Channel Control Register (n = 4)"]
    pub hstdmacontrol4: HSTDMACONTROL4,
    #[doc = "0x74c - Host DMA Channel Status Register (n = 4)"]
    pub hstdmastatus4: HSTDMASTATUS4,
    #[doc = "0x750 - Host DMA Channel Next Descriptor Address Register (n = 5)"]
    pub hstdmanxtdsc5: HSTDMANXTDSC5,
    #[doc = "0x754 - Host DMA Channel Address Register (n = 5)"]
    pub hstdmaaddress5: HSTDMAADDRESS5,
    #[doc = "0x758 - Host DMA Channel Control Register (n = 5)"]
    pub hstdmacontrol5: HSTDMACONTROL5,
    #[doc = "0x75c - Host DMA Channel Status Register (n = 5)"]
    pub hstdmastatus5: HSTDMASTATUS5,
    #[doc = "0x760 - Host DMA Channel Next Descriptor Address Register (n = 6)"]
    pub hstdmanxtdsc6: HSTDMANXTDSC6,
    #[doc = "0x764 - Host DMA Channel Address Register (n = 6)"]
    pub hstdmaaddress6: HSTDMAADDRESS6,
    #[doc = "0x768 - Host DMA Channel Control Register (n = 6)"]
    pub hstdmacontrol6: HSTDMACONTROL6,
    #[doc = "0x76c - Host DMA Channel Status Register (n = 6)"]
    pub hstdmastatus6: HSTDMASTATUS6,
    #[doc = "0x770 - Host DMA Channel Next Descriptor Address Register (n = 7)"]
    pub hstdmanxtdsc7: HSTDMANXTDSC7,
    #[doc = "0x774 - Host DMA Channel Address Register (n = 7)"]
    pub hstdmaaddress7: HSTDMAADDRESS7,
    #[doc = "0x778 - Host DMA Channel Control Register (n = 7)"]
    pub hstdmacontrol7: HSTDMACONTROL7,
    #[doc = "0x77c - Host DMA Channel Status Register (n = 7)"]
    pub hstdmastatus7: HSTDMASTATUS7,
    _reserved93: [u8; 128usize],
    #[doc = "0x800 - General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x804 - General Status Register"]
    pub sr: SR,
    #[doc = "0x808 - General Status Clear Register"]
    pub scr: SCR,
    #[doc = "0x80c - General Status Set Register"]
    pub sfr: SFR,
    _reserved97: [u8; 28usize],
    #[doc = "0x82c - General Finite State Machine Register"]
    pub fsm: FSM,
}
#[doc = "Device Endpoint Status Register (n = 0)"]
#[repr(C)]
pub union DEVEPTISR_UNION {
    #[doc = "0x130 - Device Endpoint Status Register (n = 0)"]
    pub deveptisr0_isoenpt: DEVEPTISR0_ISOENPT,
    #[doc = "0x130 - Device Endpoint Status Register (n = 0)"]
    pub deveptisr: [DEVEPTISR; 10],
}
#[doc = "Device Endpoint Clear Register (n = 0)"]
#[repr(C)]
pub union DEVEPTICR_UNION {
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0)"]
    pub devepticr0_isoenpt: DEVEPTICR0_ISOENPT,
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0)"]
    pub devepticr: [DEVEPTICR; 10],
}
#[doc = "Device Endpoint Set Register (n = 0)"]
#[repr(C)]
pub union DEVEPTIFR_UNION {
    #[doc = "0x190 - Device Endpoint Set Register (n = 0)"]
    pub deveptifr0_isoenpt: DEVEPTIFR0_ISOENPT,
    #[doc = "0x190 - Device Endpoint Set Register (n = 0)"]
    pub deveptifr: [DEVEPTIFR; 10],
}
#[doc = "Device Endpoint Mask Register (n = 0)"]
#[repr(C)]
pub union DEVEPTIMR_UNION {
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0)"]
    pub deveptimr0_isoenpt: DEVEPTIMR0_ISOENPT,
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0)"]
    pub deveptimr: [DEVEPTIMR; 10],
}
#[doc = "Device Endpoint Enable Register (n = 0)"]
#[repr(C)]
pub union DEVEPTIER_UNION {
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0)"]
    pub deveptier0_isoenpt: DEVEPTIER0_ISOENPT,
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0)"]
    pub deveptier: [DEVEPTIER; 10],
}
#[doc = "Device Endpoint Disable Register (n = 0)"]
#[repr(C)]
pub union DEVEPTIDR_UNION {
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0)"]
    pub deveptidr0_isoenpt: DEVEPTIDR0_ISOENPT,
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0)"]
    pub deveptidr: [DEVEPTIDR; 10],
}
#[doc = "Host Pipe Configuration Register (n = 0)"]
#[repr(C)]
pub union HSTPIPCFG_UNION {
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0)"]
    pub hstpipcfg0_hsbohscp: HSTPIPCFG0_HSBOHSCP,
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0)"]
    pub hstpipcfg: [HSTPIPCFG; 10],
}
#[doc = "Host Pipe Status Register (n = 0)"]
#[repr(C)]
pub union HSTPIPISR_UNION {
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    pub hstpipisr0_isopipes: HSTPIPISR0_ISOPIPES,
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    pub hstpipisr0_intpipes: HSTPIPISR0_INTPIPES,
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    pub hstpipisr: [HSTPIPISR; 10],
}
#[doc = "Host Pipe Clear Register (n = 0)"]
#[repr(C)]
pub union HSTPIPICR_UNION {
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    pub hstpipicr0_isopipes: HSTPIPICR0_ISOPIPES,
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    pub hstpipicr0_intpipes: HSTPIPICR0_INTPIPES,
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    pub hstpipicr: [HSTPIPICR; 10],
}
#[doc = "Host Pipe Set Register (n = 0)"]
#[repr(C)]
pub union HSTPIPIFR_UNION {
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    pub hstpipifr0_isopipes: HSTPIPIFR0_ISOPIPES,
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    pub hstpipifr0_intpipes: HSTPIPIFR0_INTPIPES,
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    pub hstpipifr: [HSTPIPIFR; 10],
}
#[doc = "Host Pipe Mask Register (n = 0)"]
#[repr(C)]
pub union HSTPIPIMR_UNION {
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    pub hstpipimr0_isopipes: HSTPIPIMR0_ISOPIPES,
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    pub hstpipimr0_intpipes: HSTPIPIMR0_INTPIPES,
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    pub hstpipimr: [HSTPIPIMR; 10],
}
#[doc = "Host Pipe Enable Register (n = 0)"]
#[repr(C)]
pub union HSTPIPIER_UNION {
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    pub hstpipier0_isopipes: HSTPIPIER0_ISOPIPES,
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    pub hstpipier0_intpipes: HSTPIPIER0_INTPIPES,
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    pub hstpipier: [HSTPIPIER; 10],
}
#[doc = "Host Pipe Disable Register (n = 0)"]
#[repr(C)]
pub union HSTPIPIDR_UNION {
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    pub hstpipidr0_isopipes: HSTPIPIDR0_ISOPIPES,
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    pub hstpipidr0_intpipes: HSTPIPIDR0_INTPIPES,
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    pub hstpipidr: [HSTPIPIDR; 10],
}
#[doc = "Device General Control Register"]
pub struct DEVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "Device Global Interrupt Status Register"]
pub struct DEVISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "Device Global Interrupt Clear Register"]
pub struct DEVICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "Device Global Interrupt Set Register"]
pub struct DEVIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "Device Global Interrupt Mask Register"]
pub struct DEVIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "Device Global Interrupt Disable Register"]
pub struct DEVIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "Device Global Interrupt Enable Register"]
pub struct DEVIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "Device Endpoint Register"]
pub struct DEVEPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "Device Frame Number Register"]
pub struct DEVFNUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "Device Endpoint Configuration Register (n = 0)"]
pub struct DEVEPTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Configuration Register (n = 0)"]
pub mod deveptcfg;
#[doc = "Device Endpoint Status Register (n = 0)"]
pub struct DEVEPTISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Status Register (n = 0)"]
pub mod deveptisr;
#[doc = "Device Endpoint Status Register (n = 0)"]
pub struct DEVEPTISR0_ISOENPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Status Register (n = 0)"]
pub mod deveptisr0_isoenpt;
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub struct DEVEPTICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub mod devepticr;
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub struct DEVEPTICR0_ISOENPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub mod devepticr0_isoenpt;
#[doc = "Device Endpoint Set Register (n = 0)"]
pub struct DEVEPTIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Set Register (n = 0)"]
pub mod deveptifr;
#[doc = "Device Endpoint Set Register (n = 0)"]
pub struct DEVEPTIFR0_ISOENPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Set Register (n = 0)"]
pub mod deveptifr0_isoenpt;
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub struct DEVEPTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub mod deveptimr;
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub struct DEVEPTIMR0_ISOENPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub mod deveptimr0_isoenpt;
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub struct DEVEPTIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub mod deveptier;
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub struct DEVEPTIER0_ISOENPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub mod deveptier0_isoenpt;
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub struct DEVEPTIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub mod deveptidr;
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub struct DEVEPTIDR0_ISOENPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub mod deveptidr0_isoenpt;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub struct DEVDMANXTDSC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod devdmanxtdsc1;
#[doc = "Device DMA Channel Address Register (n = 1)"]
pub struct DEVDMAADDRESS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Address Register (n = 1)"]
pub mod devdmaaddress1;
#[doc = "Device DMA Channel Control Register (n = 1)"]
pub struct DEVDMACONTROL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Control Register (n = 1)"]
pub mod devdmacontrol1;
#[doc = "Device DMA Channel Status Register (n = 1)"]
pub struct DEVDMASTATUS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Status Register (n = 1)"]
pub mod devdmastatus1;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 2)"]
pub struct DEVDMANXTDSC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod devdmanxtdsc2;
#[doc = "Device DMA Channel Address Register (n = 2)"]
pub struct DEVDMAADDRESS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Address Register (n = 2)"]
pub mod devdmaaddress2;
#[doc = "Device DMA Channel Control Register (n = 2)"]
pub struct DEVDMACONTROL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Control Register (n = 2)"]
pub mod devdmacontrol2;
#[doc = "Device DMA Channel Status Register (n = 2)"]
pub struct DEVDMASTATUS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Status Register (n = 2)"]
pub mod devdmastatus2;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 3)"]
pub struct DEVDMANXTDSC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod devdmanxtdsc3;
#[doc = "Device DMA Channel Address Register (n = 3)"]
pub struct DEVDMAADDRESS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Address Register (n = 3)"]
pub mod devdmaaddress3;
#[doc = "Device DMA Channel Control Register (n = 3)"]
pub struct DEVDMACONTROL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Control Register (n = 3)"]
pub mod devdmacontrol3;
#[doc = "Device DMA Channel Status Register (n = 3)"]
pub struct DEVDMASTATUS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Status Register (n = 3)"]
pub mod devdmastatus3;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 4)"]
pub struct DEVDMANXTDSC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod devdmanxtdsc4;
#[doc = "Device DMA Channel Address Register (n = 4)"]
pub struct DEVDMAADDRESS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Address Register (n = 4)"]
pub mod devdmaaddress4;
#[doc = "Device DMA Channel Control Register (n = 4)"]
pub struct DEVDMACONTROL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Control Register (n = 4)"]
pub mod devdmacontrol4;
#[doc = "Device DMA Channel Status Register (n = 4)"]
pub struct DEVDMASTATUS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Status Register (n = 4)"]
pub mod devdmastatus4;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 5)"]
pub struct DEVDMANXTDSC5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod devdmanxtdsc5;
#[doc = "Device DMA Channel Address Register (n = 5)"]
pub struct DEVDMAADDRESS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Address Register (n = 5)"]
pub mod devdmaaddress5;
#[doc = "Device DMA Channel Control Register (n = 5)"]
pub struct DEVDMACONTROL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Control Register (n = 5)"]
pub mod devdmacontrol5;
#[doc = "Device DMA Channel Status Register (n = 5)"]
pub struct DEVDMASTATUS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Status Register (n = 5)"]
pub mod devdmastatus5;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 6)"]
pub struct DEVDMANXTDSC6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod devdmanxtdsc6;
#[doc = "Device DMA Channel Address Register (n = 6)"]
pub struct DEVDMAADDRESS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Address Register (n = 6)"]
pub mod devdmaaddress6;
#[doc = "Device DMA Channel Control Register (n = 6)"]
pub struct DEVDMACONTROL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Control Register (n = 6)"]
pub mod devdmacontrol6;
#[doc = "Device DMA Channel Status Register (n = 6)"]
pub struct DEVDMASTATUS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Status Register (n = 6)"]
pub mod devdmastatus6;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 7)"]
pub struct DEVDMANXTDSC7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod devdmanxtdsc7;
#[doc = "Device DMA Channel Address Register (n = 7)"]
pub struct DEVDMAADDRESS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Address Register (n = 7)"]
pub mod devdmaaddress7;
#[doc = "Device DMA Channel Control Register (n = 7)"]
pub struct DEVDMACONTROL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Control Register (n = 7)"]
pub mod devdmacontrol7;
#[doc = "Device DMA Channel Status Register (n = 7)"]
pub struct DEVDMASTATUS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device DMA Channel Status Register (n = 7)"]
pub mod devdmastatus7;
#[doc = "Host General Control Register"]
pub struct HSTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "Host Global Interrupt Status Register"]
pub struct HSTISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "Host Global Interrupt Clear Register"]
pub struct HSTICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "Host Global Interrupt Set Register"]
pub struct HSTIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "Host Global Interrupt Mask Register"]
pub struct HSTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "Host Global Interrupt Disable Register"]
pub struct HSTIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "Host Global Interrupt Enable Register"]
pub struct HSTIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "Host Pipe Register"]
pub struct HSTPIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "Host Frame Number Register"]
pub struct HSTFNUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "Host Address 1 Register"]
pub struct HSTADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "Host Address 2 Register"]
pub struct HSTADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "Host Address 3 Register"]
pub struct HSTADDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub struct HSTPIPCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub mod hstpipcfg;
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub struct HSTPIPCFG0_HSBOHSCP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub mod hstpipcfg0_hsbohscp;
#[doc = "Host Pipe Status Register (n = 0)"]
pub struct HSTPIPISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod hstpipisr;
#[doc = "Host Pipe Status Register (n = 0)"]
pub struct HSTPIPISR0_INTPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod hstpipisr0_intpipes;
#[doc = "Host Pipe Status Register (n = 0)"]
pub struct HSTPIPISR0_ISOPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod hstpipisr0_isopipes;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub struct HSTPIPICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod hstpipicr;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub struct HSTPIPICR0_INTPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod hstpipicr0_intpipes;
#[doc = "Host Pipe Clear Register (n = 0)"]
pub struct HSTPIPICR0_ISOPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod hstpipicr0_isopipes;
#[doc = "Host Pipe Set Register (n = 0)"]
pub struct HSTPIPIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod hstpipifr;
#[doc = "Host Pipe Set Register (n = 0)"]
pub struct HSTPIPIFR0_INTPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod hstpipifr0_intpipes;
#[doc = "Host Pipe Set Register (n = 0)"]
pub struct HSTPIPIFR0_ISOPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod hstpipifr0_isopipes;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub struct HSTPIPIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod hstpipimr;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub struct HSTPIPIMR0_INTPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod hstpipimr0_intpipes;
#[doc = "Host Pipe Mask Register (n = 0)"]
pub struct HSTPIPIMR0_ISOPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod hstpipimr0_isopipes;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub struct HSTPIPIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod hstpipier;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub struct HSTPIPIER0_INTPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod hstpipier0_intpipes;
#[doc = "Host Pipe Enable Register (n = 0)"]
pub struct HSTPIPIER0_ISOPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod hstpipier0_isopipes;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub struct HSTPIPIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod hstpipidr;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub struct HSTPIPIDR0_INTPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod hstpipidr0_intpipes;
#[doc = "Host Pipe Disable Register (n = 0)"]
pub struct HSTPIPIDR0_ISOPIPES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod hstpipidr0_isopipes;
#[doc = "Host Pipe IN Request Register (n = 0)"]
pub struct HSTPIPINRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe IN Request Register (n = 0)"]
pub mod hstpipinrq;
#[doc = "Host Pipe Error Register (n = 0)"]
pub struct HSTPIPERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Error Register (n = 0)"]
pub mod hstpiperr;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub struct HSTDMANXTDSC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod hstdmanxtdsc1;
#[doc = "Host DMA Channel Address Register (n = 1)"]
pub struct HSTDMAADDRESS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Address Register (n = 1)"]
pub mod hstdmaaddress1;
#[doc = "Host DMA Channel Control Register (n = 1)"]
pub struct HSTDMACONTROL1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Control Register (n = 1)"]
pub mod hstdmacontrol1;
#[doc = "Host DMA Channel Status Register (n = 1)"]
pub struct HSTDMASTATUS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Status Register (n = 1)"]
pub mod hstdmastatus1;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 2)"]
pub struct HSTDMANXTDSC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod hstdmanxtdsc2;
#[doc = "Host DMA Channel Address Register (n = 2)"]
pub struct HSTDMAADDRESS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Address Register (n = 2)"]
pub mod hstdmaaddress2;
#[doc = "Host DMA Channel Control Register (n = 2)"]
pub struct HSTDMACONTROL2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Control Register (n = 2)"]
pub mod hstdmacontrol2;
#[doc = "Host DMA Channel Status Register (n = 2)"]
pub struct HSTDMASTATUS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Status Register (n = 2)"]
pub mod hstdmastatus2;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 3)"]
pub struct HSTDMANXTDSC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod hstdmanxtdsc3;
#[doc = "Host DMA Channel Address Register (n = 3)"]
pub struct HSTDMAADDRESS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Address Register (n = 3)"]
pub mod hstdmaaddress3;
#[doc = "Host DMA Channel Control Register (n = 3)"]
pub struct HSTDMACONTROL3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Control Register (n = 3)"]
pub mod hstdmacontrol3;
#[doc = "Host DMA Channel Status Register (n = 3)"]
pub struct HSTDMASTATUS3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Status Register (n = 3)"]
pub mod hstdmastatus3;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 4)"]
pub struct HSTDMANXTDSC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod hstdmanxtdsc4;
#[doc = "Host DMA Channel Address Register (n = 4)"]
pub struct HSTDMAADDRESS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Address Register (n = 4)"]
pub mod hstdmaaddress4;
#[doc = "Host DMA Channel Control Register (n = 4)"]
pub struct HSTDMACONTROL4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Control Register (n = 4)"]
pub mod hstdmacontrol4;
#[doc = "Host DMA Channel Status Register (n = 4)"]
pub struct HSTDMASTATUS4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Status Register (n = 4)"]
pub mod hstdmastatus4;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 5)"]
pub struct HSTDMANXTDSC5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod hstdmanxtdsc5;
#[doc = "Host DMA Channel Address Register (n = 5)"]
pub struct HSTDMAADDRESS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Address Register (n = 5)"]
pub mod hstdmaaddress5;
#[doc = "Host DMA Channel Control Register (n = 5)"]
pub struct HSTDMACONTROL5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Control Register (n = 5)"]
pub mod hstdmacontrol5;
#[doc = "Host DMA Channel Status Register (n = 5)"]
pub struct HSTDMASTATUS5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Status Register (n = 5)"]
pub mod hstdmastatus5;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 6)"]
pub struct HSTDMANXTDSC6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod hstdmanxtdsc6;
#[doc = "Host DMA Channel Address Register (n = 6)"]
pub struct HSTDMAADDRESS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Address Register (n = 6)"]
pub mod hstdmaaddress6;
#[doc = "Host DMA Channel Control Register (n = 6)"]
pub struct HSTDMACONTROL6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Control Register (n = 6)"]
pub mod hstdmacontrol6;
#[doc = "Host DMA Channel Status Register (n = 6)"]
pub struct HSTDMASTATUS6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Status Register (n = 6)"]
pub mod hstdmastatus6;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 7)"]
pub struct HSTDMANXTDSC7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod hstdmanxtdsc7;
#[doc = "Host DMA Channel Address Register (n = 7)"]
pub struct HSTDMAADDRESS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Address Register (n = 7)"]
pub mod hstdmaaddress7;
#[doc = "Host DMA Channel Control Register (n = 7)"]
pub struct HSTDMACONTROL7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Control Register (n = 7)"]
pub mod hstdmacontrol7;
#[doc = "Host DMA Channel Status Register (n = 7)"]
pub struct HSTDMASTATUS7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host DMA Channel Status Register (n = 7)"]
pub mod hstdmastatus7;
#[doc = "General Control Register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "General Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Status Register"]
pub mod sr;
#[doc = "General Status Clear Register"]
pub struct SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "General Status Set Register"]
pub struct SFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Status Set Register"]
pub mod sfr;
#[doc = "General Finite State Machine Register"]
pub struct FSM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Finite State Machine Register"]
pub mod fsm;
