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
#[doc = r" Value of the field"]
pub struct IDTIR {
    bits: bool,
}
impl IDTIR {
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
pub struct VBUSTIR {
    bits: bool,
}
impl VBUSTIR {
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
pub struct SRPIR {
    bits: bool,
}
impl SRPIR {
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
pub struct VBERRIR {
    bits: bool,
}
impl VBERRIR {
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
pub struct BCERRIR {
    bits: bool,
}
impl BCERRIR {
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
pub struct ROLEEXIR {
    bits: bool,
}
impl ROLEEXIR {
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
pub struct HNPERRIR {
    bits: bool,
}
impl HNPERRIR {
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
pub struct STOIR {
    bits: bool,
}
impl STOIR {
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
pub struct VBUSRQR {
    bits: bool,
}
impl VBUSRQR {
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
pub struct IDR {
    bits: bool,
}
impl IDR {
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
pub struct VBUSR {
    bits: bool,
}
impl VBUSR {
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
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "Full-Speed mode"]
    FULL_SPEED,
    #[doc = "High-Speed mode"]
    HIGH_SPEED,
    #[doc = "Low-Speed mode"]
    LOW_SPEED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPEEDR::FULL_SPEED => 0,
            SPEEDR::HIGH_SPEED => 1,
            SPEEDR::LOW_SPEED => 2,
            SPEEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPEEDR {
        match value {
            0 => SPEEDR::FULL_SPEED,
            1 => SPEEDR::HIGH_SPEED,
            2 => SPEEDR::LOW_SPEED,
            i => SPEEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline]
    pub fn is_full_speed(&self) -> bool {
        *self == SPEEDR::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline]
    pub fn is_high_speed(&self) -> bool {
        *self == SPEEDR::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED`"]
    #[inline]
    pub fn is_low_speed(&self) -> bool {
        *self == SPEEDR::LOW_SPEED
    }
}
#[doc = r" Value of the field"]
pub struct CLKUSABLER {
    bits: bool,
}
impl CLKUSABLER {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - ID Transition Interrupt"]
    #[inline]
    pub fn idti(&self) -> IDTIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDTIR { bits }
    }
    #[doc = "Bit 1 - VBus Transition Interrupt"]
    #[inline]
    pub fn vbusti(&self) -> VBUSTIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSTIR { bits }
    }
    #[doc = "Bit 2 - SRP Interrupt"]
    #[inline]
    pub fn srpi(&self) -> SRPIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRPIR { bits }
    }
    #[doc = "Bit 3 - VBus Error Interrupt"]
    #[inline]
    pub fn vberri(&self) -> VBERRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBERRIR { bits }
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt"]
    #[inline]
    pub fn bcerri(&self) -> BCERRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BCERRIR { bits }
    }
    #[doc = "Bit 5 - Role Exchange Interrupt"]
    #[inline]
    pub fn roleexi(&self) -> ROLEEXIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ROLEEXIR { bits }
    }
    #[doc = "Bit 6 - HNP Error Interrupt"]
    #[inline]
    pub fn hnperri(&self) -> HNPERRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HNPERRIR { bits }
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt"]
    #[inline]
    pub fn stoi(&self) -> STOIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOIR { bits }
    }
    #[doc = "Bit 9 - VBus Request"]
    #[inline]
    pub fn vbusrq(&self) -> VBUSRQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSRQR { bits }
    }
    #[doc = "Bit 10 - UOTGID Pin State"]
    #[inline]
    pub fn id(&self) -> IDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDR { bits }
    }
    #[doc = "Bit 11 - VBus Level"]
    #[inline]
    pub fn vbus(&self) -> VBUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSR { bits }
    }
    #[doc = "Bits 12:13 - Speed Status"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline]
    pub fn clkusable(&self) -> CLKUSABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKUSABLER { bits }
    }
}
