#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {}
#[doc = r" Proxy"]
pub struct _WDRSTTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRSTTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Watchdog Restart"]
    #[inline]
    pub fn wdrstt(&mut self) -> _WDRSTTW {
        _WDRSTTW { w: self }
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}
