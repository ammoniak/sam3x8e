#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLA2 {
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
#[doc = r" Value of the field"]
pub struct BTSIZER {
    bits: u16,
}
impl BTSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SCSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCSIZER {
    #[doc = "1 data transferred"]
    CHK_1,
    #[doc = "4 data transferred"]
    CHK_4,
    #[doc = "8 data transferred"]
    CHK_8,
    #[doc = "16 data transferred"]
    CHK_16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCSIZER::CHK_1 => 0,
            SCSIZER::CHK_4 => 1,
            SCSIZER::CHK_8 => 2,
            SCSIZER::CHK_16 => 3,
            SCSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCSIZER {
        match value {
            0 => SCSIZER::CHK_1,
            1 => SCSIZER::CHK_4,
            2 => SCSIZER::CHK_8,
            3 => SCSIZER::CHK_16,
            i => SCSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline]
    pub fn is_chk_1(&self) -> bool {
        *self == SCSIZER::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline]
    pub fn is_chk_4(&self) -> bool {
        *self == SCSIZER::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline]
    pub fn is_chk_8(&self) -> bool {
        *self == SCSIZER::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline]
    pub fn is_chk_16(&self) -> bool {
        *self == SCSIZER::CHK_16
    }
}
#[doc = "Possible values of the field `DCSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCSIZER {
    #[doc = "1 data transferred"]
    CHK_1,
    #[doc = "4 data transferred"]
    CHK_4,
    #[doc = "8 data transferred"]
    CHK_8,
    #[doc = "16 data transferred"]
    CHK_16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCSIZER::CHK_1 => 0,
            DCSIZER::CHK_4 => 1,
            DCSIZER::CHK_8 => 2,
            DCSIZER::CHK_16 => 3,
            DCSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCSIZER {
        match value {
            0 => DCSIZER::CHK_1,
            1 => DCSIZER::CHK_4,
            2 => DCSIZER::CHK_8,
            3 => DCSIZER::CHK_16,
            i => DCSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline]
    pub fn is_chk_1(&self) -> bool {
        *self == DCSIZER::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline]
    pub fn is_chk_4(&self) -> bool {
        *self == DCSIZER::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline]
    pub fn is_chk_8(&self) -> bool {
        *self == DCSIZER::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline]
    pub fn is_chk_16(&self) -> bool {
        *self == DCSIZER::CHK_16
    }
}
#[doc = "Possible values of the field `SRC_WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_WIDTHR {
    #[doc = "the transfer size is set to 8-bit width"]
    BYTE,
    #[doc = "the transfer size is set to 16-bit width"]
    HALF_WORD,
    #[doc = "the transfer size is set to 32-bit width"]
    WORD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRC_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC_WIDTHR::BYTE => 0,
            SRC_WIDTHR::HALF_WORD => 1,
            SRC_WIDTHR::WORD => 2,
            SRC_WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC_WIDTHR {
        match value {
            0 => SRC_WIDTHR::BYTE,
            1 => SRC_WIDTHR::HALF_WORD,
            2 => SRC_WIDTHR::WORD,
            i => SRC_WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == SRC_WIDTHR::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline]
    pub fn is_half_word(&self) -> bool {
        *self == SRC_WIDTHR::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline]
    pub fn is_word(&self) -> bool {
        *self == SRC_WIDTHR::WORD
    }
}
#[doc = "Possible values of the field `DST_WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_WIDTHR {
    #[doc = "the transfer size is set to 8-bit width"]
    BYTE,
    #[doc = "the transfer size is set to 16-bit width"]
    HALF_WORD,
    #[doc = "the transfer size is set to 32-bit width"]
    WORD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DST_WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DST_WIDTHR::BYTE => 0,
            DST_WIDTHR::HALF_WORD => 1,
            DST_WIDTHR::WORD => 2,
            DST_WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DST_WIDTHR {
        match value {
            0 => DST_WIDTHR::BYTE,
            1 => DST_WIDTHR::HALF_WORD,
            2 => DST_WIDTHR::WORD,
            i => DST_WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == DST_WIDTHR::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline]
    pub fn is_half_word(&self) -> bool {
        *self == DST_WIDTHR::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline]
    pub fn is_word(&self) -> bool {
        *self == DST_WIDTHR::WORD
    }
}
#[doc = r" Value of the field"]
pub struct DONER {
    bits: bool,
}
impl DONER {
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
#[doc = r" Proxy"]
pub struct _BTSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _BTSIZEW<'a> {
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
#[doc = "Values that can be written to the field `SCSIZE`"]
pub enum SCSIZEW {
    #[doc = "1 data transferred"]
    CHK_1,
    #[doc = "4 data transferred"]
    CHK_4,
    #[doc = "8 data transferred"]
    CHK_8,
    #[doc = "16 data transferred"]
    CHK_16,
}
impl SCSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCSIZEW::CHK_1 => 0,
            SCSIZEW::CHK_4 => 1,
            SCSIZEW::CHK_8 => 2,
            SCSIZEW::CHK_16 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _SCSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data transferred"]
    #[inline]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(SCSIZEW::CHK_1)
    }
    #[doc = "4 data transferred"]
    #[inline]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(SCSIZEW::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(SCSIZEW::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(SCSIZEW::CHK_16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCSIZE`"]
pub enum DCSIZEW {
    #[doc = "1 data transferred"]
    CHK_1,
    #[doc = "4 data transferred"]
    CHK_4,
    #[doc = "8 data transferred"]
    CHK_8,
    #[doc = "16 data transferred"]
    CHK_16,
}
impl DCSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCSIZEW::CHK_1 => 0,
            DCSIZEW::CHK_4 => 1,
            DCSIZEW::CHK_8 => 2,
            DCSIZEW::CHK_16 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DCSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data transferred"]
    #[inline]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(DCSIZEW::CHK_1)
    }
    #[doc = "4 data transferred"]
    #[inline]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(DCSIZEW::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(DCSIZEW::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(DCSIZEW::CHK_16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC_WIDTH`"]
pub enum SRC_WIDTHW {
    #[doc = "the transfer size is set to 8-bit width"]
    BYTE,
    #[doc = "the transfer size is set to 16-bit width"]
    HALF_WORD,
    #[doc = "the transfer size is set to 32-bit width"]
    WORD,
}
impl SRC_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC_WIDTHW::BYTE => 0,
            SRC_WIDTHW::HALF_WORD => 1,
            SRC_WIDTHW::WORD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(SRC_WIDTHW::BYTE)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SRC_WIDTHW::HALF_WORD)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline]
    pub fn word(self) -> &'a mut W {
        self.variant(SRC_WIDTHW::WORD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DST_WIDTH`"]
pub enum DST_WIDTHW {
    #[doc = "the transfer size is set to 8-bit width"]
    BYTE,
    #[doc = "the transfer size is set to 16-bit width"]
    HALF_WORD,
    #[doc = "the transfer size is set to 32-bit width"]
    WORD,
}
impl DST_WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DST_WIDTHW::BYTE => 0,
            DST_WIDTHW::HALF_WORD => 1,
            DST_WIDTHW::WORD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DST_WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(DST_WIDTHW::BYTE)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline]
    pub fn half_word(self) -> &'a mut W {
        self.variant(DST_WIDTHW::HALF_WORD)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline]
    pub fn word(self) -> &'a mut W {
        self.variant(DST_WIDTHW::WORD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _DONEW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline]
    pub fn btsize(&self) -> BTSIZER {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BTSIZER { bits }
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline]
    pub fn scsize(&self) -> SCSIZER {
        SCSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline]
    pub fn dcsize(&self) -> DCSIZER {
        DCSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline]
    pub fn src_width(&self) -> SRC_WIDTHR {
        SRC_WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline]
    pub fn dst_width(&self) -> DST_WIDTHR {
        DST_WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline]
    pub fn done(&self) -> DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DONER { bits }
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
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline]
    pub fn btsize(&mut self) -> _BTSIZEW {
        _BTSIZEW { w: self }
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline]
    pub fn scsize(&mut self) -> _SCSIZEW {
        _SCSIZEW { w: self }
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline]
    pub fn dcsize(&mut self) -> _DCSIZEW {
        _DCSIZEW { w: self }
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline]
    pub fn src_width(&mut self) -> _SRC_WIDTHW {
        _SRC_WIDTHW { w: self }
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline]
    pub fn dst_width(&mut self) -> _DST_WIDTHW {
        _DST_WIDTHW { w: self }
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline]
    pub fn done(&mut self) -> _DONEW {
        _DONEW { w: self }
    }
}
