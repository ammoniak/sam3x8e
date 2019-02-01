#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OSS {}
#[doc = r" Proxy"]
pub struct _OSSH0W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSH0W<'a> {
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
pub struct _OSSH1W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSH1W<'a> {
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
pub struct _OSSH2W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSH2W<'a> {
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
pub struct _OSSH3W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSH3W<'a> {
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
pub struct _OSSH4W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSH4W<'a> {
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
pub struct _OSSH5W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSH5W<'a> {
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
pub struct _OSSH6W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSH6W<'a> {
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
pub struct _OSSH7W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSH7W<'a> {
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
pub struct _OSSL0W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSL0W<'a> {
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
pub struct _OSSL1W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSL1W<'a> {
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
pub struct _OSSL2W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSL2W<'a> {
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
pub struct _OSSL3W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSL3W<'a> {
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
pub struct _OSSL4W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSL4W<'a> {
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
pub struct _OSSL5W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSL5W<'a> {
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
pub struct _OSSL6W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSL6W<'a> {
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
pub struct _OSSL7W<'a> {
    w: &'a mut W,
}
impl<'a> _OSSL7W<'a> {
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
    #[doc = "Bit 0 - Output Selection Set for PWMH output of the channel 0"]
    #[inline]
    pub fn ossh0(&mut self) -> _OSSH0W {
        _OSSH0W { w: self }
    }
    #[doc = "Bit 1 - Output Selection Set for PWMH output of the channel 1"]
    #[inline]
    pub fn ossh1(&mut self) -> _OSSH1W {
        _OSSH1W { w: self }
    }
    #[doc = "Bit 2 - Output Selection Set for PWMH output of the channel 2"]
    #[inline]
    pub fn ossh2(&mut self) -> _OSSH2W {
        _OSSH2W { w: self }
    }
    #[doc = "Bit 3 - Output Selection Set for PWMH output of the channel 3"]
    #[inline]
    pub fn ossh3(&mut self) -> _OSSH3W {
        _OSSH3W { w: self }
    }
    #[doc = "Bit 4 - Output Selection Set for PWMH output of the channel 4"]
    #[inline]
    pub fn ossh4(&mut self) -> _OSSH4W {
        _OSSH4W { w: self }
    }
    #[doc = "Bit 5 - Output Selection Set for PWMH output of the channel 5"]
    #[inline]
    pub fn ossh5(&mut self) -> _OSSH5W {
        _OSSH5W { w: self }
    }
    #[doc = "Bit 6 - Output Selection Set for PWMH output of the channel 6"]
    #[inline]
    pub fn ossh6(&mut self) -> _OSSH6W {
        _OSSH6W { w: self }
    }
    #[doc = "Bit 7 - Output Selection Set for PWMH output of the channel 7"]
    #[inline]
    pub fn ossh7(&mut self) -> _OSSH7W {
        _OSSH7W { w: self }
    }
    #[doc = "Bit 16 - Output Selection Set for PWML output of the channel 0"]
    #[inline]
    pub fn ossl0(&mut self) -> _OSSL0W {
        _OSSL0W { w: self }
    }
    #[doc = "Bit 17 - Output Selection Set for PWML output of the channel 1"]
    #[inline]
    pub fn ossl1(&mut self) -> _OSSL1W {
        _OSSL1W { w: self }
    }
    #[doc = "Bit 18 - Output Selection Set for PWML output of the channel 2"]
    #[inline]
    pub fn ossl2(&mut self) -> _OSSL2W {
        _OSSL2W { w: self }
    }
    #[doc = "Bit 19 - Output Selection Set for PWML output of the channel 3"]
    #[inline]
    pub fn ossl3(&mut self) -> _OSSL3W {
        _OSSL3W { w: self }
    }
    #[doc = "Bit 20 - Output Selection Set for PWML output of the channel 4"]
    #[inline]
    pub fn ossl4(&mut self) -> _OSSL4W {
        _OSSL4W { w: self }
    }
    #[doc = "Bit 21 - Output Selection Set for PWML output of the channel 5"]
    #[inline]
    pub fn ossl5(&mut self) -> _OSSL5W {
        _OSSL5W { w: self }
    }
    #[doc = "Bit 22 - Output Selection Set for PWML output of the channel 6"]
    #[inline]
    pub fn ossl6(&mut self) -> _OSSL6W {
        _OSSL6W { w: self }
    }
    #[doc = "Bit 23 - Output Selection Set for PWML output of the channel 7"]
    #[inline]
    pub fn ossl7(&mut self) -> _OSSL7W {
        _OSSL7W { w: self }
    }
}
