#[doc = "Reader of register PMC_PCSR1"]
pub type R = crate::R<u32, super::PMC_PCSR1>;
#[doc = "Reader of field `PID32`"]
pub type PID32_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID33`"]
pub type PID33_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID34`"]
pub type PID34_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID35`"]
pub type PID35_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID36`"]
pub type PID36_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID37`"]
pub type PID37_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID38`"]
pub type PID38_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID39`"]
pub type PID39_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID40`"]
pub type PID40_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID41`"]
pub type PID41_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID42`"]
pub type PID42_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID43`"]
pub type PID43_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID44`"]
pub type PID44_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Peripheral Clock 32 Status"]
    #[inline(always)]
    pub fn pid32(&self) -> PID32_R {
        PID32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Status"]
    #[inline(always)]
    pub fn pid33(&self) -> PID33_R {
        PID33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Status"]
    #[inline(always)]
    pub fn pid34(&self) -> PID34_R {
        PID34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Status"]
    #[inline(always)]
    pub fn pid35(&self) -> PID35_R {
        PID35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Peripheral Clock 36 Status"]
    #[inline(always)]
    pub fn pid36(&self) -> PID36_R {
        PID36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Status"]
    #[inline(always)]
    pub fn pid37(&self) -> PID37_R {
        PID37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral Clock 38 Status"]
    #[inline(always)]
    pub fn pid38(&self) -> PID38_R {
        PID38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Status"]
    #[inline(always)]
    pub fn pid39(&self) -> PID39_R {
        PID39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Status"]
    #[inline(always)]
    pub fn pid40(&self) -> PID40_R {
        PID40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Status"]
    #[inline(always)]
    pub fn pid41(&self) -> PID41_R {
        PID41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Status"]
    #[inline(always)]
    pub fn pid42(&self) -> PID42_R {
        PID42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Status"]
    #[inline(always)]
    pub fn pid43(&self) -> PID43_R {
        PID43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Status"]
    #[inline(always)]
    pub fn pid44(&self) -> PID44_R {
        PID44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
