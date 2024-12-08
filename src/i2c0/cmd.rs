///Register `CMD` writer
pub type W = crate::W<CMDrs>;
///Field `START` writer - Send Start Condition
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOP` writer - Send Stop Condition
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACK` writer - Send ACK
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACK` writer - Send NACK
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONT` writer - Continue Transmission
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ABORT` writer - Abort Transmission
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEARTX` writer - Clear TX
pub type CleartxW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEARPC` writer - Clear Pending Commands
pub type ClearpcW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<CMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Send Start Condition
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CMDrs> {
        StartW::new(self, 0)
    }
    ///Bit 1 - Send Stop Condition
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> StopW<CMDrs> {
        StopW::new(self, 1)
    }
    ///Bit 2 - Send ACK
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<CMDrs> {
        AckW::new(self, 2)
    }
    ///Bit 3 - Send NACK
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NackW<CMDrs> {
        NackW::new(self, 3)
    }
    ///Bit 4 - Continue Transmission
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> ContW<CMDrs> {
        ContW::new(self, 4)
    }
    ///Bit 5 - Abort Transmission
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> AbortW<CMDrs> {
        AbortW::new(self, 5)
    }
    ///Bit 6 - Clear TX
    #[inline(always)]
    #[must_use]
    pub fn cleartx(&mut self) -> CleartxW<CMDrs> {
        CleartxW::new(self, 6)
    }
    ///Bit 7 - Clear Pending Commands
    #[inline(always)]
    #[must_use]
    pub fn clearpc(&mut self) -> ClearpcW<CMDrs> {
        ClearpcW::new(self, 7)
    }
}
///Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CMDrs;
impl crate::RegisterSpec for CMDrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cmd::W`](W) writer structure
impl crate::Writable for CMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMD to value 0
impl crate::Resettable for CMDrs {
    const RESET_VALUE: u32 = 0;
}
