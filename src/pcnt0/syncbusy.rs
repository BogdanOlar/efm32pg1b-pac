///Register `SYNCBUSY` reader
pub type R = crate::R<SYNCBUSYrs>;
///Field `CTRL` reader - CTRL Register Busy
pub type CtrlR = crate::BitReader;
///Field `CMD` reader - CMD Register Busy
pub type CmdR = crate::BitReader;
///Field `TOPB` reader - TOPB Register Busy
pub type TopbR = crate::BitReader;
///Field `OVSCFG` reader - OVSCFG Register Busy
pub type OvscfgR = crate::BitReader;
impl R {
    ///Bit 0 - CTRL Register Busy
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CMD Register Busy
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TOPB Register Busy
    #[inline(always)]
    pub fn topb(&self) -> TopbR {
        TopbR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - OVSCFG Register Busy
    #[inline(always)]
    pub fn ovscfg(&self) -> OvscfgR {
        OvscfgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCBUSY")
            .field("ctrl", &self.ctrl())
            .field("cmd", &self.cmd())
            .field("topb", &self.topb())
            .field("ovscfg", &self.ovscfg())
            .finish()
    }
}
///Synchronization Busy Register
///
///You can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SYNCBUSYrs;
impl crate::RegisterSpec for SYNCBUSYrs {
    type Ux = u32;
}
///`read()` method returns [`syncbusy::R`](R) reader structure
impl crate::Readable for SYNCBUSYrs {}
///`reset()` method sets SYNCBUSY to value 0
impl crate::Resettable for SYNCBUSYrs {
    const RESET_VALUE: u32 = 0;
}
