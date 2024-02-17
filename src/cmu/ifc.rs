#[doc = "Register `IFC` writer"]
pub type W = crate::W<IFCrs>;
#[doc = "Field `HFRCORDY` writer - Clear HFRCORDY Interrupt Flag"]
pub type HFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXORDY` writer - Clear HFXORDY Interrupt Flag"]
pub type HFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCORDY` writer - Clear LFRCORDY Interrupt Flag"]
pub type LFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXORDY` writer - Clear LFXORDY Interrupt Flag"]
pub type LFXORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXHFRCORDY` writer - Clear AUXHFRCORDY Interrupt Flag"]
pub type AUXHFRCORDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRDY` writer - Clear CALRDY Interrupt Flag"]
pub type CALRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOF` writer - Clear CALOF Interrupt Flag"]
pub type CALOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXODISERR` writer - Clear HFXODISERR Interrupt Flag"]
pub type HFXODISERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOAUTOSW` writer - Clear HFXOAUTOSW Interrupt Flag"]
pub type HFXOAUTOSW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETERR` writer - Clear HFXOPEAKDETERR Interrupt Flag"]
pub type HFXOPEAKDETERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOPEAKDETRDY` writer - Clear HFXOPEAKDETRDY Interrupt Flag"]
pub type HFXOPEAKDETRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXOSHUNTOPTRDY` writer - Clear HFXOSHUNTOPTRDY Interrupt Flag"]
pub type HFXOSHUNTOPTRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCODIS` writer - Clear HFRCODIS Interrupt Flag"]
pub type HFRCODIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFTIMEOUTERR` writer - Clear LFTIMEOUTERR Interrupt Flag"]
pub type LFTIMEOUTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUERR` writer - Clear CMUERR Interrupt Flag"]
pub type CMUERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear HFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W<IFCrs> {
        HFRCORDY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear HFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HFXORDY_W<IFCrs> {
        HFXORDY_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear LFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W<IFCrs> {
        LFRCORDY_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear LFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LFXORDY_W<IFCrs> {
        LFXORDY_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear AUXHFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W<IFCrs> {
        AUXHFRCORDY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear CALRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CALRDY_W<IFCrs> {
        CALRDY_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear CALOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CALOF_W<IFCrs> {
        CALOF_W::new(self, 6)
    }
    #[doc = "Bit 8 - Clear HFXODISERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodiserr(&mut self) -> HFXODISERR_W<IFCrs> {
        HFXODISERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear HFXOAUTOSW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoautosw(&mut self) -> HFXOAUTOSW_W<IFCrs> {
        HFXOAUTOSW_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear HFXOPEAKDETERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdeterr(&mut self) -> HFXOPEAKDETERR_W<IFCrs> {
        HFXOPEAKDETERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear HFXOPEAKDETRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetrdy(&mut self) -> HFXOPEAKDETRDY_W<IFCrs> {
        HFXOPEAKDETRDY_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear HFXOSHUNTOPTRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoshuntoptrdy(&mut self) -> HFXOSHUNTOPTRDY_W<IFCrs> {
        HFXOSHUNTOPTRDY_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear HFRCODIS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W<IFCrs> {
        HFRCODIS_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear LFTIMEOUTERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W<IFCrs> {
        LFTIMEOUTERR_W::new(self, 14)
    }
    #[doc = "Bit 31 - Clear CMUERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmuerr(&mut self) -> CMUERR_W<IFCrs> {
        CMUERR_W::new(self, 31)
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
