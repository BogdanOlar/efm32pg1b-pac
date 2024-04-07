#[doc = "Register `CTRLX` reader"]
pub type R = crate::R<CTRLXrs>;
#[doc = "Register `CTRLX` writer"]
pub type W = crate::W<CTRLXrs>;
#[doc = "Field `DBGHALT` reader - Debug Halt"]
pub type DbghaltR = crate::BitReader;
#[doc = "Field `DBGHALT` writer - Debug Halt"]
pub type DbghaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSINV` reader - CTS Pin Inversion"]
pub type CtsinvR = crate::BitReader;
#[doc = "Field `CTSINV` writer - CTS Pin Inversion"]
pub type CtsinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSEN` reader - CTS Function Enabled"]
pub type CtsenR = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTS Function Enabled"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSINV` reader - RTS Pin Inversion"]
pub type RtsinvR = crate::BitReader;
#[doc = "Field `RTSINV` writer - RTS Pin Inversion"]
pub type RtsinvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DbghaltR {
        DbghaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CtsinvR {
        CtsinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTS Function Enabled"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RtsinvR {
        RtsinvR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Halt"]
    #[inline(always)]
    #[must_use]
    pub fn dbghalt(&mut self) -> DbghaltW<CTRLXrs> {
        DbghaltW::new(self, 0)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn ctsinv(&mut self) -> CtsinvW<CTRLXrs> {
        CtsinvW::new(self, 1)
    }
    #[doc = "Bit 2 - CTS Function Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CtsenW<CTRLXrs> {
        CtsenW::new(self, 2)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rtsinv(&mut self) -> RtsinvW<CTRLXrs> {
        RtsinvW::new(self, 3)
    }
}
#[doc = "Control Register Extended\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLXrs;
impl crate::RegisterSpec for CTRLXrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlx::R`](R) reader structure"]
impl crate::Readable for CTRLXrs {}
#[doc = "`write(|w| ..)` method takes [`ctrlx::W`](W) writer structure"]
impl crate::Writable for CTRLXrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLX to value 0"]
impl crate::Resettable for CTRLXrs {
    const RESET_VALUE: u32 = 0;
}
