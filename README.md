# test_cpuid
Running results on my test machine:

```
function: 0x0, index: 0, flags: 0
kvm support cpuid: eax: d, ebx: 756e6547, ecx: 6c65746e, edx: 49656e69
       host cpuid: eax: 16, ebx: 756e6547, ecx: 6c65746e, edx: 49656e69

function: 0x1, index: 0, flags: 0
kvm support cpuid: eax: 50654, ebx: 69400800, ecx: 76fa3203, edx: f8bfbff
       host cpuid: eax: 50654, ebx: 69400800, ecx: 7ffefbff, edx: bfebfbff

function: 0x2, index: 0, flags: 6
kvm support cpuid: eax: 76036301, ebx: f0b5ff, ecx: 0, edx: c30000
       host cpuid: eax: 76036301, ebx: f0b5ff, ecx: 0, edx: c30000

function: 0x3, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0

function: 0x4, index: 0, flags: 1
kvm support cpuid: eax: 7c004121, ebx: 1c0003f, ecx: 3f, edx: 0
       host cpuid: eax: 7c004121, ebx: 1c0003f, ecx: 3f, edx: 0

function: 0x4, index: 1, flags: 1
kvm support cpuid: eax: 7c004122, ebx: 1c0003f, ecx: 3f, edx: 0
       host cpuid: eax: 7c004122, ebx: 1c0003f, ecx: 3f, edx: 0

function: 0x4, index: 2, flags: 1
kvm support cpuid: eax: 7c004143, ebx: 3c0003f, ecx: 3ff, edx: 0
       host cpuid: eax: 7c004143, ebx: 3c0003f, ecx: 3ff, edx: 0

function: 0x4, index: 3, flags: 1
kvm support cpuid: eax: 7c0fc163, ebx: 280003f, ecx: bfff, edx: 4
       host cpuid: eax: 7c0fc163, ebx: 280003f, ecx: bfff, edx: 4

function: 0x4, index: 4, flags: 1
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0

function: 0x5, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 40, ebx: 40, ecx: 3, edx: 2020

function: 0x6, index: 0, flags: 0
kvm support cpuid: eax: 4, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 77, ebx: 2, ecx: 9, edx: 0

function: 0x7, index: 0, flags: 1
kvm support cpuid: eax: 0, ebx: d19f4fba, ecx: c, edx: ac000400
       host cpuid: eax: 0, ebx: d39ffffb, ecx: 18, edx: 9c002400

function: 0x8, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0

function: 0x9, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0

function: 0xa, index: 0, flags: 0
kvm support cpuid: eax: 7300402, ebx: 0, ecx: 0, edx: 8603
       host cpuid: eax: 7300404, ebx: 0, ecx: 0, edx: 603

function: 0xb, index: 0, flags: 1
kvm support cpuid: eax: 1, ebx: 2, ecx: 100, edx: 69
       host cpuid: eax: 1, ebx: 2, ecx: 100, edx: 69

function: 0xb, index: 1, flags: 1
kvm support cpuid: eax: 6, ebx: 30, ecx: 201, edx: 69
       host cpuid: eax: 6, ebx: 30, ecx: 201, edx: 69

function: 0xb, index: 2, flags: 1
kvm support cpuid: eax: 0, ebx: 0, ecx: 2, edx: 69
       host cpuid: eax: 0, ebx: 0, ecx: 2, edx: 69

function: 0xc, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0

function: 0xd, index: 0, flags: 1
kvm support cpuid: eax: 2ff, ebx: a88, ecx: a88, edx: 0
       host cpuid: eax: 2ff, ebx: a88, ecx: a88, edx: 0

function: 0xd, index: 1, flags: 1
kvm support cpuid: eax: f, ebx: a08, ecx: 0, edx: 0
       host cpuid: eax: f, ebx: a08, ecx: 100, edx: 0

function: 0xd, index: 2, flags: 1
kvm support cpuid: eax: 100, ebx: 240, ecx: 0, edx: 0
       host cpuid: eax: 100, ebx: 240, ecx: 0, edx: 0

function: 0xd, index: 3, flags: 1
kvm support cpuid: eax: 40, ebx: 3c0, ecx: 0, edx: 0
       host cpuid: eax: 40, ebx: 3c0, ecx: 0, edx: 0

function: 0xd, index: 4, flags: 1
kvm support cpuid: eax: 40, ebx: 400, ecx: 0, edx: 0
       host cpuid: eax: 40, ebx: 400, ecx: 0, edx: 0

function: 0xd, index: 5, flags: 1
kvm support cpuid: eax: 40, ebx: 440, ecx: 0, edx: 0
       host cpuid: eax: 40, ebx: 440, ecx: 0, edx: 0

function: 0xd, index: 6, flags: 1
kvm support cpuid: eax: 200, ebx: 480, ecx: 0, edx: 0
       host cpuid: eax: 200, ebx: 480, ecx: 0, edx: 0

function: 0xd, index: 7, flags: 1
kvm support cpuid: eax: 400, ebx: 680, ecx: 0, edx: 0
       host cpuid: eax: 400, ebx: 680, ecx: 0, edx: 0

function: 0xd, index: 9, flags: 1
kvm support cpuid: eax: 8, ebx: a80, ecx: 0, edx: 0
       host cpuid: eax: 8, ebx: a80, ecx: 0, edx: 0

function: 0x80000000, index: 0, flags: 0
kvm support cpuid: eax: 80000008, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 80000008, ebx: 0, ecx: 0, edx: 0

function: 0x80000001, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 121, edx: 2c100800
       host cpuid: eax: 0, ebx: 0, ecx: 121, edx: 2c100800

function: 0x80000002, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 65746e49, ebx: 2952286c, ecx: 6f655820, edx: 2952286e

function: 0x80000003, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 616c5020, ebx: 756e6974, ecx: 3138206d, edx: 43203336

function: 0x80000004, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 40205550, ebx: 352e3220, ecx: 7a484730, edx: 0

function: 0x80000005, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0

function: 0x80000006, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 0, ebx: 0, ecx: 1006040, edx: 0

function: 0x80000007, index: 0, flags: 0
kvm support cpuid: eax: 0, ebx: 0, ecx: 0, edx: 100
       host cpuid: eax: 0, ebx: 0, ecx: 0, edx: 100

function: 0x80000008, index: 0, flags: 0
kvm support cpuid: eax: 302e, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 302e, ebx: 0, ecx: 0, edx: 0

function: 0x40000000, index: 0, flags: 0
kvm support cpuid: eax: 40000001, ebx: 4b4d564b, ecx: 564b4d56, edx: 4d
       host cpuid: eax: 9c4, ebx: c1c, ecx: 64, edx: 0

function: 0x40000001, index: 0, flags: 0
kvm support cpuid: eax: 1001efb, ebx: 0, ecx: 0, edx: 0
       host cpuid: eax: 9c4, ebx: c1c, ecx: 64, edx: 0
```

```
$ uname -a
Linux xxxxx 4.19.<...>.x86_64 #1 SMP Thu Jan 6 15:04:24 CST 2022 x86_64 x86_64 x86_64 GNU/Linux

$ modinfo kvm
filename:       /lib/modules/4.19.<...>.x86_64/kernel/arch/x86/kvm/kvm.ko
license:        GPL
author:         Qumranet
srcversion:     <...>
depends:        irqbypass
intree:         Y
name:           kvm
vermagic:       4.19.<...>.x86_64 SMP mod_unload modversions 
parm:           nx_huge_pages:bool
parm:           nx_huge_pages_recovery_ratio:uint
parm:           ignore_msrs:bool
parm:           report_ignored_msrs:bool
parm:           min_timer_period_us:uint
parm:           kvmclock_periodic_sync:bool
parm:           tsc_tolerance_ppm:uint
parm:           lapic_timer_advance_ns:uint
parm:           vector_hashing:bool
parm:           enable_vmware_backdoor:bool
parm:           force_emulation_prefix:bool
parm:           halt_poll_ns:uint
parm:           halt_poll_ns_grow:uint
parm:           halt_poll_ns_shrink:uint
```
