#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {}
#[doc = r" Proxy"]
pub struct _TXRDYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRDYW<'a> {
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
pub struct _EOCW<'a> {
    w: &'a mut W,
}
impl<'a> _EOCW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENDTXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDTXW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXBUFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBUFEW<'a> {
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Transmit Ready Interrupt Enable"]
    #[inline]
    pub fn txrdy(&mut self) -> _TXRDYW {
        _TXRDYW { w: self }
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Enable"]
    #[inline]
    pub fn eoc(&mut self) -> _EOCW {
        _EOCW { w: self }
    }
    #[doc = "Bit 2 - End of Transmit Buffer Interrupt Enable"]
    #[inline]
    pub fn endtx(&mut self) -> _ENDTXW {
        _ENDTXW { w: self }
    }
    #[doc = "Bit 3 - Transmit Buffer Empty Interrupt Enable"]
    #[inline]
    pub fn txbufe(&mut self) -> _TXBUFEW {
        _TXBUFEW { w: self }
    }
}
