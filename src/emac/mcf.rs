#[doc = "Reader of register MCF"]
pub type R = crate::R<u32, super::MCF>;
#[doc = "Writer for register MCF"]
pub type W = crate::W<u32, super::MCF>;
#[doc = "Register MCF `reset()`'s with value 0"]
impl crate::ResetValue for super::MCF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCF`"]
pub type MCF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MCF`"]
pub struct MCF_W<'a> {
    w: &'a mut W,
}
impl<'a> MCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Multicollision Frames"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Multicollision Frames"]
    #[inline(always)]
    pub fn mcf(&mut self) -> MCF_W {
        MCF_W { w: self }
    }
}
