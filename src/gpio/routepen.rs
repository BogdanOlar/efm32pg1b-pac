#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPENrs>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPENrs>;
#[doc = "Field `SWCLKTCKPEN` reader - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SWCLKTCKPEN_R = crate::BitReader;
#[doc = "Field `SWCLKTCKPEN` writer - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SWCLKTCKPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDIOTMSPEN` reader - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SWDIOTMSPEN_R = crate::BitReader;
#[doc = "Field `SWDIOTMSPEN` writer - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SWDIOTMSPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDOPEN` reader - JTAG Test Debug Output Pin Enable"]
pub type TDOPEN_R = crate::BitReader;
#[doc = "Field `TDOPEN` writer - JTAG Test Debug Output Pin Enable"]
pub type TDOPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDIPEN` reader - JTAG Test Debug Input Pin Enable"]
pub type TDIPEN_R = crate::BitReader;
#[doc = "Field `TDIPEN` writer - JTAG Test Debug Input Pin Enable"]
pub type TDIPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWVPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SWVPEN_R = crate::BitReader;
#[doc = "Field `SWVPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SWVPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&self) -> SWCLKTCKPEN_R {
        SWCLKTCKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    pub fn swdiotmspen(&self) -> SWDIOTMSPEN_R {
        SWDIOTMSPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&self) -> TDOPEN_R {
        TDOPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&self) -> TDIPEN_R {
        TDIPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&self) -> SWVPEN_R {
        SWVPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swclktckpen(&mut self) -> SWCLKTCKPEN_W<ROUTEPENrs> {
        SWCLKTCKPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swdiotmspen(&mut self) -> SWDIOTMSPEN_W<ROUTEPENrs> {
        SWDIOTMSPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdopen(&mut self) -> TDOPEN_W<ROUTEPENrs> {
        TDOPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdipen(&mut self) -> TDIPEN_W<ROUTEPENrs> {
        TDIPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swvpen(&mut self) -> SWVPEN_W<ROUTEPENrs> {
        SWVPEN_W::new(self, 4)
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
#[doc = "I/O Routing Pin Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routepen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routepen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTEPENrs;
impl crate::RegisterSpec for ROUTEPENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routepen::R`](R) reader structure"]
impl crate::Readable for ROUTEPENrs {}
#[doc = "`write(|w| ..)` method takes [`routepen::W`](W) writer structure"]
impl crate::Writable for ROUTEPENrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0x0f"]
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0x0f;
}
