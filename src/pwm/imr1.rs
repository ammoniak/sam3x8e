#[doc = "Reader of register IMR1"]
pub type R = crate::R<u32, super::IMR1>;
#[doc = "Reader of field `CHID0`"]
pub type CHID0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID1`"]
pub type CHID1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID2`"]
pub type CHID2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID3`"]
pub type CHID3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID4`"]
pub type CHID4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID5`"]
pub type CHID5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID6`"]
pub type CHID6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID7`"]
pub type CHID7_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID0`"]
pub type FCHID0_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID1`"]
pub type FCHID1_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID2`"]
pub type FCHID2_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID3`"]
pub type FCHID3_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID4`"]
pub type FCHID4_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID5`"]
pub type FCHID5_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID6`"]
pub type FCHID6_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID7`"]
pub type FCHID7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Counter Event on Channel 4 Interrupt Mask"]
    #[inline(always)]
    pub fn chid4(&self) -> CHID4_R {
        CHID4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Counter Event on Channel 5 Interrupt Mask"]
    #[inline(always)]
    pub fn chid5(&self) -> CHID5_R {
        CHID5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Counter Event on Channel 6 Interrupt Mask"]
    #[inline(always)]
    pub fn chid6(&self) -> CHID6_R {
        CHID6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter Event on Channel 7 Interrupt Mask"]
    #[inline(always)]
    pub fn chid7(&self) -> CHID7_R {
        CHID7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid0(&self) -> FCHID0_R {
        FCHID0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid1(&self) -> FCHID1_R {
        FCHID1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid2(&self) -> FCHID2_R {
        FCHID2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid3(&self) -> FCHID3_R {
        FCHID3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fault Protection Trigger on Channel 4 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid4(&self) -> FCHID4_R {
        FCHID4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fault Protection Trigger on Channel 5 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid5(&self) -> FCHID5_R {
        FCHID5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fault Protection Trigger on Channel 6 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid6(&self) -> FCHID6_R {
        FCHID6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fault Protection Trigger on Channel 7 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid7(&self) -> FCHID7_R {
        FCHID7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
