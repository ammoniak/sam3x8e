#[doc = "Reader of register TPR"]
pub type R = crate::R<u32, super::TPR>;
#[doc = "Writer for register TPR"]
pub type W = crate::W<u32, super::TPR>;
#[doc = "Register TPR `reset()`'s with value 0"]
impl crate::ResetValue for super::TPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXPTR`"]
pub type TXPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TXPTR`"]
pub struct TXPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transmit Counter Register"]
    #[inline(always)]
    pub fn txptr(&self) -> TXPTR_R {
        TXPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Counter Register"]
    #[inline(always)]
    pub fn txptr(&mut self) -> TXPTR_W {
        TXPTR_W { w: self }
    }
}
