#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MR {
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
#[doc = "Possible values of the field `TRGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGENR {
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    DIS,
    #[doc = "External trigger mode enabled."]
    EN,
}
impl TRGENR {
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
            TRGENR::DIS => false,
            TRGENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGENR {
        match value {
            false => TRGENR::DIS,
            true => TRGENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TRGENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TRGENR::EN
    }
}
#[doc = r" Value of the field"]
pub struct TRGSELR {
    bits: u8,
}
impl TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WORD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORDR {
    #[doc = "Half-Word transfer"]
    HALF,
    #[doc = "Word Transfer"]
    WORD,
}
impl WORDR {
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
            WORDR::HALF => false,
            WORDR::WORD => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WORDR {
        match value {
            false => WORDR::HALF,
            true => WORDR::WORD,
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == WORDR::HALF
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline]
    pub fn is_word(&self) -> bool {
        *self == WORDR::WORD
    }
}
#[doc = r" Value of the field"]
pub struct SLEEPR {
    bits: bool,
}
impl SLEEPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct FASTWKUPR {
    bits: bool,
}
impl FASTWKUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct REFRESHR {
    bits: u8,
}
impl REFRESHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `USER_SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USER_SELR {
    #[doc = "Channel 0"]
    CHANNEL0,
    #[doc = "Channel 1"]
    CHANNEL1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USER_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USER_SELR::CHANNEL0 => 0,
            USER_SELR::CHANNEL1 => 1,
            USER_SELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USER_SELR {
        match value {
            0 => USER_SELR::CHANNEL0,
            1 => USER_SELR::CHANNEL1,
            i => USER_SELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL0`"]
    #[inline]
    pub fn is_channel0(&self) -> bool {
        *self == USER_SELR::CHANNEL0
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline]
    pub fn is_channel1(&self) -> bool {
        *self == USER_SELR::CHANNEL1
    }
}
#[doc = "Possible values of the field `TAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAGR {
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    DIS,
    #[doc = "Tag selection mode enabled"]
    EN,
}
impl TAGR {
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
            TAGR::DIS => false,
            TAGR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAGR {
        match value {
            false => TAGR::DIS,
            true => TAGR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TAGR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TAGR::EN
    }
}
#[doc = "Possible values of the field `MAXS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXSR {
    #[doc = "Normal Mode"]
    NORMAL,
    #[doc = "Max Speed Mode enabled"]
    MAXIMUM,
}
impl MAXSR {
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
            MAXSR::NORMAL => false,
            MAXSR::MAXIMUM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAXSR {
        match value {
            false => MAXSR::NORMAL,
            true => MAXSR::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == MAXSR::NORMAL
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline]
    pub fn is_maximum(&self) -> bool {
        *self == MAXSR::MAXIMUM
    }
}
#[doc = "Possible values of the field `STARTUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUPR {
    #[doc = "0 periods of DACClock"]
    _0,
    #[doc = "8 periods of DACClock"]
    _8,
    #[doc = "16 periods of DACClock"]
    _16,
    #[doc = "24 periods of DACClock"]
    _24,
    #[doc = "64 periods of DACClock"]
    _64,
    #[doc = "80 periods of DACClock"]
    _80,
    #[doc = "96 periods of DACClock"]
    _96,
    #[doc = "112 periods of DACClock"]
    _112,
    #[doc = "512 periods of DACClock"]
    _512,
    #[doc = "576 periods of DACClock"]
    _576,
    #[doc = "640 periods of DACClock"]
    _640,
    #[doc = "704 periods of DACClock"]
    _704,
    #[doc = "768 periods of DACClock"]
    _768,
    #[doc = "832 periods of DACClock"]
    _832,
    #[doc = "896 periods of DACClock"]
    _896,
    #[doc = "960 periods of DACClock"]
    _960,
    #[doc = "1024 periods of DACClock"]
    _1024,
    #[doc = "1088 periods of DACClock"]
    _1088,
    #[doc = "1152 periods of DACClock"]
    _1152,
    #[doc = "1216 periods of DACClock"]
    _1216,
    #[doc = "1280 periods of DACClock"]
    _1280,
    #[doc = "1344 periods of DACClock"]
    _1344,
    #[doc = "1408 periods of DACClock"]
    _1408,
    #[doc = "1472 periods of DACClock"]
    _1472,
    #[doc = "1536 periods of DACClock"]
    _1536,
    #[doc = "1600 periods of DACClock"]
    _1600,
    #[doc = "1664 periods of DACClock"]
    _1664,
    #[doc = "1728 periods of DACClock"]
    _1728,
    #[doc = "1792 periods of DACClock"]
    _1792,
    #[doc = "1856 periods of DACClock"]
    _1856,
    #[doc = "1920 periods of DACClock"]
    _1920,
    #[doc = "1984 periods of DACClock"]
    _1984,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STARTUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STARTUPR::_0 => 0,
            STARTUPR::_8 => 1,
            STARTUPR::_16 => 2,
            STARTUPR::_24 => 3,
            STARTUPR::_64 => 4,
            STARTUPR::_80 => 5,
            STARTUPR::_96 => 6,
            STARTUPR::_112 => 7,
            STARTUPR::_512 => 8,
            STARTUPR::_576 => 9,
            STARTUPR::_640 => 10,
            STARTUPR::_704 => 11,
            STARTUPR::_768 => 12,
            STARTUPR::_832 => 13,
            STARTUPR::_896 => 14,
            STARTUPR::_960 => 15,
            STARTUPR::_1024 => 16,
            STARTUPR::_1088 => 17,
            STARTUPR::_1152 => 18,
            STARTUPR::_1216 => 19,
            STARTUPR::_1280 => 20,
            STARTUPR::_1344 => 21,
            STARTUPR::_1408 => 22,
            STARTUPR::_1472 => 23,
            STARTUPR::_1536 => 24,
            STARTUPR::_1600 => 25,
            STARTUPR::_1664 => 26,
            STARTUPR::_1728 => 27,
            STARTUPR::_1792 => 28,
            STARTUPR::_1856 => 29,
            STARTUPR::_1920 => 30,
            STARTUPR::_1984 => 31,
            STARTUPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTUPR {
        match value {
            0 => STARTUPR::_0,
            1 => STARTUPR::_8,
            2 => STARTUPR::_16,
            3 => STARTUPR::_24,
            4 => STARTUPR::_64,
            5 => STARTUPR::_80,
            6 => STARTUPR::_96,
            7 => STARTUPR::_112,
            8 => STARTUPR::_512,
            9 => STARTUPR::_576,
            10 => STARTUPR::_640,
            11 => STARTUPR::_704,
            12 => STARTUPR::_768,
            13 => STARTUPR::_832,
            14 => STARTUPR::_896,
            15 => STARTUPR::_960,
            16 => STARTUPR::_1024,
            17 => STARTUPR::_1088,
            18 => STARTUPR::_1152,
            19 => STARTUPR::_1216,
            20 => STARTUPR::_1280,
            21 => STARTUPR::_1344,
            22 => STARTUPR::_1408,
            23 => STARTUPR::_1472,
            24 => STARTUPR::_1536,
            25 => STARTUPR::_1600,
            26 => STARTUPR::_1664,
            27 => STARTUPR::_1728,
            28 => STARTUPR::_1792,
            29 => STARTUPR::_1856,
            30 => STARTUPR::_1920,
            31 => STARTUPR::_1984,
            i => STARTUPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STARTUPR::_0
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == STARTUPR::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == STARTUPR::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline]
    pub fn is_24(&self) -> bool {
        *self == STARTUPR::_24
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline]
    pub fn is_64(&self) -> bool {
        *self == STARTUPR::_64
    }
    #[doc = "Checks if the value of the field is `_80`"]
    #[inline]
    pub fn is_80(&self) -> bool {
        *self == STARTUPR::_80
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline]
    pub fn is_96(&self) -> bool {
        *self == STARTUPR::_96
    }
    #[doc = "Checks if the value of the field is `_112`"]
    #[inline]
    pub fn is_112(&self) -> bool {
        *self == STARTUPR::_112
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline]
    pub fn is_512(&self) -> bool {
        *self == STARTUPR::_512
    }
    #[doc = "Checks if the value of the field is `_576`"]
    #[inline]
    pub fn is_576(&self) -> bool {
        *self == STARTUPR::_576
    }
    #[doc = "Checks if the value of the field is `_640`"]
    #[inline]
    pub fn is_640(&self) -> bool {
        *self == STARTUPR::_640
    }
    #[doc = "Checks if the value of the field is `_704`"]
    #[inline]
    pub fn is_704(&self) -> bool {
        *self == STARTUPR::_704
    }
    #[doc = "Checks if the value of the field is `_768`"]
    #[inline]
    pub fn is_768(&self) -> bool {
        *self == STARTUPR::_768
    }
    #[doc = "Checks if the value of the field is `_832`"]
    #[inline]
    pub fn is_832(&self) -> bool {
        *self == STARTUPR::_832
    }
    #[doc = "Checks if the value of the field is `_896`"]
    #[inline]
    pub fn is_896(&self) -> bool {
        *self == STARTUPR::_896
    }
    #[doc = "Checks if the value of the field is `_960`"]
    #[inline]
    pub fn is_960(&self) -> bool {
        *self == STARTUPR::_960
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline]
    pub fn is_1024(&self) -> bool {
        *self == STARTUPR::_1024
    }
    #[doc = "Checks if the value of the field is `_1088`"]
    #[inline]
    pub fn is_1088(&self) -> bool {
        *self == STARTUPR::_1088
    }
    #[doc = "Checks if the value of the field is `_1152`"]
    #[inline]
    pub fn is_1152(&self) -> bool {
        *self == STARTUPR::_1152
    }
    #[doc = "Checks if the value of the field is `_1216`"]
    #[inline]
    pub fn is_1216(&self) -> bool {
        *self == STARTUPR::_1216
    }
    #[doc = "Checks if the value of the field is `_1280`"]
    #[inline]
    pub fn is_1280(&self) -> bool {
        *self == STARTUPR::_1280
    }
    #[doc = "Checks if the value of the field is `_1344`"]
    #[inline]
    pub fn is_1344(&self) -> bool {
        *self == STARTUPR::_1344
    }
    #[doc = "Checks if the value of the field is `_1408`"]
    #[inline]
    pub fn is_1408(&self) -> bool {
        *self == STARTUPR::_1408
    }
    #[doc = "Checks if the value of the field is `_1472`"]
    #[inline]
    pub fn is_1472(&self) -> bool {
        *self == STARTUPR::_1472
    }
    #[doc = "Checks if the value of the field is `_1536`"]
    #[inline]
    pub fn is_1536(&self) -> bool {
        *self == STARTUPR::_1536
    }
    #[doc = "Checks if the value of the field is `_1600`"]
    #[inline]
    pub fn is_1600(&self) -> bool {
        *self == STARTUPR::_1600
    }
    #[doc = "Checks if the value of the field is `_1664`"]
    #[inline]
    pub fn is_1664(&self) -> bool {
        *self == STARTUPR::_1664
    }
    #[doc = "Checks if the value of the field is `_1728`"]
    #[inline]
    pub fn is_1728(&self) -> bool {
        *self == STARTUPR::_1728
    }
    #[doc = "Checks if the value of the field is `_1792`"]
    #[inline]
    pub fn is_1792(&self) -> bool {
        *self == STARTUPR::_1792
    }
    #[doc = "Checks if the value of the field is `_1856`"]
    #[inline]
    pub fn is_1856(&self) -> bool {
        *self == STARTUPR::_1856
    }
    #[doc = "Checks if the value of the field is `_1920`"]
    #[inline]
    pub fn is_1920(&self) -> bool {
        *self == STARTUPR::_1920
    }
    #[doc = "Checks if the value of the field is `_1984`"]
    #[inline]
    pub fn is_1984(&self) -> bool {
        *self == STARTUPR::_1984
    }
}
#[doc = "Values that can be written to the field `TRGEN`"]
pub enum TRGENW {
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    DIS,
    #[doc = "External trigger mode enabled."]
    EN,
}
impl TRGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGENW::DIS => false,
            TRGENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGENW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGENW::DIS)
    }
    #[doc = "External trigger mode enabled."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGENW::EN)
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
#[doc = r" Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WORD`"]
pub enum WORDW {
    #[doc = "Half-Word transfer"]
    HALF,
    #[doc = "Word Transfer"]
    WORD,
}
impl WORDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WORDW::HALF => false,
            WORDW::WORD => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WORDW<'a> {
    w: &'a mut W,
}
impl<'a> _WORDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WORDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Half-Word transfer"]
    #[inline]
    pub fn half(self) -> &'a mut W {
        self.variant(WORDW::HALF)
    }
    #[doc = "Word Transfer"]
    #[inline]
    pub fn word(self) -> &'a mut W {
        self.variant(WORDW::WORD)
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
#[doc = r" Proxy"]
pub struct _SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPW<'a> {
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
pub struct _FASTWKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTWKUPW<'a> {
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
pub struct _REFRESHW<'a> {
    w: &'a mut W,
}
impl<'a> _REFRESHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USER_SEL`"]
pub enum USER_SELW {
    #[doc = "Channel 0"]
    CHANNEL0,
    #[doc = "Channel 1"]
    CHANNEL1,
}
impl USER_SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USER_SELW::CHANNEL0 => 0,
            USER_SELW::CHANNEL1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USER_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _USER_SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USER_SELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Channel 0"]
    #[inline]
    pub fn channel0(self) -> &'a mut W {
        self.variant(USER_SELW::CHANNEL0)
    }
    #[doc = "Channel 1"]
    #[inline]
    pub fn channel1(self) -> &'a mut W {
        self.variant(USER_SELW::CHANNEL1)
    }
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
#[doc = "Values that can be written to the field `TAG`"]
pub enum TAGW {
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    DIS,
    #[doc = "Tag selection mode enabled"]
    EN,
}
impl TAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TAGW::DIS => false,
            TAGW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TAGW<'a> {
    w: &'a mut W,
}
impl<'a> _TAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAGW::DIS)
    }
    #[doc = "Tag selection mode enabled"]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TAGW::EN)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAXS`"]
pub enum MAXSW {
    #[doc = "Normal Mode"]
    NORMAL,
    #[doc = "Max Speed Mode enabled"]
    MAXIMUM,
}
impl MAXSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAXSW::NORMAL => false,
            MAXSW::MAXIMUM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAXSW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAXSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(MAXSW::NORMAL)
    }
    #[doc = "Max Speed Mode enabled"]
    #[inline]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXSW::MAXIMUM)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STARTUP`"]
pub enum STARTUPW {
    #[doc = "0 periods of DACClock"]
    _0,
    #[doc = "8 periods of DACClock"]
    _8,
    #[doc = "16 periods of DACClock"]
    _16,
    #[doc = "24 periods of DACClock"]
    _24,
    #[doc = "64 periods of DACClock"]
    _64,
    #[doc = "80 periods of DACClock"]
    _80,
    #[doc = "96 periods of DACClock"]
    _96,
    #[doc = "112 periods of DACClock"]
    _112,
    #[doc = "512 periods of DACClock"]
    _512,
    #[doc = "576 periods of DACClock"]
    _576,
    #[doc = "640 periods of DACClock"]
    _640,
    #[doc = "704 periods of DACClock"]
    _704,
    #[doc = "768 periods of DACClock"]
    _768,
    #[doc = "832 periods of DACClock"]
    _832,
    #[doc = "896 periods of DACClock"]
    _896,
    #[doc = "960 periods of DACClock"]
    _960,
    #[doc = "1024 periods of DACClock"]
    _1024,
    #[doc = "1088 periods of DACClock"]
    _1088,
    #[doc = "1152 periods of DACClock"]
    _1152,
    #[doc = "1216 periods of DACClock"]
    _1216,
    #[doc = "1280 periods of DACClock"]
    _1280,
    #[doc = "1344 periods of DACClock"]
    _1344,
    #[doc = "1408 periods of DACClock"]
    _1408,
    #[doc = "1472 periods of DACClock"]
    _1472,
    #[doc = "1536 periods of DACClock"]
    _1536,
    #[doc = "1600 periods of DACClock"]
    _1600,
    #[doc = "1664 periods of DACClock"]
    _1664,
    #[doc = "1728 periods of DACClock"]
    _1728,
    #[doc = "1792 periods of DACClock"]
    _1792,
    #[doc = "1856 periods of DACClock"]
    _1856,
    #[doc = "1920 periods of DACClock"]
    _1920,
    #[doc = "1984 periods of DACClock"]
    _1984,
}
impl STARTUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTUPW::_0 => 0,
            STARTUPW::_8 => 1,
            STARTUPW::_16 => 2,
            STARTUPW::_24 => 3,
            STARTUPW::_64 => 4,
            STARTUPW::_80 => 5,
            STARTUPW::_96 => 6,
            STARTUPW::_112 => 7,
            STARTUPW::_512 => 8,
            STARTUPW::_576 => 9,
            STARTUPW::_640 => 10,
            STARTUPW::_704 => 11,
            STARTUPW::_768 => 12,
            STARTUPW::_832 => 13,
            STARTUPW::_896 => 14,
            STARTUPW::_960 => 15,
            STARTUPW::_1024 => 16,
            STARTUPW::_1088 => 17,
            STARTUPW::_1152 => 18,
            STARTUPW::_1216 => 19,
            STARTUPW::_1280 => 20,
            STARTUPW::_1344 => 21,
            STARTUPW::_1408 => 22,
            STARTUPW::_1472 => 23,
            STARTUPW::_1536 => 24,
            STARTUPW::_1600 => 25,
            STARTUPW::_1664 => 26,
            STARTUPW::_1728 => 27,
            STARTUPW::_1792 => 28,
            STARTUPW::_1856 => 29,
            STARTUPW::_1920 => 30,
            STARTUPW::_1984 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTUPW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTUPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0 periods of DACClock"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTUPW::_0)
    }
    #[doc = "8 periods of DACClock"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(STARTUPW::_8)
    }
    #[doc = "16 periods of DACClock"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(STARTUPW::_16)
    }
    #[doc = "24 periods of DACClock"]
    #[inline]
    pub fn _24(self) -> &'a mut W {
        self.variant(STARTUPW::_24)
    }
    #[doc = "64 periods of DACClock"]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(STARTUPW::_64)
    }
    #[doc = "80 periods of DACClock"]
    #[inline]
    pub fn _80(self) -> &'a mut W {
        self.variant(STARTUPW::_80)
    }
    #[doc = "96 periods of DACClock"]
    #[inline]
    pub fn _96(self) -> &'a mut W {
        self.variant(STARTUPW::_96)
    }
    #[doc = "112 periods of DACClock"]
    #[inline]
    pub fn _112(self) -> &'a mut W {
        self.variant(STARTUPW::_112)
    }
    #[doc = "512 periods of DACClock"]
    #[inline]
    pub fn _512(self) -> &'a mut W {
        self.variant(STARTUPW::_512)
    }
    #[doc = "576 periods of DACClock"]
    #[inline]
    pub fn _576(self) -> &'a mut W {
        self.variant(STARTUPW::_576)
    }
    #[doc = "640 periods of DACClock"]
    #[inline]
    pub fn _640(self) -> &'a mut W {
        self.variant(STARTUPW::_640)
    }
    #[doc = "704 periods of DACClock"]
    #[inline]
    pub fn _704(self) -> &'a mut W {
        self.variant(STARTUPW::_704)
    }
    #[doc = "768 periods of DACClock"]
    #[inline]
    pub fn _768(self) -> &'a mut W {
        self.variant(STARTUPW::_768)
    }
    #[doc = "832 periods of DACClock"]
    #[inline]
    pub fn _832(self) -> &'a mut W {
        self.variant(STARTUPW::_832)
    }
    #[doc = "896 periods of DACClock"]
    #[inline]
    pub fn _896(self) -> &'a mut W {
        self.variant(STARTUPW::_896)
    }
    #[doc = "960 periods of DACClock"]
    #[inline]
    pub fn _960(self) -> &'a mut W {
        self.variant(STARTUPW::_960)
    }
    #[doc = "1024 periods of DACClock"]
    #[inline]
    pub fn _1024(self) -> &'a mut W {
        self.variant(STARTUPW::_1024)
    }
    #[doc = "1088 periods of DACClock"]
    #[inline]
    pub fn _1088(self) -> &'a mut W {
        self.variant(STARTUPW::_1088)
    }
    #[doc = "1152 periods of DACClock"]
    #[inline]
    pub fn _1152(self) -> &'a mut W {
        self.variant(STARTUPW::_1152)
    }
    #[doc = "1216 periods of DACClock"]
    #[inline]
    pub fn _1216(self) -> &'a mut W {
        self.variant(STARTUPW::_1216)
    }
    #[doc = "1280 periods of DACClock"]
    #[inline]
    pub fn _1280(self) -> &'a mut W {
        self.variant(STARTUPW::_1280)
    }
    #[doc = "1344 periods of DACClock"]
    #[inline]
    pub fn _1344(self) -> &'a mut W {
        self.variant(STARTUPW::_1344)
    }
    #[doc = "1408 periods of DACClock"]
    #[inline]
    pub fn _1408(self) -> &'a mut W {
        self.variant(STARTUPW::_1408)
    }
    #[doc = "1472 periods of DACClock"]
    #[inline]
    pub fn _1472(self) -> &'a mut W {
        self.variant(STARTUPW::_1472)
    }
    #[doc = "1536 periods of DACClock"]
    #[inline]
    pub fn _1536(self) -> &'a mut W {
        self.variant(STARTUPW::_1536)
    }
    #[doc = "1600 periods of DACClock"]
    #[inline]
    pub fn _1600(self) -> &'a mut W {
        self.variant(STARTUPW::_1600)
    }
    #[doc = "1664 periods of DACClock"]
    #[inline]
    pub fn _1664(self) -> &'a mut W {
        self.variant(STARTUPW::_1664)
    }
    #[doc = "1728 periods of DACClock"]
    #[inline]
    pub fn _1728(self) -> &'a mut W {
        self.variant(STARTUPW::_1728)
    }
    #[doc = "1792 periods of DACClock"]
    #[inline]
    pub fn _1792(self) -> &'a mut W {
        self.variant(STARTUPW::_1792)
    }
    #[doc = "1856 periods of DACClock"]
    #[inline]
    pub fn _1856(self) -> &'a mut W {
        self.variant(STARTUPW::_1856)
    }
    #[doc = "1920 periods of DACClock"]
    #[inline]
    pub fn _1920(self) -> &'a mut W {
        self.variant(STARTUPW::_1920)
    }
    #[doc = "1984 periods of DACClock"]
    #[inline]
    pub fn _1984(self) -> &'a mut W {
        self.variant(STARTUPW::_1984)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline]
    pub fn trgen(&self) -> TRGENR {
        TRGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline]
    pub fn trgsel(&self) -> TRGSELR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGSELR { bits }
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline]
    pub fn word(&self) -> WORDR {
        WORDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline]
    pub fn sleep(&self) -> SLEEPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SLEEPR { bits }
    }
    #[doc = "Bit 6 - Fast Wake up Mode"]
    #[inline]
    pub fn fastwkup(&self) -> FASTWKUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FASTWKUPR { bits }
    }
    #[doc = "Bits 8:15 - Refresh Period"]
    #[inline]
    pub fn refresh(&self) -> REFRESHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        REFRESHR { bits }
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline]
    pub fn user_sel(&self) -> USER_SELR {
        USER_SELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline]
    pub fn tag(&self) -> TAGR {
        TAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Max Speed Mode"]
    #[inline]
    pub fn maxs(&self) -> MAXSR {
        MAXSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline]
    pub fn startup(&self) -> STARTUPR {
        STARTUPR::_from({
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline]
    pub fn trgen(&mut self) -> _TRGENW {
        _TRGENW { w: self }
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline]
    pub fn word(&mut self) -> _WORDW {
        _WORDW { w: self }
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline]
    pub fn sleep(&mut self) -> _SLEEPW {
        _SLEEPW { w: self }
    }
    #[doc = "Bit 6 - Fast Wake up Mode"]
    #[inline]
    pub fn fastwkup(&mut self) -> _FASTWKUPW {
        _FASTWKUPW { w: self }
    }
    #[doc = "Bits 8:15 - Refresh Period"]
    #[inline]
    pub fn refresh(&mut self) -> _REFRESHW {
        _REFRESHW { w: self }
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline]
    pub fn user_sel(&mut self) -> _USER_SELW {
        _USER_SELW { w: self }
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline]
    pub fn tag(&mut self) -> _TAGW {
        _TAGW { w: self }
    }
    #[doc = "Bit 21 - Max Speed Mode"]
    #[inline]
    pub fn maxs(&mut self) -> _MAXSW {
        _MAXSW { w: self }
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline]
    pub fn startup(&mut self) -> _STARTUPW {
        _STARTUPW { w: self }
    }
}
