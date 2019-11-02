#[doc = "Writer for register CDTYUPD0"]
pub type W = crate::W<u32, super::CDTYUPD0>;
#[doc = "Write proxy for field `CDTYUPD`"]
pub struct CDTYUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CDTYUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Duty-Cycle Update"]
    #[inline(always)]
    pub fn cdtyupd(&mut self) -> CDTYUPD_W {
        CDTYUPD_W { w: self }
    }
}
