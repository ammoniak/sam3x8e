#[doc = "Reader of register ROV"]
pub type R = crate::R<u32, super::ROV>;
#[doc = "Writer for register ROV"]
pub type W = crate::W<u32, super::ROV>;
#[doc = "Register ROV `reset()`'s with value 0"]
impl crate::ResetValue for super::ROV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ROVR`"]
pub type ROVR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ROVR`"]
pub struct ROVR_W<'a> {
    w: &'a mut W,
}
impl<'a> ROVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&mut self) -> ROVR_W {
        ROVR_W { w: self }
    }
}
