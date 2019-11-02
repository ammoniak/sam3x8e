#[doc = "Reader of register TIM"]
pub type R = crate::R<u32, super::TIM>;
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
