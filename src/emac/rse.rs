#[doc = "Reader of register RSE"]
pub type R = crate::R<u32, super::RSE>;
#[doc = "Writer for register RSE"]
pub type W = crate::W<u32, super::RSE>;
#[doc = "Register RSE `reset()`'s with value 0"]
impl crate::ResetValue for super::RSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RSE`"]
pub type RSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSE`"]
pub struct RSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Symbol Errors"]
    #[inline(always)]
    pub fn rse(&mut self) -> RSE_W {
        RSE_W { w: self }
    }
}
