#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MR {
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
#[doc = "Possible values of the field `PAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARR {
    #[doc = "Even Parity"]
    EVEN,
    #[doc = "Odd Parity"]
    ODD,
    #[doc = "Space: parity forced to 0"]
    SPACE,
    #[doc = "Mark: parity forced to 1"]
    MARK,
    #[doc = "No Parity"]
    NO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARR::EVEN => 0,
            PARR::ODD => 1,
            PARR::SPACE => 2,
            PARR::MARK => 3,
            PARR::NO => 4,
            PARR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARR {
        match value {
            0 => PARR::EVEN,
            1 => PARR::ODD,
            2 => PARR::SPACE,
            3 => PARR::MARK,
            4 => PARR::NO,
            i => PARR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == PARR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == PARR::ODD
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline]
    pub fn is_space(&self) -> bool {
        *self == PARR::SPACE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline]
    pub fn is_mark(&self) -> bool {
        *self == PARR::MARK
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == PARR::NO
    }
}
#[doc = "Possible values of the field `CHMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHMODER {
    #[doc = "Normal Mode"]
    NORMAL,
    #[doc = "Automatic Echo"]
    AUTOMATIC,
    #[doc = "Local Loopback"]
    LOCAL_LOOPBACK,
    #[doc = "Remote Loopback"]
    REMOTE_LOOPBACK,
}
impl CHMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHMODER::NORMAL => 0,
            CHMODER::AUTOMATIC => 1,
            CHMODER::LOCAL_LOOPBACK => 2,
            CHMODER::REMOTE_LOOPBACK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHMODER {
        match value {
            0 => CHMODER::NORMAL,
            1 => CHMODER::AUTOMATIC,
            2 => CHMODER::LOCAL_LOOPBACK,
            3 => CHMODER::REMOTE_LOOPBACK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == CHMODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline]
    pub fn is_automatic(&self) -> bool {
        *self == CHMODER::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOPBACK`"]
    #[inline]
    pub fn is_local_loopback(&self) -> bool {
        *self == CHMODER::LOCAL_LOOPBACK
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOPBACK`"]
    #[inline]
    pub fn is_remote_loopback(&self) -> bool {
        *self == CHMODER::REMOTE_LOOPBACK
    }
}
#[doc = "Values that can be written to the field `PAR`"]
pub enum PARW {
    #[doc = "Even Parity"]
    EVEN,
    #[doc = "Odd Parity"]
    ODD,
    #[doc = "Space: parity forced to 0"]
    SPACE,
    #[doc = "Mark: parity forced to 1"]
    MARK,
    #[doc = "No Parity"]
    NO,
}
impl PARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARW::EVEN => 0,
            PARW::ODD => 1,
            PARW::SPACE => 2,
            PARW::MARK => 3,
            PARW::NO => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARW<'a> {
    w: &'a mut W,
}
impl<'a> _PARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Even Parity"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(PARW::EVEN)
    }
    #[doc = "Odd Parity"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARW::ODD)
    }
    #[doc = "Space: parity forced to 0"]
    #[inline]
    pub fn space(self) -> &'a mut W {
        self.variant(PARW::SPACE)
    }
    #[doc = "Mark: parity forced to 1"]
    #[inline]
    pub fn mark(self) -> &'a mut W {
        self.variant(PARW::MARK)
    }
    #[doc = "No Parity"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(PARW::NO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHMODE`"]
pub enum CHMODEW {
    #[doc = "Normal Mode"]
    NORMAL,
    #[doc = "Automatic Echo"]
    AUTOMATIC,
    #[doc = "Local Loopback"]
    LOCAL_LOOPBACK,
    #[doc = "Remote Loopback"]
    REMOTE_LOOPBACK,
}
impl CHMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHMODEW::NORMAL => 0,
            CHMODEW::AUTOMATIC => 1,
            CHMODEW::LOCAL_LOOPBACK => 2,
            CHMODEW::REMOTE_LOOPBACK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal Mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODEW::NORMAL)
    }
    #[doc = "Automatic Echo"]
    #[inline]
    pub fn automatic(self) -> &'a mut W {
        self.variant(CHMODEW::AUTOMATIC)
    }
    #[doc = "Local Loopback"]
    #[inline]
    pub fn local_loopback(self) -> &'a mut W {
        self.variant(CHMODEW::LOCAL_LOOPBACK)
    }
    #[doc = "Remote Loopback"]
    #[inline]
    pub fn remote_loopback(self) -> &'a mut W {
        self.variant(CHMODEW::REMOTE_LOOPBACK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
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
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline]
    pub fn par(&self) -> PARR {
        PARR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline]
    pub fn chmode(&self) -> CHMODER {
        CHMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
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
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline]
    pub fn par(&mut self) -> _PARW {
        _PARW { w: self }
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline]
    pub fn chmode(&mut self) -> _CHMODEW {
        _CHMODEW { w: self }
    }
}
