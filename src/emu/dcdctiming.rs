///Register `DCDCTIMING` reader
pub type R = crate::R<DCDCTIMINGrs>;
///Register `DCDCTIMING` writer
pub type W = crate::W<DCDCTIMINGrs>;
///Field `LPINITWAIT` reader - Low Power Initialization Wait Time
pub type LpinitwaitR = crate::FieldReader;
///Field `LPINITWAIT` writer - Low Power Initialization Wait Time
pub type LpinitwaitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COMPENPRCHGEN` reader - LN Mode Precharge Enable
pub type CompenprchgenR = crate::BitReader;
///Field `COMPENPRCHGEN` writer - LN Mode Precharge Enable
pub type CompenprchgenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LNWAIT` reader - Low Noise Controller Initialization Wait Time
pub type LnwaitR = crate::FieldReader;
///Field `LNWAIT` writer - Low Noise Controller Initialization Wait Time
pub type LnwaitW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `BYPWAIT` reader - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait
pub type BypwaitR = crate::FieldReader;
///Field `BYPWAIT` writer - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait
pub type BypwaitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DUTYSCALE` reader - Select Bias Duty Cycle Clock
pub type DutyscaleR = crate::FieldReader;
///Field `DUTYSCALE` writer - Select Bias Duty Cycle Clock
pub type DutyscaleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:7 - Low Power Initialization Wait Time
    #[inline(always)]
    pub fn lpinitwait(&self) -> LpinitwaitR {
        LpinitwaitR::new((self.bits & 0xff) as u8)
    }
    ///Bit 11 - LN Mode Precharge Enable
    #[inline(always)]
    pub fn compenprchgen(&self) -> CompenprchgenR {
        CompenprchgenR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:16 - Low Noise Controller Initialization Wait Time
    #[inline(always)]
    pub fn lnwait(&self) -> LnwaitR {
        LnwaitR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 20:27 - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait
    #[inline(always)]
    pub fn bypwait(&self) -> BypwaitR {
        BypwaitR::new(((self.bits >> 20) & 0xff) as u8)
    }
    ///Bits 29:30 - Select Bias Duty Cycle Clock
    #[inline(always)]
    pub fn dutyscale(&self) -> DutyscaleR {
        DutyscaleR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCDCTIMING")
            .field("lpinitwait", &self.lpinitwait())
            .field("compenprchgen", &self.compenprchgen())
            .field("lnwait", &self.lnwait())
            .field("bypwait", &self.bypwait())
            .field("dutyscale", &self.dutyscale())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Low Power Initialization Wait Time
    #[inline(always)]
    #[must_use]
    pub fn lpinitwait(&mut self) -> LpinitwaitW<DCDCTIMINGrs> {
        LpinitwaitW::new(self, 0)
    }
    ///Bit 11 - LN Mode Precharge Enable
    #[inline(always)]
    #[must_use]
    pub fn compenprchgen(&mut self) -> CompenprchgenW<DCDCTIMINGrs> {
        CompenprchgenW::new(self, 11)
    }
    ///Bits 12:16 - Low Noise Controller Initialization Wait Time
    #[inline(always)]
    #[must_use]
    pub fn lnwait(&mut self) -> LnwaitW<DCDCTIMINGrs> {
        LnwaitW::new(self, 12)
    }
    ///Bits 20:27 - Bypass Mode Transition From Low Power or Low Noise Modes Wait Wait
    #[inline(always)]
    #[must_use]
    pub fn bypwait(&mut self) -> BypwaitW<DCDCTIMINGrs> {
        BypwaitW::new(self, 20)
    }
    ///Bits 29:30 - Select Bias Duty Cycle Clock
    #[inline(always)]
    #[must_use]
    pub fn dutyscale(&mut self) -> DutyscaleW<DCDCTIMINGrs> {
        DutyscaleW::new(self, 29)
    }
}
///DCDC Controller Timing Value Register
///
///You can [`read`](crate::Reg::read) this register and get [`dcdctiming::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdctiming::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct DCDCTIMINGrs;
impl crate::RegisterSpec for DCDCTIMINGrs {
    type Ux = u32;
}
///`read()` method returns [`dcdctiming::R`](R) reader structure
impl crate::Readable for DCDCTIMINGrs {}
///`write(|w| ..)` method takes [`dcdctiming::W`](W) writer structure
impl crate::Writable for DCDCTIMINGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCDCTIMING to value 0x0ff1_f8ff
impl crate::Resettable for DCDCTIMINGrs {
    const RESET_VALUE: u32 = 0x0ff1_f8ff;
}
