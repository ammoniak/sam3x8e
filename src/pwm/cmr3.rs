#[doc = "Reader of register CMR3"]
pub type R = crate::R<u32, super::CMR3>;
#[doc = "Writer for register CMR3"]
pub type W = crate::W<u32, super::CMR3>;
#[doc = "Register CMR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CMR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPRE_A {
    #[doc = "0: Master clock"]
    MCK,
    #[doc = "1: Master clock/2"]
    MCK_DIV_2,
    #[doc = "2: Master clock/4"]
    MCK_DIV_4,
    #[doc = "3: Master clock/8"]
    MCK_DIV_8,
    #[doc = "4: Master clock/16"]
    MCK_DIV_16,
    #[doc = "5: Master clock/32"]
    MCK_DIV_32,
    #[doc = "6: Master clock/64"]
    MCK_DIV_64,
    #[doc = "7: Master clock/128"]
    MCK_DIV_128,
    #[doc = "8: Master clock/256"]
    MCK_DIV_256,
    #[doc = "9: Master clock/512"]
    MCK_DIV_512,
    #[doc = "10: Master clock/1024"]
    MCK_DIV_1024,
    #[doc = "11: Clock A"]
    CLKA,
    #[doc = "12: Clock B"]
    CLKB,
}
impl From<CPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPRE_A) -> Self {
        match variant {
            CPRE_A::MCK => 0,
            CPRE_A::MCK_DIV_2 => 1,
            CPRE_A::MCK_DIV_4 => 2,
            CPRE_A::MCK_DIV_8 => 3,
            CPRE_A::MCK_DIV_16 => 4,
            CPRE_A::MCK_DIV_32 => 5,
            CPRE_A::MCK_DIV_64 => 6,
            CPRE_A::MCK_DIV_128 => 7,
            CPRE_A::MCK_DIV_256 => 8,
            CPRE_A::MCK_DIV_512 => 9,
            CPRE_A::MCK_DIV_1024 => 10,
            CPRE_A::CLKA => 11,
            CPRE_A::CLKB => 12,
        }
    }
}
#[doc = "Reader of field `CPRE`"]
pub type CPRE_R = crate::R<u8, CPRE_A>;
impl CPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPRE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CPRE_A::MCK),
            1 => Val(CPRE_A::MCK_DIV_2),
            2 => Val(CPRE_A::MCK_DIV_4),
            3 => Val(CPRE_A::MCK_DIV_8),
            4 => Val(CPRE_A::MCK_DIV_16),
            5 => Val(CPRE_A::MCK_DIV_32),
            6 => Val(CPRE_A::MCK_DIV_64),
            7 => Val(CPRE_A::MCK_DIV_128),
            8 => Val(CPRE_A::MCK_DIV_256),
            9 => Val(CPRE_A::MCK_DIV_512),
            10 => Val(CPRE_A::MCK_DIV_1024),
            11 => Val(CPRE_A::CLKA),
            12 => Val(CPRE_A::CLKB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CPRE_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_2`"]
    #[inline(always)]
    pub fn is_mck_div_2(&self) -> bool {
        *self == CPRE_A::MCK_DIV_2
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_4`"]
    #[inline(always)]
    pub fn is_mck_div_4(&self) -> bool {
        *self == CPRE_A::MCK_DIV_4
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_8`"]
    #[inline(always)]
    pub fn is_mck_div_8(&self) -> bool {
        *self == CPRE_A::MCK_DIV_8
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_16`"]
    #[inline(always)]
    pub fn is_mck_div_16(&self) -> bool {
        *self == CPRE_A::MCK_DIV_16
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_32`"]
    #[inline(always)]
    pub fn is_mck_div_32(&self) -> bool {
        *self == CPRE_A::MCK_DIV_32
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_64`"]
    #[inline(always)]
    pub fn is_mck_div_64(&self) -> bool {
        *self == CPRE_A::MCK_DIV_64
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_128`"]
    #[inline(always)]
    pub fn is_mck_div_128(&self) -> bool {
        *self == CPRE_A::MCK_DIV_128
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_256`"]
    #[inline(always)]
    pub fn is_mck_div_256(&self) -> bool {
        *self == CPRE_A::MCK_DIV_256
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_512`"]
    #[inline(always)]
    pub fn is_mck_div_512(&self) -> bool {
        *self == CPRE_A::MCK_DIV_512
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_1024`"]
    #[inline(always)]
    pub fn is_mck_div_1024(&self) -> bool {
        *self == CPRE_A::MCK_DIV_1024
    }
    #[doc = "Checks if the value of the field is `CLKA`"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        *self == CPRE_A::CLKA
    }
    #[doc = "Checks if the value of the field is `CLKB`"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        *self == CPRE_A::CLKB
    }
}
#[doc = "Write proxy for field `CPRE`"]
pub struct CPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Master clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CPRE_A::MCK)
    }
    #[doc = "Master clock/2"]
    #[inline(always)]
    pub fn mck_div_2(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_2)
    }
    #[doc = "Master clock/4"]
    #[inline(always)]
    pub fn mck_div_4(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_4)
    }
    #[doc = "Master clock/8"]
    #[inline(always)]
    pub fn mck_div_8(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_8)
    }
    #[doc = "Master clock/16"]
    #[inline(always)]
    pub fn mck_div_16(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_16)
    }
    #[doc = "Master clock/32"]
    #[inline(always)]
    pub fn mck_div_32(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_32)
    }
    #[doc = "Master clock/64"]
    #[inline(always)]
    pub fn mck_div_64(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_64)
    }
    #[doc = "Master clock/128"]
    #[inline(always)]
    pub fn mck_div_128(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_128)
    }
    #[doc = "Master clock/256"]
    #[inline(always)]
    pub fn mck_div_256(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_256)
    }
    #[doc = "Master clock/512"]
    #[inline(always)]
    pub fn mck_div_512(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_512)
    }
    #[doc = "Master clock/1024"]
    #[inline(always)]
    pub fn mck_div_1024(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut W {
        self.variant(CPRE_A::CLKA)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut W {
        self.variant(CPRE_A::CLKB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CALG`"]
pub type CALG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALG`"]
pub struct CALG_W<'a> {
    w: &'a mut W,
}
impl<'a> CALG_W<'a> {
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
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
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
#[doc = "Reader of field `CES`"]
pub type CES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CES`"]
pub struct CES_W<'a> {
    w: &'a mut W,
}
impl<'a> CES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DTE`"]
pub type DTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTE`"]
pub struct DTE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTE_W<'a> {
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
#[doc = "Reader of field `DTHI`"]
pub type DTHI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTHI`"]
pub struct DTHI_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHI_W<'a> {
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
#[doc = "Reader of field `DTLI`"]
pub type DTLI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTLI`"]
pub struct DTLI_W<'a> {
    w: &'a mut W,
}
impl<'a> DTLI_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CPRE_R {
        CPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CALG_R {
        CALG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&self) -> DTHI_R {
        DTHI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&self) -> DTLI_R {
        DTLI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&mut self) -> CPRE_W {
        CPRE_W { w: self }
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&mut self) -> CALG_W {
        CALG_W { w: self }
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W {
        CES_W { w: self }
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&mut self) -> DTE_W {
        DTE_W { w: self }
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&mut self) -> DTHI_W {
        DTHI_W { w: self }
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&mut self) -> DTLI_W {
        DTLI_W { w: self }
    }
}
