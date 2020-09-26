#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PIO Enable Register"]
    pub per: PER,
    #[doc = "0x04 - PIO Disable Register"]
    pub pdr: PDR,
    #[doc = "0x08 - PIO Status Register"]
    pub psr: PSR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Output Enable Register"]
    pub oer: OER,
    #[doc = "0x14 - Output Disable Register"]
    pub odr: ODR,
    #[doc = "0x18 - Output Status Register"]
    pub osr: OSR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - Glitch Input Filter Enable Register"]
    pub ifer: IFER,
    #[doc = "0x24 - Glitch Input Filter Disable Register"]
    pub ifdr: IFDR,
    #[doc = "0x28 - Glitch Input Filter Status Register"]
    pub ifsr: IFSR,
    _reserved9: [u8; 4usize],
    #[doc = "0x30 - Set Output Data Register"]
    pub sodr: SODR,
    #[doc = "0x34 - Clear Output Data Register"]
    pub codr: CODR,
    #[doc = "0x38 - Output Data Status Register"]
    pub odsr: ODSR,
    #[doc = "0x3c - Pin Data Status Register"]
    pub pdsr: PDSR,
    #[doc = "0x40 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x44 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x48 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x4c - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x50 - Multi-driver Enable Register"]
    pub mder: MDER,
    #[doc = "0x54 - Multi-driver Disable Register"]
    pub mddr: MDDR,
    #[doc = "0x58 - Multi-driver Status Register"]
    pub mdsr: MDSR,
    _reserved20: [u8; 4usize],
    #[doc = "0x60 - Pull-up Disable Register"]
    pub pudr: PUDR,
    #[doc = "0x64 - Pull-up Enable Register"]
    pub puer: PUER,
    #[doc = "0x68 - Pad Pull-up Status Register"]
    pub pusr: PUSR,
    _reserved23: [u8; 4usize],
    #[doc = "0x70 - Peripheral AB Select Register"]
    pub absr: ABSR,
    _reserved24: [u8; 12usize],
    #[doc = "0x80 - System Clock Glitch Input Filter Select Register"]
    pub scifsr: SCIFSR,
    #[doc = "0x84 - Debouncing Input Filter Select Register"]
    pub difsr: DIFSR,
    #[doc = "0x88 - Glitch or Debouncing Input Filter Clock Selection Status Register"]
    pub ifdgsr: IFDGSR,
    #[doc = "0x8c - Slow Clock Divider Debouncing Register"]
    pub scdr: SCDR,
    _reserved28: [u8; 16usize],
    #[doc = "0xa0 - Output Write Enable"]
    pub ower: OWER,
    #[doc = "0xa4 - Output Write Disable"]
    pub owdr: OWDR,
    #[doc = "0xa8 - Output Write Status Register"]
    pub owsr: OWSR,
    _reserved31: [u8; 4usize],
    #[doc = "0xb0 - Additional Interrupt Modes Enable Register"]
    pub aimer: AIMER,
    #[doc = "0xb4 - Additional Interrupt Modes Disables Register"]
    pub aimdr: AIMDR,
    #[doc = "0xb8 - Additional Interrupt Modes Mask Register"]
    pub aimmr: AIMMR,
    _reserved34: [u8; 4usize],
    #[doc = "0xc0 - Edge Select Register"]
    pub esr: ESR,
    #[doc = "0xc4 - Level Select Register"]
    pub lsr: LSR,
    #[doc = "0xc8 - Edge/Level Status Register"]
    pub elsr: ELSR,
    _reserved37: [u8; 4usize],
    #[doc = "0xd0 - Falling Edge/Low Level Select Register"]
    pub fellsr: FELLSR,
    #[doc = "0xd4 - Rising Edge/ High Level Select Register"]
    pub rehlsr: REHLSR,
    #[doc = "0xd8 - Fall/Rise - Low/High Status Register"]
    pub frlhsr: FRLHSR,
    _reserved40: [u8; 4usize],
    #[doc = "0xe0 - Lock Status"]
    pub locksr: LOCKSR,
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "PIO Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per](per) module"]
pub type PER = crate::Reg<u32, _PER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PER;
#[doc = "`write(|w| ..)` method takes [per::W](per::W) writer structure"]
impl crate::Writable for PER {}
#[doc = "PIO Enable Register"]
pub mod per;
#[doc = "PIO Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdr](pdr) module"]
pub type PDR = crate::Reg<u32, _PDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDR;
#[doc = "`write(|w| ..)` method takes [pdr::W](pdr::W) writer structure"]
impl crate::Writable for PDR {}
#[doc = "PIO Disable Register"]
pub mod pdr;
#[doc = "PIO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](psr) module"]
pub type PSR = crate::Reg<u32, _PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSR;
#[doc = "`read()` method returns [psr::R](psr::R) reader structure"]
impl crate::Readable for PSR {}
#[doc = "PIO Status Register"]
pub mod psr;
#[doc = "Output Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oer](oer) module"]
pub type OER = crate::Reg<u32, _OER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OER;
#[doc = "`write(|w| ..)` method takes [oer::W](oer::W) writer structure"]
impl crate::Writable for OER {}
#[doc = "Output Enable Register"]
pub mod oer;
#[doc = "Output Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](odr) module"]
pub type ODR = crate::Reg<u32, _ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODR;
#[doc = "`write(|w| ..)` method takes [odr::W](odr::W) writer structure"]
impl crate::Writable for ODR {}
#[doc = "Output Disable Register"]
pub mod odr;
#[doc = "Output Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osr](osr) module"]
pub type OSR = crate::Reg<u32, _OSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSR;
#[doc = "`read()` method returns [osr::R](osr::R) reader structure"]
impl crate::Readable for OSR {}
#[doc = "Output Status Register"]
pub mod osr;
#[doc = "Glitch Input Filter Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifer](ifer) module"]
pub type IFER = crate::Reg<u32, _IFER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFER;
#[doc = "`write(|w| ..)` method takes [ifer::W](ifer::W) writer structure"]
impl crate::Writable for IFER {}
#[doc = "Glitch Input Filter Enable Register"]
pub mod ifer;
#[doc = "Glitch Input Filter Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifdr](ifdr) module"]
pub type IFDR = crate::Reg<u32, _IFDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFDR;
#[doc = "`write(|w| ..)` method takes [ifdr::W](ifdr::W) writer structure"]
impl crate::Writable for IFDR {}
#[doc = "Glitch Input Filter Disable Register"]
pub mod ifdr;
#[doc = "Glitch Input Filter Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifsr](ifsr) module"]
pub type IFSR = crate::Reg<u32, _IFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFSR;
#[doc = "`read()` method returns [ifsr::R](ifsr::R) reader structure"]
impl crate::Readable for IFSR {}
#[doc = "Glitch Input Filter Status Register"]
pub mod ifsr;
#[doc = "Set Output Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sodr](sodr) module"]
pub type SODR = crate::Reg<u32, _SODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SODR;
#[doc = "`write(|w| ..)` method takes [sodr::W](sodr::W) writer structure"]
impl crate::Writable for SODR {}
#[doc = "Set Output Data Register"]
pub mod sodr;
#[doc = "Clear Output Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codr](codr) module"]
pub type CODR = crate::Reg<u32, _CODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODR;
#[doc = "`write(|w| ..)` method takes [codr::W](codr::W) writer structure"]
impl crate::Writable for CODR {}
#[doc = "Clear Output Data Register"]
pub mod codr;
#[doc = "Output Data Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odsr](odsr) module"]
pub type ODSR = crate::Reg<u32, _ODSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODSR;
#[doc = "`read()` method returns [odsr::R](odsr::R) reader structure"]
impl crate::Readable for ODSR {}
#[doc = "`write(|w| ..)` method takes [odsr::W](odsr::W) writer structure"]
impl crate::Writable for ODSR {}
#[doc = "Output Data Status Register"]
pub mod odsr;
#[doc = "Pin Data Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsr](pdsr) module"]
pub type PDSR = crate::Reg<u32, _PDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSR;
#[doc = "`read()` method returns [pdsr::R](pdsr::R) reader structure"]
impl crate::Readable for PDSR {}
#[doc = "Pin Data Status Register"]
pub mod pdsr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Multi-driver Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mder](mder) module"]
pub type MDER = crate::Reg<u32, _MDER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDER;
#[doc = "`write(|w| ..)` method takes [mder::W](mder::W) writer structure"]
impl crate::Writable for MDER {}
#[doc = "Multi-driver Enable Register"]
pub mod mder;
#[doc = "Multi-driver Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mddr](mddr) module"]
pub type MDDR = crate::Reg<u32, _MDDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDDR;
#[doc = "`write(|w| ..)` method takes [mddr::W](mddr::W) writer structure"]
impl crate::Writable for MDDR {}
#[doc = "Multi-driver Disable Register"]
pub mod mddr;
#[doc = "Multi-driver Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdsr](mdsr) module"]
pub type MDSR = crate::Reg<u32, _MDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDSR;
#[doc = "`read()` method returns [mdsr::R](mdsr::R) reader structure"]
impl crate::Readable for MDSR {}
#[doc = "Multi-driver Status Register"]
pub mod mdsr;
#[doc = "Pull-up Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pudr](pudr) module"]
pub type PUDR = crate::Reg<u32, _PUDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUDR;
#[doc = "`write(|w| ..)` method takes [pudr::W](pudr::W) writer structure"]
impl crate::Writable for PUDR {}
#[doc = "Pull-up Disable Register"]
pub mod pudr;
#[doc = "Pull-up Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [puer](puer) module"]
pub type PUER = crate::Reg<u32, _PUER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUER;
#[doc = "`write(|w| ..)` method takes [puer::W](puer::W) writer structure"]
impl crate::Writable for PUER {}
#[doc = "Pull-up Enable Register"]
pub mod puer;
#[doc = "Pad Pull-up Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pusr](pusr) module"]
pub type PUSR = crate::Reg<u32, _PUSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUSR;
#[doc = "`read()` method returns [pusr::R](pusr::R) reader structure"]
impl crate::Readable for PUSR {}
#[doc = "Pad Pull-up Status Register"]
pub mod pusr;
#[doc = "Peripheral AB Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [absr](absr) module"]
pub type ABSR = crate::Reg<u32, _ABSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABSR;
#[doc = "`read()` method returns [absr::R](absr::R) reader structure"]
impl crate::Readable for ABSR {}
#[doc = "`write(|w| ..)` method takes [absr::W](absr::W) writer structure"]
impl crate::Writable for ABSR {}
#[doc = "Peripheral AB Select Register"]
pub mod absr;
#[doc = "System Clock Glitch Input Filter Select Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scifsr](scifsr) module"]
pub type SCIFSR = crate::Reg<u32, _SCIFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCIFSR;
#[doc = "`write(|w| ..)` method takes [scifsr::W](scifsr::W) writer structure"]
impl crate::Writable for SCIFSR {}
#[doc = "System Clock Glitch Input Filter Select Register"]
pub mod scifsr;
#[doc = "Debouncing Input Filter Select Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [difsr](difsr) module"]
pub type DIFSR = crate::Reg<u32, _DIFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIFSR;
#[doc = "`write(|w| ..)` method takes [difsr::W](difsr::W) writer structure"]
impl crate::Writable for DIFSR {}
#[doc = "Debouncing Input Filter Select Register"]
pub mod difsr;
#[doc = "Glitch or Debouncing Input Filter Clock Selection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifdgsr](ifdgsr) module"]
pub type IFDGSR = crate::Reg<u32, _IFDGSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFDGSR;
#[doc = "`read()` method returns [ifdgsr::R](ifdgsr::R) reader structure"]
impl crate::Readable for IFDGSR {}
#[doc = "Glitch or Debouncing Input Filter Clock Selection Status Register"]
pub mod ifdgsr;
#[doc = "Slow Clock Divider Debouncing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scdr](scdr) module"]
pub type SCDR = crate::Reg<u32, _SCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCDR;
#[doc = "`read()` method returns [scdr::R](scdr::R) reader structure"]
impl crate::Readable for SCDR {}
#[doc = "`write(|w| ..)` method takes [scdr::W](scdr::W) writer structure"]
impl crate::Writable for SCDR {}
#[doc = "Slow Clock Divider Debouncing Register"]
pub mod scdr;
#[doc = "Output Write Enable\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ower](ower) module"]
pub type OWER = crate::Reg<u32, _OWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OWER;
#[doc = "`write(|w| ..)` method takes [ower::W](ower::W) writer structure"]
impl crate::Writable for OWER {}
#[doc = "Output Write Enable"]
pub mod ower;
#[doc = "Output Write Disable\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [owdr](owdr) module"]
pub type OWDR = crate::Reg<u32, _OWDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OWDR;
#[doc = "`write(|w| ..)` method takes [owdr::W](owdr::W) writer structure"]
impl crate::Writable for OWDR {}
#[doc = "Output Write Disable"]
pub mod owdr;
#[doc = "Output Write Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [owsr](owsr) module"]
pub type OWSR = crate::Reg<u32, _OWSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OWSR;
#[doc = "`read()` method returns [owsr::R](owsr::R) reader structure"]
impl crate::Readable for OWSR {}
#[doc = "Output Write Status Register"]
pub mod owsr;
#[doc = "Additional Interrupt Modes Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aimer](aimer) module"]
pub type AIMER = crate::Reg<u32, _AIMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIMER;
#[doc = "`write(|w| ..)` method takes [aimer::W](aimer::W) writer structure"]
impl crate::Writable for AIMER {}
#[doc = "Additional Interrupt Modes Enable Register"]
pub mod aimer;
#[doc = "Additional Interrupt Modes Disables Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aimdr](aimdr) module"]
pub type AIMDR = crate::Reg<u32, _AIMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIMDR;
#[doc = "`write(|w| ..)` method takes [aimdr::W](aimdr::W) writer structure"]
impl crate::Writable for AIMDR {}
#[doc = "Additional Interrupt Modes Disables Register"]
pub mod aimdr;
#[doc = "Additional Interrupt Modes Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aimmr](aimmr) module"]
pub type AIMMR = crate::Reg<u32, _AIMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIMMR;
#[doc = "`read()` method returns [aimmr::R](aimmr::R) reader structure"]
impl crate::Readable for AIMMR {}
#[doc = "Additional Interrupt Modes Mask Register"]
pub mod aimmr;
#[doc = "Edge Select Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](esr) module"]
pub type ESR = crate::Reg<u32, _ESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ESR;
#[doc = "`write(|w| ..)` method takes [esr::W](esr::W) writer structure"]
impl crate::Writable for ESR {}
#[doc = "Edge Select Register"]
pub mod esr;
#[doc = "Level Select Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](lsr) module"]
pub type LSR = crate::Reg<u32, _LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSR;
#[doc = "`write(|w| ..)` method takes [lsr::W](lsr::W) writer structure"]
impl crate::Writable for LSR {}
#[doc = "Level Select Register"]
pub mod lsr;
#[doc = "Edge/Level Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elsr](elsr) module"]
pub type ELSR = crate::Reg<u32, _ELSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ELSR;
#[doc = "`read()` method returns [elsr::R](elsr::R) reader structure"]
impl crate::Readable for ELSR {}
#[doc = "Edge/Level Status Register"]
pub mod elsr;
#[doc = "Falling Edge/Low Level Select Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fellsr](fellsr) module"]
pub type FELLSR = crate::Reg<u32, _FELLSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FELLSR;
#[doc = "`write(|w| ..)` method takes [fellsr::W](fellsr::W) writer structure"]
impl crate::Writable for FELLSR {}
#[doc = "Falling Edge/Low Level Select Register"]
pub mod fellsr;
#[doc = "Rising Edge/ High Level Select Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rehlsr](rehlsr) module"]
pub type REHLSR = crate::Reg<u32, _REHLSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REHLSR;
#[doc = "`write(|w| ..)` method takes [rehlsr::W](rehlsr::W) writer structure"]
impl crate::Writable for REHLSR {}
#[doc = "Rising Edge/ High Level Select Register"]
pub mod rehlsr;
#[doc = "Fall/Rise - Low/High Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frlhsr](frlhsr) module"]
pub type FRLHSR = crate::Reg<u32, _FRLHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRLHSR;
#[doc = "`read()` method returns [frlhsr::R](frlhsr::R) reader structure"]
impl crate::Readable for FRLHSR {}
#[doc = "Fall/Rise - Low/High Status Register"]
pub mod frlhsr;
#[doc = "Lock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [locksr](locksr) module"]
pub type LOCKSR = crate::Reg<u32, _LOCKSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKSR;
#[doc = "`read()` method returns [locksr::R](locksr::R) reader structure"]
impl crate::Readable for LOCKSR {}
#[doc = "Lock Status"]
pub mod locksr;
#[doc = "Write Protect Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protect Status Register"]
pub mod wpsr;
