#[doc = "Reader of register MATRIX_SCFG[%s]"]
pub type R = crate::R<u32, super::MATRIX_SCFG>;
#[doc = "Writer for register MATRIX_SCFG[%s]"]
pub type W = crate::W<u32, super::MATRIX_SCFG>;
#[doc = "Reader of field `SLOT_CYCLE`"]
pub type SLOT_CYCLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLOT_CYCLE`"]
pub struct SLOT_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOT_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DEFMSTR_TYPE`"]
pub type DEFMSTR_TYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEFMSTR_TYPE`"]
pub struct DEFMSTR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEFMSTR_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `FIXED_DEFMSTR`"]
pub type FIXED_DEFMSTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIXED_DEFMSTR`"]
pub struct FIXED_DEFMSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIXED_DEFMSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `ARBT`"]
pub type ARBT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ARBT`"]
pub struct ARBT_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SLOT_CYCLE_R {
        SLOT_CYCLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPE_R {
        DEFMSTR_TYPE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTR_R {
        FIXED_DEFMSTR_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&self) -> ARBT_R {
        ARBT_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&mut self) -> SLOT_CYCLE_W {
        SLOT_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&mut self) -> DEFMSTR_TYPE_W {
        DEFMSTR_TYPE_W { w: self }
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W {
        FIXED_DEFMSTR_W { w: self }
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&mut self) -> ARBT_W {
        ARBT_W { w: self }
    }
}
