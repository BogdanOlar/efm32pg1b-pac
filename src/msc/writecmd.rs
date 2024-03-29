#[doc = "Register `WRITECMD` writer"]
pub type W = crate::W<WRITECMDrs>;
#[doc = "Field `LADDRIM` writer - Load MSC_ADDRB Into ADDR"]
pub type LADDRIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEPAGE` writer - Erase Page"]
pub type ERASEPAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEEND` writer - End Write Mode"]
pub type WRITEEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITEONCE` writer - Word Write-Once Trigger"]
pub type WRITEONCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITETRIG` writer - Word Write Sequence Trigger"]
pub type WRITETRIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEABORT` writer - Abort Erase Sequence"]
pub type ERASEABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERASEMAIN0` writer - Mass Erase Region 0"]
pub type ERASEMAIN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARWDATA` writer - Clear WDATA State"]
pub type CLEARWDATA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Load MSC_ADDRB Into ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn laddrim(&mut self) -> LADDRIM_W<WRITECMDrs> {
        LADDRIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Erase Page"]
    #[inline(always)]
    #[must_use]
    pub fn erasepage(&mut self) -> ERASEPAGE_W<WRITECMDrs> {
        ERASEPAGE_W::new(self, 1)
    }
    #[doc = "Bit 2 - End Write Mode"]
    #[inline(always)]
    #[must_use]
    pub fn writeend(&mut self) -> WRITEEND_W<WRITECMDrs> {
        WRITEEND_W::new(self, 2)
    }
    #[doc = "Bit 3 - Word Write-Once Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn writeonce(&mut self) -> WRITEONCE_W<WRITECMDrs> {
        WRITEONCE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Word Write Sequence Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn writetrig(&mut self) -> WRITETRIG_W<WRITECMDrs> {
        WRITETRIG_W::new(self, 4)
    }
    #[doc = "Bit 5 - Abort Erase Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn eraseabort(&mut self) -> ERASEABORT_W<WRITECMDrs> {
        ERASEABORT_W::new(self, 5)
    }
    #[doc = "Bit 8 - Mass Erase Region 0"]
    #[inline(always)]
    #[must_use]
    pub fn erasemain0(&mut self) -> ERASEMAIN0_W<WRITECMDrs> {
        ERASEMAIN0_W::new(self, 8)
    }
    #[doc = "Bit 12 - Clear WDATA State"]
    #[inline(always)]
    #[must_use]
    pub fn clearwdata(&mut self) -> CLEARWDATA_W<WRITECMDrs> {
        CLEARWDATA_W::new(self, 12)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Write Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writecmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRITECMDrs;
impl crate::RegisterSpec for WRITECMDrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`writecmd::W`](W) writer structure"]
impl crate::Writable for WRITECMDrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITECMD to value 0"]
impl crate::Resettable for WRITECMDrs {
    const RESET_VALUE: u32 = 0;
}
