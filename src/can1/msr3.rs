#[doc = "Reader of register MSR3"]
pub type R = crate::R<u32, super::MSR3>;
#[doc = "Reader of field `MTIMESTAMP`"]
pub type MTIMESTAMP_R = crate::R<u16, u16>;
#[doc = "Reader of field `MDLC`"]
pub type MDLC_R = crate::R<u8, u8>;
#[doc = "Reader of field `MRTR`"]
pub type MRTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `MABT`"]
pub type MABT_R = crate::R<bool, bool>;
#[doc = "Reader of field `MRDY`"]
pub type MRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `MMI`"]
pub type MMI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - Timer value"]
    #[inline(always)]
    pub fn mtimestamp(&self) -> MTIMESTAMP_R {
        MTIMESTAMP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Mailbox Data Length Code"]
    #[inline(always)]
    pub fn mdlc(&self) -> MDLC_R {
        MDLC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    pub fn mrtr(&self) -> MRTR_R {
        MRTR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Mailbox Message Abort"]
    #[inline(always)]
    pub fn mabt(&self) -> MABT_R {
        MABT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Mailbox Ready"]
    #[inline(always)]
    pub fn mrdy(&self) -> MRDY_R {
        MRDY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Mailbox Message Ignored"]
    #[inline(always)]
    pub fn mmi(&self) -> MMI_R {
        MMI_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
