#[doc = r"Register block"]
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
    _reserved5: [u8; 4usize],
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
    _reserved12: [u8; 8usize],
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
    _reserved18: [u8; 16usize],
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
    _reserved24: [u8; 16usize],
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
    _reserved30: [u8; 16usize],
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
    _reserved36: [u8; 16usize],
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
    _reserved42: [u8; 16usize],
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
    _reserved48: [u8; 200usize],
    #[doc = "0x1e4 - DMAC Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0x1e8 - DMAC Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "DMAC Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gcfg](gcfg) module"]
pub type GCFG = crate::Reg<u32, _GCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GCFG;
#[doc = "`read()` method returns [gcfg::R](gcfg::R) reader structure"]
impl crate::Readable for GCFG {}
#[doc = "`write(|w| ..)` method takes [gcfg::W](gcfg::W) writer structure"]
impl crate::Writable for GCFG {}
#[doc = "DMAC Global Configuration Register"]
pub mod gcfg;
#[doc = "DMAC Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [en](en) module"]
pub type EN = crate::Reg<u32, _EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EN;
#[doc = "`read()` method returns [en::R](en::R) reader structure"]
impl crate::Readable for EN {}
#[doc = "`write(|w| ..)` method takes [en::W](en::W) writer structure"]
impl crate::Writable for EN {}
#[doc = "DMAC Enable Register"]
pub mod en;
#[doc = "DMAC Software Single Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sreq](sreq) module"]
pub type SREQ = crate::Reg<u32, _SREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SREQ;
#[doc = "`read()` method returns [sreq::R](sreq::R) reader structure"]
impl crate::Readable for SREQ {}
#[doc = "`write(|w| ..)` method takes [sreq::W](sreq::W) writer structure"]
impl crate::Writable for SREQ {}
#[doc = "DMAC Software Single Request Register"]
pub mod sreq;
#[doc = "DMAC Software Chunk Transfer Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [creq](creq) module"]
pub type CREQ = crate::Reg<u32, _CREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CREQ;
#[doc = "`read()` method returns [creq::R](creq::R) reader structure"]
impl crate::Readable for CREQ {}
#[doc = "`write(|w| ..)` method takes [creq::W](creq::W) writer structure"]
impl crate::Writable for CREQ {}
#[doc = "DMAC Software Chunk Transfer Request Register"]
pub mod creq;
#[doc = "DMAC Software Last Transfer Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [last](last) module"]
pub type LAST = crate::Reg<u32, _LAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LAST;
#[doc = "`read()` method returns [last::R](last::R) reader structure"]
impl crate::Readable for LAST {}
#[doc = "`write(|w| ..)` method takes [last::W](last::W) writer structure"]
impl crate::Writable for LAST {}
#[doc = "DMAC Software Last Transfer Flag Register"]
pub mod last;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ebcier](ebcier) module"]
pub type EBCIER = crate::Reg<u32, _EBCIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBCIER;
#[doc = "`write(|w| ..)` method takes [ebcier::W](ebcier::W) writer structure"]
impl crate::Writable for EBCIER {}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Enable register."]
pub mod ebcier;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ebcidr](ebcidr) module"]
pub type EBCIDR = crate::Reg<u32, _EBCIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBCIDR;
#[doc = "`write(|w| ..)` method takes [ebcidr::W](ebcidr::W) writer structure"]
impl crate::Writable for EBCIDR {}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register."]
pub mod ebcidr;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ebcimr](ebcimr) module"]
pub type EBCIMR = crate::Reg<u32, _EBCIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBCIMR;
#[doc = "`read()` method returns [ebcimr::R](ebcimr::R) reader structure"]
impl crate::Readable for EBCIMR {}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Mask Register."]
pub mod ebcimr;
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ebcisr](ebcisr) module"]
pub type EBCISR = crate::Reg<u32, _EBCISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBCISR;
#[doc = "`read()` method returns [ebcisr::R](ebcisr::R) reader structure"]
impl crate::Readable for EBCISR {}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register."]
pub mod ebcisr;
#[doc = "DMAC Channel Handler Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cher](cher) module"]
pub type CHER = crate::Reg<u32, _CHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHER;
#[doc = "`write(|w| ..)` method takes [cher::W](cher::W) writer structure"]
impl crate::Writable for CHER {}
#[doc = "DMAC Channel Handler Enable Register"]
pub mod cher;
#[doc = "DMAC Channel Handler Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chdr](chdr) module"]
pub type CHDR = crate::Reg<u32, _CHDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDR;
#[doc = "`write(|w| ..)` method takes [chdr::W](chdr::W) writer structure"]
impl crate::Writable for CHDR {}
#[doc = "DMAC Channel Handler Disable Register"]
pub mod chdr;
#[doc = "DMAC Channel Handler Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [chsr](chsr) module"]
pub type CHSR = crate::Reg<u32, _CHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHSR;
#[doc = "`read()` method returns [chsr::R](chsr::R) reader structure"]
impl crate::Readable for CHSR {}
#[doc = "DMAC Channel Handler Status Register"]
pub mod chsr;
#[doc = "DMAC Channel Source Address Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saddr0](saddr0) module"]
pub type SADDR0 = crate::Reg<u32, _SADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SADDR0;
#[doc = "`read()` method returns [saddr0::R](saddr0::R) reader structure"]
impl crate::Readable for SADDR0 {}
#[doc = "`write(|w| ..)` method takes [saddr0::W](saddr0::W) writer structure"]
impl crate::Writable for SADDR0 {}
#[doc = "DMAC Channel Source Address Register (ch_num = 0)"]
pub mod saddr0;
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daddr0](daddr0) module"]
pub type DADDR0 = crate::Reg<u32, _DADDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR0;
#[doc = "`read()` method returns [daddr0::R](daddr0::R) reader structure"]
impl crate::Readable for DADDR0 {}
#[doc = "`write(|w| ..)` method takes [daddr0::W](daddr0::W) writer structure"]
impl crate::Writable for DADDR0 {}
#[doc = "DMAC Channel Destination Address Register (ch_num = 0)"]
pub mod daddr0;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dscr0](dscr0) module"]
pub type DSCR0 = crate::Reg<u32, _DSCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCR0;
#[doc = "`read()` method returns [dscr0::R](dscr0::R) reader structure"]
impl crate::Readable for DSCR0 {}
#[doc = "`write(|w| ..)` method takes [dscr0::W](dscr0::W) writer structure"]
impl crate::Writable for DSCR0 {}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 0)"]
pub mod dscr0;
#[doc = "DMAC Channel Control A Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla0](ctrla0) module"]
pub type CTRLA0 = crate::Reg<u32, _CTRLA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA0;
#[doc = "`read()` method returns [ctrla0::R](ctrla0::R) reader structure"]
impl crate::Readable for CTRLA0 {}
#[doc = "`write(|w| ..)` method takes [ctrla0::W](ctrla0::W) writer structure"]
impl crate::Writable for CTRLA0 {}
#[doc = "DMAC Channel Control A Register (ch_num = 0)"]
pub mod ctrla0;
#[doc = "DMAC Channel Control B Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlb0](ctrlb0) module"]
pub type CTRLB0 = crate::Reg<u32, _CTRLB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB0;
#[doc = "`read()` method returns [ctrlb0::R](ctrlb0::R) reader structure"]
impl crate::Readable for CTRLB0 {}
#[doc = "`write(|w| ..)` method takes [ctrlb0::W](ctrlb0::W) writer structure"]
impl crate::Writable for CTRLB0 {}
#[doc = "DMAC Channel Control B Register (ch_num = 0)"]
pub mod ctrlb0;
#[doc = "DMAC Channel Configuration Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg0](cfg0) module"]
pub type CFG0 = crate::Reg<u32, _CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG0;
#[doc = "`read()` method returns [cfg0::R](cfg0::R) reader structure"]
impl crate::Readable for CFG0 {}
#[doc = "`write(|w| ..)` method takes [cfg0::W](cfg0::W) writer structure"]
impl crate::Writable for CFG0 {}
#[doc = "DMAC Channel Configuration Register (ch_num = 0)"]
pub mod cfg0;
#[doc = "DMAC Channel Source Address Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saddr1](saddr1) module"]
pub type SADDR1 = crate::Reg<u32, _SADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SADDR1;
#[doc = "`read()` method returns [saddr1::R](saddr1::R) reader structure"]
impl crate::Readable for SADDR1 {}
#[doc = "`write(|w| ..)` method takes [saddr1::W](saddr1::W) writer structure"]
impl crate::Writable for SADDR1 {}
#[doc = "DMAC Channel Source Address Register (ch_num = 1)"]
pub mod saddr1;
#[doc = "DMAC Channel Destination Address Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daddr1](daddr1) module"]
pub type DADDR1 = crate::Reg<u32, _DADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR1;
#[doc = "`read()` method returns [daddr1::R](daddr1::R) reader structure"]
impl crate::Readable for DADDR1 {}
#[doc = "`write(|w| ..)` method takes [daddr1::W](daddr1::W) writer structure"]
impl crate::Writable for DADDR1 {}
#[doc = "DMAC Channel Destination Address Register (ch_num = 1)"]
pub mod daddr1;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dscr1](dscr1) module"]
pub type DSCR1 = crate::Reg<u32, _DSCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCR1;
#[doc = "`read()` method returns [dscr1::R](dscr1::R) reader structure"]
impl crate::Readable for DSCR1 {}
#[doc = "`write(|w| ..)` method takes [dscr1::W](dscr1::W) writer structure"]
impl crate::Writable for DSCR1 {}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 1)"]
pub mod dscr1;
#[doc = "DMAC Channel Control A Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla1](ctrla1) module"]
pub type CTRLA1 = crate::Reg<u32, _CTRLA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA1;
#[doc = "`read()` method returns [ctrla1::R](ctrla1::R) reader structure"]
impl crate::Readable for CTRLA1 {}
#[doc = "`write(|w| ..)` method takes [ctrla1::W](ctrla1::W) writer structure"]
impl crate::Writable for CTRLA1 {}
#[doc = "DMAC Channel Control A Register (ch_num = 1)"]
pub mod ctrla1;
#[doc = "DMAC Channel Control B Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlb1](ctrlb1) module"]
pub type CTRLB1 = crate::Reg<u32, _CTRLB1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB1;
#[doc = "`read()` method returns [ctrlb1::R](ctrlb1::R) reader structure"]
impl crate::Readable for CTRLB1 {}
#[doc = "`write(|w| ..)` method takes [ctrlb1::W](ctrlb1::W) writer structure"]
impl crate::Writable for CTRLB1 {}
#[doc = "DMAC Channel Control B Register (ch_num = 1)"]
pub mod ctrlb1;
#[doc = "DMAC Channel Configuration Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "DMAC Channel Configuration Register (ch_num = 1)"]
pub mod cfg1;
#[doc = "DMAC Channel Source Address Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saddr2](saddr2) module"]
pub type SADDR2 = crate::Reg<u32, _SADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SADDR2;
#[doc = "`read()` method returns [saddr2::R](saddr2::R) reader structure"]
impl crate::Readable for SADDR2 {}
#[doc = "`write(|w| ..)` method takes [saddr2::W](saddr2::W) writer structure"]
impl crate::Writable for SADDR2 {}
#[doc = "DMAC Channel Source Address Register (ch_num = 2)"]
pub mod saddr2;
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daddr2](daddr2) module"]
pub type DADDR2 = crate::Reg<u32, _DADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR2;
#[doc = "`read()` method returns [daddr2::R](daddr2::R) reader structure"]
impl crate::Readable for DADDR2 {}
#[doc = "`write(|w| ..)` method takes [daddr2::W](daddr2::W) writer structure"]
impl crate::Writable for DADDR2 {}
#[doc = "DMAC Channel Destination Address Register (ch_num = 2)"]
pub mod daddr2;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dscr2](dscr2) module"]
pub type DSCR2 = crate::Reg<u32, _DSCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCR2;
#[doc = "`read()` method returns [dscr2::R](dscr2::R) reader structure"]
impl crate::Readable for DSCR2 {}
#[doc = "`write(|w| ..)` method takes [dscr2::W](dscr2::W) writer structure"]
impl crate::Writable for DSCR2 {}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 2)"]
pub mod dscr2;
#[doc = "DMAC Channel Control A Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla2](ctrla2) module"]
pub type CTRLA2 = crate::Reg<u32, _CTRLA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA2;
#[doc = "`read()` method returns [ctrla2::R](ctrla2::R) reader structure"]
impl crate::Readable for CTRLA2 {}
#[doc = "`write(|w| ..)` method takes [ctrla2::W](ctrla2::W) writer structure"]
impl crate::Writable for CTRLA2 {}
#[doc = "DMAC Channel Control A Register (ch_num = 2)"]
pub mod ctrla2;
#[doc = "DMAC Channel Control B Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlb2](ctrlb2) module"]
pub type CTRLB2 = crate::Reg<u32, _CTRLB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB2;
#[doc = "`read()` method returns [ctrlb2::R](ctrlb2::R) reader structure"]
impl crate::Readable for CTRLB2 {}
#[doc = "`write(|w| ..)` method takes [ctrlb2::W](ctrlb2::W) writer structure"]
impl crate::Writable for CTRLB2 {}
#[doc = "DMAC Channel Control B Register (ch_num = 2)"]
pub mod ctrlb2;
#[doc = "DMAC Channel Configuration Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg2](cfg2) module"]
pub type CFG2 = crate::Reg<u32, _CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG2;
#[doc = "`read()` method returns [cfg2::R](cfg2::R) reader structure"]
impl crate::Readable for CFG2 {}
#[doc = "`write(|w| ..)` method takes [cfg2::W](cfg2::W) writer structure"]
impl crate::Writable for CFG2 {}
#[doc = "DMAC Channel Configuration Register (ch_num = 2)"]
pub mod cfg2;
#[doc = "DMAC Channel Source Address Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saddr3](saddr3) module"]
pub type SADDR3 = crate::Reg<u32, _SADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SADDR3;
#[doc = "`read()` method returns [saddr3::R](saddr3::R) reader structure"]
impl crate::Readable for SADDR3 {}
#[doc = "`write(|w| ..)` method takes [saddr3::W](saddr3::W) writer structure"]
impl crate::Writable for SADDR3 {}
#[doc = "DMAC Channel Source Address Register (ch_num = 3)"]
pub mod saddr3;
#[doc = "DMAC Channel Destination Address Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daddr3](daddr3) module"]
pub type DADDR3 = crate::Reg<u32, _DADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR3;
#[doc = "`read()` method returns [daddr3::R](daddr3::R) reader structure"]
impl crate::Readable for DADDR3 {}
#[doc = "`write(|w| ..)` method takes [daddr3::W](daddr3::W) writer structure"]
impl crate::Writable for DADDR3 {}
#[doc = "DMAC Channel Destination Address Register (ch_num = 3)"]
pub mod daddr3;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dscr3](dscr3) module"]
pub type DSCR3 = crate::Reg<u32, _DSCR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCR3;
#[doc = "`read()` method returns [dscr3::R](dscr3::R) reader structure"]
impl crate::Readable for DSCR3 {}
#[doc = "`write(|w| ..)` method takes [dscr3::W](dscr3::W) writer structure"]
impl crate::Writable for DSCR3 {}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)"]
pub mod dscr3;
#[doc = "DMAC Channel Control A Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla3](ctrla3) module"]
pub type CTRLA3 = crate::Reg<u32, _CTRLA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA3;
#[doc = "`read()` method returns [ctrla3::R](ctrla3::R) reader structure"]
impl crate::Readable for CTRLA3 {}
#[doc = "`write(|w| ..)` method takes [ctrla3::W](ctrla3::W) writer structure"]
impl crate::Writable for CTRLA3 {}
#[doc = "DMAC Channel Control A Register (ch_num = 3)"]
pub mod ctrla3;
#[doc = "DMAC Channel Control B Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlb3](ctrlb3) module"]
pub type CTRLB3 = crate::Reg<u32, _CTRLB3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB3;
#[doc = "`read()` method returns [ctrlb3::R](ctrlb3::R) reader structure"]
impl crate::Readable for CTRLB3 {}
#[doc = "`write(|w| ..)` method takes [ctrlb3::W](ctrlb3::W) writer structure"]
impl crate::Writable for CTRLB3 {}
#[doc = "DMAC Channel Control B Register (ch_num = 3)"]
pub mod ctrlb3;
#[doc = "DMAC Channel Configuration Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg3](cfg3) module"]
pub type CFG3 = crate::Reg<u32, _CFG3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG3;
#[doc = "`read()` method returns [cfg3::R](cfg3::R) reader structure"]
impl crate::Readable for CFG3 {}
#[doc = "`write(|w| ..)` method takes [cfg3::W](cfg3::W) writer structure"]
impl crate::Writable for CFG3 {}
#[doc = "DMAC Channel Configuration Register (ch_num = 3)"]
pub mod cfg3;
#[doc = "DMAC Channel Source Address Register (ch_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saddr4](saddr4) module"]
pub type SADDR4 = crate::Reg<u32, _SADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SADDR4;
#[doc = "`read()` method returns [saddr4::R](saddr4::R) reader structure"]
impl crate::Readable for SADDR4 {}
#[doc = "`write(|w| ..)` method takes [saddr4::W](saddr4::W) writer structure"]
impl crate::Writable for SADDR4 {}
#[doc = "DMAC Channel Source Address Register (ch_num = 4)"]
pub mod saddr4;
#[doc = "DMAC Channel Destination Address Register (ch_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daddr4](daddr4) module"]
pub type DADDR4 = crate::Reg<u32, _DADDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR4;
#[doc = "`read()` method returns [daddr4::R](daddr4::R) reader structure"]
impl crate::Readable for DADDR4 {}
#[doc = "`write(|w| ..)` method takes [daddr4::W](daddr4::W) writer structure"]
impl crate::Writable for DADDR4 {}
#[doc = "DMAC Channel Destination Address Register (ch_num = 4)"]
pub mod daddr4;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dscr4](dscr4) module"]
pub type DSCR4 = crate::Reg<u32, _DSCR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCR4;
#[doc = "`read()` method returns [dscr4::R](dscr4::R) reader structure"]
impl crate::Readable for DSCR4 {}
#[doc = "`write(|w| ..)` method takes [dscr4::W](dscr4::W) writer structure"]
impl crate::Writable for DSCR4 {}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 4)"]
pub mod dscr4;
#[doc = "DMAC Channel Control A Register (ch_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla4](ctrla4) module"]
pub type CTRLA4 = crate::Reg<u32, _CTRLA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA4;
#[doc = "`read()` method returns [ctrla4::R](ctrla4::R) reader structure"]
impl crate::Readable for CTRLA4 {}
#[doc = "`write(|w| ..)` method takes [ctrla4::W](ctrla4::W) writer structure"]
impl crate::Writable for CTRLA4 {}
#[doc = "DMAC Channel Control A Register (ch_num = 4)"]
pub mod ctrla4;
#[doc = "DMAC Channel Control B Register (ch_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlb4](ctrlb4) module"]
pub type CTRLB4 = crate::Reg<u32, _CTRLB4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB4;
#[doc = "`read()` method returns [ctrlb4::R](ctrlb4::R) reader structure"]
impl crate::Readable for CTRLB4 {}
#[doc = "`write(|w| ..)` method takes [ctrlb4::W](ctrlb4::W) writer structure"]
impl crate::Writable for CTRLB4 {}
#[doc = "DMAC Channel Control B Register (ch_num = 4)"]
pub mod ctrlb4;
#[doc = "DMAC Channel Configuration Register (ch_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg4](cfg4) module"]
pub type CFG4 = crate::Reg<u32, _CFG4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG4;
#[doc = "`read()` method returns [cfg4::R](cfg4::R) reader structure"]
impl crate::Readable for CFG4 {}
#[doc = "`write(|w| ..)` method takes [cfg4::W](cfg4::W) writer structure"]
impl crate::Writable for CFG4 {}
#[doc = "DMAC Channel Configuration Register (ch_num = 4)"]
pub mod cfg4;
#[doc = "DMAC Channel Source Address Register (ch_num = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [saddr5](saddr5) module"]
pub type SADDR5 = crate::Reg<u32, _SADDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SADDR5;
#[doc = "`read()` method returns [saddr5::R](saddr5::R) reader structure"]
impl crate::Readable for SADDR5 {}
#[doc = "`write(|w| ..)` method takes [saddr5::W](saddr5::W) writer structure"]
impl crate::Writable for SADDR5 {}
#[doc = "DMAC Channel Source Address Register (ch_num = 5)"]
pub mod saddr5;
#[doc = "DMAC Channel Destination Address Register (ch_num = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [daddr5](daddr5) module"]
pub type DADDR5 = crate::Reg<u32, _DADDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DADDR5;
#[doc = "`read()` method returns [daddr5::R](daddr5::R) reader structure"]
impl crate::Readable for DADDR5 {}
#[doc = "`write(|w| ..)` method takes [daddr5::W](daddr5::W) writer structure"]
impl crate::Writable for DADDR5 {}
#[doc = "DMAC Channel Destination Address Register (ch_num = 5)"]
pub mod daddr5;
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dscr5](dscr5) module"]
pub type DSCR5 = crate::Reg<u32, _DSCR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCR5;
#[doc = "`read()` method returns [dscr5::R](dscr5::R) reader structure"]
impl crate::Readable for DSCR5 {}
#[doc = "`write(|w| ..)` method takes [dscr5::W](dscr5::W) writer structure"]
impl crate::Writable for DSCR5 {}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 5)"]
pub mod dscr5;
#[doc = "DMAC Channel Control A Register (ch_num = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrla5](ctrla5) module"]
pub type CTRLA5 = crate::Reg<u32, _CTRLA5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLA5;
#[doc = "`read()` method returns [ctrla5::R](ctrla5::R) reader structure"]
impl crate::Readable for CTRLA5 {}
#[doc = "`write(|w| ..)` method takes [ctrla5::W](ctrla5::W) writer structure"]
impl crate::Writable for CTRLA5 {}
#[doc = "DMAC Channel Control A Register (ch_num = 5)"]
pub mod ctrla5;
#[doc = "DMAC Channel Control B Register (ch_num = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrlb5](ctrlb5) module"]
pub type CTRLB5 = crate::Reg<u32, _CTRLB5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRLB5;
#[doc = "`read()` method returns [ctrlb5::R](ctrlb5::R) reader structure"]
impl crate::Readable for CTRLB5 {}
#[doc = "`write(|w| ..)` method takes [ctrlb5::W](ctrlb5::W) writer structure"]
impl crate::Writable for CTRLB5 {}
#[doc = "DMAC Channel Control B Register (ch_num = 5)"]
pub mod ctrlb5;
#[doc = "DMAC Channel Configuration Register (ch_num = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg5](cfg5) module"]
pub type CFG5 = crate::Reg<u32, _CFG5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG5;
#[doc = "`read()` method returns [cfg5::R](cfg5::R) reader structure"]
impl crate::Readable for CFG5 {}
#[doc = "`write(|w| ..)` method takes [cfg5::W](cfg5::W) writer structure"]
impl crate::Writable for CFG5 {}
#[doc = "DMAC Channel Configuration Register (ch_num = 5)"]
pub mod cfg5;
#[doc = "DMAC Write Protect Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wpmr](wpmr) module"]
pub type WPMR = crate::Reg<u32, _WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPMR;
#[doc = "`read()` method returns [wpmr::R](wpmr::R) reader structure"]
impl crate::Readable for WPMR {}
#[doc = "`write(|w| ..)` method takes [wpmr::W](wpmr::W) writer structure"]
impl crate::Writable for WPMR {}
#[doc = "DMAC Write Protect Mode Register"]
pub mod wpmr;
#[doc = "DMAC Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wpsr](wpsr) module"]
pub type WPSR = crate::Reg<u32, _WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WPSR;
#[doc = "`read()` method returns [wpsr::R](wpsr::R) reader structure"]
impl crate::Readable for WPSR {}
#[doc = "DMAC Write Protect Status Register"]
pub mod wpsr;
