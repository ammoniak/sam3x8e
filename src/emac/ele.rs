#[doc = "Reader of register ELE"]
pub type R = crate::R<u32, super::ELE>;
#[doc = "Writer for register ELE"]
pub type W = crate::W<u32, super::ELE>;
#[doc = "Register ELE `reset()`'s with value 0"]
impl crate::ResetValue for super::ELE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXL`"]
pub type EXL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXL`"]
pub struct EXL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Excessive Length Errors"]
    #[inline(always)]
    pub fn exl(&self) -> EXL_R {
        EXL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Excessive Length Errors"]
    #[inline(always)]
    pub fn exl(&mut self) -> EXL_W {
        EXL_W { w: self }
    }
}
