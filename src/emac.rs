#[doc = r"Register block"]
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
    #[doc = "0x90 - Hash Register Bottom \\[31:0\\]
Register"]
    pub hrb: HRB,
    #[doc = "0x94 - Hash Register Top \\[63:32\\]
Register"]
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
#[doc = "Network Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncr](ncr) module"]
pub type NCR = crate::Reg<u32, _NCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCR;
#[doc = "`read()` method returns [ncr::R](ncr::R) reader structure"]
impl crate::Readable for NCR {}
#[doc = "`write(|w| ..)` method takes [ncr::W](ncr::W) writer structure"]
impl crate::Writable for NCR {}
#[doc = "Network Control Register"]
pub mod ncr;
#[doc = "Network Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncfgr](ncfgr) module"]
pub type NCFGR = crate::Reg<u32, _NCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NCFGR;
#[doc = "`read()` method returns [ncfgr::R](ncfgr::R) reader structure"]
impl crate::Readable for NCFGR {}
#[doc = "`write(|w| ..)` method takes [ncfgr::W](ncfgr::W) writer structure"]
impl crate::Writable for NCFGR {}
#[doc = "Network Configuration Register"]
pub mod ncfgr;
#[doc = "Network Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsr](nsr) module"]
pub type NSR = crate::Reg<u32, _NSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NSR;
#[doc = "`read()` method returns [nsr::R](nsr::R) reader structure"]
impl crate::Readable for NSR {}
#[doc = "Network Status Register"]
pub mod nsr;
#[doc = "Transmit Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsr](tsr) module"]
pub type TSR = crate::Reg<u32, _TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSR;
#[doc = "`read()` method returns [tsr::R](tsr::R) reader structure"]
impl crate::Readable for TSR {}
#[doc = "`write(|w| ..)` method takes [tsr::W](tsr::W) writer structure"]
impl crate::Writable for TSR {}
#[doc = "Transmit Status Register"]
pub mod tsr;
#[doc = "Receive Buffer Queue Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbqp](rbqp) module"]
pub type RBQP = crate::Reg<u32, _RBQP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBQP;
#[doc = "`read()` method returns [rbqp::R](rbqp::R) reader structure"]
impl crate::Readable for RBQP {}
#[doc = "`write(|w| ..)` method takes [rbqp::W](rbqp::W) writer structure"]
impl crate::Writable for RBQP {}
#[doc = "Receive Buffer Queue Pointer Register"]
pub mod rbqp;
#[doc = "Transmit Buffer Queue Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbqp](tbqp) module"]
pub type TBQP = crate::Reg<u32, _TBQP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBQP;
#[doc = "`read()` method returns [tbqp::R](tbqp::R) reader structure"]
impl crate::Readable for TBQP {}
#[doc = "`write(|w| ..)` method takes [tbqp::W](tbqp::W) writer structure"]
impl crate::Writable for TBQP {}
#[doc = "Transmit Buffer Queue Pointer Register"]
pub mod tbqp;
#[doc = "Receive Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsr](rsr) module"]
pub type RSR = crate::Reg<u32, _RSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSR;
#[doc = "`read()` method returns [rsr::R](rsr::R) reader structure"]
impl crate::Readable for RSR {}
#[doc = "`write(|w| ..)` method takes [rsr::W](rsr::W) writer structure"]
impl crate::Writable for RSR {}
#[doc = "Receive Status Register"]
pub mod rsr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
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
#[doc = "Phy Maintenance Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man](man) module"]
pub type MAN = crate::Reg<u32, _MAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAN;
#[doc = "`read()` method returns [man::R](man::R) reader structure"]
impl crate::Readable for MAN {}
#[doc = "`write(|w| ..)` method takes [man::W](man::W) writer structure"]
impl crate::Writable for MAN {}
#[doc = "Phy Maintenance Register"]
pub mod man;
#[doc = "Pause Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptr](ptr) module"]
pub type PTR = crate::Reg<u32, _PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTR;
#[doc = "`read()` method returns [ptr::R](ptr::R) reader structure"]
impl crate::Readable for PTR {}
#[doc = "`write(|w| ..)` method takes [ptr::W](ptr::W) writer structure"]
impl crate::Writable for PTR {}
#[doc = "Pause Time Register"]
pub mod ptr;
#[doc = "Pause Frames Received Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfr](pfr) module"]
pub type PFR = crate::Reg<u32, _PFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFR;
#[doc = "`read()` method returns [pfr::R](pfr::R) reader structure"]
impl crate::Readable for PFR {}
#[doc = "`write(|w| ..)` method takes [pfr::W](pfr::W) writer structure"]
impl crate::Writable for PFR {}
#[doc = "Pause Frames Received Register"]
pub mod pfr;
#[doc = "Frames Transmitted Ok Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fto](fto) module"]
pub type FTO = crate::Reg<u32, _FTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FTO;
#[doc = "`read()` method returns [fto::R](fto::R) reader structure"]
impl crate::Readable for FTO {}
#[doc = "`write(|w| ..)` method takes [fto::W](fto::W) writer structure"]
impl crate::Writable for FTO {}
#[doc = "Frames Transmitted Ok Register"]
pub mod fto;
#[doc = "Single Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scf](scf) module"]
pub type SCF = crate::Reg<u32, _SCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCF;
#[doc = "`read()` method returns [scf::R](scf::R) reader structure"]
impl crate::Readable for SCF {}
#[doc = "`write(|w| ..)` method takes [scf::W](scf::W) writer structure"]
impl crate::Writable for SCF {}
#[doc = "Single Collision Frames Register"]
pub mod scf;
#[doc = "Multiple Collision Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcf](mcf) module"]
pub type MCF = crate::Reg<u32, _MCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCF;
#[doc = "`read()` method returns [mcf::R](mcf::R) reader structure"]
impl crate::Readable for MCF {}
#[doc = "`write(|w| ..)` method takes [mcf::W](mcf::W) writer structure"]
impl crate::Writable for MCF {}
#[doc = "Multiple Collision Frames Register"]
pub mod mcf;
#[doc = "Frames Received Ok Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fro](fro) module"]
pub type FRO = crate::Reg<u32, _FRO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRO;
#[doc = "`read()` method returns [fro::R](fro::R) reader structure"]
impl crate::Readable for FRO {}
#[doc = "`write(|w| ..)` method takes [fro::W](fro::W) writer structure"]
impl crate::Writable for FRO {}
#[doc = "Frames Received Ok Register"]
pub mod fro;
#[doc = "Frame Check Sequence Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcse](fcse) module"]
pub type FCSE = crate::Reg<u32, _FCSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCSE;
#[doc = "`read()` method returns [fcse::R](fcse::R) reader structure"]
impl crate::Readable for FCSE {}
#[doc = "`write(|w| ..)` method takes [fcse::W](fcse::W) writer structure"]
impl crate::Writable for FCSE {}
#[doc = "Frame Check Sequence Errors Register"]
pub mod fcse;
#[doc = "Alignment Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ale](ale) module"]
pub type ALE = crate::Reg<u32, _ALE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALE;
#[doc = "`read()` method returns [ale::R](ale::R) reader structure"]
impl crate::Readable for ALE {}
#[doc = "`write(|w| ..)` method takes [ale::W](ale::W) writer structure"]
impl crate::Writable for ALE {}
#[doc = "Alignment Errors Register"]
pub mod ale;
#[doc = "Deferred Transmission Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtf](dtf) module"]
pub type DTF = crate::Reg<u32, _DTF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTF;
#[doc = "`read()` method returns [dtf::R](dtf::R) reader structure"]
impl crate::Readable for DTF {}
#[doc = "`write(|w| ..)` method takes [dtf::W](dtf::W) writer structure"]
impl crate::Writable for DTF {}
#[doc = "Deferred Transmission Frames Register"]
pub mod dtf;
#[doc = "Late Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcol](lcol) module"]
pub type LCOL = crate::Reg<u32, _LCOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCOL;
#[doc = "`read()` method returns [lcol::R](lcol::R) reader structure"]
impl crate::Readable for LCOL {}
#[doc = "`write(|w| ..)` method takes [lcol::W](lcol::W) writer structure"]
impl crate::Writable for LCOL {}
#[doc = "Late Collisions Register"]
pub mod lcol;
#[doc = "Excessive Collisions Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecol](ecol) module"]
pub type ECOL = crate::Reg<u32, _ECOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECOL;
#[doc = "`read()` method returns [ecol::R](ecol::R) reader structure"]
impl crate::Readable for ECOL {}
#[doc = "`write(|w| ..)` method takes [ecol::W](ecol::W) writer structure"]
impl crate::Writable for ECOL {}
#[doc = "Excessive Collisions Register"]
pub mod ecol;
#[doc = "Transmit Underrun Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tund](tund) module"]
pub type TUND = crate::Reg<u32, _TUND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TUND;
#[doc = "`read()` method returns [tund::R](tund::R) reader structure"]
impl crate::Readable for TUND {}
#[doc = "`write(|w| ..)` method takes [tund::W](tund::W) writer structure"]
impl crate::Writable for TUND {}
#[doc = "Transmit Underrun Errors Register"]
pub mod tund;
#[doc = "Carrier Sense Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cse](cse) module"]
pub type CSE = crate::Reg<u32, _CSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSE;
#[doc = "`read()` method returns [cse::R](cse::R) reader structure"]
impl crate::Readable for CSE {}
#[doc = "`write(|w| ..)` method takes [cse::W](cse::W) writer structure"]
impl crate::Writable for CSE {}
#[doc = "Carrier Sense Errors Register"]
pub mod cse;
#[doc = "Receive Resource Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rre](rre) module"]
pub type RRE = crate::Reg<u32, _RRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RRE;
#[doc = "`read()` method returns [rre::R](rre::R) reader structure"]
impl crate::Readable for RRE {}
#[doc = "`write(|w| ..)` method takes [rre::W](rre::W) writer structure"]
impl crate::Writable for RRE {}
#[doc = "Receive Resource Errors Register"]
pub mod rre;
#[doc = "Receive Overrun Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rov](rov) module"]
pub type ROV = crate::Reg<u32, _ROV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROV;
#[doc = "`read()` method returns [rov::R](rov::R) reader structure"]
impl crate::Readable for ROV {}
#[doc = "`write(|w| ..)` method takes [rov::W](rov::W) writer structure"]
impl crate::Writable for ROV {}
#[doc = "Receive Overrun Errors Register"]
pub mod rov;
#[doc = "Receive Symbol Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rse](rse) module"]
pub type RSE = crate::Reg<u32, _RSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSE;
#[doc = "`read()` method returns [rse::R](rse::R) reader structure"]
impl crate::Readable for RSE {}
#[doc = "`write(|w| ..)` method takes [rse::W](rse::W) writer structure"]
impl crate::Writable for RSE {}
#[doc = "Receive Symbol Errors Register"]
pub mod rse;
#[doc = "Excessive Length Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ele](ele) module"]
pub type ELE = crate::Reg<u32, _ELE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ELE;
#[doc = "`read()` method returns [ele::R](ele::R) reader structure"]
impl crate::Readable for ELE {}
#[doc = "`write(|w| ..)` method takes [ele::W](ele::W) writer structure"]
impl crate::Writable for ELE {}
#[doc = "Excessive Length Errors Register"]
pub mod ele;
#[doc = "Receive Jabbers Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rja](rja) module"]
pub type RJA = crate::Reg<u32, _RJA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RJA;
#[doc = "`read()` method returns [rja::R](rja::R) reader structure"]
impl crate::Readable for RJA {}
#[doc = "`write(|w| ..)` method takes [rja::W](rja::W) writer structure"]
impl crate::Writable for RJA {}
#[doc = "Receive Jabbers Register"]
pub mod rja;
#[doc = "Undersize Frames Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usf](usf) module"]
pub type USF = crate::Reg<u32, _USF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USF;
#[doc = "`read()` method returns [usf::R](usf::R) reader structure"]
impl crate::Readable for USF {}
#[doc = "`write(|w| ..)` method takes [usf::W](usf::W) writer structure"]
impl crate::Writable for USF {}
#[doc = "Undersize Frames Register"]
pub mod usf;
#[doc = "SQE Test Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ste](ste) module"]
pub type STE = crate::Reg<u32, _STE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STE;
#[doc = "`read()` method returns [ste::R](ste::R) reader structure"]
impl crate::Readable for STE {}
#[doc = "`write(|w| ..)` method takes [ste::W](ste::W) writer structure"]
impl crate::Writable for STE {}
#[doc = "SQE Test Errors Register"]
pub mod ste;
#[doc = "Received Length Field Mismatch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rle](rle) module"]
pub type RLE = crate::Reg<u32, _RLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RLE;
#[doc = "`read()` method returns [rle::R](rle::R) reader structure"]
impl crate::Readable for RLE {}
#[doc = "`write(|w| ..)` method takes [rle::W](rle::W) writer structure"]
impl crate::Writable for RLE {}
#[doc = "Received Length Field Mismatch Register"]
pub mod rle;
#[doc = "Hash Register Bottom \\[31:0\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrb](hrb) module"]
pub type HRB = crate::Reg<u32, _HRB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRB;
#[doc = "`read()` method returns [hrb::R](hrb::R) reader structure"]
impl crate::Readable for HRB {}
#[doc = "`write(|w| ..)` method takes [hrb::W](hrb::W) writer structure"]
impl crate::Writable for HRB {}
#[doc = "Hash Register Bottom \\[31:0\\]
Register"]
pub mod hrb;
#[doc = "Hash Register Top \\[63:32\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrt](hrt) module"]
pub type HRT = crate::Reg<u32, _HRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRT;
#[doc = "`read()` method returns [hrt::R](hrt::R) reader structure"]
impl crate::Readable for HRT {}
#[doc = "`write(|w| ..)` method takes [hrt::W](hrt::W) writer structure"]
impl crate::Writable for HRT {}
#[doc = "Hash Register Top \\[63:32\\]
Register"]
pub mod hrt;
#[doc = "Specific Address 1 Bottom Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa1b](sa1b) module"]
pub type SA1B = crate::Reg<u32, _SA1B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA1B;
#[doc = "`read()` method returns [sa1b::R](sa1b::R) reader structure"]
impl crate::Readable for SA1B {}
#[doc = "`write(|w| ..)` method takes [sa1b::W](sa1b::W) writer structure"]
impl crate::Writable for SA1B {}
#[doc = "Specific Address 1 Bottom Register"]
pub mod sa1b;
#[doc = "Specific Address 1 Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa1t](sa1t) module"]
pub type SA1T = crate::Reg<u32, _SA1T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA1T;
#[doc = "`read()` method returns [sa1t::R](sa1t::R) reader structure"]
impl crate::Readable for SA1T {}
#[doc = "`write(|w| ..)` method takes [sa1t::W](sa1t::W) writer structure"]
impl crate::Writable for SA1T {}
#[doc = "Specific Address 1 Top Register"]
pub mod sa1t;
#[doc = "Specific Address 2 Bottom Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa2b](sa2b) module"]
pub type SA2B = crate::Reg<u32, _SA2B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA2B;
#[doc = "`read()` method returns [sa2b::R](sa2b::R) reader structure"]
impl crate::Readable for SA2B {}
#[doc = "`write(|w| ..)` method takes [sa2b::W](sa2b::W) writer structure"]
impl crate::Writable for SA2B {}
#[doc = "Specific Address 2 Bottom Register"]
pub mod sa2b;
#[doc = "Specific Address 2 Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa2t](sa2t) module"]
pub type SA2T = crate::Reg<u32, _SA2T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA2T;
#[doc = "`read()` method returns [sa2t::R](sa2t::R) reader structure"]
impl crate::Readable for SA2T {}
#[doc = "`write(|w| ..)` method takes [sa2t::W](sa2t::W) writer structure"]
impl crate::Writable for SA2T {}
#[doc = "Specific Address 2 Top Register"]
pub mod sa2t;
#[doc = "Specific Address 3 Bottom Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa3b](sa3b) module"]
pub type SA3B = crate::Reg<u32, _SA3B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA3B;
#[doc = "`read()` method returns [sa3b::R](sa3b::R) reader structure"]
impl crate::Readable for SA3B {}
#[doc = "`write(|w| ..)` method takes [sa3b::W](sa3b::W) writer structure"]
impl crate::Writable for SA3B {}
#[doc = "Specific Address 3 Bottom Register"]
pub mod sa3b;
#[doc = "Specific Address 3 Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa3t](sa3t) module"]
pub type SA3T = crate::Reg<u32, _SA3T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA3T;
#[doc = "`read()` method returns [sa3t::R](sa3t::R) reader structure"]
impl crate::Readable for SA3T {}
#[doc = "`write(|w| ..)` method takes [sa3t::W](sa3t::W) writer structure"]
impl crate::Writable for SA3T {}
#[doc = "Specific Address 3 Top Register"]
pub mod sa3t;
#[doc = "Specific Address 4 Bottom Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa4b](sa4b) module"]
pub type SA4B = crate::Reg<u32, _SA4B>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA4B;
#[doc = "`read()` method returns [sa4b::R](sa4b::R) reader structure"]
impl crate::Readable for SA4B {}
#[doc = "`write(|w| ..)` method takes [sa4b::W](sa4b::W) writer structure"]
impl crate::Writable for SA4B {}
#[doc = "Specific Address 4 Bottom Register"]
pub mod sa4b;
#[doc = "Specific Address 4 Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa4t](sa4t) module"]
pub type SA4T = crate::Reg<u32, _SA4T>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA4T;
#[doc = "`read()` method returns [sa4t::R](sa4t::R) reader structure"]
impl crate::Readable for SA4T {}
#[doc = "`write(|w| ..)` method takes [sa4t::W](sa4t::W) writer structure"]
impl crate::Writable for SA4T {}
#[doc = "Specific Address 4 Top Register"]
pub mod sa4t;
#[doc = "Type ID Checking Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tid](tid) module"]
pub type TID = crate::Reg<u32, _TID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TID;
#[doc = "`read()` method returns [tid::R](tid::R) reader structure"]
impl crate::Readable for TID {}
#[doc = "`write(|w| ..)` method takes [tid::W](tid::W) writer structure"]
impl crate::Writable for TID {}
#[doc = "Type ID Checking Register"]
pub mod tid;
#[doc = "User Input/Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usrio](usrio) module"]
pub type USRIO = crate::Reg<u32, _USRIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USRIO;
#[doc = "`read()` method returns [usrio::R](usrio::R) reader structure"]
impl crate::Readable for USRIO {}
#[doc = "`write(|w| ..)` method takes [usrio::W](usrio::W) writer structure"]
impl crate::Writable for USRIO {}
#[doc = "User Input/Output Register"]
pub mod usrio;
