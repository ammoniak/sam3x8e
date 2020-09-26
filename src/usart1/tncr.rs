#[doc = "Reader of register TNCR"]
pub type R = crate::R<u32, super::TNCR>;
#[doc = "Writer for register TNCR"]
pub type W = crate::W<u32, super::TNCR>;
#[doc = "Register TNCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TNCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXNCTR`"]
pub type TXNCTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXNCTR`"]
pub struct TXNCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXNCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Counter Next"]
    #[inline(always)]
    pub fn txnctr(&self) -> TXNCTR_R {
        TXNCTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Counter Next"]
    #[inline(always)]
    pub fn txnctr(&mut self) -> TXNCTR_W {
        TXNCTR_W { w: self }
    }
}
