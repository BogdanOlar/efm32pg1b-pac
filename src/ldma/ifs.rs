#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `DONE` writer - Set DONE Interrupt Flag"]
pub type DoneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ERROR` writer - Set ERROR Interrupt Flag"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - Set DONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IFSrs> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 31 - Set ERROR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IFSrs> {
        ErrorW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
