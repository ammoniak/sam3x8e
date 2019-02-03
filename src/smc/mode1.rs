#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE1 {
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
#[doc = "Possible values of the field `READ_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_MODER {
    #[doc = "The Read operation is controlled by the NCS signal."]
    NCS_CTRL,
    #[doc = "The Read operation is controlled by the NRD signal."]
    NRD_CTRL,
}
impl READ_MODER {
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
            READ_MODER::NCS_CTRL => false,
            READ_MODER::NRD_CTRL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READ_MODER {
        match value {
            false => READ_MODER::NCS_CTRL,
            true => READ_MODER::NRD_CTRL,
        }
    }
    #[doc = "Checks if the value of the field is `NCS_CTRL`"]
    #[inline]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == READ_MODER::NCS_CTRL
    }
    #[doc = "Checks if the value of the field is `NRD_CTRL`"]
    #[inline]
    pub fn is_nrd_ctrl(&self) -> bool {
        *self == READ_MODER::NRD_CTRL
    }
}
#[doc = "Possible values of the field `WRITE_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_MODER {
    #[doc = "The Write operation is controller by the NCS signal."]
    NCS_CTRL,
    #[doc = "The Write operation is controlled by the NWE signal."]
    NWE_CTRL,
}
impl WRITE_MODER {
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
            WRITE_MODER::NCS_CTRL => false,
            WRITE_MODER::NWE_CTRL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRITE_MODER {
        match value {
            false => WRITE_MODER::NCS_CTRL,
            true => WRITE_MODER::NWE_CTRL,
        }
    }
    #[doc = "Checks if the value of the field is `NCS_CTRL`"]
    #[inline]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == WRITE_MODER::NCS_CTRL
    }
    #[doc = "Checks if the value of the field is `NWE_CTRL`"]
    #[inline]
    pub fn is_nwe_ctrl(&self) -> bool {
        *self == WRITE_MODER::NWE_CTRL
    }
}
#[doc = "Possible values of the field `EXNW_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXNW_MODER {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Frozen Mode"]
    FROZEN,
    #[doc = "Ready Mode"]
    READY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXNW_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXNW_MODER::DISABLED => 0,
            EXNW_MODER::FROZEN => 2,
            EXNW_MODER::READY => 3,
            EXNW_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXNW_MODER {
        match value {
            0 => EXNW_MODER::DISABLED,
            2 => EXNW_MODER::FROZEN,
            3 => EXNW_MODER::READY,
            i => EXNW_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EXNW_MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline]
    pub fn is_frozen(&self) -> bool {
        *self == EXNW_MODER::FROZEN
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == EXNW_MODER::READY
    }
}
#[doc = r" Value of the field"]
pub struct BATR {
    bits: bool,
}
impl BATR {
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
#[doc = "Possible values of the field `DBW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBWR {
    #[doc = "8-bit bus"]
    BIT_8,
    #[doc = "16-bit bus"]
    BIT_16,
}
impl DBWR {
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
            DBWR::BIT_8 => false,
            DBWR::BIT_16 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBWR {
        match value {
            false => DBWR::BIT_8,
            true => DBWR::BIT_16,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_8`"]
    #[inline]
    pub fn is_bit_8(&self) -> bool {
        *self == DBWR::BIT_8
    }
    #[doc = "Checks if the value of the field is `BIT_16`"]
    #[inline]
    pub fn is_bit_16(&self) -> bool {
        *self == DBWR::BIT_16
    }
}
#[doc = r" Value of the field"]
pub struct TDF_CYCLESR {
    bits: u8,
}
impl TDF_CYCLESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TDF_MODER {
    bits: bool,
}
impl TDF_MODER {
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
#[doc = "Values that can be written to the field `READ_MODE`"]
pub enum READ_MODEW {
    #[doc = "The Read operation is controlled by the NCS signal."]
    NCS_CTRL,
    #[doc = "The Read operation is controlled by the NRD signal."]
    NRD_CTRL,
}
impl READ_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READ_MODEW::NCS_CTRL => false,
            READ_MODEW::NRD_CTRL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READ_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READ_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Read operation is controlled by the NCS signal."]
    #[inline]
    pub fn ncs_ctrl(self) -> &'a mut W {
        self.variant(READ_MODEW::NCS_CTRL)
    }
    #[doc = "The Read operation is controlled by the NRD signal."]
    #[inline]
    pub fn nrd_ctrl(self) -> &'a mut W {
        self.variant(READ_MODEW::NRD_CTRL)
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
#[doc = "Values that can be written to the field `WRITE_MODE`"]
pub enum WRITE_MODEW {
    #[doc = "The Write operation is controller by the NCS signal."]
    NCS_CTRL,
    #[doc = "The Write operation is controlled by the NWE signal."]
    NWE_CTRL,
}
impl WRITE_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRITE_MODEW::NCS_CTRL => false,
            WRITE_MODEW::NWE_CTRL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRITE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRITE_MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Write operation is controller by the NCS signal."]
    #[inline]
    pub fn ncs_ctrl(self) -> &'a mut W {
        self.variant(WRITE_MODEW::NCS_CTRL)
    }
    #[doc = "The Write operation is controlled by the NWE signal."]
    #[inline]
    pub fn nwe_ctrl(self) -> &'a mut W {
        self.variant(WRITE_MODEW::NWE_CTRL)
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
#[doc = "Values that can be written to the field `EXNW_MODE`"]
pub enum EXNW_MODEW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Frozen Mode"]
    FROZEN,
    #[doc = "Ready Mode"]
    READY,
}
impl EXNW_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXNW_MODEW::DISABLED => 0,
            EXNW_MODEW::FROZEN => 2,
            EXNW_MODEW::READY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXNW_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXNW_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXNW_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXNW_MODEW::DISABLED)
    }
    #[doc = "Frozen Mode"]
    #[inline]
    pub fn frozen(self) -> &'a mut W {
        self.variant(EXNW_MODEW::FROZEN)
    }
    #[doc = "Ready Mode"]
    #[inline]
    pub fn ready(self) -> &'a mut W {
        self.variant(EXNW_MODEW::READY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BATW<'a> {
    w: &'a mut W,
}
impl<'a> _BATW<'a> {
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
#[doc = "Values that can be written to the field `DBW`"]
pub enum DBWW {
    #[doc = "8-bit bus"]
    BIT_8,
    #[doc = "16-bit bus"]
    BIT_16,
}
impl DBWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBWW::BIT_8 => false,
            DBWW::BIT_16 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBWW<'a> {
    w: &'a mut W,
}
impl<'a> _DBWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit bus"]
    #[inline]
    pub fn bit_8(self) -> &'a mut W {
        self.variant(DBWW::BIT_8)
    }
    #[doc = "16-bit bus"]
    #[inline]
    pub fn bit_16(self) -> &'a mut W {
        self.variant(DBWW::BIT_16)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TDF_CYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _TDF_CYCLESW<'a> {
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
pub struct _TDF_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDF_MODEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline]
    pub fn read_mode(&self) -> READ_MODER {
        READ_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
    #[inline]
    pub fn write_mode(&self) -> WRITE_MODER {
        WRITE_MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline]
    pub fn exnw_mode(&self) -> EXNW_MODER {
        EXNW_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline]
    pub fn bat(&self) -> BATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BATR { bits }
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline]
    pub fn dbw(&self) -> DBWR {
        DBWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline]
    pub fn tdf_cycles(&self) -> TDF_CYCLESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TDF_CYCLESR { bits }
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline]
    pub fn tdf_mode(&self) -> TDF_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TDF_MODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 268435459 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline]
    pub fn read_mode(&mut self) -> _READ_MODEW {
        _READ_MODEW { w: self }
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
    #[inline]
    pub fn write_mode(&mut self) -> _WRITE_MODEW {
        _WRITE_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline]
    pub fn exnw_mode(&mut self) -> _EXNW_MODEW {
        _EXNW_MODEW { w: self }
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline]
    pub fn bat(&mut self) -> _BATW {
        _BATW { w: self }
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline]
    pub fn dbw(&mut self) -> _DBWW {
        _DBWW { w: self }
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline]
    pub fn tdf_cycles(&mut self) -> _TDF_CYCLESW {
        _TDF_CYCLESW { w: self }
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline]
    pub fn tdf_mode(&mut self) -> _TDF_MODEW {
        _TDF_MODEW { w: self }
    }
}
