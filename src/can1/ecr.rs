#[doc = "Reader of register ECR"]
pub type R = crate::R<u32, super::ECR>;
#[doc = "Reader of field `REC`"]
pub type REC_R = crate::R<u8, u8>;
#[doc = "Reader of field `TEC`"]
pub type TEC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
