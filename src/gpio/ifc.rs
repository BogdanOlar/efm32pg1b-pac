#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `EXT` writer - Clear EXT Interrupt Flag"]
pub type ExtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EM4WU` writer - Clear EM4WU Interrupt Flag"]
pub type Em4wuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Clear EXT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> ExtW<IFCrs> {
        ExtW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Clear EM4WU Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu(&mut self) -> Em4wuW<IFCrs> {
        Em4wuW::new(self, 16)
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
