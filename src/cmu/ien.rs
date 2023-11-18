#[doc = "Register `IEN` reader"]
pub type R = crate::R<IEN_SPEC>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IEN_SPEC>;
#[doc = "Field `HFRCORDY` reader - HFRCORDY Interrupt Enable"]
pub type HFRCORDY_R = crate::BitReader;
#[doc = "Field `HFRCORDY` writer - HFRCORDY Interrupt Enable"]
pub type HFRCORDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFXORDY` reader - HFXORDY Interrupt Enable"]
pub type HFXORDY_R = crate::BitReader;
#[doc = "Field `HFXORDY` writer - HFXORDY Interrupt Enable"]
pub type HFXORDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFRCORDY` reader - LFRCORDY Interrupt Enable"]
pub type LFRCORDY_R = crate::BitReader;
#[doc = "Field `LFRCORDY` writer - LFRCORDY Interrupt Enable"]
pub type LFRCORDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFXORDY` reader - LFXORDY Interrupt Enable"]
pub type LFXORDY_R = crate::BitReader;
#[doc = "Field `LFXORDY` writer - LFXORDY Interrupt Enable"]
pub type LFXORDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCORDY Interrupt Enable"]
pub type AUXHFRCORDY_R = crate::BitReader;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCORDY Interrupt Enable"]
pub type AUXHFRCORDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALRDY` reader - CALRDY Interrupt Enable"]
pub type CALRDY_R = crate::BitReader;
#[doc = "Field `CALRDY` writer - CALRDY Interrupt Enable"]
pub type CALRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CALOF` reader - CALOF Interrupt Enable"]
pub type CALOF_R = crate::BitReader;
#[doc = "Field `CALOF` writer - CALOF Interrupt Enable"]
pub type CALOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFXODISERR` reader - HFXODISERR Interrupt Enable"]
pub type HFXODISERR_R = crate::BitReader;
#[doc = "Field `HFXODISERR` writer - HFXODISERR Interrupt Enable"]
pub type HFXODISERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFXOAUTOSW` reader - HFXOAUTOSW Interrupt Enable"]
pub type HFXOAUTOSW_R = crate::BitReader;
#[doc = "Field `HFXOAUTOSW` writer - HFXOAUTOSW Interrupt Enable"]
pub type HFXOAUTOSW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFXOPEAKDETERR` reader - HFXOPEAKDETERR Interrupt Enable"]
pub type HFXOPEAKDETERR_R = crate::BitReader;
#[doc = "Field `HFXOPEAKDETERR` writer - HFXOPEAKDETERR Interrupt Enable"]
pub type HFXOPEAKDETERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFXOPEAKDETRDY` reader - HFXOPEAKDETRDY Interrupt Enable"]
pub type HFXOPEAKDETRDY_R = crate::BitReader;
#[doc = "Field `HFXOPEAKDETRDY` writer - HFXOPEAKDETRDY Interrupt Enable"]
pub type HFXOPEAKDETRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFXOSHUNTOPTRDY` reader - HFXOSHUNTOPTRDY Interrupt Enable"]
pub type HFXOSHUNTOPTRDY_R = crate::BitReader;
#[doc = "Field `HFXOSHUNTOPTRDY` writer - HFXOSHUNTOPTRDY Interrupt Enable"]
pub type HFXOSHUNTOPTRDY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HFRCODIS` reader - HFRCODIS Interrupt Enable"]
pub type HFRCODIS_R = crate::BitReader;
#[doc = "Field `HFRCODIS` writer - HFRCODIS Interrupt Enable"]
pub type HFRCODIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LFTIMEOUTERR` reader - LFTIMEOUTERR Interrupt Enable"]
pub type LFTIMEOUTERR_R = crate::BitReader;
#[doc = "Field `LFTIMEOUTERR` writer - LFTIMEOUTERR Interrupt Enable"]
pub type LFTIMEOUTERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CMUERR` reader - CMUERR Interrupt Enable"]
pub type CMUERR_R = crate::BitReader;
#[doc = "Field `CMUERR` writer - CMUERR Interrupt Enable"]
pub type CMUERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HFXODISERR_R {
        HFXODISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HFXOAUTOSW_R {
        HFXOAUTOSW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HFXOPEAKDETERR Interrupt Enable"]
    #[inline(always)]
    pub fn hfxopeakdeterr(&self) -> HFXOPEAKDETERR_R {
        HFXOPEAKDETERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - HFXOSHUNTOPTRDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxoshuntoptrdy(&self) -> HFXOSHUNTOPTRDY_R {
        HFXOSHUNTOPTRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcodis(&self) -> HFRCODIS_R {
        HFRCODIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LFTIMEOUTERR_R {
        LFTIMEOUTERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    pub fn cmuerr(&self) -> CMUERR_R {
        CMUERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("hfrcordy", &format_args!("{}", self.hfrcordy().bit()))
            .field("hfxordy", &format_args!("{}", self.hfxordy().bit()))
            .field("lfrcordy", &format_args!("{}", self.lfrcordy().bit()))
            .field("lfxordy", &format_args!("{}", self.lfxordy().bit()))
            .field("auxhfrcordy", &format_args!("{}", self.auxhfrcordy().bit()))
            .field("calrdy", &format_args!("{}", self.calrdy().bit()))
            .field("calof", &format_args!("{}", self.calof().bit()))
            .field("hfxodiserr", &format_args!("{}", self.hfxodiserr().bit()))
            .field("hfxoautosw", &format_args!("{}", self.hfxoautosw().bit()))
            .field(
                "hfxopeakdeterr",
                &format_args!("{}", self.hfxopeakdeterr().bit()),
            )
            .field(
                "hfxopeakdetrdy",
                &format_args!("{}", self.hfxopeakdetrdy().bit()),
            )
            .field(
                "hfxoshuntoptrdy",
                &format_args!("{}", self.hfxoshuntoptrdy().bit()),
            )
            .field("hfrcodis", &format_args!("{}", self.hfrcodis().bit()))
            .field(
                "lftimeouterr",
                &format_args!("{}", self.lftimeouterr().bit()),
            )
            .field("cmuerr", &format_args!("{}", self.cmuerr().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IEN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W<IEN_SPEC, 0> {
        HFRCORDY_W::new(self)
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HFXORDY_W<IEN_SPEC, 1> {
        HFXORDY_W::new(self)
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W<IEN_SPEC, 2> {
        LFRCORDY_W::new(self)
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LFXORDY_W<IEN_SPEC, 3> {
        LFXORDY_W::new(self)
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W<IEN_SPEC, 4> {
        AUXHFRCORDY_W::new(self)
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CALRDY_W<IEN_SPEC, 5> {
        CALRDY_W::new(self)
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CALOF_W<IEN_SPEC, 6> {
        CALOF_W::new(self)
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodiserr(&mut self) -> HFXODISERR_W<IEN_SPEC, 8> {
        HFXODISERR_W::new(self)
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoautosw(&mut self) -> HFXOAUTOSW_W<IEN_SPEC, 9> {
        HFXOAUTOSW_W::new(self)
    }
    #[doc = "Bit 10 - HFXOPEAKDETERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdeterr(&mut self) -> HFXOPEAKDETERR_W<IEN_SPEC, 10> {
        HFXOPEAKDETERR_W::new(self)
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetrdy(&mut self) -> HFXOPEAKDETRDY_W<IEN_SPEC, 11> {
        HFXOPEAKDETRDY_W::new(self)
    }
    #[doc = "Bit 12 - HFXOSHUNTOPTRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoshuntoptrdy(&mut self) -> HFXOSHUNTOPTRDY_W<IEN_SPEC, 12> {
        HFXOSHUNTOPTRDY_W::new(self)
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W<IEN_SPEC, 13> {
        HFRCODIS_W::new(self)
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W<IEN_SPEC, 14> {
        LFTIMEOUTERR_W::new(self)
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmuerr(&mut self) -> CMUERR_W<IEN_SPEC, 31> {
        CMUERR_W::new(self)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
