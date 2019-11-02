#[doc = "Reader of register ALE"]
pub type R = crate::R<u32, super::ALE>;
#[doc = "Writer for register ALE"]
pub type W = crate::W<u32, super::ALE>;
#[doc = "Register ALE `reset()`'s with value 0"]
impl crate::ResetValue for super::ALE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ALE`"]
pub type ALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALE`"]
pub struct ALE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Alignment Errors"]
    #[inline(always)]
    pub fn ale(&self) -> ALE_R {
        ALE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alignment Errors"]
    #[inline(always)]
    pub fn ale(&mut self) -> ALE_W {
        ALE_W { w: self }
    }
}
