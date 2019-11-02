#[doc = "Reader of register FRO"]
pub type R = crate::R<u32, super::FRO>;
#[doc = "Writer for register FRO"]
pub type W = crate::W<u32, super::FRO>;
#[doc = "Register FRO `reset()`'s with value 0"]
impl crate::ResetValue for super::FRO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FROK`"]
pub type FROK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FROK`"]
pub struct FROK_W<'a> {
    w: &'a mut W,
}
impl<'a> FROK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Frames Received OK"]
    #[inline(always)]
    pub fn frok(&self) -> FROK_R {
        FROK_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Frames Received OK"]
    #[inline(always)]
    pub fn frok(&mut self) -> FROK_W {
        FROK_W { w: self }
    }
}
