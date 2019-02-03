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
#[doc = "Possible values of the field `BODRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENR {
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a brownout detection occurs."]
    NOT_ENABLE,
    #[doc = "the core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    ENABLE,
}
impl BODRSTENR {
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
            BODRSTENR::NOT_ENABLE => false,
            BODRSTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODRSTENR {
        match value {
            false => BODRSTENR::NOT_ENABLE,
            true => BODRSTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == BODRSTENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BODRSTENR::ENABLE
    }
}
#[doc = "Possible values of the field `BODDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODDISR {
    #[doc = "the core brownout detector is enabled."]
    ENABLE,
    #[doc = "the core brownout detector is disabled."]
    DISABLE,
}
impl BODDISR {
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
            BODDISR::ENABLE => false,
            BODDISR::DISABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODDISR {
        match value {
            false => BODDISR::ENABLE,
            true => BODDISR::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BODDISR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BODDISR::DISABLE
    }
}
#[doc = "Possible values of the field `VDDIORDY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VDDIORDYR {
    #[doc = "VDDIO is removed (used before going to backup mode when backup batteries are used)"]
    VDDIO_REMOVED,
    #[doc = "VDDIO is present (used before going to backup mode when backup batteries are used)"]
    VDDIO_PRESENT,
}
impl VDDIORDYR {
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
            VDDIORDYR::VDDIO_REMOVED => false,
            VDDIORDYR::VDDIO_PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VDDIORDYR {
        match value {
            false => VDDIORDYR::VDDIO_REMOVED,
            true => VDDIORDYR::VDDIO_PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `VDDIO_REMOVED`"]
    #[inline]
    pub fn is_vddio_removed(&self) -> bool {
        *self == VDDIORDYR::VDDIO_REMOVED
    }
    #[doc = "Checks if the value of the field is `VDDIO_PRESENT`"]
    #[inline]
    pub fn is_vddio_present(&self) -> bool {
        *self == VDDIORDYR::VDDIO_PRESENT
    }
}
#[doc = "Possible values of the field `OSCBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCBYPASSR {
    #[doc = "no effect. Clock selection depends on XTALSEL value."]
    NO_EFFECT,
    #[doc = "the 32-KHz XTAL oscillator is selected and is put in bypass mode."]
    BYPASS,
}
impl OSCBYPASSR {
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
            OSCBYPASSR::NO_EFFECT => false,
            OSCBYPASSR::BYPASS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCBYPASSR {
        match value {
            false => OSCBYPASSR::NO_EFFECT,
            true => OSCBYPASSR::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline]
    pub fn is_no_effect(&self) -> bool {
        *self == OSCBYPASSR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == OSCBYPASSR::BYPASS
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYR {
    #[doc = "Writing any other value in this field aborts the write operation."]
    PASSWD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEYR::PASSWD => 165,
            KEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEYR {
        match value {
            165 => KEYR::PASSWD,
            i => KEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline]
    pub fn is_passwd(&self) -> bool {
        *self == KEYR::PASSWD
    }
}
#[doc = "Values that can be written to the field `BODRSTEN`"]
pub enum BODRSTENW {
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a brownout detection occurs."]
    NOT_ENABLE,
    #[doc = "the core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    ENABLE,
}
impl BODRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODRSTENW::NOT_ENABLE => false,
            BODRSTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a brownout detection occurs."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(BODRSTENW::NOT_ENABLE)
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODRSTENW::ENABLE)
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
#[doc = "Values that can be written to the field `BODDIS`"]
pub enum BODDISW {
    #[doc = "the core brownout detector is enabled."]
    ENABLE,
    #[doc = "the core brownout detector is disabled."]
    DISABLE,
}
impl BODDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODDISW::ENABLE => false,
            BODDISW::DISABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODDISW<'a> {
    w: &'a mut W,
}
impl<'a> _BODDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the core brownout detector is enabled."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODDISW::ENABLE)
    }
    #[doc = "the core brownout detector is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODDISW::DISABLE)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VDDIORDY`"]
pub enum VDDIORDYW {
    #[doc = "VDDIO is removed (used before going to backup mode when backup batteries are used)"]
    VDDIO_REMOVED,
    #[doc = "VDDIO is present (used before going to backup mode when backup batteries are used)"]
    VDDIO_PRESENT,
}
impl VDDIORDYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VDDIORDYW::VDDIO_REMOVED => false,
            VDDIORDYW::VDDIO_PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VDDIORDYW<'a> {
    w: &'a mut W,
}
impl<'a> _VDDIORDYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VDDIORDYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VDDIO is removed (used before going to backup mode when backup batteries are used)"]
    #[inline]
    pub fn vddio_removed(self) -> &'a mut W {
        self.variant(VDDIORDYW::VDDIO_REMOVED)
    }
    #[doc = "VDDIO is present (used before going to backup mode when backup batteries are used)"]
    #[inline]
    pub fn vddio_present(self) -> &'a mut W {
        self.variant(VDDIORDYW::VDDIO_PRESENT)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OSCBYPASS`"]
pub enum OSCBYPASSW {
    #[doc = "no effect. Clock selection depends on XTALSEL value."]
    NO_EFFECT,
    #[doc = "the 32-KHz XTAL oscillator is selected and is put in bypass mode."]
    BYPASS,
}
impl OSCBYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCBYPASSW::NO_EFFECT => false,
            OSCBYPASSW::BYPASS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSCBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCBYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSCBYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "no effect. Clock selection depends on XTALSEL value."]
    #[inline]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OSCBYPASSW::NO_EFFECT)
    }
    #[doc = "the 32-KHz XTAL oscillator is selected and is put in bypass mode."]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(OSCBYPASSW::BYPASS)
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
#[doc = "Values that can be written to the field `KEY`"]
pub enum KEYW {
    #[doc = "Writing any other value in this field aborts the write operation."]
    PASSWD,
}
impl KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEYW::PASSWD => 165,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEYW::PASSWD)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline]
    pub fn bodrsten(&self) -> BODRSTENR {
        BODRSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline]
    pub fn boddis(&self) -> BODDISR {
        BODDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - VDDIO Ready"]
    #[inline]
    pub fn vddiordy(&self) -> VDDIORDYR {
        VDDIORDYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline]
    pub fn oscbypass(&self) -> OSCBYPASSR {
        OSCBYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline]
    pub fn key(&self) -> KEYR {
        KEYR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 23040 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline]
    pub fn bodrsten(&mut self) -> _BODRSTENW {
        _BODRSTENW { w: self }
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline]
    pub fn boddis(&mut self) -> _BODDISW {
        _BODDISW { w: self }
    }
    #[doc = "Bit 14 - VDDIO Ready"]
    #[inline]
    pub fn vddiordy(&mut self) -> _VDDIORDYW {
        _VDDIORDYW { w: self }
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline]
    pub fn oscbypass(&mut self) -> _OSCBYPASSW {
        _OSCBYPASSW { w: self }
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}
