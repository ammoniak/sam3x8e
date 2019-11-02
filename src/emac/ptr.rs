#[doc = "Reader of register PTR"]
pub type R = crate::R<u32, super::PTR>;
#[doc = "Writer for register PTR"]
pub type W = crate::W<u32, super::PTR>;
#[doc = "Register PTR `reset()`'s with value 0"]
impl crate::ResetValue for super::PTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PTIME`"]
pub type PTIME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PTIME`"]
pub struct PTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> PTIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Pause Time"]
    #[inline(always)]
    pub fn ptime(&self) -> PTIME_R {
        PTIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pause Time"]
    #[inline(always)]
    pub fn ptime(&mut self) -> PTIME_W {
        PTIME_W { w: self }
    }
}
