#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
pub struct IDTER {
    bits: bool,
}
impl IDTER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VBUSTER {
    bits: bool,
}
impl VBUSTER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SRPER {
    bits: bool,
}
impl SRPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VBERRER {
    bits: bool,
}
impl VBERRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct BCERRER {
    bits: bool,
}
impl BCERRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ROLEEXER {
    bits: bool,
}
impl ROLEEXER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct HNPERRER {
    bits: bool,
}
impl HNPERRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct STOER {
    bits: bool,
}
impl STOER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VBUSHWCR {
    bits: bool,
}
impl VBUSHWCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SRPSELR {
    bits: bool,
}
impl SRPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SRPREQR {
    bits: bool,
}
impl SRPREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct HNPREQR {
    bits: bool,
}
impl HNPREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct OTGPADER {
    bits: bool,
}
impl OTGPADER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VBUSPOR {
    bits: bool,
}
impl VBUSPOR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FRZCLKR {
    bits: bool,
}
impl FRZCLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USBER {
    bits: bool,
}
impl USBER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TIMVALUER {
    bits: u8,
}
impl TIMVALUER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TIMPAGER {
    bits: u8,
}
impl TIMPAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UNLOCKR {
    bits: bool,
}
impl UNLOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `UIDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIDER {
    #[doc = "The USB mode (device/host) is selected from the UIMOD bit."]
    UIMOD,
    #[doc = "The USB mode (device/host) is selected from the UOTGID input pin."]
    UOTGID,
}
impl UIDER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UIDER::UIMOD => false,
            UIDER::UOTGID => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UIDER {
        match value {
            false => UIDER::UIMOD,
            true => UIDER::UOTGID,
        }
    }
    #[doc = "Checks if the value of the field is `UIMOD`"]
    #[inline]
    pub fn is_uimod(&self) -> bool {
        *self == UIDER::UIMOD
    }
    #[doc = "Checks if the value of the field is `UOTGID`"]
    #[inline]
    pub fn is_uotgid(&self) -> bool {
        *self == UIDER::UOTGID
    }
}
#[doc = "Possible values of the field `UIMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UIMODR {
    #[doc = "The module is in USB host mode."]
    HOST,
    #[doc = "The module is in USB device mode."]
    DEVICE,
}
impl UIMODR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UIMODR::HOST => false,
            UIMODR::DEVICE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UIMODR {
        match value {
            false => UIMODR::HOST,
            true => UIMODR::DEVICE,
        }
    }
    #[doc = "Checks if the value of the field is `HOST`"]
    #[inline]
    pub fn is_host(&self) -> bool {
        *self == UIMODR::HOST
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline]
    pub fn is_device(&self) -> bool {
        *self == UIMODR::DEVICE
    }
}
#[doc = r" Proxy"]
pub struct _IDTEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDTEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBUSTEW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSTEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRPEW<'a> {
    w: &'a mut W,
}
impl<'a> _SRPEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBERREW<'a> {
    w: &'a mut W,
}
impl<'a> _VBERREW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BCERREW<'a> {
    w: &'a mut W,
}
impl<'a> _BCERREW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ROLEEXEW<'a> {
    w: &'a mut W,
}
impl<'a> _ROLEEXEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HNPERREW<'a> {
    w: &'a mut W,
}
impl<'a> _HNPERREW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STOEW<'a> {
    w: &'a mut W,
}
impl<'a> _STOEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBUSHWCW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSHWCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _SRPSELW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SRPREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SRPREQW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HNPREQW<'a> {
    w: &'a mut W,
}
impl<'a> _HNPREQW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OTGPADEW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGPADEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBUSPOW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSPOW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRZCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _FRZCLKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBEW<'a> {
    w: &'a mut W,
}
impl<'a> _USBEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMVALUEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMVALUEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMPAGEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMPAGEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UNLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _UNLOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UIDE`"]
pub enum UIDEW {
    #[doc = "The USB mode (device/host) is selected from the UIMOD bit."]
    UIMOD,
    #[doc = "The USB mode (device/host) is selected from the UOTGID input pin."]
    UOTGID,
}
impl UIDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UIDEW::UIMOD => false,
            UIDEW::UOTGID => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UIDEW<'a> {
    w: &'a mut W,
}
impl<'a> _UIDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UIDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The USB mode (device/host) is selected from the UIMOD bit."]
    #[inline]
    pub fn uimod(self) -> &'a mut W {
        self.variant(UIDEW::UIMOD)
    }
    #[doc = "The USB mode (device/host) is selected from the UOTGID input pin."]
    #[inline]
    pub fn uotgid(self) -> &'a mut W {
        self.variant(UIDEW::UOTGID)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UIMOD`"]
pub enum UIMODW {
    #[doc = "The module is in USB host mode."]
    HOST,
    #[doc = "The module is in USB device mode."]
    DEVICE,
}
impl UIMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UIMODW::HOST => false,
            UIMODW::DEVICE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UIMODW<'a> {
    w: &'a mut W,
}
impl<'a> _UIMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UIMODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The module is in USB host mode."]
    #[inline]
    pub fn host(self) -> &'a mut W {
        self.variant(UIMODW::HOST)
    }
    #[doc = "The module is in USB device mode."]
    #[inline]
    pub fn device(self) -> &'a mut W {
        self.variant(UIMODW::DEVICE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
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
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline]
    pub fn idte(&self) -> IDTER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDTER { bits }
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline]
    pub fn vbuste(&self) -> VBUSTER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSTER { bits }
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline]
    pub fn srpe(&self) -> SRPER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRPER { bits }
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline]
    pub fn vberre(&self) -> VBERRER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBERRER { bits }
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline]
    pub fn bcerre(&self) -> BCERRER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BCERRER { bits }
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline]
    pub fn roleexe(&self) -> ROLEEXER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ROLEEXER { bits }
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline]
    pub fn hnperre(&self) -> HNPERRER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HNPERRER { bits }
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline]
    pub fn stoe(&self) -> STOER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STOER { bits }
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline]
    pub fn vbushwc(&self) -> VBUSHWCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSHWCR { bits }
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline]
    pub fn srpsel(&self) -> SRPSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRPSELR { bits }
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline]
    pub fn srpreq(&self) -> SRPREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SRPREQR { bits }
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline]
    pub fn hnpreq(&self) -> HNPREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HNPREQR { bits }
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline]
    pub fn otgpade(&self) -> OTGPADER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTGPADER { bits }
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline]
    pub fn vbuspo(&self) -> VBUSPOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSPOR { bits }
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline]
    pub fn frzclk(&self) -> FRZCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRZCLKR { bits }
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline]
    pub fn usbe(&self) -> USBER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USBER { bits }
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline]
    pub fn timvalue(&self) -> TIMVALUER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMVALUER { bits }
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline]
    pub fn timpage(&self) -> TIMPAGER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TIMPAGER { bits }
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline]
    pub fn unlock(&self) -> UNLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNLOCKR { bits }
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline]
    pub fn uide(&self) -> UIDER {
        UIDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline]
    pub fn uimod(&self) -> UIMODR {
        UIMODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 50348032 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - ID Transition Interrupt Enable"]
    #[inline]
    pub fn idte(&mut self) -> _IDTEW {
        _IDTEW { w: self }
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Enable"]
    #[inline]
    pub fn vbuste(&mut self) -> _VBUSTEW {
        _VBUSTEW { w: self }
    }
    #[doc = "Bit 2 - SRP Interrupt Enable"]
    #[inline]
    pub fn srpe(&mut self) -> _SRPEW {
        _SRPEW { w: self }
    }
    #[doc = "Bit 3 - VBus Error Interrupt Enable"]
    #[inline]
    pub fn vberre(&mut self) -> _VBERREW {
        _VBERREW { w: self }
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Enable"]
    #[inline]
    pub fn bcerre(&mut self) -> _BCERREW {
        _BCERREW { w: self }
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Enable"]
    #[inline]
    pub fn roleexe(&mut self) -> _ROLEEXEW {
        _ROLEEXEW { w: self }
    }
    #[doc = "Bit 6 - HNP Error Interrupt Enable"]
    #[inline]
    pub fn hnperre(&mut self) -> _HNPERREW {
        _HNPERREW { w: self }
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Enable"]
    #[inline]
    pub fn stoe(&mut self) -> _STOEW {
        _STOEW { w: self }
    }
    #[doc = "Bit 8 - VBus Hardware Control"]
    #[inline]
    pub fn vbushwc(&mut self) -> _VBUSHWCW {
        _VBUSHWCW { w: self }
    }
    #[doc = "Bit 9 - SRP Selection"]
    #[inline]
    pub fn srpsel(&mut self) -> _SRPSELW {
        _SRPSELW { w: self }
    }
    #[doc = "Bit 10 - SRP Request"]
    #[inline]
    pub fn srpreq(&mut self) -> _SRPREQW {
        _SRPREQW { w: self }
    }
    #[doc = "Bit 11 - HNP Request"]
    #[inline]
    pub fn hnpreq(&mut self) -> _HNPREQW {
        _HNPREQW { w: self }
    }
    #[doc = "Bit 12 - OTG Pad Enable"]
    #[inline]
    pub fn otgpade(&mut self) -> _OTGPADEW {
        _OTGPADEW { w: self }
    }
    #[doc = "Bit 13 - VBus Polarity Off"]
    #[inline]
    pub fn vbuspo(&mut self) -> _VBUSPOW {
        _VBUSPOW { w: self }
    }
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline]
    pub fn frzclk(&mut self) -> _FRZCLKW {
        _FRZCLKW { w: self }
    }
    #[doc = "Bit 15 - UOTGHS Enable"]
    #[inline]
    pub fn usbe(&mut self) -> _USBEW {
        _USBEW { w: self }
    }
    #[doc = "Bits 16:17 - Timer Value"]
    #[inline]
    pub fn timvalue(&mut self) -> _TIMVALUEW {
        _TIMVALUEW { w: self }
    }
    #[doc = "Bits 20:21 - Timer Page"]
    #[inline]
    pub fn timpage(&mut self) -> _TIMPAGEW {
        _TIMPAGEW { w: self }
    }
    #[doc = "Bit 22 - Timer Access Unlock"]
    #[inline]
    pub fn unlock(&mut self) -> _UNLOCKW {
        _UNLOCKW { w: self }
    }
    #[doc = "Bit 24 - UOTGID Pin Enable"]
    #[inline]
    pub fn uide(&mut self) -> _UIDEW {
        _UIDEW { w: self }
    }
    #[doc = "Bit 25 - UOTGHS Mode"]
    #[inline]
    pub fn uimod(&mut self) -> _UIMODW {
        _UIMODW { w: self }
    }
}
