#[doc = "Register `SINGLECTRL` reader"]
pub type R = crate::R<SINGLECTRLrs>;
#[doc = "Register `SINGLECTRL` writer"]
pub type W = crate::W<SINGLECTRLrs>;
#[doc = "Field `REP` reader - Single Channel Repetitive Mode"]
pub type REP_R = crate::BitReader;
#[doc = "Field `REP` writer - Single Channel Repetitive Mode"]
pub type REP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIFF` reader - Single Channel Differential Mode"]
pub type DIFF_R = crate::BitReader;
#[doc = "Field `DIFF` writer - Single Channel Differential Mode"]
pub type DIFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADJ` reader - Single Channel Result Adjustment"]
pub type ADJ_R = crate::BitReader;
#[doc = "Field `ADJ` writer - Single Channel Result Adjustment"]
pub type ADJ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES` reader - Single Channel Resolution Select"]
pub type RES_R = crate::FieldReader<RES>;
#[doc = "Single Channel Resolution Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES {
    #[doc = "0: 12-bit resolution."]
    _12bit = 0,
    #[doc = "1: 8-bit resolution."]
    _8bit = 1,
    #[doc = "2: 6-bit resolution."]
    _6bit = 2,
    #[doc = "3: Oversampling enabled. Oversampling rate is set in OVSRSEL."]
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
impl RES_R {
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
    #[doc = "12-bit resolution."]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == RES::_12bit
    }
    #[doc = "8-bit resolution."]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == RES::_8bit
    }
    #[doc = "6-bit resolution."]
    #[inline(always)]
    pub fn is_6bit(&self) -> bool {
        *self == RES::_6bit
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL."]
    #[inline(always)]
    pub fn is_ovs(&self) -> bool {
        *self == RES::Ovs
    }
}
#[doc = "Field `RES` writer - Single Channel Resolution Select"]
pub type RES_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RES>;
impl<'a, REG> RES_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit resolution."]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::_12bit)
    }
    #[doc = "8-bit resolution."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::_8bit)
    }
    #[doc = "6-bit resolution."]
    #[inline(always)]
    pub fn _6bit(self) -> &'a mut crate::W<REG> {
        self.variant(RES::_6bit)
    }
    #[doc = "Oversampling enabled. Oversampling rate is set in OVSRSEL."]
    #[inline(always)]
    pub fn ovs(self) -> &'a mut crate::W<REG> {
        self.variant(RES::Ovs)
    }
}
#[doc = "Field `REF` reader - Single Channel Reference Selection"]
pub type REF_R = crate::FieldReader<REF>;
#[doc = "Single Channel Reference Selection\n\nValue on reset: 0"]
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
    #[doc = "6: VFS = 2xAVDD with AVDD as the reference source"]
    _2xvdd = 6,
    #[doc = "7: Use SINGLECTRLX to configure reference"]
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
impl REF_R {
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
    #[doc = "VFS = 2xAVDD with AVDD as the reference source"]
    #[inline(always)]
    pub fn is_2xvdd(&self) -> bool {
        *self == REF::_2xvdd
    }
    #[doc = "Use SINGLECTRLX to configure reference"]
    #[inline(always)]
    pub fn is_conf(&self) -> bool {
        *self == REF::Conf
    }
}
#[doc = "Field `REF` writer - Single Channel Reference Selection"]
pub type REF_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3, REF>;
impl<'a, REG> REF_W<'a, REG>
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
    #[doc = "VFS = 2xAVDD with AVDD as the reference source"]
    #[inline(always)]
    pub fn _2xvdd(self) -> &'a mut crate::W<REG> {
        self.variant(REF::_2xvdd)
    }
    #[doc = "Use SINGLECTRLX to configure reference"]
    #[inline(always)]
    pub fn conf(self) -> &'a mut crate::W<REG> {
        self.variant(REF::Conf)
    }
}
#[doc = "Field `POSSEL` reader - Single Channel Positive Input Selection"]
pub type POSSEL_R = crate::FieldReader;
#[doc = "Field `POSSEL` writer - Single Channel Positive Input Selection"]
pub type POSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NEGSEL` reader - Single Channel Negative Input Selection"]
pub type NEGSEL_R = crate::FieldReader;
#[doc = "Field `NEGSEL` writer - Single Channel Negative Input Selection"]
pub type NEGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AT` reader - Single Channel Acquisition Time"]
pub type AT_R = crate::FieldReader<AT>;
#[doc = "Single Channel Acquisition Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AT {
    #[doc = "0: 1 conversion clock cycle acquisition time for single channel"]
    _1cycle = 0,
    #[doc = "1: 2 conversion clock cycles acquisition time for single channel"]
    _2cycles = 1,
    #[doc = "2: 3 conversion clock cycles acquisition time for single channel"]
    _3cycles = 2,
    #[doc = "3: 4 conversion clock cycles acquisition time for single channel"]
    _4cycles = 3,
    #[doc = "4: 8 conversion clock cycles acquisition time for single channel"]
    _8cycles = 4,
    #[doc = "5: 16 conversion clock cycles acquisition time for single channel"]
    _16cycles = 5,
    #[doc = "6: 32 conversion clock cycles acquisition time for single channel"]
    _32cycles = 6,
    #[doc = "7: 64 conversion clock cycles acquisition time for single channel"]
    _64cycles = 7,
    #[doc = "8: 128 conversion clock cycles acquisition time for single channel"]
    _128cycles = 8,
    #[doc = "9: 256 conversion clock cycles acquisition time for single channel"]
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
impl AT_R {
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
    #[doc = "1 conversion clock cycle acquisition time for single channel"]
    #[inline(always)]
    pub fn is_1cycle(&self) -> bool {
        *self == AT::_1cycle
    }
    #[doc = "2 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn is_2cycles(&self) -> bool {
        *self == AT::_2cycles
    }
    #[doc = "3 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn is_3cycles(&self) -> bool {
        *self == AT::_3cycles
    }
    #[doc = "4 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn is_4cycles(&self) -> bool {
        *self == AT::_4cycles
    }
    #[doc = "8 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn is_8cycles(&self) -> bool {
        *self == AT::_8cycles
    }
    #[doc = "16 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn is_16cycles(&self) -> bool {
        *self == AT::_16cycles
    }
    #[doc = "32 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn is_32cycles(&self) -> bool {
        *self == AT::_32cycles
    }
    #[doc = "64 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn is_64cycles(&self) -> bool {
        *self == AT::_64cycles
    }
    #[doc = "128 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn is_128cycles(&self) -> bool {
        *self == AT::_128cycles
    }
    #[doc = "256 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn is_256cycles(&self) -> bool {
        *self == AT::_256cycles
    }
}
#[doc = "Field `AT` writer - Single Channel Acquisition Time"]
pub type AT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AT>;
impl<'a, REG> AT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 conversion clock cycle acquisition time for single channel"]
    #[inline(always)]
    pub fn _1cycle(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_1cycle)
    }
    #[doc = "2 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _2cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_2cycles)
    }
    #[doc = "3 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _3cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_3cycles)
    }
    #[doc = "4 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _4cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_4cycles)
    }
    #[doc = "8 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _8cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_8cycles)
    }
    #[doc = "16 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _16cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_16cycles)
    }
    #[doc = "32 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _32cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_32cycles)
    }
    #[doc = "64 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _64cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_64cycles)
    }
    #[doc = "128 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _128cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_128cycles)
    }
    #[doc = "256 conversion clock cycles acquisition time for single channel"]
    #[inline(always)]
    pub fn _256cycles(self) -> &'a mut crate::W<REG> {
        self.variant(AT::_256cycles)
    }
}
#[doc = "Field `PRSEN` reader - Single Channel PRS Trigger Enable"]
pub type PRSEN_R = crate::BitReader;
#[doc = "Field `PRSEN` writer - Single Channel PRS Trigger Enable"]
pub type PRSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN` reader - Compare Logic Enable for Single Channel"]
pub type CMPEN_R = crate::BitReader;
#[doc = "Field `CMPEN` writer - Compare Logic Enable for Single Channel"]
pub type CMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single Channel Repetitive Mode"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single Channel Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single Channel Result Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Single Channel Resolution Select"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Single Channel Reference Selection"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Single Channel Positive Input Selection"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Single Channel Negative Input Selection"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Single Channel Acquisition Time"]
    #[inline(always)]
    pub fn at(&self) -> AT_R {
        AT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Single Channel PRS Trigger Enable"]
    #[inline(always)]
    pub fn prsen(&self) -> PRSEN_R {
        PRSEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Compare Logic Enable for Single Channel"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single Channel Repetitive Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<SINGLECTRLrs> {
        REP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Single Channel Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<SINGLECTRLrs> {
        DIFF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Single Channel Result Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> ADJ_W<SINGLECTRLrs> {
        ADJ_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Single Channel Resolution Select"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<SINGLECTRLrs> {
        RES_W::new(self, 3)
    }
    #[doc = "Bits 5:7 - Single Channel Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> REF_W<SINGLECTRLrs> {
        REF_W::new(self, 5)
    }
    #[doc = "Bits 8:15 - Single Channel Positive Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn possel(&mut self) -> POSSEL_W<SINGLECTRLrs> {
        POSSEL_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Single Channel Negative Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn negsel(&mut self) -> NEGSEL_W<SINGLECTRLrs> {
        NEGSEL_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Single Channel Acquisition Time"]
    #[inline(always)]
    #[must_use]
    pub fn at(&mut self) -> AT_W<SINGLECTRLrs> {
        AT_W::new(self, 24)
    }
    #[doc = "Bit 29 - Single Channel PRS Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsen(&mut self) -> PRSEN_W<SINGLECTRLrs> {
        PRSEN_W::new(self, 29)
    }
    #[doc = "Bit 31 - Compare Logic Enable for Single Channel"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<SINGLECTRLrs> {
        CMPEN_W::new(self, 31)
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
#[doc = "Single Channel Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINGLECTRLrs;
impl crate::RegisterSpec for SINGLECTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlectrl::R`](R) reader structure"]
impl crate::Readable for SINGLECTRLrs {}
#[doc = "`write(|w| ..)` method takes [`singlectrl::W`](W) writer structure"]
impl crate::Writable for SINGLECTRLrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SINGLECTRL to value 0x00ff_ff00"]
impl crate::Resettable for SINGLECTRLrs {
    const RESET_VALUE: u32 = 0x00ff_ff00;
}
