#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Interrupt Flag"]
pub type AportconflictR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - APORT Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&self) -> AportconflictR {
        AportconflictR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
