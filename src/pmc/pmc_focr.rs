#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_FOCR {}
#[doc = r" Proxy"]
pub struct _FOCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FOCLRW<'a> {
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
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fault Output Clear"]
    #[inline]
    pub fn foclr(&mut self) -> _FOCLRW {
        _FOCLRW { w: self }
    }
}
