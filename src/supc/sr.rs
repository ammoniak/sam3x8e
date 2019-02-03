#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `FWUPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUPSR {
    #[doc = "no wake-up due to the assertion of the FWUP pin has occurred since the last read of SUPC_SR."]
    NO,
    #[doc = "at least one wake-up due to the assertion of the FWUP pin has occurred since the last read of SUPC_SR."]
    PRESENT,
}
impl FWUPSR {
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
            FWUPSR::NO => false,
            FWUPSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWUPSR {
        match value {
            false => FWUPSR::NO,
            true => FWUPSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == FWUPSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == FWUPSR::PRESENT
    }
}
#[doc = "Possible values of the field `WKUPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPSR {
    #[doc = "no wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    NO,
    #[doc = "at least one wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    PRESENT,
}
impl WKUPSR {
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
            WKUPSR::NO => false,
            WKUPSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPSR {
        match value {
            false => WKUPSR::NO,
            true => WKUPSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == WKUPSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == WKUPSR::PRESENT
    }
}
#[doc = "Possible values of the field `SMWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMWSR {
    #[doc = "no wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    NO,
    #[doc = "at least one wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    PRESENT,
}
impl SMWSR {
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
            SMWSR::NO => false,
            SMWSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMWSR {
        match value {
            false => SMWSR::NO,
            true => SMWSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SMWSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == SMWSR::PRESENT
    }
}
#[doc = "Possible values of the field `BODRSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTSR {
    #[doc = "no core brownout rising edge event has been detected since the last read of the SUPC_SR."]
    NO,
    #[doc = "at least one brownout output rising edge event has been detected since the last read of the SUPC_SR."]
    PRESENT,
}
impl BODRSTSR {
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
            BODRSTSR::NO => false,
            BODRSTSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODRSTSR {
        match value {
            false => BODRSTSR::NO,
            true => BODRSTSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == BODRSTSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == BODRSTSR::PRESENT
    }
}
#[doc = "Possible values of the field `SMRSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTSR {
    #[doc = "no supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    NO,
    #[doc = "at least one supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    PRESENT,
}
impl SMRSTSR {
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
            SMRSTSR::NO => false,
            SMRSTSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMRSTSR {
        match value {
            false => SMRSTSR::NO,
            true => SMRSTSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SMRSTSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == SMRSTSR::PRESENT
    }
}
#[doc = "Possible values of the field `SMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSR {
    #[doc = "no supply monitor detection since the last read of SUPC_SR."]
    NO,
    #[doc = "at least one supply monitor detection since the last read of SUPC_SR."]
    PRESENT,
}
impl SMSR {
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
            SMSR::NO => false,
            SMSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMSR {
        match value {
            false => SMSR::NO,
            true => SMSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SMSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == SMSR::PRESENT
    }
}
#[doc = "Possible values of the field `SMOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMOSR {
    #[doc = "the supply monitor detected VDDUTMI higher than its threshold at its last measurement."]
    HIGH,
    #[doc = "the supply monitor detected VDDUTMI lower than its threshold at its last measurement."]
    LOW,
}
impl SMOSR {
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
            SMOSR::HIGH => false,
            SMOSR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMOSR {
        match value {
            false => SMOSR::HIGH,
            true => SMOSR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SMOSR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SMOSR::LOW
    }
}
#[doc = "Possible values of the field `OSCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSELR {
    #[doc = "the slow clock, SLCK is generated by the embedded 32-kHz RC oscillator."]
    RC,
    #[doc = "the slow clock, SLCK is generated by the 32-kHz crystal oscillator."]
    CRYST,
}
impl OSCSELR {
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
            OSCSELR::RC => false,
            OSCSELR::CRYST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCSELR {
        match value {
            false => OSCSELR::RC,
            true => OSCSELR::CRYST,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline]
    pub fn is_rc(&self) -> bool {
        *self == OSCSELR::RC
    }
    #[doc = "Checks if the value of the field is `CRYST`"]
    #[inline]
    pub fn is_cryst(&self) -> bool {
        *self == OSCSELR::CRYST
    }
}
#[doc = "Possible values of the field `FWUPIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUPISR {
    #[doc = "FWUP input is tied low."]
    LOW,
    #[doc = "FWUP input is tied high."]
    HIGH,
}
impl FWUPISR {
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
            FWUPISR::LOW => false,
            FWUPISR::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWUPISR {
        match value {
            false => FWUPISR::LOW,
            true => FWUPISR::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == FWUPISR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == FWUPISR::HIGH
    }
}
#[doc = "Possible values of the field `WKUPIS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS0R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS0R {
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
            WKUPIS0R::DIS => false,
            WKUPIS0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS0R {
        match value {
            false => WKUPIS0R::DIS,
            true => WKUPIS0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS0R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS1R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS1R {
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
            WKUPIS1R::DIS => false,
            WKUPIS1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS1R {
        match value {
            false => WKUPIS1R::DIS,
            true => WKUPIS1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS1R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS2R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS2R {
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
            WKUPIS2R::DIS => false,
            WKUPIS2R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS2R {
        match value {
            false => WKUPIS2R::DIS,
            true => WKUPIS2R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS2R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS2R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS3R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS3R {
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
            WKUPIS3R::DIS => false,
            WKUPIS3R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS3R {
        match value {
            false => WKUPIS3R::DIS,
            true => WKUPIS3R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS3R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS3R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS4R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS4R {
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
            WKUPIS4R::DIS => false,
            WKUPIS4R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS4R {
        match value {
            false => WKUPIS4R::DIS,
            true => WKUPIS4R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS4R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS4R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS5R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS5R {
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
            WKUPIS5R::DIS => false,
            WKUPIS5R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS5R {
        match value {
            false => WKUPIS5R::DIS,
            true => WKUPIS5R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS5R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS5R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS6R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS6R {
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
            WKUPIS6R::DIS => false,
            WKUPIS6R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS6R {
        match value {
            false => WKUPIS6R::DIS,
            true => WKUPIS6R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS6R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS6R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS7R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS7R {
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
            WKUPIS7R::DIS => false,
            WKUPIS7R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS7R {
        match value {
            false => WKUPIS7R::DIS,
            true => WKUPIS7R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS7R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS7R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS8R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS8R {
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
            WKUPIS8R::DIS => false,
            WKUPIS8R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS8R {
        match value {
            false => WKUPIS8R::DIS,
            true => WKUPIS8R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS8R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS8R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS9R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS9R {
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
            WKUPIS9R::DIS => false,
            WKUPIS9R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS9R {
        match value {
            false => WKUPIS9R::DIS,
            true => WKUPIS9R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS9R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS9R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS10R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS10R {
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
            WKUPIS10R::DIS => false,
            WKUPIS10R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS10R {
        match value {
            false => WKUPIS10R::DIS,
            true => WKUPIS10R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS10R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS10R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS11R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS11R {
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
            WKUPIS11R::DIS => false,
            WKUPIS11R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS11R {
        match value {
            false => WKUPIS11R::DIS,
            true => WKUPIS11R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS11R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS11R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS12R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS12R {
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
            WKUPIS12R::DIS => false,
            WKUPIS12R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS12R {
        match value {
            false => WKUPIS12R::DIS,
            true => WKUPIS12R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS12R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS12R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS13R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS13R {
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
            WKUPIS13R::DIS => false,
            WKUPIS13R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS13R {
        match value {
            false => WKUPIS13R::DIS,
            true => WKUPIS13R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS13R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS13R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS14R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS14R {
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
            WKUPIS14R::DIS => false,
            WKUPIS14R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS14R {
        match value {
            false => WKUPIS14R::DIS,
            true => WKUPIS14R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS14R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS14R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS15R {
    #[doc = "the corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "the corresponding wake-up input was active at the time the debouncer triggered a wake-up event."]
    EN,
}
impl WKUPIS15R {
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
            WKUPIS15R::DIS => false,
            WKUPIS15R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS15R {
        match value {
            false => WKUPIS15R::DIS,
            true => WKUPIS15R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS15R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS15R::EN
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - FWUP Wake-up Status"]
    #[inline]
    pub fn fwups(&self) -> FWUPSR {
        FWUPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - WKUP Wake-up Status"]
    #[inline]
    pub fn wkups(&self) -> WKUPSR {
        WKUPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Supply Monitor Detection Wake-up Status"]
    #[inline]
    pub fn smws(&self) -> SMWSR {
        SMWSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Brownout Detector Reset Status"]
    #[inline]
    pub fn bodrsts(&self) -> BODRSTSR {
        BODRSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Supply Monitor Reset Status"]
    #[inline]
    pub fn smrsts(&self) -> SMRSTSR {
        SMRSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Supply Monitor Status"]
    #[inline]
    pub fn sms(&self) -> SMSR {
        SMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Supply Monitor Output Status"]
    #[inline]
    pub fn smos(&self) -> SMOSR {
        SMOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - 32-kHz Oscillator Selection Status"]
    #[inline]
    pub fn oscsel(&self) -> OSCSELR {
        OSCSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - FWUP Input Status"]
    #[inline]
    pub fn fwupis(&self) -> FWUPISR {
        FWUPISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - WKUP Input Status 0"]
    #[inline]
    pub fn wkupis0(&self) -> WKUPIS0R {
        WKUPIS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - WKUP Input Status 1"]
    #[inline]
    pub fn wkupis1(&self) -> WKUPIS1R {
        WKUPIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - WKUP Input Status 2"]
    #[inline]
    pub fn wkupis2(&self) -> WKUPIS2R {
        WKUPIS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - WKUP Input Status 3"]
    #[inline]
    pub fn wkupis3(&self) -> WKUPIS3R {
        WKUPIS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - WKUP Input Status 4"]
    #[inline]
    pub fn wkupis4(&self) -> WKUPIS4R {
        WKUPIS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - WKUP Input Status 5"]
    #[inline]
    pub fn wkupis5(&self) -> WKUPIS5R {
        WKUPIS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - WKUP Input Status 6"]
    #[inline]
    pub fn wkupis6(&self) -> WKUPIS6R {
        WKUPIS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - WKUP Input Status 7"]
    #[inline]
    pub fn wkupis7(&self) -> WKUPIS7R {
        WKUPIS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - WKUP Input Status 8"]
    #[inline]
    pub fn wkupis8(&self) -> WKUPIS8R {
        WKUPIS8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - WKUP Input Status 9"]
    #[inline]
    pub fn wkupis9(&self) -> WKUPIS9R {
        WKUPIS9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - WKUP Input Status 10"]
    #[inline]
    pub fn wkupis10(&self) -> WKUPIS10R {
        WKUPIS10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - WKUP Input Status 11"]
    #[inline]
    pub fn wkupis11(&self) -> WKUPIS11R {
        WKUPIS11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - WKUP Input Status 12"]
    #[inline]
    pub fn wkupis12(&self) -> WKUPIS12R {
        WKUPIS12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - WKUP Input Status 13"]
    #[inline]
    pub fn wkupis13(&self) -> WKUPIS13R {
        WKUPIS13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - WKUP Input Status 14"]
    #[inline]
    pub fn wkupis14(&self) -> WKUPIS14R {
        WKUPIS14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - WKUP Input Status 15"]
    #[inline]
    pub fn wkupis15(&self) -> WKUPIS15R {
        WKUPIS15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
