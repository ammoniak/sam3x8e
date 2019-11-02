#[doc = "Reader of register RCR"]
pub type R = crate::R<u32, super::RCR>;
#[doc = "Writer for register RCR"]
pub type W = crate::W<u32, super::RCR>;
#[doc = "Register RCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXCTR`"]
pub type RXCTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RXCTR`"]
pub struct RXCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Counter Register"]
    #[inline(always)]
    pub fn rxctr(&self) -> RXCTR_R {
        RXCTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Counter Register"]
    #[inline(always)]
    pub fn rxctr(&mut self) -> RXCTR_W {
        RXCTR_W { w: self }
    }
}
