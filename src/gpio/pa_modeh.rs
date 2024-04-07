#[doc = "Register `PA_MODEH` reader"]
pub type R = crate::R<PA_MODEHrs>;
#[doc = "Register `PA_MODEH` writer"]
pub type W = crate::W<PA_MODEHrs>;
#[doc = "Pin 8 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE8 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE8> for u8 {
    #[inline(always)]
    fn from(variant: MODE8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE8 {
    type Ux = u8;
}
impl crate::IsEnum for MODE8 {}
#[doc = "Field `MODE8` reader - Pin 8 Mode"]
pub type Mode8R = crate::FieldReader<MODE8>;
impl Mode8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE8 {
        match self.bits {
            0 => MODE8::Disabled,
            1 => MODE8::Input,
            2 => MODE8::Inputpull,
            3 => MODE8::Inputpullfilter,
            4 => MODE8::Pushpull,
            5 => MODE8::Pushpullalt,
            6 => MODE8::Wiredor,
            7 => MODE8::Wiredorpulldown,
            8 => MODE8::Wiredand,
            9 => MODE8::Wiredandfilter,
            10 => MODE8::Wiredandpullup,
            11 => MODE8::Wiredandpullupfilter,
            12 => MODE8::Wiredandalt,
            13 => MODE8::Wiredandaltfilter,
            14 => MODE8::Wiredandaltpullup,
            15 => MODE8::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE8::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE8::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE8::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE8::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE8::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE8::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE8::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE8::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE8::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE8::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE8::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE8::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE8::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE8::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE8::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE8::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE8` writer - Pin 8 Mode"]
pub type Mode8W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE8, crate::Safe>;
impl<'a, REG> Mode8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE8::Wiredandaltpullupfilter)
    }
}
#[doc = "Pin 9 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE9 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE9> for u8 {
    #[inline(always)]
    fn from(variant: MODE9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE9 {
    type Ux = u8;
}
impl crate::IsEnum for MODE9 {}
#[doc = "Field `MODE9` reader - Pin 9 Mode"]
pub type Mode9R = crate::FieldReader<MODE9>;
impl Mode9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE9 {
        match self.bits {
            0 => MODE9::Disabled,
            1 => MODE9::Input,
            2 => MODE9::Inputpull,
            3 => MODE9::Inputpullfilter,
            4 => MODE9::Pushpull,
            5 => MODE9::Pushpullalt,
            6 => MODE9::Wiredor,
            7 => MODE9::Wiredorpulldown,
            8 => MODE9::Wiredand,
            9 => MODE9::Wiredandfilter,
            10 => MODE9::Wiredandpullup,
            11 => MODE9::Wiredandpullupfilter,
            12 => MODE9::Wiredandalt,
            13 => MODE9::Wiredandaltfilter,
            14 => MODE9::Wiredandaltpullup,
            15 => MODE9::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE9::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE9::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE9::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE9::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE9::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE9::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE9::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE9::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE9::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE9::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE9::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE9::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE9::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE9::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE9::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE9::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE9` writer - Pin 9 Mode"]
pub type Mode9W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE9, crate::Safe>;
impl<'a, REG> Mode9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE9::Wiredandaltpullupfilter)
    }
}
#[doc = "Pin 10 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE10 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE10> for u8 {
    #[inline(always)]
    fn from(variant: MODE10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE10 {
    type Ux = u8;
}
impl crate::IsEnum for MODE10 {}
#[doc = "Field `MODE10` reader - Pin 10 Mode"]
pub type Mode10R = crate::FieldReader<MODE10>;
impl Mode10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE10 {
        match self.bits {
            0 => MODE10::Disabled,
            1 => MODE10::Input,
            2 => MODE10::Inputpull,
            3 => MODE10::Inputpullfilter,
            4 => MODE10::Pushpull,
            5 => MODE10::Pushpullalt,
            6 => MODE10::Wiredor,
            7 => MODE10::Wiredorpulldown,
            8 => MODE10::Wiredand,
            9 => MODE10::Wiredandfilter,
            10 => MODE10::Wiredandpullup,
            11 => MODE10::Wiredandpullupfilter,
            12 => MODE10::Wiredandalt,
            13 => MODE10::Wiredandaltfilter,
            14 => MODE10::Wiredandaltpullup,
            15 => MODE10::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE10::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE10::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE10::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE10::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE10::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE10::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE10::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE10::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE10::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE10::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE10::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE10::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE10::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE10::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE10::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE10::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE10` writer - Pin 10 Mode"]
pub type Mode10W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE10, crate::Safe>;
impl<'a, REG> Mode10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE10::Wiredandaltpullupfilter)
    }
}
#[doc = "Pin 11 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE11 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE11> for u8 {
    #[inline(always)]
    fn from(variant: MODE11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE11 {
    type Ux = u8;
}
impl crate::IsEnum for MODE11 {}
#[doc = "Field `MODE11` reader - Pin 11 Mode"]
pub type Mode11R = crate::FieldReader<MODE11>;
impl Mode11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE11 {
        match self.bits {
            0 => MODE11::Disabled,
            1 => MODE11::Input,
            2 => MODE11::Inputpull,
            3 => MODE11::Inputpullfilter,
            4 => MODE11::Pushpull,
            5 => MODE11::Pushpullalt,
            6 => MODE11::Wiredor,
            7 => MODE11::Wiredorpulldown,
            8 => MODE11::Wiredand,
            9 => MODE11::Wiredandfilter,
            10 => MODE11::Wiredandpullup,
            11 => MODE11::Wiredandpullupfilter,
            12 => MODE11::Wiredandalt,
            13 => MODE11::Wiredandaltfilter,
            14 => MODE11::Wiredandaltpullup,
            15 => MODE11::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE11::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE11::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE11::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE11::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE11::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE11::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE11::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE11::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE11::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE11::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE11::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE11::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE11::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE11::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE11::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE11::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE11` writer - Pin 11 Mode"]
pub type Mode11W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE11, crate::Safe>;
impl<'a, REG> Mode11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE11::Wiredandaltpullupfilter)
    }
}
#[doc = "Pin 12 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE12 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE12> for u8 {
    #[inline(always)]
    fn from(variant: MODE12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE12 {
    type Ux = u8;
}
impl crate::IsEnum for MODE12 {}
#[doc = "Field `MODE12` reader - Pin 12 Mode"]
pub type Mode12R = crate::FieldReader<MODE12>;
impl Mode12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE12 {
        match self.bits {
            0 => MODE12::Disabled,
            1 => MODE12::Input,
            2 => MODE12::Inputpull,
            3 => MODE12::Inputpullfilter,
            4 => MODE12::Pushpull,
            5 => MODE12::Pushpullalt,
            6 => MODE12::Wiredor,
            7 => MODE12::Wiredorpulldown,
            8 => MODE12::Wiredand,
            9 => MODE12::Wiredandfilter,
            10 => MODE12::Wiredandpullup,
            11 => MODE12::Wiredandpullupfilter,
            12 => MODE12::Wiredandalt,
            13 => MODE12::Wiredandaltfilter,
            14 => MODE12::Wiredandaltpullup,
            15 => MODE12::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE12::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE12::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE12::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE12::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE12::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE12::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE12::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE12::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE12::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE12::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE12::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE12::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE12::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE12::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE12::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE12::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE12` writer - Pin 12 Mode"]
pub type Mode12W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE12, crate::Safe>;
impl<'a, REG> Mode12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE12::Wiredandaltpullupfilter)
    }
}
#[doc = "Pin 13 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE13 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE13> for u8 {
    #[inline(always)]
    fn from(variant: MODE13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE13 {
    type Ux = u8;
}
impl crate::IsEnum for MODE13 {}
#[doc = "Field `MODE13` reader - Pin 13 Mode"]
pub type Mode13R = crate::FieldReader<MODE13>;
impl Mode13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE13 {
        match self.bits {
            0 => MODE13::Disabled,
            1 => MODE13::Input,
            2 => MODE13::Inputpull,
            3 => MODE13::Inputpullfilter,
            4 => MODE13::Pushpull,
            5 => MODE13::Pushpullalt,
            6 => MODE13::Wiredor,
            7 => MODE13::Wiredorpulldown,
            8 => MODE13::Wiredand,
            9 => MODE13::Wiredandfilter,
            10 => MODE13::Wiredandpullup,
            11 => MODE13::Wiredandpullupfilter,
            12 => MODE13::Wiredandalt,
            13 => MODE13::Wiredandaltfilter,
            14 => MODE13::Wiredandaltpullup,
            15 => MODE13::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE13::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE13::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE13::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE13::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE13::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE13::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE13::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE13::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE13::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE13::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE13::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE13::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE13::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE13::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE13::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE13::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE13` writer - Pin 13 Mode"]
pub type Mode13W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE13, crate::Safe>;
impl<'a, REG> Mode13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE13::Wiredandaltpullupfilter)
    }
}
#[doc = "Pin 14 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE14 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE14> for u8 {
    #[inline(always)]
    fn from(variant: MODE14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE14 {
    type Ux = u8;
}
impl crate::IsEnum for MODE14 {}
#[doc = "Field `MODE14` reader - Pin 14 Mode"]
pub type Mode14R = crate::FieldReader<MODE14>;
impl Mode14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE14 {
        match self.bits {
            0 => MODE14::Disabled,
            1 => MODE14::Input,
            2 => MODE14::Inputpull,
            3 => MODE14::Inputpullfilter,
            4 => MODE14::Pushpull,
            5 => MODE14::Pushpullalt,
            6 => MODE14::Wiredor,
            7 => MODE14::Wiredorpulldown,
            8 => MODE14::Wiredand,
            9 => MODE14::Wiredandfilter,
            10 => MODE14::Wiredandpullup,
            11 => MODE14::Wiredandpullupfilter,
            12 => MODE14::Wiredandalt,
            13 => MODE14::Wiredandaltfilter,
            14 => MODE14::Wiredandaltpullup,
            15 => MODE14::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE14::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE14::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE14::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE14::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE14::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE14::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE14::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE14::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE14::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE14::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE14::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE14::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE14::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE14::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE14::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE14::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE14` writer - Pin 14 Mode"]
pub type Mode14W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE14, crate::Safe>;
impl<'a, REG> Mode14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE14::Wiredandaltpullupfilter)
    }
}
#[doc = "Pin 15 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE15 {
    #[doc = "0: Input disabled. Pullup if DOUT is set."]
    Disabled = 0,
    #[doc = "1: Input enabled. Filter if DOUT is set"]
    Input = 1,
    #[doc = "2: Input enabled. DOUT determines pull direction"]
    Inputpull = 2,
    #[doc = "3: Input enabled with filter. DOUT determines pull direction"]
    Inputpullfilter = 3,
    #[doc = "4: Push-pull output"]
    Pushpull = 4,
    #[doc = "5: Push-pull using alternate control"]
    Pushpullalt = 5,
    #[doc = "6: Wired-or output"]
    Wiredor = 6,
    #[doc = "7: Wired-or output with pull-down"]
    Wiredorpulldown = 7,
    #[doc = "8: Open-drain output"]
    Wiredand = 8,
    #[doc = "9: Open-drain output with filter"]
    Wiredandfilter = 9,
    #[doc = "10: Open-drain output with pullup"]
    Wiredandpullup = 10,
    #[doc = "11: Open-drain output with filter and pullup"]
    Wiredandpullupfilter = 11,
    #[doc = "12: Open-drain output using alternate control"]
    Wiredandalt = 12,
    #[doc = "13: Open-drain output using alternate control with filter"]
    Wiredandaltfilter = 13,
    #[doc = "14: Open-drain output using alternate control with pullup"]
    Wiredandaltpullup = 14,
    #[doc = "15: Open-drain output using alternate control with filter and pullup"]
    Wiredandaltpullupfilter = 15,
}
impl From<MODE15> for u8 {
    #[inline(always)]
    fn from(variant: MODE15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE15 {
    type Ux = u8;
}
impl crate::IsEnum for MODE15 {}
#[doc = "Field `MODE15` reader - Pin 15 Mode"]
pub type Mode15R = crate::FieldReader<MODE15>;
impl Mode15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE15 {
        match self.bits {
            0 => MODE15::Disabled,
            1 => MODE15::Input,
            2 => MODE15::Inputpull,
            3 => MODE15::Inputpullfilter,
            4 => MODE15::Pushpull,
            5 => MODE15::Pushpullalt,
            6 => MODE15::Wiredor,
            7 => MODE15::Wiredorpulldown,
            8 => MODE15::Wiredand,
            9 => MODE15::Wiredandfilter,
            10 => MODE15::Wiredandpullup,
            11 => MODE15::Wiredandpullupfilter,
            12 => MODE15::Wiredandalt,
            13 => MODE15::Wiredandaltfilter,
            14 => MODE15::Wiredandaltpullup,
            15 => MODE15::Wiredandaltpullupfilter,
            _ => unreachable!(),
        }
    }
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE15::Disabled
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == MODE15::Input
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpull(&self) -> bool {
        *self == MODE15::Inputpull
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn is_inputpullfilter(&self) -> bool {
        *self == MODE15::Inputpullfilter
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn is_pushpull(&self) -> bool {
        *self == MODE15::Pushpull
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn is_pushpullalt(&self) -> bool {
        *self == MODE15::Pushpullalt
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn is_wiredor(&self) -> bool {
        *self == MODE15::Wiredor
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn is_wiredorpulldown(&self) -> bool {
        *self == MODE15::Wiredorpulldown
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn is_wiredand(&self) -> bool {
        *self == MODE15::Wiredand
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn is_wiredandfilter(&self) -> bool {
        *self == MODE15::Wiredandfilter
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn is_wiredandpullup(&self) -> bool {
        *self == MODE15::Wiredandpullup
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandpullupfilter(&self) -> bool {
        *self == MODE15::Wiredandpullupfilter
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn is_wiredandalt(&self) -> bool {
        *self == MODE15::Wiredandalt
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn is_wiredandaltfilter(&self) -> bool {
        *self == MODE15::Wiredandaltfilter
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullup(&self) -> bool {
        *self == MODE15::Wiredandaltpullup
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn is_wiredandaltpullupfilter(&self) -> bool {
        *self == MODE15::Wiredandaltpullupfilter
    }
}
#[doc = "Field `MODE15` writer - Pin 15 Mode"]
pub type Mode15W<'a, REG> = crate::FieldWriter<'a, REG, 4, MODE15, crate::Safe>;
impl<'a, REG> Mode15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Input disabled. Pullup if DOUT is set."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Disabled)
    }
    #[doc = "Input enabled. Filter if DOUT is set"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Input)
    }
    #[doc = "Input enabled. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Inputpull)
    }
    #[doc = "Input enabled with filter. DOUT determines pull direction"]
    #[inline(always)]
    pub fn inputpullfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Inputpullfilter)
    }
    #[doc = "Push-pull output"]
    #[inline(always)]
    pub fn pushpull(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Pushpull)
    }
    #[doc = "Push-pull using alternate control"]
    #[inline(always)]
    pub fn pushpullalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Pushpullalt)
    }
    #[doc = "Wired-or output"]
    #[inline(always)]
    pub fn wiredor(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredor)
    }
    #[doc = "Wired-or output with pull-down"]
    #[inline(always)]
    pub fn wiredorpulldown(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredorpulldown)
    }
    #[doc = "Open-drain output"]
    #[inline(always)]
    pub fn wiredand(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredand)
    }
    #[doc = "Open-drain output with filter"]
    #[inline(always)]
    pub fn wiredandfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredandfilter)
    }
    #[doc = "Open-drain output with pullup"]
    #[inline(always)]
    pub fn wiredandpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredandpullup)
    }
    #[doc = "Open-drain output with filter and pullup"]
    #[inline(always)]
    pub fn wiredandpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredandpullupfilter)
    }
    #[doc = "Open-drain output using alternate control"]
    #[inline(always)]
    pub fn wiredandalt(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredandalt)
    }
    #[doc = "Open-drain output using alternate control with filter"]
    #[inline(always)]
    pub fn wiredandaltfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredandaltfilter)
    }
    #[doc = "Open-drain output using alternate control with pullup"]
    #[inline(always)]
    pub fn wiredandaltpullup(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredandaltpullup)
    }
    #[doc = "Open-drain output using alternate control with filter and pullup"]
    #[inline(always)]
    pub fn wiredandaltpullupfilter(self) -> &'a mut crate::W<REG> {
        self.variant(MODE15::Wiredandaltpullupfilter)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline(always)]
    pub fn mode8(&self) -> Mode8R {
        Mode8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline(always)]
    pub fn mode9(&self) -> Mode9R {
        Mode9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline(always)]
    pub fn mode10(&self) -> Mode10R {
        Mode10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline(always)]
    pub fn mode11(&self) -> Mode11R {
        Mode11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline(always)]
    pub fn mode12(&self) -> Mode12R {
        Mode12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline(always)]
    pub fn mode13(&self) -> Mode13R {
        Mode13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline(always)]
    pub fn mode14(&self) -> Mode14R {
        Mode14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline(always)]
    pub fn mode15(&self) -> Mode15R {
        Mode15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pin 8 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode8(&mut self) -> Mode8W<PA_MODEHrs> {
        Mode8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Pin 9 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> Mode9W<PA_MODEHrs> {
        Mode9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Pin 10 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode10(&mut self) -> Mode10W<PA_MODEHrs> {
        Mode10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Pin 11 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode11(&mut self) -> Mode11W<PA_MODEHrs> {
        Mode11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Pin 12 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode12(&mut self) -> Mode12W<PA_MODEHrs> {
        Mode12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Pin 13 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode13(&mut self) -> Mode13W<PA_MODEHrs> {
        Mode13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Pin 14 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode14(&mut self) -> Mode14W<PA_MODEHrs> {
        Mode14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Pin 15 Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode15(&mut self) -> Mode15W<PA_MODEHrs> {
        Mode15W::new(self, 28)
    }
}
#[doc = "Port Pin Mode High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_modeh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_modeh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA_MODEHrs;
impl crate::RegisterSpec for PA_MODEHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_modeh::R`](R) reader structure"]
impl crate::Readable for PA_MODEHrs {}
#[doc = "`write(|w| ..)` method takes [`pa_modeh::W`](W) writer structure"]
impl crate::Writable for PA_MODEHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA_MODEH to value 0"]
impl crate::Resettable for PA_MODEHrs {
    const RESET_VALUE: u32 = 0;
}
