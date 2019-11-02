#[doc = "Reader of register RLE"]
pub type R = crate::R<u32, super::RLE>;
#[doc = "Writer for register RLE"]
pub type W = crate::W<u32, super::RLE>;
#[doc = "Register RLE `reset()`'s with value 0"]
impl crate::ResetValue for super::RLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RLFM`"]
pub type RLFM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RLFM`"]
pub struct RLFM_W<'a> {
    w: &'a mut W,
}
impl<'a> RLFM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    pub fn rlfm(&self) -> RLFM_R {
        RLFM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Length Field Mismatch"]
    #[inline(always)]
    pub fn rlfm(&mut self) -> RLFM_W {
        RLFM_W { w: self }
    }
}
