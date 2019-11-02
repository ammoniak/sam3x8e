#[doc = "Reader of register SADDR2"]
pub type R = crate::R<u32, super::SADDR2>;
#[doc = "Writer for register SADDR2"]
pub type W = crate::W<u32, super::SADDR2>;
#[doc = "Register SADDR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SADDR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SADDR`"]
pub type SADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SADDR`"]
pub struct SADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn saddr(&mut self) -> SADDR_W {
        SADDR_W { w: self }
    }
}
