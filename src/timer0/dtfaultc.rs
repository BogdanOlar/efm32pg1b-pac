#[doc = "Register `DTFAULTC` writer"]
pub type W = crate::W<DTFAULTCrs>;
#[doc = "Field `DTPRS0FC` writer - DTI PRS0 Fault Clear"]
pub type Dtprs0fcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRS1FC` writer - DTI PRS1 Fault Clear"]
pub type Dtprs1fcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDBGFC` writer - DTI Debugger Fault Clear"]
pub type DtdbgfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLOCKUPFC` writer - DTI Lockup Fault Clear"]
pub type TlockupfcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DTI PRS0 Fault Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs0fc(&mut self) -> Dtprs0fcW<DTFAULTCrs> {
        Dtprs0fcW::new(self, 0)
    }
    #[doc = "Bit 1 - DTI PRS1 Fault Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtprs1fc(&mut self) -> Dtprs1fcW<DTFAULTCrs> {
        Dtprs1fcW::new(self, 1)
    }
    #[doc = "Bit 2 - DTI Debugger Fault Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtdbgfc(&mut self) -> DtdbgfcW<DTFAULTCrs> {
        DtdbgfcW::new(self, 2)
    }
    #[doc = "Bit 3 - DTI Lockup Fault Clear"]
    #[inline(always)]
    #[must_use]
    pub fn tlockupfc(&mut self) -> TlockupfcW<DTFAULTCrs> {
        TlockupfcW::new(self, 3)
    }
}
#[doc = "DTI Fault Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtfaultc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTFAULTCrs;
impl crate::RegisterSpec for DTFAULTCrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dtfaultc::W`](W) writer structure"]
impl crate::Writable for DTFAULTCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTFAULTC to value 0"]
impl crate::Resettable for DTFAULTCrs {
    const RESET_VALUE: u32 = 0;
}
