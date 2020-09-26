#[doc = "Writer for register CPRDUPD2"]
pub type W = crate::W<u32, super::CPRDUPD2>;
#[doc = "Write proxy for field `CPRDUPD`"]
pub struct CPRDUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRDUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline(always)]
    pub fn cprdupd(&mut self) -> CPRDUPD_W {
        CPRDUPD_W { w: self }
    }
}
