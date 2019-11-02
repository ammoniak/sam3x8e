#[doc = "Reader of register PMC_PCR"]
pub type R = crate::R<u32, super::PMC_PCR>;
#[doc = "Writer for register PMC_PCR"]
pub type W = crate::W<u32, super::PMC_PCR>;
#[doc = "Register PMC_PCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC_PCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
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
#[doc = "Divisor Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_A {
    #[doc = "0: Peripheral clock is MCK"]
    PERIPH_DIV_MCK,
    #[doc = "1: Peripheral clock is MCK/2"]
    PERIPH_DIV2_MCK,
    #[doc = "2: Peripheral clock is MCK/4"]
    PERIPH_DIV4_MCK,
}
impl From<DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: DIV_A) -> Self {
        match variant {
            DIV_A::PERIPH_DIV_MCK => 0,
            DIV_A::PERIPH_DIV2_MCK => 1,
            DIV_A::PERIPH_DIV4_MCK => 2,
        }
    }
}
#[doc = "Reader of field `DIV`"]
pub type DIV_R = crate::R<u8, DIV_A>;
impl DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIV_A::PERIPH_DIV_MCK),
            1 => Val(DIV_A::PERIPH_DIV2_MCK),
            2 => Val(DIV_A::PERIPH_DIV4_MCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV_MCK`"]
    #[inline(always)]
    pub fn is_periph_div_mck(&self) -> bool {
        *self == DIV_A::PERIPH_DIV_MCK
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV2_MCK`"]
    #[inline(always)]
    pub fn is_periph_div2_mck(&self) -> bool {
        *self == DIV_A::PERIPH_DIV2_MCK
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV4_MCK`"]
    #[inline(always)]
    pub fn is_periph_div4_mck(&self) -> bool {
        *self == DIV_A::PERIPH_DIV4_MCK
    }
}
#[doc = "Write proxy for field `DIV`"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Peripheral clock is MCK"]
    #[inline(always)]
    pub fn periph_div_mck(self) -> &'a mut W {
        self.variant(DIV_A::PERIPH_DIV_MCK)
    }
    #[doc = "Peripheral clock is MCK/2"]
    #[inline(always)]
    pub fn periph_div2_mck(self) -> &'a mut W {
        self.variant(DIV_A::PERIPH_DIV2_MCK)
    }
    #[doc = "Peripheral clock is MCK/4"]
    #[inline(always)]
    pub fn periph_div4_mck(self) -> &'a mut W {
        self.variant(DIV_A::PERIPH_DIV4_MCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
