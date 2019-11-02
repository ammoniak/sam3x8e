#[doc = "Reader of register SCF"]
pub type R = crate::R<u32, super::SCF>;
#[doc = "Writer for register SCF"]
pub type W = crate::W<u32, super::SCF>;
#[doc = "Register SCF `reset()`'s with value 0"]
impl crate::ResetValue for super::SCF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCF`"]
pub type SCF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SCF`"]
pub struct SCF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Single Collision Frames"]
    #[inline(always)]
    pub fn scf(&self) -> SCF_R {
        SCF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Single Collision Frames"]
    #[inline(always)]
    pub fn scf(&mut self) -> SCF_W {
        SCF_W { w: self }
    }
}
