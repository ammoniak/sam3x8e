#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_PCK {
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
#[doc = "Possible values of the field `CSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSR {
    #[doc = "Slow Clock is selected"]
    SLOW_CLK,
    #[doc = "Main Clock is selected"]
    MAIN_CLK,
    #[doc = "PLLA Clock is selected"]
    PLLA_CLK,
    #[doc = "UPLL Clock is selected"]
    UPLL_CLK,
    #[doc = "Master Clock is selected"]
    MCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSSR::SLOW_CLK => 0,
            CSSR::MAIN_CLK => 1,
            CSSR::PLLA_CLK => 2,
            CSSR::UPLL_CLK => 3,
            CSSR::MCK => 4,
            CSSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSSR {
        match value {
            0 => CSSR::SLOW_CLK,
            1 => CSSR::MAIN_CLK,
            2 => CSSR::PLLA_CLK,
            3 => CSSR::UPLL_CLK,
            4 => CSSR::MCK,
            i => CSSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSSR::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline]
    pub fn is_main_clk(&self) -> bool {
        *self == CSSR::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSSR::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline]
    pub fn is_upll_clk(&self) -> bool {
        *self == CSSR::UPLL_CLK
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline]
    pub fn is_mck(&self) -> bool {
        *self == CSSR::MCK
    }
}
#[doc = "Possible values of the field `PRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESR {
    #[doc = "Selected clock"]
    CLK_1,
    #[doc = "Selected clock divided by 2"]
    CLK_2,
    #[doc = "Selected clock divided by 4"]
    CLK_4,
    #[doc = "Selected clock divided by 8"]
    CLK_8,
    #[doc = "Selected clock divided by 16"]
    CLK_16,
    #[doc = "Selected clock divided by 32"]
    CLK_32,
    #[doc = "Selected clock divided by 64"]
    CLK_64,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESR::CLK_1 => 0,
            PRESR::CLK_2 => 1,
            PRESR::CLK_4 => 2,
            PRESR::CLK_8 => 3,
            PRESR::CLK_16 => 4,
            PRESR::CLK_32 => 5,
            PRESR::CLK_64 => 6,
            PRESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESR {
        match value {
            0 => PRESR::CLK_1,
            1 => PRESR::CLK_2,
            2 => PRESR::CLK_4,
            3 => PRESR::CLK_8,
            4 => PRESR::CLK_16,
            5 => PRESR::CLK_32,
            6 => PRESR::CLK_64,
            i => PRESR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_1`"]
    #[inline]
    pub fn is_clk_1(&self) -> bool {
        *self == PRESR::CLK_1
    }
    #[doc = "Checks if the value of the field is `CLK_2`"]
    #[inline]
    pub fn is_clk_2(&self) -> bool {
        *self == PRESR::CLK_2
    }
    #[doc = "Checks if the value of the field is `CLK_4`"]
    #[inline]
    pub fn is_clk_4(&self) -> bool {
        *self == PRESR::CLK_4
    }
    #[doc = "Checks if the value of the field is `CLK_8`"]
    #[inline]
    pub fn is_clk_8(&self) -> bool {
        *self == PRESR::CLK_8
    }
    #[doc = "Checks if the value of the field is `CLK_16`"]
    #[inline]
    pub fn is_clk_16(&self) -> bool {
        *self == PRESR::CLK_16
    }
    #[doc = "Checks if the value of the field is `CLK_32`"]
    #[inline]
    pub fn is_clk_32(&self) -> bool {
        *self == PRESR::CLK_32
    }
    #[doc = "Checks if the value of the field is `CLK_64`"]
    #[inline]
    pub fn is_clk_64(&self) -> bool {
        *self == PRESR::CLK_64
    }
}
#[doc = "Values that can be written to the field `CSS`"]
pub enum CSSW {
    #[doc = "Slow Clock is selected"]
    SLOW_CLK,
    #[doc = "Main Clock is selected"]
    MAIN_CLK,
    #[doc = "PLLA Clock is selected"]
    PLLA_CLK,
    #[doc = "UPLL Clock is selected"]
    UPLL_CLK,
    #[doc = "Master Clock is selected"]
    MCK,
}
impl CSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSSW::SLOW_CLK => 0,
            CSSW::MAIN_CLK => 1,
            CSSW::PLLA_CLK => 2,
            CSSW::UPLL_CLK => 3,
            CSSW::MCK => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSSW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Slow Clock is selected"]
    #[inline]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSSW::SLOW_CLK)
    }
    #[doc = "Main Clock is selected"]
    #[inline]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSSW::MAIN_CLK)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSSW::PLLA_CLK)
    }
    #[doc = "UPLL Clock is selected"]
    #[inline]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(CSSW::UPLL_CLK)
    }
    #[doc = "Master Clock is selected"]
    #[inline]
    pub fn mck(self) -> &'a mut W {
        self.variant(CSSW::MCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRES`"]
pub enum PRESW {
    #[doc = "Selected clock"]
    CLK_1,
    #[doc = "Selected clock divided by 2"]
    CLK_2,
    #[doc = "Selected clock divided by 4"]
    CLK_4,
    #[doc = "Selected clock divided by 8"]
    CLK_8,
    #[doc = "Selected clock divided by 16"]
    CLK_16,
    #[doc = "Selected clock divided by 32"]
    CLK_32,
    #[doc = "Selected clock divided by 64"]
    CLK_64,
}
impl PRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESW::CLK_1 => 0,
            PRESW::CLK_2 => 1,
            PRESW::CLK_4 => 2,
            PRESW::CLK_8 => 3,
            PRESW::CLK_16 => 4,
            PRESW::CLK_32 => 5,
            PRESW::CLK_64 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Selected clock"]
    #[inline]
    pub fn clk_1(self) -> &'a mut W {
        self.variant(PRESW::CLK_1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(PRESW::CLK_2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline]
    pub fn clk_4(self) -> &'a mut W {
        self.variant(PRESW::CLK_4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline]
    pub fn clk_8(self) -> &'a mut W {
        self.variant(PRESW::CLK_8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline]
    pub fn clk_16(self) -> &'a mut W {
        self.variant(PRESW::CLK_16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline]
    pub fn clk_32(self) -> &'a mut W {
        self.variant(PRESW::CLK_32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline]
    pub fn clk_64(self) -> &'a mut W {
        self.variant(PRESW::CLK_64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline]
    pub fn css(&self) -> CSSR {
        CSSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline]
    pub fn pres(&self) -> PRESR {
        PRESR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline]
    pub fn css(&mut self) -> _CSSW {
        _CSSW { w: self }
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline]
    pub fn pres(&mut self) -> _PRESW {
        _PRESW { w: self }
    }
}
