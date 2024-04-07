#[doc = "Register `ROUTELOC0` reader"]
pub type R = crate::R<ROUTELOC0rs>;
#[doc = "Register `ROUTELOC0` writer"]
pub type W = crate::W<ROUTELOC0rs>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S0INLOC {
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
impl From<S0INLOC> for u8 {
    #[inline(always)]
    fn from(variant: S0INLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S0INLOC {
    type Ux = u8;
}
impl crate::IsEnum for S0INLOC {}
#[doc = "Field `S0INLOC` reader - I/O Location"]
pub type S0inlocR = crate::FieldReader<S0INLOC>;
impl S0inlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S0INLOC> {
        match self.bits {
            0 => Some(S0INLOC::Loc0),
            1 => Some(S0INLOC::Loc1),
            2 => Some(S0INLOC::Loc2),
            3 => Some(S0INLOC::Loc3),
            4 => Some(S0INLOC::Loc4),
            5 => Some(S0INLOC::Loc5),
            6 => Some(S0INLOC::Loc6),
            7 => Some(S0INLOC::Loc7),
            8 => Some(S0INLOC::Loc8),
            9 => Some(S0INLOC::Loc9),
            10 => Some(S0INLOC::Loc10),
            11 => Some(S0INLOC::Loc11),
            12 => Some(S0INLOC::Loc12),
            13 => Some(S0INLOC::Loc13),
            14 => Some(S0INLOC::Loc14),
            15 => Some(S0INLOC::Loc15),
            16 => Some(S0INLOC::Loc16),
            17 => Some(S0INLOC::Loc17),
            18 => Some(S0INLOC::Loc18),
            19 => Some(S0INLOC::Loc19),
            20 => Some(S0INLOC::Loc20),
            21 => Some(S0INLOC::Loc21),
            22 => Some(S0INLOC::Loc22),
            23 => Some(S0INLOC::Loc23),
            24 => Some(S0INLOC::Loc24),
            25 => Some(S0INLOC::Loc25),
            26 => Some(S0INLOC::Loc26),
            27 => Some(S0INLOC::Loc27),
            28 => Some(S0INLOC::Loc28),
            29 => Some(S0INLOC::Loc29),
            30 => Some(S0INLOC::Loc30),
            31 => Some(S0INLOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == S0INLOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == S0INLOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == S0INLOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == S0INLOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == S0INLOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == S0INLOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == S0INLOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == S0INLOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == S0INLOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == S0INLOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == S0INLOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == S0INLOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == S0INLOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == S0INLOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == S0INLOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == S0INLOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == S0INLOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == S0INLOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == S0INLOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == S0INLOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == S0INLOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == S0INLOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == S0INLOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == S0INLOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == S0INLOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == S0INLOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == S0INLOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == S0INLOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == S0INLOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == S0INLOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == S0INLOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == S0INLOC::Loc31
    }
}
#[doc = "Field `S0INLOC` writer - I/O Location"]
pub type S0inlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, S0INLOC>;
impl<'a, REG> S0inlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(S0INLOC::Loc31)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum S1INLOC {
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
impl From<S1INLOC> for u8 {
    #[inline(always)]
    fn from(variant: S1INLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for S1INLOC {
    type Ux = u8;
}
impl crate::IsEnum for S1INLOC {}
#[doc = "Field `S1INLOC` reader - I/O Location"]
pub type S1inlocR = crate::FieldReader<S1INLOC>;
impl S1inlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<S1INLOC> {
        match self.bits {
            0 => Some(S1INLOC::Loc0),
            1 => Some(S1INLOC::Loc1),
            2 => Some(S1INLOC::Loc2),
            3 => Some(S1INLOC::Loc3),
            4 => Some(S1INLOC::Loc4),
            5 => Some(S1INLOC::Loc5),
            6 => Some(S1INLOC::Loc6),
            7 => Some(S1INLOC::Loc7),
            8 => Some(S1INLOC::Loc8),
            9 => Some(S1INLOC::Loc9),
            10 => Some(S1INLOC::Loc10),
            11 => Some(S1INLOC::Loc11),
            12 => Some(S1INLOC::Loc12),
            13 => Some(S1INLOC::Loc13),
            14 => Some(S1INLOC::Loc14),
            15 => Some(S1INLOC::Loc15),
            16 => Some(S1INLOC::Loc16),
            17 => Some(S1INLOC::Loc17),
            18 => Some(S1INLOC::Loc18),
            19 => Some(S1INLOC::Loc19),
            20 => Some(S1INLOC::Loc20),
            21 => Some(S1INLOC::Loc21),
            22 => Some(S1INLOC::Loc22),
            23 => Some(S1INLOC::Loc23),
            24 => Some(S1INLOC::Loc24),
            25 => Some(S1INLOC::Loc25),
            26 => Some(S1INLOC::Loc26),
            27 => Some(S1INLOC::Loc27),
            28 => Some(S1INLOC::Loc28),
            29 => Some(S1INLOC::Loc29),
            30 => Some(S1INLOC::Loc30),
            31 => Some(S1INLOC::Loc31),
            _ => None,
        }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == S1INLOC::Loc0
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == S1INLOC::Loc1
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == S1INLOC::Loc2
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == S1INLOC::Loc3
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == S1INLOC::Loc4
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == S1INLOC::Loc5
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == S1INLOC::Loc6
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == S1INLOC::Loc7
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == S1INLOC::Loc8
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == S1INLOC::Loc9
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == S1INLOC::Loc10
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == S1INLOC::Loc11
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == S1INLOC::Loc12
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == S1INLOC::Loc13
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == S1INLOC::Loc14
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == S1INLOC::Loc15
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == S1INLOC::Loc16
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn is_loc17(&self) -> bool {
        *self == S1INLOC::Loc17
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn is_loc18(&self) -> bool {
        *self == S1INLOC::Loc18
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn is_loc19(&self) -> bool {
        *self == S1INLOC::Loc19
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn is_loc20(&self) -> bool {
        *self == S1INLOC::Loc20
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn is_loc21(&self) -> bool {
        *self == S1INLOC::Loc21
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn is_loc22(&self) -> bool {
        *self == S1INLOC::Loc22
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn is_loc23(&self) -> bool {
        *self == S1INLOC::Loc23
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn is_loc24(&self) -> bool {
        *self == S1INLOC::Loc24
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn is_loc25(&self) -> bool {
        *self == S1INLOC::Loc25
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn is_loc26(&self) -> bool {
        *self == S1INLOC::Loc26
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn is_loc27(&self) -> bool {
        *self == S1INLOC::Loc27
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn is_loc28(&self) -> bool {
        *self == S1INLOC::Loc28
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn is_loc29(&self) -> bool {
        *self == S1INLOC::Loc29
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn is_loc30(&self) -> bool {
        *self == S1INLOC::Loc30
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn is_loc31(&self) -> bool {
        *self == S1INLOC::Loc31
    }
}
#[doc = "Field `S1INLOC` writer - I/O Location"]
pub type S1inlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, S1INLOC>;
impl<'a, REG> S1inlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc7)
    }
    #[doc = "Location 8"]
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc8)
    }
    #[doc = "Location 9"]
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc9)
    }
    #[doc = "Location 10"]
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc10)
    }
    #[doc = "Location 11"]
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc11)
    }
    #[doc = "Location 12"]
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc12)
    }
    #[doc = "Location 13"]
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc13)
    }
    #[doc = "Location 14"]
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc14)
    }
    #[doc = "Location 15"]
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc15)
    }
    #[doc = "Location 16"]
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc16)
    }
    #[doc = "Location 17"]
    #[inline(always)]
    pub fn loc17(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc17)
    }
    #[doc = "Location 18"]
    #[inline(always)]
    pub fn loc18(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc18)
    }
    #[doc = "Location 19"]
    #[inline(always)]
    pub fn loc19(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc19)
    }
    #[doc = "Location 20"]
    #[inline(always)]
    pub fn loc20(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc20)
    }
    #[doc = "Location 21"]
    #[inline(always)]
    pub fn loc21(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc21)
    }
    #[doc = "Location 22"]
    #[inline(always)]
    pub fn loc22(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc22)
    }
    #[doc = "Location 23"]
    #[inline(always)]
    pub fn loc23(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc23)
    }
    #[doc = "Location 24"]
    #[inline(always)]
    pub fn loc24(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc24)
    }
    #[doc = "Location 25"]
    #[inline(always)]
    pub fn loc25(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc25)
    }
    #[doc = "Location 26"]
    #[inline(always)]
    pub fn loc26(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc26)
    }
    #[doc = "Location 27"]
    #[inline(always)]
    pub fn loc27(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc27)
    }
    #[doc = "Location 28"]
    #[inline(always)]
    pub fn loc28(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc28)
    }
    #[doc = "Location 29"]
    #[inline(always)]
    pub fn loc29(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc29)
    }
    #[doc = "Location 30"]
    #[inline(always)]
    pub fn loc30(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc30)
    }
    #[doc = "Location 31"]
    #[inline(always)]
    pub fn loc31(self) -> &'a mut crate::W<REG> {
        self.variant(S1INLOC::Loc31)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn s0inloc(&self) -> S0inlocR {
        S0inlocR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn s1inloc(&self) -> S1inlocR {
        S1inlocR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn s0inloc(&mut self) -> S0inlocW<ROUTELOC0rs> {
        S0inlocW::new(self, 0)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn s1inloc(&mut self) -> S1inlocW<ROUTELOC0rs> {
        S1inlocW::new(self, 8)
    }
}
#[doc = "I/O Routing Location Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`routeloc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROUTELOC0rs;
impl crate::RegisterSpec for ROUTELOC0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`routeloc0::R`](R) reader structure"]
impl crate::Readable for ROUTELOC0rs {}
#[doc = "`write(|w| ..)` method takes [`routeloc0::W`](W) writer structure"]
impl crate::Writable for ROUTELOC0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROUTELOC0 to value 0"]
impl crate::Resettable for ROUTELOC0rs {
    const RESET_VALUE: u32 = 0;
}
