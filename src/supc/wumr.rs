#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUMR {
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
#[doc = "Possible values of the field `FWUPEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUPENR {
    #[doc = "the Force Wake-up pin has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the Force Wake-up pin low forces the wake-up of the core power supply."]
    ENABLE,
}
impl FWUPENR {
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
            FWUPENR::NOT_ENABLE => false,
            FWUPENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWUPENR {
        match value {
            false => FWUPENR::NOT_ENABLE,
            true => FWUPENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == FWUPENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == FWUPENR::ENABLE
    }
}
#[doc = "Possible values of the field `SMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMENR {
    #[doc = "the supply monitor detection has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the supply monitor detection forces the wake-up of the core power supply."]
    ENABLE,
}
impl SMENR {
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
            SMENR::NOT_ENABLE => false,
            SMENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMENR {
        match value {
            false => SMENR::NOT_ENABLE,
            true => SMENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == SMENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SMENR::ENABLE
    }
}
#[doc = "Possible values of the field `RTTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTTENR {
    #[doc = "the RTT alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTTENR {
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
            RTTENR::NOT_ENABLE => false,
            RTTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTTENR {
        match value {
            false => RTTENR::NOT_ENABLE,
            true => RTTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == RTTENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTTENR::ENABLE
    }
}
#[doc = "Possible values of the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCENR {
    #[doc = "the RTC alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTCENR {
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
            RTCENR::NOT_ENABLE => false,
            RTCENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCENR {
        match value {
            false => RTCENR::NOT_ENABLE,
            true => RTCENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == RTCENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTCENR::ENABLE
    }
}
#[doc = "Possible values of the field `FWUPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUPDBCR {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "FWUP shall be low for at least 3 SLCK periods"]
    _3_SCLK,
    #[doc = "FWUP shall be low for at least 32 SLCK periods"]
    _32_SCLK,
    #[doc = "FWUP shall be low for at least 512 SLCK periods"]
    _512_SCLK,
    #[doc = "FWUP shall be low for at least 4,096 SLCK periods"]
    _4096_SCLK,
    #[doc = "FWUP shall be low for at least 32,768 SLCK periods"]
    _32768_SCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FWUPDBCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FWUPDBCR::IMMEDIATE => 0,
            FWUPDBCR::_3_SCLK => 1,
            FWUPDBCR::_32_SCLK => 2,
            FWUPDBCR::_512_SCLK => 3,
            FWUPDBCR::_4096_SCLK => 4,
            FWUPDBCR::_32768_SCLK => 5,
            FWUPDBCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FWUPDBCR {
        match value {
            0 => FWUPDBCR::IMMEDIATE,
            1 => FWUPDBCR::_3_SCLK,
            2 => FWUPDBCR::_32_SCLK,
            3 => FWUPDBCR::_512_SCLK,
            4 => FWUPDBCR::_4096_SCLK,
            5 => FWUPDBCR::_32768_SCLK,
            i => FWUPDBCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline]
    pub fn is_immediate(&self) -> bool {
        *self == FWUPDBCR::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SCLK`"]
    #[inline]
    pub fn is_3_sclk(&self) -> bool {
        *self == FWUPDBCR::_3_SCLK
    }
    #[doc = "Checks if the value of the field is `_32_SCLK`"]
    #[inline]
    pub fn is_32_sclk(&self) -> bool {
        *self == FWUPDBCR::_32_SCLK
    }
    #[doc = "Checks if the value of the field is `_512_SCLK`"]
    #[inline]
    pub fn is_512_sclk(&self) -> bool {
        *self == FWUPDBCR::_512_SCLK
    }
    #[doc = "Checks if the value of the field is `_4096_SCLK`"]
    #[inline]
    pub fn is_4096_sclk(&self) -> bool {
        *self == FWUPDBCR::_4096_SCLK
    }
    #[doc = "Checks if the value of the field is `_32768_SCLK`"]
    #[inline]
    pub fn is_32768_sclk(&self) -> bool {
        *self == FWUPDBCR::_32768_SCLK
    }
}
#[doc = "Possible values of the field `WKUPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPDBCR {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SCLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WKUPDBCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WKUPDBCR::IMMEDIATE => 0,
            WKUPDBCR::_3_SCLK => 1,
            WKUPDBCR::_32_SCLK => 2,
            WKUPDBCR::_512_SCLK => 3,
            WKUPDBCR::_4096_SCLK => 4,
            WKUPDBCR::_32768_SCLK => 5,
            WKUPDBCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WKUPDBCR {
        match value {
            0 => WKUPDBCR::IMMEDIATE,
            1 => WKUPDBCR::_3_SCLK,
            2 => WKUPDBCR::_32_SCLK,
            3 => WKUPDBCR::_512_SCLK,
            4 => WKUPDBCR::_4096_SCLK,
            5 => WKUPDBCR::_32768_SCLK,
            i => WKUPDBCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline]
    pub fn is_immediate(&self) -> bool {
        *self == WKUPDBCR::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SCLK`"]
    #[inline]
    pub fn is_3_sclk(&self) -> bool {
        *self == WKUPDBCR::_3_SCLK
    }
    #[doc = "Checks if the value of the field is `_32_SCLK`"]
    #[inline]
    pub fn is_32_sclk(&self) -> bool {
        *self == WKUPDBCR::_32_SCLK
    }
    #[doc = "Checks if the value of the field is `_512_SCLK`"]
    #[inline]
    pub fn is_512_sclk(&self) -> bool {
        *self == WKUPDBCR::_512_SCLK
    }
    #[doc = "Checks if the value of the field is `_4096_SCLK`"]
    #[inline]
    pub fn is_4096_sclk(&self) -> bool {
        *self == WKUPDBCR::_4096_SCLK
    }
    #[doc = "Checks if the value of the field is `_32768_SCLK`"]
    #[inline]
    pub fn is_32768_sclk(&self) -> bool {
        *self == WKUPDBCR::_32768_SCLK
    }
}
#[doc = "Values that can be written to the field `FWUPEN`"]
pub enum FWUPENW {
    #[doc = "the Force Wake-up pin has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the Force Wake-up pin low forces the wake-up of the core power supply."]
    ENABLE,
}
impl FWUPENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWUPENW::NOT_ENABLE => false,
            FWUPENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWUPENW<'a> {
    w: &'a mut W,
}
impl<'a> _FWUPENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWUPENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the Force Wake-up pin has no wake-up effect."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(FWUPENW::NOT_ENABLE)
    }
    #[doc = "the Force Wake-up pin low forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(FWUPENW::ENABLE)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMEN`"]
pub enum SMENW {
    #[doc = "the supply monitor detection has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the supply monitor detection forces the wake-up of the core power supply."]
    ENABLE,
}
impl SMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMENW::NOT_ENABLE => false,
            SMENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the supply monitor detection has no wake-up effect."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMENW::NOT_ENABLE)
    }
    #[doc = "the supply monitor detection forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMENW::ENABLE)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTTEN`"]
pub enum RTTENW {
    #[doc = "the RTT alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTTENW::NOT_ENABLE => false,
            RTTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the RTT alarm signal has no wake-up effect."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTTENW::NOT_ENABLE)
    }
    #[doc = "the RTT alarm signal forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTTENW::ENABLE)
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
#[doc = "Values that can be written to the field `RTCEN`"]
pub enum RTCENW {
    #[doc = "the RTC alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "the RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCENW::NOT_ENABLE => false,
            RTCENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the RTC alarm signal has no wake-up effect."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTCENW::NOT_ENABLE)
    }
    #[doc = "the RTC alarm signal forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCENW::ENABLE)
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
#[doc = "Values that can be written to the field `FWUPDBC`"]
pub enum FWUPDBCW {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "FWUP shall be low for at least 3 SLCK periods"]
    _3_SCLK,
    #[doc = "FWUP shall be low for at least 32 SLCK periods"]
    _32_SCLK,
    #[doc = "FWUP shall be low for at least 512 SLCK periods"]
    _512_SCLK,
    #[doc = "FWUP shall be low for at least 4,096 SLCK periods"]
    _4096_SCLK,
    #[doc = "FWUP shall be low for at least 32,768 SLCK periods"]
    _32768_SCLK,
}
impl FWUPDBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FWUPDBCW::IMMEDIATE => 0,
            FWUPDBCW::_3_SCLK => 1,
            FWUPDBCW::_32_SCLK => 2,
            FWUPDBCW::_512_SCLK => 3,
            FWUPDBCW::_4096_SCLK => 4,
            FWUPDBCW::_32768_SCLK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWUPDBCW<'a> {
    w: &'a mut W,
}
impl<'a> _FWUPDBCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWUPDBCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline]
    pub fn immediate(self) -> &'a mut W {
        self.variant(FWUPDBCW::IMMEDIATE)
    }
    #[doc = "FWUP shall be low for at least 3 SLCK periods"]
    #[inline]
    pub fn _3_sclk(self) -> &'a mut W {
        self.variant(FWUPDBCW::_3_SCLK)
    }
    #[doc = "FWUP shall be low for at least 32 SLCK periods"]
    #[inline]
    pub fn _32_sclk(self) -> &'a mut W {
        self.variant(FWUPDBCW::_32_SCLK)
    }
    #[doc = "FWUP shall be low for at least 512 SLCK periods"]
    #[inline]
    pub fn _512_sclk(self) -> &'a mut W {
        self.variant(FWUPDBCW::_512_SCLK)
    }
    #[doc = "FWUP shall be low for at least 4,096 SLCK periods"]
    #[inline]
    pub fn _4096_sclk(self) -> &'a mut W {
        self.variant(FWUPDBCW::_4096_SCLK)
    }
    #[doc = "FWUP shall be low for at least 32,768 SLCK periods"]
    #[inline]
    pub fn _32768_sclk(self) -> &'a mut W {
        self.variant(FWUPDBCW::_32768_SCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPDBC`"]
pub enum WKUPDBCW {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SCLK,
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SCLK,
}
impl WKUPDBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WKUPDBCW::IMMEDIATE => 0,
            WKUPDBCW::_3_SCLK => 1,
            WKUPDBCW::_32_SCLK => 2,
            WKUPDBCW::_512_SCLK => 3,
            WKUPDBCW::_4096_SCLK => 4,
            WKUPDBCW::_32768_SCLK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPDBCW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPDBCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPDBCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline]
    pub fn immediate(self) -> &'a mut W {
        self.variant(WKUPDBCW::IMMEDIATE)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline]
    pub fn _3_sclk(self) -> &'a mut W {
        self.variant(WKUPDBCW::_3_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline]
    pub fn _32_sclk(self) -> &'a mut W {
        self.variant(WKUPDBCW::_32_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline]
    pub fn _512_sclk(self) -> &'a mut W {
        self.variant(WKUPDBCW::_512_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline]
    pub fn _4096_sclk(self) -> &'a mut W {
        self.variant(WKUPDBCW::_4096_SCLK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline]
    pub fn _32768_sclk(self) -> &'a mut W {
        self.variant(WKUPDBCW::_32768_SCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 0 - Force Wake-up Enable"]
    #[inline]
    pub fn fwupen(&self) -> FWUPENR {
        FWUPENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline]
    pub fn smen(&self) -> SMENR {
        SMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Real Time Timer Wake-up Enable"]
    #[inline]
    pub fn rtten(&self) -> RTTENR {
        RTTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Real Time Clock Wake-up Enable"]
    #[inline]
    pub fn rtcen(&self) -> RTCENR {
        RTCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Force Wake-up Debouncer Period"]
    #[inline]
    pub fn fwupdbc(&self) -> FWUPDBCR {
        FWUPDBCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline]
    pub fn wkupdbc(&self) -> WKUPDBCR {
        WKUPDBCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bit 0 - Force Wake-up Enable"]
    #[inline]
    pub fn fwupen(&mut self) -> _FWUPENW {
        _FWUPENW { w: self }
    }
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline]
    pub fn smen(&mut self) -> _SMENW {
        _SMENW { w: self }
    }
    #[doc = "Bit 2 - Real Time Timer Wake-up Enable"]
    #[inline]
    pub fn rtten(&mut self) -> _RTTENW {
        _RTTENW { w: self }
    }
    #[doc = "Bit 3 - Real Time Clock Wake-up Enable"]
    #[inline]
    pub fn rtcen(&mut self) -> _RTCENW {
        _RTCENW { w: self }
    }
    #[doc = "Bits 8:10 - Force Wake-up Debouncer Period"]
    #[inline]
    pub fn fwupdbc(&mut self) -> _FWUPDBCW {
        _FWUPDBCW { w: self }
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline]
    pub fn wkupdbc(&mut self) -> _WKUPDBCW {
        _WKUPDBCW { w: self }
    }
}
