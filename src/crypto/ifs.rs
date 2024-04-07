#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `INSTRDONE` writer - Set INSTRDONE Interrupt Flag"]
pub type InstrdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQDONE` writer - Set SEQDONE Interrupt Flag"]
pub type SeqdoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set INSTRDONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn instrdone(&mut self) -> InstrdoneW<IFSrs> {
        InstrdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Set SEQDONE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn seqdone(&mut self) -> SeqdoneW<IFSrs> {
        SeqdoneW::new(self, 1)
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
