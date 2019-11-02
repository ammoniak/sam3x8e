#[doc = "Reader of register QISR"]
pub type R = crate::R<u32, super::QISR>;
#[doc = "Reader of field `IDX`"]
pub type IDX_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIRCHG`"]
pub type DIRCHG_R = crate::R<bool, bool>;
#[doc = "Reader of field `QERR`"]
pub type QERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - InDeX"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIRection CHanGe"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Quadrature ERRor"]
    #[inline(always)]
    pub fn qerr(&self) -> QERR_R {
        QERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIRection"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
