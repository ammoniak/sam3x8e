#[doc = "Reader of register CKGR_MCFR"]
pub type R = crate::R<u32, super::CKGR_MCFR>;
#[doc = "Reader of field `MAINF`"]
pub type MAINF_R = crate::R<u16, u16>;
#[doc = "Reader of field `MAINFRDY`"]
pub type MAINFRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Main Clock Frequency"]
    #[inline(always)]
    pub fn mainf(&self) -> MAINF_R {
        MAINF_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Main Clock Ready"]
    #[inline(always)]
    pub fn mainfrdy(&self) -> MAINFRDY_R {
        MAINFRDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
