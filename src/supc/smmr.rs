#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMMR {
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
#[doc = "Possible values of the field `SMTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMTHR {
    #[doc = "1.9 V"]
    _1_9V,
    #[doc = "2.0 V"]
    _2_0V,
    #[doc = "2.1 V"]
    _2_1V,
    #[doc = "2.2 V"]
    _2_2V,
    #[doc = "2.3 V"]
    _2_3V,
    #[doc = "2.4 V"]
    _2_4V,
    #[doc = "2.5 V"]
    _2_5V,
    #[doc = "2.6 V"]
    _2_6V,
    #[doc = "2.7 V"]
    _2_7V,
    #[doc = "2.8 V"]
    _2_8V,
    #[doc = "2.9 V"]
    _2_9V,
    #[doc = "3.0 V"]
    _3_0V,
    #[doc = "3.1 V"]
    _3_1V,
    #[doc = "3.2 V"]
    _3_2V,
    #[doc = "3.3 V"]
    _3_3V,
    #[doc = "3.4 V"]
    _3_4V,
}
impl SMTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMTHR::_1_9V => 0,
            SMTHR::_2_0V => 1,
            SMTHR::_2_1V => 2,
            SMTHR::_2_2V => 3,
            SMTHR::_2_3V => 4,
            SMTHR::_2_4V => 5,
            SMTHR::_2_5V => 6,
            SMTHR::_2_6V => 7,
            SMTHR::_2_7V => 8,
            SMTHR::_2_8V => 9,
            SMTHR::_2_9V => 10,
            SMTHR::_3_0V => 11,
            SMTHR::_3_1V => 12,
            SMTHR::_3_2V => 13,
            SMTHR::_3_3V => 14,
            SMTHR::_3_4V => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMTHR {
        match value {
            0 => SMTHR::_1_9V,
            1 => SMTHR::_2_0V,
            2 => SMTHR::_2_1V,
            3 => SMTHR::_2_2V,
            4 => SMTHR::_2_3V,
            5 => SMTHR::_2_4V,
            6 => SMTHR::_2_5V,
            7 => SMTHR::_2_6V,
            8 => SMTHR::_2_7V,
            9 => SMTHR::_2_8V,
            10 => SMTHR::_2_9V,
            11 => SMTHR::_3_0V,
            12 => SMTHR::_3_1V,
            13 => SMTHR::_3_2V,
            14 => SMTHR::_3_3V,
            15 => SMTHR::_3_4V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_9V`"]
    #[inline]
    pub fn is_1_9v(&self) -> bool {
        *self == SMTHR::_1_9V
    }
    #[doc = "Checks if the value of the field is `_2_0V`"]
    #[inline]
    pub fn is_2_0v(&self) -> bool {
        *self == SMTHR::_2_0V
    }
    #[doc = "Checks if the value of the field is `_2_1V`"]
    #[inline]
    pub fn is_2_1v(&self) -> bool {
        *self == SMTHR::_2_1V
    }
    #[doc = "Checks if the value of the field is `_2_2V`"]
    #[inline]
    pub fn is_2_2v(&self) -> bool {
        *self == SMTHR::_2_2V
    }
    #[doc = "Checks if the value of the field is `_2_3V`"]
    #[inline]
    pub fn is_2_3v(&self) -> bool {
        *self == SMTHR::_2_3V
    }
    #[doc = "Checks if the value of the field is `_2_4V`"]
    #[inline]
    pub fn is_2_4v(&self) -> bool {
        *self == SMTHR::_2_4V
    }
    #[doc = "Checks if the value of the field is `_2_5V`"]
    #[inline]
    pub fn is_2_5v(&self) -> bool {
        *self == SMTHR::_2_5V
    }
    #[doc = "Checks if the value of the field is `_2_6V`"]
    #[inline]
    pub fn is_2_6v(&self) -> bool {
        *self == SMTHR::_2_6V
    }
    #[doc = "Checks if the value of the field is `_2_7V`"]
    #[inline]
    pub fn is_2_7v(&self) -> bool {
        *self == SMTHR::_2_7V
    }
    #[doc = "Checks if the value of the field is `_2_8V`"]
    #[inline]
    pub fn is_2_8v(&self) -> bool {
        *self == SMTHR::_2_8V
    }
    #[doc = "Checks if the value of the field is `_2_9V`"]
    #[inline]
    pub fn is_2_9v(&self) -> bool {
        *self == SMTHR::_2_9V
    }
    #[doc = "Checks if the value of the field is `_3_0V`"]
    #[inline]
    pub fn is_3_0v(&self) -> bool {
        *self == SMTHR::_3_0V
    }
    #[doc = "Checks if the value of the field is `_3_1V`"]
    #[inline]
    pub fn is_3_1v(&self) -> bool {
        *self == SMTHR::_3_1V
    }
    #[doc = "Checks if the value of the field is `_3_2V`"]
    #[inline]
    pub fn is_3_2v(&self) -> bool {
        *self == SMTHR::_3_2V
    }
    #[doc = "Checks if the value of the field is `_3_3V`"]
    #[inline]
    pub fn is_3_3v(&self) -> bool {
        *self == SMTHR::_3_3V
    }
    #[doc = "Checks if the value of the field is `_3_4V`"]
    #[inline]
    pub fn is_3_4v(&self) -> bool {
        *self == SMTHR::_3_4V
    }
}
#[doc = "Possible values of the field `SMSMPL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSMPLR {
    #[doc = "Supply Monitor disabled"]
    SMD,
    #[doc = "Continuous Supply Monitor"]
    CSM,
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048SLCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMSMPLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMSMPLR::SMD => 0,
            SMSMPLR::CSM => 1,
            SMSMPLR::_32SLCK => 2,
            SMSMPLR::_256SLCK => 3,
            SMSMPLR::_2048SLCK => 4,
            SMSMPLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMSMPLR {
        match value {
            0 => SMSMPLR::SMD,
            1 => SMSMPLR::CSM,
            2 => SMSMPLR::_32SLCK,
            3 => SMSMPLR::_256SLCK,
            4 => SMSMPLR::_2048SLCK,
            i => SMSMPLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SMD`"]
    #[inline]
    pub fn is_smd(&self) -> bool {
        *self == SMSMPLR::SMD
    }
    #[doc = "Checks if the value of the field is `CSM`"]
    #[inline]
    pub fn is_csm(&self) -> bool {
        *self == SMSMPLR::CSM
    }
    #[doc = "Checks if the value of the field is `_32SLCK`"]
    #[inline]
    pub fn is_32slck(&self) -> bool {
        *self == SMSMPLR::_32SLCK
    }
    #[doc = "Checks if the value of the field is `_256SLCK`"]
    #[inline]
    pub fn is_256slck(&self) -> bool {
        *self == SMSMPLR::_256SLCK
    }
    #[doc = "Checks if the value of the field is `_2048SLCK`"]
    #[inline]
    pub fn is_2048slck(&self) -> bool {
        *self == SMSMPLR::_2048SLCK
    }
}
#[doc = "Possible values of the field `SMRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTENR {
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl SMRSTENR {
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
            SMRSTENR::NOT_ENABLE => false,
            SMRSTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMRSTENR {
        match value {
            false => SMRSTENR::NOT_ENABLE,
            true => SMRSTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == SMRSTENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SMRSTENR::ENABLE
    }
}
#[doc = "Possible values of the field `SMIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMIENR {
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl SMIENR {
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
            SMIENR::NOT_ENABLE => false,
            SMIENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMIENR {
        match value {
            false => SMIENR::NOT_ENABLE,
            true => SMIENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == SMIENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SMIENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `SMTH`"]
pub enum SMTHW {
    #[doc = "1.9 V"]
    _1_9V,
    #[doc = "2.0 V"]
    _2_0V,
    #[doc = "2.1 V"]
    _2_1V,
    #[doc = "2.2 V"]
    _2_2V,
    #[doc = "2.3 V"]
    _2_3V,
    #[doc = "2.4 V"]
    _2_4V,
    #[doc = "2.5 V"]
    _2_5V,
    #[doc = "2.6 V"]
    _2_6V,
    #[doc = "2.7 V"]
    _2_7V,
    #[doc = "2.8 V"]
    _2_8V,
    #[doc = "2.9 V"]
    _2_9V,
    #[doc = "3.0 V"]
    _3_0V,
    #[doc = "3.1 V"]
    _3_1V,
    #[doc = "3.2 V"]
    _3_2V,
    #[doc = "3.3 V"]
    _3_3V,
    #[doc = "3.4 V"]
    _3_4V,
}
impl SMTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMTHW::_1_9V => 0,
            SMTHW::_2_0V => 1,
            SMTHW::_2_1V => 2,
            SMTHW::_2_2V => 3,
            SMTHW::_2_3V => 4,
            SMTHW::_2_4V => 5,
            SMTHW::_2_5V => 6,
            SMTHW::_2_6V => 7,
            SMTHW::_2_7V => 8,
            SMTHW::_2_8V => 9,
            SMTHW::_2_9V => 10,
            SMTHW::_3_0V => 11,
            SMTHW::_3_1V => 12,
            SMTHW::_3_2V => 13,
            SMTHW::_3_3V => 14,
            SMTHW::_3_4V => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMTHW<'a> {
    w: &'a mut W,
}
impl<'a> _SMTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMTHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1.9 V"]
    #[inline]
    pub fn _1_9v(self) -> &'a mut W {
        self.variant(SMTHW::_1_9V)
    }
    #[doc = "2.0 V"]
    #[inline]
    pub fn _2_0v(self) -> &'a mut W {
        self.variant(SMTHW::_2_0V)
    }
    #[doc = "2.1 V"]
    #[inline]
    pub fn _2_1v(self) -> &'a mut W {
        self.variant(SMTHW::_2_1V)
    }
    #[doc = "2.2 V"]
    #[inline]
    pub fn _2_2v(self) -> &'a mut W {
        self.variant(SMTHW::_2_2V)
    }
    #[doc = "2.3 V"]
    #[inline]
    pub fn _2_3v(self) -> &'a mut W {
        self.variant(SMTHW::_2_3V)
    }
    #[doc = "2.4 V"]
    #[inline]
    pub fn _2_4v(self) -> &'a mut W {
        self.variant(SMTHW::_2_4V)
    }
    #[doc = "2.5 V"]
    #[inline]
    pub fn _2_5v(self) -> &'a mut W {
        self.variant(SMTHW::_2_5V)
    }
    #[doc = "2.6 V"]
    #[inline]
    pub fn _2_6v(self) -> &'a mut W {
        self.variant(SMTHW::_2_6V)
    }
    #[doc = "2.7 V"]
    #[inline]
    pub fn _2_7v(self) -> &'a mut W {
        self.variant(SMTHW::_2_7V)
    }
    #[doc = "2.8 V"]
    #[inline]
    pub fn _2_8v(self) -> &'a mut W {
        self.variant(SMTHW::_2_8V)
    }
    #[doc = "2.9 V"]
    #[inline]
    pub fn _2_9v(self) -> &'a mut W {
        self.variant(SMTHW::_2_9V)
    }
    #[doc = "3.0 V"]
    #[inline]
    pub fn _3_0v(self) -> &'a mut W {
        self.variant(SMTHW::_3_0V)
    }
    #[doc = "3.1 V"]
    #[inline]
    pub fn _3_1v(self) -> &'a mut W {
        self.variant(SMTHW::_3_1V)
    }
    #[doc = "3.2 V"]
    #[inline]
    pub fn _3_2v(self) -> &'a mut W {
        self.variant(SMTHW::_3_2V)
    }
    #[doc = "3.3 V"]
    #[inline]
    pub fn _3_3v(self) -> &'a mut W {
        self.variant(SMTHW::_3_3V)
    }
    #[doc = "3.4 V"]
    #[inline]
    pub fn _3_4v(self) -> &'a mut W {
        self.variant(SMTHW::_3_4V)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMSMPL`"]
pub enum SMSMPLW {
    #[doc = "Supply Monitor disabled"]
    SMD,
    #[doc = "Continuous Supply Monitor"]
    CSM,
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    _32SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    _256SLCK,
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    _2048SLCK,
}
impl SMSMPLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMSMPLW::SMD => 0,
            SMSMPLW::CSM => 1,
            SMSMPLW::_32SLCK => 2,
            SMSMPLW::_256SLCK => 3,
            SMSMPLW::_2048SLCK => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMSMPLW<'a> {
    w: &'a mut W,
}
impl<'a> _SMSMPLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMSMPLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Supply Monitor disabled"]
    #[inline]
    pub fn smd(self) -> &'a mut W {
        self.variant(SMSMPLW::SMD)
    }
    #[doc = "Continuous Supply Monitor"]
    #[inline]
    pub fn csm(self) -> &'a mut W {
        self.variant(SMSMPLW::CSM)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 32 SLCK periods"]
    #[inline]
    pub fn _32slck(self) -> &'a mut W {
        self.variant(SMSMPLW::_32SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 256 SLCK periods"]
    #[inline]
    pub fn _256slck(self) -> &'a mut W {
        self.variant(SMSMPLW::_256SLCK)
    }
    #[doc = "Supply Monitor enabled one SLCK period every 2,048 SLCK periods"]
    #[inline]
    pub fn _2048slck(self) -> &'a mut W {
        self.variant(SMSMPLW::_2048SLCK)
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
#[doc = "Values that can be written to the field `SMRSTEN`"]
pub enum SMRSTENW {
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl SMRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMRSTENW::NOT_ENABLE => false,
            SMRSTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMRSTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the core reset signal \"vddcore_nreset\" is not affected when a supply monitor detection occurs."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMRSTENW::NOT_ENABLE)
    }
    #[doc = "the core reset signal, vddcore_nreset is asserted when a supply monitor detection occurs."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMRSTENW::ENABLE)
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
#[doc = "Values that can be written to the field `SMIEN`"]
pub enum SMIENW {
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    NOT_ENABLE,
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    ENABLE,
}
impl SMIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMIENW::NOT_ENABLE => false,
            SMIENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the SUPC interrupt signal is not affected when a supply monitor detection occurs."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMIENW::NOT_ENABLE)
    }
    #[doc = "the SUPC interrupt signal is asserted when a supply monitor detection occurs."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMIENW::ENABLE)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline]
    pub fn smth(&self) -> SMTHR {
        SMTHR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline]
    pub fn smsmpl(&self) -> SMSMPLR {
        SMSMPLR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline]
    pub fn smrsten(&self) -> SMRSTENR {
        SMRSTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline]
    pub fn smien(&self) -> SMIENR {
        SMIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:3 - Supply Monitor Threshold"]
    #[inline]
    pub fn smth(&mut self) -> _SMTHW {
        _SMTHW { w: self }
    }
    #[doc = "Bits 8:10 - Supply Monitor Sampling Period"]
    #[inline]
    pub fn smsmpl(&mut self) -> _SMSMPLW {
        _SMSMPLW { w: self }
    }
    #[doc = "Bit 12 - Supply Monitor Reset Enable"]
    #[inline]
    pub fn smrsten(&mut self) -> _SMRSTENW {
        _SMRSTENW { w: self }
    }
    #[doc = "Bit 13 - Supply Monitor Interrupt Enable"]
    #[inline]
    pub fn smien(&mut self) -> _SMIENW {
        _SMIENW { w: self }
    }
}
