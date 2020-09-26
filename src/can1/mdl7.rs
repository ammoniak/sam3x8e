#[doc = "Reader of register MDL7"]
pub type R = crate::R<u32, super::MDL7>;
#[doc = "Writer for register MDL7"]
pub type W = crate::W<u32, super::MDL7>;
#[doc = "Register MDL7 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDL7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDL`"]
pub type MDL_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MDL`"]
pub struct MDL_W<'a> {
    w: &'a mut W,
}
impl<'a> MDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    pub fn mdl(&self) -> MDL_R {
        MDL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data Low Value"]
    #[inline(always)]
    pub fn mdl(&mut self) -> MDL_W {
        MDL_W { w: self }
    }
}
