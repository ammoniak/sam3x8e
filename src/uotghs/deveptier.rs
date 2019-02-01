#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVEPTIER {}
#[doc = r" Proxy"]
pub struct _TXINESW<'a> {
    w: &'a mut W,
}
impl<'a> _TXINESW<'a> {
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
pub struct _RXOUTESW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOUTESW<'a> {
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
pub struct _RXSTPESW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSTPESW<'a> {
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
pub struct _UNDERFESW<'a> {
    w: &'a mut W,
}
impl<'a> _UNDERFESW<'a> {
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
pub struct _NAKOUTESW<'a> {
    w: &'a mut W,
}
impl<'a> _NAKOUTESW<'a> {
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
pub struct _HBISOINERRESW<'a> {
    w: &'a mut W,
}
impl<'a> _HBISOINERRESW<'a> {
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
pub struct _NAKINESW<'a> {
    w: &'a mut W,
}
impl<'a> _NAKINESW<'a> {
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
pub struct _HBISOFLUSHESW<'a> {
    w: &'a mut W,
}
impl<'a> _HBISOFLUSHESW<'a> {
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
pub struct _OVERFESW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERFESW<'a> {
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
pub struct _STALLEDESW<'a> {
    w: &'a mut W,
}
impl<'a> _STALLEDESW<'a> {
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
pub struct _CRCERRESW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCERRESW<'a> {
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
pub struct _SHORTPACKETESW<'a> {
    w: &'a mut W,
}
impl<'a> _SHORTPACKETESW<'a> {
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
pub struct _MDATAESW<'a> {
    w: &'a mut W,
}
impl<'a> _MDATAESW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATAXESW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAXESW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ERRORTRANSESW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRORTRANSESW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NBUSYBKESW<'a> {
    w: &'a mut W,
}
impl<'a> _NBUSYBKESW<'a> {
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
#[doc = r" Proxy"]
pub struct _KILLBKSW<'a> {
    w: &'a mut W,
}
impl<'a> _KILLBKSW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPDISHDMASW<'a> {
    w: &'a mut W,
}
impl<'a> _EPDISHDMASW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NYETDISSW<'a> {
    w: &'a mut W,
}
impl<'a> _NYETDISSW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSTDTSW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTDTSW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STALLRQSW<'a> {
    w: &'a mut W,
}
impl<'a> _STALLRQSW<'a> {
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
        const OFFSET: u8 = 19;
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
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Enable"]
    #[inline]
    pub fn txines(&mut self) -> _TXINESW {
        _TXINESW { w: self }
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Enable"]
    #[inline]
    pub fn rxoutes(&mut self) -> _RXOUTESW {
        _RXOUTESW { w: self }
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Enable"]
    #[inline]
    pub fn rxstpes(&mut self) -> _RXSTPESW {
        _RXSTPESW { w: self }
    }
    #[doc = "Bit 2 - Underflow Interrupt Enable"]
    #[inline]
    pub fn underfes(&mut self) -> _UNDERFESW {
        _UNDERFESW { w: self }
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Enable"]
    #[inline]
    pub fn nakoutes(&mut self) -> _NAKOUTESW {
        _NAKOUTESW { w: self }
    }
    #[doc = "Bit 3 - High Bandwidth Isochronous IN Error Interrupt Enable"]
    #[inline]
    pub fn hbisoinerres(&mut self) -> _HBISOINERRESW {
        _HBISOINERRESW { w: self }
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Enable"]
    #[inline]
    pub fn nakines(&mut self) -> _NAKINESW {
        _NAKINESW { w: self }
    }
    #[doc = "Bit 4 - High Bandwidth Isochronous IN Flush Interrupt Enable"]
    #[inline]
    pub fn hbisoflushes(&mut self) -> _HBISOFLUSHESW {
        _HBISOFLUSHESW { w: self }
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline]
    pub fn overfes(&mut self) -> _OVERFESW {
        _OVERFESW { w: self }
    }
    #[doc = "Bit 6 - STALLed Interrupt Enable"]
    #[inline]
    pub fn stalledes(&mut self) -> _STALLEDESW {
        _STALLEDESW { w: self }
    }
    #[doc = "Bit 6 - CRC Error Interrupt Enable"]
    #[inline]
    pub fn crcerres(&mut self) -> _CRCERRESW {
        _CRCERRESW { w: self }
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline]
    pub fn shortpacketes(&mut self) -> _SHORTPACKETESW {
        _SHORTPACKETESW { w: self }
    }
    #[doc = "Bit 8 - MData Interrupt Enable"]
    #[inline]
    pub fn mdataes(&mut self) -> _MDATAESW {
        _MDATAESW { w: self }
    }
    #[doc = "Bit 9 - DataX Interrupt Enable"]
    #[inline]
    pub fn dataxes(&mut self) -> _DATAXESW {
        _DATAXESW { w: self }
    }
    #[doc = "Bit 10 - Transaction Error Interrupt Enable"]
    #[inline]
    pub fn errortranses(&mut self) -> _ERRORTRANSESW {
        _ERRORTRANSESW { w: self }
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline]
    pub fn nbusybkes(&mut self) -> _NBUSYBKESW {
        _NBUSYBKESW { w: self }
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline]
    pub fn killbks(&mut self) -> _KILLBKSW {
        _KILLBKSW { w: self }
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request Enable"]
    #[inline]
    pub fn epdishdmas(&mut self) -> _EPDISHDMASW {
        _EPDISHDMASW { w: self }
    }
    #[doc = "Bit 17 - NYET Token Disable Enable"]
    #[inline]
    pub fn nyetdiss(&mut self) -> _NYETDISSW {
        _NYETDISSW { w: self }
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline]
    pub fn rstdts(&mut self) -> _RSTDTSW {
        _RSTDTSW { w: self }
    }
    #[doc = "Bit 19 - STALL Request Enable"]
    #[inline]
    pub fn stallrqs(&mut self) -> _STALLRQSW {
        _STALLRQSW { w: self }
    }
}
