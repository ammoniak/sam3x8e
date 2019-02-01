#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SEQR2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct USCH9R {
    bits: u8,
}
impl USCH9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH10R {
    bits: u8,
}
impl USCH10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH11R {
    bits: u8,
}
impl USCH11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH12R {
    bits: u8,
}
impl USCH12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH13R {
    bits: u8,
}
impl USCH13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH14R {
    bits: u8,
}
impl USCH14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH15R {
    bits: u8,
}
impl USCH15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct USCH16R {
    bits: u8,
}
impl USCH16R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _USCH9W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH9W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH10W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH10W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH11W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH11W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH12W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH12W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH13W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH13W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH14W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH14W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH15W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH15W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USCH16W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH16W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - User Sequence Number 9"]
    #[inline]
    pub fn usch9(&self) -> USCH9R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH9R { bits }
    }
    #[doc = "Bits 4:7 - User Sequence Number 10"]
    #[inline]
    pub fn usch10(&self) -> USCH10R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH10R { bits }
    }
    #[doc = "Bits 8:11 - User Sequence Number 11"]
    #[inline]
    pub fn usch11(&self) -> USCH11R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH11R { bits }
    }
    #[doc = "Bits 12:15 - User Sequence Number 12"]
    #[inline]
    pub fn usch12(&self) -> USCH12R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH12R { bits }
    }
    #[doc = "Bits 16:19 - User Sequence Number 13"]
    #[inline]
    pub fn usch13(&self) -> USCH13R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH13R { bits }
    }
    #[doc = "Bits 20:23 - User Sequence Number 14"]
    #[inline]
    pub fn usch14(&self) -> USCH14R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH14R { bits }
    }
    #[doc = "Bits 24:27 - User Sequence Number 15"]
    #[inline]
    pub fn usch15(&self) -> USCH15R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH15R { bits }
    }
    #[doc = "Bits 28:31 - User Sequence Number 16"]
    #[inline]
    pub fn usch16(&self) -> USCH16R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        USCH16R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - User Sequence Number 9"]
    #[inline]
    pub fn usch9(&mut self) -> _USCH9W {
        _USCH9W { w: self }
    }
    #[doc = "Bits 4:7 - User Sequence Number 10"]
    #[inline]
    pub fn usch10(&mut self) -> _USCH10W {
        _USCH10W { w: self }
    }
    #[doc = "Bits 8:11 - User Sequence Number 11"]
    #[inline]
    pub fn usch11(&mut self) -> _USCH11W {
        _USCH11W { w: self }
    }
    #[doc = "Bits 12:15 - User Sequence Number 12"]
    #[inline]
    pub fn usch12(&mut self) -> _USCH12W {
        _USCH12W { w: self }
    }
    #[doc = "Bits 16:19 - User Sequence Number 13"]
    #[inline]
    pub fn usch13(&mut self) -> _USCH13W {
        _USCH13W { w: self }
    }
    #[doc = "Bits 20:23 - User Sequence Number 14"]
    #[inline]
    pub fn usch14(&mut self) -> _USCH14W {
        _USCH14W { w: self }
    }
    #[doc = "Bits 24:27 - User Sequence Number 15"]
    #[inline]
    pub fn usch15(&mut self) -> _USCH15W {
        _USCH15W { w: self }
    }
    #[doc = "Bits 28:31 - User Sequence Number 16"]
    #[inline]
    pub fn usch16(&mut self) -> _USCH16W {
        _USCH16W { w: self }
    }
}
