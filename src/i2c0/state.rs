///Register `STATE` reader
pub type R = crate::R<STATErs>;
///Field `BUSY` reader - Bus Busy
pub type BusyR = crate::BitReader;
///Field `MASTER` reader - Master
pub type MasterR = crate::BitReader;
///Field `TRANSMITTER` reader - Transmitter
pub type TransmitterR = crate::BitReader;
///Field `NACKED` reader - Nack Received
pub type NackedR = crate::BitReader;
///Field `BUSHOLD` reader - Bus Held
pub type BusholdR = crate::BitReader;
///Transmission State
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE {
    ///0: No transmission is being performed.
    Idle = 0,
    ///1: Waiting for idle. Will send a start condition as soon as the bus is idle.
    Wait = 1,
    ///2: Start transmitted or received
    Start = 2,
    ///3: Address transmitted or received
    Addr = 3,
    ///4: Address ack/nack transmitted or received
    Addrack = 4,
    ///5: Data transmitted or received
    Data = 5,
    ///6: Data ack/nack transmitted or received
    Dataack = 6,
}
impl From<STATE> for u8 {
    #[inline(always)]
    fn from(variant: STATE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STATE {
    type Ux = u8;
}
impl crate::IsEnum for STATE {}
///Field `STATE` reader - Transmission State
pub type StateR = crate::FieldReader<STATE>;
impl StateR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STATE> {
        match self.bits {
            0 => Some(STATE::Idle),
            1 => Some(STATE::Wait),
            2 => Some(STATE::Start),
            3 => Some(STATE::Addr),
            4 => Some(STATE::Addrack),
            5 => Some(STATE::Data),
            6 => Some(STATE::Dataack),
            _ => None,
        }
    }
    ///No transmission is being performed.
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE::Idle
    }
    ///Waiting for idle. Will send a start condition as soon as the bus is idle.
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == STATE::Wait
    }
    ///Start transmitted or received
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STATE::Start
    }
    ///Address transmitted or received
    #[inline(always)]
    pub fn is_addr(&self) -> bool {
        *self == STATE::Addr
    }
    ///Address ack/nack transmitted or received
    #[inline(always)]
    pub fn is_addrack(&self) -> bool {
        *self == STATE::Addrack
    }
    ///Data transmitted or received
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == STATE::Data
    }
    ///Data ack/nack transmitted or received
    #[inline(always)]
    pub fn is_dataack(&self) -> bool {
        *self == STATE::Dataack
    }
}
impl R {
    ///Bit 0 - Bus Busy
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Master
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmitter
    #[inline(always)]
    pub fn transmitter(&self) -> TransmitterR {
        TransmitterR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Nack Received
    #[inline(always)]
    pub fn nacked(&self) -> NackedR {
        NackedR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Bus Held
    #[inline(always)]
    pub fn bushold(&self) -> BusholdR {
        BusholdR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Transmission State
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE")
            .field("busy", &self.busy())
            .field("master", &self.master())
            .field("transmitter", &self.transmitter())
            .field("nacked", &self.nacked())
            .field("bushold", &self.bushold())
            .field("state", &self.state())
            .finish()
    }
}
///State Register
///
///You can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATErs;
impl crate::RegisterSpec for STATErs {
    type Ux = u32;
}
///`read()` method returns [`state::R`](R) reader structure
impl crate::Readable for STATErs {}
///`reset()` method sets STATE to value 0x01
impl crate::Resettable for STATErs {
    const RESET_VALUE: u32 = 0x01;
}
