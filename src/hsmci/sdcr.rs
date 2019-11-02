#[doc = "Reader of register SDCR"]
pub type R = crate::R<u32, super::SDCR>;
#[doc = "Writer for register SDCR"]
pub type W = crate::W<u32, super::SDCR>;
#[doc = "Register SDCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SDCard/SDIO Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCSEL_A {
    #[doc = "0: Slot A is selected."]
    SLOTA,
    #[doc = "1: SDCARD/SDIO Slot B selected"]
    SLOTB,
    #[doc = "2: -"]
    SLOTC,
    #[doc = "3: -"]
    SLOTD,
}
impl From<SDCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCSEL_A) -> Self {
        match variant {
            SDCSEL_A::SLOTA => 0,
            SDCSEL_A::SLOTB => 1,
            SDCSEL_A::SLOTC => 2,
            SDCSEL_A::SLOTD => 3,
        }
    }
}
#[doc = "Reader of field `SDCSEL`"]
pub type SDCSEL_R = crate::R<u8, SDCSEL_A>;
impl SDCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDCSEL_A {
        match self.bits {
            0 => SDCSEL_A::SLOTA,
            1 => SDCSEL_A::SLOTB,
            2 => SDCSEL_A::SLOTC,
            3 => SDCSEL_A::SLOTD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOTA`"]
    #[inline(always)]
    pub fn is_slota(&self) -> bool {
        *self == SDCSEL_A::SLOTA
    }
    #[doc = "Checks if the value of the field is `SLOTB`"]
    #[inline(always)]
    pub fn is_slotb(&self) -> bool {
        *self == SDCSEL_A::SLOTB
    }
    #[doc = "Checks if the value of the field is `SLOTC`"]
    #[inline(always)]
    pub fn is_slotc(&self) -> bool {
        *self == SDCSEL_A::SLOTC
    }
    #[doc = "Checks if the value of the field is `SLOTD`"]
    #[inline(always)]
    pub fn is_slotd(&self) -> bool {
        *self == SDCSEL_A::SLOTD
    }
}
#[doc = "Write proxy for field `SDCSEL`"]
pub struct SDCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Slot A is selected."]
    #[inline(always)]
    pub fn slota(self) -> &'a mut W {
        self.variant(SDCSEL_A::SLOTA)
    }
    #[doc = "SDCARD/SDIO Slot B selected"]
    #[inline(always)]
    pub fn slotb(self) -> &'a mut W {
        self.variant(SDCSEL_A::SLOTB)
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn slotc(self) -> &'a mut W {
        self.variant(SDCSEL_A::SLOTC)
    }
    #[doc = "-"]
    #[inline(always)]
    pub fn slotd(self) -> &'a mut W {
        self.variant(SDCSEL_A::SLOTD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "SDCard/SDIO Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDCBUS_A {
    #[doc = "0: 1 bit"]
    _1,
    #[doc = "2: 4 bits"]
    _4,
    #[doc = "3: 8 bits"]
    _8,
}
impl From<SDCBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCBUS_A) -> Self {
        match variant {
            SDCBUS_A::_1 => 0,
            SDCBUS_A::_4 => 2,
            SDCBUS_A::_8 => 3,
        }
    }
}
#[doc = "Reader of field `SDCBUS`"]
pub type SDCBUS_R = crate::R<u8, SDCBUS_A>;
impl SDCBUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDCBUS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SDCBUS_A::_1),
            2 => Val(SDCBUS_A::_4),
            3 => Val(SDCBUS_A::_8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCBUS_A::_1
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SDCBUS_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SDCBUS_A::_8
    }
}
#[doc = "Write proxy for field `SDCBUS`"]
pub struct SDCBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDCBUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDCBUS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCBUS_A::_1)
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SDCBUS_A::_4)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SDCBUS_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&self) -> SDCSEL_R {
        SDCSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&self) -> SDCBUS_R {
        SDCBUS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&mut self) -> SDCSEL_W {
        SDCSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&mut self) -> SDCBUS_W {
        SDCBUS_W { w: self }
    }
}
