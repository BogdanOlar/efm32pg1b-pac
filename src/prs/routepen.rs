#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPENrs>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPENrs>;
#[doc = "Field `CH0PEN` reader - CH0 Pin Enable"]
pub type CH0PEN_R = crate::BitReader;
#[doc = "Field `CH0PEN` writer - CH0 Pin Enable"]
pub type CH0PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PEN` reader - CH1 Pin Enable"]
pub type CH1PEN_R = crate::BitReader;
#[doc = "Field `CH1PEN` writer - CH1 Pin Enable"]
pub type CH1PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PEN` reader - CH2 Pin Enable"]
pub type CH2PEN_R = crate::BitReader;
#[doc = "Field `CH2PEN` writer - CH2 Pin Enable"]
pub type CH2PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PEN` reader - CH3 Pin Enable"]
pub type CH3PEN_R = crate::BitReader;
#[doc = "Field `CH3PEN` writer - CH3 Pin Enable"]
pub type CH3PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PEN` reader - CH4 Pin Enable"]
pub type CH4PEN_R = crate::BitReader;
#[doc = "Field `CH4PEN` writer - CH4 Pin Enable"]
pub type CH4PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PEN` reader - CH5 Pin Enable"]
pub type CH5PEN_R = crate::BitReader;
#[doc = "Field `CH5PEN` writer - CH5 Pin Enable"]
pub type CH5PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6PEN` reader - CH6 Pin Enable"]
pub type CH6PEN_R = crate::BitReader;
#[doc = "Field `CH6PEN` writer - CH6 Pin Enable"]
pub type CH6PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7PEN` reader - CH7 Pin Enable"]
pub type CH7PEN_R = crate::BitReader;
#[doc = "Field `CH7PEN` writer - CH7 Pin Enable"]
pub type CH7PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8PEN` reader - CH8 Pin Enable"]
pub type CH8PEN_R = crate::BitReader;
#[doc = "Field `CH8PEN` writer - CH8 Pin Enable"]
pub type CH8PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9PEN` reader - CH9 Pin Enable"]
pub type CH9PEN_R = crate::BitReader;
#[doc = "Field `CH9PEN` writer - CH9 Pin Enable"]
pub type CH9PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10PEN` reader - CH10 Pin Enable"]
pub type CH10PEN_R = crate::BitReader;
#[doc = "Field `CH10PEN` writer - CH10 Pin Enable"]
pub type CH10PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11PEN` reader - CH11 Pin Enable"]
pub type CH11PEN_R = crate::BitReader;
#[doc = "Field `CH11PEN` writer - CH11 Pin Enable"]
pub type CH11PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&self) -> CH0PEN_R {
        CH0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&self) -> CH1PEN_R {
        CH1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&self) -> CH2PEN_R {
        CH2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&self) -> CH3PEN_R {
        CH3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    pub fn ch4pen(&self) -> CH4PEN_R {
        CH4PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    pub fn ch5pen(&self) -> CH5PEN_R {
        CH5PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    pub fn ch6pen(&self) -> CH6PEN_R {
        CH6PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    pub fn ch7pen(&self) -> CH7PEN_R {
        CH7PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    pub fn ch8pen(&self) -> CH8PEN_R {
        CH8PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    pub fn ch9pen(&self) -> CH9PEN_R {
        CH9PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    pub fn ch10pen(&self) -> CH10PEN_R {
        CH10PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    pub fn ch11pen(&self) -> CH11PEN_R {
        CH11PEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pen(&mut self) -> CH0PEN_W<ROUTEPENrs> {
        CH0PEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pen(&mut self) -> CH1PEN_W<ROUTEPENrs> {
        CH1PEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pen(&mut self) -> CH2PEN_W<ROUTEPENrs> {
        CH2PEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pen(&mut self) -> CH3PEN_W<ROUTEPENrs> {
        CH3PEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pen(&mut self) -> CH4PEN_W<ROUTEPENrs> {
        CH4PEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pen(&mut self) -> CH5PEN_W<ROUTEPENrs> {
        CH5PEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pen(&mut self) -> CH6PEN_W<ROUTEPENrs> {
        CH6PEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pen(&mut self) -> CH7PEN_W<ROUTEPENrs> {
        CH7PEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch8pen(&mut self) -> CH8PEN_W<ROUTEPENrs> {
        CH8PEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch9pen(&mut self) -> CH9PEN_W<ROUTEPENrs> {
        CH9PEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch10pen(&mut self) -> CH10PEN_W<ROUTEPENrs> {
        CH10PEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch11pen(&mut self) -> CH11PEN_W<ROUTEPENrs> {
        CH11PEN_W::new(self, 11)
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
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0;
}
