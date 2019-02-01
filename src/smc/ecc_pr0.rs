#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ECC_PR0 {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Bit Address"]
    #[inline]
    pub fn bitaddr(&self) -> BITADDRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BITADDRR { bits }
    }
    #[doc = "Bits 4:15 - Word Address"]
    #[inline]
    pub fn wordaddr(&self) -> WORDADDRR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        WORDADDRR { bits }
    }
}
