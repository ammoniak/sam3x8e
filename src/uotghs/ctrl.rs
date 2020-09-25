#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x0300_4000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0300_4000
    }
}
#[doc = "Reader of field `IDTE`"]
pub type IDTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDTE`"]
pub struct IDTE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTE_W<'a> {
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
#[doc = "Reader of field `VBUSTE`"]
pub type VBUSTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSTE`"]
pub struct VBUSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSTE_W<'a> {
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
#[doc = "Reader of field `SRPE`"]
pub type SRPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRPE`"]
pub struct SRPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `VBERRE`"]
pub type VBERRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBERRE`"]
pub struct VBERRE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBERRE_W<'a> {
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
#[doc = "Reader of field `BCERRE`"]
pub type BCERRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BCERRE`"]
pub struct BCERRE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCERRE_W<'a> {
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
#[doc = "Reader of field `ROLEEXE`"]
pub type ROLEEXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ROLEEXE`"]
pub struct ROLEEXE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROLEEXE_W<'a> {
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
#[doc = "Reader of field `HNPERRE`"]
pub type HNPERRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNPERRE`"]
pub struct HNPERRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPERRE_W<'a> {
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
#[doc = "Reader of field `STOE`"]
pub type STOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOE`"]
pub struct STOE_W<'a> {
    w: &'a mut W,
}
impl<'a> STOE_W<'a> {
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
#[doc = "Reader of field `VBUSHWC`"]
pub type VBUSHWC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSHWC`"]
pub struct VBUSHWC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSHWC_W<'a> {
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
#[doc = "Reader of field `SRPSEL`"]
pub type SRPSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRPSEL`"]
pub struct SRPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPSEL_W<'a> {
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
#[doc = "Reader of field `SRPREQ`"]
pub type SRPREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRPREQ`"]
pub struct SRPREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPREQ_W<'a> {
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
#[doc = "Reader of field `HNPREQ`"]
pub type HNPREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HNPREQ`"]
pub struct HNPREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `OTGPADE`"]
pub type OTGPADE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGPADE`"]
pub struct OTGPADE_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGPADE_W<'a> {
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
#[doc = "Reader of field `VBUSPO`"]
pub type VBUSPO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSPO`"]
pub struct VBUSPO_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSPO_W<'a> {
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
#[doc = "Reader of field `FRZCLK`"]
pub type FRZCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRZCLK`"]
pub struct FRZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `USBE`"]
pub type USBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBE`"]
pub struct USBE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TIMVALUE`"]
pub type TIMVALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMVALUE`"]
pub struct TIMVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `TIMPAGE`"]
pub type TIMPAGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMPAGE`"]
pub struct TIMPAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `UNLOCK`"]
pub type UNLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNLOCK`"]
pub struct UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "UOTGID Pin Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIDE_A {
    #[doc = "0: The USB mode (device/host) is selected from the UIMOD bit."]
    UIMOD = 0,
    #[doc = "1: The USB mode (device/host) is selected from the UOTGID input pin."]
    UOTGID = 1,
}
impl From<UIDE_A> for bool {
    #[inline(always)]
    fn from(variant: UIDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UIDE`"]
pub type UIDE_R = crate::R<bool, UIDE_A>;
impl UIDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIDE_A {
        match self.bits {
            false => UIDE_A::UIMOD,
            true => UIDE_A::UOTGID,
        }
    }
    #[doc = "Checks if the value of the field is `UIMOD`"]
    #[inline(always)]
    pub fn is_uimod(&self) -> bool {
        *self == UIDE_A::UIMOD
    }
    #[doc = "Checks if the value of the field is `UOTGID`"]
    #[inline(always)]
    pub fn is_uotgid(&self) -> bool {
        *self == UIDE_A::UOTGID
    }
}
#[doc = "Write proxy for field `UIDE`"]
pub struct UIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> UIDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UIDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The USB mode (device/host) is selected from the UIMOD bit."]
    #[inline(always)]
    pub fn uimod(self) -> &'a mut W {
        self.variant(UIDE_A::UIMOD)
    }
    #[doc = "The USB mode (device/host) is selected from the UOTGID input pin."]
    #[inline(always)]
    pub fn uotgid(self) -> &'a mut W {
        self.variant(UIDE_A::UOTGID)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "UOTGHS Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIMOD_A {
    #[doc = "0: The module is in USB host mode."]
    HOST = 0,
    #[doc = "1: The module is in USB device mode."]
    DEVICE = 1,
}
impl From<UIMOD_A> for bool {
    #[inline(always)]
    fn from(variant: UIMOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UIMOD`"]
pub type UIMOD_R = crate::R<bool, UIMOD_A>;
impl UIMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UIMOD_A {
        match self.bits {
            false => UIMOD_A::HOST,
            true => UIMOD_A::DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == UIMOD_A::HOST
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == UIMOD_A::DEVICE
    }
}
#[doc = "Write proxy for field `UIMOD`"]
pub struct UIMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> UIMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UIMOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The module is in USB host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut W {
        self.variant(UIMOD_A::HOST)
    }
    #[doc = "The module is in USB device mode."]
    #[inline(always)]
    pub fn device(self) -> &'a mut W {
        self.variant(UIMOD_A::DEVICE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline(always)]
    pub fn idte(&self) -> IDTE_R {
        IDTE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline(always)]
    pub fn vbuste(&self) -> VBUSTE_R {
        VBUSTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline(always)]
    pub fn srpe(&self) -> SRPE_R {
        SRPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline(always)]
    pub fn vberre(&self) -> VBERRE_R {
        VBERRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn bcerre(&self) -> BCERRE_R {
        BCERRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline(always)]
    pub fn roleexe(&self) -> ROLEEXE_R {
        ROLEEXE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline(always)]
    pub fn hnperre(&self) -> HNPERRE_R {
        HNPERRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline(always)]
    pub fn stoe(&self) -> STOE_R {
        STOE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&self) -> VBUSHWC_R {
        VBUSHWC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline(always)]
    pub fn srpsel(&self) -> SRPSEL_R {
        SRPSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline(always)]
    pub fn srpreq(&self) -> SRPREQ_R {
        SRPREQ_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline(always)]
    pub fn otgpade(&self) -> OTGPADE_R {
        OTGPADE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline(always)]
    pub fn vbuspo(&self) -> VBUSPO_R {
        VBUSPO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline(always)]
    pub fn timvalue(&self) -> TIMVALUE_R {
        TIMVALUE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline(always)]
    pub fn timpage(&self) -> TIMPAGE_R {
        TIMPAGE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline(always)]
    pub fn uide(&self) -> UIDE_R {
        UIDE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline(always)]
    pub fn idte(&mut self) -> IDTE_W {
        IDTE_W { w: self }
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline(always)]
    pub fn vbuste(&mut self) -> VBUSTE_W {
        VBUSTE_W { w: self }
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline(always)]
    pub fn srpe(&mut self) -> SRPE_W {
        SRPE_W { w: self }
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline(always)]
    pub fn vberre(&mut self) -> VBERRE_W {
        VBERRE_W { w: self }
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline(always)]
    pub fn bcerre(&mut self) -> BCERRE_W {
        BCERRE_W { w: self }
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline(always)]
    pub fn roleexe(&mut self) -> ROLEEXE_W {
        ROLEEXE_W { w: self }
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline(always)]
    pub fn hnperre(&mut self) -> HNPERRE_W {
        HNPERRE_W { w: self }
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline(always)]
    pub fn stoe(&mut self) -> STOE_W {
        STOE_W { w: self }
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline(always)]
    pub fn vbushwc(&mut self) -> VBUSHWC_W {
        VBUSHWC_W { w: self }
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline(always)]
    pub fn srpsel(&mut self) -> SRPSEL_W {
        SRPSEL_W { w: self }
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline(always)]
    pub fn srpreq(&mut self) -> SRPREQ_W {
        SRPREQ_W { w: self }
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline(always)]
    pub fn hnpreq(&mut self) -> HNPREQ_W {
        HNPREQ_W { w: self }
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline(always)]
    pub fn otgpade(&mut self) -> OTGPADE_W {
        OTGPADE_W { w: self }
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline(always)]
    pub fn vbuspo(&mut self) -> VBUSPO_W {
        VBUSPO_W { w: self }
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&mut self) -> FRZCLK_W {
        FRZCLK_W { w: self }
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline(always)]
    pub fn usbe(&mut self) -> USBE_W {
        USBE_W { w: self }
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline(always)]
    pub fn timvalue(&mut self) -> TIMVALUE_W {
        TIMVALUE_W { w: self }
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline(always)]
    pub fn timpage(&mut self) -> TIMPAGE_W {
        TIMPAGE_W { w: self }
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W {
        UNLOCK_W { w: self }
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline(always)]
    pub fn uide(&mut self) -> UIDE_W {
        UIDE_W { w: self }
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline(always)]
    pub fn uimod(&mut self) -> UIMOD_W {
        UIMOD_W { w: self }
    }
}
