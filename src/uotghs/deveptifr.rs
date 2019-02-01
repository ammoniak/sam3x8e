#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVEPTIFR {}
#[doc = r" Proxy"]
pub struct _TXINISW<'a> {
    w: &'a mut W,
}
impl<'a> _TXINISW<'a> {
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
pub struct _RXOUTISW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOUTISW<'a> {
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
pub struct _RXSTPISW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSTPISW<'a> {
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
pub struct _UNDERFISW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDERFISW<'a> {
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
pub struct _NAKOUTISW<'a> {
    w: &'a mut W,
}
impl<'a> _NAKOUTISW<'a> {
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
#[doc = r" Proxy"]
pub struct _HBISOINERRISW<'a> {
    w: &'a mut W,
}
impl<'a> _HBISOINERRISW<'a> {
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
#[doc = r" Proxy"]
pub struct _NAKINISW<'a> {
    w: &'a mut W,
}
impl<'a> _NAKINISW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HBISOFLUSHISW<'a> {
    w: &'a mut W,
}
impl<'a> _HBISOFLUSHISW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OVERFISW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFISW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STALLEDISW<'a> {
    w: &'a mut W,
}
impl<'a> _STALLEDISW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CRCERRISW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCERRISW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SHORTPACKETSW<'a> {
    w: &'a mut W,
}
impl<'a> _SHORTPACKETSW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NBUSYBKSW<'a> {
    w: &'a mut W,
}
impl<'a> _NBUSYBKSW<'a> {
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline]
    pub fn txinis(&mut self) -> _TXINISW {
        _TXINISW { w: self }
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline]
    pub fn rxoutis(&mut self) -> _RXOUTISW {
        _RXOUTISW { w: self }
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Set"]
    #[inline]
    pub fn rxstpis(&mut self) -> _RXSTPISW {
        _RXSTPISW { w: self }
    }
    #[doc = "Bit 2 - Underflow Interrupt Set"]
    #[inline]
    pub fn underfis(&mut self) -> _UNDERFISW {
        _UNDERFISW { w: self }
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Set"]
    #[inline]
    pub fn nakoutis(&mut self) -> _NAKOUTISW {
        _NAKOUTISW { w: self }
    }
    #[doc = "Bit 3 - High bandwidth isochronous IN Underflow Error Interrupt Set"]
    #[inline]
    pub fn hbisoinerris(&mut self) -> _HBISOINERRISW {
        _HBISOINERRISW { w: self }
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Set"]
    #[inline]
    pub fn nakinis(&mut self) -> _NAKINISW {
        _NAKINISW { w: self }
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Set"]
    #[inline]
    pub fn hbisoflushis(&mut self) -> _HBISOFLUSHISW {
        _HBISOFLUSHISW { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline]
    pub fn overfis(&mut self) -> _OVERFISW {
        _OVERFISW { w: self }
    }
    #[doc = "Bit 6 - STALLed Interrupt Set"]
    #[inline]
    pub fn stalledis(&mut self) -> _STALLEDISW {
        _STALLEDISW { w: self }
    }
    #[doc = "Bit 6 - CRC Error Interrupt Set"]
    #[inline]
    pub fn crcerris(&mut self) -> _CRCERRISW {
        _CRCERRISW { w: self }
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline]
    pub fn shortpackets(&mut self) -> _SHORTPACKETSW {
        _SHORTPACKETSW { w: self }
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline]
    pub fn nbusybks(&mut self) -> _NBUSYBKSW {
        _NBUSYBKSW { w: self }
    }
}
