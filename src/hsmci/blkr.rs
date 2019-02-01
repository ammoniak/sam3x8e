#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BLKR {
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
#[doc = "Possible values of the field `BCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCNTR {
    #[doc = "MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    MULTIPLE,
    #[doc = "SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    BYTE,
    #[doc = "SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    BLOCK,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BCNTR::MULTIPLE => 0,
            BCNTR::BYTE => 4,
            BCNTR::BLOCK => 5,
            BCNTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BCNTR {
        match value {
            0 => BCNTR::MULTIPLE,
            4 => BCNTR::BYTE,
            5 => BCNTR::BLOCK,
            i => BCNTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline]
    pub fn is_multiple(&self) -> bool {
        *self == BCNTR::MULTIPLE
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == BCNTR::BYTE
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline]
    pub fn is_block(&self) -> bool {
        *self == BCNTR::BLOCK
    }
}
#[doc = r" Value of the field"]
pub struct BLKLENR {
    bits: u16,
}
impl BLKLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `BCNT`"]
pub enum BCNTW {
    #[doc = "MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    MULTIPLE,
    #[doc = "SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    BYTE,
    #[doc = "SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    BLOCK,
}
impl BCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BCNTW::MULTIPLE => 0,
            BCNTW::BYTE => 4,
            BCNTW::BLOCK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _BCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCNTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MMC/SDCARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    #[inline]
    pub fn multiple(self) -> &'a mut W {
        self.variant(BCNTW::MULTIPLE)
    }
    #[doc = "SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(BCNTW::BYTE)
    }
    #[doc = "SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline]
    pub fn block(self) -> &'a mut W {
        self.variant(BCNTW::BLOCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BLKLENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline]
    pub fn bcnt(&self) -> BCNTR {
        BCNTR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        })
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline]
    pub fn blklen(&self) -> BLKLENR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BLKLENR { bits }
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
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline]
    pub fn bcnt(&mut self) -> _BCNTW {
        _BCNTW { w: self }
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline]
    pub fn blklen(&mut self) -> _BLKLENW {
        _BLKLENW { w: self }
    }
}
