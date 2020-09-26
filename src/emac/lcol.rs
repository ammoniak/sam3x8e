#[doc = "Reader of register LCOL"]
pub type R = crate::R<u32, super::LCOL>;
#[doc = "Writer for register LCOL"]
pub type W = crate::W<u32, super::LCOL>;
#[doc = "Register LCOL `reset()`'s with value 0"]
impl crate::ResetValue for super::LCOL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCOL`"]
pub type LCOL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LCOL`"]
pub struct LCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> LCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Late Collisions"]
    #[inline(always)]
    pub fn lcol(&mut self) -> LCOL_W {
        LCOL_W { w: self }
    }
}
