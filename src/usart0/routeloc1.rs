#[doc = "Register `ROUTELOC1` reader"]
pub type R = crate::R<ROUTELOC1rs>;
#[doc = "Register `ROUTELOC1` writer"]
pub type W = crate::W<ROUTELOC1rs>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSLOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
    #[doc = "14: Location 14"]
    Loc14 = 14,
    #[doc = "15: Location 15"]
    Loc15 = 15,
    #[doc = "16: Location 16"]
    Loc16 = 16,
    #[doc = "17: Location 17"]
    Loc17 = 17,
    #[doc = "18: Location 18"]
    Loc18 = 18,
    #[doc = "19: Location 19"]
    Loc19 = 19,
    #[doc = "20: Location 20"]
    Loc20 = 20,
    #[doc = "21: Location 21"]
    Loc21 = 21,
    #[doc = "22: Location 22"]
    Loc22 = 22,
    #[doc = "23: Location 23"]
    Loc23 = 23,
    #[doc = "24: Location 24"]
    Loc24 = 24,
    #[doc = "25: Location 25"]
    Loc25 = 25,
    #[doc = "26: Location 26"]
    Loc26 = 26,
    #[doc = "27: Location 27"]
    Loc27 = 27,
    #[doc = "28: Location 28"]
    Loc28 = 28,
    #[doc = "29: Location 29"]
    Loc29 = 29,
    #[doc = "30: Location 30"]
    Loc30 = 30,
    #[doc = "31: Location 31"]
    Loc31 = 31,
}
impl From<CTSLOC> for u8 {
    #[inline(always)]
    fn from(variant: CTSLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSLOC {
    type Ux = u8;
}
impl crate::IsEnum for CTSLOC {}
#[doc = "Field `CTSLOC` reader - I/O Location"]
pub type CtslocR = crate::FieldReader<CTSLOC>;
impl CtslocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSLOC> {
        match self.bits {
            0 => Some(CTSLOC::Loc0),
            1 => Some(CTSLOC::Loc1),
            2 => Some(CTSLOC::Loc2),
            3 => Some(CTSLOC::Loc3),
            4 => Some(CTSLOC::Loc4),
            5 => Some(CTSLOC::Loc5),
            6 => Some(CTSLOC::Loc6),
            7 => Some(CTSLOC::Loc7),
            8 => Some(CTSLOC::Loc8),
            9 => Some(CTSLOC::Loc9),
            10 => Some(CTSLOC::Loc10),
            11 => Some(CTSLOC::Loc11),
            12 => Some(CTSLOC::Loc12),
            13 => Some(CTSLOC::Loc13),
            14 => Some(CTSLOC::Loc14),
            15 => Some(CTSLOC::Loc15),
            16 => Some(CTSLOC::Loc16),
            17 => Some(CTSLOC::Loc17),
            18 => Some(CTSLOC::Loc18),
            19 => Some(CTSLOC::Loc19),
            20 => Some(CTSLOC::Loc20),
            21 => Some(CTSLOC::Loc21),
            22 => Some(CTSLOC::Loc22),
            23 => Some(CTSLOC::Loc23),
            24 => Some(CTSLOC::Loc24),
            25 => Some(CTSLOC::Loc25),
            26 => Some(CTSLOC::Loc26),
            27 => Some(CTSLOC::Loc27),
            28 => Some(CTSLOC::Loc28),
            29 => Some(CTSLOC::Loc29),
            30 => Some(CTSLOC::Loc30),
            31 => Some(CTSLOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CTSLOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CTSLOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CTSLOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CTSLOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CTSLOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CTSLOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CTSLOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CTSLOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CTSLOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CTSLOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CTSLOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CTSLOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CTSLOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CTSLOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CTSLOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CTSLOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CTSLOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == CTSLOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == CTSLOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == CTSLOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == CTSLOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == CTSLOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == CTSLOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == CTSLOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == CTSLOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == CTSLOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == CTSLOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == CTSLOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == CTSLOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == CTSLOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == CTSLOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == CTSLOC::Loc31
    }
}
#[doc = "Field `CTSLOC` writer - I/O Location"]
pub type CtslocW<'a, REG> = crate::FieldWriter<'a, REG, 6, CTSLOC>;
impl<'a, REG> CtslocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(CTSLOC::Loc31)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTSLOC {
    #[doc = "0: Location 0"]
    Loc0 = 0,
    #[doc = "1: Location 1"]
    Loc1 = 1,
    #[doc = "2: Location 2"]
    Loc2 = 2,
    #[doc = "3: Location 3"]
    Loc3 = 3,
    #[doc = "4: Location 4"]
    Loc4 = 4,
    #[doc = "5: Location 5"]
    Loc5 = 5,
    #[doc = "6: Location 6"]
    Loc6 = 6,
    #[doc = "7: Location 7"]
    Loc7 = 7,
    #[doc = "8: Location 8"]
    Loc8 = 8,
    #[doc = "9: Location 9"]
    Loc9 = 9,
    #[doc = "10: Location 10"]
    Loc10 = 10,
    #[doc = "11: Location 11"]
    Loc11 = 11,
    #[doc = "12: Location 12"]
    Loc12 = 12,
    #[doc = "13: Location 13"]
    Loc13 = 13,
    #[doc = "14: Location 14"]
    Loc14 = 14,
    #[doc = "15: Location 15"]
    Loc15 = 15,
    #[doc = "16: Location 16"]
    Loc16 = 16,
    #[doc = "17: Location 17"]
    Loc17 = 17,
    #[doc = "18: Location 18"]
    Loc18 = 18,
    #[doc = "19: Location 19"]
    Loc19 = 19,
    #[doc = "20: Location 20"]
    Loc20 = 20,
    #[doc = "21: Location 21"]
    Loc21 = 21,
    #[doc = "22: Location 22"]
    Loc22 = 22,
    #[doc = "23: Location 23"]
    Loc23 = 23,
    #[doc = "24: Location 24"]
    Loc24 = 24,
    #[doc = "25: Location 25"]
    Loc25 = 25,
    #[doc = "26: Location 26"]
    Loc26 = 26,
    #[doc = "27: Location 27"]
    Loc27 = 27,
    #[doc = "28: Location 28"]
    Loc28 = 28,
    #[doc = "29: Location 29"]
    Loc29 = 29,
    #[doc = "30: Location 30"]
    Loc30 = 30,
    #[doc = "31: Location 31"]
    Loc31 = 31,
}
impl From<RTSLOC> for u8 {
    #[inline(always)]
    fn from(variant: RTSLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTSLOC {
    type Ux = u8;
}
impl crate::IsEnum for RTSLOC {}
#[doc = "Field `RTSLOC` reader - I/O Location"]
pub type RtslocR = crate::FieldReader<RTSLOC>;
impl RtslocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RTSLOC> {
        match self.bits {
            0 => Some(RTSLOC::Loc0),
            1 => Some(RTSLOC::Loc1),
            2 => Some(RTSLOC::Loc2),
            3 => Some(RTSLOC::Loc3),
            4 => Some(RTSLOC::Loc4),
            5 => Some(RTSLOC::Loc5),
            6 => Some(RTSLOC::Loc6),
            7 => Some(RTSLOC::Loc7),
            8 => Some(RTSLOC::Loc8),
            9 => Some(RTSLOC::Loc9),
            10 => Some(RTSLOC::Loc10),
            11 => Some(RTSLOC::Loc11),
            12 => Some(RTSLOC::Loc12),
            13 => Some(RTSLOC::Loc13),
            14 => Some(RTSLOC::Loc14),
            15 => Some(RTSLOC::Loc15),
            16 => Some(RTSLOC::Loc16),
            17 => Some(RTSLOC::Loc17),
            18 => Some(RTSLOC::Loc18),
            19 => Some(RTSLOC::Loc19),
            20 => Some(RTSLOC::Loc20),
            21 => Some(RTSLOC::Loc21),
            22 => Some(RTSLOC::Loc22),
            23 => Some(RTSLOC::Loc23),
            24 => Some(RTSLOC::Loc24),
            25 => Some(RTSLOC::Loc25),
            26 => Some(RTSLOC::Loc26),
            27 => Some(RTSLOC::Loc27),
            28 => Some(RTSLOC::Loc28),
            29 => Some(RTSLOC::Loc29),
            30 => Some(RTSLOC::Loc30),
            31 => Some(RTSLOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RTSLOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RTSLOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == RTSLOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == RTSLOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == RTSLOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == RTSLOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == RTSLOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == RTSLOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == RTSLOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == RTSLOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == RTSLOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == RTSLOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == RTSLOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == RTSLOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == RTSLOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == RTSLOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == RTSLOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == RTSLOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == RTSLOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == RTSLOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == RTSLOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == RTSLOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == RTSLOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == RTSLOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == RTSLOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == RTSLOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == RTSLOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == RTSLOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == RTSLOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == RTSLOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == RTSLOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == RTSLOC::Loc31
    }
}
#[doc = "Field `RTSLOC` writer - I/O Location"]
pub type RtslocW<'a, REG> = crate::FieldWriter<'a, REG, 6, RTSLOC>;
impl<'a, REG> RtslocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(RTSLOC::Loc31)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ctsloc(&self) -> CtslocR {
        CtslocR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn rtsloc(&self) -> RtslocR {
        RtslocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTELOC1")
            .field("ctsloc", &self.ctsloc())
            .field("rtsloc", &self.rtsloc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn ctsloc(&mut self) -> CtslocW<ROUTELOC1rs> {
        CtslocW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn rtsloc(&mut self) -> RtslocW<ROUTELOC1rs> {
        RtslocW::new(self, 8)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`routeloc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC1rs;
impl crate::RegisterSpec for ROUTELOC1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc1::R`](R) reader structure"]
impl crate::Readable for ROUTELOC1rs {}
#[doc = "`write(|w| ..)` method takes [`routeloc1::W`](W) writer structure"]
impl crate::Writable for ROUTELOC1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC1 to value 0"]
impl crate::Resettable for ROUTELOC1rs {
    const RESET_VALUE: u32 = 0;
}
