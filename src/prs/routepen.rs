#[doc = "Register `ROUTEPEN` reader"]
pub type R = crate::R<ROUTEPENrs>;
#[doc = "Register `ROUTEPEN` writer"]
pub type W = crate::W<ROUTEPENrs>;
#[doc = "Field `CH0PEN` reader - CH0 Pin Enable"]
pub type Ch0penR = crate::BitReader;
#[doc = "Field `CH0PEN` writer - CH0 Pin Enable"]
pub type Ch0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1PEN` reader - CH1 Pin Enable"]
pub type Ch1penR = crate::BitReader;
#[doc = "Field `CH1PEN` writer - CH1 Pin Enable"]
pub type Ch1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2PEN` reader - CH2 Pin Enable"]
pub type Ch2penR = crate::BitReader;
#[doc = "Field `CH2PEN` writer - CH2 Pin Enable"]
pub type Ch2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3PEN` reader - CH3 Pin Enable"]
pub type Ch3penR = crate::BitReader;
#[doc = "Field `CH3PEN` writer - CH3 Pin Enable"]
pub type Ch3penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4PEN` reader - CH4 Pin Enable"]
pub type Ch4penR = crate::BitReader;
#[doc = "Field `CH4PEN` writer - CH4 Pin Enable"]
pub type Ch4penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5PEN` reader - CH5 Pin Enable"]
pub type Ch5penR = crate::BitReader;
#[doc = "Field `CH5PEN` writer - CH5 Pin Enable"]
pub type Ch5penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6PEN` reader - CH6 Pin Enable"]
pub type Ch6penR = crate::BitReader;
#[doc = "Field `CH6PEN` writer - CH6 Pin Enable"]
pub type Ch6penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7PEN` reader - CH7 Pin Enable"]
pub type Ch7penR = crate::BitReader;
#[doc = "Field `CH7PEN` writer - CH7 Pin Enable"]
pub type Ch7penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8PEN` reader - CH8 Pin Enable"]
pub type Ch8penR = crate::BitReader;
#[doc = "Field `CH8PEN` writer - CH8 Pin Enable"]
pub type Ch8penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9PEN` reader - CH9 Pin Enable"]
pub type Ch9penR = crate::BitReader;
#[doc = "Field `CH9PEN` writer - CH9 Pin Enable"]
pub type Ch9penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10PEN` reader - CH10 Pin Enable"]
pub type Ch10penR = crate::BitReader;
#[doc = "Field `CH10PEN` writer - CH10 Pin Enable"]
pub type Ch10penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11PEN` reader - CH11 Pin Enable"]
pub type Ch11penR = crate::BitReader;
#[doc = "Field `CH11PEN` writer - CH11 Pin Enable"]
pub type Ch11penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&self) -> Ch0penR {
        Ch0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&self) -> Ch1penR {
        Ch1penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&self) -> Ch2penR {
        Ch2penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&self) -> Ch3penR {
        Ch3penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    pub fn ch4pen(&self) -> Ch4penR {
        Ch4penR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    pub fn ch5pen(&self) -> Ch5penR {
        Ch5penR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    pub fn ch6pen(&self) -> Ch6penR {
        Ch6penR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    pub fn ch7pen(&self) -> Ch7penR {
        Ch7penR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    pub fn ch8pen(&self) -> Ch8penR {
        Ch8penR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    pub fn ch9pen(&self) -> Ch9penR {
        Ch9penR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    pub fn ch10pen(&self) -> Ch10penR {
        Ch10penR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    pub fn ch11pen(&self) -> Ch11penR {
        Ch11penR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTEPEN")
            .field("ch0pen", &self.ch0pen())
            .field("ch1pen", &self.ch1pen())
            .field("ch2pen", &self.ch2pen())
            .field("ch3pen", &self.ch3pen())
            .field("ch4pen", &self.ch4pen())
            .field("ch5pen", &self.ch5pen())
            .field("ch6pen", &self.ch6pen())
            .field("ch7pen", &self.ch7pen())
            .field("ch8pen", &self.ch8pen())
            .field("ch9pen", &self.ch9pen())
            .field("ch10pen", &self.ch10pen())
            .field("ch11pen", &self.ch11pen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pen(&mut self) -> Ch0penW<ROUTEPENrs> {
        Ch0penW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pen(&mut self) -> Ch1penW<ROUTEPENrs> {
        Ch1penW::new(self, 1)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pen(&mut self) -> Ch2penW<ROUTEPENrs> {
        Ch2penW::new(self, 2)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pen(&mut self) -> Ch3penW<ROUTEPENrs> {
        Ch3penW::new(self, 3)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pen(&mut self) -> Ch4penW<ROUTEPENrs> {
        Ch4penW::new(self, 4)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pen(&mut self) -> Ch5penW<ROUTEPENrs> {
        Ch5penW::new(self, 5)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pen(&mut self) -> Ch6penW<ROUTEPENrs> {
        Ch6penW::new(self, 6)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pen(&mut self) -> Ch7penW<ROUTEPENrs> {
        Ch7penW::new(self, 7)
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch8pen(&mut self) -> Ch8penW<ROUTEPENrs> {
        Ch8penW::new(self, 8)
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch9pen(&mut self) -> Ch9penW<ROUTEPENrs> {
        Ch9penW::new(self, 9)
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch10pen(&mut self) -> Ch10penW<ROUTEPENrs> {
        Ch10penW::new(self, 10)
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch11pen(&mut self) -> Ch11penW<ROUTEPENrs> {
        Ch11penW::new(self, 11)
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
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPENrs {
    const RESET_VALUE: u32 = 0;
}
