#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ECC_SR2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RECERR8R {
    bits: bool,
}
impl RECERR8R {
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
pub struct ECCERR8R {
    bits: u8,
}
impl ECCERR8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RECERR9R {
    bits: bool,
}
impl RECERR9R {
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
pub struct ECCERR9R {
    bits: bool,
}
impl ECCERR9R {
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
pub struct MULERR9R {
    bits: bool,
}
impl MULERR9R {
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
pub struct RECERR10R {
    bits: bool,
}
impl RECERR10R {
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
pub struct ECCERR10R {
    bits: bool,
}
impl ECCERR10R {
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
pub struct MULERR10R {
    bits: bool,
}
impl MULERR10R {
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
pub struct RECERR11R {
    bits: bool,
}
impl RECERR11R {
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
pub struct ECCERR11R {
    bits: bool,
}
impl ECCERR11R {
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
pub struct MULERR11R {
    bits: bool,
}
impl MULERR11R {
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
pub struct RECERR12R {
    bits: bool,
}
impl RECERR12R {
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
pub struct ECCERR12R {
    bits: u8,
}
impl ECCERR12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RECERR13R {
    bits: bool,
}
impl RECERR13R {
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
pub struct ECCERR13R {
    bits: u8,
}
impl ECCERR13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RECERR14R {
    bits: bool,
}
impl RECERR14R {
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
pub struct ECCERR14R {
    bits: u8,
}
impl ECCERR14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RECERR15R {
    bits: bool,
}
impl RECERR15R {
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
pub struct ECCERR15R {
    bits: u8,
}
impl ECCERR15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Recoverable Error in the page between the 2048th and the 2303rd bytes"]
    #[inline]
    pub fn recerr8(&self) -> RECERR8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECERR8R { bits }
    }
    #[doc = "Bits 1:2 - ECC Error in the page between the 2048th and the 2303rd bytes"]
    #[inline]
    pub fn eccerr8(&self) -> ECCERR8R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECCERR8R { bits }
    }
    #[doc = "Bit 4 - Recoverable Error in the page between the 2304th and the 2559th bytes"]
    #[inline]
    pub fn recerr9(&self) -> RECERR9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECERR9R { bits }
    }
    #[doc = "Bit 5 - ECC Error in the page between the 2304th and the 2559th bytes"]
    #[inline]
    pub fn eccerr9(&self) -> ECCERR9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ECCERR9R { bits }
    }
    #[doc = "Bit 6 - Multiple Error in the page between the 2304th and the 2559th bytes"]
    #[inline]
    pub fn mulerr9(&self) -> MULERR9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MULERR9R { bits }
    }
    #[doc = "Bit 8 - Recoverable Error in the page between the 2560th and the 2815th bytes"]
    #[inline]
    pub fn recerr10(&self) -> RECERR10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECERR10R { bits }
    }
    #[doc = "Bit 9 - ECC Error in the page between the 2560th and the 2815th bytes"]
    #[inline]
    pub fn eccerr10(&self) -> ECCERR10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ECCERR10R { bits }
    }
    #[doc = "Bit 10 - Multiple Error in the page between the 2560th and the 2815th bytes"]
    #[inline]
    pub fn mulerr10(&self) -> MULERR10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MULERR10R { bits }
    }
    #[doc = "Bit 12 - Recoverable Error in the page between the 2816th and the 3071st bytes"]
    #[inline]
    pub fn recerr11(&self) -> RECERR11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECERR11R { bits }
    }
    #[doc = "Bit 13 - ECC Error in the page between the 2816th and the 3071st bytes"]
    #[inline]
    pub fn eccerr11(&self) -> ECCERR11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ECCERR11R { bits }
    }
    #[doc = "Bit 14 - Multiple Error in the page between the 2816th and the 3071st bytes"]
    #[inline]
    pub fn mulerr11(&self) -> MULERR11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MULERR11R { bits }
    }
    #[doc = "Bit 16 - Recoverable Error in the page between the 3072nd and the 3327th bytes"]
    #[inline]
    pub fn recerr12(&self) -> RECERR12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECERR12R { bits }
    }
    #[doc = "Bits 17:18 - ECC Error in the page between the 3072nd and the 3327th bytes"]
    #[inline]
    pub fn eccerr12(&self) -> ECCERR12R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECCERR12R { bits }
    }
    #[doc = "Bit 20 - Recoverable Error in the page between the 3328th and the 3583rd bytes"]
    #[inline]
    pub fn recerr13(&self) -> RECERR13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECERR13R { bits }
    }
    #[doc = "Bits 21:22 - ECC Error in the page between the 3328th and the 3583rd bytes"]
    #[inline]
    pub fn eccerr13(&self) -> ECCERR13R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECCERR13R { bits }
    }
    #[doc = "Bit 24 - Recoverable Error in the page between the 3584th and the 3839th bytes"]
    #[inline]
    pub fn recerr14(&self) -> RECERR14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECERR14R { bits }
    }
    #[doc = "Bits 25:26 - ECC Error in the page between the 3584th and the 3839th bytes"]
    #[inline]
    pub fn eccerr14(&self) -> ECCERR14R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECCERR14R { bits }
    }
    #[doc = "Bit 28 - Recoverable Error in the page between the 3840th and the 4095th bytes"]
    #[inline]
    pub fn recerr15(&self) -> RECERR15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RECERR15R { bits }
    }
    #[doc = "Bits 29:30 - ECC Error in the page between the 3840th and the 4095th bytes"]
    #[inline]
    pub fn eccerr15(&self) -> ECCERR15R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ECCERR15R { bits }
    }
}
