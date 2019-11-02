#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Data Timeout Register"]
    pub dtor: DTOR,
    #[doc = "0x0c - SD/SDIO Card Register"]
    pub sdcr: SDCR,
    #[doc = "0x10 - Argument Register"]
    pub argr: ARGR,
    #[doc = "0x14 - Command Register"]
    pub cmdr: CMDR,
    #[doc = "0x18 - Block Register"]
    pub blkr: BLKR,
    #[doc = "0x1c - Completion Signal Timeout Register"]
    pub cstor: CSTOR,
    #[doc = "0x20 - Response Register"]
    pub rspr: [RSPR; 4],
    #[doc = "0x30 - Receive Data Register"]
    pub rdr: RDR,
    #[doc = "0x34 - Transmit Data Register"]
    pub tdr: TDR,
    _reserved11: [u8; 8usize],
    #[doc = "0x40 - Status Register"]
    pub sr: SR,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x50 - DMA Configuration Register"]
    pub dma: DMA,
    #[doc = "0x54 - Configuration Register"]
    pub cfg: CFG,
    _reserved17: [u8; 140usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved19: [u8; 276usize],
    #[doc = "0x200 - FIFO Memory Aperture0"]
    pub fifo: [FIFO; 256],
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
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mr](mr) module"]
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
#[doc = "Data Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dtor](dtor) module"]
pub type DTOR = crate::Reg<u32, _DTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTOR;
#[doc = "`read()` method returns [dtor::R](dtor::R) reader structure"]
impl crate::Readable for DTOR {}
#[doc = "`write(|w| ..)` method takes [dtor::W](dtor::W) writer structure"]
impl crate::Writable for DTOR {}
#[doc = "Data Timeout Register"]
pub mod dtor;
#[doc = "SD/SDIO Card Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdcr](sdcr) module"]
pub type SDCR = crate::Reg<u32, _SDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDCR;
#[doc = "`read()` method returns [sdcr::R](sdcr::R) reader structure"]
impl crate::Readable for SDCR {}
#[doc = "`write(|w| ..)` method takes [sdcr::W](sdcr::W) writer structure"]
impl crate::Writable for SDCR {}
#[doc = "SD/SDIO Card Register"]
pub mod sdcr;
#[doc = "Argument Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [argr](argr) module"]
pub type ARGR = crate::Reg<u32, _ARGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ARGR;
#[doc = "`read()` method returns [argr::R](argr::R) reader structure"]
impl crate::Readable for ARGR {}
#[doc = "`write(|w| ..)` method takes [argr::W](argr::W) writer structure"]
impl crate::Writable for ARGR {}
#[doc = "Argument Register"]
pub mod argr;
#[doc = "Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdr](cmdr) module"]
pub type CMDR = crate::Reg<u32, _CMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDR;
#[doc = "`write(|w| ..)` method takes [cmdr::W](cmdr::W) writer structure"]
impl crate::Writable for CMDR {}
#[doc = "Command Register"]
pub mod cmdr;
#[doc = "Block Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blkr](blkr) module"]
pub type BLKR = crate::Reg<u32, _BLKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLKR;
#[doc = "`read()` method returns [blkr::R](blkr::R) reader structure"]
impl crate::Readable for BLKR {}
#[doc = "`write(|w| ..)` method takes [blkr::W](blkr::W) writer structure"]
impl crate::Writable for BLKR {}
#[doc = "Block Register"]
pub mod blkr;
#[doc = "Completion Signal Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cstor](cstor) module"]
pub type CSTOR = crate::Reg<u32, _CSTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSTOR;
#[doc = "`read()` method returns [cstor::R](cstor::R) reader structure"]
impl crate::Readable for CSTOR {}
#[doc = "`write(|w| ..)` method takes [cstor::W](cstor::W) writer structure"]
impl crate::Writable for CSTOR {}
#[doc = "Completion Signal Timeout Register"]
pub mod cstor;
#[doc = "Response Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rspr](rspr) module"]
pub type RSPR = crate::Reg<u32, _RSPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSPR;
#[doc = "`read()` method returns [rspr::R](rspr::R) reader structure"]
impl crate::Readable for RSPR {}
#[doc = "Response Register"]
pub mod rspr;
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rdr](rdr) module"]
pub type RDR = crate::Reg<u32, _RDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDR;
#[doc = "`read()` method returns [rdr::R](rdr::R) reader structure"]
impl crate::Readable for RDR {}
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "Transmit Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tdr](tdr) module"]
pub type TDR = crate::Reg<u32, _TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TDR;
#[doc = "`write(|w| ..)` method takes [tdr::W](tdr::W) writer structure"]
impl crate::Writable for TDR {}
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dma](dma) module"]
pub type DMA = crate::Reg<u32, _DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMA;
#[doc = "`read()` method returns [dma::R](dma::R) reader structure"]
impl crate::Readable for DMA {}
#[doc = "`write(|w| ..)` method takes [dma::W](dma::W) writer structure"]
impl crate::Writable for DMA {}
#[doc = "DMA Configuration Register"]
pub mod dma;
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wpmr](wpmr) module"]
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
#[doc = "FIFO Memory Aperture0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "FIFO Memory Aperture0"]
pub mod fifo;
