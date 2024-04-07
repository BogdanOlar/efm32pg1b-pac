#[doc = "Register `STATE` reader"]
pub type R = crate::R<STATErs>;
#[doc = "Field `BUSY` reader - Bus Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `MASTER` reader - Master"]
pub type MasterR = crate::BitReader;
#[doc = "Field `TRANSMITTER` reader - Transmitter"]
pub type TransmitterR = crate::BitReader;
#[doc = "Field `NACKED` reader - Nack Received"]
pub type NackedR = crate::BitReader;
#[doc = "Field `BUSHOLD` reader - Bus Held"]
pub type BusholdR = crate::BitReader;
#[doc = "Transmission State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE {
    #[doc = "0: No transmission is being performed."]
    Idle = 0,
    #[doc = "1: Waiting for idle. Will send a start condition as soon as the bus is idle."]
    Wait = 1,
    #[doc = "2: Start transmitted or received"]
    Start = 2,
    #[doc = "3: Address transmitted or received"]
    Addr = 3,
    #[doc = "4: Address ack/nack transmitted or received"]
    Addrack = 4,
    #[doc = "5: Data transmitted or received"]
    Data = 5,
    #[doc = "6: Data ack/nack transmitted or received"]
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
#[doc = "Field `STATE` reader - Transmission State"]
pub type StateR = crate::FieldReader<STATE>;
impl StateR {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No transmission is being performed."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE::Idle
    }
    #[doc = "Waiting for idle. Will send a start condition as soon as the bus is idle."]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == STATE::Wait
    }
    #[doc = "Start transmitted or received"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == STATE::Start
    }
    #[doc = "Address transmitted or received"]
    #[inline(always)]
    pub fn is_addr(&self) -> bool {
        *self == STATE::Addr
    }
    #[doc = "Address ack/nack transmitted or received"]
    #[inline(always)]
    pub fn is_addrack(&self) -> bool {
        *self == STATE::Addrack
    }
    #[doc = "Data transmitted or received"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == STATE::Data
    }
    #[doc = "Data ack/nack transmitted or received"]
    #[inline(always)]
    pub fn is_dataack(&self) -> bool {
        *self == STATE::Dataack
    }
}
impl R {
    #[doc = "Bit 0 - Bus Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter"]
    #[inline(always)]
    pub fn transmitter(&self) -> TransmitterR {
        TransmitterR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Nack Received"]
    #[inline(always)]
    pub fn nacked(&self) -> NackedR {
        NackedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Held"]
    #[inline(always)]
    pub fn bushold(&self) -> BusholdR {
        BusholdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Transmission State"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 5) & 7) as u8)
    }
}
#[doc = "State Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATErs;
impl crate::RegisterSpec for STATErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for STATErs {}
#[doc = "`reset()` method sets STATE to value 0x01"]
impl crate::Resettable for STATErs {
    const RESET_VALUE: u32 = 0x01;
}
