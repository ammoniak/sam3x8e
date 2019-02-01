#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {}
#[doc = "Values that can be written to the field `VROFF`"]
pub enum VROFFW {
    #[doc = "no effect."]
    NO_EFFECT,
    #[doc = "if KEY is correct, asserts vddcore_nreset and stops the voltage regulator."]
    STOP_VREG,
}
impl VROFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VROFFW::NO_EFFECT => false,
            VROFFW::STOP_VREG => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VROFFW<'a> {
    w: &'a mut W,
}
impl<'a> _VROFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VROFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(VROFFW::NO_EFFECT)
    }
    #[doc = "if KEY is correct, asserts vddcore_nreset and stops the voltage regulator."]
    #[inline]
    pub fn stop_vreg(self) -> &'a mut W {
        self.variant(VROFFW::STOP_VREG)
    }
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
#[doc = "Values that can be written to the field `XTALSEL`"]
pub enum XTALSELW {
    #[doc = "no effect."]
    NO_EFFECT,
    #[doc = "if KEY is correct, switches the slow clock on the crystal oscillator output."]
    CRYSTAL_SEL,
}
impl XTALSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            XTALSELW::NO_EFFECT => false,
            XTALSELW::CRYSTAL_SEL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _XTALSELW<'a> {
    w: &'a mut W,
}
impl<'a> _XTALSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: XTALSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(XTALSELW::NO_EFFECT)
    }
    #[doc = "if KEY is correct, switches the slow clock on the crystal oscillator output."]
    #[inline]
    pub fn crystal_sel(self) -> &'a mut W {
        self.variant(XTALSELW::CRYSTAL_SEL)
    }
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
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
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
    #[doc = "Bit 2 - Voltage Regulator Off"]
    #[inline]
    pub fn vroff(&mut self) -> _VROFFW {
        _VROFFW { w: self }
    }
    #[doc = "Bit 3 - Crystal Oscillator Select"]
    #[inline]
    pub fn xtalsel(&mut self) -> _XTALSELW {
        _XTALSELW { w: self }
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}
