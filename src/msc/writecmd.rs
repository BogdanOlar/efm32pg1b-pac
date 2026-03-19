///Register `WRITECMD` writer
pub type W = crate::W<WRITECMDrs>;
///Field `LADDRIM` writer - Load MSC_ADDRB Into ADDR
pub type LaddrimW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERASEPAGE` writer - Erase Page
pub type ErasepageW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITEEND` writer - End Write Mode
pub type WriteendW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITEONCE` writer - Word Write-Once Trigger
pub type WriteonceW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WRITETRIG` writer - Word Write Sequence Trigger
pub type WritetrigW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERASEABORT` writer - Abort Erase Sequence
pub type EraseabortW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERASEMAIN0` writer - Mass Erase Region 0
pub type Erasemain0W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLEARWDATA` writer - Clear WDATA State
pub type ClearwdataW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<WRITECMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Load MSC_ADDRB Into ADDR
    #[inline(always)]
    pub fn laddrim(&mut self) -> LaddrimW<'_, WRITECMDrs> {
        LaddrimW::new(self, 0)
    }
    ///Bit 1 - Erase Page
    #[inline(always)]
    pub fn erasepage(&mut self) -> ErasepageW<'_, WRITECMDrs> {
        ErasepageW::new(self, 1)
    }
    ///Bit 2 - End Write Mode
    #[inline(always)]
    pub fn writeend(&mut self) -> WriteendW<'_, WRITECMDrs> {
        WriteendW::new(self, 2)
    }
    ///Bit 3 - Word Write-Once Trigger
    #[inline(always)]
    pub fn writeonce(&mut self) -> WriteonceW<'_, WRITECMDrs> {
        WriteonceW::new(self, 3)
    }
    ///Bit 4 - Word Write Sequence Trigger
    #[inline(always)]
    pub fn writetrig(&mut self) -> WritetrigW<'_, WRITECMDrs> {
        WritetrigW::new(self, 4)
    }
    ///Bit 5 - Abort Erase Sequence
    #[inline(always)]
    pub fn eraseabort(&mut self) -> EraseabortW<'_, WRITECMDrs> {
        EraseabortW::new(self, 5)
    }
    ///Bit 8 - Mass Erase Region 0
    #[inline(always)]
    pub fn erasemain0(&mut self) -> Erasemain0W<'_, WRITECMDrs> {
        Erasemain0W::new(self, 8)
    }
    ///Bit 12 - Clear WDATA State
    #[inline(always)]
    pub fn clearwdata(&mut self) -> ClearwdataW<'_, WRITECMDrs> {
        ClearwdataW::new(self, 12)
    }
}
///Write Command Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writecmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WRITECMDrs;
impl crate::RegisterSpec for WRITECMDrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`writecmd::W`](W) writer structure
impl crate::Writable for WRITECMDrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WRITECMD to value 0
impl crate::Resettable for WRITECMDrs {}
