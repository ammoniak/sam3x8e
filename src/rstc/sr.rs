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
pub struct URSTSR {
    bits: bool,
}
impl URSTSR {
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
#[doc = "Possible values of the field `RSTTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTTYPR {
    #[doc = "First power-up Reset"]
    GENERALRESET,
    #[doc = "Return from Backup Mode"]
    BACKUPRESET,
    #[doc = "Watchdog fault occurred"]
    WATCHDOGRESET,
    #[doc = "Processor reset required by the software"]
    SOFTWARERESET,
    #[doc = "NRST pin detected low"]
    USERRESET,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RSTTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSTTYPR::GENERALRESET => 0,
            RSTTYPR::BACKUPRESET => 1,
            RSTTYPR::WATCHDOGRESET => 2,
            RSTTYPR::SOFTWARERESET => 3,
            RSTTYPR::USERRESET => 4,
            RSTTYPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSTTYPR {
        match value {
            0 => RSTTYPR::GENERALRESET,
            1 => RSTTYPR::BACKUPRESET,
            2 => RSTTYPR::WATCHDOGRESET,
            3 => RSTTYPR::SOFTWARERESET,
            4 => RSTTYPR::USERRESET,
            i => RSTTYPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GENERALRESET`"]
    #[inline]
    pub fn is_general_reset(&self) -> bool {
        *self == RSTTYPR::GENERALRESET
    }
    #[doc = "Checks if the value of the field is `BACKUPRESET`"]
    #[inline]
    pub fn is_backup_reset(&self) -> bool {
        *self == RSTTYPR::BACKUPRESET
    }
    #[doc = "Checks if the value of the field is `WATCHDOGRESET`"]
    #[inline]
    pub fn is_watchdog_reset(&self) -> bool {
        *self == RSTTYPR::WATCHDOGRESET
    }
    #[doc = "Checks if the value of the field is `SOFTWARERESET`"]
    #[inline]
    pub fn is_software_reset(&self) -> bool {
        *self == RSTTYPR::SOFTWARERESET
    }
    #[doc = "Checks if the value of the field is `USERRESET`"]
    #[inline]
    pub fn is_user_reset(&self) -> bool {
        *self == RSTTYPR::USERRESET
    }
}
#[doc = r" Value of the field"]
pub struct NRSTLR {
    bits: bool,
}
impl NRSTLR {
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
pub struct SRCMPR {
    bits: bool,
}
impl SRCMPR {
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
    #[doc = "Bit 0 - User Reset Status"]
    #[inline]
    pub fn ursts(&self) -> URSTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        URSTSR { bits }
    }
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline]
    pub fn rsttyp(&self) -> RSTTYPR {
        RSTTYPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline]
    pub fn nrstl(&self) -> NRSTLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NRSTLR { bits }
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline]
    pub fn srcmp(&self) -> SRCMPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRCMPR { bits }
    }
}
