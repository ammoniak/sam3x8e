#[doc = "Reader of register FMR"]
pub type R = crate::R<u32, super::FMR>;
#[doc = "Writer for register FMR"]
pub type W = crate::W<u32, super::FMR>;
#[doc = "Register FMR `reset()`'s with value 0"]
impl crate::ResetValue for super::FMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRDY`"]
pub type FRDY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRDY`"]
pub struct FRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> FRDY_W<'a> {
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
#[doc = "Reader of field `FWS`"]
pub type FWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FWS`"]
pub struct FWS_W<'a> {
    w: &'a mut W,
}
impl<'a> FWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCOD`"]
pub type SCOD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCOD`"]
pub struct SCOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SCOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FAM`"]
pub type FAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAM`"]
pub struct FAM_W<'a> {
    w: &'a mut W,
}
impl<'a> FAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&self) -> FWS_R {
        FWS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&self) -> SCOD_R {
        SCOD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Flash Access Mode"]
    #[inline(always)]
    pub fn fam(&self) -> FAM_R {
        FAM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&mut self) -> FRDY_W {
        FRDY_W { w: self }
    }
    #[doc = "Bits 8:11 - Flash Wait State"]
    #[inline(always)]
    pub fn fws(&mut self) -> FWS_W {
        FWS_W { w: self }
    }
    #[doc = "Bit 16 - Sequential Code Optimization Disable"]
    #[inline(always)]
    pub fn scod(&mut self) -> SCOD_W {
        SCOD_W { w: self }
    }
    #[doc = "Bit 24 - Flash Access Mode"]
    #[inline(always)]
    pub fn fam(&mut self) -> FAM_W {
        FAM_W { w: self }
    }
}
