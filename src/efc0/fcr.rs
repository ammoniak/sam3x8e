#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCR {}
#[doc = r" Proxy"]
pub struct _FCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _FCMDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FARGW<'a> {
    w: &'a mut W,
}
impl<'a> _FARGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _FKEYW<'a> {
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
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline]
    pub fn fcmd(&mut self) -> _FCMDW {
        _FCMDW { w: self }
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline]
    pub fn farg(&mut self) -> _FARGW {
        _FARGW { w: self }
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline]
    pub fn fkey(&mut self) -> _FKEYW {
        _FKEYW { w: self }
    }
}
