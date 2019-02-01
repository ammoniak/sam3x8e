#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSCUPD {}
#[doc = r" Proxy"]
pub struct _OSCUPH0W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPH0W<'a> {
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
pub struct _OSCUPH1W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPH1W<'a> {
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
pub struct _OSCUPH2W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPH2W<'a> {
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
pub struct _OSCUPH3W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPH3W<'a> {
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
pub struct _OSCUPH4W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPH4W<'a> {
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
pub struct _OSCUPH5W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPH5W<'a> {
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
pub struct _OSCUPH6W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPH6W<'a> {
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
pub struct _OSCUPH7W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPH7W<'a> {
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
pub struct _OSCUPL0W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPL0W<'a> {
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
pub struct _OSCUPL1W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPL1W<'a> {
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
pub struct _OSCUPL2W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPL2W<'a> {
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
pub struct _OSCUPL3W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPL3W<'a> {
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
pub struct _OSCUPL4W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPL4W<'a> {
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
pub struct _OSCUPL5W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPL5W<'a> {
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
pub struct _OSCUPDL6W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPDL6W<'a> {
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
pub struct _OSCUPL7W<'a> {
    w: &'a mut W,
}
impl<'a> _OSCUPL7W<'a> {
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
    pub fn oscuph0(&mut self) -> _OSCUPH0W {
        _OSCUPH0W { w: self }
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline]
    pub fn oscuph1(&mut self) -> _OSCUPH1W {
        _OSCUPH1W { w: self }
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline]
    pub fn oscuph2(&mut self) -> _OSCUPH2W {
        _OSCUPH2W { w: self }
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline]
    pub fn oscuph3(&mut self) -> _OSCUPH3W {
        _OSCUPH3W { w: self }
    }
    #[doc = "Bit 4 - Output Selection Clear for PWMH output of the channel 4"]
    #[inline]
    pub fn oscuph4(&mut self) -> _OSCUPH4W {
        _OSCUPH4W { w: self }
    }
    #[doc = "Bit 5 - Output Selection Clear for PWMH output of the channel 5"]
    #[inline]
    pub fn oscuph5(&mut self) -> _OSCUPH5W {
        _OSCUPH5W { w: self }
    }
    #[doc = "Bit 6 - Output Selection Clear for PWMH output of the channel 6"]
    #[inline]
    pub fn oscuph6(&mut self) -> _OSCUPH6W {
        _OSCUPH6W { w: self }
    }
    #[doc = "Bit 7 - Output Selection Clear for PWMH output of the channel 7"]
    #[inline]
    pub fn oscuph7(&mut self) -> _OSCUPH7W {
        _OSCUPH7W { w: self }
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline]
    pub fn oscupl0(&mut self) -> _OSCUPL0W {
        _OSCUPL0W { w: self }
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline]
    pub fn oscupl1(&mut self) -> _OSCUPL1W {
        _OSCUPL1W { w: self }
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline]
    pub fn oscupl2(&mut self) -> _OSCUPL2W {
        _OSCUPL2W { w: self }
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline]
    pub fn oscupl3(&mut self) -> _OSCUPL3W {
        _OSCUPL3W { w: self }
    }
    #[doc = "Bit 20 - Output Selection Clear for PWML output of the channel 4"]
    #[inline]
    pub fn oscupl4(&mut self) -> _OSCUPL4W {
        _OSCUPL4W { w: self }
    }
    #[doc = "Bit 21 - Output Selection Clear for PWML output of the channel 5"]
    #[inline]
    pub fn oscupl5(&mut self) -> _OSCUPL5W {
        _OSCUPL5W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline]
    pub fn oscupdl6(&mut self) -> _OSCUPDL6W {
        _OSCUPDL6W { w: self }
    }
    #[doc = "Bit 23 - Output Selection Clear for PWML output of the channel 7"]
    #[inline]
    pub fn oscupl7(&mut self) -> _OSCUPL7W {
        _OSCUPL7W { w: self }
    }
}
