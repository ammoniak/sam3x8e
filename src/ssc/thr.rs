#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::THR {}
#[doc = r" Proxy"]
pub struct _TDATW<'a> {
    w: &'a mut W,
}
impl<'a> _TDATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Transmit Data"]
    #[inline]
    pub fn tdat(&mut self) -> _TDATW {
        _TDATW { w: self }
    }
}
