#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `PAGESIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAGESIZER {
    #[doc = "Main area 512 Bytes"]
    PS512,
    #[doc = "Main area 1024 Bytes"]
    PS1024,
    #[doc = "Main area 2048 Bytes"]
    PS2048,
    #[doc = "Main area 4096 Bytes"]
    PS4096,
}
impl PAGESIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PAGESIZER::PS512 => 0,
            PAGESIZER::PS1024 => 1,
            PAGESIZER::PS2048 => 2,
            PAGESIZER::PS4096 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PAGESIZER {
        match value {
            0 => PAGESIZER::PS512,
            1 => PAGESIZER::PS1024,
            2 => PAGESIZER::PS2048,
            3 => PAGESIZER::PS4096,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PS512`"]
    #[inline]
    pub fn is_ps512(&self) -> bool {
        *self == PAGESIZER::PS512
    }
    #[doc = "Checks if the value of the field is `PS1024`"]
    #[inline]
    pub fn is_ps1024(&self) -> bool {
        *self == PAGESIZER::PS1024
    }
    #[doc = "Checks if the value of the field is `PS2048`"]
    #[inline]
    pub fn is_ps2048(&self) -> bool {
        *self == PAGESIZER::PS2048
    }
    #[doc = "Checks if the value of the field is `PS4096`"]
    #[inline]
    pub fn is_ps4096(&self) -> bool {
        *self == PAGESIZER::PS4096
    }
}
#[doc = r" Value of the field"]
pub struct WSPARER {
    bits: bool,
}
impl WSPARER {
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
pub struct RSPARER {
    bits: bool,
}
impl RSPARER {
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
pub struct EDGECTRLR {
    bits: bool,
}
impl EDGECTRLR {
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
pub struct RBEDGER {
    bits: bool,
}
impl RBEDGER {
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
pub struct DTOCYCR {
    bits: u8,
}
impl DTOCYCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DTOMUL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOMULR {
    #[doc = "DTOCYC"]
    X1,
    #[doc = "DTOCYC x 16"]
    X16,
    #[doc = "DTOCYC x 128"]
    X128,
    #[doc = "DTOCYC x 256"]
    X256,
    #[doc = "DTOCYC x 1024"]
    X1024,
    #[doc = "DTOCYC x 4096"]
    X4096,
    #[doc = "DTOCYC x 65536"]
    X65536,
    #[doc = "DTOCYC x 1048576"]
    X1048576,
}
impl DTOMULR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTOMULR::X1 => 0,
            DTOMULR::X16 => 1,
            DTOMULR::X128 => 2,
            DTOMULR::X256 => 3,
            DTOMULR::X1024 => 4,
            DTOMULR::X4096 => 5,
            DTOMULR::X65536 => 6,
            DTOMULR::X1048576 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTOMULR {
        match value {
            0 => DTOMULR::X1,
            1 => DTOMULR::X16,
            2 => DTOMULR::X128,
            3 => DTOMULR::X256,
            4 => DTOMULR::X1024,
            5 => DTOMULR::X4096,
            6 => DTOMULR::X65536,
            7 => DTOMULR::X1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline]
    pub fn is_x1(&self) -> bool {
        *self == DTOMULR::X1
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline]
    pub fn is_x16(&self) -> bool {
        *self == DTOMULR::X16
    }
    #[doc = "Checks if the value of the field is `X128`"]
    #[inline]
    pub fn is_x128(&self) -> bool {
        *self == DTOMULR::X128
    }
    #[doc = "Checks if the value of the field is `X256`"]
    #[inline]
    pub fn is_x256(&self) -> bool {
        *self == DTOMULR::X256
    }
    #[doc = "Checks if the value of the field is `X1024`"]
    #[inline]
    pub fn is_x1024(&self) -> bool {
        *self == DTOMULR::X1024
    }
    #[doc = "Checks if the value of the field is `X4096`"]
    #[inline]
    pub fn is_x4096(&self) -> bool {
        *self == DTOMULR::X4096
    }
    #[doc = "Checks if the value of the field is `X65536`"]
    #[inline]
    pub fn is_x65536(&self) -> bool {
        *self == DTOMULR::X65536
    }
    #[doc = "Checks if the value of the field is `X1048576`"]
    #[inline]
    pub fn is_x1048576(&self) -> bool {
        *self == DTOMULR::X1048576
    }
}
#[doc = "Values that can be written to the field `PAGESIZE`"]
pub enum PAGESIZEW {
    #[doc = "Main area 512 Bytes"]
    PS512,
    #[doc = "Main area 1024 Bytes"]
    PS1024,
    #[doc = "Main area 2048 Bytes"]
    PS2048,
    #[doc = "Main area 4096 Bytes"]
    PS4096,
}
impl PAGESIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PAGESIZEW::PS512 => 0,
            PAGESIZEW::PS1024 => 1,
            PAGESIZEW::PS2048 => 2,
            PAGESIZEW::PS4096 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PAGESIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PAGESIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PAGESIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Main area 512 Bytes"]
    #[inline]
    pub fn ps512(self) -> &'a mut W {
        self.variant(PAGESIZEW::PS512)
    }
    #[doc = "Main area 1024 Bytes"]
    #[inline]
    pub fn ps1024(self) -> &'a mut W {
        self.variant(PAGESIZEW::PS1024)
    }
    #[doc = "Main area 2048 Bytes"]
    #[inline]
    pub fn ps2048(self) -> &'a mut W {
        self.variant(PAGESIZEW::PS2048)
    }
    #[doc = "Main area 4096 Bytes"]
    #[inline]
    pub fn ps4096(self) -> &'a mut W {
        self.variant(PAGESIZEW::PS4096)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WSPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _WSPAREW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSPAREW<'a> {
    w: &'a mut W,
}
impl<'a> _RSPAREW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EDGECTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGECTRLW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RBEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _RBEDGEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTOCYCW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOCYCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTOMUL`"]
pub enum DTOMULW {
    #[doc = "DTOCYC"]
    X1,
    #[doc = "DTOCYC x 16"]
    X16,
    #[doc = "DTOCYC x 128"]
    X128,
    #[doc = "DTOCYC x 256"]
    X256,
    #[doc = "DTOCYC x 1024"]
    X1024,
    #[doc = "DTOCYC x 4096"]
    X4096,
    #[doc = "DTOCYC x 65536"]
    X65536,
    #[doc = "DTOCYC x 1048576"]
    X1048576,
}
impl DTOMULW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTOMULW::X1 => 0,
            DTOMULW::X16 => 1,
            DTOMULW::X128 => 2,
            DTOMULW::X256 => 3,
            DTOMULW::X1024 => 4,
            DTOMULW::X4096 => 5,
            DTOMULW::X65536 => 6,
            DTOMULW::X1048576 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTOMULW<'a> {
    w: &'a mut W,
}
impl<'a> _DTOMULW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTOMULW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "DTOCYC"]
    #[inline]
    pub fn x1(self) -> &'a mut W {
        self.variant(DTOMULW::X1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline]
    pub fn x16(self) -> &'a mut W {
        self.variant(DTOMULW::X16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline]
    pub fn x128(self) -> &'a mut W {
        self.variant(DTOMULW::X128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline]
    pub fn x256(self) -> &'a mut W {
        self.variant(DTOMULW::X256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline]
    pub fn x1024(self) -> &'a mut W {
        self.variant(DTOMULW::X1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline]
    pub fn x4096(self) -> &'a mut W {
        self.variant(DTOMULW::X4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline]
    pub fn x65536(self) -> &'a mut W {
        self.variant(DTOMULW::X65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline]
    pub fn x1048576(self) -> &'a mut W {
        self.variant(DTOMULW::X1048576)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:1 - Page Size of the NAND Flash Device"]
    #[inline]
    pub fn pagesize(&self) -> PAGESIZER {
        PAGESIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline]
    pub fn wspare(&self) -> WSPARER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WSPARER { bits }
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline]
    pub fn rspare(&self) -> RSPARER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSPARER { bits }
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline]
    pub fn edgectrl(&self) -> EDGECTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EDGECTRLR { bits }
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline]
    pub fn rbedge(&self) -> RBEDGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RBEDGER { bits }
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline]
    pub fn dtocyc(&self) -> DTOCYCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTOCYCR { bits }
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline]
    pub fn dtomul(&self) -> DTOMULR {
        DTOMULR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - Page Size of the NAND Flash Device"]
    #[inline]
    pub fn pagesize(&mut self) -> _PAGESIZEW {
        _PAGESIZEW { w: self }
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline]
    pub fn wspare(&mut self) -> _WSPAREW {
        _WSPAREW { w: self }
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline]
    pub fn rspare(&mut self) -> _RSPAREW {
        _RSPAREW { w: self }
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline]
    pub fn edgectrl(&mut self) -> _EDGECTRLW {
        _EDGECTRLW { w: self }
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline]
    pub fn rbedge(&mut self) -> _RBEDGEW {
        _RBEDGEW { w: self }
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline]
    pub fn dtocyc(&mut self) -> _DTOCYCW {
        _DTOCYCW { w: self }
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline]
    pub fn dtomul(&mut self) -> _DTOMULW {
        _DTOMULW { w: self }
    }
}
