#[doc = "Register `APORTCONFLICT` reader"]
pub type R = crate::R<APORTCONFLICT_SPEC>;
#[doc = "Field `APORT1XCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
pub type APORT1XCONFLICT_R = crate::BitReader;
#[doc = "Field `APORT1YCONFLICT` reader - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
pub type APORT1YCONFLICT_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1xconflict(&self) -> APORT1XCONFLICT_R {
        APORT1XCONFLICT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1yconflict(&self) -> APORT1YCONFLICT_R {
        APORT1YCONFLICT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APORTCONFLICT")
            .field(
                "aport1xconflict",
                &format_args!("{}", self.aport1xconflict().bit()),
            )
            .field(
                "aport1yconflict",
                &format_args!("{}", self.aport1yconflict().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<APORTCONFLICT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "APORT Request Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aportconflict::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APORTCONFLICT_SPEC;
impl crate::RegisterSpec for APORTCONFLICT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportconflict::R`](R) reader structure"]
impl crate::Readable for APORTCONFLICT_SPEC {}
#[doc = "`reset()` method sets APORTCONFLICT to value 0"]
impl crate::Resettable for APORTCONFLICT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
