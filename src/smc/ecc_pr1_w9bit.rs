#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ECC_PR1_W9BIT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct BITADDRR {
    bits: u8,
}
impl BITADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WORDADDRR {
    bits: u16,
}
impl WORDADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NPARITYR {
    bits: u16,
}
impl NPARITYR {
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
    #[doc = "Bits 0:2 - Corrupted Bit Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
    #[inline]
    pub fn bitaddr(&self) -> BITADDRR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BITADDRR { bits }
    }
    #[doc = "Bits 3:11 - Corrupted Word Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
    #[inline]
    pub fn wordaddr(&self) -> WORDADDRR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WORDADDRR { bits }
    }
    #[doc = "Bits 12:23 - Parity N"]
    #[inline]
    pub fn nparity(&self) -> NPARITYR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        NPARITYR { bits }
    }
}
