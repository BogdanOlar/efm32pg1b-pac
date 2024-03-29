#[doc = "Register `DBGCLKSEL` reader"]
pub type R = crate::R<DBGCLKSELrs>;
#[doc = "Register `DBGCLKSEL` writer"]
pub type W = crate::W<DBGCLKSELrs>;
#[doc = "Field `DBG` reader - Debug Trace Clock"]
pub type DBG_R = crate::BitReader<DBG>;
#[doc = "Debug Trace Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG {
    #[doc = "0: AUXHFRCO is the debug trace clock"]
    Auxhfrco = 0,
    #[doc = "1: HFCLK is the debug trace clock"]
    Hfclk = 1,
}
impl From<DBG> for bool {
    #[inline(always)]
    fn from(variant: DBG) -> Self {
        variant as u8 != 0
    }
}
impl DBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBG {
        match self.bits {
            false => DBG::Auxhfrco,
            true => DBG::Hfclk,
        }
    }
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DBG::Auxhfrco
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DBG::Hfclk
    }
}
#[doc = "Field `DBG` writer - Debug Trace Clock"]
pub type DBG_W<'a, REG> = crate::BitWriter<'a, REG, DBG>;
impl<'a, REG> DBG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(DBG::Auxhfrco)
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(DBG::Hfclk)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Trace Clock"]
    #[inline(always)]
    #[must_use]
    pub fn dbg(&mut self) -> DBG_W<DBGCLKSELrs> {
        DBG_W::new(self, 0)
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
pub struct DBGCLKSELrs;
impl crate::RegisterSpec for DBGCLKSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgclksel::R`](R) reader structure"]
impl crate::Readable for DBGCLKSELrs {}
#[doc = "`write(|w| ..)` method takes [`dbgclksel::W`](W) writer structure"]
impl crate::Writable for DBGCLKSELrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGCLKSEL to value 0"]
impl crate::Resettable for DBGCLKSELrs {
    const RESET_VALUE: u32 = 0;
}
