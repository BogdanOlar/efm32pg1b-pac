#[doc = "Register `APORTREQ` reader"]
pub type R = crate::R<APORTREQrs>;
#[doc = "Field `APORT1XREQ` reader - 1 If the APORT Bus Connected to APORT1X is Requested"]
pub type Aport1xreqR = crate::BitReader;
#[doc = "Field `APORT1YREQ` reader - 1 If the Bus Connected to APORT1Y is Requested"]
pub type Aport1yreqR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - 1 If the APORT Bus Connected to APORT1X is Requested"]
    #[inline(always)]
    pub fn aport1xreq(&self) -> Aport1xreqR {
        Aport1xreqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is Requested"]
    #[inline(always)]
    pub fn aport1yreq(&self) -> Aport1yreqR {
        Aport1yreqR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APORTREQ")
            .field("aport1xreq", &self.aport1xreq())
            .field("aport1yreq", &self.aport1yreq())
            .finish()
    }
}
#[doc = "APORT Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aportreq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APORTREQrs;
impl crate::RegisterSpec for APORTREQrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportreq::R`](R) reader structure"]
impl crate::Readable for APORTREQrs {}
#[doc = "`reset()` method sets APORTREQ to value 0"]
impl crate::Resettable for APORTREQrs {
    const RESET_VALUE: u32 = 0;
}
