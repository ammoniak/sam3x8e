#[doc = "Reader of register CSE"]
pub type R = crate::R<u32, super::CSE>;
#[doc = "Writer for register CSE"]
pub type W = crate::W<u32, super::CSE>;
#[doc = "Register CSE `reset()`'s with value 0"]
impl crate::ResetValue for super::CSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSE`"]
pub type CSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSE`"]
pub struct CSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Carrier Sense Errors"]
    #[inline(always)]
    pub fn cse(&self) -> CSE_R {
        CSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Carrier Sense Errors"]
    #[inline(always)]
    pub fn cse(&mut self) -> CSE_W {
        CSE_W { w: self }
    }
}
