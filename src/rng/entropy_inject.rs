#[doc = "Register `ENTROPY_INJECT` reader"]
pub type R = crate::R<EntropyInjectSpec>;
#[doc = "Register `ENTROPY_INJECT` writer"]
pub type W = crate::W<EntropyInjectSpec>;
#[doc = "Field `ENTROPY` reader - Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
pub type EntropyR = crate::FieldReader<u32>;
#[doc = "Field `ENTROPY` writer - Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
pub type EntropyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
    #[inline(always)]
    pub fn entropy(&self) -> EntropyR {
        EntropyR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENTROPY_INJECT")
            .field("entropy", &self.entropy())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Use this register to inject or restore entropy 32 bits at a time. Writing is blocking thus see to it to have analog clocks activated. Injection can be usefull to add contribution from an external source of entropy, like for example LSBs of an ADC or of a temperature sensor. Restore can be usefull to store N random numbers in central memory before going powerdown then restore this entropy to RNG IP after power-up. It is useless to inject or restore more than 1*(number of RNGs) 32b words consecutively. Recommendation is to inject 1*(number of RNGs) words, and possibly later (2*32 clock cycles of slowest analog clock) inject again 1*(number of RNGs) words. Then maximum capacity of restoration is reached: about 44 bits per RNG (not to be mistaken with maximum capacity of entropy accumulation which is about 100 bits per RNG). You can inject less than 32 bits words (let unused bits to 0). Injection cannot degrade overall performance due to the fact that some internal PRNGs are excluded on purpose from this external action."]
    #[inline(always)]
    pub fn entropy(&mut self) -> EntropyW<'_, EntropyInjectSpec> {
        EntropyW::new(self, 0)
    }
}
#[doc = "no description available\n\nYou can [`read`](crate::Reg::read) this register and get [`entropy_inject::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entropy_inject::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EntropyInjectSpec;
impl crate::RegisterSpec for EntropyInjectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entropy_inject::R`](R) reader structure"]
impl crate::Readable for EntropyInjectSpec {}
#[doc = "`write(|w| ..)` method takes [`entropy_inject::W`](W) writer structure"]
impl crate::Writable for EntropyInjectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENTROPY_INJECT to value 0"]
impl crate::Resettable for EntropyInjectSpec {}
