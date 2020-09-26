#[doc = "Reader of register ECOL"]
pub type R = crate::R<u32, super::ECOL>;
#[doc = "Writer for register ECOL"]
pub type W = crate::W<u32, super::ECOL>;
#[doc = "Register ECOL `reset()`'s with value 0"]
impl crate::ResetValue for super::ECOL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EXCOL`"]
pub type EXCOL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXCOL`"]
pub struct EXCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&self) -> EXCOL_R {
        EXCOL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Excessive Collisions"]
    #[inline(always)]
    pub fn excol(&mut self) -> EXCOL_W {
        EXCOL_W { w: self }
    }
}
