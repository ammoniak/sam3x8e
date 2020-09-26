#[doc = "Reader of register FCSE"]
pub type R = crate::R<u32, super::FCSE>;
#[doc = "Writer for register FCSE"]
pub type W = crate::W<u32, super::FCSE>;
#[doc = "Register FCSE `reset()`'s with value 0"]
impl crate::ResetValue for super::FCSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCSE`"]
pub type FCSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FCSE`"]
pub struct FCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fcse(&self) -> FCSE_R {
        FCSE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Check Sequence Errors"]
    #[inline(always)]
    pub fn fcse(&mut self) -> FCSE_W {
        FCSE_W { w: self }
    }
}
