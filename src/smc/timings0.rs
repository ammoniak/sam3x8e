#[doc = "Reader of register TIMINGS0"]
pub type R = crate::R<u32, super::TIMINGS0>;
#[doc = "Writer for register TIMINGS0"]
pub type W = crate::W<u32, super::TIMINGS0>;
#[doc = "Register TIMINGS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMINGS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TCLR`"]
pub type TCLR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCLR`"]
pub struct TCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TADL`"]
pub type TADL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TADL`"]
pub struct TADL_W<'a> {
    w: &'a mut W,
}
impl<'a> TADL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `TAR`"]
pub type TAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TAR`"]
pub struct TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `OCMS`"]
pub type OCMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OCMS`"]
pub struct OCMS_W<'a> {
    w: &'a mut W,
}
impl<'a> OCMS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TRR`"]
pub type TRR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRR`"]
pub struct TRR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TWB`"]
pub type TWB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TWB`"]
pub struct TWB_W<'a> {
    w: &'a mut W,
}
impl<'a> TWB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RBNSEL`"]
pub type RBNSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RBNSEL`"]
pub struct RBNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RBNSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Reader of field `NFSEL`"]
pub type NFSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NFSEL`"]
pub struct NFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NFSEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CLE to REN Low Delay"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ALE to Data Start"]
    #[inline(always)]
    pub fn tadl(&self) -> TADL_R {
        TADL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ALE to REN Low Delay"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Off Chip Memory Scrambling Enable"]
    #[inline(always)]
    pub fn ocms(&self) -> OCMS_R {
        OCMS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Ready to REN Low Delay"]
    #[inline(always)]
    pub fn trr(&self) -> TRR_R {
        TRR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - WEN High to REN to Busy"]
    #[inline(always)]
    pub fn twb(&self) -> TWB_R {
        TWB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Ready/Busy Line Selection"]
    #[inline(always)]
    pub fn rbnsel(&self) -> RBNSEL_R {
        RBNSEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - NAND Flash Selection"]
    #[inline(always)]
    pub fn nfsel(&self) -> NFSEL_R {
        NFSEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLE to REN Low Delay"]
    #[inline(always)]
    pub fn tclr(&mut self) -> TCLR_W {
        TCLR_W { w: self }
    }
    #[doc = "Bits 4:7 - ALE to Data Start"]
    #[inline(always)]
    pub fn tadl(&mut self) -> TADL_W {
        TADL_W { w: self }
    }
    #[doc = "Bits 8:11 - ALE to REN Low Delay"]
    #[inline(always)]
    pub fn tar(&mut self) -> TAR_W {
        TAR_W { w: self }
    }
    #[doc = "Bit 12 - Off Chip Memory Scrambling Enable"]
    #[inline(always)]
    pub fn ocms(&mut self) -> OCMS_W {
        OCMS_W { w: self }
    }
    #[doc = "Bits 16:19 - Ready to REN Low Delay"]
    #[inline(always)]
    pub fn trr(&mut self) -> TRR_W {
        TRR_W { w: self }
    }
    #[doc = "Bits 24:27 - WEN High to REN to Busy"]
    #[inline(always)]
    pub fn twb(&mut self) -> TWB_W {
        TWB_W { w: self }
    }
    #[doc = "Bits 28:30 - Ready/Busy Line Selection"]
    #[inline(always)]
    pub fn rbnsel(&mut self) -> RBNSEL_W {
        RBNSEL_W { w: self }
    }
    #[doc = "Bit 31 - NAND Flash Selection"]
    #[inline(always)]
    pub fn nfsel(&mut self) -> NFSEL_W {
        NFSEL_W { w: self }
    }
}
