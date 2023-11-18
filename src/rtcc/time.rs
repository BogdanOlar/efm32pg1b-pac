#[doc = "Register `TIME` reader"]
pub type R = crate::R<TIME_SPEC>;
#[doc = "Register `TIME` writer"]
pub type W = crate::W<TIME_SPEC>;
#[doc = "Field `SECU` reader - Seconds, Units"]
pub type SECU_R = crate::FieldReader;
#[doc = "Field `SECU` writer - Seconds, Units"]
pub type SECU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SECT` reader - Seconds, Tens"]
pub type SECT_R = crate::FieldReader;
#[doc = "Field `SECT` writer - Seconds, Tens"]
pub type SECT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MINU` reader - Minutes, Units"]
pub type MINU_R = crate::FieldReader;
#[doc = "Field `MINU` writer - Minutes, Units"]
pub type MINU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MINT` reader - Minutes, Tens"]
pub type MINT_R = crate::FieldReader;
#[doc = "Field `MINT` writer - Minutes, Tens"]
pub type MINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `HOURU` reader - Hours, Units"]
pub type HOURU_R = crate::FieldReader;
#[doc = "Field `HOURU` writer - Hours, Units"]
pub type HOURU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HOURT` reader - Hours, Tens"]
pub type HOURT_R = crate::FieldReader;
#[doc = "Field `HOURT` writer - Hours, Tens"]
pub type HOURT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&self) -> SECU_R {
        SECU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&self) -> SECT_R {
        SECT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&self) -> MINU_R {
        MINU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&self) -> MINT_R {
        MINT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&self) -> HOURU_R {
        HOURU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&self) -> HOURT_R {
        HOURT_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIME")
            .field("secu", &format_args!("{}", self.secu().bits()))
            .field("sect", &format_args!("{}", self.sect().bits()))
            .field("minu", &format_args!("{}", self.minu().bits()))
            .field("mint", &format_args!("{}", self.mint().bits()))
            .field("houru", &format_args!("{}", self.houru().bits()))
            .field("hourt", &format_args!("{}", self.hourt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    #[must_use]
    pub fn secu(&mut self) -> SECU_W<TIME_SPEC, 0> {
        SECU_W::new(self)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn sect(&mut self) -> SECT_W<TIME_SPEC, 4> {
        SECT_W::new(self)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    #[must_use]
    pub fn minu(&mut self) -> MINU_W<TIME_SPEC, 8> {
        MINU_W::new(self)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn mint(&mut self) -> MINT_W<TIME_SPEC, 12> {
        MINT_W::new(self)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    #[must_use]
    pub fn houru(&mut self) -> HOURU_W<TIME_SPEC, 16> {
        HOURU_W::new(self)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn hourt(&mut self) -> HOURT_W<TIME_SPEC, 20> {
        HOURT_W::new(self)
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
#[doc = "Time of Day Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_SPEC;
impl crate::RegisterSpec for TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time::W`](W) writer structure"]
impl crate::Writable for TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
