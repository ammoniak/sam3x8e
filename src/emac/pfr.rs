#[doc = "Reader of register PFR"]
pub type R = crate::R<u32, super::PFR>;
#[doc = "Writer for register PFR"]
pub type W = crate::W<u32, super::PFR>;
#[doc = "Register PFR `reset()`'s with value 0"]
impl crate::ResetValue for super::PFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FROK`"]
pub type FROK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FROK`"]
pub struct FROK_W<'a> {
    w: &'a mut W,
}
impl<'a> FROK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Pause Frames received OK"]
    #[inline(always)]
    pub fn frok(&self) -> FROK_R {
        FROK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pause Frames received OK"]
    #[inline(always)]
    pub fn frok(&mut self) -> FROK_W {
        FROK_W { w: self }
    }
}
