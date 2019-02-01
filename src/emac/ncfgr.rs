#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NCFGR {
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
pub struct SPDR {
    bits: bool,
}
impl SPDR {
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
pub struct FDR {
    bits: bool,
}
impl FDR {
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
pub struct JFRAMER {
    bits: bool,
}
impl JFRAMER {
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
pub struct CAFR {
    bits: bool,
}
impl CAFR {
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
pub struct NBCR {
    bits: bool,
}
impl NBCR {
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
pub struct MTIR {
    bits: bool,
}
impl MTIR {
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
pub struct UNIR {
    bits: bool,
}
impl UNIR {
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
pub struct BIGR {
    bits: bool,
}
impl BIGR {
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
#[doc = "Possible values of the field `CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKR {
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)."]
    MCK_8,
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)."]
    MCK_16,
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)."]
    MCK_32,
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)."]
    MCK_64,
}
impl CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKR::MCK_8 => 0,
            CLKR::MCK_16 => 1,
            CLKR::MCK_32 => 2,
            CLKR::MCK_64 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKR {
        match value {
            0 => CLKR::MCK_8,
            1 => CLKR::MCK_16,
            2 => CLKR::MCK_32,
            3 => CLKR::MCK_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MCK_8`"]
    #[inline]
    pub fn is_mck_8(&self) -> bool {
        *self == CLKR::MCK_8
    }
    #[doc = "Checks if the value of the field is `MCK_16`"]
    #[inline]
    pub fn is_mck_16(&self) -> bool {
        *self == CLKR::MCK_16
    }
    #[doc = "Checks if the value of the field is `MCK_32`"]
    #[inline]
    pub fn is_mck_32(&self) -> bool {
        *self == CLKR::MCK_32
    }
    #[doc = "Checks if the value of the field is `MCK_64`"]
    #[inline]
    pub fn is_mck_64(&self) -> bool {
        *self == CLKR::MCK_64
    }
}
#[doc = r" Value of the field"]
pub struct RTYR {
    bits: bool,
}
impl RTYR {
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
pub struct PAER {
    bits: bool,
}
impl PAER {
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
#[doc = "Possible values of the field `RBOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBOFR {
    #[doc = "No offset from start of receive buffer."]
    OFFSET_0,
    #[doc = "One-byte offset from start of receive buffer."]
    OFFSET_1,
    #[doc = "Two-byte offset from start of receive buffer."]
    OFFSET_2,
    #[doc = "Three-byte offset from start of receive buffer."]
    OFFSET_3,
}
impl RBOFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RBOFR::OFFSET_0 => 0,
            RBOFR::OFFSET_1 => 1,
            RBOFR::OFFSET_2 => 2,
            RBOFR::OFFSET_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RBOFR {
        match value {
            0 => RBOFR::OFFSET_0,
            1 => RBOFR::OFFSET_1,
            2 => RBOFR::OFFSET_2,
            3 => RBOFR::OFFSET_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET_0`"]
    #[inline]
    pub fn is_offset_0(&self) -> bool {
        *self == RBOFR::OFFSET_0
    }
    #[doc = "Checks if the value of the field is `OFFSET_1`"]
    #[inline]
    pub fn is_offset_1(&self) -> bool {
        *self == RBOFR::OFFSET_1
    }
    #[doc = "Checks if the value of the field is `OFFSET_2`"]
    #[inline]
    pub fn is_offset_2(&self) -> bool {
        *self == RBOFR::OFFSET_2
    }
    #[doc = "Checks if the value of the field is `OFFSET_3`"]
    #[inline]
    pub fn is_offset_3(&self) -> bool {
        *self == RBOFR::OFFSET_3
    }
}
#[doc = r" Value of the field"]
pub struct RLCER {
    bits: bool,
}
impl RLCER {
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
pub struct DRFCSR {
    bits: bool,
}
impl DRFCSR {
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
pub struct EFRHDR {
    bits: bool,
}
impl EFRHDR {
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
pub struct IRXFCSR {
    bits: bool,
}
impl IRXFCSR {
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
#[doc = r" Proxy"]
pub struct _SPDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDW<'a> {
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
pub struct _FDW<'a> {
    w: &'a mut W,
}
impl<'a> _FDW<'a> {
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
#[doc = r" Proxy"]
pub struct _JFRAMEW<'a> {
    w: &'a mut W,
}
impl<'a> _JFRAMEW<'a> {
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
pub struct _CAFW<'a> {
    w: &'a mut W,
}
impl<'a> _CAFW<'a> {
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
pub struct _NBCW<'a> {
    w: &'a mut W,
}
impl<'a> _NBCW<'a> {
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
pub struct _MTIW<'a> {
    w: &'a mut W,
}
impl<'a> _MTIW<'a> {
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
pub struct _UNIW<'a> {
    w: &'a mut W,
}
impl<'a> _UNIW<'a> {
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
pub struct _BIGW<'a> {
    w: &'a mut W,
}
impl<'a> _BIGW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLK`"]
pub enum CLKW {
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)."]
    MCK_8,
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)."]
    MCK_16,
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)."]
    MCK_32,
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)."]
    MCK_64,
}
impl CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKW::MCK_8 => 0,
            CLKW::MCK_16 => 1,
            CLKW::MCK_32 => 2,
            CLKW::MCK_64 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)."]
    #[inline]
    pub fn mck_8(self) -> &'a mut W {
        self.variant(CLKW::MCK_8)
    }
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)."]
    #[inline]
    pub fn mck_16(self) -> &'a mut W {
        self.variant(CLKW::MCK_16)
    }
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)."]
    #[inline]
    pub fn mck_32(self) -> &'a mut W {
        self.variant(CLKW::MCK_32)
    }
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)."]
    #[inline]
    pub fn mck_64(self) -> &'a mut W {
        self.variant(CLKW::MCK_64)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTYW<'a> {
    w: &'a mut W,
}
impl<'a> _RTYW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAEW<'a> {
    w: &'a mut W,
}
impl<'a> _PAEW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RBOF`"]
pub enum RBOFW {
    #[doc = "No offset from start of receive buffer."]
    OFFSET_0,
    #[doc = "One-byte offset from start of receive buffer."]
    OFFSET_1,
    #[doc = "Two-byte offset from start of receive buffer."]
    OFFSET_2,
    #[doc = "Three-byte offset from start of receive buffer."]
    OFFSET_3,
}
impl RBOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RBOFW::OFFSET_0 => 0,
            RBOFW::OFFSET_1 => 1,
            RBOFW::OFFSET_2 => 2,
            RBOFW::OFFSET_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBOFW<'a> {
    w: &'a mut W,
}
impl<'a> _RBOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBOFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No offset from start of receive buffer."]
    #[inline]
    pub fn offset_0(self) -> &'a mut W {
        self.variant(RBOFW::OFFSET_0)
    }
    #[doc = "One-byte offset from start of receive buffer."]
    #[inline]
    pub fn offset_1(self) -> &'a mut W {
        self.variant(RBOFW::OFFSET_1)
    }
    #[doc = "Two-byte offset from start of receive buffer."]
    #[inline]
    pub fn offset_2(self) -> &'a mut W {
        self.variant(RBOFW::OFFSET_2)
    }
    #[doc = "Three-byte offset from start of receive buffer."]
    #[inline]
    pub fn offset_3(self) -> &'a mut W {
        self.variant(RBOFW::OFFSET_3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RLCEW<'a> {
    w: &'a mut W,
}
impl<'a> _RLCEW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DRFCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DRFCSW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EFRHDW<'a> {
    w: &'a mut W,
}
impl<'a> _EFRHDW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IRXFCSW<'a> {
    w: &'a mut W,
}
impl<'a> _IRXFCSW<'a> {
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
        const OFFSET: u8 = 19;
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
    #[doc = "Bit 0 - Speed"]
    #[inline]
    pub fn spd(&self) -> SPDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPDR { bits }
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline]
    pub fn fd(&self) -> FDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FDR { bits }
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline]
    pub fn jframe(&self) -> JFRAMER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        JFRAMER { bits }
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline]
    pub fn caf(&self) -> CAFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CAFR { bits }
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline]
    pub fn nbc(&self) -> NBCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NBCR { bits }
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline]
    pub fn mti(&self) -> MTIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MTIR { bits }
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline]
    pub fn uni(&self) -> UNIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNIR { bits }
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline]
    pub fn big(&self) -> BIGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BIGR { bits }
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline]
    pub fn clk(&self) -> CLKR {
        CLKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline]
    pub fn rty(&self) -> RTYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTYR { bits }
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline]
    pub fn pae(&self) -> PAER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAER { bits }
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline]
    pub fn rbof(&self) -> RBOFR {
        RBOFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline]
    pub fn rlce(&self) -> RLCER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RLCER { bits }
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline]
    pub fn drfcs(&self) -> DRFCSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DRFCSR { bits }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn efrhd(&self) -> EFRHDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EFRHDR { bits }
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline]
    pub fn irxfcs(&self) -> IRXFCSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IRXFCSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2048 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Speed"]
    #[inline]
    pub fn spd(&mut self) -> _SPDW {
        _SPDW { w: self }
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline]
    pub fn fd(&mut self) -> _FDW {
        _FDW { w: self }
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline]
    pub fn jframe(&mut self) -> _JFRAMEW {
        _JFRAMEW { w: self }
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline]
    pub fn caf(&mut self) -> _CAFW {
        _CAFW { w: self }
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline]
    pub fn nbc(&mut self) -> _NBCW {
        _NBCW { w: self }
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline]
    pub fn mti(&mut self) -> _MTIW {
        _MTIW { w: self }
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline]
    pub fn uni(&mut self) -> _UNIW {
        _UNIW { w: self }
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline]
    pub fn big(&mut self) -> _BIGW {
        _BIGW { w: self }
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline]
    pub fn clk(&mut self) -> _CLKW {
        _CLKW { w: self }
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline]
    pub fn rty(&mut self) -> _RTYW {
        _RTYW { w: self }
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline]
    pub fn pae(&mut self) -> _PAEW {
        _PAEW { w: self }
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline]
    pub fn rbof(&mut self) -> _RBOFW {
        _RBOFW { w: self }
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline]
    pub fn rlce(&mut self) -> _RLCEW {
        _RLCEW { w: self }
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline]
    pub fn drfcs(&mut self) -> _DRFCSW {
        _DRFCSW { w: self }
    }
    #[doc = "Bit 18"]
    #[inline]
    pub fn efrhd(&mut self) -> _EFRHDW {
        _EFRHDW { w: self }
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline]
    pub fn irxfcs(&mut self) -> _IRXFCSW {
        _IRXFCSW { w: self }
    }
}
