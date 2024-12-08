///Register `SINGLECTRL` reader
pub type R = crate::R<SINGLECTRLrs>;
///Register `SINGLECTRL` writer
pub type W = crate::W<SINGLECTRLrs>;
///Field `REP` reader - Single Channel Repetitive Mode
pub type RepR = crate::BitReader;
///Field `REP` writer - Single Channel Repetitive Mode
pub type RepW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIFF` reader - Single Channel Differential Mode
pub type DiffR = crate::BitReader;
///Field `DIFF` writer - Single Channel Differential Mode
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADJ` reader - Single Channel Result Adjustment
pub type AdjR = crate::BitReader;
///Field `ADJ` writer - Single Channel Result Adjustment
pub type AdjW<'a, REG> = crate::BitWriter<'a, REG>;
///Single Channel Resolution Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    ///0: 12-bit resolution.
    _12bit = 0,
    ///1: 8-bit resolution.
    _8bit = 1,
    ///2: 6-bit resolution.
    _6bit = 2,
    ///3: Oversampling enabled. Oversampling rate is set in OVSRSEL.
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
///Field `RES` reader - Single Channel Resolution Select
pub type ResR = crate::FieldReader<RES>;
impl ResR {
    ///Get enumerated values variant
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
    ///12-bit resolution.
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RES::_12bit
    }
    ///8-bit resolution.
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RES::_8bit
    }
    ///6-bit resolution.
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == RES::_6bit
    }
    ///Oversampling enabled. Oversampling rate is set in OVSRSEL.
    #[inline(always)]
    pub fn is_ovs(&self) -> bool {
        *self == RES::Ovs
    }
}
///Field `RES` writer - Single Channel Resolution Select
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 2, RES, crate::Safe>;
impl<'a, REG> ResW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///12-bit resolution.
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::_12bit)
    }
    ///8-bit resolution.
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::_8bit)
    }
    ///6-bit resolution.
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::_6bit)
    }
    ///Oversampling enabled. Oversampling rate is set in OVSRSEL.
    #[inline(always)]
    pub fn ovs(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Ovs)
    }
}
///Single Channel Reference Selection
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF {
    ///0: VFS = 1.25V with internal VBGR reference
    _1v25 = 0,
    ///1: VFS = 2.5V with internal VBGR reference
    _2v5 = 1,
    ///2: VFS = AVDD with AVDD as reference source
    Vdd = 2,
    ///3: VFS = 5V with internal VBGR reference
    _5v = 3,
    ///4: Single ended external reference
    Extsingle = 4,
    ///5: Differential external reference, 2x
    _2xextdiff = 5,
    ///6: VFS = 2xAVDD with AVDD as the reference source
    _2xvdd = 6,
    ///7: Use SINGLECTRLX to configure reference
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
///Field `REF` reader - Single Channel Reference Selection
pub type RefR = crate::FieldReader<REF>;
impl RefR {
    ///Get enumerated values variant
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
    ///VFS = 1.25V with internal VBGR reference
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        *self == REF::_1v25
    }
    ///VFS = 2.5V with internal VBGR reference
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        *self == REF::_2v5
    }
    ///VFS = AVDD with AVDD as reference source
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REF::Vdd
    }
    ///VFS = 5V with internal VBGR reference
    #[inline(always)]
    pub fn is_5v(&self) -> bool {
        *self == REF::_5v
    }
    ///Single ended external reference
    #[inline(always)]
    pub fn is_extsingle(&self) -> bool {
        *self == REF::Extsingle
    }
    ///Differential external reference, 2x
    #[inline(always)]
    pub fn is_2xextdiff(&self) -> bool {
        *self == REF::_2xextdiff
    }
    ///VFS = 2xAVDD with AVDD as the reference source
    #[inline(always)]
    pub fn is_2xvdd(&self) -> bool {
        *self == REF::_2xvdd
    }
    ///Use SINGLECTRLX to configure reference
    #[inline(always)]
    pub fn is_conf(&self) -> bool {
        *self == REF::Conf
    }
}
///Field `REF` writer - Single Channel Reference Selection
pub type RefW<'a, REG> = crate::FieldWriter<'a, REG, 3, REF, crate::Safe>;
impl<'a, REG> RefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///VFS = 1.25V with internal VBGR reference
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_1v25)
    }
    ///VFS = 2.5V with internal VBGR reference
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_2v5)
    }
    ///VFS = AVDD with AVDD as reference source
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(REF::Vdd)
    }
    ///VFS = 5V with internal VBGR reference
    #[inline(always)]
    pub fn _5v(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_5v)
    }
    ///Single ended external reference
    #[inline(always)]
    pub fn extsingle(self) -> &'a mut crate::W<REG> {
        self.variant(REF::Extsingle)
    }
    ///Differential external reference, 2x
    #[inline(always)]
    pub fn _2xextdiff(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_2xextdiff)
    }
    ///VFS = 2xAVDD with AVDD as the reference source
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_2xvdd)
    }
    ///Use SINGLECTRLX to configure reference
    #[inline(always)]
    pub fn conf(self) -> &'a mut crate::W<REG> {
        self.variant(REF::Conf)
    }
}
///Field `POSSEL` reader - Single Channel Positive Input Selection
pub type PosselR = crate::FieldReader;
///Field `POSSEL` writer - Single Channel Positive Input Selection
pub type PosselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `NEGSEL` reader - Single Channel Negative Input Selection
pub type NegselR = crate::FieldReader;
///Field `NEGSEL` writer - Single Channel Negative Input Selection
pub type NegselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Single Channel Acquisition Time
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AT {
    ///0: 1 conversion clock cycle acquisition time for single channel
    _1cycle = 0,
    ///1: 2 conversion clock cycles acquisition time for single channel
    _2cycles = 1,
    ///2: 3 conversion clock cycles acquisition time for single channel
    _3cycles = 2,
    ///3: 4 conversion clock cycles acquisition time for single channel
    _4cycles = 3,
    ///4: 8 conversion clock cycles acquisition time for single channel
    _8cycles = 4,
    ///5: 16 conversion clock cycles acquisition time for single channel
    _16cycles = 5,
    ///6: 32 conversion clock cycles acquisition time for single channel
    _32cycles = 6,
    ///7: 64 conversion clock cycles acquisition time for single channel
    _64cycles = 7,
    ///8: 128 conversion clock cycles acquisition time for single channel
    _128cycles = 8,
    ///9: 256 conversion clock cycles acquisition time for single channel
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
///Field `AT` reader - Single Channel Acquisition Time
pub type AtR = crate::FieldReader<AT>;
impl AtR {
    ///Get enumerated values variant
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
    ///1 conversion clock cycle acquisition time for single channel
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == AT::_1cycle
    }
    ///2 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == AT::_2cycles
    }
    ///3 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn is_3cycles(&self) -> bool {
        *self == AT::_3cycles
    }
    ///4 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == AT::_4cycles
    }
    ///8 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == AT::_8cycles
    }
    ///16 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == AT::_16cycles
    }
    ///32 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == AT::_32cycles
    }
    ///64 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == AT::_64cycles
    }
    ///128 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == AT::_128cycles
    }
    ///256 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == AT::_256cycles
    }
}
///Field `AT` writer - Single Channel Acquisition Time
pub type AtW<'a, REG> = crate::FieldWriter<'a, REG, 4, AT>;
impl<'a, REG> AtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 conversion clock cycle acquisition time for single channel
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_1cycle)
    }
    ///2 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_2cycles)
    }
    ///3 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn _3cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_3cycles)
    }
    ///4 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_4cycles)
    }
    ///8 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_8cycles)
    }
    ///16 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_16cycles)
    }
    ///32 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_32cycles)
    }
    ///64 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_64cycles)
    }
    ///128 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_128cycles)
    }
    ///256 conversion clock cycles acquisition time for single channel
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_256cycles)
    }
}
///Field `PRSEN` reader - Single Channel PRS Trigger Enable
pub type PrsenR = crate::BitReader;
///Field `PRSEN` writer - Single Channel PRS Trigger Enable
pub type PrsenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CMPEN` reader - Compare Logic Enable for Single Channel
pub type CmpenR = crate::BitReader;
///Field `CMPEN` writer - Compare Logic Enable for Single Channel
pub type CmpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Single Channel Repetitive Mode
    #[inline(always)]
    pub fn rep(&self) -> RepR {
        RepR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Single Channel Differential Mode
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Single Channel Result Adjustment
    #[inline(always)]
    pub fn adj(&self) -> AdjR {
        AdjR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - Single Channel Resolution Select
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 5:7 - Single Channel Reference Selection
    #[inline(always)]
    pub fn ref_(&self) -> RefR {
        RefR::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:15 - Single Channel Positive Input Selection
    #[inline(always)]
    pub fn possel(&self) -> PosselR {
        PosselR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Single Channel Negative Input Selection
    #[inline(always)]
    pub fn negsel(&self) -> NegselR {
        NegselR::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - Single Channel Acquisition Time
    #[inline(always)]
    pub fn at(&self) -> AtR {
        AtR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 29 - Single Channel PRS Trigger Enable
    #[inline(always)]
    pub fn prsen(&self) -> PrsenR {
        PrsenR::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - Compare Logic Enable for Single Channel
    #[inline(always)]
    pub fn cmpen(&self) -> CmpenR {
        CmpenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINGLECTRL")
            .field("rep", &self.rep())
            .field("diff", &self.diff())
            .field("adj", &self.adj())
            .field("res", &self.res())
            .field("ref_", &self.ref_())
            .field("possel", &self.possel())
            .field("negsel", &self.negsel())
            .field("at", &self.at())
            .field("prsen", &self.prsen())
            .field("cmpen", &self.cmpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Single Channel Repetitive Mode
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> RepW<SINGLECTRLrs> {
        RepW::new(self, 0)
    }
    ///Bit 1 - Single Channel Differential Mode
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DiffW<SINGLECTRLrs> {
        DiffW::new(self, 1)
    }
    ///Bit 2 - Single Channel Result Adjustment
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> AdjW<SINGLECTRLrs> {
        AdjW::new(self, 2)
    }
    ///Bits 3:4 - Single Channel Resolution Select
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<SINGLECTRLrs> {
        ResW::new(self, 3)
    }
    ///Bits 5:7 - Single Channel Reference Selection
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> RefW<SINGLECTRLrs> {
        RefW::new(self, 5)
    }
    ///Bits 8:15 - Single Channel Positive Input Selection
    #[inline(always)]
    #[must_use]
    pub fn possel(&mut self) -> PosselW<SINGLECTRLrs> {
        PosselW::new(self, 8)
    }
    ///Bits 16:23 - Single Channel Negative Input Selection
    #[inline(always)]
    #[must_use]
    pub fn negsel(&mut self) -> NegselW<SINGLECTRLrs> {
        NegselW::new(self, 16)
    }
    ///Bits 24:27 - Single Channel Acquisition Time
    #[inline(always)]
    #[must_use]
    pub fn at(&mut self) -> AtW<SINGLECTRLrs> {
        AtW::new(self, 24)
    }
    ///Bit 29 - Single Channel PRS Trigger Enable
    #[inline(always)]
    #[must_use]
    pub fn prsen(&mut self) -> PrsenW<SINGLECTRLrs> {
        PrsenW::new(self, 29)
    }
    ///Bit 31 - Compare Logic Enable for Single Channel
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CmpenW<SINGLECTRLrs> {
        CmpenW::new(self, 31)
    }
}
///Single Channel Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`singlectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct SINGLECTRLrs;
impl crate::RegisterSpec for SINGLECTRLrs {
    type Ux = u32;
}
///`read()` method returns [`singlectrl::R`](R) reader structure
impl crate::Readable for SINGLECTRLrs {}
///`write(|w| ..)` method takes [`singlectrl::W`](W) writer structure
impl crate::Writable for SINGLECTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SINGLECTRL to value 0x00ff_ff00
impl crate::Resettable for SINGLECTRLrs {
    const RESET_VALUE: u32 = 0x00ff_ff00;
}
