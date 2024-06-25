#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `EDGE` reader - Edge Triggered Interrupt Flag"]
pub type EdgeR = crate::BitReader;
#[doc = "Field `WARMUP` reader - Warm-up Interrupt Flag"]
pub type WarmupR = crate::BitReader;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Interrupt Flag"]
pub type AportconflictR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Warm-up Interrupt Flag"]
    #[inline(always)]
    pub fn warmup(&self) -> WarmupR {
        WarmupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APORT Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&self) -> AportconflictR {
        AportconflictR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("edge", &self.edge())
            .field("warmup", &self.warmup())
            .field("aportconflict", &self.aportconflict())
            .finish()
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IFrs {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
