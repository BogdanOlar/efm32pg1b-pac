#[doc = "Register `IEN` reader"]
pub type R = crate::R<IENrs>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IENrs>;
#[doc = "Field `TOUT` reader - TOUT Interrupt Enable"]
pub type ToutR = crate::BitReader;
#[doc = "Field `TOUT` writer - TOUT Interrupt Enable"]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARN` reader - WARN Interrupt Enable"]
pub type WarnR = crate::BitReader;
#[doc = "Field `WARN` writer - WARN Interrupt Enable"]
pub type WarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN` reader - WIN Interrupt Enable"]
pub type WinR = crate::BitReader;
#[doc = "Field `WIN` writer - WIN Interrupt Enable"]
pub type WinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM0` reader - PEM0 Interrupt Enable"]
pub type Pem0R = crate::BitReader;
#[doc = "Field `PEM0` writer - PEM0 Interrupt Enable"]
pub type Pem0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM1` reader - PEM1 Interrupt Enable"]
pub type Pem1R = crate::BitReader;
#[doc = "Field `PEM1` writer - PEM1 Interrupt Enable"]
pub type Pem1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TOUT Interrupt Enable"]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WARN Interrupt Enable"]
    #[inline(always)]
    pub fn warn(&self) -> WarnR {
        WarnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WIN Interrupt Enable"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pem0(&self) -> Pem0R {
        Pem0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PEM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pem1(&self) -> Pem1R {
        Pem1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TOUT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> ToutW<IENrs> {
        ToutW::new(self, 0)
    }
    #[doc = "Bit 1 - WARN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn warn(&mut self) -> WarnW<IENrs> {
        WarnW::new(self, 1)
    }
    #[doc = "Bit 2 - WIN Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WinW<IENrs> {
        WinW::new(self, 2)
    }
    #[doc = "Bit 3 - PEM0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pem0(&mut self) -> Pem0W<IENrs> {
        Pem0W::new(self, 3)
    }
    #[doc = "Bit 4 - PEM1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pem1(&mut self) -> Pem1W<IENrs> {
        Pem1W::new(self, 4)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IENrs {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
