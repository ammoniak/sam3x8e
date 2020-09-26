#[doc = "Reader of register RJA"]
pub type R = crate::R<u32, super::RJA>;
#[doc = "Writer for register RJA"]
pub type W = crate::W<u32, super::RJA>;
#[doc = "Register RJA `reset()`'s with value 0"]
impl crate::ResetValue for super::RJA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RJB`"]
pub type RJB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RJB`"]
pub struct RJB_W<'a> {
    w: &'a mut W,
}
impl<'a> RJB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    pub fn rjb(&self) -> RJB_R {
        RJB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Jabbers"]
    #[inline(always)]
    pub fn rjb(&mut self) -> RJB_W {
        RJB_W { w: self }
    }
}
