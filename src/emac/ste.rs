#[doc = "Reader of register STE"]
pub type R = crate::R<u32, super::STE>;
#[doc = "Writer for register STE"]
pub type W = crate::W<u32, super::STE>;
#[doc = "Register STE `reset()`'s with value 0"]
impl crate::ResetValue for super::STE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SQER`"]
pub type SQER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SQER`"]
pub struct SQER_W<'a> {
    w: &'a mut W,
}
impl<'a> SQER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    pub fn sqer(&self) -> SQER_R {
        SQER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    pub fn sqer(&mut self) -> SQER_W {
        SQER_W { w: self }
    }
}
