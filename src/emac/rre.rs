#[doc = "Reader of register RRE"]
pub type R = crate::R<u32, super::RRE>;
#[doc = "Writer for register RRE"]
pub type W = crate::W<u32, super::RRE>;
#[doc = "Register RRE `reset()`'s with value 0"]
impl crate::ResetValue for super::RRE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RRE`"]
pub type RRE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RRE`"]
pub struct RRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rre(&self) -> RRE_R {
        RRE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Resource Errors"]
    #[inline(always)]
    pub fn rre(&mut self) -> RRE_W {
        RRE_W { w: self }
    }
}
