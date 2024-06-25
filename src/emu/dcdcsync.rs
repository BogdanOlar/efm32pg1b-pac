#[doc = "Register `DCDCSYNC` reader"]
pub type R = crate::R<DCDCSYNCrs>;
#[doc = "Field `DCDCCTRLBUSY` reader - DCDC CTRL Register Transfer Busy"]
pub type DcdcctrlbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DCDC CTRL Register Transfer Busy"]
    #[inline(always)]
    pub fn dcdcctrlbusy(&self) -> DcdcctrlbusyR {
        DcdcctrlbusyR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCSYNC")
            .field("dcdcctrlbusy", &self.dcdcctrlbusy())
            .finish()
    }
}
#[doc = "DCDC Read Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdcsync::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCSYNCrs;
impl crate::RegisterSpec for DCDCSYNCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcsync::R`](R) reader structure"]
impl crate::Readable for DCDCSYNCrs {}
#[doc = "`reset()` method sets DCDCSYNC to value 0"]
impl crate::Resettable for DCDCSYNCrs {
    const RESET_VALUE: u32 = 0;
}
