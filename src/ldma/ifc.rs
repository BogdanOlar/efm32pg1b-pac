#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `DONE` writer - Clear DONE Interrupt Flag"]
pub type DoneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ERROR` writer - Clear ERROR Interrupt Flag"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Clear DONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IFCrs> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 31 - Clear ERROR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IFCrs> {
        ErrorW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
