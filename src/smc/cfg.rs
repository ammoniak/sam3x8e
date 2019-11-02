#[doc = "Reader of register CFG"]
pub type R = crate::R<u32, super::CFG>;
#[doc = "Writer for register CFG"]
pub type W = crate::W<u32, super::CFG>;
#[doc = "Register CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Page Size of the NAND Flash Device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PAGESIZE_A {
    #[doc = "0: Main area 512 Bytes"]
    PS512,
    #[doc = "1: Main area 1024 Bytes"]
    PS1024,
    #[doc = "2: Main area 2048 Bytes"]
    PS2048,
    #[doc = "3: Main area 4096 Bytes"]
    PS4096,
}
impl From<PAGESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PAGESIZE_A) -> Self {
        match variant {
            PAGESIZE_A::PS512 => 0,
            PAGESIZE_A::PS1024 => 1,
            PAGESIZE_A::PS2048 => 2,
            PAGESIZE_A::PS4096 => 3,
        }
    }
}
#[doc = "Reader of field `PAGESIZE`"]
pub type PAGESIZE_R = crate::R<u8, PAGESIZE_A>;
impl PAGESIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGESIZE_A {
        match self.bits {
            0 => PAGESIZE_A::PS512,
            1 => PAGESIZE_A::PS1024,
            2 => PAGESIZE_A::PS2048,
            3 => PAGESIZE_A::PS4096,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PS512`"]
    #[inline(always)]
    pub fn is_ps512(&self) -> bool {
        *self == PAGESIZE_A::PS512
    }
    #[doc = "Checks if the value of the field is `PS1024`"]
    #[inline(always)]
    pub fn is_ps1024(&self) -> bool {
        *self == PAGESIZE_A::PS1024
    }
    #[doc = "Checks if the value of the field is `PS2048`"]
    #[inline(always)]
    pub fn is_ps2048(&self) -> bool {
        *self == PAGESIZE_A::PS2048
    }
    #[doc = "Checks if the value of the field is `PS4096`"]
    #[inline(always)]
    pub fn is_ps4096(&self) -> bool {
        *self == PAGESIZE_A::PS4096
    }
}
#[doc = "Write proxy for field `PAGESIZE`"]
pub struct PAGESIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGESIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAGESIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Main area 512 Bytes"]
    #[inline(always)]
    pub fn ps512(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS512)
    }
    #[doc = "Main area 1024 Bytes"]
    #[inline(always)]
    pub fn ps1024(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS1024)
    }
    #[doc = "Main area 2048 Bytes"]
    #[inline(always)]
    pub fn ps2048(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS2048)
    }
    #[doc = "Main area 4096 Bytes"]
    #[inline(always)]
    pub fn ps4096(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PS4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `WSPARE`"]
pub type WSPARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WSPARE`"]
pub struct WSPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> WSPARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RSPARE`"]
pub type RSPARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSPARE`"]
pub struct RSPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSPARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `EDGECTRL`"]
pub type EDGECTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGECTRL`"]
pub struct EDGECTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGECTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `RBEDGE`"]
pub type RBEDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBEDGE`"]
pub struct RBEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBEDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `DTOCYC`"]
pub type DTOCYC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DTOCYC`"]
pub struct DTOCYC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOCYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Data Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTOMUL_A {
    #[doc = "0: DTOCYC"]
    X1,
    #[doc = "1: DTOCYC x 16"]
    X16,
    #[doc = "2: DTOCYC x 128"]
    X128,
    #[doc = "3: DTOCYC x 256"]
    X256,
    #[doc = "4: DTOCYC x 1024"]
    X1024,
    #[doc = "5: DTOCYC x 4096"]
    X4096,
    #[doc = "6: DTOCYC x 65536"]
    X65536,
    #[doc = "7: DTOCYC x 1048576"]
    X1048576,
}
impl From<DTOMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOMUL_A) -> Self {
        match variant {
            DTOMUL_A::X1 => 0,
            DTOMUL_A::X16 => 1,
            DTOMUL_A::X128 => 2,
            DTOMUL_A::X256 => 3,
            DTOMUL_A::X1024 => 4,
            DTOMUL_A::X4096 => 5,
            DTOMUL_A::X65536 => 6,
            DTOMUL_A::X1048576 => 7,
        }
    }
}
#[doc = "Reader of field `DTOMUL`"]
pub type DTOMUL_R = crate::R<u8, DTOMUL_A>;
impl DTOMUL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOMUL_A {
        match self.bits {
            0 => DTOMUL_A::X1,
            1 => DTOMUL_A::X16,
            2 => DTOMUL_A::X128,
            3 => DTOMUL_A::X256,
            4 => DTOMUL_A::X1024,
            5 => DTOMUL_A::X4096,
            6 => DTOMUL_A::X65536,
            7 => DTOMUL_A::X1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == DTOMUL_A::X1
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == DTOMUL_A::X16
    }
    #[doc = "Checks if the value of the field is `X128`"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == DTOMUL_A::X128
    }
    #[doc = "Checks if the value of the field is `X256`"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == DTOMUL_A::X256
    }
    #[doc = "Checks if the value of the field is `X1024`"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == DTOMUL_A::X1024
    }
    #[doc = "Checks if the value of the field is `X4096`"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == DTOMUL_A::X4096
    }
    #[doc = "Checks if the value of the field is `X65536`"]
    #[inline(always)]
    pub fn is_x65536(&self) -> bool {
        *self == DTOMUL_A::X65536
    }
    #[doc = "Checks if the value of the field is `X1048576`"]
    #[inline(always)]
    pub fn is_x1048576(&self) -> bool {
        *self == DTOMUL_A::X1048576
    }
}
#[doc = "Write proxy for field `DTOMUL`"]
pub struct DTOMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOMUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTOMUL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DTOCYC"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(DTOMUL_A::X1)
    }
    #[doc = "DTOCYC x 16"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(DTOMUL_A::X16)
    }
    #[doc = "DTOCYC x 128"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut W {
        self.variant(DTOMUL_A::X128)
    }
    #[doc = "DTOCYC x 256"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut W {
        self.variant(DTOMUL_A::X256)
    }
    #[doc = "DTOCYC x 1024"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut W {
        self.variant(DTOMUL_A::X1024)
    }
    #[doc = "DTOCYC x 4096"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut W {
        self.variant(DTOMUL_A::X4096)
    }
    #[doc = "DTOCYC x 65536"]
    #[inline(always)]
    pub fn x65536(self) -> &'a mut W {
        self.variant(DTOMUL_A::X65536)
    }
    #[doc = "DTOCYC x 1048576"]
    #[inline(always)]
    pub fn x1048576(self) -> &'a mut W {
        self.variant(DTOMUL_A::X1048576)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Page Size of the NAND Flash Device"]
    #[inline(always)]
    pub fn pagesize(&self) -> PAGESIZE_R {
        PAGESIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline(always)]
    pub fn wspare(&self) -> WSPARE_R {
        WSPARE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline(always)]
    pub fn rspare(&self) -> RSPARE_R {
        RSPARE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline(always)]
    pub fn edgectrl(&self) -> EDGECTRL_R {
        EDGECTRL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline(always)]
    pub fn rbedge(&self) -> RBEDGE_R {
        RBEDGE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&self) -> DTOCYC_R {
        DTOCYC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&self) -> DTOMUL_R {
        DTOMUL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Page Size of the NAND Flash Device"]
    #[inline(always)]
    pub fn pagesize(&mut self) -> PAGESIZE_W {
        PAGESIZE_W { w: self }
    }
    #[doc = "Bit 8 - Write Spare Area"]
    #[inline(always)]
    pub fn wspare(&mut self) -> WSPARE_W {
        WSPARE_W { w: self }
    }
    #[doc = "Bit 9 - Read Spare Area"]
    #[inline(always)]
    pub fn rspare(&mut self) -> RSPARE_W {
        RSPARE_W { w: self }
    }
    #[doc = "Bit 12 - Rising/Falling Edge Detection Control"]
    #[inline(always)]
    pub fn edgectrl(&mut self) -> EDGECTRL_W {
        EDGECTRL_W { w: self }
    }
    #[doc = "Bit 13 - Ready/Busy Signal Edge Detection"]
    #[inline(always)]
    pub fn rbedge(&mut self) -> RBEDGE_W {
        RBEDGE_W { w: self }
    }
    #[doc = "Bits 16:19 - Data Timeout Cycle Number"]
    #[inline(always)]
    pub fn dtocyc(&mut self) -> DTOCYC_W {
        DTOCYC_W { w: self }
    }
    #[doc = "Bits 20:22 - Data Timeout Multiplier"]
    #[inline(always)]
    pub fn dtomul(&mut self) -> DTOMUL_W {
        DTOMUL_W { w: self }
    }
}
