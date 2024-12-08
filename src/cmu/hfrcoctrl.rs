///Register `HFRCOCTRL` reader
pub type R = crate::R<HFRCOCTRLrs>;
///Register `HFRCOCTRL` writer
pub type W = crate::W<HFRCOCTRLrs>;
///Field `TUNING` reader - HFRCO Tuning Value
pub type TuningR = crate::FieldReader;
///Field `TUNING` writer - HFRCO Tuning Value
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `FINETUNING` reader - HFRCO Fine Tuning Value
pub type FinetuningR = crate::FieldReader;
///Field `FINETUNING` writer - HFRCO Fine Tuning Value
pub type FinetuningW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `FREQRANGE` reader - HFRCO Frequency Range
pub type FreqrangeR = crate::FieldReader;
///Field `FREQRANGE` writer - HFRCO Frequency Range
pub type FreqrangeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CMPBIAS` reader - HFRCO Comparator Bias Current
pub type CmpbiasR = crate::FieldReader;
///Field `CMPBIAS` writer - HFRCO Comparator Bias Current
pub type CmpbiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LDOHP` reader - HFRCO LDO High Power Mode
pub type LdohpR = crate::BitReader;
///Field `LDOHP` writer - HFRCO LDO High Power Mode
pub type LdohpW<'a, REG> = crate::BitWriter<'a, REG>;
///Locally Divide HFRCO Clock Output
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV {
    ///0: Divide by 1.
    Div1 = 0,
    ///1: Divide by 2.
    Div2 = 1,
    ///2: Divide by 4.
    Div4 = 2,
}
impl From<CLKDIV> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKDIV {
    type Ux = u8;
}
impl crate::IsEnum for CLKDIV {}
///Field `CLKDIV` reader - Locally Divide HFRCO Clock Output
pub type ClkdivR = crate::FieldReader<CLKDIV>;
impl ClkdivR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKDIV> {
        match self.bits {
            0 => Some(CLKDIV::Div1),
            1 => Some(CLKDIV::Div2),
            2 => Some(CLKDIV::Div4),
            _ => None,
        }
    }
    ///Divide by 1.
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CLKDIV::Div1
    }
    ///Divide by 2.
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKDIV::Div2
    }
    ///Divide by 4.
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLKDIV::Div4
    }
}
///Field `CLKDIV` writer - Locally Divide HFRCO Clock Output
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKDIV>;
impl<'a, REG> ClkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Divide by 1.
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div1)
    }
    ///Divide by 2.
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div2)
    }
    ///Divide by 4.
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV::Div4)
    }
}
///Field `FINETUNINGEN` reader - Enable Reference for Fine Tuning
pub type FinetuningenR = crate::BitReader;
///Field `FINETUNINGEN` writer - Enable Reference for Fine Tuning
pub type FinetuningenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFTC` reader - HFRCO Temperature Coefficient Trim on Comparator Reference
pub type VreftcR = crate::FieldReader;
///Field `VREFTC` writer - HFRCO Temperature Coefficient Trim on Comparator Reference
pub type VreftcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:6 - HFRCO Tuning Value
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:13 - HFRCO Fine Tuning Value
    #[inline(always)]
    pub fn finetuning(&self) -> FinetuningR {
        FinetuningR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:20 - HFRCO Frequency Range
    #[inline(always)]
    pub fn freqrange(&self) -> FreqrangeR {
        FreqrangeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:23 - HFRCO Comparator Bias Current
    #[inline(always)]
    pub fn cmpbias(&self) -> CmpbiasR {
        CmpbiasR::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bit 24 - HFRCO LDO High Power Mode
    #[inline(always)]
    pub fn ldohp(&self) -> LdohpR {
        LdohpR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Locally Divide HFRCO Clock Output
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - Enable Reference for Fine Tuning
    #[inline(always)]
    pub fn finetuningen(&self) -> FinetuningenR {
        FinetuningenR::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31 - HFRCO Temperature Coefficient Trim on Comparator Reference
    #[inline(always)]
    pub fn vreftc(&self) -> VreftcR {
        VreftcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFRCOCTRL")
            .field("tuning", &self.tuning())
            .field("finetuning", &self.finetuning())
            .field("freqrange", &self.freqrange())
            .field("cmpbias", &self.cmpbias())
            .field("ldohp", &self.ldohp())
            .field("clkdiv", &self.clkdiv())
            .field("finetuningen", &self.finetuningen())
            .field("vreftc", &self.vreftc())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - HFRCO Tuning Value
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TuningW<HFRCOCTRLrs> {
        TuningW::new(self, 0)
    }
    ///Bits 8:13 - HFRCO Fine Tuning Value
    #[inline(always)]
    #[must_use]
    pub fn finetuning(&mut self) -> FinetuningW<HFRCOCTRLrs> {
        FinetuningW::new(self, 8)
    }
    ///Bits 16:20 - HFRCO Frequency Range
    #[inline(always)]
    #[must_use]
    pub fn freqrange(&mut self) -> FreqrangeW<HFRCOCTRLrs> {
        FreqrangeW::new(self, 16)
    }
    ///Bits 21:23 - HFRCO Comparator Bias Current
    #[inline(always)]
    #[must_use]
    pub fn cmpbias(&mut self) -> CmpbiasW<HFRCOCTRLrs> {
        CmpbiasW::new(self, 21)
    }
    ///Bit 24 - HFRCO LDO High Power Mode
    #[inline(always)]
    #[must_use]
    pub fn ldohp(&mut self) -> LdohpW<HFRCOCTRLrs> {
        LdohpW::new(self, 24)
    }
    ///Bits 25:26 - Locally Divide HFRCO Clock Output
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<HFRCOCTRLrs> {
        ClkdivW::new(self, 25)
    }
    ///Bit 27 - Enable Reference for Fine Tuning
    #[inline(always)]
    #[must_use]
    pub fn finetuningen(&mut self) -> FinetuningenW<HFRCOCTRLrs> {
        FinetuningenW::new(self, 27)
    }
    ///Bits 28:31 - HFRCO Temperature Coefficient Trim on Comparator Reference
    #[inline(always)]
    #[must_use]
    pub fn vreftc(&mut self) -> VreftcW<HFRCOCTRLrs> {
        VreftcW::new(self, 28)
    }
}
///HFRCO Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`hfrcoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrcoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HFRCOCTRLrs;
impl crate::RegisterSpec for HFRCOCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`hfrcoctrl::R`](R) reader structure
impl crate::Readable for HFRCOCTRLrs {}
///`write(|w| ..)` method takes [`hfrcoctrl::W`](W) writer structure
impl crate::Writable for HFRCOCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFRCOCTRL to value 0xb148_1f3c
impl crate::Resettable for HFRCOCTRLrs {
    const RESET_VALUE: u32 = 0xb148_1f3c;
}
