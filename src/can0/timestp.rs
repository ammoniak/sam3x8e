#[doc = "Reader of register TIMESTP"]
pub type R = crate::R<u32, super::TIMESTP>;
#[doc = "Reader of field `MTIMESTAMP`"]
pub type MTIMESTAMP_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp"]
    #[inline(always)]
    pub fn mtimestamp(&self) -> MTIMESTAMP_R {
        MTIMESTAMP_R::new((self.bits & 0xffff) as u16)
    }
}
