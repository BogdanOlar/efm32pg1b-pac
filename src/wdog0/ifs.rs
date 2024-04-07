#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFSrs>;
#[doc = "Field `TOUT` writer - Set TOUT Interrupt Flag"]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARN` writer - Set WARN Interrupt Flag"]
pub type WarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN` writer - Set WIN Interrupt Flag"]
pub type WinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM0` writer - Set PEM0 Interrupt Flag"]
pub type Pem0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM1` writer - Set PEM1 Interrupt Flag"]
pub type Pem1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Set TOUT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> ToutW<IFSrs> {
        ToutW::new(self, 0)
    }
    #[doc = "Bit 1 - Set WARN Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WarnW<IFSrs> {
        WarnW::new(self, 1)
    }
    #[doc = "Bit 2 - Set WIN Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WinW<IFSrs> {
        WinW::new(self, 2)
    }
    #[doc = "Bit 3 - Set PEM0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pem0(&mut self) -> Pem0W<IFSrs> {
        Pem0W::new(self, 3)
    }
    #[doc = "Bit 4 - Set PEM1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pem1(&mut self) -> Pem1W<IFSrs> {
        Pem1W::new(self, 4)
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
