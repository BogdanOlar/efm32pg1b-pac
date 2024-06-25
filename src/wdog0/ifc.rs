#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `TOUT` writer - Clear TOUT Interrupt Flag"]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARN` writer - Clear WARN Interrupt Flag"]
pub type WarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN` writer - Clear WIN Interrupt Flag"]
pub type WinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM0` writer - Clear PEM0 Interrupt Flag"]
pub type Pem0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM1` writer - Clear PEM1 Interrupt Flag"]
pub type Pem1W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear TOUT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> ToutW<IFCrs> {
        ToutW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear WARN Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WarnW<IFCrs> {
        WarnW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear WIN Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WinW<IFCrs> {
        WinW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear PEM0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pem0(&mut self) -> Pem0W<IFCrs> {
        Pem0W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear PEM1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pem1(&mut self) -> Pem1W<IFCrs> {
        Pem1W::new(self, 4)
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
