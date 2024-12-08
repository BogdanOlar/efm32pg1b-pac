///Register `DBGCLKSEL` reader
pub type R = crate::R<DBGCLKSELrs>;
///Register `DBGCLKSEL` writer
pub type W = crate::W<DBGCLKSELrs>;
///Debug Trace Clock
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG {
    ///0: AUXHFRCO is the debug trace clock
    Auxhfrco = 0,
    ///1: HFCLK is the debug trace clock
    Hfclk = 1,
}
impl From<DBG> for bool {
    #[inline(always)]
    fn from(variant: DBG) -> Self {
        variant as u8 != 0
    }
}
///Field `DBG` reader - Debug Trace Clock
pub type DbgR = crate::BitReader<DBG>;
impl DbgR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBG {
        match self.bits {
            false => DBG::Auxhfrco,
            true => DBG::Hfclk,
        }
    }
    ///AUXHFRCO is the debug trace clock
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DBG::Auxhfrco
    }
    ///HFCLK is the debug trace clock
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DBG::Hfclk
    }
}
///Field `DBG` writer - Debug Trace Clock
pub type DbgW<'a, REG> = crate::BitWriter<'a, REG, DBG>;
impl<'a, REG> DbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///AUXHFRCO is the debug trace clock
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut crate::W<REG> {
        self.variant(DBG::Auxhfrco)
    }
    ///HFCLK is the debug trace clock
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(DBG::Hfclk)
    }
}
impl R {
    ///Bit 0 - Debug Trace Clock
    #[inline(always)]
    pub fn dbg(&self) -> DbgR {
        DbgR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGCLKSEL")
            .field("dbg", &self.dbg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Debug Trace Clock
    #[inline(always)]
    #[must_use]
    pub fn dbg(&mut self) -> DbgW<DBGCLKSELrs> {
        DbgW::new(self, 0)
    }
}
///Debug Trace Clock Select
///
///You can [`read`](crate::Reg::read) this register and get [`dbgclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DBGCLKSELrs;
impl crate::RegisterSpec for DBGCLKSELrs {
    type Ux = u32;
}
///`read()` method returns [`dbgclksel::R`](R) reader structure
impl crate::Readable for DBGCLKSELrs {}
///`write(|w| ..)` method takes [`dbgclksel::W`](W) writer structure
impl crate::Writable for DBGCLKSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBGCLKSEL to value 0
impl crate::Resettable for DBGCLKSELrs {
    const RESET_VALUE: u32 = 0;
}
