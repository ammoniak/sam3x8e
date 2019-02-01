#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MMR6 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct MTIMEMARKR {
    bits: u16,
}
impl MTIMEMARKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRIORR {
    bits: u8,
}
impl PRIORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOTR {
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    MB_DISABLED,
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    MB_RX,
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    MB_RX_OVERWRITE,
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    MB_TX,
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    MB_CONSUMER,
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    MB_PRODUCER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MOTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MOTR::MB_DISABLED => 0,
            MOTR::MB_RX => 1,
            MOTR::MB_RX_OVERWRITE => 2,
            MOTR::MB_TX => 3,
            MOTR::MB_CONSUMER => 4,
            MOTR::MB_PRODUCER => 5,
            MOTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MOTR {
        match value {
            0 => MOTR::MB_DISABLED,
            1 => MOTR::MB_RX,
            2 => MOTR::MB_RX_OVERWRITE,
            3 => MOTR::MB_TX,
            4 => MOTR::MB_CONSUMER,
            5 => MOTR::MB_PRODUCER,
            i => MOTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MB_DISABLED`"]
    #[inline]
    pub fn is_mb_disabled(&self) -> bool {
        *self == MOTR::MB_DISABLED
    }
    #[doc = "Checks if the value of the field is `MB_RX`"]
    #[inline]
    pub fn is_mb_rx(&self) -> bool {
        *self == MOTR::MB_RX
    }
    #[doc = "Checks if the value of the field is `MB_RX_OVERWRITE`"]
    #[inline]
    pub fn is_mb_rx_overwrite(&self) -> bool {
        *self == MOTR::MB_RX_OVERWRITE
    }
    #[doc = "Checks if the value of the field is `MB_TX`"]
    #[inline]
    pub fn is_mb_tx(&self) -> bool {
        *self == MOTR::MB_TX
    }
    #[doc = "Checks if the value of the field is `MB_CONSUMER`"]
    #[inline]
    pub fn is_mb_consumer(&self) -> bool {
        *self == MOTR::MB_CONSUMER
    }
    #[doc = "Checks if the value of the field is `MB_PRODUCER`"]
    #[inline]
    pub fn is_mb_producer(&self) -> bool {
        *self == MOTR::MB_PRODUCER
    }
}
#[doc = r" Proxy"]
pub struct _MTIMEMARKW<'a> {
    w: &'a mut W,
}
impl<'a> _MTIMEMARKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRIORW<'a> {
    w: &'a mut W,
}
impl<'a> _PRIORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MOT`"]
pub enum MOTW {
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    MB_DISABLED,
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    MB_RX,
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    MB_RX_OVERWRITE,
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    MB_TX,
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    MB_CONSUMER,
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    MB_PRODUCER,
}
impl MOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MOTW::MB_DISABLED => 0,
            MOTW::MB_RX => 1,
            MOTW::MB_RX_OVERWRITE => 2,
            MOTW::MB_TX => 3,
            MOTW::MB_CONSUMER => 4,
            MOTW::MB_PRODUCER => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MOTW<'a> {
    w: &'a mut W,
}
impl<'a> _MOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MOTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Mailbox is disabled. This prevents receiving or transmitting any messages with this mailbox."]
    #[inline]
    pub fn mb_disabled(self) -> &'a mut W {
        self.variant(MOTW::MB_DISABLED)
    }
    #[doc = "Reception Mailbox. Mailbox is configured for reception. If a message is received while the mailbox data register is full, it is discarded."]
    #[inline]
    pub fn mb_rx(self) -> &'a mut W {
        self.variant(MOTW::MB_RX)
    }
    #[doc = "Reception mailbox with overwrite. Mailbox is configured for reception. If a message is received while the mailbox is full, it overwrites the previous message."]
    #[inline]
    pub fn mb_rx_overwrite(self) -> &'a mut W {
        self.variant(MOTW::MB_RX_OVERWRITE)
    }
    #[doc = "Transmit mailbox. Mailbox is configured for transmission."]
    #[inline]
    pub fn mb_tx(self) -> &'a mut W {
        self.variant(MOTW::MB_TX)
    }
    #[doc = "Consumer Mailbox. Mailbox is configured in reception but behaves as a Transmit Mailbox, i.e., it sends a remote frame and waits for an answer."]
    #[inline]
    pub fn mb_consumer(self) -> &'a mut W {
        self.variant(MOTW::MB_CONSUMER)
    }
    #[doc = "Producer Mailbox. Mailbox is configured in transmission but also behaves like a reception mailbox, i.e., it waits to receive a Remote Frame before sending its contents."]
    #[inline]
    pub fn mb_producer(self) -> &'a mut W {
        self.variant(MOTW::MB_PRODUCER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline]
    pub fn mtimemark(&self) -> MTIMEMARKR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MTIMEMARKR { bits }
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline]
    pub fn prior(&self) -> PRIORR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRIORR { bits }
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline]
    pub fn mot(&self) -> MOTR {
        MOTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Mailbox Timemark"]
    #[inline]
    pub fn mtimemark(&mut self) -> _MTIMEMARKW {
        _MTIMEMARKW { w: self }
    }
    #[doc = "Bits 16:19 - Mailbox Priority"]
    #[inline]
    pub fn prior(&mut self) -> _PRIORW {
        _PRIORW { w: self }
    }
    #[doc = "Bits 24:26 - Mailbox Object Type"]
    #[inline]
    pub fn mot(&mut self) -> _MOTW {
        _MOTW { w: self }
    }
}
