#[doc = "Reader of register MODE5"]
pub type R = crate::R<u32, super::MODE5>;
#[doc = "Writer for register MODE5"]
pub type W = crate::W<u32, super::MODE5>;
#[doc = "Register MODE5 `reset()`'s with value 0x1000_0003"]
impl crate::ResetValue for super::MODE5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1000_0003
    }
}
#[doc = "Selection of the Control Signal for Read Operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_MODE_A {
    #[doc = "0: The Read operation is controlled by the NCS signal."]
    NCS_CTRL,
    #[doc = "1: The Read operation is controlled by the NRD signal."]
    NRD_CTRL,
}
impl From<READ_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_MODE_A) -> Self {
        match variant {
            READ_MODE_A::NCS_CTRL => false,
            READ_MODE_A::NRD_CTRL => true,
        }
    }
}
#[doc = "Reader of field `READ_MODE`"]
pub type READ_MODE_R = crate::R<bool, READ_MODE_A>;
impl READ_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_MODE_A {
        match self.bits {
            false => READ_MODE_A::NCS_CTRL,
            true => READ_MODE_A::NRD_CTRL,
        }
    }
    #[doc = "Checks if the value of the field is `NCS_CTRL`"]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == READ_MODE_A::NCS_CTRL
    }
    #[doc = "Checks if the value of the field is `NRD_CTRL`"]
    #[inline(always)]
    pub fn is_nrd_ctrl(&self) -> bool {
        *self == READ_MODE_A::NRD_CTRL
    }
}
#[doc = "Write proxy for field `READ_MODE`"]
pub struct READ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Read operation is controlled by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut W {
        self.variant(READ_MODE_A::NCS_CTRL)
    }
    #[doc = "The Read operation is controlled by the NRD signal."]
    #[inline(always)]
    pub fn nrd_ctrl(self) -> &'a mut W {
        self.variant(READ_MODE_A::NRD_CTRL)
    }
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
#[doc = "Selection of the Control Signal for Write Operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_MODE_A {
    #[doc = "0: The Write operation is controller by the NCS signal."]
    NCS_CTRL,
    #[doc = "1: The Write operation is controlled by the NWE signal."]
    NWE_CTRL,
}
impl From<WRITE_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_MODE_A) -> Self {
        match variant {
            WRITE_MODE_A::NCS_CTRL => false,
            WRITE_MODE_A::NWE_CTRL => true,
        }
    }
}
#[doc = "Reader of field `WRITE_MODE`"]
pub type WRITE_MODE_R = crate::R<bool, WRITE_MODE_A>;
impl WRITE_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_MODE_A {
        match self.bits {
            false => WRITE_MODE_A::NCS_CTRL,
            true => WRITE_MODE_A::NWE_CTRL,
        }
    }
    #[doc = "Checks if the value of the field is `NCS_CTRL`"]
    #[inline(always)]
    pub fn is_ncs_ctrl(&self) -> bool {
        *self == WRITE_MODE_A::NCS_CTRL
    }
    #[doc = "Checks if the value of the field is `NWE_CTRL`"]
    #[inline(always)]
    pub fn is_nwe_ctrl(&self) -> bool {
        *self == WRITE_MODE_A::NWE_CTRL
    }
}
#[doc = "Write proxy for field `WRITE_MODE`"]
pub struct WRITE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The Write operation is controller by the NCS signal."]
    #[inline(always)]
    pub fn ncs_ctrl(self) -> &'a mut W {
        self.variant(WRITE_MODE_A::NCS_CTRL)
    }
    #[doc = "The Write operation is controlled by the NWE signal."]
    #[inline(always)]
    pub fn nwe_ctrl(self) -> &'a mut W {
        self.variant(WRITE_MODE_A::NWE_CTRL)
    }
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
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXNW_MODE_A {
    #[doc = "0: Disabled"]
    DISABLED,
    #[doc = "2: Frozen Mode"]
    FROZEN,
    #[doc = "3: Ready Mode"]
    READY,
}
impl From<EXNW_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXNW_MODE_A) -> Self {
        match variant {
            EXNW_MODE_A::DISABLED => 0,
            EXNW_MODE_A::FROZEN => 2,
            EXNW_MODE_A::READY => 3,
        }
    }
}
#[doc = "Reader of field `EXNW_MODE`"]
pub type EXNW_MODE_R = crate::R<u8, EXNW_MODE_A>;
impl EXNW_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXNW_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXNW_MODE_A::DISABLED),
            2 => Val(EXNW_MODE_A::FROZEN),
            3 => Val(EXNW_MODE_A::READY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXNW_MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == EXNW_MODE_A::FROZEN
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXNW_MODE_A::READY
    }
}
#[doc = "Write proxy for field `EXNW_MODE`"]
pub struct EXNW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXNW_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXNW_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::DISABLED)
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::FROZEN)
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(EXNW_MODE_A::READY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `BAT`"]
pub type BAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BAT`"]
pub struct BAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAT_W<'a> {
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
#[doc = "Data Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBW_A {
    #[doc = "0: 8-bit bus"]
    BIT_8,
    #[doc = "1: 16-bit bus"]
    BIT_16,
}
impl From<DBW_A> for bool {
    #[inline(always)]
    fn from(variant: DBW_A) -> Self {
        match variant {
            DBW_A::BIT_8 => false,
            DBW_A::BIT_16 => true,
        }
    }
}
#[doc = "Reader of field `DBW`"]
pub type DBW_R = crate::R<bool, DBW_A>;
impl DBW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBW_A {
        match self.bits {
            false => DBW_A::BIT_8,
            true => DBW_A::BIT_16,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_8`"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == DBW_A::BIT_8
    }
    #[doc = "Checks if the value of the field is `BIT_16`"]
    #[inline(always)]
    pub fn is_bit_16(&self) -> bool {
        *self == DBW_A::BIT_16
    }
}
#[doc = "Write proxy for field `DBW`"]
pub struct DBW_W<'a> {
    w: &'a mut W,
}
impl<'a> DBW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "8-bit bus"]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut W {
        self.variant(DBW_A::BIT_8)
    }
    #[doc = "16-bit bus"]
    #[inline(always)]
    pub fn bit_16(self) -> &'a mut W {
        self.variant(DBW_A::BIT_16)
    }
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
#[doc = "Reader of field `TDF_CYCLES`"]
pub type TDF_CYCLES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDF_CYCLES`"]
pub struct TDF_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> TDF_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TDF_MODE`"]
pub type TDF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDF_MODE`"]
pub struct TDF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
    #[inline(always)]
    pub fn write_mode(&self) -> WRITE_MODE_R {
        WRITE_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> EXNW_MODE_R {
        EXNW_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&self) -> BAT_R {
        BAT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TDF_CYCLES_R {
        TDF_CYCLES_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TDF_MODE_R {
        TDF_MODE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selection of the Control Signal for Read Operation"]
    #[inline(always)]
    pub fn read_mode(&mut self) -> READ_MODE_W {
        READ_MODE_W { w: self }
    }
    #[doc = "Bit 1 - Selection of the Control Signal for Write Operation"]
    #[inline(always)]
    pub fn write_mode(&mut self) -> WRITE_MODE_W {
        WRITE_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&mut self) -> EXNW_MODE_W {
        EXNW_MODE_W { w: self }
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&mut self) -> BAT_W {
        BAT_W { w: self }
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DBW_W {
        DBW_W { w: self }
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&mut self) -> TDF_CYCLES_W {
        TDF_CYCLES_W { w: self }
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&mut self) -> TDF_MODE_W {
        TDF_MODE_W { w: self }
    }
}
