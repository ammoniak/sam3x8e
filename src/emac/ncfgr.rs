#[doc = "Reader of register NCFGR"]
pub type R = crate::R<u32, super::NCFGR>;
#[doc = "Writer for register NCFGR"]
pub type W = crate::W<u32, super::NCFGR>;
#[doc = "Register NCFGR `reset()`'s with value 0x0800"]
impl crate::ResetValue for super::NCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0800
    }
}
#[doc = "Reader of field `SPD`"]
pub type SPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD`"]
pub struct SPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `FD`"]
pub type FD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FD`"]
pub struct FD_W<'a> {
    w: &'a mut W,
}
impl<'a> FD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `JFRAME`"]
pub type JFRAME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JFRAME`"]
pub struct JFRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> JFRAME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CAF`"]
pub type CAF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAF`"]
pub struct CAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CAF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `NBC`"]
pub type NBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NBC`"]
pub struct NBC_W<'a> {
    w: &'a mut W,
}
impl<'a> NBC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MTI`"]
pub type MTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MTI`"]
pub struct MTI_W<'a> {
    w: &'a mut W,
}
impl<'a> MTI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `UNI`"]
pub type UNI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNI`"]
pub struct UNI_W<'a> {
    w: &'a mut W,
}
impl<'a> UNI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `BIG`"]
pub type BIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIG`"]
pub struct BIG_W<'a> {
    w: &'a mut W,
}
impl<'a> BIG_W<'a> {
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
#[doc = "MDC clock divider\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLK_A {
    #[doc = "0: MCK divided by 8 (MCK up to 20 MHz)."]
    MCK_8,
    #[doc = "1: MCK divided by 16 (MCK up to 40 MHz)."]
    MCK_16,
    #[doc = "2: MCK divided by 32 (MCK up to 80 MHz)."]
    MCK_32,
    #[doc = "3: MCK divided by 64 (MCK up to 160 MHz)."]
    MCK_64,
}
impl From<CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_A) -> Self {
        match variant {
            CLK_A::MCK_8 => 0,
            CLK_A::MCK_16 => 1,
            CLK_A::MCK_32 => 2,
            CLK_A::MCK_64 => 3,
        }
    }
}
#[doc = "Reader of field `CLK`"]
pub type CLK_R = crate::R<u8, CLK_A>;
impl CLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_A {
        match self.bits {
            0 => CLK_A::MCK_8,
            1 => CLK_A::MCK_16,
            2 => CLK_A::MCK_32,
            3 => CLK_A::MCK_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MCK_8`"]
    #[inline(always)]
    pub fn is_mck_8(&self) -> bool {
        *self == CLK_A::MCK_8
    }
    #[doc = "Checks if the value of the field is `MCK_16`"]
    #[inline(always)]
    pub fn is_mck_16(&self) -> bool {
        *self == CLK_A::MCK_16
    }
    #[doc = "Checks if the value of the field is `MCK_32`"]
    #[inline(always)]
    pub fn is_mck_32(&self) -> bool {
        *self == CLK_A::MCK_32
    }
    #[doc = "Checks if the value of the field is `MCK_64`"]
    #[inline(always)]
    pub fn is_mck_64(&self) -> bool {
        *self == CLK_A::MCK_64
    }
}
#[doc = "Write proxy for field `CLK`"]
pub struct CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)."]
    #[inline(always)]
    pub fn mck_8(self) -> &'a mut W {
        self.variant(CLK_A::MCK_8)
    }
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)."]
    #[inline(always)]
    pub fn mck_16(self) -> &'a mut W {
        self.variant(CLK_A::MCK_16)
    }
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)."]
    #[inline(always)]
    pub fn mck_32(self) -> &'a mut W {
        self.variant(CLK_A::MCK_32)
    }
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)."]
    #[inline(always)]
    pub fn mck_64(self) -> &'a mut W {
        self.variant(CLK_A::MCK_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `RTY`"]
pub type RTY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTY`"]
pub struct RTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTY_W<'a> {
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
#[doc = "Reader of field `PAE`"]
pub type PAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAE`"]
pub struct PAE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAE_W<'a> {
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
#[doc = "Receive Buffer Offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBOF_A {
    #[doc = "0: No offset from start of receive buffer."]
    OFFSET_0,
    #[doc = "1: One-byte offset from start of receive buffer."]
    OFFSET_1,
    #[doc = "2: Two-byte offset from start of receive buffer."]
    OFFSET_2,
    #[doc = "3: Three-byte offset from start of receive buffer."]
    OFFSET_3,
}
impl From<RBOF_A> for u8 {
    #[inline(always)]
    fn from(variant: RBOF_A) -> Self {
        match variant {
            RBOF_A::OFFSET_0 => 0,
            RBOF_A::OFFSET_1 => 1,
            RBOF_A::OFFSET_2 => 2,
            RBOF_A::OFFSET_3 => 3,
        }
    }
}
#[doc = "Reader of field `RBOF`"]
pub type RBOF_R = crate::R<u8, RBOF_A>;
impl RBOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBOF_A {
        match self.bits {
            0 => RBOF_A::OFFSET_0,
            1 => RBOF_A::OFFSET_1,
            2 => RBOF_A::OFFSET_2,
            3 => RBOF_A::OFFSET_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET_0`"]
    #[inline(always)]
    pub fn is_offset_0(&self) -> bool {
        *self == RBOF_A::OFFSET_0
    }
    #[doc = "Checks if the value of the field is `OFFSET_1`"]
    #[inline(always)]
    pub fn is_offset_1(&self) -> bool {
        *self == RBOF_A::OFFSET_1
    }
    #[doc = "Checks if the value of the field is `OFFSET_2`"]
    #[inline(always)]
    pub fn is_offset_2(&self) -> bool {
        *self == RBOF_A::OFFSET_2
    }
    #[doc = "Checks if the value of the field is `OFFSET_3`"]
    #[inline(always)]
    pub fn is_offset_3(&self) -> bool {
        *self == RBOF_A::OFFSET_3
    }
}
#[doc = "Write proxy for field `RBOF`"]
pub struct RBOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RBOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBOF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_0(self) -> &'a mut W {
        self.variant(RBOF_A::OFFSET_0)
    }
    #[doc = "One-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_1(self) -> &'a mut W {
        self.variant(RBOF_A::OFFSET_1)
    }
    #[doc = "Two-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_2(self) -> &'a mut W {
        self.variant(RBOF_A::OFFSET_2)
    }
    #[doc = "Three-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_3(self) -> &'a mut W {
        self.variant(RBOF_A::OFFSET_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `RLCE`"]
pub type RLCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RLCE`"]
pub struct RLCE_W<'a> {
    w: &'a mut W,
}
impl<'a> RLCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DRFCS`"]
pub type DRFCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRFCS`"]
pub struct DRFCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRFCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `EFRHD`"]
pub type EFRHD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFRHD`"]
pub struct EFRHD_W<'a> {
    w: &'a mut W,
}
impl<'a> EFRHD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `IRXFCS`"]
pub type IRXFCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRXFCS`"]
pub struct IRXFCS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRXFCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FD_R {
        FD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline(always)]
    pub fn jframe(&self) -> JFRAME_R {
        JFRAME_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&self) -> NBC_R {
        NBC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mti(&self) -> MTI_R {
        MTI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn uni(&self) -> UNI_R {
        UNI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline(always)]
    pub fn big(&self) -> BIG_R {
        BIG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn rty(&self) -> RTY_R {
        RTY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pae(&self) -> PAE_R {
        PAE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rbof(&self) -> RBOF_R {
        RBOF_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline(always)]
    pub fn rlce(&self) -> RLCE_R {
        RLCE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline(always)]
    pub fn drfcs(&self) -> DRFCS_R {
        DRFCS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn efrhd(&self) -> EFRHD_R {
        EFRHD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&self) -> IRXFCS_R {
        IRXFCS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&mut self) -> SPD_W {
        SPD_W { w: self }
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&mut self) -> FD_W {
        FD_W { w: self }
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline(always)]
    pub fn jframe(&mut self) -> JFRAME_W {
        JFRAME_W { w: self }
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&mut self) -> CAF_W {
        CAF_W { w: self }
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&mut self) -> NBC_W {
        NBC_W { w: self }
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mti(&mut self) -> MTI_W {
        MTI_W { w: self }
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn uni(&mut self) -> UNI_W {
        UNI_W { w: self }
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline(always)]
    pub fn big(&mut self) -> BIG_W {
        BIG_W { w: self }
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline(always)]
    pub fn clk(&mut self) -> CLK_W {
        CLK_W { w: self }
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn rty(&mut self) -> RTY_W {
        RTY_W { w: self }
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pae(&mut self) -> PAE_W {
        PAE_W { w: self }
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rbof(&mut self) -> RBOF_W {
        RBOF_W { w: self }
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline(always)]
    pub fn rlce(&mut self) -> RLCE_W {
        RLCE_W { w: self }
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline(always)]
    pub fn drfcs(&mut self) -> DRFCS_W {
        DRFCS_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn efrhd(&mut self) -> EFRHD_W {
        EFRHD_W { w: self }
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&mut self) -> IRXFCS_W {
        IRXFCS_W { w: self }
    }
}
