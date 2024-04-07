#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `APORTCONFLICT` writer - Clear APORTCONFLICT Interrupt Flag"]
pub type AportconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Clear APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn aportconflict(&mut self) -> AportconflictW<IFCrs> {
        AportconflictW::new(self, 1)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
