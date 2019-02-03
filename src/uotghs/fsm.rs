#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FSM {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `DRDSTATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRDSTATER {
    #[doc = "This is the start state for A-devices (when the ID pin is 0)"]
    A_IDLESTATE,
    #[doc = "In this state, the A-device waits for the voltage on VBus to rise above the A-device VBus Valid threshold (4.4 V)."]
    A_WAIT_VRISE,
    #[doc = "In this state, the A-device waits for the B-device to signal a connection."]
    A_WAIT_BCON,
    #[doc = "In this state, the A-device that operates in Host mode is operational."]
    A_HOST,
    #[doc = "The A-device operating as a host is in the suspend mode."]
    A_SUSPEND,
    #[doc = "The A-device operates as a peripheral."]
    A_PERIPHERAL,
    #[doc = "In this state, the A-device waits for the voltage on VBus to drop below the A-device Session Valid threshold (1.4 V)."]
    A_WAIT_VFALL,
    #[doc = "In this state, the A-device waits for recovery of the over-current condition that caused it to enter this state."]
    A_VBUS_ERR,
    #[doc = "In this state, the A-device waits for the data USB line to discharge (100 us)."]
    A_WAIT_DISCHARGE,
    #[doc = "This is the start state for B-device (when the ID pin is 1)."]
    B_IDLE,
    #[doc = "In this state, the B-device acts as the peripheral."]
    B_PERIPHERAL,
    #[doc = "In this state, the B-device is in suspend mode and waits until 3 ms before initiating the HNP protocol if requested."]
    B_WAIT_BEGIN_HNP,
    #[doc = "In this state, the B-device waits for the data USB line to discharge (100 us) before becoming Host."]
    B_WAIT_DISCHARGE,
    #[doc = "In this state, the B-device waits for the A-device to signal a connect before becoming B-Host."]
    B_WAIT_ACON,
    #[doc = "In this state, the B-device acts as the Host."]
    B_HOST,
    #[doc = "In this state, the B-device attempts to start a session using the SRP protocol."]
    B_SRP_INIT,
}
impl DRDSTATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRDSTATER::A_IDLESTATE => 0,
            DRDSTATER::A_WAIT_VRISE => 1,
            DRDSTATER::A_WAIT_BCON => 2,
            DRDSTATER::A_HOST => 3,
            DRDSTATER::A_SUSPEND => 4,
            DRDSTATER::A_PERIPHERAL => 5,
            DRDSTATER::A_WAIT_VFALL => 6,
            DRDSTATER::A_VBUS_ERR => 7,
            DRDSTATER::A_WAIT_DISCHARGE => 8,
            DRDSTATER::B_IDLE => 9,
            DRDSTATER::B_PERIPHERAL => 10,
            DRDSTATER::B_WAIT_BEGIN_HNP => 11,
            DRDSTATER::B_WAIT_DISCHARGE => 12,
            DRDSTATER::B_WAIT_ACON => 13,
            DRDSTATER::B_HOST => 14,
            DRDSTATER::B_SRP_INIT => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRDSTATER {
        match value {
            0 => DRDSTATER::A_IDLESTATE,
            1 => DRDSTATER::A_WAIT_VRISE,
            2 => DRDSTATER::A_WAIT_BCON,
            3 => DRDSTATER::A_HOST,
            4 => DRDSTATER::A_SUSPEND,
            5 => DRDSTATER::A_PERIPHERAL,
            6 => DRDSTATER::A_WAIT_VFALL,
            7 => DRDSTATER::A_VBUS_ERR,
            8 => DRDSTATER::A_WAIT_DISCHARGE,
            9 => DRDSTATER::B_IDLE,
            10 => DRDSTATER::B_PERIPHERAL,
            11 => DRDSTATER::B_WAIT_BEGIN_HNP,
            12 => DRDSTATER::B_WAIT_DISCHARGE,
            13 => DRDSTATER::B_WAIT_ACON,
            14 => DRDSTATER::B_HOST,
            15 => DRDSTATER::B_SRP_INIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `A_IDLESTATE`"]
    #[inline]
    pub fn is_a_idlestate(&self) -> bool {
        *self == DRDSTATER::A_IDLESTATE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VRISE`"]
    #[inline]
    pub fn is_a_wait_vrise(&self) -> bool {
        *self == DRDSTATER::A_WAIT_VRISE
    }
    #[doc = "Checks if the value of the field is `A_WAIT_BCON`"]
    #[inline]
    pub fn is_a_wait_bcon(&self) -> bool {
        *self == DRDSTATER::A_WAIT_BCON
    }
    #[doc = "Checks if the value of the field is `A_HOST`"]
    #[inline]
    pub fn is_a_host(&self) -> bool {
        *self == DRDSTATER::A_HOST
    }
    #[doc = "Checks if the value of the field is `A_SUSPEND`"]
    #[inline]
    pub fn is_a_suspend(&self) -> bool {
        *self == DRDSTATER::A_SUSPEND
    }
    #[doc = "Checks if the value of the field is `A_PERIPHERAL`"]
    #[inline]
    pub fn is_a_peripheral(&self) -> bool {
        *self == DRDSTATER::A_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `A_WAIT_VFALL`"]
    #[inline]
    pub fn is_a_wait_vfall(&self) -> bool {
        *self == DRDSTATER::A_WAIT_VFALL
    }
    #[doc = "Checks if the value of the field is `A_VBUS_ERR`"]
    #[inline]
    pub fn is_a_vbus_err(&self) -> bool {
        *self == DRDSTATER::A_VBUS_ERR
    }
    #[doc = "Checks if the value of the field is `A_WAIT_DISCHARGE`"]
    #[inline]
    pub fn is_a_wait_discharge(&self) -> bool {
        *self == DRDSTATER::A_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_IDLE`"]
    #[inline]
    pub fn is_b_idle(&self) -> bool {
        *self == DRDSTATER::B_IDLE
    }
    #[doc = "Checks if the value of the field is `B_PERIPHERAL`"]
    #[inline]
    pub fn is_b_peripheral(&self) -> bool {
        *self == DRDSTATER::B_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `B_WAIT_BEGIN_HNP`"]
    #[inline]
    pub fn is_b_wait_begin_hnp(&self) -> bool {
        *self == DRDSTATER::B_WAIT_BEGIN_HNP
    }
    #[doc = "Checks if the value of the field is `B_WAIT_DISCHARGE`"]
    #[inline]
    pub fn is_b_wait_discharge(&self) -> bool {
        *self == DRDSTATER::B_WAIT_DISCHARGE
    }
    #[doc = "Checks if the value of the field is `B_WAIT_ACON`"]
    #[inline]
    pub fn is_b_wait_acon(&self) -> bool {
        *self == DRDSTATER::B_WAIT_ACON
    }
    #[doc = "Checks if the value of the field is `B_HOST`"]
    #[inline]
    pub fn is_b_host(&self) -> bool {
        *self == DRDSTATER::B_HOST
    }
    #[doc = "Checks if the value of the field is `B_SRP_INIT`"]
    #[inline]
    pub fn is_b_srp_init(&self) -> bool {
        *self == DRDSTATER::B_SRP_INIT
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Dual Role Device State"]
    #[inline]
    pub fn drdstate(&self) -> DRDSTATER {
        DRDSTATER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
