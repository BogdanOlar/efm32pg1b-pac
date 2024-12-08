///Register `DTFAULTC` writer
pub type W = crate::W<DTFAULTCrs>;
///Field `DTPRS0FC` writer - DTI PRS0 Fault Clear
pub type Dtprs0fcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTPRS1FC` writer - DTI PRS1 Fault Clear
pub type Dtprs1fcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DTDBGFC` writer - DTI Debugger Fault Clear
pub type DtdbgfcW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TLOCKUPFC` writer - DTI Lockup Fault Clear
pub type TlockupfcW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DTFAULTCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - DTI PRS0 Fault Clear
    #[inline(always)]
    #[must_use]
    pub fn dtprs0fc(&mut self) -> Dtprs0fcW<DTFAULTCrs> {
        Dtprs0fcW::new(self, 0)
    }
    ///Bit 1 - DTI PRS1 Fault Clear
    #[inline(always)]
    #[must_use]
    pub fn dtprs1fc(&mut self) -> Dtprs1fcW<DTFAULTCrs> {
        Dtprs1fcW::new(self, 1)
    }
    ///Bit 2 - DTI Debugger Fault Clear
    #[inline(always)]
    #[must_use]
    pub fn dtdbgfc(&mut self) -> DtdbgfcW<DTFAULTCrs> {
        DtdbgfcW::new(self, 2)
    }
    ///Bit 3 - DTI Lockup Fault Clear
    #[inline(always)]
    #[must_use]
    pub fn tlockupfc(&mut self) -> TlockupfcW<DTFAULTCrs> {
        TlockupfcW::new(self, 3)
    }
}
///DTI Fault Clear Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtfaultc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DTFAULTCrs;
impl crate::RegisterSpec for DTFAULTCrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dtfaultc::W`](W) writer structure
impl crate::Writable for DTFAULTCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DTFAULTC to value 0
impl crate::Resettable for DTFAULTCrs {
    const RESET_VALUE: u32 = 0;
}
