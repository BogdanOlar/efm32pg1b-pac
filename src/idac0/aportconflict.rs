#[doc = "Register `APORTCONFLICT` reader"]
pub type R = crate::R<APORTCONFLICTrs>;
#[doc = "Field `APORT1XCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
pub type Aport1xconflictR = crate::BitReader;
#[doc = "Field `APORT1YCONFLICT` reader - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
pub type Aport1yconflictR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1xconflict(&self) -> Aport1xconflictR {
        Aport1xconflictR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1yconflict(&self) -> Aport1yconflictR {
        Aport1yconflictR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APORTCONFLICT")
            .field("aport1xconflict", &self.aport1xconflict())
            .field("aport1yconflict", &self.aport1yconflict())
            .finish()
    }
}
#[doc = "APORT Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aportconflict::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APORTCONFLICTrs;
impl crate::RegisterSpec for APORTCONFLICTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aportconflict::R`](R) reader structure"]
impl crate::Readable for APORTCONFLICTrs {}
#[doc = "`reset()` method sets APORTCONFLICT to value 0"]
impl crate::Resettable for APORTCONFLICTrs {
    const RESET_VALUE: u32 = 0;
}
