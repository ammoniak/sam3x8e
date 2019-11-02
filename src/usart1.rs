#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 4usize],
    _reserved_1_mr: [u8; 4usize],
    _reserved_2_ier: [u8; 4usize],
    _reserved_3_idr: [u8; 4usize],
    _reserved_4_imr: [u8; 4usize],
    _reserved_5_csr: [u8; 4usize],
    #[doc = "0x18 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x1c - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: BRGR,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub rtor: RTOR,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub ttgr: TTGR,
    _reserved11: [u8; 20usize],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub fidi: FIDI,
    #[doc = "0x44 - Number of Errors Register"]
    pub ner: NER,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - IrDA Filter Register"]
    pub if_: IF,
    #[doc = "0x50 - Manchester Configuration Register"]
    pub man: MAN,
    #[doc = "0x54 - LIN Mode Register"]
    pub linmr: LINMR,
    #[doc = "0x58 - LIN Identifier Register"]
    pub linir: LINIR,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub linbrr: LINBRR,
    _reserved18: [u8; 132usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved20: [u8; 20usize],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: RPR,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: RCR,
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: TPR,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: TCR,
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: RNPR,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: RNCR,
    #[doc = "0x118 - Transmit Next Pointer Register"]
    pub tnpr: TNPR,
    #[doc = "0x11c - Transmit Next Counter Register"]
    pub tncr: TNCR,
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: PTCR,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: PTSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_spi_mode(&self) -> &CR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR_SPI_MODE) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_spi_mode_mut(&self) -> &mut CR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CR_SPI_MODE) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr(&self) -> &CR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CR) }
    }
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub fn cr_mut(&self) -> &mut CR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CR) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr_spi_mode(&self) -> &MR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const MR_SPI_MODE) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr_spi_mode_mut(&self) -> &mut MR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut MR_SPI_MODE) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr(&self) -> &MR {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const MR) }
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub fn mr_mut(&self) -> &mut MR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut MR) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_lin_mode(&self) -> &IER_LIN_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER_LIN_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_lin_mode_mut(&self) -> &mut IER_LIN_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER_LIN_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_spi_mode(&self) -> &IER_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER_SPI_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_spi_mode_mut(&self) -> &mut IER_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER_SPI_MODE) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier(&self) -> &IER {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IER) }
    }
    #[doc = "0x08 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_mut(&self) -> &mut IER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IER) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_lin_mode(&self) -> &IDR_LIN_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR_LIN_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_lin_mode_mut(&self) -> &mut IDR_LIN_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR_LIN_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_spi_mode(&self) -> &IDR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR_SPI_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_spi_mode_mut(&self) -> &mut IDR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR_SPI_MODE) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr(&self) -> &IDR {
        unsafe { &*(((self as *const Self) as *const u8).add(12usize) as *const IDR) }
    }
    #[doc = "0x0c - Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr_mut(&self) -> &mut IDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(12usize) as *mut IDR) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_lin_mode(&self) -> &IMR_LIN_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR_LIN_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_lin_mode_mut(&self) -> &mut IMR_LIN_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut IMR_LIN_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_spi_mode(&self) -> &IMR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR_SPI_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_spi_mode_mut(&self) -> &mut IMR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut IMR_SPI_MODE) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr(&self) -> &IMR {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const IMR) }
    }
    #[doc = "0x10 - Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr_mut(&self) -> &mut IMR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut IMR) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_lin_mode(&self) -> &CSR_LIN_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR_LIN_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_lin_mode_mut(&self) -> &mut CSR_LIN_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR_LIN_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_spi_mode(&self) -> &CSR_SPI_MODE {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR_SPI_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_spi_mode_mut(&self) -> &mut CSR_SPI_MODE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR_SPI_MODE) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr(&self) -> &CSR {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const CSR) }
    }
    #[doc = "0x14 - Channel Status Register"]
    #[inline(always)]
    pub fn csr_mut(&self) -> &mut CSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut CSR) }
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cr_spi_mode](cr_spi_mode) module"]
pub type CR_SPI_MODE = crate::Reg<u32, _CR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR_SPI_MODE;
#[doc = "`write(|w| ..)` method takes [cr_spi_mode::W](cr_spi_mode::W) writer structure"]
impl crate::Writable for CR_SPI_MODE {}
#[doc = "Control Register"]
pub mod cr_spi_mode;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mr](mr) module"]
pub type MR = crate::Reg<u32, _MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR;
#[doc = "`read()` method returns [mr::R](mr::R) reader structure"]
impl crate::Readable for MR {}
#[doc = "`write(|w| ..)` method takes [mr::W](mr::W) writer structure"]
impl crate::Writable for MR {}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mr_spi_mode](mr_spi_mode) module"]
pub type MR_SPI_MODE = crate::Reg<u32, _MR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MR_SPI_MODE;
#[doc = "`read()` method returns [mr_spi_mode::R](mr_spi_mode::R) reader structure"]
impl crate::Readable for MR_SPI_MODE {}
#[doc = "`write(|w| ..)` method takes [mr_spi_mode::W](mr_spi_mode::W) writer structure"]
impl crate::Writable for MR_SPI_MODE {}
#[doc = "Mode Register"]
pub mod mr_spi_mode;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier_spi_mode](ier_spi_mode) module"]
pub type IER_SPI_MODE = crate::Reg<u32, _IER_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER_SPI_MODE;
#[doc = "`write(|w| ..)` method takes [ier_spi_mode::W](ier_spi_mode::W) writer structure"]
impl crate::Writable for IER_SPI_MODE {}
#[doc = "Interrupt Enable Register"]
pub mod ier_spi_mode;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier_lin_mode](ier_lin_mode) module"]
pub type IER_LIN_MODE = crate::Reg<u32, _IER_LIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER_LIN_MODE;
#[doc = "`write(|w| ..)` method takes [ier_lin_mode::W](ier_lin_mode::W) writer structure"]
impl crate::Writable for IER_LIN_MODE {}
#[doc = "Interrupt Enable Register"]
pub mod ier_lin_mode;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idr_spi_mode](idr_spi_mode) module"]
pub type IDR_SPI_MODE = crate::Reg<u32, _IDR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR_SPI_MODE;
#[doc = "`write(|w| ..)` method takes [idr_spi_mode::W](idr_spi_mode::W) writer structure"]
impl crate::Writable for IDR_SPI_MODE {}
#[doc = "Interrupt Disable Register"]
pub mod idr_spi_mode;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idr_lin_mode](idr_lin_mode) module"]
pub type IDR_LIN_MODE = crate::Reg<u32, _IDR_LIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR_LIN_MODE;
#[doc = "`write(|w| ..)` method takes [idr_lin_mode::W](idr_lin_mode::W) writer structure"]
impl crate::Writable for IDR_LIN_MODE {}
#[doc = "Interrupt Disable Register"]
pub mod idr_lin_mode;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr_spi_mode](imr_spi_mode) module"]
pub type IMR_SPI_MODE = crate::Reg<u32, _IMR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR_SPI_MODE;
#[doc = "`read()` method returns [imr_spi_mode::R](imr_spi_mode::R) reader structure"]
impl crate::Readable for IMR_SPI_MODE {}
#[doc = "Interrupt Mask Register"]
pub mod imr_spi_mode;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr_lin_mode](imr_lin_mode) module"]
pub type IMR_LIN_MODE = crate::Reg<u32, _IMR_LIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR_LIN_MODE;
#[doc = "`read()` method returns [imr_lin_mode::R](imr_lin_mode::R) reader structure"]
impl crate::Readable for IMR_LIN_MODE {}
#[doc = "Interrupt Mask Register"]
pub mod imr_lin_mode;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "Channel Status Register"]
pub mod csr;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csr_spi_mode](csr_spi_mode) module"]
pub type CSR_SPI_MODE = crate::Reg<u32, _CSR_SPI_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR_SPI_MODE;
#[doc = "`read()` method returns [csr_spi_mode::R](csr_spi_mode::R) reader structure"]
impl crate::Readable for CSR_SPI_MODE {}
#[doc = "Channel Status Register"]
pub mod csr_spi_mode;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [csr_lin_mode](csr_lin_mode) module"]
pub type CSR_LIN_MODE = crate::Reg<u32, _CSR_LIN_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR_LIN_MODE;
#[doc = "`read()` method returns [csr_lin_mode::R](csr_lin_mode::R) reader structure"]
impl crate::Readable for CSR_LIN_MODE {}
#[doc = "Channel Status Register"]
pub mod csr_lin_mode;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rhr](rhr) module"]
pub type RHR = crate::Reg<u32, _RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RHR;
#[doc = "`read()` method returns [rhr::R](rhr::R) reader structure"]
impl crate::Readable for RHR {}
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [brgr](brgr) module"]
pub type BRGR = crate::Reg<u32, _BRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGR;
#[doc = "`read()` method returns [brgr::R](brgr::R) reader structure"]
impl crate::Readable for BRGR {}
#[doc = "`write(|w| ..)` method takes [brgr::W](brgr::W) writer structure"]
impl crate::Writable for BRGR {}
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "Receiver Time-out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rtor](rtor) module"]
pub type RTOR = crate::Reg<u32, _RTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTOR;
#[doc = "`read()` method returns [rtor::R](rtor::R) reader structure"]
impl crate::Readable for RTOR {}
#[doc = "`write(|w| ..)` method takes [rtor::W](rtor::W) writer structure"]
impl crate::Writable for RTOR {}
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "Transmitter Timeguard Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ttgr](ttgr) module"]
pub type TTGR = crate::Reg<u32, _TTGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TTGR;
#[doc = "`read()` method returns [ttgr::R](ttgr::R) reader structure"]
impl crate::Readable for TTGR {}
#[doc = "`write(|w| ..)` method takes [ttgr::W](ttgr::W) writer structure"]
impl crate::Writable for TTGR {}
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "FI DI Ratio Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fidi](fidi) module"]
pub type FIDI = crate::Reg<u32, _FIDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIDI;
#[doc = "`read()` method returns [fidi::R](fidi::R) reader structure"]
impl crate::Readable for FIDI {}
#[doc = "`write(|w| ..)` method takes [fidi::W](fidi::W) writer structure"]
impl crate::Writable for FIDI {}
#[doc = "FI DI Ratio Register"]
pub mod fidi;
#[doc = "Number of Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ner](ner) module"]
pub type NER = crate::Reg<u32, _NER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NER;
#[doc = "`read()` method returns [ner::R](ner::R) reader structure"]
impl crate::Readable for NER {}
#[doc = "Number of Errors Register"]
pub mod ner;
#[doc = "IrDA Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "`write(|w| ..)` method takes [if_::W](if_::W) writer structure"]
impl crate::Writable for IF {}
#[doc = "IrDA Filter Register"]
pub mod if_;
#[doc = "Manchester Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [man](man) module"]
pub type MAN = crate::Reg<u32, _MAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAN;
#[doc = "`read()` method returns [man::R](man::R) reader structure"]
impl crate::Readable for MAN {}
#[doc = "`write(|w| ..)` method takes [man::W](man::W) writer structure"]
impl crate::Writable for MAN {}
#[doc = "Manchester Configuration Register"]
pub mod man;
#[doc = "LIN Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [linmr](linmr) module"]
pub type LINMR = crate::Reg<u32, _LINMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINMR;
#[doc = "`read()` method returns [linmr::R](linmr::R) reader structure"]
impl crate::Readable for LINMR {}
#[doc = "`write(|w| ..)` method takes [linmr::W](linmr::W) writer structure"]
impl crate::Writable for LINMR {}
#[doc = "LIN Mode Register"]
pub mod linmr;
#[doc = "LIN Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [linir](linir) module"]
pub type LINIR = crate::Reg<u32, _LINIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINIR;
#[doc = "`read()` method returns [linir::R](linir::R) reader structure"]
impl crate::Readable for LINIR {}
#[doc = "`write(|w| ..)` method takes [linir::W](linir::W) writer structure"]
impl crate::Writable for LINIR {}
#[doc = "LIN Identifier Register"]
pub mod linir;
#[doc = "LIN Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [linbrr](linbrr) module"]
pub type LINBRR = crate::Reg<u32, _LINBRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINBRR;
#[doc = "`read()` method returns [linbrr::R](linbrr::R) reader structure"]
impl crate::Readable for LINBRR {}
#[doc = "LIN Baud Rate Register"]
pub mod linbrr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "Receive Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rpr](rpr) module"]
pub type RPR = crate::Reg<u32, _RPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPR;
#[doc = "`read()` method returns [rpr::R](rpr::R) reader structure"]
impl crate::Readable for RPR {}
#[doc = "`write(|w| ..)` method takes [rpr::W](rpr::W) writer structure"]
impl crate::Writable for RPR {}
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "Receive Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rcr](rcr) module"]
pub type RCR = crate::Reg<u32, _RCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCR;
#[doc = "`read()` method returns [rcr::R](rcr::R) reader structure"]
impl crate::Readable for RCR {}
#[doc = "`write(|w| ..)` method takes [rcr::W](rcr::W) writer structure"]
impl crate::Writable for RCR {}
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "Transmit Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tpr](tpr) module"]
pub type TPR = crate::Reg<u32, _TPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TPR;
#[doc = "`read()` method returns [tpr::R](tpr::R) reader structure"]
impl crate::Readable for TPR {}
#[doc = "`write(|w| ..)` method takes [tpr::W](tpr::W) writer structure"]
impl crate::Writable for TPR {}
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "Transmit Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tcr](tcr) module"]
pub type TCR = crate::Reg<u32, _TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCR;
#[doc = "`read()` method returns [tcr::R](tcr::R) reader structure"]
impl crate::Readable for TCR {}
#[doc = "`write(|w| ..)` method takes [tcr::W](tcr::W) writer structure"]
impl crate::Writable for TCR {}
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "Receive Next Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rnpr](rnpr) module"]
pub type RNPR = crate::Reg<u32, _RNPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNPR;
#[doc = "`read()` method returns [rnpr::R](rnpr::R) reader structure"]
impl crate::Readable for RNPR {}
#[doc = "`write(|w| ..)` method takes [rnpr::W](rnpr::W) writer structure"]
impl crate::Writable for RNPR {}
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "Receive Next Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rncr](rncr) module"]
pub type RNCR = crate::Reg<u32, _RNCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNCR;
#[doc = "`read()` method returns [rncr::R](rncr::R) reader structure"]
impl crate::Readable for RNCR {}
#[doc = "`write(|w| ..)` method takes [rncr::W](rncr::W) writer structure"]
impl crate::Writable for RNCR {}
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "Transmit Next Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tnpr](tnpr) module"]
pub type TNPR = crate::Reg<u32, _TNPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TNPR;
#[doc = "`read()` method returns [tnpr::R](tnpr::R) reader structure"]
impl crate::Readable for TNPR {}
#[doc = "`write(|w| ..)` method takes [tnpr::W](tnpr::W) writer structure"]
impl crate::Writable for TNPR {}
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "Transmit Next Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tncr](tncr) module"]
pub type TNCR = crate::Reg<u32, _TNCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TNCR;
#[doc = "`read()` method returns [tncr::R](tncr::R) reader structure"]
impl crate::Readable for TNCR {}
#[doc = "`write(|w| ..)` method takes [tncr::W](tncr::W) writer structure"]
impl crate::Writable for TNCR {}
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "Transfer Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ptcr](ptcr) module"]
pub type PTCR = crate::Reg<u32, _PTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTCR;
#[doc = "`write(|w| ..)` method takes [ptcr::W](ptcr::W) writer structure"]
impl crate::Writable for PTCR {}
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "Transfer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ptsr](ptsr) module"]
pub type PTSR = crate::Reg<u32, _PTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTSR;
#[doc = "`read()` method returns [ptsr::R](ptsr::R) reader structure"]
impl crate::Readable for PTSR {}
#[doc = "Transfer Status Register"]
pub mod ptsr;
