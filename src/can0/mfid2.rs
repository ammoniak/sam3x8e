#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MFID2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MFIDR {
    bits: u32,
}
impl MFIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:28 - Family ID"]
    #[inline]
    pub fn mfid(&self) -> MFIDR {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        MFIDR { bits }
    }
}
