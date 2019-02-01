#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WPCR {}
#[doc = r" Proxy"]
pub struct _WPCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _WPCMDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WPRG0W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG0W<'a> {
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
pub struct _WPRG1W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG1W<'a> {
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
pub struct _WPRG2W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG2W<'a> {
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
pub struct _WPRG3W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG3W<'a> {
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
pub struct _WPRG4W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG4W<'a> {
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
pub struct _WPRG5W<'a> {
    w: &'a mut W,
}
impl<'a> _WPRG5W<'a> {
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
pub struct _WPKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _WPKEYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - Write Protect Command"]
    #[inline]
    pub fn wpcmd(&mut self) -> _WPCMDW {
        _WPCMDW { w: self }
    }
    #[doc = "Bit 2 - Write Protect Register Group 0"]
    #[inline]
    pub fn wprg0(&mut self) -> _WPRG0W {
        _WPRG0W { w: self }
    }
    #[doc = "Bit 3 - Write Protect Register Group 1"]
    #[inline]
    pub fn wprg1(&mut self) -> _WPRG1W {
        _WPRG1W { w: self }
    }
    #[doc = "Bit 4 - Write Protect Register Group 2"]
    #[inline]
    pub fn wprg2(&mut self) -> _WPRG2W {
        _WPRG2W { w: self }
    }
    #[doc = "Bit 5 - Write Protect Register Group 3"]
    #[inline]
    pub fn wprg3(&mut self) -> _WPRG3W {
        _WPRG3W { w: self }
    }
    #[doc = "Bit 6 - Write Protect Register Group 4"]
    #[inline]
    pub fn wprg4(&mut self) -> _WPRG4W {
        _WPRG4W { w: self }
    }
    #[doc = "Bit 7 - Write Protect Register Group 5"]
    #[inline]
    pub fn wprg5(&mut self) -> _WPRG5W {
        _WPRG5W { w: self }
    }
    #[doc = "Bits 8:31 - Write Protect Key"]
    #[inline]
    pub fn wpkey(&mut self) -> _WPKEYW {
        _WPKEYW { w: self }
    }
}
