#[doc = "Reader of register GCFG"]
pub type R = crate::R<u32, super::GCFG>;
#[doc = "Writer for register GCFG"]
pub type W = crate::W<u32, super::GCFG>;
#[doc = "Register GCFG `reset()`'s with value 0x10"]
impl crate::ResetValue for super::GCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Arbiter Configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARB_CFG_A {
    #[doc = "0: Fixed priority arbiter (see \"Basic Definitions\" )"]
    FIXED = 0,
    #[doc = "1: Modified round robin arbiter."]
    ROUND_ROBIN = 1,
}
impl From<ARB_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_CFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ARB_CFG`"]
pub type ARB_CFG_R = crate::R<bool, ARB_CFG_A>;
impl ARB_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_CFG_A {
        match self.bits {
            false => ARB_CFG_A::FIXED,
            true => ARB_CFG_A::ROUND_ROBIN,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == ARB_CFG_A::FIXED
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN`"]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        *self == ARB_CFG_A::ROUND_ROBIN
    }
}
#[doc = "Write proxy for field `ARB_CFG`"]
pub struct ARB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARB_CFG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Fixed priority arbiter (see \"Basic Definitions\" )"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(ARB_CFG_A::FIXED)
    }
    #[doc = "Modified round robin arbiter."]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(ARB_CFG_A::ROUND_ROBIN)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline(always)]
    pub fn arb_cfg(&self) -> ARB_CFG_R {
        ARB_CFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline(always)]
    pub fn arb_cfg(&mut self) -> ARB_CFG_W {
        ARB_CFG_W { w: self }
    }
}
