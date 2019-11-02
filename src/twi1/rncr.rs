#[doc = "Reader of register RNCR"]
pub type R = crate::R<u32, super::RNCR>;
#[doc = "Writer for register RNCR"]
pub type W = crate::W<u32, super::RNCR>;
#[doc = "Register RNCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RNCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXNCTR`"]
pub type RXNCTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RXNCTR`"]
pub struct RXNCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNCTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Next Counter"]
    #[inline(always)]
    pub fn rxnctr(&self) -> RXNCTR_R {
        RXNCTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Next Counter"]
    #[inline(always)]
    pub fn rxnctr(&mut self) -> RXNCTR_W {
        RXNCTR_W { w: self }
    }
}
