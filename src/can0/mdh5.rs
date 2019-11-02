#[doc = "Reader of register MDH5"]
pub type R = crate::R<u32, super::MDH5>;
#[doc = "Writer for register MDH5"]
pub type W = crate::W<u32, super::MDH5>;
#[doc = "Register MDH5 `reset()`'s with value 0"]
impl crate::ResetValue for super::MDH5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDH`"]
pub type MDH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MDH`"]
pub struct MDH_W<'a> {
    w: &'a mut W,
}
impl<'a> MDH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&self) -> MDH_R {
        MDH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Data High Value"]
    #[inline(always)]
    pub fn mdh(&mut self) -> MDH_W {
        MDH_W { w: self }
    }
}
