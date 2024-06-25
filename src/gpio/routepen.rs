#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPENrs>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPENrs>;
#[doc = "Field `SWCLKTCKPEN` reader - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SwclktckpenR = crate::BitReader;
#[doc = "Field `SWCLKTCKPEN` writer - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SwclktckpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDIOTMSPEN` reader - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SwdiotmspenR = crate::BitReader;
#[doc = "Field `SWDIOTMSPEN` writer - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SwdiotmspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDOPEN` reader - JTAG Test Debug Output Pin Enable"]
pub type TdopenR = crate::BitReader;
#[doc = "Field `TDOPEN` writer - JTAG Test Debug Output Pin Enable"]
pub type TdopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIPEN` reader - JTAG Test Debug Input Pin Enable"]
pub type TdipenR = crate::BitReader;
#[doc = "Field `TDIPEN` writer - JTAG Test Debug Input Pin Enable"]
pub type TdipenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWVPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SwvpenR = crate::BitReader;
#[doc = "Field `SWVPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SwvpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&self) -> SwclktckpenR {
        SwclktckpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    pub fn swdiotmspen(&self) -> SwdiotmspenR {
        SwdiotmspenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&self) -> TdopenR {
        TdopenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&self) -> TdipenR {
        TdipenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&self) -> SwvpenR {
        SwvpenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTEPEN")
            .field("swclktckpen", &self.swclktckpen())
            .field("swdiotmspen", &self.swdiotmspen())
            .field("tdopen", &self.tdopen())
            .field("tdipen", &self.tdipen())
            .field("swvpen", &self.swvpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swclktckpen(&mut self) -> SwclktckpenW<ROUTEPENrs> {
        SwclktckpenW::new(self, 0)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swdiotmspen(&mut self) -> SwdiotmspenW<ROUTEPENrs> {
        SwdiotmspenW::new(self, 1)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdopen(&mut self) -> TdopenW<ROUTEPENrs> {
        TdopenW::new(self, 2)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdipen(&mut self) -> TdipenW<ROUTEPENrs> {
        TdipenW::new(self, 3)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swvpen(&mut self) -> SwvpenW<ROUTEPENrs> {
        SwvpenW::new(self, 4)
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routepen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPENrs;
impl crate::RegisterSpec for ROUTEPENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPENrs {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0x0f"]
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0x0f;
}
