#[doc = "Register `WRITECMD` writer"]
pub type W = crate::W<WRITECMDrs>;
#[doc = "Field `LADDRIM` writer - Load MSC_ADDRB Into ADDR"]
pub type LaddrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEPAGE` writer - Erase Page"]
pub type ErasepageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEEND` writer - End Write Mode"]
pub type WriteendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEONCE` writer - Word Write-Once Trigger"]
pub type WriteonceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITETRIG` writer - Word Write Sequence Trigger"]
pub type WritetrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEABORT` writer - Abort Erase Sequence"]
pub type EraseabortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEMAIN0` writer - Mass Erase Region 0"]
pub type Erasemain0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARWDATA` writer - Clear WDATA State"]
pub type ClearwdataW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<WRITECMDrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Load MSC_ADDRB Into ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn laddrim(&mut self) -> LaddrimW<WRITECMDrs> {
        LaddrimW::new(self, 0)
    }
    #[doc = "Bit 1 - Erase Page"]
    #[inline(always)]
    #[must_use]
    pub fn erasepage(&mut self) -> ErasepageW<WRITECMDrs> {
        ErasepageW::new(self, 1)
    }
    #[doc = "Bit 2 - End Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn writeend(&mut self) -> WriteendW<WRITECMDrs> {
        WriteendW::new(self, 2)
    }
    #[doc = "Bit 3 - Word Write-Once Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn writeonce(&mut self) -> WriteonceW<WRITECMDrs> {
        WriteonceW::new(self, 3)
    }
    #[doc = "Bit 4 - Word Write Sequence Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn writetrig(&mut self) -> WritetrigW<WRITECMDrs> {
        WritetrigW::new(self, 4)
    }
    #[doc = "Bit 5 - Abort Erase Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eraseabort(&mut self) -> EraseabortW<WRITECMDrs> {
        EraseabortW::new(self, 5)
    }
    #[doc = "Bit 8 - Mass Erase Region 0"]
    #[inline(always)]
    #[must_use]
    pub fn erasemain0(&mut self) -> Erasemain0W<WRITECMDrs> {
        Erasemain0W::new(self, 8)
    }
    #[doc = "Bit 12 - Clear WDATA State"]
    #[inline(always)]
    #[must_use]
    pub fn clearwdata(&mut self) -> ClearwdataW<WRITECMDrs> {
        ClearwdataW::new(self, 12)
    }
}
#[doc = "Write Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writecmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRITECMDrs;
impl crate::RegisterSpec for WRITECMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`writecmd::W`](W) writer structure"]
impl crate::Writable for WRITECMDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITECMD to value 0"]
impl crate::Resettable for WRITECMDrs {
    const RESET_VALUE: u32 = 0;
}
