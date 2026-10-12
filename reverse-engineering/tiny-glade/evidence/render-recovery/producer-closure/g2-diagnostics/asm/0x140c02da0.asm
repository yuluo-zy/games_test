==== 0x140c02da0  file_off=0xc021a0
  140c02da0  4889c8                   mov rax, rcx
  140c02da3  31c9                     xor ecx, ecx
  140c02da5  48870a                   xchg qword ptr [rdx], rcx
  140c02da8  4531c0                   xor r8d, r8d
  140c02dab  4c874208                 xchg qword ptr [rdx + 8], r8
  140c02daf  4531c9                   xor r9d, r9d
  140c02db2  4c874a10                 xchg qword ptr [rdx + 0x10], r9
  140c02db6  488908                   mov qword ptr [rax], rcx
  140c02db9  4c894008                 mov qword ptr [rax + 8], r8
  140c02dbd  4c894810                 mov qword ptr [rax + 0x10], r9
  140c02dc1  c3                       ret 
  140c02dc2  cc                       int3 
  140c02dc3  cc                       int3 
  140c02dc4  cc                       int3 
  140c02dc5  cc                       int3 
  140c02dc6  cc                       int3 
  140c02dc7  cc                       int3 
  140c02dc8  cc                       int3 
  140c02dc9  cc                       int3 
  140c02dca  cc                       int3 
  140c02dcb  cc                       int3 
  140c02dcc  cc                       int3 
  140c02dcd  cc                       int3 
  140c02dce  cc                       int3 
  140c02dcf  cc                       int3 
  140c02dd0  55                       push rbp
  140c02dd1  4883ec40                 sub rsp, 0x40
  140c02dd5  488d6c2440               lea rbp, [rsp + 0x40]
  140c02dda  48c745f8feffffff         mov qword ptr [rbp - 8], 0xfffffffffffffffe
  140c02de2  31c0                     xor eax, eax
  140c02de4  48894df0                 mov qword ptr [rbp - 0x10], rcx
  140c02de8  483b4140                 cmp rax, qword ptr [rcx + 0x40]
  140c02dec  700d                     jo 0x140c02dfb
  140c02dee  488b45f0                 mov rax, qword ptr [rbp - 0x10]
  140c02df2  488d4840                 lea rcx, [rax + 0x40]
  140c02df6  e865cafdff               call 0x140bdf860
  140c02dfb  488b45f0                 mov rax, qword ptr [rbp - 0x10]
  140c02dff  488b4830                 mov rcx, qword ptr [rax + 0x30]
  140c02e03  488b5038                 mov rdx, qword ptr [rax + 0x38]
  140c02e07  48894de8                 mov qword ptr [rbp - 0x18], rcx
  140c02e0b  e8a0e6fdff               call 0x140be14b0
  140c02e10  488b45f0                 mov rax, qword ptr [rbp - 0x10]
  140c02e14  488b4028                 mov rax, qword ptr [rax + 0x28]
  140c02e18  4885c0                   test rax, rax
  140c02e1b  7418                     je 0x140c02e35
  140c02e1d  486bd058                 imul rdx, rax, 0x58
  140c02e21  41b808000000             mov r8d, 8
  140c02e27  488b4de8                 mov rcx, qword ptr [rbp - 0x18]
  140c02e2b  4883c440                 add rsp, 0x40
  140c02e2f  5d                       pop rbp
  140c02e30  e9eb0da1ff               jmp 0x140613c20
  140c02e35  90                       nop 
  140c02e36  4883c440                 add rsp, 0x40
  140c02e3a  5d                       pop rbp
  140c02e3b  c3                       ret 
  140c02e3c  0f1f4000                 nop dword ptr [rax]
  140c02e40  4889542410               mov qword ptr [rsp + 0x10], rdx
  140c02e45  55                       push rbp
  140c02e46  4883ec20                 sub rsp, 0x20
  140c02e4a  488d6a40                 lea rbp, [rdx + 0x40]
  140c02e4e  488b45f0                 mov rax, qword ptr [rbp - 0x10]
  140c02e52  488b4028                 mov rax, qword ptr [rax + 0x28]
  140c02e56  4885c0                   test rax, rax
  140c02e59  7413                     je 0x140c02e6e
  140c02e5b  486bd058                 imul rdx, rax, 0x58
  140c02e5f  41b808000000             mov r8d, 8
  140c02e65  488b4de8                 mov rcx, qword ptr [rbp - 0x18]
  140c02e69  e8b20da1ff               call 0x140613c20
  140c02e6e  90                       nop 
  140c02e6f  4883c420                 add rsp, 0x20
  140c02e73  5d                       pop rbp
  140c02e74  c3                       ret 
  140c02e75  66662e0f1f840000000000   nop word ptr cs:[rax + rax]
  140c02e80  4889542410               mov qword ptr [rsp + 0x10], rdx
  140c02e85  55                       push rbp
  140c02e86  4883ec20                 sub rsp, 0x20
  140c02e8a  488d6a40                 lea rbp, [rdx + 0x40]
  140c02e8e  488b4df0                 mov rcx, qword ptr [rbp - 0x10]
  140c02e92  4883c128                 add rcx, 0x28
  140c02e96  e825f1fdff               call 0x140be1fc0
  140c02e9b  90                       nop 
  140c02e9c  4883c420                 add rsp, 0x20
  140c02ea0  5d                       pop rbp
  140c02ea1  c3                       ret 
  140c02ea2  cc                       int3 
  140c02ea3  cc                       int3 
  140c02ea4  cc                       int3 
  140c02ea5  cc                       int3 
  140c02ea6  cc                       int3 
  140c02ea7  cc                       int3 
  140c02ea8  cc                       int3 
  140c02ea9  cc                       int3 
  140c02eaa  cc                       int3 
  140c02eab  cc                       int3 
  140c02eac  cc                       int3 
  140c02ead  cc                       int3 
  140c02eae  cc                       int3 
  140c02eaf  cc                       int3 
  140c02eb0  56                       push rsi
  140c02eb1  57                       push rdi
  140c02eb2  4881ecb8000000           sub rsp, 0xb8
  140c02eb9  4989c8                   mov r8, rcx
  140c02ebc  48c744242002000000       mov qword ptr [rsp + 0x20], 2
  140c02ec5  488d152455f101           lea rdx, [rip + 0x1f15524] -> 0x142b183f0
  140c02ecc  4c8d0ded55f101           lea r9, [rip + 0x1f155ed] -> 0x142b184c0
  140c02ed3  488d4c2450               lea rcx, [rsp + 0x50]
  140c02ed8  e8636a1800               call 0x140d89940
  140c02edd  f644245001               test byte ptr [rsp + 0x50], 1
  140c02ee2  7448                     je 0x140c02f2c
  140c02ee4  488d542458               lea rdx, [rsp + 0x58]
  140c02ee9  4883bc2488000000ff       cmp qword ptr [rsp + 0x88], -1
  140c02ef2  4c8b842498000000         mov r8, qword ptr [rsp + 0x98]
  140c02efa  4c8b8c24a0000000         mov r9, qword ptr [rsp + 0xa0]
  140c02f02  488b8424a8000000         mov rax, qword ptr [rsp + 0xa8]
  140c02f0a  488b8c24b0000000         mov rcx, qword ptr [rsp + 0xb0]
  140c02f12  0f842b010000             je 0x140c03043
  140c02f18  48894c2428               mov qword ptr [rsp + 0x28], rcx
  140c02f1d  4889442420               mov qword ptr [rsp + 0x20], rax
  140c02f22  c644243000               mov byte ptr [rsp + 0x30], 0
  140c02f27  e926010000               jmp 0x140c03052
  140c02f2c  807c246a00               cmp byte ptr [rsp + 0x6a], 0
  140c02f31  7408                     je 0x140c02f3b
  140c02f33  4531c9                   xor r9d, r9d
  140c02f36  e937010000               jmp 0x140c03072
  140c02f3b  4c8b4c2460               mov r9, qword ptr [rsp + 0x60]
  140c02f40  440fb6442469             movzx r8d, byte ptr [rsp + 0x69]
  140c02f46  4d85c9                   test r9, r9
  140c02f49  0f840f010000             je 0x140c0305e
  140c02f4f  488b8c2498000000         mov rcx, qword ptr [rsp + 0x98]
  140c02f57  488b9424a0000000         mov rdx, qword ptr [rsp + 0xa0]
  140c02f5f  4489c0                   mov eax, r8d
  140c02f62  eb25                     jmp 0x140c02f89
  140c02f64  6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
  140c02f70  4189c2                   mov r10d, eax
  140c02f73  4180f201                 xor r10b, 1
  140c02f77  3c01                     cmp al, 1
  140c02f79  410f95c0                 setne r8b
  140c02f7d  4489d0                   mov eax, r10d
  140c02f80  4d01d9                   add r9, r11
  140c02f83  0f84d5000000             je 0x140c0305e
  140c02f89  4939d1                   cmp r9, rdx
  140c02f8c  7312                     jae 0x140c02fa0
  140c02f8e  42803c09bf               cmp byte ptr [rcx + r9], 0xbf
  140c02f93  7f11                     jg 0x140c02fa6
  140c02f95  e9f1000000               jmp 0x140c0308b
  140c02f9a  660f1f440000             nop word ptr [rax + rax]
  140c02fa0  0f85e5000000             jne 0x140c0308b
  140c02fa6  460fbe5409ff             movsx r10d, byte ptr [rcx + r9 - 1]
  140c02fac  4585d2                   test r10d, r10d
  140c02faf  794d                     jns 0x140c02ffe
  140c02fb1  460fb65c09fe             movzx r11d, byte ptr [rcx + r9 - 2]
  140c02fb7  4180fbc0                 cmp r11b, 0xc0
  140c02fbb  7d1f                     jge 0x140c02fdc
  140c02fbd  420fb67409fd             movzx esi, byte ptr [rcx + r9 - 3]
  140c02fc3  4080fec0                 cmp sil, 0xc0
  140c02fc7  7d19                     jge 0x140c02fe2
  140c02fc9  420fb67c09fc             movzx edi, byte ptr [rcx + r9 - 4]
  140c02fcf  83e707                   and edi, 7
  140c02fd2  c1e706                   shl edi, 6
  140c02fd5  83e63f                   and esi, 0x3f
  140c02fd8  09fe                     or esi, edi
  140c02fda  eb09                     jmp 0x140c02fe5
  140c02fdc  4183e31f                 and r11d, 0x1f
  140c02fe0  eb0d                     jmp 0x140c02fef
  140c02fe2  83e60f                   and esi, 0xf
  140c02fe5  c1e606                   shl esi, 6
  140c02fe8  4183e33f                 and r11d, 0x3f
  140c02fec  4109f3                   or r11d, esi
  140c02fef  41c1e306                 shl r11d, 6
  140c02ff3  4180e23f                 and r10b, 0x3f
  140c02ff7  450fb6d2                 movzx r10d, r10b
  140c02ffb  4509da                   or r10d, r11d
  140c02ffe  41f6c001                 test r8b, 1
  140c03002  7563                     jne 0x140c03067
  140c03004  49c7c3ffffffff           mov r11, 0xffffffffffffffff
  140c0300b  4181fa80000000           cmp r10d, 0x80
  140c03012  0f8258ffffff             jb 0x140c02f70
  140c03018  49c7c3feffffff           mov r11, 0xfffffffffffffffe
  140c0301f  4181fa00080000           cmp r10d, 0x800
  140c03026  0f8244ffffff             jb 0x140c02f70
  140c0302c  4531db                   xor r11d, r11d
  140c0302f  4181fa00000100           cmp r10d, 0x10000
  140c03036  410f92c3                 setb r11b
  140c0303a  4983cbfc                 or r11, 0xfffffffffffffffc
  140c0303e  e92dffffff               jmp 0x140c02f70
  140c03043  48894c2428               mov qword ptr [rsp + 0x28], rcx
  140c03048  4889442420               mov qword ptr [rsp + 0x20], rax
  140c0304d  c644243001               mov byte ptr [rsp + 0x30], 1
  140c03052  488d4c2438               lea rcx, [rsp + 0x38]
  140c03057  e844000000               call 0x140c030a0
  140c0305c  eb19                     jmp 0x140c03077
  140c0305e  4531c9                   xor r9d, r9d
  140c03061  41f6c001                 test r8b, 1
  140c03065  740b                     je 0x140c03072
  140c03067  4c894c2440               mov qword ptr [rsp + 0x40], r9
  140c0306c  41b901000000             mov r9d, 1
  140c03072  4c894c2438               mov qword ptr [rsp + 0x38], r9
  140c03077  488b442438               mov rax, qword ptr [rsp + 0x38]
  140c0307c  488b542440               mov rdx, qword ptr [rsp + 0x40]
  140c03081  4881c4b8000000           add rsp, 0xb8
  140c03088  5f                       pop rdi
  140c03089  5e                       pop rsi
  140c0308a  c3                       ret 
  140c0308b  488d05f655f101           lea rax, [rip + 0x1f155f6] -> 0x142b18688
  140c03092  4889442420               mov qword ptr [rsp + 0x20], rax
  140c03097  4531c0                   xor r8d, r8d
  140c0309a  e8a168cd01               call 0x1428d9940
  140c0309f  cc                       int3 
  140c030a0  4157                     push r15
  140c030a2  4156                     push r14
  140c030a4  4155                     push r13
  140c030a6  4154                     push r12
  140c030a8  56                       push rsi
  140c030a9  57                       push rdi
  140c030aa  55                       push rbp
  140c030ab  53                       push rbx
  140c030ac  4883ec48                 sub rsp, 0x48
  140c030b0  4889d0                   mov rax, rdx
  140c030b3  48894c2430               mov qword ptr [rsp + 0x30], rcx
  140c030b8  488b9424b8000000         mov rdx, qword ptr [rsp + 0xb8]
  140c030c0  488b6828                 mov rbp, qword ptr [rax + 0x28]
  140c030c4  4989ea                   mov r10, rbp
  140c030c7  4929d2                   sub r10, rdx
  140c030ca  4d39ca                   cmp r10, r9
  140c030cd  0f836f010000             jae 0x140c03242
  140c030d3  440fb6b424c0000000       movzx r14d, byte ptr [rsp + 0xc0]
  140c030dc  488bbc24b0000000         mov rdi, qword ptr [rsp + 0xb0]
  140c030e4  488b5818                 mov rbx, qword ptr [rax + 0x18]
  140c030e8  488b4808                 mov rcx, qword ptr [rax + 8]
  140c030ec  4c8b5810                 mov r11, qword ptr [rax + 0x10]
  140c030f0  4c895c2438               mov qword ptr [rsp + 0x38], r11
  140c030f5  4c8b6838                 mov r13, qword ptr [rax + 0x38]
  140c030f9  4c8d5fff                 lea r11, [rdi - 1]
  140c030fd  4c895c2440               mov qword ptr [rsp + 0x40], r11
  140c03102  48895c2428               mov qword ptr [rsp + 0x28], rbx
  140c03107  eb1d                     jmp 0x140c03126
  140c03109  0f1f8000000000           nop dword ptr [rax]
  140c03110  48897038                 mov qword ptr [rax + 0x38], rsi
  140c03114  4989f5                   mov r13, rsi
  140c03117  4c89d5                   mov rbp, r10
  140c0311a  4929d2                   sub r10, rdx
  140c0311d  4d39ca                   cmp r10, r9
  140c03120  0f831c010000             jae 0x140c03242
  140c03126  470fb61c10               movzx r11d, byte ptr [r8 + r10]
  140c0312b  4c0fa3db                 bt rbx, r11
  140c0312f  737f                     jae 0x140c031b0
  140c03131  4939cd                   cmp r13, rcx
  140c03134  4989cb                   mov r11, rcx
  140c03137  4d0f42dd                 cmovb r11, r13
  140c0313b  4489f3                   mov ebx, r14d
  140c0313e  4584f6                   test r14b, r14b
  140c03141  4c0f45d9                 cmovne r11, rcx
  140c03145  498d73ff                 lea rsi, [r11 - 1]
  140c03149  4839d6                   cmp rsi, rdx
  140c0314c  0f837e000000             jae 0x140c031d0
  140c03152  4d89dc                   mov r12, r11
  140c03155  49f7dc                   neg r12
  140c03158  488b742440               mov rsi, qword ptr [rsp + 0x40]
  140c0315d  4e8d3c1e                 lea r15, [rsi + r11]
  140c03161  4b8d3413                 lea rsi, [r11 + r10]
  140c03165  48ffce                   dec rsi
  140c03168  0f1f840000000000         nop dword ptr [rax + rax]
  140c03170  4d85e4                   test r12, r12
  140c03173  7464                     je 0x140c031d9
  140c03175  4c39ce                   cmp rsi, r9
  140c03178  0f8336010000             jae 0x140c032b4
  140c0317e  450fb61f                 movzx r11d, byte ptr [r15]
  140c03182  49ffc4                   inc r12
  140c03185  49ffcf                   dec r15
  140c03188  4c8d76ff                 lea r14, [rsi - 1]
  140c0318c  453a1c30                 cmp r11b, byte ptr [r8 + rsi]
  140c03190  4c89f6                   mov rsi, r14
  140c03193  74db                     je 0x140c03170
  140c03195  4829cd                   sub rbp, rcx
  140c03198  4c29e5                   sub rbp, r12
  140c0319b  48896828                 mov qword ptr [rax + 0x28], rbp
  140c0319f  4889d6                   mov rsi, rdx
  140c031a2  e983000000               jmp 0x140c0322a
  140c031a7  660f1f840000000000       nop word ptr [rax + rax]
  140c031b0  4c895028                 mov qword ptr [rax + 0x28], r10
  140c031b4  4889d6                   mov rsi, rdx
  140c031b7  4584f6                   test r14b, r14b
  140c031ba  0f8450ffffff             je 0x140c03110
  140c031c0  e952ffffff               jmp 0x140c03117
  140c031c5  66662e0f1f840000000000   nop word ptr cs:[rax + rax]
  140c031d0  4d85db                   test r11, r11
  140c031d3  0f85ed000000             jne 0x140c032c6
  140c031d9  84db                     test bl, bl
  140c031db  4c89ee                   mov rsi, r13
  140c031de  480f45f2                 cmovne rsi, rdx
  140c031e2  4f8d3c10                 lea r15, [r8 + r10]
  140c031e6  4989cc                   mov r12, rcx
  140c031e9  0f1f8000000000           nop dword ptr [rax]
  140c031f0  4939f4                   cmp r12, rsi
  140c031f3  735e                     jae 0x140c03253
  140c031f5  4939d4                   cmp r12, rdx
  140c031f8  0f8387000000             jae 0x140c03285
  140c031fe  4f8d1c22                 lea r11, [r10 + r12]
  140c03202  4d39cb                   cmp r11, r9
  140c03205  0f838d000000             jae 0x140c03298
  140c0320b  460fb61c27               movzx r11d, byte ptr [rdi + r12]
  140c03210  4d8d742401               lea r14, [r12 + 1]
  140c03215  473a1c27                 cmp r11b, byte ptr [r15 + r12]
  140c03219  4d89f4                   mov r12, r14
  140c0321c  74d2                     je 0x140c031f0
  140c0321e  488b742438               mov rsi, qword ptr [rsp + 0x38]
  140c03223  4829f5                   sub rbp, rsi
  140c03226  48896828                 mov qword ptr [rax + 0x28], rbp
  140c0322a  4989ea                   mov r10, rbp
  140c0322d  4189de                   mov r14d, ebx
  140c03230  84db                     test bl, bl
  140c03232  488b5c2428               mov rbx, qword ptr [rsp + 0x28]
  140c03237  0f84d3feffff             je 0x140c03110
  140c0323d  e9d5feffff               jmp 0x140c03117
  140c03242  48c7402800000000         mov qword ptr [rax + 0x28], 0
  140c0324a  31c0                     xor eax, eax
  140c0324c  488b4c2430               mov rcx, qword ptr [rsp + 0x30]
  140c03251  eb1e                     jmp 0x140c03271
  140c03253  4c895028                 mov qword ptr [rax + 0x28], r10
  140c03257  84db                     test bl, bl
  140c03259  7504                     jne 0x140c0325f
  140c0325b  48895038                 mov qword ptr [rax + 0x38], rdx
  140c0325f  488b4c2430               mov rcx, qword ptr [rsp + 0x30]
  140c03264  4c895108                 mov qword ptr [rcx + 8], r10
  140c03268  48896910                 mov qword ptr [rcx + 0x10], rbp
  140c0326c  b801000000               mov eax, 1
  140c03271  488901                   mov qword ptr [rcx], rax
  140c03274  4883c448                 add rsp, 0x48
  140c03278  5b                       pop rbx
  140c03279  5d                       pop rbp
  140c0327a  5f                       pop rdi
  140c0327b  5e                       pop rsi
  140c0327c  415c                     pop r12
  140c0327e  415d                     pop r13
  140c03280  415e                     pop r14
  140c03282  415f                     pop r15
  140c03284  c3                       ret 
  140c03285  4839d1                   cmp rcx, rdx
  140c03288  480f46ca                 cmovbe rcx, rdx
  140c0328c  4c8d05554ff101           lea r8, [rip + 0x1f14f55] -> 0x142b181e8
  140c03293  e88062cd01               call 0x1428d9518
  140c03298  4901ca                   add r10, rcx
  140c0329b  4d39d1                   cmp r9, r10
  140c0329e  4d0f47d1                 cmova r10, r9
  140c032a2  4c8d05574ff101           lea r8, [rip + 0x1f14f57] -> 0x142b18200
  140c032a9  4c89d1                   mov rcx, r10
  140c032ac  4c89ca                   mov rdx, r9
  140c032af  e86462cd01               call 0x1428d9518
  140c032b4  4c8d05754ff101           lea r8, [rip + 0x1f14f75] -> 0x142b18230
  140c032bb  4889f1                   mov rcx, rsi
  140c032be  4c89ca                   mov rdx, r9
  140c032c1  e85262cd01               call 0x1428d9518
  140c032c6  4c8d054b4ff101           lea r8, [rip + 0x1f14f4b] -> 0x142b18218
  140c032cd  4889f1                   mov rcx, rsi
  140c032d0  e84362cd01               call 0x1428d9518
  140c032d5  cc                       int3 
  140c032d6  cc                       int3 
  140c032d7  cc                       int3 
  140c032d8  cc                       int3 
  140c032d9  cc                       int3 
  140c032da  cc                       int3 
  140c032db  cc                       int3 
  140c032dc  cc                       int3 
  140c032dd  cc                       int3 
  140c032de  cc                       int3 
  140c032df  cc                       int3 
  140c032e0  4883ec48                 sub rsp, 0x48
  140c032e4  4889d0                   mov rax, rdx
  140c032e7  48894c2440               mov qword ptr [rsp + 0x40], rcx
  140c032ec  4c8d4918                 lea r9, [rcx + 0x18]
  140c032f0  488d0dd150f101           lea rcx, [rip + 0x1f150d1] -> 0x142b183c8
  140c032f7  48894c2430               mov qword ptr [rsp + 0x30], rcx
  140c032fc  488d4c2440               lea rcx, [rsp + 0x40]
  140c03301  48894c2428               mov qword ptr [rsp + 0x28], rcx
  140c03306  488d0d9b50f101           lea rcx, [rip + 0x1f1509b] -> 0x142b183a8
  140c0330d  48894c2420               mov qword ptr [rsp + 0x20], rcx
  140c03312  488d15cf50f101           lea rdx, [rip + 0x1f150cf] -> 0x142b183e8
  140c03319  41b808000000             mov r8d, 8
  140c0331f  4889c1                   mov rcx, rax
  140c03322  e809411800               call 0x140d87430
  140c03327  90                       nop 
  140c03328  4883c448                 add rsp, 0x48
  140c0332c  c3                       ret 
  140c0332d  cc                       int3 
  140c0332e  cc                       int3 
  140c0332f  cc                       int3 
  140c03330  4883ec38                 sub rsp, 0x38
  140c03334  4889d0                   mov rax, rdx
  140c03337  488b11                   mov rdx, qword ptr [rcx]
  140c0333a  4c8d05675af101           lea r8, [rip + 0x1f15a67] -> 0x142b18da8
  140c03341  49631490                 movsxd rdx, dword ptr [r8 + rdx*4]
  140c03345  4c01c2                   add rdx, r8
  140c03348  ffe2                     jmp rdx
  140c0334a  488d15a751f101           lea rdx, [rip + 0x1f151a7] -> 0x142b184f8
  140c03351  41b80b000000             mov r8d, 0xb
  140c03357  4889c1                   mov rcx, rax
  140c0335a  4883c438                 add rsp, 0x38
  140c0335e  e92d381800               jmp 0x140d86b90
  140c03363  4883c108                 add rcx, 8
  140c03367  48894c2430               mov qword ptr [rsp + 0x30], rcx
  140c0336c  488d0d9551f101           lea rcx, [rip + 0x1f15195] -> 0x142b18508
