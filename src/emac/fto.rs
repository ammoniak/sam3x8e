#[doc = "Reader of register FTO"]
pub type R = crate::R<u32, super::FTO>;
#[doc = "Writer for register FTO"]
pub type W = crate::W<u32, super::FTO>;
#[doc = "Register FTO `reset()`'s with value 0"]
impl crate::ResetValue for super::FTO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FTOK`"]
pub type FTOK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FTOK`"]
pub struct FTOK_W<'a> {
    w: &'a mut W,
}
impl<'a> FTOK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Frames Transmitted OK"]
    #[inline(always)]
    pub fn ftok(&self) -> FTOK_R {
        FTOK_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Frames Transmitted OK"]
    #[inline(always)]
    pub fn ftok(&mut self) -> FTOK_W {
        FTOK_W { w: self }
    }
}
