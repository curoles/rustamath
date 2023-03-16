
Here is an example of Naive vs Horner method polynomial
function calculation performance:

![Naive vs Horner](criterion/Polynomial/report/lines.svg)

## Naive can beat Horner

On some machines Naive beats Horner despite of common wisdom.

```text
processor	: 63
vendor_id	: AuthenticAMD
cpu family	: 23
model		: 49
model name	: AMD Ryzen Threadripper 3970X 32-Core Processor
stepping	: 0
microcode	: 0x8301025
cpu MHz		: 2194.784
cache size	: 512 KB
physical id	: 0
siblings	: 64
core id		: 31
cpu cores	: 32
apicid		: 63
initial apicid	: 63
fpu		: yes
fpu_exception	: yes
cpuid level	: 16
wp		: yes
flags		: fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush mmx fxsr sse sse2 ht syscall nx mmxext fxsr_opt pdpe1gb rdtscp lm constant_tsc rep_good nopl nonstop_tsc cpuid extd_apicid aperfmperf pni pclmulqdq monitor ssse3 fma cx16 sse4_1 sse4_2 movbe popcnt aes xsave avx f16c rdrand lahf_lm cmp_legacy svm extapic cr8_legacy abm sse4a misalignsse 3dnowprefetch osvw ibs skinit wdt tce topoext perfctr_core perfctr_nb bpext perfctr_llc mwaitx cpb cat_l3 cdp_l3 hw_pstate sme ssbd mba sev ibpb stibp vmmcall fsgsbase bmi1 avx2 smep bmi2 cqm rdt_a rdseed adx smap clflushopt clwb sha_ni xsaveopt xsavec xgetbv1 xsaves cqm_llc cqm_occup_llc cqm_mbm_total cqm_mbm_local clzero irperf xsaveerptr wbnoinvd arat npt lbrv svm_lock nrip_save tsc_scale vmcb_clean flushbyasid decodeassists pausefilter pfthreshold avic v_vmsave_vmload vgif umip rdpid overflow_recov succor smca
bugs		: sysret_ss_attrs spectre_v1 spectre_v2 spec_store_bypass retbleed
bogomips	: 7400.28
TLB size	: 3072 4K pages
clflush size	: 64
cache_alignment	: 64
address sizes	: 43 bits physical, 48 bits virtual
power management: ts ttp tm hwpstate cpb eff_freq_ro [13] [14]
```

```text
Percent│
       │
       │    000000000005bca0 <rustamath::polynomial::polynomial_n>:
       │    _ZN9rustamath10polynomial12polynomial_n17h70ce18f09b1235e8E():
       │      push    %rax
       │      mov     %rsi,%rax
       │      sub     $0x1,%rax
       │    ↓ jb      39
       │      vmovsd  (%rdi,%rax,8),%xmm1
       │    ↓ je      33
  0.45 │      data16  data16 data16 data16 data16 nopw %cs:0x0(%rax,%rax,1)
       │20:   vmulsd  %xmm0,%xmm1,%xmm1
 79.98 │      vaddsd  -0x10(%rdi,%rsi,8),%xmm1,%xmm1
 16.26 │      dec     %rsi
  3.24 │      cmp     $0x1,%rsi
       │    ↑ jne     20
       │33:   vmovapd %xmm1,%xmm0
  0.06 │      pop     %rax
       │    ← retq
       │39:   lea     anon.85ee5d3783bfcaf8bb1cd6cd80eceb86.35.llvm.17066916603741537869+0x48,%rdx
       │      mov     %rax,%rdi
       │      xor     %esi,%esi
       │    → callq   *0x265685(%rip)        # 2c1370 <_GLOBAL_OFFSET_TABLE_+0x1170>
       │      ud2
       │      nop
```

```text
Percent│
       │
       │    000000000005bc30 <rustamath::polynomial::naive_polynomial_n>:
       │    _ZN9rustamath10polynomial18naive_polynomial_n17h0b92334ddab08b16E():
       │      push    %rax
       │      test    %rsi,%rsi
       │    ↓ je      59
       │      vmovsd  (%rdi),%xmm1
       │      vmovsd  anon.4af8eb02b571123d11491cbcb3ba5f78.22.llvm.6019450797387638298+0x178,%xmm2
       │      lea     (%rdi,%rsi,8),%rax
       │      mov     $0x1,%ecx
  0.61 │      xor     %edx,%edx
       │    ↓ jmp     39
       │      nop
       │20:   cmp     %rax,%rdi
       │    ↓ je      53
       │25:   vmulsd  %xmm0,%xmm2,%xmm2
       │      vmulsd  (%rdi),%xmm2,%xmm3
 63.23 │      add     $0x8,%rdi
       │      mov     $0x1,%dl
       │      xor     %ecx,%ecx
       │      vaddsd  %xmm3,%xmm1,%xmm1
 27.56 │39:   test    $0x1,%dl
  7.86 │    ↑ jne     20
       │      mov     %rax,%rdx
       │      sub     %rdi,%rdx
       │      shr     $0x3,%rdx
       │      cmp     %rcx,%rdx
       │    ↓ jbe     53
       │      lea     (%rdi,%rcx,8),%rdi
  0.62 │    ↑ jmp     25
       │53:   vmovapd %xmm1,%xmm0
  0.12 │      pop     %rax
       │    ← retq
       │59:   lea     anon.85ee5d3783bfcaf8bb1cd6cd80eceb86.35.llvm.17066916603741537869+0x30,%rdx
       │      xor     %edi,%edi
       │      xor     %esi,%esi
       │    → callq   *0x2656d6(%rip)        # 2c1370 <_GLOBAL_OFFSET_TABLE_+0x1170>
       │      ud2
       │      nop
```

Horner:

1. `vmulsd  %xmm0,%xmm1,%xmm1               // x*res -> res`
2. `vaddsd  -0x10(%rdi,%rsi,8),%xmm1,%xmm1  // c + res -> res`

Naive:

1. `vmulsd  %xmm0,%xmm2,%xmm2   // x*xn -> xn`
2. `vmulsd  (%rdi),%xmm2,%xmm3  // c*xn -> p`
3. `vaddsd  %xmm3,%xmm1,%xmm1   // p + res -> res`

In Naive case, we can run 3 SIMD instructions in parallel (are we ?).
In Horner's method we must run 2 instructions sequentially.

1. `x*xn(t-1) -> xn(t)`
2. `c*xn(t-1) -> p(t)`
3. `p(t-1) + res(t-1) -> res(t)`