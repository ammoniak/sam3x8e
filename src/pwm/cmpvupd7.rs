#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMPVUPD7 {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let bits = self.register.get();
        let mut w = W { bits: bits };
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Proxy"]
pub struct _CVUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _CVUPDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CVMUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _CVMUPDW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value Update"]
    #[inline]
    pub fn cvupd(&mut self) -> _CVUPDW {
        _CVUPDW { w: self }
    }
    #[doc = "Bit 24 - Comparison x Value Mode Update"]
    #[inline]
    pub fn cvmupd(&mut self) -> _CVMUPDW {
        _CVMUPDW { w: self }
    }
}
