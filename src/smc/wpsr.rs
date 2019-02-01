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
#[doc = r" Value of the field"]
pub struct WP_VSR {
    bits: u8,
}
impl WP_VSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WP_VSR { bits }
    }
    #[doc = "Bits 8:23 - Write Protection Violation Source"]
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
