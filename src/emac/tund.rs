#[doc = "Reader of register TUND"]
pub type R = crate::R<u32, super::TUND>;
#[doc = "Writer for register TUND"]
pub type W = crate::W<u32, super::TUND>;
#[doc = "Register TUND `reset()`'s with value 0"]
impl crate::ResetValue for super::TUND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TUND`"]
pub type TUND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUND`"]
pub struct TUND_W<'a> {
    w: &'a mut W,
}
impl<'a> TUND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit Underruns"]
    #[inline(always)]
    pub fn tund(&self) -> TUND_R {
        TUND_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Underruns"]
    #[inline(always)]
    pub fn tund(&mut self) -> TUND_W {
        TUND_W { w: self }
    }
}
