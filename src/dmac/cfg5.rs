#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG5 {
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
pub struct SRC_PERR {
    bits: u8,
}
impl SRC_PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DST_PERR {
    bits: u8,
}
impl DST_PERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SRC_H2SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_H2SELR {
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    SW,
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    HW,
}
impl SRC_H2SELR {
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
            SRC_H2SELR::SW => false,
            SRC_H2SELR::HW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRC_H2SELR {
        match value {
            false => SRC_H2SELR::SW,
            true => SRC_H2SELR::HW,
        }
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline]
    pub fn is_sw(&self) -> bool {
        *self == SRC_H2SELR::SW
    }
    #[doc = "Checks if the value of the field is `HW`"]
    #[inline]
    pub fn is_hw(&self) -> bool {
        *self == SRC_H2SELR::HW
    }
}
#[doc = "Possible values of the field `DST_H2SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_H2SELR {
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    SW,
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    HW,
}
impl DST_H2SELR {
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
            DST_H2SELR::SW => false,
            DST_H2SELR::HW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DST_H2SELR {
        match value {
            false => DST_H2SELR::SW,
            true => DST_H2SELR::HW,
        }
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline]
    pub fn is_sw(&self) -> bool {
        *self == DST_H2SELR::SW
    }
    #[doc = "Checks if the value of the field is `HW`"]
    #[inline]
    pub fn is_hw(&self) -> bool {
        *self == DST_H2SELR::HW
    }
}
#[doc = "Possible values of the field `SOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SODR {
    #[doc = "STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    DISABLE,
    #[doc = "STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    ENABLE,
}
impl SODR {
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
            SODR::DISABLE => false,
            SODR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SODR {
        match value {
            false => SODR::DISABLE,
            true => SODR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == SODR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SODR::ENABLE
    }
}
#[doc = "Possible values of the field `LOCK_IF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_IFR {
    #[doc = "Interface Lock capability is disabled"]
    DISABLE,
    #[doc = "Interface Lock capability is enabled"]
    ENABLE,
}
impl LOCK_IFR {
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
            LOCK_IFR::DISABLE => false,
            LOCK_IFR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_IFR {
        match value {
            false => LOCK_IFR::DISABLE,
            true => LOCK_IFR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == LOCK_IFR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LOCK_IFR::ENABLE
    }
}
#[doc = "Possible values of the field `LOCK_B`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_BR {
    #[doc = "AHB Bus Locking capability is disabled."]
    DISABLE,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LOCK_BR {
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
            LOCK_BR::DISABLE => false,
            LOCK_BR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_BR {
        match value {
            false => LOCK_BR::DISABLE,
            i => LOCK_BR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == LOCK_BR::DISABLE
    }
}
#[doc = "Possible values of the field `LOCK_IF_L`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_IF_LR {
    #[doc = "The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    CHUNK,
    #[doc = "The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    BUFFER,
}
impl LOCK_IF_LR {
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
            LOCK_IF_LR::CHUNK => false,
            LOCK_IF_LR::BUFFER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCK_IF_LR {
        match value {
            false => LOCK_IF_LR::CHUNK,
            true => LOCK_IF_LR::BUFFER,
        }
    }
    #[doc = "Checks if the value of the field is `CHUNK`"]
    #[inline]
    pub fn is_chunk(&self) -> bool {
        *self == LOCK_IF_LR::CHUNK
    }
    #[doc = "Checks if the value of the field is `BUFFER`"]
    #[inline]
    pub fn is_buffer(&self) -> bool {
        *self == LOCK_IF_LR::BUFFER
    }
}
#[doc = r" Value of the field"]
pub struct AHB_PROTR {
    bits: u8,
}
impl AHB_PROTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FIFOCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOCFGR {
    #[doc = "The largest defined length AHB burst is performed on the destination AHB interface."]
    ALAP_CFG,
    #[doc = "When half FIFO size is available/filled, a source/destination request is serviced."]
    HALF_CFG,
    #[doc = "When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    ASAP_CFG,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FIFOCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FIFOCFGR::ALAP_CFG => 0,
            FIFOCFGR::HALF_CFG => 1,
            FIFOCFGR::ASAP_CFG => 2,
            FIFOCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FIFOCFGR {
        match value {
            0 => FIFOCFGR::ALAP_CFG,
            1 => FIFOCFGR::HALF_CFG,
            2 => FIFOCFGR::ASAP_CFG,
            i => FIFOCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALAP_CFG`"]
    #[inline]
    pub fn is_alap_cfg(&self) -> bool {
        *self == FIFOCFGR::ALAP_CFG
    }
    #[doc = "Checks if the value of the field is `HALF_CFG`"]
    #[inline]
    pub fn is_half_cfg(&self) -> bool {
        *self == FIFOCFGR::HALF_CFG
    }
    #[doc = "Checks if the value of the field is `ASAP_CFG`"]
    #[inline]
    pub fn is_asap_cfg(&self) -> bool {
        *self == FIFOCFGR::ASAP_CFG
    }
}
#[doc = r" Proxy"]
pub struct _SRC_PERW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_PERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DST_PERW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_PERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC_H2SEL`"]
pub enum SRC_H2SELW {
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    SW,
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    HW,
}
impl SRC_H2SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRC_H2SELW::SW => false,
            SRC_H2SELW::HW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_H2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_H2SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_H2SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline]
    pub fn sw(self) -> &'a mut W {
        self.variant(SRC_H2SELW::SW)
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline]
    pub fn hw(self) -> &'a mut W {
        self.variant(SRC_H2SELW::HW)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DST_H2SEL`"]
pub enum DST_H2SELW {
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    SW,
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    HW,
}
impl DST_H2SELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DST_H2SELW::SW => false,
            DST_H2SELW::HW => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DST_H2SELW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_H2SELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_H2SELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software handshaking interface is used to trigger a transfer request."]
    #[inline]
    pub fn sw(self) -> &'a mut W {
        self.variant(DST_H2SELW::SW)
    }
    #[doc = "Hardware handshaking interface is used to trigger a transfer request."]
    #[inline]
    pub fn hw(self) -> &'a mut W {
        self.variant(DST_H2SELW::HW)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SOD`"]
pub enum SODW {
    #[doc = "STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    DISABLE,
    #[doc = "STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    ENABLE,
}
impl SODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SODW::DISABLE => false,
            SODW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SODW<'a> {
    w: &'a mut W,
}
impl<'a> _SODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SODW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "STOP ON DONE disabled, the descriptor fetch operation ignores DONE Field of CTRLA register."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(SODW::DISABLE)
    }
    #[doc = "STOP ON DONE activated, the DMAC module is automatically disabled if DONE FIELD is set to 1."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SODW::ENABLE)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK_IF`"]
pub enum LOCK_IFW {
    #[doc = "Interface Lock capability is disabled"]
    DISABLE,
    #[doc = "Interface Lock capability is enabled"]
    ENABLE,
}
impl LOCK_IFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_IFW::DISABLE => false,
            LOCK_IFW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_IFW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_IFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_IFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interface Lock capability is disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOCK_IFW::DISABLE)
    }
    #[doc = "Interface Lock capability is enabled"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LOCK_IFW::ENABLE)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK_B`"]
pub enum LOCK_BW {
    #[doc = "AHB Bus Locking capability is disabled."]
    DISABLE,
}
impl LOCK_BW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_BW::DISABLE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_BW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_BW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_BW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AHB Bus Locking capability is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(LOCK_BW::DISABLE)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOCK_IF_L`"]
pub enum LOCK_IF_LW {
    #[doc = "The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    CHUNK,
    #[doc = "The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    BUFFER,
}
impl LOCK_IF_LW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCK_IF_LW::CHUNK => false,
            LOCK_IF_LW::BUFFER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCK_IF_LW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_IF_LW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCK_IF_LW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a chunk transfer."]
    #[inline]
    pub fn chunk(self) -> &'a mut W {
        self.variant(LOCK_IF_LW::CHUNK)
    }
    #[doc = "The Master Interface Arbiter is locked by the channel x for a buffer transfer."]
    #[inline]
    pub fn buffer(self) -> &'a mut W {
        self.variant(LOCK_IF_LW::BUFFER)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AHB_PROTW<'a> {
    w: &'a mut W,
}
impl<'a> _AHB_PROTW<'a> {
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
#[doc = "Values that can be written to the field `FIFOCFG`"]
pub enum FIFOCFGW {
    #[doc = "The largest defined length AHB burst is performed on the destination AHB interface."]
    ALAP_CFG,
    #[doc = "When half FIFO size is available/filled, a source/destination request is serviced."]
    HALF_CFG,
    #[doc = "When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    ASAP_CFG,
}
impl FIFOCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FIFOCFGW::ALAP_CFG => 0,
            FIFOCFGW::HALF_CFG => 1,
            FIFOCFGW::ASAP_CFG => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIFOCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _FIFOCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIFOCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The largest defined length AHB burst is performed on the destination AHB interface."]
    #[inline]
    pub fn alap_cfg(self) -> &'a mut W {
        self.variant(FIFOCFGW::ALAP_CFG)
    }
    #[doc = "When half FIFO size is available/filled, a source/destination request is serviced."]
    #[inline]
    pub fn half_cfg(self) -> &'a mut W {
        self.variant(FIFOCFGW::HALF_CFG)
    }
    #[doc = "When there is enough space/data available to perform a single AHB access, then the request is serviced."]
    #[inline]
    pub fn asap_cfg(self) -> &'a mut W {
        self.variant(FIFOCFGW::ASAP_CFG)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - Source with Peripheral identifier"]
    #[inline]
    pub fn src_per(&self) -> SRC_PERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SRC_PERR { bits }
    }
    #[doc = "Bits 4:7 - Destination with Peripheral identifier"]
    #[inline]
    pub fn dst_per(&self) -> DST_PERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DST_PERR { bits }
    }
    #[doc = "Bit 9 - Software or Hardware Selection for the Source"]
    #[inline]
    pub fn src_h2sel(&self) -> SRC_H2SELR {
        SRC_H2SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Software or Hardware Selection for the Destination"]
    #[inline]
    pub fn dst_h2sel(&self) -> DST_H2SELR {
        DST_H2SELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Stop On Done"]
    #[inline]
    pub fn sod(&self) -> SODR {
        SODR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Interface Lock"]
    #[inline]
    pub fn lock_if(&self) -> LOCK_IFR {
        LOCK_IFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Bus Lock"]
    #[inline]
    pub fn lock_b(&self) -> LOCK_BR {
        LOCK_BR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Master Interface Arbiter Lock"]
    #[inline]
    pub fn lock_if_l(&self) -> LOCK_IF_LR {
        LOCK_IF_LR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:26 - AHB Protection"]
    #[inline]
    pub fn ahb_prot(&self) -> AHB_PROTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AHB_PROTR { bits }
    }
    #[doc = "Bits 28:29 - FIFO Configuration"]
    #[inline]
    pub fn fifocfg(&self) -> FIFOCFGR {
        FIFOCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16777216 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Source with Peripheral identifier"]
    #[inline]
    pub fn src_per(&mut self) -> _SRC_PERW {
        _SRC_PERW { w: self }
    }
    #[doc = "Bits 4:7 - Destination with Peripheral identifier"]
    #[inline]
    pub fn dst_per(&mut self) -> _DST_PERW {
        _DST_PERW { w: self }
    }
    #[doc = "Bit 9 - Software or Hardware Selection for the Source"]
    #[inline]
    pub fn src_h2sel(&mut self) -> _SRC_H2SELW {
        _SRC_H2SELW { w: self }
    }
    #[doc = "Bit 13 - Software or Hardware Selection for the Destination"]
    #[inline]
    pub fn dst_h2sel(&mut self) -> _DST_H2SELW {
        _DST_H2SELW { w: self }
    }
    #[doc = "Bit 16 - Stop On Done"]
    #[inline]
    pub fn sod(&mut self) -> _SODW {
        _SODW { w: self }
    }
    #[doc = "Bit 20 - Interface Lock"]
    #[inline]
    pub fn lock_if(&mut self) -> _LOCK_IFW {
        _LOCK_IFW { w: self }
    }
    #[doc = "Bit 21 - Bus Lock"]
    #[inline]
    pub fn lock_b(&mut self) -> _LOCK_BW {
        _LOCK_BW { w: self }
    }
    #[doc = "Bit 22 - Master Interface Arbiter Lock"]
    #[inline]
    pub fn lock_if_l(&mut self) -> _LOCK_IF_LW {
        _LOCK_IF_LW { w: self }
    }
    #[doc = "Bits 24:26 - AHB Protection"]
    #[inline]
    pub fn ahb_prot(&mut self) -> _AHB_PROTW {
        _AHB_PROTW { w: self }
    }
    #[doc = "Bits 28:29 - FIFO Configuration"]
    #[inline]
    pub fn fifocfg(&mut self) -> _FIFOCFGW {
        _FIFOCFGW { w: self }
    }
}
