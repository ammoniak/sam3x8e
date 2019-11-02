#[doc = "Reader of register OCMS"]
pub type R = crate::R<u32, super::OCMS>;
#[doc = "Writer for register OCMS"]
pub type W = crate::W<u32, super::OCMS>;
#[doc = "Register OCMS `reset()`'s with value 0"]
impl crate::ResetValue for super::OCMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMSE`"]
pub type SMSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMSE`"]
pub struct SMSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SRSE`"]
pub type SRSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRSE`"]
pub struct SRSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SMSE_R {
        SMSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRAM Scrambling Enable"]
    #[inline(always)]
    pub fn srse(&self) -> SRSE_R {
        SRSE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&mut self) -> SMSE_W {
        SMSE_W { w: self }
    }
    #[doc = "Bit 1 - SRAM Scrambling Enable"]
    #[inline(always)]
    pub fn srse(&mut self) -> SRSE_W {
        SRSE_W { w: self }
    }
}
