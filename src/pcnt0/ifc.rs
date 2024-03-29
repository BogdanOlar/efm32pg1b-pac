#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `UF` writer - Clear UF Interrupt Flag"]
pub type UF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OF` writer - Clear OF Interrupt Flag"]
pub type OF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRCNG` writer - Clear DIRCNG Interrupt Flag"]
pub type DIRCNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOF` writer - Clear AUXOF Interrupt Flag"]
pub type AUXOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` writer - Clear TCC Interrupt Flag"]
pub type TCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OQSTERR` writer - Clear OQSTERR Interrupt Flag"]
pub type OQSTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear UF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn uf(&mut self) -> UF_W<IFCrs> {
        UF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear OF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn of(&mut self) -> OF_W<IFCrs> {
        OF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear DIRCNG Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dircng(&mut self) -> DIRCNG_W<IFCrs> {
        DIRCNG_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear AUXOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn auxof(&mut self) -> AUXOF_W<IFCrs> {
        AUXOF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear TCC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TCC_W<IFCrs> {
        TCC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear OQSTERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn oqsterr(&mut self) -> OQSTERR_W<IFCrs> {
        OQSTERR_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCrs;
impl crate::RegisterSpec for IFCrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifc::W`](W) writer structure"]
impl crate::Writable for IFCrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFCrs {
    const RESET_VALUE: u32 = 0;
}
