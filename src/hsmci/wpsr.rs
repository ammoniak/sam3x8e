#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WPSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `WP_VS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_VSR {
    #[doc = "No Write Protection Violation occurred since the last read of this register (WP_SR)"]
    NONE,
    #[doc = "Write Protection detected unauthorized attempt to write a control register had occurred (since the last read.)"]
    WRITE,
    #[doc = "Software reset had been performed while Write Protection was enabled (since the last read)."]
    RESET,
    #[doc = "Both Write Protection violation and software reset with Write Protection enabled have occurred since the last read."]
    BOTH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WP_VSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WP_VSR::NONE => 0,
            WP_VSR::WRITE => 1,
            WP_VSR::RESET => 2,
            WP_VSR::BOTH => 3,
            WP_VSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WP_VSR {
        match value {
            0 => WP_VSR::NONE,
            1 => WP_VSR::WRITE,
            2 => WP_VSR::RESET,
            3 => WP_VSR::BOTH,
            i => WP_VSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == WP_VSR::NONE
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline]
    pub fn is_write(&self) -> bool {
        *self == WP_VSR::WRITE
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == WP_VSR::RESET
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline]
    pub fn is_both(&self) -> bool {
        *self == WP_VSR::BOTH
    }
}
#[doc = r" Value of the field"]
pub struct WP_VSRCR {
    bits: u16,
}
impl WP_VSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Write Protection Violation Status"]
    #[inline]
    pub fn wp_vs(&self) -> WP_VSR {
        WP_VSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:23 - Write Protection Violation SouRCe"]
    #[inline]
    pub fn wp_vsrc(&self) -> WP_VSRCR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WP_VSRCR { bits }
    }
}
