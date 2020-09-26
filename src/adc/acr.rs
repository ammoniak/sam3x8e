#[doc = "Reader of register ACR"]
pub type R = crate::R<u32, super::ACR>;
#[doc = "Writer for register ACR"]
pub type W = crate::W<u32, super::ACR>;
#[doc = "Register ACR `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `TSON`"]
pub type TSON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSON`"]
pub struct TSON_W<'a> {
    w: &'a mut W,
}
impl<'a> TSON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `IBCTL`"]
pub type IBCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBCTL`"]
pub struct IBCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> IBCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Temperature Sensor On"]
    #[inline(always)]
    pub fn tson(&self) -> TSON_R {
        TSON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - ADC Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&self) -> IBCTL_R {
        IBCTL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - Temperature Sensor On"]
    #[inline(always)]
    pub fn tson(&mut self) -> TSON_W {
        TSON_W { w: self }
    }
    #[doc = "Bits 8:9 - ADC Bias Current Control"]
    #[inline(always)]
    pub fn ibctl(&mut self) -> IBCTL_W {
        IBCTL_W { w: self }
    }
}
