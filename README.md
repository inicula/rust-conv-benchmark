Relates to:
* [https://blog.m-ou.se/floats/#results](https://github.com/m-ou-se/blog/issues/8#issuecomment-2372047123)

Result:

![bench](https://github.com/user-attachments/assets/2f2fd0e2-c107-41c6-aefd-8be43e595fa7)

CPU info:
```
$ lscpu
Architecture:             x86_64
  CPU op-mode(s):         32-bit, 64-bit
  Address sizes:          39 bits physical, 48 bits virtual
  Byte Order:             Little Endian
CPU(s):                   4
  On-line CPU(s) list:    0-3
Vendor ID:                GenuineIntel
  Model name:             Intel(R) Core(TM) i5-4460  CPU @ 3.20GHz
    CPU family:           6
    Model:                60
    Thread(s) per core:   1
    Core(s) per socket:   4
    Socket(s):            1
    Stepping:             3
    CPU(s) scaling MHz:   100%
    CPU max MHz:          3400.0000
    CPU min MHz:          800.0000
    BogoMIPS:             6398.67
    Flags:                fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx
                          fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp lm constant_tsc arch_perfmon pebs bts re
                          p_good nopl xtopology nonstop_tsc cpuid aperfmperf pni pclmulqdq dtes64 monitor ds_cpl est tm
                          2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes
                           xsave avx f16c rdrand lahf_lm abm cpuid_fault invpcid_single pti ssbd ibrs ibpb stibp fsgsba
                          se tsc_adjust bmi1 avx2 smep bmi2 erms invpcid xsaveopt dtherm ida arat pln pts md_clear flus
                          h_l1d
Caches (sum of all):
  L1d:                    128 KiB (4 instances)
  L1i:                    128 KiB (4 instances)
  L2:                     1 MiB (4 instances)
  L3:                     6 MiB (1 instance)
NUMA:
  NUMA node(s):           1
  NUMA node0 CPU(s):      0-3
Vulnerabilities:
  Gather data sampling:   Not affected
  Itlb multihit:          KVM: Mitigation: VMX unsupported
  L1tf:                   Mitigation; PTE Inversion
  Mds:                    Mitigation; Clear CPU buffers; SMT disabled
  Meltdown:               Mitigation; PTI
  Mmio stale data:        Unknown: No mitigations
  Reg file data sampling: Not affected
  Retbleed:               Not affected
  Spec rstack overflow:   Not affected
  Spec store bypass:      Mitigation; Speculative Store Bypass disabled via prctl
  Spectre v1:             Mitigation; usercopy/swapgs barriers and __user pointer sanitization
  Spectre v2:             Mitigation; Retpolines; IBPB conditional; IBRS_FW; STIBP disabled; RSB filling; PBRSB-eIBRS N
                          ot affected; BHI Not affected
  Srbds:                  Mitigation; Microcode
  Tsx async abort:        Not affected
