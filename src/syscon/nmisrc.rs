#[doc = "Register `NMISRC` reader"]
pub type R = crate::R<NmisrcSpec>;
#[doc = "Register `NMISRC` writer"]
pub type W = crate::W<NmisrcSpec>;
#[doc = "Field `IRQCPU0` reader - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
pub type Irqcpu0R = crate::FieldReader;
#[doc = "Field `IRQCPU0` writer - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
pub type Irqcpu0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NMIENCPU0` reader - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
pub type Nmiencpu0R = crate::BitReader;
#[doc = "Field `NMIENCPU0` writer - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
pub type Nmiencpu0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub fn irqcpu0(&self) -> Irqcpu0R {
        Irqcpu0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub fn nmiencpu0(&self) -> Nmiencpu0R {
        Nmiencpu0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for the CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub fn irqcpu0(&mut self) -> Irqcpu0W<'_, NmisrcSpec> {
        Irqcpu0W::new(self, 0)
    }
    #[doc = "Bit 31 - Write a 1 to this bit to enable the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub fn nmiencpu0(&mut self) -> Nmiencpu0W<'_, NmisrcSpec> {
        Nmiencpu0W::new(self, 31)
    }
}
#[doc = "NMI Source Select\n\nYou can [`read`](crate::Reg::read) this register and get [`nmisrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmisrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmisrcSpec;
impl crate::RegisterSpec for NmisrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmisrc::R`](R) reader structure"]
impl crate::Readable for NmisrcSpec {}
#[doc = "`write(|w| ..)` method takes [`nmisrc::W`](W) writer structure"]
impl crate::Writable for NmisrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMISRC to value 0"]
impl crate::Resettable for NmisrcSpec {}
