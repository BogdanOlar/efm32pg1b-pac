///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `EN` reader - I2C Enable
pub type EnR = crate::BitReader;
///Field `EN` writer - I2C Enable
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLAVE` reader - Addressable as Slave
pub type SlaveR = crate::BitReader;
///Field `SLAVE` writer - Addressable as Slave
pub type SlaveW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTOACK` reader - Automatic Acknowledge
pub type AutoackR = crate::BitReader;
///Field `AUTOACK` writer - Automatic Acknowledge
pub type AutoackW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTOSE` reader - Automatic STOP When Empty
pub type AutoseR = crate::BitReader;
///Field `AUTOSE` writer - Automatic STOP When Empty
pub type AutoseW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AUTOSN` reader - Automatic STOP on NACK
pub type AutosnR = crate::BitReader;
///Field `AUTOSN` writer - Automatic STOP on NACK
pub type AutosnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARBDIS` reader - Arbitration Disable
pub type ArbdisR = crate::BitReader;
///Field `ARBDIS` writer - Arbitration Disable
pub type ArbdisW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCAMEN` reader - General Call Address Match Enable
pub type GcamenR = crate::BitReader;
///Field `GCAMEN` writer - General Call Address Match Enable
pub type GcamenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXBIL` reader - TX Buffer Interrupt Level
pub type TxbilR = crate::BitReader;
///Field `TXBIL` writer - TX Buffer Interrupt Level
pub type TxbilW<'a, REG> = crate::BitWriter<'a, REG>;
///Clock Low High Ratio
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLHR {
    ///0: The ratio between low period and high period counters (Nlow:Nhigh) is 4:4
    Standard = 0,
    ///1: The ratio between low period and high period counters (Nlow:Nhigh) is 6:3
    Asymmetric = 1,
    ///2: The ratio between low period and high period counters (Nlow:Nhigh) is 11:6
    Fast = 2,
}
impl From<CLHR> for u8 {
    #[inline(always)]
    fn from(variant: CLHR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLHR {
    type Ux = u8;
}
impl crate::IsEnum for CLHR {}
///Field `CLHR` reader - Clock Low High Ratio
pub type ClhrR = crate::FieldReader<CLHR>;
impl ClhrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLHR> {
        match self.bits {
            0 => Some(CLHR::Standard),
            1 => Some(CLHR::Asymmetric),
            2 => Some(CLHR::Fast),
            _ => None,
        }
    }
    ///The ratio between low period and high period counters (Nlow:Nhigh) is 4:4
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == CLHR::Standard
    }
    ///The ratio between low period and high period counters (Nlow:Nhigh) is 6:3
    #[inline(always)]
    pub fn is_asymmetric(&self) -> bool {
        *self == CLHR::Asymmetric
    }
    ///The ratio between low period and high period counters (Nlow:Nhigh) is 11:6
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == CLHR::Fast
    }
}
///Field `CLHR` writer - Clock Low High Ratio
pub type ClhrW<'a, REG> = crate::FieldWriter<'a, REG, 2, CLHR>;
impl<'a, REG> ClhrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///The ratio between low period and high period counters (Nlow:Nhigh) is 4:4
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(CLHR::Standard)
    }
    ///The ratio between low period and high period counters (Nlow:Nhigh) is 6:3
    #[inline(always)]
    pub fn asymmetric(self) -> &'a mut crate::W<REG> {
        self.variant(CLHR::Asymmetric)
    }
    ///The ratio between low period and high period counters (Nlow:Nhigh) is 11:6
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(CLHR::Fast)
    }
}
///Bus Idle Timeout
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITO {
    ///0: Timeout disabled
    Off = 0,
    ///1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout.
    _40pcc = 1,
    ///2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout.
    _80pcc = 2,
    ///3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout.
    _160pcc = 3,
}
impl From<BITO> for u8 {
    #[inline(always)]
    fn from(variant: BITO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BITO {
    type Ux = u8;
}
impl crate::IsEnum for BITO {}
///Field `BITO` reader - Bus Idle Timeout
pub type BitoR = crate::FieldReader<BITO>;
impl BitoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BITO {
        match self.bits {
            0 => BITO::Off,
            1 => BITO::_40pcc,
            2 => BITO::_80pcc,
            3 => BITO::_160pcc,
            _ => unreachable!(),
        }
    }
    ///Timeout disabled
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == BITO::Off
    }
    ///Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout.
    #[inline(always)]
    pub fn is_40pcc(&self) -> bool {
        *self == BITO::_40pcc
    }
    ///Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout.
    #[inline(always)]
    pub fn is_80pcc(&self) -> bool {
        *self == BITO::_80pcc
    }
    ///Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout.
    #[inline(always)]
    pub fn is_160pcc(&self) -> bool {
        *self == BITO::_160pcc
    }
}
///Field `BITO` writer - Bus Idle Timeout
pub type BitoW<'a, REG> = crate::FieldWriter<'a, REG, 2, BITO, crate::Safe>;
impl<'a, REG> BitoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timeout disabled
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(BITO::Off)
    }
    ///Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout.
    #[inline(always)]
    pub fn _40pcc(self) -> &'a mut crate::W<REG> {
        self.variant(BITO::_40pcc)
    }
    ///Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout.
    #[inline(always)]
    pub fn _80pcc(self) -> &'a mut crate::W<REG> {
        self.variant(BITO::_80pcc)
    }
    ///Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout.
    #[inline(always)]
    pub fn _160pcc(self) -> &'a mut crate::W<REG> {
        self.variant(BITO::_160pcc)
    }
}
///Field `GIBITO` reader - Go Idle on Bus Idle Timeout
pub type GibitoR = crate::BitReader;
///Field `GIBITO` writer - Go Idle on Bus Idle Timeout
pub type GibitoW<'a, REG> = crate::BitWriter<'a, REG>;
///Clock Low Timeout
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLTO {
    ///0: Timeout disabled
    Off = 0,
    ///1: Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout.
    _40pcc = 1,
    ///2: Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout.
    _80pcc = 2,
    ///3: Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout.
    _160pcc = 3,
    ///4: Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout.
    _320pcc = 4,
    ///5: Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout.
    _1024pcc = 5,
}
impl From<CLTO> for u8 {
    #[inline(always)]
    fn from(variant: CLTO) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLTO {
    type Ux = u8;
}
impl crate::IsEnum for CLTO {}
///Field `CLTO` reader - Clock Low Timeout
pub type CltoR = crate::FieldReader<CLTO>;
impl CltoR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLTO> {
        match self.bits {
            0 => Some(CLTO::Off),
            1 => Some(CLTO::_40pcc),
            2 => Some(CLTO::_80pcc),
            3 => Some(CLTO::_160pcc),
            4 => Some(CLTO::_320pcc),
            5 => Some(CLTO::_1024pcc),
            _ => None,
        }
    }
    ///Timeout disabled
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CLTO::Off
    }
    ///Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout.
    #[inline(always)]
    pub fn is_40pcc(&self) -> bool {
        *self == CLTO::_40pcc
    }
    ///Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout.
    #[inline(always)]
    pub fn is_80pcc(&self) -> bool {
        *self == CLTO::_80pcc
    }
    ///Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout.
    #[inline(always)]
    pub fn is_160pcc(&self) -> bool {
        *self == CLTO::_160pcc
    }
    ///Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout.
    #[inline(always)]
    pub fn is_320pcc(&self) -> bool {
        *self == CLTO::_320pcc
    }
    ///Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout.
    #[inline(always)]
    pub fn is_1024pcc(&self) -> bool {
        *self == CLTO::_1024pcc
    }
}
///Field `CLTO` writer - Clock Low Timeout
pub type CltoW<'a, REG> = crate::FieldWriter<'a, REG, 3, CLTO>;
impl<'a, REG> CltoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timeout disabled
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO::Off)
    }
    ///Timeout after 40 prescaled clock cycles. In standard mode at 100 kHz, this results in a 50us timeout.
    #[inline(always)]
    pub fn _40pcc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO::_40pcc)
    }
    ///Timeout after 80 prescaled clock cycles. In standard mode at 100 kHz, this results in a 100us timeout.
    #[inline(always)]
    pub fn _80pcc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO::_80pcc)
    }
    ///Timeout after 160 prescaled clock cycles. In standard mode at 100 kHz, this results in a 200us timeout.
    #[inline(always)]
    pub fn _160pcc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO::_160pcc)
    }
    ///Timeout after 320 prescaled clock cycles. In standard mode at 100 kHz, this results in a 400us timeout.
    #[inline(always)]
    pub fn _320pcc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO::_320pcc)
    }
    ///Timeout after 1024 prescaled clock cycles. In standard mode at 100 kHz, this results in a 1280us timeout.
    #[inline(always)]
    pub fn _1024pcc(self) -> &'a mut crate::W<REG> {
        self.variant(CLTO::_1024pcc)
    }
}
impl R {
    ///Bit 0 - I2C Enable
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Addressable as Slave
    #[inline(always)]
    pub fn slave(&self) -> SlaveR {
        SlaveR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Automatic Acknowledge
    #[inline(always)]
    pub fn autoack(&self) -> AutoackR {
        AutoackR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Automatic STOP When Empty
    #[inline(always)]
    pub fn autose(&self) -> AutoseR {
        AutoseR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Automatic STOP on NACK
    #[inline(always)]
    pub fn autosn(&self) -> AutosnR {
        AutosnR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Arbitration Disable
    #[inline(always)]
    pub fn arbdis(&self) -> ArbdisR {
        ArbdisR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - General Call Address Match Enable
    #[inline(always)]
    pub fn gcamen(&self) -> GcamenR {
        GcamenR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TX Buffer Interrupt Level
    #[inline(always)]
    pub fn txbil(&self) -> TxbilR {
        TxbilR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Clock Low High Ratio
    #[inline(always)]
    pub fn clhr(&self) -> ClhrR {
        ClhrR::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Bus Idle Timeout
    #[inline(always)]
    pub fn bito(&self) -> BitoR {
        BitoR::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 15 - Go Idle on Bus Idle Timeout
    #[inline(always)]
    pub fn gibito(&self) -> GibitoR {
        GibitoR::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:18 - Clock Low Timeout
    #[inline(always)]
    pub fn clto(&self) -> CltoR {
        CltoR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("en", &self.en())
            .field("slave", &self.slave())
            .field("autoack", &self.autoack())
            .field("autose", &self.autose())
            .field("autosn", &self.autosn())
            .field("arbdis", &self.arbdis())
            .field("gcamen", &self.gcamen())
            .field("txbil", &self.txbil())
            .field("clhr", &self.clhr())
            .field("bito", &self.bito())
            .field("gibito", &self.gibito())
            .field("clto", &self.clto())
            .finish()
    }
}
impl W {
    ///Bit 0 - I2C Enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CTRLrs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - Addressable as Slave
    #[inline(always)]
    #[must_use]
    pub fn slave(&mut self) -> SlaveW<CTRLrs> {
        SlaveW::new(self, 1)
    }
    ///Bit 2 - Automatic Acknowledge
    #[inline(always)]
    #[must_use]
    pub fn autoack(&mut self) -> AutoackW<CTRLrs> {
        AutoackW::new(self, 2)
    }
    ///Bit 3 - Automatic STOP When Empty
    #[inline(always)]
    #[must_use]
    pub fn autose(&mut self) -> AutoseW<CTRLrs> {
        AutoseW::new(self, 3)
    }
    ///Bit 4 - Automatic STOP on NACK
    #[inline(always)]
    #[must_use]
    pub fn autosn(&mut self) -> AutosnW<CTRLrs> {
        AutosnW::new(self, 4)
    }
    ///Bit 5 - Arbitration Disable
    #[inline(always)]
    #[must_use]
    pub fn arbdis(&mut self) -> ArbdisW<CTRLrs> {
        ArbdisW::new(self, 5)
    }
    ///Bit 6 - General Call Address Match Enable
    #[inline(always)]
    #[must_use]
    pub fn gcamen(&mut self) -> GcamenW<CTRLrs> {
        GcamenW::new(self, 6)
    }
    ///Bit 7 - TX Buffer Interrupt Level
    #[inline(always)]
    #[must_use]
    pub fn txbil(&mut self) -> TxbilW<CTRLrs> {
        TxbilW::new(self, 7)
    }
    ///Bits 8:9 - Clock Low High Ratio
    #[inline(always)]
    #[must_use]
    pub fn clhr(&mut self) -> ClhrW<CTRLrs> {
        ClhrW::new(self, 8)
    }
    ///Bits 12:13 - Bus Idle Timeout
    #[inline(always)]
    #[must_use]
    pub fn bito(&mut self) -> BitoW<CTRLrs> {
        BitoW::new(self, 12)
    }
    ///Bit 15 - Go Idle on Bus Idle Timeout
    #[inline(always)]
    #[must_use]
    pub fn gibito(&mut self) -> GibitoW<CTRLrs> {
        GibitoW::new(self, 15)
    }
    ///Bits 16:18 - Clock Low Timeout
    #[inline(always)]
    #[must_use]
    pub fn clto(&mut self) -> CltoW<CTRLrs> {
        CltoW::new(self, 16)
    }
}
///Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
