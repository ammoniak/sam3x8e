#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXCTR`"]
pub type TXCTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXCTR`"]
pub struct TXCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Counter Register"]
    #[inline(always)]
    pub fn txctr(&self) -> TXCTR_R {
        TXCTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Counter Register"]
    #[inline(always)]
    pub fn txctr(&mut self) -> TXCTR_W {
        TXCTR_W { w: self }
    }
}
