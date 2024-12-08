///Register `INPUTSEL` reader
pub type R = crate::R<INPUTSELrs>;
///Register `INPUTSEL` writer
pub type W = crate::W<INPUTSELrs>;
///Field `POSSEL` reader - Positive Input Select
pub type PosselR = crate::FieldReader;
///Field `POSSEL` writer - Positive Input Select
pub type PosselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `NEGSEL` reader - Negative Input Select
pub type NegselR = crate::FieldReader;
///Field `NEGSEL` writer - Negative Input Select
pub type NegselW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///VA Selection
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VASEL {
    ///0: ACMPVDD
    Vdd = 0,
    ///1: APORT2Y Channel 0
    Aport2ych0 = 1,
    ///3: APORT2Y Channel 2
    Aport2ych2 = 3,
    ///5: APORT2Y Channel 4
    Aport2ych4 = 5,
    ///7: APORT2Y Channel 6
    Aport2ych6 = 7,
    ///9: APORT2Y Channel 8
    Aport2ych8 = 9,
    ///11: APORT2Y Channel 10
    Aport2ych10 = 11,
    ///13: APORT2Y Channel 12
    Aport2ych12 = 13,
    ///15: APORT2Y Channel 14
    Aport2ych14 = 15,
    ///17: APORT2Y Channel 16
    Aport2ych16 = 17,
    ///19: APORT2Y Channel 18
    Aport2ych18 = 19,
    ///21: APORT2Y Channel 20
    Aport2ych20 = 21,
    ///23: APORT2Y Channel 22
    Aport2ych22 = 23,
    ///25: APORT2Y Channel 24
    Aport2ych24 = 25,
    ///27: APORT2Y Channel 26
    Aport2ych26 = 27,
    ///29: APORT2Y Channel 28
    Aport2ych28 = 29,
    ///31: APORT2Y Channel 30
    Aport2ych30 = 31,
    ///32: APORT1X Channel 0
    Aport1xch0 = 32,
    ///33: APORT1Y Channel 1
    Aport1ych1 = 33,
    ///34: APORT1X Channel 2
    Aport1xch2 = 34,
    ///35: APORT1Y Channel 3
    Aport1ych3 = 35,
    ///36: APORT1X Channel 4
    Aport1xch4 = 36,
    ///37: APORT1Y Channel 5
    Aport1ych5 = 37,
    ///38: APORT1X Channel 6
    Aport1xch6 = 38,
    ///39: APORT1Y Channel 7
    Aport1ych7 = 39,
    ///40: APORT1X Channel 8
    Aport1xch8 = 40,
    ///41: APORT1Y Channel 9
    Aport1ych9 = 41,
    ///42: APORT1X Channel 10
    Aport1xch10 = 42,
    ///43: APORT1Y Channel 11
    Aport1ych11 = 43,
    ///44: APORT1X Channel 12
    Aport1xch12 = 44,
    ///45: APORT1Y Channel 13
    Aport1ych13 = 45,
    ///46: APORT1X Channel 14
    Aport1xch14 = 46,
    ///47: APORT1Y Channel 15
    Aport1ych15 = 47,
    ///48: APORT1X Channel 16
    Aport1xch16 = 48,
    ///49: APORT1Y Channel 17
    Aport1ych17 = 49,
    ///50: APORT1X Channel 18
    Aport1xch18 = 50,
    ///51: APORT1Y Channel 19
    Aport1ych19 = 51,
    ///52: APORT1X Channel 20
    Aport1xch20 = 52,
    ///53: APORT1Y Channel 21
    Aport1ych21 = 53,
    ///54: APORT1X Channel 22
    Aport1xch22 = 54,
    ///55: APORT1Y Channel 23
    Aport1ych23 = 55,
    ///56: APORT1X Channel 24
    Aport1xch24 = 56,
    ///57: APORT1Y Channel 25
    Aport1ych25 = 57,
    ///58: APORT1X Channel 26
    Aport1xch26 = 58,
    ///59: APORT1Y Channel 27
    Aport1ych27 = 59,
    ///60: APORT1X Channel 28
    Aport1xch28 = 60,
    ///61: APORT1Y Channel 29
    Aport1ych29 = 61,
    ///62: APORT1X Channel 30
    Aport1xch30 = 62,
    ///63: APORT1Y Channel 31
    Aport1ych31 = 63,
}
impl From<VASEL> for u8 {
    #[inline(always)]
    fn from(variant: VASEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VASEL {
    type Ux = u8;
}
impl crate::IsEnum for VASEL {}
///Field `VASEL` reader - VA Selection
pub type VaselR = crate::FieldReader<VASEL>;
impl VaselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<VASEL> {
        match self.bits {
            0 => Some(VASEL::Vdd),
            1 => Some(VASEL::Aport2ych0),
            3 => Some(VASEL::Aport2ych2),
            5 => Some(VASEL::Aport2ych4),
            7 => Some(VASEL::Aport2ych6),
            9 => Some(VASEL::Aport2ych8),
            11 => Some(VASEL::Aport2ych10),
            13 => Some(VASEL::Aport2ych12),
            15 => Some(VASEL::Aport2ych14),
            17 => Some(VASEL::Aport2ych16),
            19 => Some(VASEL::Aport2ych18),
            21 => Some(VASEL::Aport2ych20),
            23 => Some(VASEL::Aport2ych22),
            25 => Some(VASEL::Aport2ych24),
            27 => Some(VASEL::Aport2ych26),
            29 => Some(VASEL::Aport2ych28),
            31 => Some(VASEL::Aport2ych30),
            32 => Some(VASEL::Aport1xch0),
            33 => Some(VASEL::Aport1ych1),
            34 => Some(VASEL::Aport1xch2),
            35 => Some(VASEL::Aport1ych3),
            36 => Some(VASEL::Aport1xch4),
            37 => Some(VASEL::Aport1ych5),
            38 => Some(VASEL::Aport1xch6),
            39 => Some(VASEL::Aport1ych7),
            40 => Some(VASEL::Aport1xch8),
            41 => Some(VASEL::Aport1ych9),
            42 => Some(VASEL::Aport1xch10),
            43 => Some(VASEL::Aport1ych11),
            44 => Some(VASEL::Aport1xch12),
            45 => Some(VASEL::Aport1ych13),
            46 => Some(VASEL::Aport1xch14),
            47 => Some(VASEL::Aport1ych15),
            48 => Some(VASEL::Aport1xch16),
            49 => Some(VASEL::Aport1ych17),
            50 => Some(VASEL::Aport1xch18),
            51 => Some(VASEL::Aport1ych19),
            52 => Some(VASEL::Aport1xch20),
            53 => Some(VASEL::Aport1ych21),
            54 => Some(VASEL::Aport1xch22),
            55 => Some(VASEL::Aport1ych23),
            56 => Some(VASEL::Aport1xch24),
            57 => Some(VASEL::Aport1ych25),
            58 => Some(VASEL::Aport1xch26),
            59 => Some(VASEL::Aport1ych27),
            60 => Some(VASEL::Aport1xch28),
            61 => Some(VASEL::Aport1ych29),
            62 => Some(VASEL::Aport1xch30),
            63 => Some(VASEL::Aport1ych31),
            _ => None,
        }
    }
    ///ACMPVDD
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == VASEL::Vdd
    }
    ///APORT2Y Channel 0
    #[inline(always)]
    pub fn is_aport2ych0(&self) -> bool {
        *self == VASEL::Aport2ych0
    }
    ///APORT2Y Channel 2
    #[inline(always)]
    pub fn is_aport2ych2(&self) -> bool {
        *self == VASEL::Aport2ych2
    }
    ///APORT2Y Channel 4
    #[inline(always)]
    pub fn is_aport2ych4(&self) -> bool {
        *self == VASEL::Aport2ych4
    }
    ///APORT2Y Channel 6
    #[inline(always)]
    pub fn is_aport2ych6(&self) -> bool {
        *self == VASEL::Aport2ych6
    }
    ///APORT2Y Channel 8
    #[inline(always)]
    pub fn is_aport2ych8(&self) -> bool {
        *self == VASEL::Aport2ych8
    }
    ///APORT2Y Channel 10
    #[inline(always)]
    pub fn is_aport2ych10(&self) -> bool {
        *self == VASEL::Aport2ych10
    }
    ///APORT2Y Channel 12
    #[inline(always)]
    pub fn is_aport2ych12(&self) -> bool {
        *self == VASEL::Aport2ych12
    }
    ///APORT2Y Channel 14
    #[inline(always)]
    pub fn is_aport2ych14(&self) -> bool {
        *self == VASEL::Aport2ych14
    }
    ///APORT2Y Channel 16
    #[inline(always)]
    pub fn is_aport2ych16(&self) -> bool {
        *self == VASEL::Aport2ych16
    }
    ///APORT2Y Channel 18
    #[inline(always)]
    pub fn is_aport2ych18(&self) -> bool {
        *self == VASEL::Aport2ych18
    }
    ///APORT2Y Channel 20
    #[inline(always)]
    pub fn is_aport2ych20(&self) -> bool {
        *self == VASEL::Aport2ych20
    }
    ///APORT2Y Channel 22
    #[inline(always)]
    pub fn is_aport2ych22(&self) -> bool {
        *self == VASEL::Aport2ych22
    }
    ///APORT2Y Channel 24
    #[inline(always)]
    pub fn is_aport2ych24(&self) -> bool {
        *self == VASEL::Aport2ych24
    }
    ///APORT2Y Channel 26
    #[inline(always)]
    pub fn is_aport2ych26(&self) -> bool {
        *self == VASEL::Aport2ych26
    }
    ///APORT2Y Channel 28
    #[inline(always)]
    pub fn is_aport2ych28(&self) -> bool {
        *self == VASEL::Aport2ych28
    }
    ///APORT2Y Channel 30
    #[inline(always)]
    pub fn is_aport2ych30(&self) -> bool {
        *self == VASEL::Aport2ych30
    }
    ///APORT1X Channel 0
    #[inline(always)]
    pub fn is_aport1xch0(&self) -> bool {
        *self == VASEL::Aport1xch0
    }
    ///APORT1Y Channel 1
    #[inline(always)]
    pub fn is_aport1ych1(&self) -> bool {
        *self == VASEL::Aport1ych1
    }
    ///APORT1X Channel 2
    #[inline(always)]
    pub fn is_aport1xch2(&self) -> bool {
        *self == VASEL::Aport1xch2
    }
    ///APORT1Y Channel 3
    #[inline(always)]
    pub fn is_aport1ych3(&self) -> bool {
        *self == VASEL::Aport1ych3
    }
    ///APORT1X Channel 4
    #[inline(always)]
    pub fn is_aport1xch4(&self) -> bool {
        *self == VASEL::Aport1xch4
    }
    ///APORT1Y Channel 5
    #[inline(always)]
    pub fn is_aport1ych5(&self) -> bool {
        *self == VASEL::Aport1ych5
    }
    ///APORT1X Channel 6
    #[inline(always)]
    pub fn is_aport1xch6(&self) -> bool {
        *self == VASEL::Aport1xch6
    }
    ///APORT1Y Channel 7
    #[inline(always)]
    pub fn is_aport1ych7(&self) -> bool {
        *self == VASEL::Aport1ych7
    }
    ///APORT1X Channel 8
    #[inline(always)]
    pub fn is_aport1xch8(&self) -> bool {
        *self == VASEL::Aport1xch8
    }
    ///APORT1Y Channel 9
    #[inline(always)]
    pub fn is_aport1ych9(&self) -> bool {
        *self == VASEL::Aport1ych9
    }
    ///APORT1X Channel 10
    #[inline(always)]
    pub fn is_aport1xch10(&self) -> bool {
        *self == VASEL::Aport1xch10
    }
    ///APORT1Y Channel 11
    #[inline(always)]
    pub fn is_aport1ych11(&self) -> bool {
        *self == VASEL::Aport1ych11
    }
    ///APORT1X Channel 12
    #[inline(always)]
    pub fn is_aport1xch12(&self) -> bool {
        *self == VASEL::Aport1xch12
    }
    ///APORT1Y Channel 13
    #[inline(always)]
    pub fn is_aport1ych13(&self) -> bool {
        *self == VASEL::Aport1ych13
    }
    ///APORT1X Channel 14
    #[inline(always)]
    pub fn is_aport1xch14(&self) -> bool {
        *self == VASEL::Aport1xch14
    }
    ///APORT1Y Channel 15
    #[inline(always)]
    pub fn is_aport1ych15(&self) -> bool {
        *self == VASEL::Aport1ych15
    }
    ///APORT1X Channel 16
    #[inline(always)]
    pub fn is_aport1xch16(&self) -> bool {
        *self == VASEL::Aport1xch16
    }
    ///APORT1Y Channel 17
    #[inline(always)]
    pub fn is_aport1ych17(&self) -> bool {
        *self == VASEL::Aport1ych17
    }
    ///APORT1X Channel 18
    #[inline(always)]
    pub fn is_aport1xch18(&self) -> bool {
        *self == VASEL::Aport1xch18
    }
    ///APORT1Y Channel 19
    #[inline(always)]
    pub fn is_aport1ych19(&self) -> bool {
        *self == VASEL::Aport1ych19
    }
    ///APORT1X Channel 20
    #[inline(always)]
    pub fn is_aport1xch20(&self) -> bool {
        *self == VASEL::Aport1xch20
    }
    ///APORT1Y Channel 21
    #[inline(always)]
    pub fn is_aport1ych21(&self) -> bool {
        *self == VASEL::Aport1ych21
    }
    ///APORT1X Channel 22
    #[inline(always)]
    pub fn is_aport1xch22(&self) -> bool {
        *self == VASEL::Aport1xch22
    }
    ///APORT1Y Channel 23
    #[inline(always)]
    pub fn is_aport1ych23(&self) -> bool {
        *self == VASEL::Aport1ych23
    }
    ///APORT1X Channel 24
    #[inline(always)]
    pub fn is_aport1xch24(&self) -> bool {
        *self == VASEL::Aport1xch24
    }
    ///APORT1Y Channel 25
    #[inline(always)]
    pub fn is_aport1ych25(&self) -> bool {
        *self == VASEL::Aport1ych25
    }
    ///APORT1X Channel 26
    #[inline(always)]
    pub fn is_aport1xch26(&self) -> bool {
        *self == VASEL::Aport1xch26
    }
    ///APORT1Y Channel 27
    #[inline(always)]
    pub fn is_aport1ych27(&self) -> bool {
        *self == VASEL::Aport1ych27
    }
    ///APORT1X Channel 28
    #[inline(always)]
    pub fn is_aport1xch28(&self) -> bool {
        *self == VASEL::Aport1xch28
    }
    ///APORT1Y Channel 29
    #[inline(always)]
    pub fn is_aport1ych29(&self) -> bool {
        *self == VASEL::Aport1ych29
    }
    ///APORT1X Channel 30
    #[inline(always)]
    pub fn is_aport1xch30(&self) -> bool {
        *self == VASEL::Aport1xch30
    }
    ///APORT1Y Channel 31
    #[inline(always)]
    pub fn is_aport1ych31(&self) -> bool {
        *self == VASEL::Aport1ych31
    }
}
///Field `VASEL` writer - VA Selection
pub type VaselW<'a, REG> = crate::FieldWriter<'a, REG, 6, VASEL>;
impl<'a, REG> VaselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ACMPVDD
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Vdd)
    }
    ///APORT2Y Channel 0
    #[inline(always)]
    pub fn aport2ych0(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych0)
    }
    ///APORT2Y Channel 2
    #[inline(always)]
    pub fn aport2ych2(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych2)
    }
    ///APORT2Y Channel 4
    #[inline(always)]
    pub fn aport2ych4(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych4)
    }
    ///APORT2Y Channel 6
    #[inline(always)]
    pub fn aport2ych6(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych6)
    }
    ///APORT2Y Channel 8
    #[inline(always)]
    pub fn aport2ych8(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych8)
    }
    ///APORT2Y Channel 10
    #[inline(always)]
    pub fn aport2ych10(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych10)
    }
    ///APORT2Y Channel 12
    #[inline(always)]
    pub fn aport2ych12(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych12)
    }
    ///APORT2Y Channel 14
    #[inline(always)]
    pub fn aport2ych14(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych14)
    }
    ///APORT2Y Channel 16
    #[inline(always)]
    pub fn aport2ych16(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych16)
    }
    ///APORT2Y Channel 18
    #[inline(always)]
    pub fn aport2ych18(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych18)
    }
    ///APORT2Y Channel 20
    #[inline(always)]
    pub fn aport2ych20(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych20)
    }
    ///APORT2Y Channel 22
    #[inline(always)]
    pub fn aport2ych22(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych22)
    }
    ///APORT2Y Channel 24
    #[inline(always)]
    pub fn aport2ych24(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych24)
    }
    ///APORT2Y Channel 26
    #[inline(always)]
    pub fn aport2ych26(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych26)
    }
    ///APORT2Y Channel 28
    #[inline(always)]
    pub fn aport2ych28(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych28)
    }
    ///APORT2Y Channel 30
    #[inline(always)]
    pub fn aport2ych30(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport2ych30)
    }
    ///APORT1X Channel 0
    #[inline(always)]
    pub fn aport1xch0(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch0)
    }
    ///APORT1Y Channel 1
    #[inline(always)]
    pub fn aport1ych1(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych1)
    }
    ///APORT1X Channel 2
    #[inline(always)]
    pub fn aport1xch2(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch2)
    }
    ///APORT1Y Channel 3
    #[inline(always)]
    pub fn aport1ych3(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych3)
    }
    ///APORT1X Channel 4
    #[inline(always)]
    pub fn aport1xch4(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch4)
    }
    ///APORT1Y Channel 5
    #[inline(always)]
    pub fn aport1ych5(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych5)
    }
    ///APORT1X Channel 6
    #[inline(always)]
    pub fn aport1xch6(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch6)
    }
    ///APORT1Y Channel 7
    #[inline(always)]
    pub fn aport1ych7(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych7)
    }
    ///APORT1X Channel 8
    #[inline(always)]
    pub fn aport1xch8(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch8)
    }
    ///APORT1Y Channel 9
    #[inline(always)]
    pub fn aport1ych9(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych9)
    }
    ///APORT1X Channel 10
    #[inline(always)]
    pub fn aport1xch10(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch10)
    }
    ///APORT1Y Channel 11
    #[inline(always)]
    pub fn aport1ych11(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych11)
    }
    ///APORT1X Channel 12
    #[inline(always)]
    pub fn aport1xch12(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch12)
    }
    ///APORT1Y Channel 13
    #[inline(always)]
    pub fn aport1ych13(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych13)
    }
    ///APORT1X Channel 14
    #[inline(always)]
    pub fn aport1xch14(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch14)
    }
    ///APORT1Y Channel 15
    #[inline(always)]
    pub fn aport1ych15(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych15)
    }
    ///APORT1X Channel 16
    #[inline(always)]
    pub fn aport1xch16(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch16)
    }
    ///APORT1Y Channel 17
    #[inline(always)]
    pub fn aport1ych17(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych17)
    }
    ///APORT1X Channel 18
    #[inline(always)]
    pub fn aport1xch18(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch18)
    }
    ///APORT1Y Channel 19
    #[inline(always)]
    pub fn aport1ych19(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych19)
    }
    ///APORT1X Channel 20
    #[inline(always)]
    pub fn aport1xch20(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch20)
    }
    ///APORT1Y Channel 21
    #[inline(always)]
    pub fn aport1ych21(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych21)
    }
    ///APORT1X Channel 22
    #[inline(always)]
    pub fn aport1xch22(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch22)
    }
    ///APORT1Y Channel 23
    #[inline(always)]
    pub fn aport1ych23(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych23)
    }
    ///APORT1X Channel 24
    #[inline(always)]
    pub fn aport1xch24(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch24)
    }
    ///APORT1Y Channel 25
    #[inline(always)]
    pub fn aport1ych25(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych25)
    }
    ///APORT1X Channel 26
    #[inline(always)]
    pub fn aport1xch26(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch26)
    }
    ///APORT1Y Channel 27
    #[inline(always)]
    pub fn aport1ych27(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych27)
    }
    ///APORT1X Channel 28
    #[inline(always)]
    pub fn aport1xch28(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch28)
    }
    ///APORT1Y Channel 29
    #[inline(always)]
    pub fn aport1ych29(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych29)
    }
    ///APORT1X Channel 30
    #[inline(always)]
    pub fn aport1xch30(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1xch30)
    }
    ///APORT1Y Channel 31
    #[inline(always)]
    pub fn aport1ych31(self) -> &'a mut crate::W<REG> {
        self.variant(VASEL::Aport1ych31)
    }
}
///Field `VBSEL` reader - VB Selection
pub type VbselR = crate::BitReader;
///Field `VBSEL` writer - VB Selection
pub type VbselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VLPSEL` reader - Low-Power Sampled Voltage Selection
pub type VlpselR = crate::BitReader;
///Field `VLPSEL` writer - Low-Power Sampled Voltage Selection
pub type VlpselW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSRESEN` reader - Capacitive Sense Mode Internal Resistor Enable
pub type CsresenR = crate::BitReader;
///Field `CSRESEN` writer - Capacitive Sense Mode Internal Resistor Enable
pub type CsresenW<'a, REG> = crate::BitWriter<'a, REG>;
///Capacitive Sense Mode Internal Resistor Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSRESSEL {
    ///0: Internal capacitive sense resistor value 0
    Res0 = 0,
    ///1: Internal capacitive sense resistor value 1
    Res1 = 1,
    ///2: Internal capacitive sense resistor value 2
    Res2 = 2,
    ///3: Internal capacitive sense resistor value 3
    Res3 = 3,
    ///4: Internal capacitive sense resistor value 4
    Res4 = 4,
    ///5: Internal capacitive sense resistor value 5
    Res5 = 5,
    ///6: Internal capacitive sense resistor value 6
    Res6 = 6,
    ///7: Internal capacitive sense resistor value 7
    Res7 = 7,
}
impl From<CSRESSEL> for u8 {
    #[inline(always)]
    fn from(variant: CSRESSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CSRESSEL {
    type Ux = u8;
}
impl crate::IsEnum for CSRESSEL {}
///Field `CSRESSEL` reader - Capacitive Sense Mode Internal Resistor Select
pub type CsresselR = crate::FieldReader<CSRESSEL>;
impl CsresselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CSRESSEL {
        match self.bits {
            0 => CSRESSEL::Res0,
            1 => CSRESSEL::Res1,
            2 => CSRESSEL::Res2,
            3 => CSRESSEL::Res3,
            4 => CSRESSEL::Res4,
            5 => CSRESSEL::Res5,
            6 => CSRESSEL::Res6,
            7 => CSRESSEL::Res7,
            _ => unreachable!(),
        }
    }
    ///Internal capacitive sense resistor value 0
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == CSRESSEL::Res0
    }
    ///Internal capacitive sense resistor value 1
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == CSRESSEL::Res1
    }
    ///Internal capacitive sense resistor value 2
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == CSRESSEL::Res2
    }
    ///Internal capacitive sense resistor value 3
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == CSRESSEL::Res3
    }
    ///Internal capacitive sense resistor value 4
    #[inline(always)]
    pub fn is_res4(&self) -> bool {
        *self == CSRESSEL::Res4
    }
    ///Internal capacitive sense resistor value 5
    #[inline(always)]
    pub fn is_res5(&self) -> bool {
        *self == CSRESSEL::Res5
    }
    ///Internal capacitive sense resistor value 6
    #[inline(always)]
    pub fn is_res6(&self) -> bool {
        *self == CSRESSEL::Res6
    }
    ///Internal capacitive sense resistor value 7
    #[inline(always)]
    pub fn is_res7(&self) -> bool {
        *self == CSRESSEL::Res7
    }
}
///Field `CSRESSEL` writer - Capacitive Sense Mode Internal Resistor Select
pub type CsresselW<'a, REG> = crate::FieldWriter<'a, REG, 3, CSRESSEL, crate::Safe>;
impl<'a, REG> CsresselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Internal capacitive sense resistor value 0
    #[inline(always)]
    pub fn res0(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL::Res0)
    }
    ///Internal capacitive sense resistor value 1
    #[inline(always)]
    pub fn res1(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL::Res1)
    }
    ///Internal capacitive sense resistor value 2
    #[inline(always)]
    pub fn res2(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL::Res2)
    }
    ///Internal capacitive sense resistor value 3
    #[inline(always)]
    pub fn res3(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL::Res3)
    }
    ///Internal capacitive sense resistor value 4
    #[inline(always)]
    pub fn res4(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL::Res4)
    }
    ///Internal capacitive sense resistor value 5
    #[inline(always)]
    pub fn res5(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL::Res5)
    }
    ///Internal capacitive sense resistor value 6
    #[inline(always)]
    pub fn res6(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL::Res6)
    }
    ///Internal capacitive sense resistor value 7
    #[inline(always)]
    pub fn res7(self) -> &'a mut crate::W<REG> {
        self.variant(CSRESSEL::Res7)
    }
}
impl R {
    ///Bits 0:7 - Positive Input Select
    #[inline(always)]
    pub fn possel(&self) -> PosselR {
        PosselR::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Negative Input Select
    #[inline(always)]
    pub fn negsel(&self) -> NegselR {
        NegselR::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:21 - VA Selection
    #[inline(always)]
    pub fn vasel(&self) -> VaselR {
        VaselR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bit 22 - VB Selection
    #[inline(always)]
    pub fn vbsel(&self) -> VbselR {
        VbselR::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Low-Power Sampled Voltage Selection
    #[inline(always)]
    pub fn vlpsel(&self) -> VlpselR {
        VlpselR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - Capacitive Sense Mode Internal Resistor Enable
    #[inline(always)]
    pub fn csresen(&self) -> CsresenR {
        CsresenR::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 28:30 - Capacitive Sense Mode Internal Resistor Select
    #[inline(always)]
    pub fn csressel(&self) -> CsresselR {
        CsresselR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INPUTSEL")
            .field("possel", &self.possel())
            .field("negsel", &self.negsel())
            .field("vasel", &self.vasel())
            .field("vbsel", &self.vbsel())
            .field("vlpsel", &self.vlpsel())
            .field("csresen", &self.csresen())
            .field("csressel", &self.csressel())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Positive Input Select
    #[inline(always)]
    #[must_use]
    pub fn possel(&mut self) -> PosselW<INPUTSELrs> {
        PosselW::new(self, 0)
    }
    ///Bits 8:15 - Negative Input Select
    #[inline(always)]
    #[must_use]
    pub fn negsel(&mut self) -> NegselW<INPUTSELrs> {
        NegselW::new(self, 8)
    }
    ///Bits 16:21 - VA Selection
    #[inline(always)]
    #[must_use]
    pub fn vasel(&mut self) -> VaselW<INPUTSELrs> {
        VaselW::new(self, 16)
    }
    ///Bit 22 - VB Selection
    #[inline(always)]
    #[must_use]
    pub fn vbsel(&mut self) -> VbselW<INPUTSELrs> {
        VbselW::new(self, 22)
    }
    ///Bit 24 - Low-Power Sampled Voltage Selection
    #[inline(always)]
    #[must_use]
    pub fn vlpsel(&mut self) -> VlpselW<INPUTSELrs> {
        VlpselW::new(self, 24)
    }
    ///Bit 26 - Capacitive Sense Mode Internal Resistor Enable
    #[inline(always)]
    #[must_use]
    pub fn csresen(&mut self) -> CsresenW<INPUTSELrs> {
        CsresenW::new(self, 26)
    }
    ///Bits 28:30 - Capacitive Sense Mode Internal Resistor Select
    #[inline(always)]
    #[must_use]
    pub fn csressel(&mut self) -> CsresselW<INPUTSELrs> {
        CsresselW::new(self, 28)
    }
}
///Input Selection Register
///
///You can [`read`](crate::Reg::read) this register and get [`inputsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct INPUTSELrs;
impl crate::RegisterSpec for INPUTSELrs {
    type Ux = u32;
}
///`read()` method returns [`inputsel::R`](R) reader structure
impl crate::Readable for INPUTSELrs {}
///`write(|w| ..)` method takes [`inputsel::W`](W) writer structure
impl crate::Writable for INPUTSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INPUTSEL to value 0
impl crate::Resettable for INPUTSELrs {
    const RESET_VALUE: u32 = 0;
}
