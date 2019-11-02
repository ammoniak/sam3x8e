#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `MB0`"]
pub type MB0_R = crate::R<bool, bool>;
#[doc = "Reader of field `MB1`"]
pub type MB1_R = crate::R<bool, bool>;
#[doc = "Reader of field `MB2`"]
pub type MB2_R = crate::R<bool, bool>;
#[doc = "Reader of field `MB3`"]
pub type MB3_R = crate::R<bool, bool>;
#[doc = "Reader of field `MB4`"]
pub type MB4_R = crate::R<bool, bool>;
#[doc = "Reader of field `MB5`"]
pub type MB5_R = crate::R<bool, bool>;
#[doc = "Reader of field `MB6`"]
pub type MB6_R = crate::R<bool, bool>;
#[doc = "Reader of field `MB7`"]
pub type MB7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRA`"]
pub type ERRA_R = crate::R<bool, bool>;
#[doc = "Reader of field `WARN`"]
pub type WARN_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRP`"]
pub type ERRP_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOFF`"]
pub type BOFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, bool>;
#[doc = "Reader of field `WAKEUP`"]
pub type WAKEUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TOVF`"]
pub type TOVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSTP`"]
pub type TSTP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CERR`"]
pub type CERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SERR`"]
pub type SERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `AERR`"]
pub type AERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `FERR`"]
pub type FERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `BERR`"]
pub type BERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Mailbox 0 Interrupt Mask"]
    #[inline(always)]
    pub fn mb0(&self) -> MB0_R {
        MB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mailbox 1 Interrupt Mask"]
    #[inline(always)]
    pub fn mb1(&self) -> MB1_R {
        MB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mailbox 2 Interrupt Mask"]
    #[inline(always)]
    pub fn mb2(&self) -> MB2_R {
        MB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mailbox 3 Interrupt Mask"]
    #[inline(always)]
    pub fn mb3(&self) -> MB3_R {
        MB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mailbox 4 Interrupt Mask"]
    #[inline(always)]
    pub fn mb4(&self) -> MB4_R {
        MB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mailbox 5 Interrupt Mask"]
    #[inline(always)]
    pub fn mb5(&self) -> MB5_R {
        MB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mailbox 6 Interrupt Mask"]
    #[inline(always)]
    pub fn mb6(&self) -> MB6_R {
        MB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mailbox 7 Interrupt Mask"]
    #[inline(always)]
    pub fn mb7(&self) -> MB7_R {
        MB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Error Active Mode Interrupt Mask"]
    #[inline(always)]
    pub fn erra(&self) -> ERRA_R {
        ERRA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Warning Limit Interrupt Mask"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Error Passive Mode Interrupt Mask"]
    #[inline(always)]
    pub fn errp(&self) -> ERRP_R {
        ERRP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bus Off Mode Interrupt Mask"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Sleep Interrupt Mask"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Wakeup Interrupt Mask"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timer Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn tovf(&self) -> TOVF_R {
        TOVF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tstp(&self) -> TSTP_R {
        TSTP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Stuffing Error Interrupt Mask"]
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Acknowledgment Error Interrupt Mask"]
    #[inline(always)]
    pub fn aerr(&self) -> AERR_R {
        AERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Form Error Interrupt Mask"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
