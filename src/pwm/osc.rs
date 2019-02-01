#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSC {}
#[doc = r" Proxy"]
pub struct _OSCH0W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCH0W<'a> {
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
pub struct _OSCH1W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCH1W<'a> {
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
pub struct _OSCH2W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCH2W<'a> {
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
pub struct _OSCH3W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCH3W<'a> {
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
pub struct _OSCH4W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCH4W<'a> {
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
pub struct _OSCH5W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCH5W<'a> {
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
pub struct _OSCH6W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCH6W<'a> {
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
pub struct _OSCH7W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCH7W<'a> {
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
pub struct _OSCL0W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCL0W<'a> {
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
pub struct _OSCL1W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCL1W<'a> {
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
pub struct _OSCL2W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCL2W<'a> {
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
pub struct _OSCL3W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCL3W<'a> {
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
#[doc = r" Proxy"]
pub struct _OSCL4W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCL4W<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSCL5W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCL5W<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSCL6W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCL6W<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OSCL7W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCL7W<'a> {
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
        const OFFSET: u8 = 23;
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
    #[doc = "Bit 0 - Output Selection Clear for PWMH output of the channel 0"]
    #[inline]
    pub fn osch0(&mut self) -> _OSCH0W {
        _OSCH0W { w: self }
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline]
    pub fn osch1(&mut self) -> _OSCH1W {
        _OSCH1W { w: self }
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline]
    pub fn osch2(&mut self) -> _OSCH2W {
        _OSCH2W { w: self }
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline]
    pub fn osch3(&mut self) -> _OSCH3W {
        _OSCH3W { w: self }
    }
    #[doc = "Bit 4 - Output Selection Clear for PWMH output of the channel 4"]
    #[inline]
    pub fn osch4(&mut self) -> _OSCH4W {
        _OSCH4W { w: self }
    }
    #[doc = "Bit 5 - Output Selection Clear for PWMH output of the channel 5"]
    #[inline]
    pub fn osch5(&mut self) -> _OSCH5W {
        _OSCH5W { w: self }
    }
    #[doc = "Bit 6 - Output Selection Clear for PWMH output of the channel 6"]
    #[inline]
    pub fn osch6(&mut self) -> _OSCH6W {
        _OSCH6W { w: self }
    }
    #[doc = "Bit 7 - Output Selection Clear for PWMH output of the channel 7"]
    #[inline]
    pub fn osch7(&mut self) -> _OSCH7W {
        _OSCH7W { w: self }
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline]
    pub fn oscl0(&mut self) -> _OSCL0W {
        _OSCL0W { w: self }
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline]
    pub fn oscl1(&mut self) -> _OSCL1W {
        _OSCL1W { w: self }
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline]
    pub fn oscl2(&mut self) -> _OSCL2W {
        _OSCL2W { w: self }
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline]
    pub fn oscl3(&mut self) -> _OSCL3W {
        _OSCL3W { w: self }
    }
    #[doc = "Bit 20 - Output Selection Clear for PWML output of the channel 4"]
    #[inline]
    pub fn oscl4(&mut self) -> _OSCL4W {
        _OSCL4W { w: self }
    }
    #[doc = "Bit 21 - Output Selection Clear for PWML output of the channel 5"]
    #[inline]
    pub fn oscl5(&mut self) -> _OSCL5W {
        _OSCL5W { w: self }
    }
    #[doc = "Bit 22 - Output Selection Clear for PWML output of the channel 6"]
    #[inline]
    pub fn oscl6(&mut self) -> _OSCL6W {
        _OSCL6W { w: self }
    }
    #[doc = "Bit 23 - Output Selection Clear for PWML output of the channel 7"]
    #[inline]
    pub fn oscl7(&mut self) -> _OSCL7W {
        _OSCL7W { w: self }
    }
}
