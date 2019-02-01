#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MATRIX_SCFG {
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
}
#[doc = r" Value of the field"]
pub struct SLOT_CYCLER {
    bits: u8,
}
impl SLOT_CYCLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEFMSTR_TYPER {
    bits: u8,
}
impl DEFMSTR_TYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FIXED_DEFMSTRR {
    bits: u8,
}
impl FIXED_DEFMSTRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ARBTR {
    bits: u8,
}
impl ARBTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SLOT_CYCLEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOT_CYCLEW<'a> {
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
pub struct _DEFMSTR_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _DEFMSTR_TYPEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FIXED_DEFMSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _FIXED_DEFMSTRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ARBTW<'a> {
    w: &'a mut W,
}
impl<'a> _ARBTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline]
    pub fn slot_cycle(&self) -> SLOT_CYCLER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SLOT_CYCLER { bits }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEFMSTR_TYPER { bits }
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTRR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FIXED_DEFMSTRR { bits }
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline]
    pub fn arbt(&self) -> ARBTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ARBTR { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline]
    pub fn slot_cycle(&mut self) -> _SLOT_CYCLEW {
        _SLOT_CYCLEW { w: self }
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline]
    pub fn defmstr_type(&mut self) -> _DEFMSTR_TYPEW {
        _DEFMSTR_TYPEW { w: self }
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline]
    pub fn fixed_defmstr(&mut self) -> _FIXED_DEFMSTRW {
        _FIXED_DEFMSTRW { w: self }
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline]
    pub fn arbt(&mut self) -> _ARBTW {
        _ARBTW { w: self }
    }
}
