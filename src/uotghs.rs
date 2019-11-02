#[doc = r"Register block"]
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
    _reserved_10_deveptisr: [u8; 40usize],
    _reserved11: [u8; 8usize],
    _reserved_11_devepticr: [u8; 40usize],
    _reserved12: [u8; 8usize],
    _reserved_12_deveptifr: [u8; 40usize],
    _reserved13: [u8; 8usize],
    _reserved_13_deveptimr: [u8; 40usize],
    _reserved14: [u8; 8usize],
    _reserved_14_deveptier: [u8; 40usize],
    _reserved15: [u8; 8usize],
    _reserved_15_deveptidr: [u8; 40usize],
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
    _reserved_56_hstpipcfg: [u8; 40usize],
    _reserved57: [u8; 8usize],
    _reserved_57_hstpipisr: [u8; 40usize],
    _reserved58: [u8; 8usize],
    _reserved_58_hstpipicr: [u8; 40usize],
    _reserved59: [u8; 8usize],
    _reserved_59_hstpipifr: [u8; 40usize],
    _reserved60: [u8; 8usize],
    _reserved_60_hstpipimr: [u8; 40usize],
    _reserved61: [u8; 8usize],
    _reserved_61_hstpipier: [u8; 40usize],
    _reserved62: [u8; 8usize],
    _reserved_62_hstpipidr: [u8; 40usize],
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
impl RegisterBlock {
    #[doc = "0x130 - Device Endpoint Status Register (n = 0)"]
    #[inline(always)]
    pub fn deveptisr0_isoenpt(&self) -> &DEVEPTISR0_ISOENPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(304usize) as *const DEVEPTISR0_ISOENPT)
        }
    }
    #[doc = "0x130 - Device Endpoint Status Register (n = 0)"]
    #[inline(always)]
    pub fn deveptisr0_isoenpt_mut(&self) -> &mut DEVEPTISR0_ISOENPT {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(304usize) as *mut DEVEPTISR0_ISOENPT)
        }
    }
    #[doc = "0x130 - Device Endpoint Status Register (n = 0)"]
    #[inline(always)]
    pub fn deveptisr(&self) -> &[DEVEPTISR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(304usize) as *const [DEVEPTISR; 10]) }
    }
    #[doc = "0x130 - Device Endpoint Status Register (n = 0)"]
    #[inline(always)]
    pub fn deveptisr_mut(&self) -> &mut [DEVEPTISR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(304usize) as *mut [DEVEPTISR; 10]) }
    }
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0)"]
    #[inline(always)]
    pub fn devepticr0_isoenpt(&self) -> &DEVEPTICR0_ISOENPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(352usize) as *const DEVEPTICR0_ISOENPT)
        }
    }
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0)"]
    #[inline(always)]
    pub fn devepticr0_isoenpt_mut(&self) -> &mut DEVEPTICR0_ISOENPT {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(352usize) as *mut DEVEPTICR0_ISOENPT)
        }
    }
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0)"]
    #[inline(always)]
    pub fn devepticr(&self) -> &[DEVEPTICR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(352usize) as *const [DEVEPTICR; 10]) }
    }
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0)"]
    #[inline(always)]
    pub fn devepticr_mut(&self) -> &mut [DEVEPTICR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(352usize) as *mut [DEVEPTICR; 10]) }
    }
    #[doc = "0x190 - Device Endpoint Set Register (n = 0)"]
    #[inline(always)]
    pub fn deveptifr0_isoenpt(&self) -> &DEVEPTIFR0_ISOENPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(400usize) as *const DEVEPTIFR0_ISOENPT)
        }
    }
    #[doc = "0x190 - Device Endpoint Set Register (n = 0)"]
    #[inline(always)]
    pub fn deveptifr0_isoenpt_mut(&self) -> &mut DEVEPTIFR0_ISOENPT {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(400usize) as *mut DEVEPTIFR0_ISOENPT)
        }
    }
    #[doc = "0x190 - Device Endpoint Set Register (n = 0)"]
    #[inline(always)]
    pub fn deveptifr(&self) -> &[DEVEPTIFR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(400usize) as *const [DEVEPTIFR; 10]) }
    }
    #[doc = "0x190 - Device Endpoint Set Register (n = 0)"]
    #[inline(always)]
    pub fn deveptifr_mut(&self) -> &mut [DEVEPTIFR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(400usize) as *mut [DEVEPTIFR; 10]) }
    }
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0)"]
    #[inline(always)]
    pub fn deveptimr0_isoenpt(&self) -> &DEVEPTIMR0_ISOENPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(448usize) as *const DEVEPTIMR0_ISOENPT)
        }
    }
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0)"]
    #[inline(always)]
    pub fn deveptimr0_isoenpt_mut(&self) -> &mut DEVEPTIMR0_ISOENPT {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(448usize) as *mut DEVEPTIMR0_ISOENPT)
        }
    }
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0)"]
    #[inline(always)]
    pub fn deveptimr(&self) -> &[DEVEPTIMR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(448usize) as *const [DEVEPTIMR; 10]) }
    }
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0)"]
    #[inline(always)]
    pub fn deveptimr_mut(&self) -> &mut [DEVEPTIMR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(448usize) as *mut [DEVEPTIMR; 10]) }
    }
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptier0_isoenpt(&self) -> &DEVEPTIER0_ISOENPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(496usize) as *const DEVEPTIER0_ISOENPT)
        }
    }
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptier0_isoenpt_mut(&self) -> &mut DEVEPTIER0_ISOENPT {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(496usize) as *mut DEVEPTIER0_ISOENPT)
        }
    }
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptier(&self) -> &[DEVEPTIER; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(496usize) as *const [DEVEPTIER; 10]) }
    }
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptier_mut(&self) -> &mut [DEVEPTIER; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(496usize) as *mut [DEVEPTIER; 10]) }
    }
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptidr0_isoenpt(&self) -> &DEVEPTIDR0_ISOENPT {
        unsafe {
            &*(((self as *const Self) as *const u8).add(544usize) as *const DEVEPTIDR0_ISOENPT)
        }
    }
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptidr0_isoenpt_mut(&self) -> &mut DEVEPTIDR0_ISOENPT {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(544usize) as *mut DEVEPTIDR0_ISOENPT)
        }
    }
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptidr(&self) -> &[DEVEPTIDR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(544usize) as *const [DEVEPTIDR; 10]) }
    }
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0)"]
    #[inline(always)]
    pub fn deveptidr_mut(&self) -> &mut [DEVEPTIDR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(544usize) as *mut [DEVEPTIDR; 10]) }
    }
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipcfg0_hsbohscp(&self) -> &HSTPIPCFG0_HSBOHSCP {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1280usize) as *const HSTPIPCFG0_HSBOHSCP)
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipcfg0_hsbohscp_mut(&self) -> &mut HSTPIPCFG0_HSBOHSCP {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1280usize) as *mut HSTPIPCFG0_HSBOHSCP)
        }
    }
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipcfg(&self) -> &[HSTPIPCFG; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(1280usize) as *const [HSTPIPCFG; 10]) }
    }
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipcfg_mut(&self) -> &mut [HSTPIPCFG; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1280usize) as *mut [HSTPIPCFG; 10]) }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipisr0_isopipes(&self) -> &HSTPIPISR0_ISOPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize) as *const HSTPIPISR0_ISOPIPES)
        }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipisr0_isopipes_mut(&self) -> &mut HSTPIPISR0_ISOPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize) as *mut HSTPIPISR0_ISOPIPES)
        }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipisr0_intpipes(&self) -> &HSTPIPISR0_INTPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1328usize) as *const HSTPIPISR0_INTPIPES)
        }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipisr0_intpipes_mut(&self) -> &mut HSTPIPISR0_INTPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1328usize) as *mut HSTPIPISR0_INTPIPES)
        }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipisr(&self) -> &[HSTPIPISR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(1328usize) as *const [HSTPIPISR; 10]) }
    }
    #[doc = "0x530 - Host Pipe Status Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipisr_mut(&self) -> &mut [HSTPIPISR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1328usize) as *mut [HSTPIPISR; 10]) }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipicr0_isopipes(&self) -> &HSTPIPICR0_ISOPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize) as *const HSTPIPICR0_ISOPIPES)
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipicr0_isopipes_mut(&self) -> &mut HSTPIPICR0_ISOPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize) as *mut HSTPIPICR0_ISOPIPES)
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipicr0_intpipes(&self) -> &HSTPIPICR0_INTPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1376usize) as *const HSTPIPICR0_INTPIPES)
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipicr0_intpipes_mut(&self) -> &mut HSTPIPICR0_INTPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1376usize) as *mut HSTPIPICR0_INTPIPES)
        }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipicr(&self) -> &[HSTPIPICR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(1376usize) as *const [HSTPIPICR; 10]) }
    }
    #[doc = "0x560 - Host Pipe Clear Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipicr_mut(&self) -> &mut [HSTPIPICR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1376usize) as *mut [HSTPIPICR; 10]) }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipifr0_isopipes(&self) -> &HSTPIPIFR0_ISOPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize) as *const HSTPIPIFR0_ISOPIPES)
        }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipifr0_isopipes_mut(&self) -> &mut HSTPIPIFR0_ISOPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize) as *mut HSTPIPIFR0_ISOPIPES)
        }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipifr0_intpipes(&self) -> &HSTPIPIFR0_INTPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1424usize) as *const HSTPIPIFR0_INTPIPES)
        }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipifr0_intpipes_mut(&self) -> &mut HSTPIPIFR0_INTPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1424usize) as *mut HSTPIPIFR0_INTPIPES)
        }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipifr(&self) -> &[HSTPIPIFR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(1424usize) as *const [HSTPIPIFR; 10]) }
    }
    #[doc = "0x590 - Host Pipe Set Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipifr_mut(&self) -> &mut [HSTPIPIFR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1424usize) as *mut [HSTPIPIFR; 10]) }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipimr0_isopipes(&self) -> &HSTPIPIMR0_ISOPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize) as *const HSTPIPIMR0_ISOPIPES)
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipimr0_isopipes_mut(&self) -> &mut HSTPIPIMR0_ISOPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize) as *mut HSTPIPIMR0_ISOPIPES)
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipimr0_intpipes(&self) -> &HSTPIPIMR0_INTPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1472usize) as *const HSTPIPIMR0_INTPIPES)
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipimr0_intpipes_mut(&self) -> &mut HSTPIPIMR0_INTPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1472usize) as *mut HSTPIPIMR0_INTPIPES)
        }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipimr(&self) -> &[HSTPIPIMR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(1472usize) as *const [HSTPIPIMR; 10]) }
    }
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipimr_mut(&self) -> &mut [HSTPIPIMR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1472usize) as *mut [HSTPIPIMR; 10]) }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipier0_isopipes(&self) -> &HSTPIPIER0_ISOPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize) as *const HSTPIPIER0_ISOPIPES)
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipier0_isopipes_mut(&self) -> &mut HSTPIPIER0_ISOPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize) as *mut HSTPIPIER0_ISOPIPES)
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipier0_intpipes(&self) -> &HSTPIPIER0_INTPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1520usize) as *const HSTPIPIER0_INTPIPES)
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipier0_intpipes_mut(&self) -> &mut HSTPIPIER0_INTPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1520usize) as *mut HSTPIPIER0_INTPIPES)
        }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipier(&self) -> &[HSTPIPIER; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(1520usize) as *const [HSTPIPIER; 10]) }
    }
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipier_mut(&self) -> &mut [HSTPIPIER; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1520usize) as *mut [HSTPIPIER; 10]) }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipidr0_isopipes(&self) -> &HSTPIPIDR0_ISOPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize) as *const HSTPIPIDR0_ISOPIPES)
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipidr0_isopipes_mut(&self) -> &mut HSTPIPIDR0_ISOPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize) as *mut HSTPIPIDR0_ISOPIPES)
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipidr0_intpipes(&self) -> &HSTPIPIDR0_INTPIPES {
        unsafe {
            &*(((self as *const Self) as *const u8).add(1568usize) as *const HSTPIPIDR0_INTPIPES)
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipidr0_intpipes_mut(&self) -> &mut HSTPIPIDR0_INTPIPES {
        unsafe {
            &mut *(((self as *const Self) as *mut u8).add(1568usize) as *mut HSTPIPIDR0_INTPIPES)
        }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipidr(&self) -> &[HSTPIPIDR; 10] {
        unsafe { &*(((self as *const Self) as *const u8).add(1568usize) as *const [HSTPIPIDR; 10]) }
    }
    #[doc = "0x620 - Host Pipe Disable Register (n = 0)"]
    #[inline(always)]
    pub fn hstpipidr_mut(&self) -> &mut [HSTPIPIDR; 10] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1568usize) as *mut [HSTPIPIDR; 10]) }
    }
}
#[doc = "Device General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devctrl](devctrl) module"]
pub type DEVCTRL = crate::Reg<u32, _DEVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVCTRL;
#[doc = "`read()` method returns [devctrl::R](devctrl::R) reader structure"]
impl crate::Readable for DEVCTRL {}
#[doc = "`write(|w| ..)` method takes [devctrl::W](devctrl::W) writer structure"]
impl crate::Writable for DEVCTRL {}
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "Device Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devisr](devisr) module"]
pub type DEVISR = crate::Reg<u32, _DEVISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVISR;
#[doc = "`read()` method returns [devisr::R](devisr::R) reader structure"]
impl crate::Readable for DEVISR {}
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "Device Global Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devicr](devicr) module"]
pub type DEVICR = crate::Reg<u32, _DEVICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICR;
#[doc = "`write(|w| ..)` method takes [devicr::W](devicr::W) writer structure"]
impl crate::Writable for DEVICR {}
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "Device Global Interrupt Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devifr](devifr) module"]
pub type DEVIFR = crate::Reg<u32, _DEVIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVIFR;
#[doc = "`write(|w| ..)` method takes [devifr::W](devifr::W) writer structure"]
impl crate::Writable for DEVIFR {}
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "Device Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devimr](devimr) module"]
pub type DEVIMR = crate::Reg<u32, _DEVIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVIMR;
#[doc = "`read()` method returns [devimr::R](devimr::R) reader structure"]
impl crate::Readable for DEVIMR {}
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "Device Global Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devidr](devidr) module"]
pub type DEVIDR = crate::Reg<u32, _DEVIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVIDR;
#[doc = "`write(|w| ..)` method takes [devidr::W](devidr::W) writer structure"]
impl crate::Writable for DEVIDR {}
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "Device Global Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devier](devier) module"]
pub type DEVIER = crate::Reg<u32, _DEVIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVIER;
#[doc = "`write(|w| ..)` method takes [devier::W](devier::W) writer structure"]
impl crate::Writable for DEVIER {}
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "Device Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devept](devept) module"]
pub type DEVEPT = crate::Reg<u32, _DEVEPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPT;
#[doc = "`read()` method returns [devept::R](devept::R) reader structure"]
impl crate::Readable for DEVEPT {}
#[doc = "`write(|w| ..)` method takes [devept::W](devept::W) writer structure"]
impl crate::Writable for DEVEPT {}
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "Device Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devfnum](devfnum) module"]
pub type DEVFNUM = crate::Reg<u32, _DEVFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVFNUM;
#[doc = "`read()` method returns [devfnum::R](devfnum::R) reader structure"]
impl crate::Readable for DEVFNUM {}
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "Device Endpoint Configuration Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptcfg](deveptcfg) module"]
pub type DEVEPTCFG = crate::Reg<u32, _DEVEPTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTCFG;
#[doc = "`read()` method returns [deveptcfg::R](deveptcfg::R) reader structure"]
impl crate::Readable for DEVEPTCFG {}
#[doc = "`write(|w| ..)` method takes [deveptcfg::W](deveptcfg::W) writer structure"]
impl crate::Writable for DEVEPTCFG {}
#[doc = "Device Endpoint Configuration Register (n = 0)"]
pub mod deveptcfg;
#[doc = "Device Endpoint Status Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptisr](deveptisr) module"]
pub type DEVEPTISR = crate::Reg<u32, _DEVEPTISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTISR;
#[doc = "`read()` method returns [deveptisr::R](deveptisr::R) reader structure"]
impl crate::Readable for DEVEPTISR {}
#[doc = "Device Endpoint Status Register (n = 0)"]
pub mod deveptisr;
#[doc = "Device Endpoint Status Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptisr0_isoenpt](deveptisr0_isoenpt) module"]
pub type DEVEPTISR0_ISOENPT = crate::Reg<u32, _DEVEPTISR0_ISOENPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTISR0_ISOENPT;
#[doc = "`read()` method returns [deveptisr0_isoenpt::R](deveptisr0_isoenpt::R) reader structure"]
impl crate::Readable for DEVEPTISR0_ISOENPT {}
#[doc = "Device Endpoint Status Register (n = 0)"]
pub mod deveptisr0_isoenpt;
#[doc = "Device Endpoint Clear Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devepticr](devepticr) module"]
pub type DEVEPTICR = crate::Reg<u32, _DEVEPTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTICR;
#[doc = "`write(|w| ..)` method takes [devepticr::W](devepticr::W) writer structure"]
impl crate::Writable for DEVEPTICR {}
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub mod devepticr;
#[doc = "Device Endpoint Clear Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devepticr0_isoenpt](devepticr0_isoenpt) module"]
pub type DEVEPTICR0_ISOENPT = crate::Reg<u32, _DEVEPTICR0_ISOENPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTICR0_ISOENPT;
#[doc = "`write(|w| ..)` method takes [devepticr0_isoenpt::W](devepticr0_isoenpt::W) writer structure"]
impl crate::Writable for DEVEPTICR0_ISOENPT {}
#[doc = "Device Endpoint Clear Register (n = 0)"]
pub mod devepticr0_isoenpt;
#[doc = "Device Endpoint Set Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptifr](deveptifr) module"]
pub type DEVEPTIFR = crate::Reg<u32, _DEVEPTIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIFR;
#[doc = "`write(|w| ..)` method takes [deveptifr::W](deveptifr::W) writer structure"]
impl crate::Writable for DEVEPTIFR {}
#[doc = "Device Endpoint Set Register (n = 0)"]
pub mod deveptifr;
#[doc = "Device Endpoint Set Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptifr0_isoenpt](deveptifr0_isoenpt) module"]
pub type DEVEPTIFR0_ISOENPT = crate::Reg<u32, _DEVEPTIFR0_ISOENPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIFR0_ISOENPT;
#[doc = "`write(|w| ..)` method takes [deveptifr0_isoenpt::W](deveptifr0_isoenpt::W) writer structure"]
impl crate::Writable for DEVEPTIFR0_ISOENPT {}
#[doc = "Device Endpoint Set Register (n = 0)"]
pub mod deveptifr0_isoenpt;
#[doc = "Device Endpoint Mask Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptimr](deveptimr) module"]
pub type DEVEPTIMR = crate::Reg<u32, _DEVEPTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIMR;
#[doc = "`read()` method returns [deveptimr::R](deveptimr::R) reader structure"]
impl crate::Readable for DEVEPTIMR {}
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub mod deveptimr;
#[doc = "Device Endpoint Mask Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptimr0_isoenpt](deveptimr0_isoenpt) module"]
pub type DEVEPTIMR0_ISOENPT = crate::Reg<u32, _DEVEPTIMR0_ISOENPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIMR0_ISOENPT;
#[doc = "`read()` method returns [deveptimr0_isoenpt::R](deveptimr0_isoenpt::R) reader structure"]
impl crate::Readable for DEVEPTIMR0_ISOENPT {}
#[doc = "Device Endpoint Mask Register (n = 0)"]
pub mod deveptimr0_isoenpt;
#[doc = "Device Endpoint Enable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptier](deveptier) module"]
pub type DEVEPTIER = crate::Reg<u32, _DEVEPTIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIER;
#[doc = "`write(|w| ..)` method takes [deveptier::W](deveptier::W) writer structure"]
impl crate::Writable for DEVEPTIER {}
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub mod deveptier;
#[doc = "Device Endpoint Enable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptier0_isoenpt](deveptier0_isoenpt) module"]
pub type DEVEPTIER0_ISOENPT = crate::Reg<u32, _DEVEPTIER0_ISOENPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIER0_ISOENPT;
#[doc = "`write(|w| ..)` method takes [deveptier0_isoenpt::W](deveptier0_isoenpt::W) writer structure"]
impl crate::Writable for DEVEPTIER0_ISOENPT {}
#[doc = "Device Endpoint Enable Register (n = 0)"]
pub mod deveptier0_isoenpt;
#[doc = "Device Endpoint Disable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptidr](deveptidr) module"]
pub type DEVEPTIDR = crate::Reg<u32, _DEVEPTIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIDR;
#[doc = "`write(|w| ..)` method takes [deveptidr::W](deveptidr::W) writer structure"]
impl crate::Writable for DEVEPTIDR {}
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub mod deveptidr;
#[doc = "Device Endpoint Disable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deveptidr0_isoenpt](deveptidr0_isoenpt) module"]
pub type DEVEPTIDR0_ISOENPT = crate::Reg<u32, _DEVEPTIDR0_ISOENPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVEPTIDR0_ISOENPT;
#[doc = "`write(|w| ..)` method takes [deveptidr0_isoenpt::W](deveptidr0_isoenpt::W) writer structure"]
impl crate::Writable for DEVEPTIDR0_ISOENPT {}
#[doc = "Device Endpoint Disable Register (n = 0)"]
pub mod deveptidr0_isoenpt;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmanxtdsc1](devdmanxtdsc1) module"]
pub type DEVDMANXTDSC1 = crate::Reg<u32, _DEVDMANXTDSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMANXTDSC1;
#[doc = "`read()` method returns [devdmanxtdsc1::R](devdmanxtdsc1::R) reader structure"]
impl crate::Readable for DEVDMANXTDSC1 {}
#[doc = "`write(|w| ..)` method takes [devdmanxtdsc1::W](devdmanxtdsc1::W) writer structure"]
impl crate::Writable for DEVDMANXTDSC1 {}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod devdmanxtdsc1;
#[doc = "Device DMA Channel Address Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmaaddress1](devdmaaddress1) module"]
pub type DEVDMAADDRESS1 = crate::Reg<u32, _DEVDMAADDRESS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMAADDRESS1;
#[doc = "`read()` method returns [devdmaaddress1::R](devdmaaddress1::R) reader structure"]
impl crate::Readable for DEVDMAADDRESS1 {}
#[doc = "`write(|w| ..)` method takes [devdmaaddress1::W](devdmaaddress1::W) writer structure"]
impl crate::Writable for DEVDMAADDRESS1 {}
#[doc = "Device DMA Channel Address Register (n = 1)"]
pub mod devdmaaddress1;
#[doc = "Device DMA Channel Control Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmacontrol1](devdmacontrol1) module"]
pub type DEVDMACONTROL1 = crate::Reg<u32, _DEVDMACONTROL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMACONTROL1;
#[doc = "`read()` method returns [devdmacontrol1::R](devdmacontrol1::R) reader structure"]
impl crate::Readable for DEVDMACONTROL1 {}
#[doc = "`write(|w| ..)` method takes [devdmacontrol1::W](devdmacontrol1::W) writer structure"]
impl crate::Writable for DEVDMACONTROL1 {}
#[doc = "Device DMA Channel Control Register (n = 1)"]
pub mod devdmacontrol1;
#[doc = "Device DMA Channel Status Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmastatus1](devdmastatus1) module"]
pub type DEVDMASTATUS1 = crate::Reg<u32, _DEVDMASTATUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMASTATUS1;
#[doc = "`read()` method returns [devdmastatus1::R](devdmastatus1::R) reader structure"]
impl crate::Readable for DEVDMASTATUS1 {}
#[doc = "`write(|w| ..)` method takes [devdmastatus1::W](devdmastatus1::W) writer structure"]
impl crate::Writable for DEVDMASTATUS1 {}
#[doc = "Device DMA Channel Status Register (n = 1)"]
pub mod devdmastatus1;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmanxtdsc2](devdmanxtdsc2) module"]
pub type DEVDMANXTDSC2 = crate::Reg<u32, _DEVDMANXTDSC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMANXTDSC2;
#[doc = "`read()` method returns [devdmanxtdsc2::R](devdmanxtdsc2::R) reader structure"]
impl crate::Readable for DEVDMANXTDSC2 {}
#[doc = "`write(|w| ..)` method takes [devdmanxtdsc2::W](devdmanxtdsc2::W) writer structure"]
impl crate::Writable for DEVDMANXTDSC2 {}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod devdmanxtdsc2;
#[doc = "Device DMA Channel Address Register (n = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmaaddress2](devdmaaddress2) module"]
pub type DEVDMAADDRESS2 = crate::Reg<u32, _DEVDMAADDRESS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMAADDRESS2;
#[doc = "`read()` method returns [devdmaaddress2::R](devdmaaddress2::R) reader structure"]
impl crate::Readable for DEVDMAADDRESS2 {}
#[doc = "`write(|w| ..)` method takes [devdmaaddress2::W](devdmaaddress2::W) writer structure"]
impl crate::Writable for DEVDMAADDRESS2 {}
#[doc = "Device DMA Channel Address Register (n = 2)"]
pub mod devdmaaddress2;
#[doc = "Device DMA Channel Control Register (n = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmacontrol2](devdmacontrol2) module"]
pub type DEVDMACONTROL2 = crate::Reg<u32, _DEVDMACONTROL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMACONTROL2;
#[doc = "`read()` method returns [devdmacontrol2::R](devdmacontrol2::R) reader structure"]
impl crate::Readable for DEVDMACONTROL2 {}
#[doc = "`write(|w| ..)` method takes [devdmacontrol2::W](devdmacontrol2::W) writer structure"]
impl crate::Writable for DEVDMACONTROL2 {}
#[doc = "Device DMA Channel Control Register (n = 2)"]
pub mod devdmacontrol2;
#[doc = "Device DMA Channel Status Register (n = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmastatus2](devdmastatus2) module"]
pub type DEVDMASTATUS2 = crate::Reg<u32, _DEVDMASTATUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMASTATUS2;
#[doc = "`read()` method returns [devdmastatus2::R](devdmastatus2::R) reader structure"]
impl crate::Readable for DEVDMASTATUS2 {}
#[doc = "`write(|w| ..)` method takes [devdmastatus2::W](devdmastatus2::W) writer structure"]
impl crate::Writable for DEVDMASTATUS2 {}
#[doc = "Device DMA Channel Status Register (n = 2)"]
pub mod devdmastatus2;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmanxtdsc3](devdmanxtdsc3) module"]
pub type DEVDMANXTDSC3 = crate::Reg<u32, _DEVDMANXTDSC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMANXTDSC3;
#[doc = "`read()` method returns [devdmanxtdsc3::R](devdmanxtdsc3::R) reader structure"]
impl crate::Readable for DEVDMANXTDSC3 {}
#[doc = "`write(|w| ..)` method takes [devdmanxtdsc3::W](devdmanxtdsc3::W) writer structure"]
impl crate::Writable for DEVDMANXTDSC3 {}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod devdmanxtdsc3;
#[doc = "Device DMA Channel Address Register (n = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmaaddress3](devdmaaddress3) module"]
pub type DEVDMAADDRESS3 = crate::Reg<u32, _DEVDMAADDRESS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMAADDRESS3;
#[doc = "`read()` method returns [devdmaaddress3::R](devdmaaddress3::R) reader structure"]
impl crate::Readable for DEVDMAADDRESS3 {}
#[doc = "`write(|w| ..)` method takes [devdmaaddress3::W](devdmaaddress3::W) writer structure"]
impl crate::Writable for DEVDMAADDRESS3 {}
#[doc = "Device DMA Channel Address Register (n = 3)"]
pub mod devdmaaddress3;
#[doc = "Device DMA Channel Control Register (n = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmacontrol3](devdmacontrol3) module"]
pub type DEVDMACONTROL3 = crate::Reg<u32, _DEVDMACONTROL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMACONTROL3;
#[doc = "`read()` method returns [devdmacontrol3::R](devdmacontrol3::R) reader structure"]
impl crate::Readable for DEVDMACONTROL3 {}
#[doc = "`write(|w| ..)` method takes [devdmacontrol3::W](devdmacontrol3::W) writer structure"]
impl crate::Writable for DEVDMACONTROL3 {}
#[doc = "Device DMA Channel Control Register (n = 3)"]
pub mod devdmacontrol3;
#[doc = "Device DMA Channel Status Register (n = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmastatus3](devdmastatus3) module"]
pub type DEVDMASTATUS3 = crate::Reg<u32, _DEVDMASTATUS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMASTATUS3;
#[doc = "`read()` method returns [devdmastatus3::R](devdmastatus3::R) reader structure"]
impl crate::Readable for DEVDMASTATUS3 {}
#[doc = "`write(|w| ..)` method takes [devdmastatus3::W](devdmastatus3::W) writer structure"]
impl crate::Writable for DEVDMASTATUS3 {}
#[doc = "Device DMA Channel Status Register (n = 3)"]
pub mod devdmastatus3;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmanxtdsc4](devdmanxtdsc4) module"]
pub type DEVDMANXTDSC4 = crate::Reg<u32, _DEVDMANXTDSC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMANXTDSC4;
#[doc = "`read()` method returns [devdmanxtdsc4::R](devdmanxtdsc4::R) reader structure"]
impl crate::Readable for DEVDMANXTDSC4 {}
#[doc = "`write(|w| ..)` method takes [devdmanxtdsc4::W](devdmanxtdsc4::W) writer structure"]
impl crate::Writable for DEVDMANXTDSC4 {}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod devdmanxtdsc4;
#[doc = "Device DMA Channel Address Register (n = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmaaddress4](devdmaaddress4) module"]
pub type DEVDMAADDRESS4 = crate::Reg<u32, _DEVDMAADDRESS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMAADDRESS4;
#[doc = "`read()` method returns [devdmaaddress4::R](devdmaaddress4::R) reader structure"]
impl crate::Readable for DEVDMAADDRESS4 {}
#[doc = "`write(|w| ..)` method takes [devdmaaddress4::W](devdmaaddress4::W) writer structure"]
impl crate::Writable for DEVDMAADDRESS4 {}
#[doc = "Device DMA Channel Address Register (n = 4)"]
pub mod devdmaaddress4;
#[doc = "Device DMA Channel Control Register (n = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmacontrol4](devdmacontrol4) module"]
pub type DEVDMACONTROL4 = crate::Reg<u32, _DEVDMACONTROL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMACONTROL4;
#[doc = "`read()` method returns [devdmacontrol4::R](devdmacontrol4::R) reader structure"]
impl crate::Readable for DEVDMACONTROL4 {}
#[doc = "`write(|w| ..)` method takes [devdmacontrol4::W](devdmacontrol4::W) writer structure"]
impl crate::Writable for DEVDMACONTROL4 {}
#[doc = "Device DMA Channel Control Register (n = 4)"]
pub mod devdmacontrol4;
#[doc = "Device DMA Channel Status Register (n = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmastatus4](devdmastatus4) module"]
pub type DEVDMASTATUS4 = crate::Reg<u32, _DEVDMASTATUS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMASTATUS4;
#[doc = "`read()` method returns [devdmastatus4::R](devdmastatus4::R) reader structure"]
impl crate::Readable for DEVDMASTATUS4 {}
#[doc = "`write(|w| ..)` method takes [devdmastatus4::W](devdmastatus4::W) writer structure"]
impl crate::Writable for DEVDMASTATUS4 {}
#[doc = "Device DMA Channel Status Register (n = 4)"]
pub mod devdmastatus4;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmanxtdsc5](devdmanxtdsc5) module"]
pub type DEVDMANXTDSC5 = crate::Reg<u32, _DEVDMANXTDSC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMANXTDSC5;
#[doc = "`read()` method returns [devdmanxtdsc5::R](devdmanxtdsc5::R) reader structure"]
impl crate::Readable for DEVDMANXTDSC5 {}
#[doc = "`write(|w| ..)` method takes [devdmanxtdsc5::W](devdmanxtdsc5::W) writer structure"]
impl crate::Writable for DEVDMANXTDSC5 {}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod devdmanxtdsc5;
#[doc = "Device DMA Channel Address Register (n = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmaaddress5](devdmaaddress5) module"]
pub type DEVDMAADDRESS5 = crate::Reg<u32, _DEVDMAADDRESS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMAADDRESS5;
#[doc = "`read()` method returns [devdmaaddress5::R](devdmaaddress5::R) reader structure"]
impl crate::Readable for DEVDMAADDRESS5 {}
#[doc = "`write(|w| ..)` method takes [devdmaaddress5::W](devdmaaddress5::W) writer structure"]
impl crate::Writable for DEVDMAADDRESS5 {}
#[doc = "Device DMA Channel Address Register (n = 5)"]
pub mod devdmaaddress5;
#[doc = "Device DMA Channel Control Register (n = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmacontrol5](devdmacontrol5) module"]
pub type DEVDMACONTROL5 = crate::Reg<u32, _DEVDMACONTROL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMACONTROL5;
#[doc = "`read()` method returns [devdmacontrol5::R](devdmacontrol5::R) reader structure"]
impl crate::Readable for DEVDMACONTROL5 {}
#[doc = "`write(|w| ..)` method takes [devdmacontrol5::W](devdmacontrol5::W) writer structure"]
impl crate::Writable for DEVDMACONTROL5 {}
#[doc = "Device DMA Channel Control Register (n = 5)"]
pub mod devdmacontrol5;
#[doc = "Device DMA Channel Status Register (n = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmastatus5](devdmastatus5) module"]
pub type DEVDMASTATUS5 = crate::Reg<u32, _DEVDMASTATUS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMASTATUS5;
#[doc = "`read()` method returns [devdmastatus5::R](devdmastatus5::R) reader structure"]
impl crate::Readable for DEVDMASTATUS5 {}
#[doc = "`write(|w| ..)` method takes [devdmastatus5::W](devdmastatus5::W) writer structure"]
impl crate::Writable for DEVDMASTATUS5 {}
#[doc = "Device DMA Channel Status Register (n = 5)"]
pub mod devdmastatus5;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmanxtdsc6](devdmanxtdsc6) module"]
pub type DEVDMANXTDSC6 = crate::Reg<u32, _DEVDMANXTDSC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMANXTDSC6;
#[doc = "`read()` method returns [devdmanxtdsc6::R](devdmanxtdsc6::R) reader structure"]
impl crate::Readable for DEVDMANXTDSC6 {}
#[doc = "`write(|w| ..)` method takes [devdmanxtdsc6::W](devdmanxtdsc6::W) writer structure"]
impl crate::Writable for DEVDMANXTDSC6 {}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod devdmanxtdsc6;
#[doc = "Device DMA Channel Address Register (n = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmaaddress6](devdmaaddress6) module"]
pub type DEVDMAADDRESS6 = crate::Reg<u32, _DEVDMAADDRESS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMAADDRESS6;
#[doc = "`read()` method returns [devdmaaddress6::R](devdmaaddress6::R) reader structure"]
impl crate::Readable for DEVDMAADDRESS6 {}
#[doc = "`write(|w| ..)` method takes [devdmaaddress6::W](devdmaaddress6::W) writer structure"]
impl crate::Writable for DEVDMAADDRESS6 {}
#[doc = "Device DMA Channel Address Register (n = 6)"]
pub mod devdmaaddress6;
#[doc = "Device DMA Channel Control Register (n = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmacontrol6](devdmacontrol6) module"]
pub type DEVDMACONTROL6 = crate::Reg<u32, _DEVDMACONTROL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMACONTROL6;
#[doc = "`read()` method returns [devdmacontrol6::R](devdmacontrol6::R) reader structure"]
impl crate::Readable for DEVDMACONTROL6 {}
#[doc = "`write(|w| ..)` method takes [devdmacontrol6::W](devdmacontrol6::W) writer structure"]
impl crate::Writable for DEVDMACONTROL6 {}
#[doc = "Device DMA Channel Control Register (n = 6)"]
pub mod devdmacontrol6;
#[doc = "Device DMA Channel Status Register (n = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmastatus6](devdmastatus6) module"]
pub type DEVDMASTATUS6 = crate::Reg<u32, _DEVDMASTATUS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMASTATUS6;
#[doc = "`read()` method returns [devdmastatus6::R](devdmastatus6::R) reader structure"]
impl crate::Readable for DEVDMASTATUS6 {}
#[doc = "`write(|w| ..)` method takes [devdmastatus6::W](devdmastatus6::W) writer structure"]
impl crate::Writable for DEVDMASTATUS6 {}
#[doc = "Device DMA Channel Status Register (n = 6)"]
pub mod devdmastatus6;
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmanxtdsc7](devdmanxtdsc7) module"]
pub type DEVDMANXTDSC7 = crate::Reg<u32, _DEVDMANXTDSC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMANXTDSC7;
#[doc = "`read()` method returns [devdmanxtdsc7::R](devdmanxtdsc7::R) reader structure"]
impl crate::Readable for DEVDMANXTDSC7 {}
#[doc = "`write(|w| ..)` method takes [devdmanxtdsc7::W](devdmanxtdsc7::W) writer structure"]
impl crate::Writable for DEVDMANXTDSC7 {}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod devdmanxtdsc7;
#[doc = "Device DMA Channel Address Register (n = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmaaddress7](devdmaaddress7) module"]
pub type DEVDMAADDRESS7 = crate::Reg<u32, _DEVDMAADDRESS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMAADDRESS7;
#[doc = "`read()` method returns [devdmaaddress7::R](devdmaaddress7::R) reader structure"]
impl crate::Readable for DEVDMAADDRESS7 {}
#[doc = "`write(|w| ..)` method takes [devdmaaddress7::W](devdmaaddress7::W) writer structure"]
impl crate::Writable for DEVDMAADDRESS7 {}
#[doc = "Device DMA Channel Address Register (n = 7)"]
pub mod devdmaaddress7;
#[doc = "Device DMA Channel Control Register (n = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmacontrol7](devdmacontrol7) module"]
pub type DEVDMACONTROL7 = crate::Reg<u32, _DEVDMACONTROL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMACONTROL7;
#[doc = "`read()` method returns [devdmacontrol7::R](devdmacontrol7::R) reader structure"]
impl crate::Readable for DEVDMACONTROL7 {}
#[doc = "`write(|w| ..)` method takes [devdmacontrol7::W](devdmacontrol7::W) writer structure"]
impl crate::Writable for DEVDMACONTROL7 {}
#[doc = "Device DMA Channel Control Register (n = 7)"]
pub mod devdmacontrol7;
#[doc = "Device DMA Channel Status Register (n = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [devdmastatus7](devdmastatus7) module"]
pub type DEVDMASTATUS7 = crate::Reg<u32, _DEVDMASTATUS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVDMASTATUS7;
#[doc = "`read()` method returns [devdmastatus7::R](devdmastatus7::R) reader structure"]
impl crate::Readable for DEVDMASTATUS7 {}
#[doc = "`write(|w| ..)` method takes [devdmastatus7::W](devdmastatus7::W) writer structure"]
impl crate::Writable for DEVDMASTATUS7 {}
#[doc = "Device DMA Channel Status Register (n = 7)"]
pub mod devdmastatus7;
#[doc = "Host General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstctrl](hstctrl) module"]
pub type HSTCTRL = crate::Reg<u32, _HSTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTCTRL;
#[doc = "`read()` method returns [hstctrl::R](hstctrl::R) reader structure"]
impl crate::Readable for HSTCTRL {}
#[doc = "`write(|w| ..)` method takes [hstctrl::W](hstctrl::W) writer structure"]
impl crate::Writable for HSTCTRL {}
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "Host Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstisr](hstisr) module"]
pub type HSTISR = crate::Reg<u32, _HSTISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTISR;
#[doc = "`read()` method returns [hstisr::R](hstisr::R) reader structure"]
impl crate::Readable for HSTISR {}
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "Host Global Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hsticr](hsticr) module"]
pub type HSTICR = crate::Reg<u32, _HSTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTICR;
#[doc = "`write(|w| ..)` method takes [hsticr::W](hsticr::W) writer structure"]
impl crate::Writable for HSTICR {}
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "Host Global Interrupt Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstifr](hstifr) module"]
pub type HSTIFR = crate::Reg<u32, _HSTIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIFR;
#[doc = "`write(|w| ..)` method takes [hstifr::W](hstifr::W) writer structure"]
impl crate::Writable for HSTIFR {}
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "Host Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstimr](hstimr) module"]
pub type HSTIMR = crate::Reg<u32, _HSTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIMR;
#[doc = "`read()` method returns [hstimr::R](hstimr::R) reader structure"]
impl crate::Readable for HSTIMR {}
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "Host Global Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstidr](hstidr) module"]
pub type HSTIDR = crate::Reg<u32, _HSTIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIDR;
#[doc = "`write(|w| ..)` method takes [hstidr::W](hstidr::W) writer structure"]
impl crate::Writable for HSTIDR {}
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "Host Global Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstier](hstier) module"]
pub type HSTIER = crate::Reg<u32, _HSTIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTIER;
#[doc = "`write(|w| ..)` method takes [hstier::W](hstier::W) writer structure"]
impl crate::Writable for HSTIER {}
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "Host Pipe Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpip](hstpip) module"]
pub type HSTPIP = crate::Reg<u32, _HSTPIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIP;
#[doc = "`read()` method returns [hstpip::R](hstpip::R) reader structure"]
impl crate::Readable for HSTPIP {}
#[doc = "`write(|w| ..)` method takes [hstpip::W](hstpip::W) writer structure"]
impl crate::Writable for HSTPIP {}
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "Host Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstfnum](hstfnum) module"]
pub type HSTFNUM = crate::Reg<u32, _HSTFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTFNUM;
#[doc = "`read()` method returns [hstfnum::R](hstfnum::R) reader structure"]
impl crate::Readable for HSTFNUM {}
#[doc = "`write(|w| ..)` method takes [hstfnum::W](hstfnum::W) writer structure"]
impl crate::Writable for HSTFNUM {}
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "Host Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstaddr1](hstaddr1) module"]
pub type HSTADDR1 = crate::Reg<u32, _HSTADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTADDR1;
#[doc = "`read()` method returns [hstaddr1::R](hstaddr1::R) reader structure"]
impl crate::Readable for HSTADDR1 {}
#[doc = "`write(|w| ..)` method takes [hstaddr1::W](hstaddr1::W) writer structure"]
impl crate::Writable for HSTADDR1 {}
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "Host Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstaddr2](hstaddr2) module"]
pub type HSTADDR2 = crate::Reg<u32, _HSTADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTADDR2;
#[doc = "`read()` method returns [hstaddr2::R](hstaddr2::R) reader structure"]
impl crate::Readable for HSTADDR2 {}
#[doc = "`write(|w| ..)` method takes [hstaddr2::W](hstaddr2::W) writer structure"]
impl crate::Writable for HSTADDR2 {}
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "Host Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstaddr3](hstaddr3) module"]
pub type HSTADDR3 = crate::Reg<u32, _HSTADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTADDR3;
#[doc = "`read()` method returns [hstaddr3::R](hstaddr3::R) reader structure"]
impl crate::Readable for HSTADDR3 {}
#[doc = "`write(|w| ..)` method takes [hstaddr3::W](hstaddr3::W) writer structure"]
impl crate::Writable for HSTADDR3 {}
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "Host Pipe Configuration Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipcfg](hstpipcfg) module"]
pub type HSTPIPCFG = crate::Reg<u32, _HSTPIPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPCFG;
#[doc = "`read()` method returns [hstpipcfg::R](hstpipcfg::R) reader structure"]
impl crate::Readable for HSTPIPCFG {}
#[doc = "`write(|w| ..)` method takes [hstpipcfg::W](hstpipcfg::W) writer structure"]
impl crate::Writable for HSTPIPCFG {}
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub mod hstpipcfg;
#[doc = "Host Pipe Configuration Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipcfg0_hsbohscp](hstpipcfg0_hsbohscp) module"]
pub type HSTPIPCFG0_HSBOHSCP = crate::Reg<u32, _HSTPIPCFG0_HSBOHSCP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPCFG0_HSBOHSCP;
#[doc = "`read()` method returns [hstpipcfg0_hsbohscp::R](hstpipcfg0_hsbohscp::R) reader structure"]
impl crate::Readable for HSTPIPCFG0_HSBOHSCP {}
#[doc = "`write(|w| ..)` method takes [hstpipcfg0_hsbohscp::W](hstpipcfg0_hsbohscp::W) writer structure"]
impl crate::Writable for HSTPIPCFG0_HSBOHSCP {}
#[doc = "Host Pipe Configuration Register (n = 0)"]
pub mod hstpipcfg0_hsbohscp;
#[doc = "Host Pipe Status Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipisr](hstpipisr) module"]
pub type HSTPIPISR = crate::Reg<u32, _HSTPIPISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPISR;
#[doc = "`read()` method returns [hstpipisr::R](hstpipisr::R) reader structure"]
impl crate::Readable for HSTPIPISR {}
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod hstpipisr;
#[doc = "Host Pipe Status Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipisr0_intpipes](hstpipisr0_intpipes) module"]
pub type HSTPIPISR0_INTPIPES = crate::Reg<u32, _HSTPIPISR0_INTPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPISR0_INTPIPES;
#[doc = "`read()` method returns [hstpipisr0_intpipes::R](hstpipisr0_intpipes::R) reader structure"]
impl crate::Readable for HSTPIPISR0_INTPIPES {}
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod hstpipisr0_intpipes;
#[doc = "Host Pipe Status Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipisr0_isopipes](hstpipisr0_isopipes) module"]
pub type HSTPIPISR0_ISOPIPES = crate::Reg<u32, _HSTPIPISR0_ISOPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPISR0_ISOPIPES;
#[doc = "`read()` method returns [hstpipisr0_isopipes::R](hstpipisr0_isopipes::R) reader structure"]
impl crate::Readable for HSTPIPISR0_ISOPIPES {}
#[doc = "Host Pipe Status Register (n = 0)"]
pub mod hstpipisr0_isopipes;
#[doc = "Host Pipe Clear Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipicr](hstpipicr) module"]
pub type HSTPIPICR = crate::Reg<u32, _HSTPIPICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPICR;
#[doc = "`write(|w| ..)` method takes [hstpipicr::W](hstpipicr::W) writer structure"]
impl crate::Writable for HSTPIPICR {}
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod hstpipicr;
#[doc = "Host Pipe Clear Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipicr0_intpipes](hstpipicr0_intpipes) module"]
pub type HSTPIPICR0_INTPIPES = crate::Reg<u32, _HSTPIPICR0_INTPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPICR0_INTPIPES;
#[doc = "`write(|w| ..)` method takes [hstpipicr0_intpipes::W](hstpipicr0_intpipes::W) writer structure"]
impl crate::Writable for HSTPIPICR0_INTPIPES {}
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod hstpipicr0_intpipes;
#[doc = "Host Pipe Clear Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipicr0_isopipes](hstpipicr0_isopipes) module"]
pub type HSTPIPICR0_ISOPIPES = crate::Reg<u32, _HSTPIPICR0_ISOPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPICR0_ISOPIPES;
#[doc = "`write(|w| ..)` method takes [hstpipicr0_isopipes::W](hstpipicr0_isopipes::W) writer structure"]
impl crate::Writable for HSTPIPICR0_ISOPIPES {}
#[doc = "Host Pipe Clear Register (n = 0)"]
pub mod hstpipicr0_isopipes;
#[doc = "Host Pipe Set Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipifr](hstpipifr) module"]
pub type HSTPIPIFR = crate::Reg<u32, _HSTPIPIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIFR;
#[doc = "`write(|w| ..)` method takes [hstpipifr::W](hstpipifr::W) writer structure"]
impl crate::Writable for HSTPIPIFR {}
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod hstpipifr;
#[doc = "Host Pipe Set Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipifr0_intpipes](hstpipifr0_intpipes) module"]
pub type HSTPIPIFR0_INTPIPES = crate::Reg<u32, _HSTPIPIFR0_INTPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIFR0_INTPIPES;
#[doc = "`write(|w| ..)` method takes [hstpipifr0_intpipes::W](hstpipifr0_intpipes::W) writer structure"]
impl crate::Writable for HSTPIPIFR0_INTPIPES {}
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod hstpipifr0_intpipes;
#[doc = "Host Pipe Set Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipifr0_isopipes](hstpipifr0_isopipes) module"]
pub type HSTPIPIFR0_ISOPIPES = crate::Reg<u32, _HSTPIPIFR0_ISOPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIFR0_ISOPIPES;
#[doc = "`write(|w| ..)` method takes [hstpipifr0_isopipes::W](hstpipifr0_isopipes::W) writer structure"]
impl crate::Writable for HSTPIPIFR0_ISOPIPES {}
#[doc = "Host Pipe Set Register (n = 0)"]
pub mod hstpipifr0_isopipes;
#[doc = "Host Pipe Mask Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipimr](hstpipimr) module"]
pub type HSTPIPIMR = crate::Reg<u32, _HSTPIPIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIMR;
#[doc = "`read()` method returns [hstpipimr::R](hstpipimr::R) reader structure"]
impl crate::Readable for HSTPIPIMR {}
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod hstpipimr;
#[doc = "Host Pipe Mask Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipimr0_intpipes](hstpipimr0_intpipes) module"]
pub type HSTPIPIMR0_INTPIPES = crate::Reg<u32, _HSTPIPIMR0_INTPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIMR0_INTPIPES;
#[doc = "`read()` method returns [hstpipimr0_intpipes::R](hstpipimr0_intpipes::R) reader structure"]
impl crate::Readable for HSTPIPIMR0_INTPIPES {}
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod hstpipimr0_intpipes;
#[doc = "Host Pipe Mask Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipimr0_isopipes](hstpipimr0_isopipes) module"]
pub type HSTPIPIMR0_ISOPIPES = crate::Reg<u32, _HSTPIPIMR0_ISOPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIMR0_ISOPIPES;
#[doc = "`read()` method returns [hstpipimr0_isopipes::R](hstpipimr0_isopipes::R) reader structure"]
impl crate::Readable for HSTPIPIMR0_ISOPIPES {}
#[doc = "Host Pipe Mask Register (n = 0)"]
pub mod hstpipimr0_isopipes;
#[doc = "Host Pipe Enable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipier](hstpipier) module"]
pub type HSTPIPIER = crate::Reg<u32, _HSTPIPIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIER;
#[doc = "`write(|w| ..)` method takes [hstpipier::W](hstpipier::W) writer structure"]
impl crate::Writable for HSTPIPIER {}
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod hstpipier;
#[doc = "Host Pipe Enable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipier0_intpipes](hstpipier0_intpipes) module"]
pub type HSTPIPIER0_INTPIPES = crate::Reg<u32, _HSTPIPIER0_INTPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIER0_INTPIPES;
#[doc = "`write(|w| ..)` method takes [hstpipier0_intpipes::W](hstpipier0_intpipes::W) writer structure"]
impl crate::Writable for HSTPIPIER0_INTPIPES {}
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod hstpipier0_intpipes;
#[doc = "Host Pipe Enable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipier0_isopipes](hstpipier0_isopipes) module"]
pub type HSTPIPIER0_ISOPIPES = crate::Reg<u32, _HSTPIPIER0_ISOPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIER0_ISOPIPES;
#[doc = "`write(|w| ..)` method takes [hstpipier0_isopipes::W](hstpipier0_isopipes::W) writer structure"]
impl crate::Writable for HSTPIPIER0_ISOPIPES {}
#[doc = "Host Pipe Enable Register (n = 0)"]
pub mod hstpipier0_isopipes;
#[doc = "Host Pipe Disable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipidr](hstpipidr) module"]
pub type HSTPIPIDR = crate::Reg<u32, _HSTPIPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIDR;
#[doc = "`write(|w| ..)` method takes [hstpipidr::W](hstpipidr::W) writer structure"]
impl crate::Writable for HSTPIPIDR {}
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod hstpipidr;
#[doc = "Host Pipe Disable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipidr0_intpipes](hstpipidr0_intpipes) module"]
pub type HSTPIPIDR0_INTPIPES = crate::Reg<u32, _HSTPIPIDR0_INTPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIDR0_INTPIPES;
#[doc = "`write(|w| ..)` method takes [hstpipidr0_intpipes::W](hstpipidr0_intpipes::W) writer structure"]
impl crate::Writable for HSTPIPIDR0_INTPIPES {}
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod hstpipidr0_intpipes;
#[doc = "Host Pipe Disable Register (n = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipidr0_isopipes](hstpipidr0_isopipes) module"]
pub type HSTPIPIDR0_ISOPIPES = crate::Reg<u32, _HSTPIPIDR0_ISOPIPES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPIDR0_ISOPIPES;
#[doc = "`write(|w| ..)` method takes [hstpipidr0_isopipes::W](hstpipidr0_isopipes::W) writer structure"]
impl crate::Writable for HSTPIPIDR0_ISOPIPES {}
#[doc = "Host Pipe Disable Register (n = 0)"]
pub mod hstpipidr0_isopipes;
#[doc = "Host Pipe IN Request Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpipinrq](hstpipinrq) module"]
pub type HSTPIPINRQ = crate::Reg<u32, _HSTPIPINRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPINRQ;
#[doc = "`read()` method returns [hstpipinrq::R](hstpipinrq::R) reader structure"]
impl crate::Readable for HSTPIPINRQ {}
#[doc = "`write(|w| ..)` method takes [hstpipinrq::W](hstpipinrq::W) writer structure"]
impl crate::Writable for HSTPIPINRQ {}
#[doc = "Host Pipe IN Request Register (n = 0)"]
pub mod hstpipinrq;
#[doc = "Host Pipe Error Register (n = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstpiperr](hstpiperr) module"]
pub type HSTPIPERR = crate::Reg<u32, _HSTPIPERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTPIPERR;
#[doc = "`read()` method returns [hstpiperr::R](hstpiperr::R) reader structure"]
impl crate::Readable for HSTPIPERR {}
#[doc = "`write(|w| ..)` method takes [hstpiperr::W](hstpiperr::W) writer structure"]
impl crate::Writable for HSTPIPERR {}
#[doc = "Host Pipe Error Register (n = 0)"]
pub mod hstpiperr;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmanxtdsc1](hstdmanxtdsc1) module"]
pub type HSTDMANXTDSC1 = crate::Reg<u32, _HSTDMANXTDSC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMANXTDSC1;
#[doc = "`read()` method returns [hstdmanxtdsc1::R](hstdmanxtdsc1::R) reader structure"]
impl crate::Readable for HSTDMANXTDSC1 {}
#[doc = "`write(|w| ..)` method takes [hstdmanxtdsc1::W](hstdmanxtdsc1::W) writer structure"]
impl crate::Writable for HSTDMANXTDSC1 {}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod hstdmanxtdsc1;
#[doc = "Host DMA Channel Address Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmaaddress1](hstdmaaddress1) module"]
pub type HSTDMAADDRESS1 = crate::Reg<u32, _HSTDMAADDRESS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMAADDRESS1;
#[doc = "`read()` method returns [hstdmaaddress1::R](hstdmaaddress1::R) reader structure"]
impl crate::Readable for HSTDMAADDRESS1 {}
#[doc = "`write(|w| ..)` method takes [hstdmaaddress1::W](hstdmaaddress1::W) writer structure"]
impl crate::Writable for HSTDMAADDRESS1 {}
#[doc = "Host DMA Channel Address Register (n = 1)"]
pub mod hstdmaaddress1;
#[doc = "Host DMA Channel Control Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmacontrol1](hstdmacontrol1) module"]
pub type HSTDMACONTROL1 = crate::Reg<u32, _HSTDMACONTROL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMACONTROL1;
#[doc = "`read()` method returns [hstdmacontrol1::R](hstdmacontrol1::R) reader structure"]
impl crate::Readable for HSTDMACONTROL1 {}
#[doc = "`write(|w| ..)` method takes [hstdmacontrol1::W](hstdmacontrol1::W) writer structure"]
impl crate::Writable for HSTDMACONTROL1 {}
#[doc = "Host DMA Channel Control Register (n = 1)"]
pub mod hstdmacontrol1;
#[doc = "Host DMA Channel Status Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmastatus1](hstdmastatus1) module"]
pub type HSTDMASTATUS1 = crate::Reg<u32, _HSTDMASTATUS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMASTATUS1;
#[doc = "`read()` method returns [hstdmastatus1::R](hstdmastatus1::R) reader structure"]
impl crate::Readable for HSTDMASTATUS1 {}
#[doc = "`write(|w| ..)` method takes [hstdmastatus1::W](hstdmastatus1::W) writer structure"]
impl crate::Writable for HSTDMASTATUS1 {}
#[doc = "Host DMA Channel Status Register (n = 1)"]
pub mod hstdmastatus1;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmanxtdsc2](hstdmanxtdsc2) module"]
pub type HSTDMANXTDSC2 = crate::Reg<u32, _HSTDMANXTDSC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMANXTDSC2;
#[doc = "`read()` method returns [hstdmanxtdsc2::R](hstdmanxtdsc2::R) reader structure"]
impl crate::Readable for HSTDMANXTDSC2 {}
#[doc = "`write(|w| ..)` method takes [hstdmanxtdsc2::W](hstdmanxtdsc2::W) writer structure"]
impl crate::Writable for HSTDMANXTDSC2 {}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 2)"]
pub mod hstdmanxtdsc2;
#[doc = "Host DMA Channel Address Register (n = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmaaddress2](hstdmaaddress2) module"]
pub type HSTDMAADDRESS2 = crate::Reg<u32, _HSTDMAADDRESS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMAADDRESS2;
#[doc = "`read()` method returns [hstdmaaddress2::R](hstdmaaddress2::R) reader structure"]
impl crate::Readable for HSTDMAADDRESS2 {}
#[doc = "`write(|w| ..)` method takes [hstdmaaddress2::W](hstdmaaddress2::W) writer structure"]
impl crate::Writable for HSTDMAADDRESS2 {}
#[doc = "Host DMA Channel Address Register (n = 2)"]
pub mod hstdmaaddress2;
#[doc = "Host DMA Channel Control Register (n = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmacontrol2](hstdmacontrol2) module"]
pub type HSTDMACONTROL2 = crate::Reg<u32, _HSTDMACONTROL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMACONTROL2;
#[doc = "`read()` method returns [hstdmacontrol2::R](hstdmacontrol2::R) reader structure"]
impl crate::Readable for HSTDMACONTROL2 {}
#[doc = "`write(|w| ..)` method takes [hstdmacontrol2::W](hstdmacontrol2::W) writer structure"]
impl crate::Writable for HSTDMACONTROL2 {}
#[doc = "Host DMA Channel Control Register (n = 2)"]
pub mod hstdmacontrol2;
#[doc = "Host DMA Channel Status Register (n = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmastatus2](hstdmastatus2) module"]
pub type HSTDMASTATUS2 = crate::Reg<u32, _HSTDMASTATUS2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMASTATUS2;
#[doc = "`read()` method returns [hstdmastatus2::R](hstdmastatus2::R) reader structure"]
impl crate::Readable for HSTDMASTATUS2 {}
#[doc = "`write(|w| ..)` method takes [hstdmastatus2::W](hstdmastatus2::W) writer structure"]
impl crate::Writable for HSTDMASTATUS2 {}
#[doc = "Host DMA Channel Status Register (n = 2)"]
pub mod hstdmastatus2;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmanxtdsc3](hstdmanxtdsc3) module"]
pub type HSTDMANXTDSC3 = crate::Reg<u32, _HSTDMANXTDSC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMANXTDSC3;
#[doc = "`read()` method returns [hstdmanxtdsc3::R](hstdmanxtdsc3::R) reader structure"]
impl crate::Readable for HSTDMANXTDSC3 {}
#[doc = "`write(|w| ..)` method takes [hstdmanxtdsc3::W](hstdmanxtdsc3::W) writer structure"]
impl crate::Writable for HSTDMANXTDSC3 {}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 3)"]
pub mod hstdmanxtdsc3;
#[doc = "Host DMA Channel Address Register (n = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmaaddress3](hstdmaaddress3) module"]
pub type HSTDMAADDRESS3 = crate::Reg<u32, _HSTDMAADDRESS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMAADDRESS3;
#[doc = "`read()` method returns [hstdmaaddress3::R](hstdmaaddress3::R) reader structure"]
impl crate::Readable for HSTDMAADDRESS3 {}
#[doc = "`write(|w| ..)` method takes [hstdmaaddress3::W](hstdmaaddress3::W) writer structure"]
impl crate::Writable for HSTDMAADDRESS3 {}
#[doc = "Host DMA Channel Address Register (n = 3)"]
pub mod hstdmaaddress3;
#[doc = "Host DMA Channel Control Register (n = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmacontrol3](hstdmacontrol3) module"]
pub type HSTDMACONTROL3 = crate::Reg<u32, _HSTDMACONTROL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMACONTROL3;
#[doc = "`read()` method returns [hstdmacontrol3::R](hstdmacontrol3::R) reader structure"]
impl crate::Readable for HSTDMACONTROL3 {}
#[doc = "`write(|w| ..)` method takes [hstdmacontrol3::W](hstdmacontrol3::W) writer structure"]
impl crate::Writable for HSTDMACONTROL3 {}
#[doc = "Host DMA Channel Control Register (n = 3)"]
pub mod hstdmacontrol3;
#[doc = "Host DMA Channel Status Register (n = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmastatus3](hstdmastatus3) module"]
pub type HSTDMASTATUS3 = crate::Reg<u32, _HSTDMASTATUS3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMASTATUS3;
#[doc = "`read()` method returns [hstdmastatus3::R](hstdmastatus3::R) reader structure"]
impl crate::Readable for HSTDMASTATUS3 {}
#[doc = "`write(|w| ..)` method takes [hstdmastatus3::W](hstdmastatus3::W) writer structure"]
impl crate::Writable for HSTDMASTATUS3 {}
#[doc = "Host DMA Channel Status Register (n = 3)"]
pub mod hstdmastatus3;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmanxtdsc4](hstdmanxtdsc4) module"]
pub type HSTDMANXTDSC4 = crate::Reg<u32, _HSTDMANXTDSC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMANXTDSC4;
#[doc = "`read()` method returns [hstdmanxtdsc4::R](hstdmanxtdsc4::R) reader structure"]
impl crate::Readable for HSTDMANXTDSC4 {}
#[doc = "`write(|w| ..)` method takes [hstdmanxtdsc4::W](hstdmanxtdsc4::W) writer structure"]
impl crate::Writable for HSTDMANXTDSC4 {}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 4)"]
pub mod hstdmanxtdsc4;
#[doc = "Host DMA Channel Address Register (n = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmaaddress4](hstdmaaddress4) module"]
pub type HSTDMAADDRESS4 = crate::Reg<u32, _HSTDMAADDRESS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMAADDRESS4;
#[doc = "`read()` method returns [hstdmaaddress4::R](hstdmaaddress4::R) reader structure"]
impl crate::Readable for HSTDMAADDRESS4 {}
#[doc = "`write(|w| ..)` method takes [hstdmaaddress4::W](hstdmaaddress4::W) writer structure"]
impl crate::Writable for HSTDMAADDRESS4 {}
#[doc = "Host DMA Channel Address Register (n = 4)"]
pub mod hstdmaaddress4;
#[doc = "Host DMA Channel Control Register (n = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmacontrol4](hstdmacontrol4) module"]
pub type HSTDMACONTROL4 = crate::Reg<u32, _HSTDMACONTROL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMACONTROL4;
#[doc = "`read()` method returns [hstdmacontrol4::R](hstdmacontrol4::R) reader structure"]
impl crate::Readable for HSTDMACONTROL4 {}
#[doc = "`write(|w| ..)` method takes [hstdmacontrol4::W](hstdmacontrol4::W) writer structure"]
impl crate::Writable for HSTDMACONTROL4 {}
#[doc = "Host DMA Channel Control Register (n = 4)"]
pub mod hstdmacontrol4;
#[doc = "Host DMA Channel Status Register (n = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmastatus4](hstdmastatus4) module"]
pub type HSTDMASTATUS4 = crate::Reg<u32, _HSTDMASTATUS4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMASTATUS4;
#[doc = "`read()` method returns [hstdmastatus4::R](hstdmastatus4::R) reader structure"]
impl crate::Readable for HSTDMASTATUS4 {}
#[doc = "`write(|w| ..)` method takes [hstdmastatus4::W](hstdmastatus4::W) writer structure"]
impl crate::Writable for HSTDMASTATUS4 {}
#[doc = "Host DMA Channel Status Register (n = 4)"]
pub mod hstdmastatus4;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmanxtdsc5](hstdmanxtdsc5) module"]
pub type HSTDMANXTDSC5 = crate::Reg<u32, _HSTDMANXTDSC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMANXTDSC5;
#[doc = "`read()` method returns [hstdmanxtdsc5::R](hstdmanxtdsc5::R) reader structure"]
impl crate::Readable for HSTDMANXTDSC5 {}
#[doc = "`write(|w| ..)` method takes [hstdmanxtdsc5::W](hstdmanxtdsc5::W) writer structure"]
impl crate::Writable for HSTDMANXTDSC5 {}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 5)"]
pub mod hstdmanxtdsc5;
#[doc = "Host DMA Channel Address Register (n = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmaaddress5](hstdmaaddress5) module"]
pub type HSTDMAADDRESS5 = crate::Reg<u32, _HSTDMAADDRESS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMAADDRESS5;
#[doc = "`read()` method returns [hstdmaaddress5::R](hstdmaaddress5::R) reader structure"]
impl crate::Readable for HSTDMAADDRESS5 {}
#[doc = "`write(|w| ..)` method takes [hstdmaaddress5::W](hstdmaaddress5::W) writer structure"]
impl crate::Writable for HSTDMAADDRESS5 {}
#[doc = "Host DMA Channel Address Register (n = 5)"]
pub mod hstdmaaddress5;
#[doc = "Host DMA Channel Control Register (n = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmacontrol5](hstdmacontrol5) module"]
pub type HSTDMACONTROL5 = crate::Reg<u32, _HSTDMACONTROL5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMACONTROL5;
#[doc = "`read()` method returns [hstdmacontrol5::R](hstdmacontrol5::R) reader structure"]
impl crate::Readable for HSTDMACONTROL5 {}
#[doc = "`write(|w| ..)` method takes [hstdmacontrol5::W](hstdmacontrol5::W) writer structure"]
impl crate::Writable for HSTDMACONTROL5 {}
#[doc = "Host DMA Channel Control Register (n = 5)"]
pub mod hstdmacontrol5;
#[doc = "Host DMA Channel Status Register (n = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmastatus5](hstdmastatus5) module"]
pub type HSTDMASTATUS5 = crate::Reg<u32, _HSTDMASTATUS5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMASTATUS5;
#[doc = "`read()` method returns [hstdmastatus5::R](hstdmastatus5::R) reader structure"]
impl crate::Readable for HSTDMASTATUS5 {}
#[doc = "`write(|w| ..)` method takes [hstdmastatus5::W](hstdmastatus5::W) writer structure"]
impl crate::Writable for HSTDMASTATUS5 {}
#[doc = "Host DMA Channel Status Register (n = 5)"]
pub mod hstdmastatus5;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmanxtdsc6](hstdmanxtdsc6) module"]
pub type HSTDMANXTDSC6 = crate::Reg<u32, _HSTDMANXTDSC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMANXTDSC6;
#[doc = "`read()` method returns [hstdmanxtdsc6::R](hstdmanxtdsc6::R) reader structure"]
impl crate::Readable for HSTDMANXTDSC6 {}
#[doc = "`write(|w| ..)` method takes [hstdmanxtdsc6::W](hstdmanxtdsc6::W) writer structure"]
impl crate::Writable for HSTDMANXTDSC6 {}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 6)"]
pub mod hstdmanxtdsc6;
#[doc = "Host DMA Channel Address Register (n = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmaaddress6](hstdmaaddress6) module"]
pub type HSTDMAADDRESS6 = crate::Reg<u32, _HSTDMAADDRESS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMAADDRESS6;
#[doc = "`read()` method returns [hstdmaaddress6::R](hstdmaaddress6::R) reader structure"]
impl crate::Readable for HSTDMAADDRESS6 {}
#[doc = "`write(|w| ..)` method takes [hstdmaaddress6::W](hstdmaaddress6::W) writer structure"]
impl crate::Writable for HSTDMAADDRESS6 {}
#[doc = "Host DMA Channel Address Register (n = 6)"]
pub mod hstdmaaddress6;
#[doc = "Host DMA Channel Control Register (n = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmacontrol6](hstdmacontrol6) module"]
pub type HSTDMACONTROL6 = crate::Reg<u32, _HSTDMACONTROL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMACONTROL6;
#[doc = "`read()` method returns [hstdmacontrol6::R](hstdmacontrol6::R) reader structure"]
impl crate::Readable for HSTDMACONTROL6 {}
#[doc = "`write(|w| ..)` method takes [hstdmacontrol6::W](hstdmacontrol6::W) writer structure"]
impl crate::Writable for HSTDMACONTROL6 {}
#[doc = "Host DMA Channel Control Register (n = 6)"]
pub mod hstdmacontrol6;
#[doc = "Host DMA Channel Status Register (n = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmastatus6](hstdmastatus6) module"]
pub type HSTDMASTATUS6 = crate::Reg<u32, _HSTDMASTATUS6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMASTATUS6;
#[doc = "`read()` method returns [hstdmastatus6::R](hstdmastatus6::R) reader structure"]
impl crate::Readable for HSTDMASTATUS6 {}
#[doc = "`write(|w| ..)` method takes [hstdmastatus6::W](hstdmastatus6::W) writer structure"]
impl crate::Writable for HSTDMASTATUS6 {}
#[doc = "Host DMA Channel Status Register (n = 6)"]
pub mod hstdmastatus6;
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmanxtdsc7](hstdmanxtdsc7) module"]
pub type HSTDMANXTDSC7 = crate::Reg<u32, _HSTDMANXTDSC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMANXTDSC7;
#[doc = "`read()` method returns [hstdmanxtdsc7::R](hstdmanxtdsc7::R) reader structure"]
impl crate::Readable for HSTDMANXTDSC7 {}
#[doc = "`write(|w| ..)` method takes [hstdmanxtdsc7::W](hstdmanxtdsc7::W) writer structure"]
impl crate::Writable for HSTDMANXTDSC7 {}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 7)"]
pub mod hstdmanxtdsc7;
#[doc = "Host DMA Channel Address Register (n = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmaaddress7](hstdmaaddress7) module"]
pub type HSTDMAADDRESS7 = crate::Reg<u32, _HSTDMAADDRESS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMAADDRESS7;
#[doc = "`read()` method returns [hstdmaaddress7::R](hstdmaaddress7::R) reader structure"]
impl crate::Readable for HSTDMAADDRESS7 {}
#[doc = "`write(|w| ..)` method takes [hstdmaaddress7::W](hstdmaaddress7::W) writer structure"]
impl crate::Writable for HSTDMAADDRESS7 {}
#[doc = "Host DMA Channel Address Register (n = 7)"]
pub mod hstdmaaddress7;
#[doc = "Host DMA Channel Control Register (n = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmacontrol7](hstdmacontrol7) module"]
pub type HSTDMACONTROL7 = crate::Reg<u32, _HSTDMACONTROL7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMACONTROL7;
#[doc = "`read()` method returns [hstdmacontrol7::R](hstdmacontrol7::R) reader structure"]
impl crate::Readable for HSTDMACONTROL7 {}
#[doc = "`write(|w| ..)` method takes [hstdmacontrol7::W](hstdmacontrol7::W) writer structure"]
impl crate::Writable for HSTDMACONTROL7 {}
#[doc = "Host DMA Channel Control Register (n = 7)"]
pub mod hstdmacontrol7;
#[doc = "Host DMA Channel Status Register (n = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hstdmastatus7](hstdmastatus7) module"]
pub type HSTDMASTATUS7 = crate::Reg<u32, _HSTDMASTATUS7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSTDMASTATUS7;
#[doc = "`read()` method returns [hstdmastatus7::R](hstdmastatus7::R) reader structure"]
impl crate::Readable for HSTDMASTATUS7 {}
#[doc = "`write(|w| ..)` method takes [hstdmastatus7::W](hstdmastatus7::W) writer structure"]
impl crate::Writable for HSTDMASTATUS7 {}
#[doc = "Host DMA Channel Status Register (n = 7)"]
pub mod hstdmastatus7;
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "General Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "General Status Register"]
pub mod sr;
#[doc = "General Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "General Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sfr](sfr) module"]
pub type SFR = crate::Reg<u32, _SFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SFR;
#[doc = "`write(|w| ..)` method takes [sfr::W](sfr::W) writer structure"]
impl crate::Writable for SFR {}
#[doc = "General Status Set Register"]
pub mod sfr;
#[doc = "General Finite State Machine Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fsm](fsm) module"]
pub type FSM = crate::Reg<u32, _FSM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FSM;
#[doc = "`read()` method returns [fsm::R](fsm::R) reader structure"]
impl crate::Readable for FSM {}
#[doc = "General Finite State Machine Register"]
pub mod fsm;
