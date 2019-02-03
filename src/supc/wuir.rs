#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WUIR {
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
#[doc = "Possible values of the field `WKUPEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN0R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN0R {
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
            WKUPEN0R::DISABLE => false,
            WKUPEN0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN0R {
        match value {
            false => WKUPEN0R::DISABLE,
            true => WKUPEN0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN0R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN1R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN1R {
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
            WKUPEN1R::DISABLE => false,
            WKUPEN1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN1R {
        match value {
            false => WKUPEN1R::DISABLE,
            true => WKUPEN1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN1R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN2R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN2R {
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
            WKUPEN2R::DISABLE => false,
            WKUPEN2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN2R {
        match value {
            false => WKUPEN2R::DISABLE,
            true => WKUPEN2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN2R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN3R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN3R {
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
            WKUPEN3R::DISABLE => false,
            WKUPEN3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN3R {
        match value {
            false => WKUPEN3R::DISABLE,
            true => WKUPEN3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN3R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN4R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN4R {
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
            WKUPEN4R::DISABLE => false,
            WKUPEN4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN4R {
        match value {
            false => WKUPEN4R::DISABLE,
            true => WKUPEN4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN4R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN5R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN5R {
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
            WKUPEN5R::DISABLE => false,
            WKUPEN5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN5R {
        match value {
            false => WKUPEN5R::DISABLE,
            true => WKUPEN5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN5R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN6R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN6R {
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
            WKUPEN6R::DISABLE => false,
            WKUPEN6R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN6R {
        match value {
            false => WKUPEN6R::DISABLE,
            true => WKUPEN6R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN6R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN7R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN7R {
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
            WKUPEN7R::DISABLE => false,
            WKUPEN7R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN7R {
        match value {
            false => WKUPEN7R::DISABLE,
            true => WKUPEN7R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN7R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN8R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN8R {
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
            WKUPEN8R::DISABLE => false,
            WKUPEN8R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN8R {
        match value {
            false => WKUPEN8R::DISABLE,
            true => WKUPEN8R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN8R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN8R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN9R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN9R {
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
            WKUPEN9R::DISABLE => false,
            WKUPEN9R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN9R {
        match value {
            false => WKUPEN9R::DISABLE,
            true => WKUPEN9R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN9R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN9R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN10R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN10R {
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
            WKUPEN10R::DISABLE => false,
            WKUPEN10R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN10R {
        match value {
            false => WKUPEN10R::DISABLE,
            true => WKUPEN10R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN10R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN10R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN11R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN11R {
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
            WKUPEN11R::DISABLE => false,
            WKUPEN11R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN11R {
        match value {
            false => WKUPEN11R::DISABLE,
            true => WKUPEN11R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN11R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN11R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN12R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN12R {
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
            WKUPEN12R::DISABLE => false,
            WKUPEN12R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN12R {
        match value {
            false => WKUPEN12R::DISABLE,
            true => WKUPEN12R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN12R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN12R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN13R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN13R {
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
            WKUPEN13R::DISABLE => false,
            WKUPEN13R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN13R {
        match value {
            false => WKUPEN13R::DISABLE,
            true => WKUPEN13R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN13R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN13R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN14R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN14R {
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
            WKUPEN14R::DISABLE => false,
            WKUPEN14R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN14R {
        match value {
            false => WKUPEN14R::DISABLE,
            true => WKUPEN14R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN14R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN14R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPEN15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN15R {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN15R {
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
            WKUPEN15R::DISABLE => false,
            WKUPEN15R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN15R {
        match value {
            false => WKUPEN15R::DISABLE,
            true => WKUPEN15R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN15R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN15R::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT0R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT0R {
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
            WKUPT0R::HIGH_TO_LOW => false,
            WKUPT0R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT0R {
        match value {
            false => WKUPT0R::HIGH_TO_LOW,
            true => WKUPT0R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT0R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT0R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT1R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT1R {
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
            WKUPT1R::HIGH_TO_LOW => false,
            WKUPT1R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT1R {
        match value {
            false => WKUPT1R::HIGH_TO_LOW,
            true => WKUPT1R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT1R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT1R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT2R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT2R {
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
            WKUPT2R::HIGH_TO_LOW => false,
            WKUPT2R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT2R {
        match value {
            false => WKUPT2R::HIGH_TO_LOW,
            true => WKUPT2R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT2R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT2R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT3R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT3R {
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
            WKUPT3R::HIGH_TO_LOW => false,
            WKUPT3R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT3R {
        match value {
            false => WKUPT3R::HIGH_TO_LOW,
            true => WKUPT3R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT3R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT3R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT4R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT4R {
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
            WKUPT4R::HIGH_TO_LOW => false,
            WKUPT4R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT4R {
        match value {
            false => WKUPT4R::HIGH_TO_LOW,
            true => WKUPT4R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT4R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT4R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT5R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT5R {
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
            WKUPT5R::HIGH_TO_LOW => false,
            WKUPT5R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT5R {
        match value {
            false => WKUPT5R::HIGH_TO_LOW,
            true => WKUPT5R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT5R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT5R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT6R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT6R {
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
            WKUPT6R::HIGH_TO_LOW => false,
            WKUPT6R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT6R {
        match value {
            false => WKUPT6R::HIGH_TO_LOW,
            true => WKUPT6R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT6R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT6R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT7R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT7R {
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
            WKUPT7R::HIGH_TO_LOW => false,
            WKUPT7R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT7R {
        match value {
            false => WKUPT7R::HIGH_TO_LOW,
            true => WKUPT7R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT7R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT7R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT8R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT8R {
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
            WKUPT8R::HIGH_TO_LOW => false,
            WKUPT8R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT8R {
        match value {
            false => WKUPT8R::HIGH_TO_LOW,
            true => WKUPT8R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT8R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT8R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT9R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT9R {
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
            WKUPT9R::HIGH_TO_LOW => false,
            WKUPT9R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT9R {
        match value {
            false => WKUPT9R::HIGH_TO_LOW,
            true => WKUPT9R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT9R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT9R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT10R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT10R {
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
            WKUPT10R::HIGH_TO_LOW => false,
            WKUPT10R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT10R {
        match value {
            false => WKUPT10R::HIGH_TO_LOW,
            true => WKUPT10R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT10R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT10R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT11R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT11R {
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
            WKUPT11R::HIGH_TO_LOW => false,
            WKUPT11R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT11R {
        match value {
            false => WKUPT11R::HIGH_TO_LOW,
            true => WKUPT11R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT11R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT11R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT12R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT12R {
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
            WKUPT12R::HIGH_TO_LOW => false,
            WKUPT12R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT12R {
        match value {
            false => WKUPT12R::HIGH_TO_LOW,
            true => WKUPT12R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT12R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT12R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT13R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT13R {
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
            WKUPT13R::HIGH_TO_LOW => false,
            WKUPT13R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT13R {
        match value {
            false => WKUPT13R::HIGH_TO_LOW,
            true => WKUPT13R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT13R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT13R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT14R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT14R {
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
            WKUPT14R::HIGH_TO_LOW => false,
            WKUPT14R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT14R {
        match value {
            false => WKUPT14R::HIGH_TO_LOW,
            true => WKUPT14R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT14R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT14R::LOW_TO_HIGH
    }
}
#[doc = "Possible values of the field `WKUPT15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT15R {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT15R {
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
            WKUPT15R::HIGH_TO_LOW => false,
            WKUPT15R::LOW_TO_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT15R {
        match value {
            false => WKUPT15R::HIGH_TO_LOW,
            true => WKUPT15R::LOW_TO_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH_TO_LOW`"]
    #[inline]
    pub fn is_high_to_low(&self) -> bool {
        *self == WKUPT15R::HIGH_TO_LOW
    }
    #[doc = "Checks if the value of the field is `LOW_TO_HIGH`"]
    #[inline]
    pub fn is_low_to_high(&self) -> bool {
        *self == WKUPT15R::LOW_TO_HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPEN0`"]
pub enum WKUPEN0W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN0W::DISABLE => false,
            WKUPEN0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN0W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN0W::ENABLE)
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
#[doc = "Values that can be written to the field `WKUPEN1`"]
pub enum WKUPEN1W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN1W::DISABLE => false,
            WKUPEN1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN1W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN1W::ENABLE)
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
#[doc = "Values that can be written to the field `WKUPEN2`"]
pub enum WKUPEN2W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN2W::DISABLE => false,
            WKUPEN2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN2W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN2W::ENABLE)
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
#[doc = "Values that can be written to the field `WKUPEN3`"]
pub enum WKUPEN3W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN3W::DISABLE => false,
            WKUPEN3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN3W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN3W::ENABLE)
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
#[doc = "Values that can be written to the field `WKUPEN4`"]
pub enum WKUPEN4W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN4W::DISABLE => false,
            WKUPEN4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN4W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN4W::ENABLE)
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
#[doc = "Values that can be written to the field `WKUPEN5`"]
pub enum WKUPEN5W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN5W::DISABLE => false,
            WKUPEN5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN5W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN5W::ENABLE)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPEN6`"]
pub enum WKUPEN6W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN6W::DISABLE => false,
            WKUPEN6W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN6W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN6W::ENABLE)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPEN7`"]
pub enum WKUPEN7W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN7W::DISABLE => false,
            WKUPEN7W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN7W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN7W::ENABLE)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPEN8`"]
pub enum WKUPEN8W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN8W::DISABLE => false,
            WKUPEN8W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN8W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN8W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN8W::ENABLE)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPEN9`"]
pub enum WKUPEN9W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN9W::DISABLE => false,
            WKUPEN9W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN9W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN9W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN9W::ENABLE)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPEN10`"]
pub enum WKUPEN10W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN10W::DISABLE => false,
            WKUPEN10W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN10W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN10W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN10W::ENABLE)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPEN11`"]
pub enum WKUPEN11W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN11W::DISABLE => false,
            WKUPEN11W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN11W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN11W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN11W::ENABLE)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPEN12`"]
pub enum WKUPEN12W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN12W::DISABLE => false,
            WKUPEN12W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN12W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN12W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN12W::ENABLE)
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
#[doc = "Values that can be written to the field `WKUPEN13`"]
pub enum WKUPEN13W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN13W::DISABLE => false,
            WKUPEN13W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN13W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN13W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN13W::ENABLE)
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
#[doc = "Values that can be written to the field `WKUPEN14`"]
pub enum WKUPEN14W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN14W::DISABLE => false,
            WKUPEN14W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN14W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN14W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN14W::ENABLE)
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
#[doc = "Values that can be written to the field `WKUPEN15`"]
pub enum WKUPEN15W {
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN15W::DISABLE => false,
            WKUPEN15W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN15W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "the corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN15W::DISABLE)
    }
    #[doc = "the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN15W::ENABLE)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT0`"]
pub enum WKUPT0W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT0W::HIGH_TO_LOW => false,
            WKUPT0W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT0W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT0W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT0W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT1`"]
pub enum WKUPT1W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT1W::HIGH_TO_LOW => false,
            WKUPT1W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT1W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT1W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT1W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT2`"]
pub enum WKUPT2W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT2W::HIGH_TO_LOW => false,
            WKUPT2W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT2W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT2W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT2W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT3`"]
pub enum WKUPT3W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT3W::HIGH_TO_LOW => false,
            WKUPT3W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT3W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT3W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT3W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT4`"]
pub enum WKUPT4W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT4W::HIGH_TO_LOW => false,
            WKUPT4W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT4W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT4W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT4W::LOW_TO_HIGH)
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
#[doc = "Values that can be written to the field `WKUPT5`"]
pub enum WKUPT5W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT5W::HIGH_TO_LOW => false,
            WKUPT5W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT5W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT5W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT5W::LOW_TO_HIGH)
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
#[doc = "Values that can be written to the field `WKUPT6`"]
pub enum WKUPT6W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT6W::HIGH_TO_LOW => false,
            WKUPT6W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT6W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT6W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT6W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT7`"]
pub enum WKUPT7W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT7W::HIGH_TO_LOW => false,
            WKUPT7W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT7W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT7W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT7W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT8`"]
pub enum WKUPT8W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT8W::HIGH_TO_LOW => false,
            WKUPT8W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT8W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT8W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT8W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT9`"]
pub enum WKUPT9W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT9W::HIGH_TO_LOW => false,
            WKUPT9W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT9W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT9W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT9W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT10`"]
pub enum WKUPT10W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT10W::HIGH_TO_LOW => false,
            WKUPT10W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT10W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT10W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT10W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT11`"]
pub enum WKUPT11W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT11W::HIGH_TO_LOW => false,
            WKUPT11W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT11W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT11W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT11W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT12`"]
pub enum WKUPT12W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT12W::HIGH_TO_LOW => false,
            WKUPT12W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT12W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT12W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT12W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT13`"]
pub enum WKUPT13W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT13W::HIGH_TO_LOW => false,
            WKUPT13W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT13W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT13W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT13W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT14`"]
pub enum WKUPT14W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT14W::HIGH_TO_LOW => false,
            WKUPT14W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT14W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT14W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT14W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPT15`"]
pub enum WKUPT15W {
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    HIGH_TO_LOW,
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    LOW_TO_HIGH,
}
impl WKUPT15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT15W::HIGH_TO_LOW => false,
            WKUPT15W::LOW_TO_HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT15W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "a high to low level transition for a period defined by WKUPDBC on the corresponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high_to_low(self) -> &'a mut W {
        self.variant(WKUPT15W::HIGH_TO_LOW)
    }
    #[doc = "a low to high level transition for a period defined by WKUPDBC on the correspond-ing wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low_to_high(self) -> &'a mut W {
        self.variant(WKUPT15W::LOW_TO_HIGH)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Wake-up Input Enable 0"]
    #[inline]
    pub fn wkupen0(&self) -> WKUPEN0R {
        WKUPEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake-up Input Enable 1"]
    #[inline]
    pub fn wkupen1(&self) -> WKUPEN1R {
        WKUPEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Wake-up Input Enable 2"]
    #[inline]
    pub fn wkupen2(&self) -> WKUPEN2R {
        WKUPEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Wake-up Input Enable 3"]
    #[inline]
    pub fn wkupen3(&self) -> WKUPEN3R {
        WKUPEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Wake-up Input Enable 4"]
    #[inline]
    pub fn wkupen4(&self) -> WKUPEN4R {
        WKUPEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Wake-up Input Enable 5"]
    #[inline]
    pub fn wkupen5(&self) -> WKUPEN5R {
        WKUPEN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Wake-up Input Enable 6"]
    #[inline]
    pub fn wkupen6(&self) -> WKUPEN6R {
        WKUPEN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Wake-up Input Enable 7"]
    #[inline]
    pub fn wkupen7(&self) -> WKUPEN7R {
        WKUPEN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Wake-up Input Enable 8"]
    #[inline]
    pub fn wkupen8(&self) -> WKUPEN8R {
        WKUPEN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Wake-up Input Enable 9"]
    #[inline]
    pub fn wkupen9(&self) -> WKUPEN9R {
        WKUPEN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Wake-up Input Enable 10"]
    #[inline]
    pub fn wkupen10(&self) -> WKUPEN10R {
        WKUPEN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Wake-up Input Enable 11"]
    #[inline]
    pub fn wkupen11(&self) -> WKUPEN11R {
        WKUPEN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Wake-up Input Enable 12"]
    #[inline]
    pub fn wkupen12(&self) -> WKUPEN12R {
        WKUPEN12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Wake-up Input Enable 13"]
    #[inline]
    pub fn wkupen13(&self) -> WKUPEN13R {
        WKUPEN13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Wake-up Input Enable 14"]
    #[inline]
    pub fn wkupen14(&self) -> WKUPEN14R {
        WKUPEN14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Wake-up Input Enable 15"]
    #[inline]
    pub fn wkupen15(&self) -> WKUPEN15R {
        WKUPEN15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Wake-up Input Type 0"]
    #[inline]
    pub fn wkupt0(&self) -> WKUPT0R {
        WKUPT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Wake-up Input Type 1"]
    #[inline]
    pub fn wkupt1(&self) -> WKUPT1R {
        WKUPT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Wake-up Input Type 2"]
    #[inline]
    pub fn wkupt2(&self) -> WKUPT2R {
        WKUPT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Wake-up Input Type 3"]
    #[inline]
    pub fn wkupt3(&self) -> WKUPT3R {
        WKUPT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Wake-up Input Type 4"]
    #[inline]
    pub fn wkupt4(&self) -> WKUPT4R {
        WKUPT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Wake-up Input Type 5"]
    #[inline]
    pub fn wkupt5(&self) -> WKUPT5R {
        WKUPT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Wake-up Input Type 6"]
    #[inline]
    pub fn wkupt6(&self) -> WKUPT6R {
        WKUPT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Wake-up Input Type 7"]
    #[inline]
    pub fn wkupt7(&self) -> WKUPT7R {
        WKUPT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Wake-up Input Type 8"]
    #[inline]
    pub fn wkupt8(&self) -> WKUPT8R {
        WKUPT8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Wake-up Input Type 9"]
    #[inline]
    pub fn wkupt9(&self) -> WKUPT9R {
        WKUPT9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Wake-up Input Type 10"]
    #[inline]
    pub fn wkupt10(&self) -> WKUPT10R {
        WKUPT10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Wake-up Input Type 11"]
    #[inline]
    pub fn wkupt11(&self) -> WKUPT11R {
        WKUPT11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Wake-up Input Type 12"]
    #[inline]
    pub fn wkupt12(&self) -> WKUPT12R {
        WKUPT12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Wake-up Input Type 13"]
    #[inline]
    pub fn wkupt13(&self) -> WKUPT13R {
        WKUPT13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Wake-up Input Type 14"]
    #[inline]
    pub fn wkupt14(&self) -> WKUPT14R {
        WKUPT14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Wake-up Input Type 15"]
    #[inline]
    pub fn wkupt15(&self) -> WKUPT15R {
        WKUPT15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Wake-up Input Enable 0"]
    #[inline]
    pub fn wkupen0(&mut self) -> _WKUPEN0W {
        _WKUPEN0W { w: self }
    }
    #[doc = "Bit 1 - Wake-up Input Enable 1"]
    #[inline]
    pub fn wkupen1(&mut self) -> _WKUPEN1W {
        _WKUPEN1W { w: self }
    }
    #[doc = "Bit 2 - Wake-up Input Enable 2"]
    #[inline]
    pub fn wkupen2(&mut self) -> _WKUPEN2W {
        _WKUPEN2W { w: self }
    }
    #[doc = "Bit 3 - Wake-up Input Enable 3"]
    #[inline]
    pub fn wkupen3(&mut self) -> _WKUPEN3W {
        _WKUPEN3W { w: self }
    }
    #[doc = "Bit 4 - Wake-up Input Enable 4"]
    #[inline]
    pub fn wkupen4(&mut self) -> _WKUPEN4W {
        _WKUPEN4W { w: self }
    }
    #[doc = "Bit 5 - Wake-up Input Enable 5"]
    #[inline]
    pub fn wkupen5(&mut self) -> _WKUPEN5W {
        _WKUPEN5W { w: self }
    }
    #[doc = "Bit 6 - Wake-up Input Enable 6"]
    #[inline]
    pub fn wkupen6(&mut self) -> _WKUPEN6W {
        _WKUPEN6W { w: self }
    }
    #[doc = "Bit 7 - Wake-up Input Enable 7"]
    #[inline]
    pub fn wkupen7(&mut self) -> _WKUPEN7W {
        _WKUPEN7W { w: self }
    }
    #[doc = "Bit 8 - Wake-up Input Enable 8"]
    #[inline]
    pub fn wkupen8(&mut self) -> _WKUPEN8W {
        _WKUPEN8W { w: self }
    }
    #[doc = "Bit 9 - Wake-up Input Enable 9"]
    #[inline]
    pub fn wkupen9(&mut self) -> _WKUPEN9W {
        _WKUPEN9W { w: self }
    }
    #[doc = "Bit 10 - Wake-up Input Enable 10"]
    #[inline]
    pub fn wkupen10(&mut self) -> _WKUPEN10W {
        _WKUPEN10W { w: self }
    }
    #[doc = "Bit 11 - Wake-up Input Enable 11"]
    #[inline]
    pub fn wkupen11(&mut self) -> _WKUPEN11W {
        _WKUPEN11W { w: self }
    }
    #[doc = "Bit 12 - Wake-up Input Enable 12"]
    #[inline]
    pub fn wkupen12(&mut self) -> _WKUPEN12W {
        _WKUPEN12W { w: self }
    }
    #[doc = "Bit 13 - Wake-up Input Enable 13"]
    #[inline]
    pub fn wkupen13(&mut self) -> _WKUPEN13W {
        _WKUPEN13W { w: self }
    }
    #[doc = "Bit 14 - Wake-up Input Enable 14"]
    #[inline]
    pub fn wkupen14(&mut self) -> _WKUPEN14W {
        _WKUPEN14W { w: self }
    }
    #[doc = "Bit 15 - Wake-up Input Enable 15"]
    #[inline]
    pub fn wkupen15(&mut self) -> _WKUPEN15W {
        _WKUPEN15W { w: self }
    }
    #[doc = "Bit 16 - Wake-up Input Type 0"]
    #[inline]
    pub fn wkupt0(&mut self) -> _WKUPT0W {
        _WKUPT0W { w: self }
    }
    #[doc = "Bit 17 - Wake-up Input Type 1"]
    #[inline]
    pub fn wkupt1(&mut self) -> _WKUPT1W {
        _WKUPT1W { w: self }
    }
    #[doc = "Bit 18 - Wake-up Input Type 2"]
    #[inline]
    pub fn wkupt2(&mut self) -> _WKUPT2W {
        _WKUPT2W { w: self }
    }
    #[doc = "Bit 19 - Wake-up Input Type 3"]
    #[inline]
    pub fn wkupt3(&mut self) -> _WKUPT3W {
        _WKUPT3W { w: self }
    }
    #[doc = "Bit 20 - Wake-up Input Type 4"]
    #[inline]
    pub fn wkupt4(&mut self) -> _WKUPT4W {
        _WKUPT4W { w: self }
    }
    #[doc = "Bit 21 - Wake-up Input Type 5"]
    #[inline]
    pub fn wkupt5(&mut self) -> _WKUPT5W {
        _WKUPT5W { w: self }
    }
    #[doc = "Bit 22 - Wake-up Input Type 6"]
    #[inline]
    pub fn wkupt6(&mut self) -> _WKUPT6W {
        _WKUPT6W { w: self }
    }
    #[doc = "Bit 23 - Wake-up Input Type 7"]
    #[inline]
    pub fn wkupt7(&mut self) -> _WKUPT7W {
        _WKUPT7W { w: self }
    }
    #[doc = "Bit 24 - Wake-up Input Type 8"]
    #[inline]
    pub fn wkupt8(&mut self) -> _WKUPT8W {
        _WKUPT8W { w: self }
    }
    #[doc = "Bit 25 - Wake-up Input Type 9"]
    #[inline]
    pub fn wkupt9(&mut self) -> _WKUPT9W {
        _WKUPT9W { w: self }
    }
    #[doc = "Bit 26 - Wake-up Input Type 10"]
    #[inline]
    pub fn wkupt10(&mut self) -> _WKUPT10W {
        _WKUPT10W { w: self }
    }
    #[doc = "Bit 27 - Wake-up Input Type 11"]
    #[inline]
    pub fn wkupt11(&mut self) -> _WKUPT11W {
        _WKUPT11W { w: self }
    }
    #[doc = "Bit 28 - Wake-up Input Type 12"]
    #[inline]
    pub fn wkupt12(&mut self) -> _WKUPT12W {
        _WKUPT12W { w: self }
    }
    #[doc = "Bit 29 - Wake-up Input Type 13"]
    #[inline]
    pub fn wkupt13(&mut self) -> _WKUPT13W {
        _WKUPT13W { w: self }
    }
    #[doc = "Bit 30 - Wake-up Input Type 14"]
    #[inline]
    pub fn wkupt14(&mut self) -> _WKUPT14W {
        _WKUPT14W { w: self }
    }
    #[doc = "Bit 31 - Wake-up Input Type 15"]
    #[inline]
    pub fn wkupt15(&mut self) -> _WKUPT15W {
        _WKUPT15W { w: self }
    }
}
