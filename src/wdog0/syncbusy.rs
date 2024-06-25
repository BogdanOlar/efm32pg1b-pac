#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SYNCBUSYrs>;
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CtrlR = crate::BitReader;
#[doc = "Field `CMD` reader - CMD Register Busy"]
pub type CmdR = crate::BitReader;
#[doc = "Field `PCH0_PRSCTRL` reader - PCH0_PRSCTRL Register Busy"]
pub type Pch0PrsctrlR = crate::BitReader;
#[doc = "Field `PCH1_PRSCTRL` reader - PCH1_PRSCTRL Register Busy"]
pub type Pch1PrsctrlR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CtrlR {
        CtrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD Register Busy"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCH0_PRSCTRL Register Busy"]
    #[inline(always)]
    pub fn pch0_prsctrl(&self) -> Pch0PrsctrlR {
        Pch0PrsctrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCH1_PRSCTRL Register Busy"]
    #[inline(always)]
    pub fn pch1_prsctrl(&self) -> Pch1PrsctrlR {
        Pch1PrsctrlR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNCBUSY")
            .field("ctrl", &self.ctrl())
            .field("cmd", &self.cmd())
            .field("pch0_prsctrl", &self.pch0_prsctrl())
            .field("pch1_prsctrl", &self.pch1_prsctrl())
            .finish()
    }
}
#[doc = "Synchronization Busy Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNCBUSYrs;
impl crate::RegisterSpec for SYNCBUSYrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SYNCBUSYrs {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSYrs {
    const RESET_VALUE: u32 = 0;
}
