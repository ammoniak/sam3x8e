#[doc = "Reader of register CHSR"]
pub type R = crate::R<u32, super::CHSR>;
#[doc = "Reader of field `ENA0`"]
pub type ENA0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENA1`"]
pub type ENA1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENA2`"]
pub type ENA2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENA3`"]
pub type ENA3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENA4`"]
pub type ENA4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENA5`"]
pub type ENA5_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP0`"]
pub type SUSP0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP1`"]
pub type SUSP1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP2`"]
pub type SUSP2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP3`"]
pub type SUSP3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP4`"]
pub type SUSP4_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSP5`"]
pub type SUSP5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMPT0`"]
pub type EMPT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMPT1`"]
pub type EMPT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMPT2`"]
pub type EMPT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMPT3`"]
pub type EMPT3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMPT4`"]
pub type EMPT4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EMPT5`"]
pub type EMPT5_R = crate::R<bool, bool>;
#[doc = "Reader of field `STAL0`"]
pub type STAL0_R = crate::R<bool, bool>;
#[doc = "Reader of field `STAL1`"]
pub type STAL1_R = crate::R<bool, bool>;
#[doc = "Reader of field `STAL2`"]
pub type STAL2_R = crate::R<bool, bool>;
#[doc = "Reader of field `STAL3`"]
pub type STAL3_R = crate::R<bool, bool>;
#[doc = "Reader of field `STAL4`"]
pub type STAL4_R = crate::R<bool, bool>;
#[doc = "Reader of field `STAL5`"]
pub type STAL5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena0(&self) -> ENA0_R {
        ENA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena1(&self) -> ENA1_R {
        ENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena2(&self) -> ENA2_R {
        ENA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena3(&self) -> ENA3_R {
        ENA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena4(&self) -> ENA4_R {
        ENA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable \\[5:0\\]"]
    #[inline(always)]
    pub fn ena5(&self) -> ENA5_R {
        ENA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp0(&self) -> SUSP0_R {
        SUSP0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp1(&self) -> SUSP1_R {
        SUSP1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp2(&self) -> SUSP2_R {
        SUSP2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp3(&self) -> SUSP3_R {
        SUSP3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp4(&self) -> SUSP4_R {
        SUSP4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Suspend \\[5:0\\]"]
    #[inline(always)]
    pub fn susp5(&self) -> SUSP5_R {
        SUSP5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt0(&self) -> EMPT0_R {
        EMPT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt1(&self) -> EMPT1_R {
        EMPT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt2(&self) -> EMPT2_R {
        EMPT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt3(&self) -> EMPT3_R {
        EMPT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt4(&self) -> EMPT4_R {
        EMPT4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Empty \\[5:0\\]"]
    #[inline(always)]
    pub fn empt5(&self) -> EMPT5_R {
        EMPT5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal0(&self) -> STAL0_R {
        STAL0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal1(&self) -> STAL1_R {
        STAL1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal2(&self) -> STAL2_R {
        STAL2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal3(&self) -> STAL3_R {
        STAL3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal4(&self) -> STAL4_R {
        STAL4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Stalled \\[5:0\\]"]
    #[inline(always)]
    pub fn stal5(&self) -> STAL5_R {
        STAL5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
