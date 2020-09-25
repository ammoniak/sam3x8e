#[doc = "Reader of register CTRLA2"]
pub type R = crate::R<u32, super::CTRLA2>;
#[doc = "Writer for register CTRLA2"]
pub type W = crate::W<u32, super::CTRLA2>;
#[doc = "Register CTRLA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BTSIZE`"]
pub type BTSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BTSIZE`"]
pub struct BTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BTSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Source Chunk Transfer Size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SCSIZE_A {
    #[doc = "0: 1 data transferred"]
    CHK_1 = 0,
    #[doc = "1: 4 data transferred"]
    CHK_4 = 1,
    #[doc = "2: 8 data transferred"]
    CHK_8 = 2,
    #[doc = "3: 16 data transferred"]
    CHK_16 = 3,
}
impl From<SCSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SCSIZE`"]
pub type SCSIZE_R = crate::R<u8, SCSIZE_A>;
impl SCSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SCSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SCSIZE_A::CHK_1),
            1 => Val(SCSIZE_A::CHK_4),
            2 => Val(SCSIZE_A::CHK_8),
            3 => Val(SCSIZE_A::CHK_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == SCSIZE_A::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == SCSIZE_A::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == SCSIZE_A::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == SCSIZE_A::CHK_16
    }
}
#[doc = "Write proxy for field `SCSIZE`"]
pub struct SCSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(SCSIZE_A::CHK_1)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(SCSIZE_A::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(SCSIZE_A::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(SCSIZE_A::CHK_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Destination Chunk Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCSIZE_A {
    #[doc = "0: 1 data transferred"]
    CHK_1 = 0,
    #[doc = "1: 4 data transferred"]
    CHK_4 = 1,
    #[doc = "2: 8 data transferred"]
    CHK_8 = 2,
    #[doc = "3: 16 data transferred"]
    CHK_16 = 3,
}
impl From<DCSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DCSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCSIZE`"]
pub type DCSIZE_R = crate::R<u8, DCSIZE_A>;
impl DCSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DCSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DCSIZE_A::CHK_1),
            1 => Val(DCSIZE_A::CHK_4),
            2 => Val(DCSIZE_A::CHK_8),
            3 => Val(DCSIZE_A::CHK_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == DCSIZE_A::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == DCSIZE_A::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == DCSIZE_A::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == DCSIZE_A::CHK_16
    }
}
#[doc = "Write proxy for field `DCSIZE`"]
pub struct DCSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(DCSIZE_A::CHK_1)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(DCSIZE_A::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(DCSIZE_A::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(DCSIZE_A::CHK_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Transfer Width for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_WIDTH_A {
    #[doc = "0: the transfer size is set to 8-bit width"]
    BYTE = 0,
    #[doc = "1: the transfer size is set to 16-bit width"]
    HALF_WORD = 1,
    #[doc = "2: the transfer size is set to 32-bit width"]
    WORD = 2,
}
impl From<SRC_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC_WIDTH`"]
pub type SRC_WIDTH_R = crate::R<u8, SRC_WIDTH_A>;
impl SRC_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_WIDTH_A::BYTE),
            1 => Val(SRC_WIDTH_A::HALF_WORD),
            2 => Val(SRC_WIDTH_A::WORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SRC_WIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SRC_WIDTH_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SRC_WIDTH_A::WORD
    }
}
#[doc = "Write proxy for field `SRC_WIDTH`"]
pub struct SRC_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SRC_WIDTH_A::BYTE)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(SRC_WIDTH_A::HALF_WORD)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SRC_WIDTH_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Transfer Width for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DST_WIDTH_A {
    #[doc = "0: the transfer size is set to 8-bit width"]
    BYTE = 0,
    #[doc = "1: the transfer size is set to 16-bit width"]
    HALF_WORD = 1,
    #[doc = "2: the transfer size is set to 32-bit width"]
    WORD = 2,
}
impl From<DST_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DST_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DST_WIDTH`"]
pub type DST_WIDTH_R = crate::R<u8, DST_WIDTH_A>;
impl DST_WIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DST_WIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DST_WIDTH_A::BYTE),
            1 => Val(DST_WIDTH_A::HALF_WORD),
            2 => Val(DST_WIDTH_A::WORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DST_WIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == DST_WIDTH_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DST_WIDTH_A::WORD
    }
}
#[doc = "Write proxy for field `DST_WIDTH`"]
pub struct DST_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_WIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "the transfer size is set to 8-bit width"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DST_WIDTH_A::BYTE)
    }
    #[doc = "the transfer size is set to 16-bit width"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(DST_WIDTH_A::HALF_WORD)
    }
    #[doc = "the transfer size is set to 32-bit width"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DST_WIDTH_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    pub fn btsize(&self) -> BTSIZE_R {
        BTSIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline(always)]
    pub fn scsize(&self) -> SCSIZE_R {
        SCSIZE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline(always)]
    pub fn dcsize(&self) -> DCSIZE_R {
        DCSIZE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    pub fn src_width(&self) -> SRC_WIDTH_R {
        SRC_WIDTH_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    pub fn dst_width(&self) -> DST_WIDTH_R {
        DST_WIDTH_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Buffer Transfer Size"]
    #[inline(always)]
    pub fn btsize(&mut self) -> BTSIZE_W {
        BTSIZE_W { w: self }
    }
    #[doc = "Bits 16:18 - Source Chunk Transfer Size."]
    #[inline(always)]
    pub fn scsize(&mut self) -> SCSIZE_W {
        SCSIZE_W { w: self }
    }
    #[doc = "Bits 20:22 - Destination Chunk Transfer Size"]
    #[inline(always)]
    pub fn dcsize(&mut self) -> DCSIZE_W {
        DCSIZE_W { w: self }
    }
    #[doc = "Bits 24:25 - Transfer Width for the Source"]
    #[inline(always)]
    pub fn src_width(&mut self) -> SRC_WIDTH_W {
        SRC_WIDTH_W { w: self }
    }
    #[doc = "Bits 28:29 - Transfer Width for the Destination"]
    #[inline(always)]
    pub fn dst_width(&mut self) -> DST_WIDTH_W {
        DST_WIDTH_W { w: self }
    }
    #[doc = "Bit 31 - Current Descriptor Stop Command and Transfer Completed Memory Indicator"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
}
