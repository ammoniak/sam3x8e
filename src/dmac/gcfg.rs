#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::GCFG {
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
#[doc = "Possible values of the field `ARB_CFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_CFGR {
    #[doc = "Fixed priority arbiter (see \"Basic Definitions\" )"]
    FIXED,
    #[doc = "Modified round robin arbiter."]
    ROUND_ROBIN,
}
impl ARB_CFGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ARB_CFGR::FIXED => false,
            ARB_CFGR::ROUND_ROBIN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ARB_CFGR {
        match value {
            false => ARB_CFGR::FIXED,
            true => ARB_CFGR::ROUND_ROBIN,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline]
    pub fn is_fixed(&self) -> bool {
        *self == ARB_CFGR::FIXED
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN`"]
    #[inline]
    pub fn is_round_robin(&self) -> bool {
        *self == ARB_CFGR::ROUND_ROBIN
    }
}
#[doc = "Values that can be written to the field `ARB_CFG`"]
pub enum ARB_CFGW {
    #[doc = "Fixed priority arbiter (see \"Basic Definitions\" )"]
    FIXED,
    #[doc = "Modified round robin arbiter."]
    ROUND_ROBIN,
}
impl ARB_CFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ARB_CFGW::FIXED => false,
            ARB_CFGW::ROUND_ROBIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ARB_CFGW<'a> {
    w: &'a mut W,
}
impl<'a> _ARB_CFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ARB_CFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Fixed priority arbiter (see \"Basic Definitions\" )"]
    #[inline]
    pub fn fixed(self) -> &'a mut W {
        self.variant(ARB_CFGW::FIXED)
    }
    #[doc = "Modified round robin arbiter."]
    #[inline]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(ARB_CFGW::ROUND_ROBIN)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline]
    pub fn arb_cfg(&self) -> ARB_CFGR {
        ARB_CFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline]
    pub fn arb_cfg(&mut self) -> _ARB_CFGW {
        _ARB_CFGW { w: self }
    }
}
