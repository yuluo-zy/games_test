142071590: 55 push rbp
142071591: 4157 push r15
142071593: 4156 push r14
142071595: 4155 push r13
142071597: 4154 push r12
142071599: 56 push rsi
14207159a: 57 push rdi
14207159b: 53 push rbx
14207159c: 4881ec48010000 sub rsp, 0x148
1420715a3: 488dac2480000000 lea rbp, [rsp + 0x80]
1420715ab: 440f2995b0000000 movaps xmmword ptr [rbp + 0xb0], xmm10
1420715b3: 440f298da0000000 movaps xmmword ptr [rbp + 0xa0], xmm9
1420715bb: 440f298590000000 movaps xmmword ptr [rbp + 0x90], xmm8
1420715c3: 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
1420715ca: 0f297570 movaps xmmword ptr [rbp + 0x70], xmm6
1420715ce: 48c74568feffffff mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
1420715d6: 4889d6 mov rsi, rdx
1420715d9: 4889cf mov rdi, rcx
1420715dc: e81f54bafe call 0x140c16a00 ; puffin::are_scopes_on
1420715e1: 84c0 test al, al
1420715e3: 745f je 0x142071644
1420715e5: 4c8d35c61bf600 lea r14, [rip + 0xf61bc6]
1420715ec: bb4a000000 mov ebx, 0x4a
1420715f1: ba4a000000 mov edx, 0x4a
1420715f6: 4c89f1 mov rcx, r14
1420715f9: e872d8ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
1420715fe: 4883f801 cmp rax, 1
142071602: 0f85b1000000 jne 0x1420716b9
142071608: 4989d1 mov r9, rdx
14207160b: 4885d2 test rdx, rdx
14207160e: 743d je 0x14207164d
142071610: 4983f94a cmp r9, 0x4a
142071614: 7335 jae 0x14207164b
142071616: 488d05951bf600 lea rax, [rip + 0xf61b95]
14207161d: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
142071622: 7f29 jg 0x14207164d
142071624: 488d05ad1df600 lea rax, [rip + 0xf61dad]
14207162b: 4889442420 mov qword ptr [rsp + 0x20], rax
142071630: 488d0d7b1bf600 lea rcx, [rip + 0xf61b7b]
142071637: ba4a000000 mov edx, 0x4a
14207163c: 4531c0 xor r8d, r8d
14207163f: e8fc828600 call 0x1428d9940 ; core::str::slice_error_fail
142071644: 31c0 xor eax, eax
142071646: e9bf010000 jmp 0x14207180a ; unknown
14207164b: 75d7 jne 0x142071624
14207164d: 4c8d355e1bf600 lea r14, [rip + 0xf61b5e]
142071654: 4c89f1 mov rcx, r14
142071657: 4c89ca mov rdx, r9
14207165a: e811d8ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207165f: 4883f801 cmp rax, 1
142071663: 7554 jne 0x1420716b9
142071665: 4989d0 mov r8, rdx
142071668: 4983c002 add r8, 2
14207166c: 7439 je 0x1420716a7
14207166e: 4983f84a cmp r8, 0x4a
142071672: 7331 jae 0x1420716a5
142071674: 488d05371bf600 lea rax, [rip + 0xf61b37]
14207167b: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
142071680: 7f25 jg 0x1420716a7
142071682: 488d05671df600 lea rax, [rip + 0xf61d67]
142071689: 4889442420 mov qword ptr [rsp + 0x20], rax
14207168e: 488d0d1d1bf600 lea rcx, [rip + 0xf61b1d]
142071695: ba4a000000 mov edx, 0x4a
14207169a: 41b94a000000 mov r9d, 0x4a
1420716a0: e89b828600 call 0x1428d9940 ; core::str::slice_error_fail
1420716a5: 75db jne 0x142071682
1420716a7: bb48000000 mov ebx, 0x48
1420716ac: 4829d3 sub rbx, rdx
1420716af: 4c8d35fc1af600 lea r14, [rip + 0xf61afc]
1420716b6: 4d01c6 add r14, r8
1420716b9: 4c8d059e1df600 lea r8, [rip + 0xf61d9e]
1420716c0: 4c8d3d691df600 lea r15, [rip + 0xf61d69]
1420716c7: 660f1f840000000000 nop word ptr [rax + rax]
1420716d0: 410fbe40ff movsx eax, byte ptr [r8 - 1]
1420716d5: 85c0 test eax, eax
1420716d7: 7817 js 0x1420716f0
1420716d9: 49ffc8 dec r8
1420716dc: 83f85c cmp eax, 0x5c
1420716df: 7563 jne 0x142071744
1420716e1: eb73 jmp 0x142071756 ; unknown
1420716e3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1420716f0: 410fb648fe movzx ecx, byte ptr [r8 - 2]
1420716f5: 80f9c0 cmp cl, 0xc0
1420716f8: 7d23 jge 0x14207171d
1420716fa: 410fb650fd movzx edx, byte ptr [r8 - 3]
1420716ff: 80fac0 cmp dl, 0xc0
142071702: 7d22 jge 0x142071726
142071704: 450fb648fc movzx r9d, byte ptr [r8 - 4]
142071709: 4983c0fc add r8, -4
14207170d: 4183e107 and r9d, 7
142071711: 41c1e106 shl r9d, 6
142071715: 83e23f and edx, 0x3f
142071718: 4409ca or edx, r9d
14207171b: eb10 jmp 0x14207172d ; unknown
14207171d: 4983c0fe add r8, -2
142071721: 83e11f and ecx, 0x1f
142071724: eb0f jmp 0x142071735 ; unknown
142071726: 4983c0fd add r8, -3
14207172a: 83e20f and edx, 0xf
14207172d: c1e206 shl edx, 6
142071730: 83e13f and ecx, 0x3f
142071733: 09d1 or ecx, edx
142071735: c1e106 shl ecx, 6
142071738: 243f and al, 0x3f
14207173a: 0fb6c0 movzx eax, al
14207173d: 09c8 or eax, ecx
14207173f: 83f85c cmp eax, 0x5c
142071742: 7412 je 0x142071756
142071744: 83f82f cmp eax, 0x2f
142071747: 740d je 0x142071756
142071749: 4d39f8 cmp r8, r15
14207174c: 7582 jne 0x1420716d0
14207174e: 41bc2e000000 mov r12d, 0x2e
142071754: eb4d jmp 0x1420717a3 ; unknown
142071756: 4c8d3dd31cf600 lea r15, [rip + 0xf61cd3]
14207175d: 4d29f8 sub r8, r15
142071760: 49ffc0 inc r8
142071763: 7432 je 0x142071797
142071765: 4983f82e cmp r8, 0x2e
142071769: 732a jae 0x142071795
14207176b: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
142071770: 7f25 jg 0x142071797
142071772: 488d053f1cf600 lea rax, [rip + 0xf61c3f]
142071779: 4889442420 mov qword ptr [rsp + 0x20], rax
14207177e: 488d0dab1cf600 lea rcx, [rip + 0xf61cab]
142071785: ba2e000000 mov edx, 0x2e
14207178a: 41b92e000000 mov r9d, 0x2e
142071790: e8ab818600 call 0x1428d9940 ; core::str::slice_error_fail
142071795: 75db jne 0x142071772
142071797: 41bc2e000000 mov r12d, 0x2e
14207179d: 4d29c4 sub r12, r8
1420717a0: 4d01c7 add r15, r8
1420717a3: e88869bafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1420717a8: 4889c1 mov rcx, rax
1420717ab: 488b00 mov rax, qword ptr [rax]
1420717ae: 4883f801 cmp rax, 1
1420717b2: 0f85f8050000 jne 0x142071db0
1420717b8: 4883c108 add rcx, 8
1420717bc: 48833900 cmp qword ptr [rcx], 0
1420717c0: 0f850e060000 jne 0x142071dd4
1420717c6: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1420717cd: 48894d48 mov qword ptr [rbp + 0x48], rcx
1420717d1: 4883c108 add rcx, 8
1420717d5: 4c89642420 mov qword ptr [rsp + 0x20], r12
1420717da: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
1420717e3: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
1420717ec: 4c89f2 mov rdx, r14
1420717ef: 4989d8 mov r8, rbx
1420717f2: 4d89f9 mov r9, r15
1420717f5: e84655bafe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1420717fa: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
1420717fe: 48ff01 inc qword ptr [rcx]
142071801: 48894530 mov qword ptr [rbp + 0x30], rax
142071805: b801000000 mov eax, 1
14207180a: 48894528 mov qword ptr [rbp + 0x28], rax
14207180e: 488b0e mov rcx, qword ptr [rsi]
142071811: b204 mov dl, 4
142071813: e8380492fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
142071818: 48895520 mov qword ptr [rbp + 0x20], rdx
14207181c: 48894540 mov qword ptr [rbp + 0x40], rax
142071820: 488b1f mov rbx, qword ptr [rdi]
142071823: 4c8b7708 mov r14, qword ptr [rdi + 8]
142071827: 4c8bbbf8000000 mov r15, qword ptr [rbx + 0xf8]
14207182e: 488b8300010000 mov rax, qword ptr [rbx + 0x100]
142071835: 4d8d2487 lea r12, [r15 + rax*4]
142071839: 0fb68308010000 movzx eax, byte ptr [rbx + 0x108]
142071840: 884548 mov byte ptr [rbp + 0x48], al
142071843: b808000000 mov eax, 8
142071848: 48894538 mov qword ptr [rbp + 0x38], rax
14207184c: 31ff xor edi, edi
14207184e: f30f1035ae408b00 movss xmm6, dword ptr [rip + 0x8b40ae]
142071856: 0f283d23d18b00 movaps xmm7, xmmword ptr [rip + 0x8bd123]
14207185d: 31f6 xor esi, esi
14207185f: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
142071867: 48c7455800000000 mov qword ptr [rbp + 0x58], 0
14207186f: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142071877: 4531ed xor r13d, r13d
14207187a: 807d4800 cmp byte ptr [rbp + 0x48], 0
14207187e: 0f857c010000 jne 0x142071a00
142071884: 4839fe cmp rsi, rdi
142071887: 0f8552010000 jne 0x1420719df
14207188d: 0f1f00 nop dword ptr [rax]
142071890: 4d39e7 cmp r15, r12
142071893: 0f845b040000 je 0x142071cf4
142071899: 418b0f mov ecx, dword ptr [r15]
14207189c: 4983c704 add r15, 4
1420718a0: 498b8600010000 mov rax, qword ptr [r14 + 0x100]
1420718a7: 488d0c89 lea rcx, [rcx + rcx*4]
1420718ab: 48c1e105 shl rcx, 5
1420718af: 488b7c0858 mov rdi, qword ptr [rax + rcx + 0x58]
1420718b4: 4885ff test rdi, rdi
1420718b7: 74d7 je 0x142071890
1420718b9: 4801c8 add rax, rcx
1420718bc: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
1420718c2: 498b96a8010000 mov rdx, qword ptr [r14 + 0x1a8]
1420718c9: 4c8d0cc9 lea r9, [rcx + rcx*8]
1420718cd: 4a8d0cca lea rcx, [rdx + r9*8]
1420718d1: 4c8b8310010000 mov r8, qword ptr [rbx + 0x110]
1420718d8: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
1420718dd: 4c39c2 cmp rdx, r8
1420718e0: 7633 jbe 0x142071915
1420718e2: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
1420718e6: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
1420718ea: 4d85c0 test r8, r8
1420718ed: 7426 je 0x142071915
1420718ef: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
1420718f3: 49f7d0 not r8
1420718f6: 4f8d0440 lea r8, [r8 + r8*2]
1420718fa: 49c1e004 shl r8, 4
1420718fe: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
142071903: 4c894550 mov qword ptr [rbp + 0x50], r8
142071907: 4c8b8318010000 mov r8, qword ptr [rbx + 0x118]
14207190e: 4c39c2 cmp rdx, r8
142071911: 7716 ja 0x142071929
142071913: eb47 jmp 0x14207195c ; unknown
142071915: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
14207191d: 4c8b8318010000 mov r8, qword ptr [rbx + 0x118]
142071924: 4c39c2 cmp rdx, r8
142071927: 7633 jbe 0x14207195c
142071929: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207192d: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
142071931: 4d85c0 test r8, r8
142071934: 7426 je 0x14207195c
142071936: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207193a: 49f7d0 not r8
14207193d: 4f8d0440 lea r8, [r8 + r8*2]
142071941: 49c1e004 shl r8, 4
142071945: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14207194a: 4c894558 mov qword ptr [rbp + 0x58], r8
14207194e: 4c8b8320010000 mov r8, qword ptr [rbx + 0x120]
142071955: 4c39c2 cmp rdx, r8
142071958: 764d jbe 0x1420719a7
14207195a: eb14 jmp 0x142071970 ; unknown
14207195c: 48c7455800000000 mov qword ptr [rbp + 0x58], 0
142071964: 4c8b8320010000 mov r8, qword ptr [rbx + 0x120]
14207196b: 4c39c2 cmp rdx, r8
14207196e: 7637 jbe 0x1420719a7
142071970: 488b5138 mov rdx, qword ptr [rcx + 0x38]
142071974: 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
142071978: 4885d2 test rdx, rdx
14207197b: 742a je 0x1420719a7
14207197d: 488b4918 mov rcx, qword ptr [rcx + 0x18]
142071981: 48f7d2 not rdx
142071984: 488d1452 lea rdx, [rdx + rdx*2]
142071988: 48c1e204 shl rdx, 4
14207198c: 488b4c1110 mov rcx, qword ptr [rcx + rdx + 0x10]
142071991: 48894d60 mov qword ptr [rbp + 0x60], rcx
142071995: 488b8b28010000 mov rcx, qword ptr [rbx + 0x128]
14207199c: 48398888000000 cmp qword ptr [rax + 0x88], rcx
1420719a3: 762d jbe 0x1420719d2
1420719a5: eb18 jmp 0x1420719bf ; unknown
1420719a7: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
1420719af: 488b8b28010000 mov rcx, qword ptr [rbx + 0x128]
1420719b6: 48398888000000 cmp qword ptr [rax + 0x88], rcx
1420719bd: 7613 jbe 0x1420719d2
1420719bf: 488b9080000000 mov rdx, qword ptr [rax + 0x80]
1420719c6: 48833cca00 cmp qword ptr [rdx + rcx*8], 0
1420719cb: 7405 je 0x1420719d2
1420719cd: 41b501 mov r13b, 1
1420719d0: eb03 jmp 0x1420719d5 ; unknown
1420719d2: 4531ed xor r13d, r13d
1420719d5: 488b4050 mov rax, qword ptr [rax + 0x50]
1420719d9: 48894538 mov qword ptr [rbp + 0x38], rax
1420719dd: 31f6 xor esi, esi
1420719df: 4889f0 mov rax, rsi
1420719e2: 48c1e004 shl rax, 4
1420719e6: 488b4d38 mov rcx, qword ptr [rbp + 0x38]
1420719ea: 8b4c0108 mov ecx, dword ptr [rcx + rax + 8]
1420719ee: e94f010000 jmp 0x142071b42 ; unknown
1420719f3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142071a00: 4839fe cmp rsi, rdi
142071a03: 0f8537010000 jne 0x142071b40
142071a09: 0f1f8000000000 nop dword ptr [rax]
142071a10: 4d39e7 cmp r15, r12
142071a13: 0f84db020000 je 0x142071cf4
142071a19: 418b0f mov ecx, dword ptr [r15]
142071a1c: 4983c704 add r15, 4
142071a20: 498b86a8010000 mov rax, qword ptr [r14 + 0x1a8]
142071a27: 488d0cc9 lea rcx, [rcx + rcx*8]
142071a2b: 488b7cc810 mov rdi, qword ptr [rax + rcx*8 + 0x10]
142071a30: 4885ff test rdi, rdi
142071a33: 74db je 0x142071a10
142071a35: 488d04c8 lea rax, [rax + rcx*8]
142071a39: 488b9310010000 mov rdx, qword ptr [rbx + 0x110]
142071a40: 488b4840 mov rcx, qword ptr [rax + 0x40]
142071a44: 4839d1 cmp rcx, rdx
142071a47: 7633 jbe 0x142071a7c
142071a49: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142071a4d: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142071a51: 4885d2 test rdx, rdx
142071a54: 7426 je 0x142071a7c
142071a56: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142071a5a: 48f7d2 not rdx
142071a5d: 488d1452 lea rdx, [rdx + rdx*2]
142071a61: 48c1e204 shl rdx, 4
142071a65: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142071a6a: 48895550 mov qword ptr [rbp + 0x50], rdx
142071a6e: 488b9318010000 mov rdx, qword ptr [rbx + 0x118]
142071a75: 4839d1 cmp rcx, rdx
142071a78: 7716 ja 0x142071a90
142071a7a: eb47 jmp 0x142071ac3 ; unknown
142071a7c: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
142071a84: 488b9318010000 mov rdx, qword ptr [rbx + 0x118]
142071a8b: 4839d1 cmp rcx, rdx
142071a8e: 7633 jbe 0x142071ac3
142071a90: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142071a94: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142071a98: 4885d2 test rdx, rdx
142071a9b: 7426 je 0x142071ac3
142071a9d: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142071aa1: 48f7d2 not rdx
142071aa4: 488d1452 lea rdx, [rdx + rdx*2]
142071aa8: 48c1e204 shl rdx, 4
142071aac: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142071ab1: 48895558 mov qword ptr [rbp + 0x58], rdx
142071ab5: 488b9320010000 mov rdx, qword ptr [rbx + 0x120]
142071abc: 4839d1 cmp rcx, rdx
142071abf: 764e jbe 0x142071b0f
142071ac1: eb14 jmp 0x142071ad7 ; unknown
142071ac3: 48c7455800000000 mov qword ptr [rbp + 0x58], 0
142071acb: 488b9320010000 mov rdx, qword ptr [rbx + 0x120]
142071ad2: 4839d1 cmp rcx, rdx
142071ad5: 7638 jbe 0x142071b0f
142071ad7: 4c8b4038 mov r8, qword ptr [rax + 0x38]
142071adb: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
142071adf: 4885d2 test rdx, rdx
142071ae2: 742b je 0x142071b0f
142071ae4: 4c8b4018 mov r8, qword ptr [rax + 0x18]
142071ae8: 48f7d2 not rdx
142071aeb: 488d1452 lea rdx, [rdx + rdx*2]
142071aef: 48c1e204 shl rdx, 4
142071af3: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
142071af8: 48895560 mov qword ptr [rbp + 0x60], rdx
142071afc: 488b9328010000 mov rdx, qword ptr [rbx + 0x128]
142071b03: 4839d1 cmp rcx, rdx
142071b06: 771b ja 0x142071b23
142071b08: 31f6 xor esi, esi
142071b0a: 4531ed xor r13d, r13d
142071b0d: eb31 jmp 0x142071b40 ; unknown
142071b0f: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
142071b17: 488b9328010000 mov rdx, qword ptr [rbx + 0x128]
142071b1e: 4839d1 cmp rcx, rdx
142071b21: 76e5 jbe 0x142071b08
142071b23: 488b4038 mov rax, qword ptr [rax + 0x38]
142071b27: 48833cd000 cmp qword ptr [rax + rdx*8], 0
142071b2c: 410f95c5 setne r13b
142071b30: 31f6 xor esi, esi
142071b32: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142071b40: 89f1 mov ecx, esi
142071b42: 488d0449 lea rax, [rcx + rcx*2]
142071b46: 488b5550 mov rdx, qword ptr [rbp + 0x50]
142071b4a: 488d1482 lea rdx, [rdx + rax*4]
142071b4e: 48c1e004 shl rax, 4
142071b52: 48034558 add rax, qword ptr [rbp + 0x58]
142071b56: 4c8b4560 mov r8, qword ptr [rbp + 0x60]
142071b5a: 498d0c88 lea rcx, [r8 + rcx*4]
142071b5e: 448b4018 mov r8d, dword ptr [rax + 0x18]
142071b62: 44894518 mov dword ptr [rbp + 0x18], r8d
142071b66: 4c8b4010 mov r8, qword ptr [rax + 0x10]
142071b6a: 4c894510 mov qword ptr [rbp + 0x10], r8
142071b6e: 0f2820 movaps xmm4, xmmword ptr [rax]
142071b71: f30f105808 movss xmm3, dword ptr [rax + 8]
142071b76: 440f28c3 movaps xmm8, xmm3
142071b7a: f3440f58c3 addss xmm8, xmm3
142071b7f: f3410f59d8 mulss xmm3, xmm8
142071b84: 0f28cc movaps xmm1, xmm4
142071b87: 0f58cc addps xmm1, xmm4
142071b8a: f30f16c1 movshdup xmm0, xmm1
142071b8e: f30f59c4 mulss xmm0, xmm4
142071b92: 0f28d4 movaps xmm2, xmm4
142071b95: f3410f12e8 movsldup xmm5, xmm8
142071b9a: 0f59ec mulps xmm5, xmm4
142071b9d: 0fc6e4ff shufps xmm4, xmm4, 0xff
142071ba1: 0f59d1 mulps xmm2, xmm1
142071ba4: 0f59cc mulps xmm1, xmm4
142071ba7: 440f28cc movaps xmm9, xmm4
142071bab: f3450f59c8 mulss xmm9, xmm8
142071bb0: f3440f16d2 movshdup xmm10, xmm2
142071bb5: 450f28c2 movaps xmm8, xmm10
142071bb9: f3440f58c3 addss xmm8, xmm3
142071bbe: 0f28e6 movaps xmm4, xmm6
142071bc1: f3410f5ce0 subss xmm4, xmm8
142071bc6: 440f28c0 movaps xmm8, xmm0
142071bca: f3450f58c1 addss xmm8, xmm9
142071bcf: 410f14e0 unpcklps xmm4, xmm8
142071bd3: f3410f5cc1 subss xmm0, xmm9
142071bd8: f30f58da addss xmm3, xmm2
142071bdc: 440f28c6 movaps xmm8, xmm6
142071be0: f3440f5cc3 subss xmm8, xmm3
142071be5: 0fc6c9e1 shufps xmm1, xmm1, 0xe1
142071be9: 410f14c0 unpcklps xmm0, xmm8
142071bed: 440f28c5 movaps xmm8, xmm5
142071bf1: 440f5cc1 subps xmm8, xmm1
142071bf5: 0f58cd addps xmm1, xmm5
142071bf8: f30f105820 movss xmm3, dword ptr [rax + 0x20]
142071bfd: f3410f58d2 addss xmm2, xmm10
142071c02: 440f28ce movaps xmm9, xmm6
142071c06: f3440f5cca subss xmm9, xmm2
142071c0b: f30f106824 movss xmm5, dword ptr [rax + 0x24]
142071c10: 410f16e0 movlhps xmm4, xmm8
142071c14: 0fc6c1d4 shufps xmm0, xmm1, 0xd4
142071c18: 440fc6c101 shufps xmm8, xmm1, 1
142071c1d: 440fc6c930 shufps xmm9, xmm1, 0x30
142071c22: 0f10501c movups xmm2, xmmword ptr [rax + 0x1c]
142071c26: 450fc6c182 shufps xmm8, xmm9, 0x82
142071c2b: 0f104820 movups xmm1, xmmword ptr [rax + 0x20]
142071c2f: 0fc6d200 shufps xmm2, xmm2, 0
142071c33: 0f59d4 mulps xmm2, xmm4
142071c36: 0f28e1 movaps xmm4, xmm1
142071c39: 0fc6e100 shufps xmm4, xmm1, 0
142071c3d: 0f59e0 mulps xmm4, xmm0
142071c40: 0fc6c955 shufps xmm1, xmm1, 0x55
142071c44: 410f59c8 mulps xmm1, xmm8
142071c48: 0f54df andps xmm3, xmm7
142071c4b: 0f54ef andps xmm5, xmm7
142071c4e: 0f28c3 movaps xmm0, xmm3
142071c51: f30fc2c303 cmpunordss xmm0, xmm3
142071c56: 440f28c0 movaps xmm8, xmm0
142071c5a: 440f54c5 andps xmm8, xmm5
142071c5e: f30f5feb maxss xmm5, xmm3
142071c62: f30f10581c movss xmm3, dword ptr [rax + 0x1c]
142071c67: 0f54df andps xmm3, xmm7
142071c6a: 0f55c5 andnps xmm0, xmm5
142071c6d: 410f56c0 orps xmm0, xmm8
142071c71: 0f28e8 movaps xmm5, xmm0
142071c74: f30f5feb maxss xmm5, xmm3
142071c78: f30fc2db03 cmpunordss xmm3, xmm3
142071c7d: 440f28c3 movaps xmm8, xmm3
142071c81: 440f54c0 andps xmm8, xmm0
142071c85: 0f55dd andnps xmm3, xmm5
142071c88: 410f56d8 orps xmm3, xmm8
142071c8c: 0f28c2 movaps xmm0, xmm2
142071c8f: f20f10c4 movsd xmm0, xmm4
142071c93: 0fc6d024 shufps xmm2, xmm0, 0x24
142071c97: 0f284010 movaps xmm0, xmmword ptr [rax + 0x10]
142071c9b: 8b01 mov eax, dword ptr [rcx]
142071c9d: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
142071ca1: 0fc6e149 shufps xmm4, xmm1, 0x49
142071ca5: 0f2965d0 movaps xmmword ptr [rbp - 0x30], xmm4
142071ca9: 0fc6c802 shufps xmm1, xmm0, 2
142071cad: 0fc6c898 shufps xmm1, xmm0, 0x98
142071cb1: 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
142071cb5: 4489e9 mov ecx, r13d
142071cb8: 80e101 and cl, 1
142071cbb: 0fb6c9 movzx ecx, cl
142071cbe: 894df0 mov dword ptr [rbp - 0x10], ecx
142071cc1: 8945f4 mov dword ptr [rbp - 0xc], eax
142071cc4: 48c745f800000000 mov qword ptr [rbp - 8], 0
142071ccc: 488d45c0 lea rax, [rbp - 0x40]
142071cd0: 4889442420 mov qword ptr [rsp + 0x20], rax
142071cd5: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
142071cd9: 4c8d4510 lea r8, [rbp + 0x10]
142071cdd: e89ecb4fff call 0x14156e880 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::SolidVertexColorWindowInstanceData>
142071ce2: 48ffc6 inc rsi
142071ce5: 807d4800 cmp byte ptr [rbp + 0x48], 0
142071ce9: 0f8495fbffff je 0x142071884
142071cef: e90cfdffff jmp 0x142071a00 ; unknown
142071cf4: 488b7540 mov rsi, qword ptr [rbp + 0x40]
142071cf8: 4889f1 mov rcx, rsi
142071cfb: e81001c1fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
142071d00: 4889f1 mov rcx, rsi
142071d03: 4883c901 or rcx, 1
142071d07: e80401c1fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
142071d0c: 31c9 xor ecx, ecx
142071d0e: b808000000 mov eax, 8
142071d13: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
142071d18: 0f85c2000000 jne 0x142071de0
142071d1e: 48837d2800 cmp qword ptr [rbp + 0x28], 0
142071d23: 743e je 0x142071d63
142071d25: e80664bafe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
142071d2a: 4889c1 mov rcx, rax
142071d2d: 488b00 mov rax, qword ptr [rax]
142071d30: 4883f801 cmp rax, 1
142071d34: 7564 jne 0x142071d9a
142071d36: 4883c108 add rcx, 8
142071d3a: 48833900 cmp qword ptr [rcx], 0
142071d3e: 0f8590000000 jne 0x142071dd4
142071d44: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
142071d4b: 48894d48 mov qword ptr [rbp + 0x48], rcx
142071d4f: 4883c108 add rcx, 8
142071d53: 488b5530 mov rdx, qword ptr [rbp + 0x30]
142071d57: e89453bafe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
142071d5c: 488b4548 mov rax, qword ptr [rbp + 0x48]
142071d60: 48ff00 inc qword ptr [rax]
142071d63: 0f287570 movaps xmm6, xmmword ptr [rbp + 0x70]
142071d67: 0f28bd80000000 movaps xmm7, xmmword ptr [rbp + 0x80]
142071d6e: 440f288590000000 movaps xmm8, xmmword ptr [rbp + 0x90]
142071d76: 440f288da0000000 movaps xmm9, xmmword ptr [rbp + 0xa0]
142071d7e: 440f2895b0000000 movaps xmm10, xmmword ptr [rbp + 0xb0]
142071d86: 4881c448010000 add rsp, 0x148
142071d8d: 5b pop rbx
142071d8e: 5f pop rdi
142071d8f: 5e pop rsi
142071d90: 415c pop r12
142071d92: 415d pop r13
142071d94: 415e pop r14
142071d96: 415f pop r15
142071d98: 5d pop rbp
142071d99: c3 ret 
142071d9a: 4885c0 test rax, rax
142071d9d: 7529 jne 0x142071dc8
142071d9f: 31d2 xor edx, edx
142071da1: e82a6a8800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142071da6: 4889c1 mov rcx, rax
142071da9: 4885c0 test rax, rax
142071dac: 758c jne 0x142071d3a
142071dae: eb18 jmp 0x142071dc8 ; unknown
142071db0: 4885c0 test rax, rax
142071db3: 7513 jne 0x142071dc8
142071db5: 31d2 xor edx, edx
142071db7: e8146a8800 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
142071dbc: 4889c1 mov rcx, rax
142071dbf: 4885c0 test rax, rax
142071dc2: 0f85f4f9ffff jne 0x1420717bc
142071dc8: 488d0dd18cbc00 lea rcx, [rip + 0xbc8cd1]
142071dcf: e8fc628600 call 0x1428d80d0 ; std::thread::local::panic_access_error
142071dd4: 488d0d958dbc00 lea rcx, [rip + 0xbc8d95]
142071ddb: e870748600 call 0x1428d9250 ; core::cell::panic_already_borrowed
142071de0: 4889f1 mov rcx, rsi
142071de3: 31d2 xor edx, edx
142071de5: e876a18500 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
142071dea: e92fffffff jmp 0x142071d1e ; unknown
142071def: 90 nop 
142071df0: 4889542410 mov qword ptr [rsp + 0x10], rdx
142071df5: 55 push rbp
142071df6: 4157 push r15
142071df8: 4156 push r14
142071dfa: 4155 push r13
142071dfc: 4154 push r12
142071dfe: 56 push rsi
142071dff: 57 push rdi
142071e00: 53 push rbx
142071e01: 4881ec88000000 sub rsp, 0x88
142071e08: 488daa80000000 lea rbp, [rdx + 0x80]
142071e0f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142071e15: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
142071e1b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142071e21: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142071e26: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
142071e2b: 488b4548 mov rax, qword ptr [rbp + 0x48]
142071e2f: 48ff00 inc qword ptr [rax]
142071e32: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142071e37: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
142071e3c: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142071e42: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142071e48: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
142071e4e: 4881c488000000 add rsp, 0x88
142071e55: 5b pop rbx
142071e56: 5f pop rdi
142071e57: 5e pop rsi
142071e58: 415c pop r12
142071e5a: 415d pop r13
142071e5c: 415e pop r14
142071e5e: 415f pop r15
142071e60: 5d pop rbp
142071e61: c3 ret 
142071e62: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142071e70: 4889542410 mov qword ptr [rsp + 0x10], rdx
142071e75: 55 push rbp
142071e76: 4157 push r15
142071e78: 4156 push r14
142071e7a: 4155 push r13
142071e7c: 4154 push r12
142071e7e: 56 push rsi
142071e7f: 57 push rdi
142071e80: 53 push rbx
142071e81: 4881ec88000000 sub rsp, 0x88
142071e88: 488daa80000000 lea rbp, [rdx + 0x80]
142071e8f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142071e95: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
142071e9b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142071ea1: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142071ea6: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
142071eab: 488b4548 mov rax, qword ptr [rbp + 0x48]
142071eaf: 48ff00 inc qword ptr [rax]
142071eb2: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142071eb7: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
142071ebc: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142071ec2: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142071ec8: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
142071ece: 4881c488000000 add rsp, 0x88
142071ed5: 5b pop rbx
142071ed6: 5f pop rdi
142071ed7: 5e pop rsi
142071ed8: 415c pop r12
142071eda: 415d pop r13
142071edc: 415e pop r14
142071ede: 415f pop r15
142071ee0: 5d pop rbp
142071ee1: c3 ret 
142071ee2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142071ef0: 4889542410 mov qword ptr [rsp + 0x10], rdx
142071ef5: 55 push rbp
142071ef6: 4157 push r15
142071ef8: 4156 push r14
142071efa: 4155 push r13
142071efc: 4154 push r12
142071efe: 56 push rsi
142071eff: 57 push rdi
142071f00: 53 push rbx
142071f01: 4881ec88000000 sub rsp, 0x88
142071f08: 488daa80000000 lea rbp, [rdx + 0x80]
142071f0f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142071f15: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
142071f1b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142071f21: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142071f26: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
142071f2b: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
142071f2f: e89cf3fcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
142071f34: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142071f39: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
142071f3e: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142071f44: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142071f4a: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
142071f50: 4881c488000000 add rsp, 0x88
142071f57: 5b pop rbx
142071f58: 5f pop rdi
142071f59: 5e pop rsi
142071f5a: 415c pop r12
142071f5c: 415d pop r13
142071f5e: 415e pop r14
142071f60: 415f pop r15
142071f62: 5d pop rbp
142071f63: c3 ret 
142071f64: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
142071f70: 4889542410 mov qword ptr [rsp + 0x10], rdx
142071f75: 55 push rbp
142071f76: 4157 push r15
142071f78: 4156 push r14
142071f7a: 4155 push r13
142071f7c: 4154 push r12
142071f7e: 56 push rsi
142071f7f: 57 push rdi
142071f80: 53 push rbx
142071f81: 4881ec88000000 sub rsp, 0x88
142071f88: 488daa80000000 lea rbp, [rdx + 0x80]
142071f8f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
142071f95: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
142071f9b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
142071fa1: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
142071fa6: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
142071fab: 48837d2800 cmp qword ptr [rbp + 0x28], 0
142071fb0: 741f je 0x142071fd1
142071fb2: 488d4530 lea rax, [rbp + 0x30]
142071fb6: 48894508 mov qword ptr [rbp + 8], rax
142071fba: 488d4508 lea rax, [rbp + 8]
142071fbe: 48894500 mov qword ptr [rbp], rax
142071fc2: 488d0d8f13f600 lea rcx, [rip + 0xf6138f]
142071fc9: 4889ea mov rdx, rbp
142071fcc: e8ffd22bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
142071fd1: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
142071fd6: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
142071fdb: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
142071fe1: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
142071fe7: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
142071fed: 4881c488000000 add rsp, 0x88
142071ff4: 5b pop rbx
142071ff5: 5f pop rdi
142071ff6: 5e pop rsi
142071ff7: 415c pop r12
142071ff9: 415d pop r13
142071ffb: 415e pop r14
142071ffd: 415f pop r15
142071fff: 5d pop rbp
142072000: c3 ret 
