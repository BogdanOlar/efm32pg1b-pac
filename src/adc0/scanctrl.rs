#[doc = "Register `SCANCTRL` reader"]
pub type R = crate::R<SCANCTRLrs>;
#[doc = "Register `SCANCTRL` writer"]
pub type W = crate::W<SCANCTRLrs>;
#[doc = "Field `REP` reader - Scan Sequence Repetitive Mode"]
pub type RepR = crate::BitReader;
#[doc = "Field `REP` writer - Scan Sequence Repetitive Mode"]
pub type RepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF` reader - Scan Sequence Differential Mode"]
pub type DiffR = crate::BitReader;
#[doc = "Field `DIFF` writer - Scan Sequence Differential Mode"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ` reader - Scan Sequence Result Adjustment"]
pub type AdjR = crate::BitReader;
#[doc = "Field `ADJ` writer - Scan Sequence Result Adjustment"]
pub type AdjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Scan Sequence Resolution Select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    #[doc = "0: 12-bit resolution"]
    _12bit = 0,
    #[doc = "1: 8-bit resolution"]
    _8bit = 1,
    #[doc = "2: 6-bit resolution"]
    _6bit = 2,
    #[doc = "3: Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    Ovs = 3,
}
impl From<RES> for u8 {
    #[inline(always)]
    fn from(variant: RES) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES {
    type Ux = u8;
}
impl crate::IsEnum for RES {}
#[doc = "Field `RES` reader - Scan Sequence Resolution Select"]
pub type ResR = crate::FieldReader<RES>;
impl ResR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RES {
        match self.bits {
            0 => RES::_12bit,
            1 => RES::_8bit,
            2 => RES::_6bit,
            3 => RES::Ovs,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RES::_12bit
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RES::_8bit
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == RES::_6bit
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline(always)]
    pub fn is_ovs(&self) -> bool {
        *self == RES::Ovs
    }
}
#[doc = "Field `RES` writer - Scan Sequence Resolution Select"]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2, RES, crate::Safe>;
impl<'a, REG> ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit resolution"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::_12bit)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::_8bit)
    }
    #[doc = "6-bit resolution"]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::_6bit)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL"]
    #[inline(always)]
    pub fn ovs(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Ovs)
    }
}
#[doc = "Scan Sequence Reference Selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF {
    #[doc = "0: VFS = 1.25V with internal VBGR reference"]
    _1v25 = 0,
    #[doc = "1: VFS = 2.5V with internal VBGR reference"]
    _2v5 = 1,
    #[doc = "2: VFS = AVDD with AVDD as reference source"]
    Vdd = 2,
    #[doc = "3: VFS = 5V with internal VBGR reference"]
    _5v = 3,
    #[doc = "4: Single ended external reference"]
    Extsingle = 4,
    #[doc = "5: Differential external reference, 2x"]
    _2xextdiff = 5,
    #[doc = "6: VFS=2xAVDD with AVDD as the reference source"]
    _2xvdd = 6,
    #[doc = "7: Use SCANCTRLX to configure reference"]
    Conf = 7,
}
impl From<REF> for u8 {
    #[inline(always)]
    fn from(variant: REF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REF {
    type Ux = u8;
}
impl crate::IsEnum for REF {}
#[doc = "Field `REF` reader - Scan Sequence Reference Selection"]
pub type RefR = crate::FieldReader<REF>;
impl RefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REF {
        match self.bits {
            0 => REF::_1v25,
            1 => REF::_2v5,
            2 => REF::Vdd,
            3 => REF::_5v,
            4 => REF::Extsingle,
            5 => REF::_2xextdiff,
            6 => REF::_2xvdd,
            7 => REF::Conf,
            _ => unreachable!(),
        }
    }
    #[doc = "VFS = 1.25V with internal VBGR reference"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REF::_1v25
    }
    #[doc = "VFS = 2.5V with internal VBGR reference"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REF::_2v5
    }
    #[doc = "VFS = AVDD with AVDD as reference source"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REF::Vdd
    }
    #[doc = "VFS = 5V with internal VBGR reference"]
    #[inline(always)]
    pub fn is_5v(&self) -> bool {
        *self == REF::_5v
    }
    #[doc = "Single ended external reference"]
    #[inline(always)]
    pub fn is_extsingle(&self) -> bool {
        *self == REF::Extsingle
    }
    #[doc = "Differential external reference, 2x"]
    #[inline(always)]
    pub fn is_2xextdiff(&self) -> bool {
        *self == REF::_2xextdiff
    }
    #[doc = "VFS=2xAVDD with AVDD as the reference source"]
    #[inline(always)]
    pub fn is_2xvdd(&self) -> bool {
        *self == REF::_2xvdd
    }
    #[doc = "Use SCANCTRLX to configure reference"]
    #[inline(always)]
    pub fn is_conf(&self) -> bool {
        *self == REF::Conf
    }
}
#[doc = "Field `REF` writer - Scan Sequence Reference Selection"]
pub type RefW<'a, REG> = crate::FieldWriter<'a, REG, 3, REF, crate::Safe>;
impl<'a, REG> RefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VFS = 1.25V with internal VBGR reference"]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_1v25)
    }
    #[doc = "VFS = 2.5V with internal VBGR reference"]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_2v5)
    }
    #[doc = "VFS = AVDD with AVDD as reference source"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(REF::Vdd)
    }
    #[doc = "VFS = 5V with internal VBGR reference"]
    #[inline(always)]
    pub fn _5v(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_5v)
    }
    #[doc = "Single ended external reference"]
    #[inline(always)]
    pub fn extsingle(self) -> &'a mut crate::W<REG> {
        self.variant(REF::Extsingle)
    }
    #[doc = "Differential external reference, 2x"]
    #[inline(always)]
    pub fn _2xextdiff(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_2xextdiff)
    }
    #[doc = "VFS=2xAVDD with AVDD as the reference source"]
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_2xvdd)
    }
    #[doc = "Use SCANCTRLX to configure reference"]
    #[inline(always)]
    pub fn conf(self) -> &'a mut crate::W<REG> {
        self.variant(REF::Conf)
    }
}
#[doc = "Scan Acquisition Time\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AT {
    #[doc = "0: 1 conversion clock cycle acquisition time for scan"]
    _1cycle = 0,
    #[doc = "1: 2 conversion clock cycles acquisition time for scan"]
    _2cycles = 1,
    #[doc = "2: 3 conversion clock cycles acquisition time for scan"]
    _3cycles = 2,
    #[doc = "3: 4 conversion clock cycles acquisition time for scan"]
    _4cycles = 3,
    #[doc = "4: 8 conversion clock cycles acquisition time for scan"]
    _8cycles = 4,
    #[doc = "5: 16 conversion clock cycles acquisition time for scan"]
    _16cycles = 5,
    #[doc = "6: 32 conversion clock cycles acquisition time for scan"]
    _32cycles = 6,
    #[doc = "7: 64 conversion clock cycles acquisition time for scan"]
    _64cycles = 7,
    #[doc = "8: 128 conversion clock cycles acquisition time for scan"]
    _128cycles = 8,
    #[doc = "9: 256 conversion clock cycles acquisition time for scan"]
    _256cycles = 9,
}
impl From<AT> for u8 {
    #[inline(always)]
    fn from(variant: AT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AT {
    type Ux = u8;
}
impl crate::IsEnum for AT {}
#[doc = "Field `AT` reader - Scan Acquisition Time"]
pub type AtR = crate::FieldReader<AT>;
impl AtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AT> {
        match self.bits {
            0 => Some(AT::_1cycle),
            1 => Some(AT::_2cycles),
            2 => Some(AT::_3cycles),
            3 => Some(AT::_4cycles),
            4 => Some(AT::_8cycles),
            5 => Some(AT::_16cycles),
            6 => Some(AT::_32cycles),
            7 => Some(AT::_64cycles),
            8 => Some(AT::_128cycles),
            9 => Some(AT::_256cycles),
            _ => None,
        }
    }
    #[doc = "1 conversion clock cycle acquisition time for scan"]
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == AT::_1cycle
    }
    #[doc = "2 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == AT::_2cycles
    }
    #[doc = "3 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn is_3cycles(&self) -> bool {
        *self == AT::_3cycles
    }
    #[doc = "4 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == AT::_4cycles
    }
    #[doc = "8 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == AT::_8cycles
    }
    #[doc = "16 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == AT::_16cycles
    }
    #[doc = "32 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == AT::_32cycles
    }
    #[doc = "64 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == AT::_64cycles
    }
    #[doc = "128 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == AT::_128cycles
    }
    #[doc = "256 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == AT::_256cycles
    }
}
#[doc = "Field `AT` writer - Scan Acquisition Time"]
pub type AtW<'a, REG> = crate::FieldWriter<'a, REG, 4, AT>;
impl<'a, REG> AtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 conversion clock cycle acquisition time for scan"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_1cycle)
    }
    #[doc = "2 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_2cycles)
    }
    #[doc = "3 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _3cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_3cycles)
    }
    #[doc = "4 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_4cycles)
    }
    #[doc = "8 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_8cycles)
    }
    #[doc = "16 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_16cycles)
    }
    #[doc = "32 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_32cycles)
    }
    #[doc = "64 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_64cycles)
    }
    #[doc = "128 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_128cycles)
    }
    #[doc = "256 conversion clock cycles acquisition time for scan"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_256cycles)
    }
}
#[doc = "Field `PRSEN` reader - Scan Sequence PRS Trigger Enable"]
pub type PrsenR = crate::BitReader;
#[doc = "Field `PRSEN` writer - Scan Sequence PRS Trigger Enable"]
pub type PrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN` reader - Compare Logic Enable for Scan"]
pub type CmpenR = crate::BitReader;
#[doc = "Field `CMPEN` writer - Compare Logic Enable for Scan"]
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> AdjR {
        AdjR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Scan Sequence Resolution Select"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Scan Sequence Reference Selection"]
    #[inline(always)]
    pub fn ref_(&self) -> RefR {
        RefR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Scan Acquisition Time"]
    #[inline(always)]
    pub fn at(&self) -> AtR {
        AtR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Scan Sequence PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PrsenR {
        PrsenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Compare Logic Enable for Scan"]
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCANCTRL")
            .field("rep", &self.rep())
            .field("diff", &self.diff())
            .field("adj", &self.adj())
            .field("res", &self.res())
            .field("ref_", &self.ref_())
            .field("at", &self.at())
            .field("prsen", &self.prsen())
            .field("cmpen", &self.cmpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Scan Sequence Repetitive Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> RepW<SCANCTRLrs> {
        RepW::new(self, 0)
    }
    #[doc = "Bit 1 - Scan Sequence Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DiffW<SCANCTRLrs> {
        DiffW::new(self, 1)
    }
    #[doc = "Bit 2 - Scan Sequence Result Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> AdjW<SCANCTRLrs> {
        AdjW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Scan Sequence Resolution Select"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<SCANCTRLrs> {
        ResW::new(self, 3)
    }
    #[doc = "Bits 5:7 - Scan Sequence Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> RefW<SCANCTRLrs> {
        RefW::new(self, 5)
    }
    #[doc = "Bits 24:27 - Scan Acquisition Time"]
    #[inline(always)]
    #[must_use]
    pub fn at(&mut self) -> AtW<SCANCTRLrs> {
        AtW::new(self, 24)
    }
    #[doc = "Bit 29 - Scan Sequence PRS Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsen(&mut self) -> PrsenW<SCANCTRLrs> {
        PrsenW::new(self, 29)
    }
    #[doc = "Bit 31 - Compare Logic Enable for Scan"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CmpenW<SCANCTRLrs> {
        CmpenW::new(self, 31)
    }
}
#[doc = "Scan Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANCTRLrs;
impl crate::RegisterSpec for SCANCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanctrl::R`](R) reader structure"]
impl crate::Readable for SCANCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`scanctrl::W`](W) writer structure"]
impl crate::Writable for SCANCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCANCTRL to value 0"]
impl crate::Resettable for SCANCTRLrs {
    const RESET_VALUE: u32 = 0;
}
