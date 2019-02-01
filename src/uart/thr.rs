#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::THR {}
#[doc = r" Proxy"]
pub struct _TXCHRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCHRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - Character to be Transmitted"]
    #[inline]
    pub fn txchr(&mut self) -> _TXCHRW {
        _TXCHRW { w: self }
    }
}
