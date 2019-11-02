#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
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
impl R {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel ID"]
    #[inline(always)]
    pub fn chid4(&self) -> CHID4_R {
        CHID4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel ID"]
    #[inline(always)]
    pub fn chid5(&self) -> CHID5_R {
        CHID5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel ID"]
    #[inline(always)]
    pub fn chid6(&self) -> CHID6_R {
        CHID6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel ID"]
    #[inline(always)]
    pub fn chid7(&self) -> CHID7_R {
        CHID7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
