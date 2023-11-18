#[doc = "Register `DCDCSYNC` reader"]
pub type R = crate::R<DCDCSYNC_SPEC>;
#[doc = "Field `DCDCCTRLBUSY` reader - DCDC CTRL Register Transfer Busy"]
pub type DCDCCTRLBUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DCDC CTRL Register Transfer Busy"]
    #[inline(always)]
    pub fn dcdcctrlbusy(&self) -> DCDCCTRLBUSY_R {
        DCDCCTRLBUSY_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCSYNC")
            .field(
                "dcdcctrlbusy",
                &format_args!("{}", self.dcdcctrlbusy().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DCDCSYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DCDC Read Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcdcsync::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCDCSYNC_SPEC;
impl crate::RegisterSpec for DCDCSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdcsync::R`](R) reader structure"]
impl crate::Readable for DCDCSYNC_SPEC {}
#[doc = "`reset()` method sets DCDCSYNC to value 0"]
impl crate::Resettable for DCDCSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
