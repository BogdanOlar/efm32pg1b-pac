#[doc = "Register `DBGCLKSEL` reader"]
pub type R = crate::R<DBGCLKSEL_SPEC>;
#[doc = "Register `DBGCLKSEL` writer"]
pub type W = crate::W<DBGCLKSEL_SPEC>;
#[doc = "Field `DBG` reader - Debug Trace Clock"]
pub type DBG_R = crate::BitReader<DBG_A>;
#[doc = "Debug Trace Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_A {
    #[doc = "0: AUXHFRCO is the debug trace clock"]
    AUXHFRCO = 0,
    #[doc = "1: HFCLK is the debug trace clock"]
    HFCLK = 1,
}
impl From<DBG_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG_A {
        match self.bits {
            false => DBG_A::AUXHFRCO,
            true => DBG_A::HFCLK,
        }
    }
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DBG_A::AUXHFRCO
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DBG_A::HFCLK
    }
}
#[doc = "Field `DBG` writer - Debug Trace Clock"]
pub type DBG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, DBG_A>;
impl<'a, REG, const O: u8> DBG_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_A::AUXHFRCO)
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(DBG_A::HFCLK)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGCLKSEL")
            .field("dbg", &format_args!("{}", self.dbg().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DBGCLKSEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Trace Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dbg(&mut self) -> DBG_W<DBGCLKSEL_SPEC, 0> {
        DBG_W::new(self)
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
#[doc = "Debug Trace Clock Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGCLKSEL_SPEC;
impl crate::RegisterSpec for DBGCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgclksel::R`](R) reader structure"]
impl crate::Readable for DBGCLKSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbgclksel::W`](W) writer structure"]
impl crate::Writable for DBGCLKSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGCLKSEL to value 0"]
impl crate::Resettable for DBGCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
