#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `IDTI`"]
pub type IDTI_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBUSTI`"]
pub type VBUSTI_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRPI`"]
pub type SRPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBERRI`"]
pub type VBERRI_R = crate::R<bool, bool>;
#[doc = "Reader of field `BCERRI`"]
pub type BCERRI_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROLEEXI`"]
pub type ROLEEXI_R = crate::R<bool, bool>;
#[doc = "Reader of field `HNPERRI`"]
pub type HNPERRI_R = crate::R<bool, bool>;
#[doc = "Reader of field `STOI`"]
pub type STOI_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBUSRQ`"]
pub type VBUSRQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBUS`"]
pub type VBUS_R = crate::R<bool, bool>;
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEED_A {
    #[doc = "0: Full-Speed mode"]
    FULL_SPEED,
    #[doc = "1: High-Speed mode"]
    HIGH_SPEED,
    #[doc = "2: Low-Speed mode"]
    LOW_SPEED,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        match variant {
            SPEED_A::FULL_SPEED => 0,
            SPEED_A::HIGH_SPEED => 1,
            SPEED_A::LOW_SPEED => 2,
        }
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPEED_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPEED_A::FULL_SPEED),
            1 => Val(SPEED_A::HIGH_SPEED),
            2 => Val(SPEED_A::LOW_SPEED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        *self == SPEED_A::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == SPEED_A::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == SPEED_A::LOW_SPEED
    }
}
#[doc = "Reader of field `CLKUSABLE`"]
pub type CLKUSABLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ID Transition Interrupt"]
    #[inline(always)]
    pub fn idti(&self) -> IDTI_R {
        IDTI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VBus Transition Interrupt"]
    #[inline(always)]
    pub fn vbusti(&self) -> VBUSTI_R {
        VBUSTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SRP Interrupt"]
    #[inline(always)]
    pub fn srpi(&self) -> SRPI_R {
        SRPI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VBus Error Interrupt"]
    #[inline(always)]
    pub fn vberri(&self) -> VBERRI_R {
        VBERRI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt"]
    #[inline(always)]
    pub fn bcerri(&self) -> BCERRI_R {
        BCERRI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Role Exchange Interrupt"]
    #[inline(always)]
    pub fn roleexi(&self) -> ROLEEXI_R {
        ROLEEXI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HNP Error Interrupt"]
    #[inline(always)]
    pub fn hnperri(&self) -> HNPERRI_R {
        HNPERRI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt"]
    #[inline(always)]
    pub fn stoi(&self) -> STOI_R {
        STOI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VBus Request"]
    #[inline(always)]
    pub fn vbusrq(&self) -> VBUSRQ_R {
        VBUSRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UOTGID Pin State"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - VBus Level"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> CLKUSABLE_R {
        CLKUSABLE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
