#[doc = "Reader of register MMR2"]
pub type R = crate::R<u32, super::MMR2>;
#[doc = "Writer for register MMR2"]
pub type W = crate::W<u32, super::MMR2>;
#[doc = "Register MMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::MMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MTIMEMARK`"]
pub type MTIMEMARK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MTIMEMARK`"]
pub struct MTIMEMARK_W<'a> {
    w: &'a mut W,
}
impl<'a> MTIMEMARK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `PRIOR`"]
pub type PRIOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRIOR`"]
pub struct PRIOR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Mailbox Object Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MOT_A {
    #[doc = "0: Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    MB_DISABLED = 0,
    #[doc = "1: Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    MB_RX = 1,
    #[doc = "2: Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    MB_RX_OVERWRITE = 2,
    #[doc = "3: Transmit mailbox. Mailbox is configured for transmission."]
    MB_TX = 3,
    #[doc = "4: Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    MB_CONSUMER = 4,
    #[doc = "5: Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    MB_PRODUCER = 5,
}
impl From<MOT_A> for u8 {
    #[inline(always)]
    fn from(variant: MOT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MOT`"]
pub type MOT_R = crate::R<u8, MOT_A>;
impl MOT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MOT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MOT_A::MB_DISABLED),
            1 => Val(MOT_A::MB_RX),
            2 => Val(MOT_A::MB_RX_OVERWRITE),
            3 => Val(MOT_A::MB_TX),
            4 => Val(MOT_A::MB_CONSUMER),
            5 => Val(MOT_A::MB_PRODUCER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MB_DISABLED`"]
    #[inline(always)]
    pub fn is_mb_disabled(&self) -> bool {
        *self == MOT_A::MB_DISABLED
    }
    #[doc = "Checks if the value of the field is `MB_RX`"]
    #[inline(always)]
    pub fn is_mb_rx(&self) -> bool {
        *self == MOT_A::MB_RX
    }
    #[doc = "Checks if the value of the field is `MB_RX_OVERWRITE`"]
    #[inline(always)]
    pub fn is_mb_rx_overwrite(&self) -> bool {
        *self == MOT_A::MB_RX_OVERWRITE
    }
    #[doc = "Checks if the value of the field is `MB_TX`"]
    #[inline(always)]
    pub fn is_mb_tx(&self) -> bool {
        *self == MOT_A::MB_TX
    }
    #[doc = "Checks if the value of the field is `MB_CONSUMER`"]
    #[inline(always)]
    pub fn is_mb_consumer(&self) -> bool {
        *self == MOT_A::MB_CONSUMER
    }
    #[doc = "Checks if the value of the field is `MB_PRODUCER`"]
    #[inline(always)]
    pub fn is_mb_producer(&self) -> bool {
        *self == MOT_A::MB_PRODUCER
    }
}
#[doc = "Write proxy for field `MOT`"]
pub struct MOT_W<'a> {
    w: &'a mut W,
}
impl<'a> MOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    #[inline(always)]
    pub fn mb_disabled(self) -> &'a mut W {
        self.variant(MOT_A::MB_DISABLED)
    }
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    #[inline(always)]
    pub fn mb_rx(self) -> &'a mut W {
        self.variant(MOT_A::MB_RX)
    }
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    #[inline(always)]
    pub fn mb_rx_overwrite(self) -> &'a mut W {
        self.variant(MOT_A::MB_RX_OVERWRITE)
    }
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    #[inline(always)]
    pub fn mb_tx(self) -> &'a mut W {
        self.variant(MOT_A::MB_TX)
    }
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    #[inline(always)]
    pub fn mb_consumer(self) -> &'a mut W {
        self.variant(MOT_A::MB_CONSUMER)
    }
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    #[inline(always)]
    pub fn mb_producer(self) -> &'a mut W {
        self.variant(MOT_A::MB_PRODUCER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline(always)]
    pub fn mtimemark(&self) -> MTIMEMARK_R {
        MTIMEMARK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline(always)]
    pub fn prior(&self) -> PRIOR_R {
        PRIOR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline(always)]
    pub fn mot(&self) -> MOT_R {
        MOT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline(always)]
    pub fn mtimemark(&mut self) -> MTIMEMARK_W {
        MTIMEMARK_W { w: self }
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline(always)]
    pub fn prior(&mut self) -> PRIOR_W {
        PRIOR_W { w: self }
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline(always)]
    pub fn mot(&mut self) -> MOT_W {
        MOT_W { w: self }
    }
}
