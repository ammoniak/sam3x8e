#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPRDUPD5 {}
#[doc = r" Proxy"]
pub struct _CPRDUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _CPRDUPDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline]
    pub fn cprdupd(&mut self) -> _CPRDUPDW {
        _CPRDUPDW { w: self }
    }
}
