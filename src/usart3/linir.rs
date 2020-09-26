#[doc = "Reader of register LINIR"]
pub type R = crate::R<u32, super::LINIR>;
#[doc = "Writer for register LINIR"]
pub type W = crate::W<u32, super::LINIR>;
#[doc = "Register LINIR `reset()`'s with value 0"]
impl crate::ResetValue for super::LINIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDCHR`"]
pub type IDCHR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDCHR`"]
pub struct IDCHR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDCHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    pub fn idchr(&self) -> IDCHR_R {
        IDCHR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    pub fn idchr(&mut self) -> IDCHR_W {
        IDCHR_W { w: self }
    }
}
