1407e5190: 55 push rbp
1407e5191: 4157 push r15
1407e5193: 4156 push r14
1407e5195: 4155 push r13
1407e5197: 4154 push r12
1407e5199: 56 push rsi
1407e519a: 57 push rdi
1407e519b: 53 push rbx
1407e519c: 4881ec18060000 sub rsp, 0x618
1407e51a3: 488dac2480000000 lea rbp, [rsp + 0x80]
1407e51ab: 440f29bd80050000 movaps xmmword ptr [rbp + 0x580], xmm15
1407e51b3: 440f29b570050000 movaps xmmword ptr [rbp + 0x570], xmm14
1407e51bb: 440f29ad60050000 movaps xmmword ptr [rbp + 0x560], xmm13
1407e51c3: 440f29a550050000 movaps xmmword ptr [rbp + 0x550], xmm12
1407e51cb: 440f299d40050000 movaps xmmword ptr [rbp + 0x540], xmm11
1407e51d3: 440f299530050000 movaps xmmword ptr [rbp + 0x530], xmm10
1407e51db: 440f298d20050000 movaps xmmword ptr [rbp + 0x520], xmm9
1407e51e3: 440f298510050000 movaps xmmword ptr [rbp + 0x510], xmm8
1407e51eb: 0f29bd00050000 movaps xmmword ptr [rbp + 0x500], xmm7
1407e51f2: 0f29b5f0040000 movaps xmmword ptr [rbp + 0x4f0], xmm6
1407e51f9: 48c785e8040000feffffff mov qword ptr [rbp + 0x4e8], 0xfffffffffffffffe
1407e5204: 4c898d58020000 mov qword ptr [rbp + 0x258], r9
1407e520b: 4c898560020000 mov qword ptr [rbp + 0x260], r8
1407e5212: 48899568020000 mov qword ptr [rbp + 0x268], rdx
1407e5219: 4989cf mov r15, rcx
1407e521c: e8df174300 call 0x140c16a00 ; puffin::are_scopes_on
1407e5221: 84c0 test al, al
1407e5223: 745c je 0x1407e5281
1407e5225: be52000000 mov esi, 0x52
1407e522a: b952000000 mov ecx, 0x52
1407e522f: e82cfbffff call 0x1407e4d60 ; core::str::impl$0::rfind<ref$<str$> >
1407e5234: 488d1dfd402a02 lea rbx, [rip + 0x22a40fd]
1407e523b: 4883f801 cmp rax, 1
1407e523f: 0f85a7000000 jne 0x1407e52ec
1407e5245: 4989d1 mov r9, rdx
1407e5248: 4885d2 test rdx, rdx
1407e524b: 743d je 0x1407e528a
1407e524d: 4983f952 cmp r9, 0x52
1407e5251: 7335 jae 0x1407e5288
1407e5253: 488d05de402a02 lea rax, [rip + 0x22a40de]
1407e525a: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
1407e525f: 7f29 jg 0x1407e528a
1407e5261: 488d05a8412a02 lea rax, [rip + 0x22a41a8]
1407e5268: 4889442420 mov qword ptr [rsp + 0x20], rax
1407e526d: 488d0dc4402a02 lea rcx, [rip + 0x22a40c4]
1407e5274: ba52000000 mov edx, 0x52
1407e5279: 4531c0 xor r8d, r8d
1407e527c: e8bf460f02 call 0x1428d9940 ; core::str::slice_error_fail
1407e5281: 31c0 xor eax, eax
1407e5283: e9b9010000 jmp 0x1407e5441 ; unknown
1407e5288: 75d7 jne 0x1407e5261
1407e528a: 4c89c9 mov rcx, r9
1407e528d: e8cefaffff call 0x1407e4d60 ; core::str::impl$0::rfind<ref$<str$> >
1407e5292: 4883f801 cmp rax, 1
1407e5296: 7554 jne 0x1407e52ec
1407e5298: 4989d0 mov r8, rdx
1407e529b: 4983c002 add r8, 2
1407e529f: 7439 je 0x1407e52da
1407e52a1: 4983f852 cmp r8, 0x52
1407e52a5: 7331 jae 0x1407e52d8
1407e52a7: 488d058a402a02 lea rax, [rip + 0x22a408a]
1407e52ae: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
1407e52b3: 7f25 jg 0x1407e52da
1407e52b5: 488d056c412a02 lea rax, [rip + 0x22a416c]
1407e52bc: 4889442420 mov qword ptr [rsp + 0x20], rax
1407e52c1: 488d0d70402a02 lea rcx, [rip + 0x22a4070]
1407e52c8: ba52000000 mov edx, 0x52
1407e52cd: 41b952000000 mov r9d, 0x52
1407e52d3: e868460f02 call 0x1428d9940 ; core::str::slice_error_fail
1407e52d8: 75db jne 0x1407e52b5
1407e52da: be50000000 mov esi, 0x50
1407e52df: 4829d6 sub rsi, rdx
1407e52e2: 488d1d4f402a02 lea rbx, [rip + 0x22a404f]
1407e52e9: 4c01c3 add rbx, r8
1407e52ec: 4c8d05a8412a02 lea r8, [rip + 0x22a41a8]
1407e52f3: 4c8d355e412a02 lea r14, [rip + 0x22a415e]
1407e52fa: 660f1f440000 nop word ptr [rax + rax]
1407e5300: 410fbe40ff movsx eax, byte ptr [r8 - 1]
1407e5305: 85c0 test eax, eax
1407e5307: 7817 js 0x1407e5320
1407e5309: 49ffc8 dec r8
1407e530c: 83f85c cmp eax, 0x5c
1407e530f: 7563 jne 0x1407e5374
1407e5311: eb72 jmp 0x1407e5385 ; unknown
1407e5313: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e5320: 410fb648fe movzx ecx, byte ptr [r8 - 2]
1407e5325: 80f9c0 cmp cl, 0xc0
1407e5328: 7d23 jge 0x1407e534d
1407e532a: 410fb650fd movzx edx, byte ptr [r8 - 3]
1407e532f: 80fac0 cmp dl, 0xc0
1407e5332: 7d22 jge 0x1407e5356
1407e5334: 450fb648fc movzx r9d, byte ptr [r8 - 4]
1407e5339: 4983c0fc add r8, -4
1407e533d: 4183e107 and r9d, 7
1407e5341: 41c1e106 shl r9d, 6
1407e5345: 83e23f and edx, 0x3f
1407e5348: 4409ca or edx, r9d
1407e534b: eb10 jmp 0x1407e535d ; unknown
1407e534d: 4983c0fe add r8, -2
1407e5351: 83e11f and ecx, 0x1f
1407e5354: eb0f jmp 0x1407e5365 ; unknown
1407e5356: 4983c0fd add r8, -3
1407e535a: 83e20f and edx, 0xf
1407e535d: c1e206 shl edx, 6
1407e5360: 83e13f and ecx, 0x3f
1407e5363: 09d1 or ecx, edx
1407e5365: c1e106 shl ecx, 6
1407e5368: 243f and al, 0x3f
1407e536a: 0fb6c0 movzx eax, al
1407e536d: 09c8 or eax, ecx
1407e536f: 83f85c cmp eax, 0x5c
1407e5372: 7411 je 0x1407e5385
1407e5374: 83f82f cmp eax, 0x2f
1407e5377: 740c je 0x1407e5385
1407e5379: 4d39f0 cmp r8, r14
1407e537c: 7582 jne 0x1407e5300
1407e537e: bf43000000 mov edi, 0x43
1407e5383: eb4c jmp 0x1407e53d1 ; unknown
1407e5385: 4c8d35cc402a02 lea r14, [rip + 0x22a40cc]
1407e538c: 4d29f0 sub r8, r14
1407e538f: 49ffc0 inc r8
1407e5392: 7432 je 0x1407e53c6
1407e5394: 4983f843 cmp r8, 0x43
1407e5398: 732a jae 0x1407e53c4
1407e539a: 43803c30bf cmp byte ptr [r8 + r14], 0xbf
1407e539f: 7f25 jg 0x1407e53c6
1407e53a1: 488d0548402a02 lea rax, [rip + 0x22a4048]
1407e53a8: 4889442420 mov qword ptr [rsp + 0x20], rax
1407e53ad: 488d0da4402a02 lea rcx, [rip + 0x22a40a4]
1407e53b4: ba43000000 mov edx, 0x43
1407e53b9: 41b943000000 mov r9d, 0x43
1407e53bf: e87c450f02 call 0x1428d9940 ; core::str::slice_error_fail
1407e53c4: 75db jne 0x1407e53a1
1407e53c6: bf43000000 mov edi, 0x43
1407e53cb: 4c29c7 sub rdi, r8
1407e53ce: 4d01c6 add r14, r8
1407e53d1: e85a2d4300 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1407e53d6: 4889c1 mov rcx, rax
1407e53d9: 488b00 mov rax, qword ptr [rax]
1407e53dc: 4883f801 cmp rax, 1
1407e53e0: 0f85d61e0000 jne 0x1407e72bc
1407e53e6: 4883c108 add rcx, 8
1407e53ea: 48833900 cmp qword ptr [rcx], 0
1407e53ee: 0f85ec1e0000 jne 0x1407e72e0
1407e53f4: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1407e53fb: 48898dd8040000 mov qword ptr [rbp + 0x4d8], rcx
1407e5402: 4883c108 add rcx, 8
1407e5406: 48897c2420 mov qword ptr [rsp + 0x20], rdi
1407e540b: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
1407e5414: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
1407e541d: 4889da mov rdx, rbx
1407e5420: 4989f0 mov r8, rsi
1407e5423: 4d89f1 mov r9, r14
1407e5426: e815194300 call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
1407e542b: 488b8dd8040000 mov rcx, qword ptr [rbp + 0x4d8]
1407e5432: 48ff01 inc qword ptr [rcx]
1407e5435: 48898540030000 mov qword ptr [rbp + 0x340], rax
1407e543c: b801000000 mov eax, 1
1407e5441: 48898538030000 mov qword ptr [rbp + 0x338], rax
1407e5448: 4c89bd70040000 mov qword ptr [rbp + 0x470], r15
1407e544f: 498b37 mov rsi, qword ptr [r15]
1407e5452: 488b5608 mov rdx, qword ptr [rsi + 8]
1407e5456: 488b4e10 mov rcx, qword ptr [rsi + 0x10]
1407e545a: 4c8d0489 lea r8, [rcx + rcx*4]
1407e545e: 49c1e004 shl r8, 4
1407e5462: 4901d0 add r8, rdx
1407e5465: 48898530030000 mov qword ptr [rbp + 0x330], rax
1407e546c: 4c8d0dad3e2a02 lea r9, [rip + 0x22a3ead]
1407e5473: 488d8d08030000 lea rcx, [rbp + 0x308]
1407e547a: e891bfa700 call 0x141261410 ; alloc::vec::spec_from_iter::impl$0::from_iter<system_decorator::decorator_visual::flag_connections::CandidateInfo,core::iter::adapters::map::Map<core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo
1407e547f: 488bbd18060000 mov rdi, qword ptr [rbp + 0x618]
1407e5486: 4c8ba510060000 mov r12, qword ptr [rbp + 0x610]
1407e548d: 4c8bad18030000 mov r13, qword ptr [rbp + 0x318]
1407e5494: 4d85ed test r13, r13
1407e5497: 4c89adb0030000 mov qword ptr [rbp + 0x3b0], r13
1407e549e: 4889b5c8040000 mov qword ptr [rbp + 0x4c8], rsi
1407e54a5: 0f8496010000 je 0x1407e5641
1407e54ab: 4a8d34ad00000000 lea rsi, [r13*4]
1407e54b3: 0fb6054ff7e402 movzx eax, byte ptr [rip + 0x2e4f74f]
1407e54ba: ba04000000 mov edx, 4
1407e54bf: 4889f1 mov rcx, rsi
1407e54c2: e849e7e2ff call 0x140613c10 ; unknown
1407e54c7: 4885c0 test rax, rax
1407e54ca: 0f84681e0000 je 0x1407e7338
1407e54d0: 4c89ad40040000 mov qword ptr [rbp + 0x440], r13
1407e54d7: 48898548040000 mov qword ptr [rbp + 0x448], rax
1407e54de: 48c7855004000000000000 mov qword ptr [rbp + 0x450], 0
1407e54e9: 488bb510030000 mov rsi, qword ptr [rbp + 0x310]
1407e54f0: 4a8d04ad00000000 lea rax, [r13*4]
1407e54f8: 4c01e8 add rax, r13
1407e54fb: 48c1e004 shl rax, 4
1407e54ff: 4801f0 add rax, rsi
1407e5502: 488985b8040000 mov qword ptr [rbp + 0x4b8], rax
1407e5509: 448b6f1c mov r13d, dword ptr [rdi + 0x1c]
1407e550d: 488b1f mov rbx, qword ptr [rdi]
1407e5510: 488b7f10 mov rdi, qword ptr [rdi + 0x10]
1407e5514: 488d4320 lea rax, [rbx + 0x20]
1407e5518: 488985d8040000 mov qword ptr [rbp + 0x4d8], rax
1407e551f: 4531e4 xor r12d, r12d
1407e5522: 4889b5c0040000 mov qword ptr [rbp + 0x4c0], rsi
1407e5529: eb27 jmp 0x1407e5552 ; unknown
1407e552b: 0f1f440000 nop dword ptr [rax + rax]
1407e5530: 488b8548040000 mov rax, qword ptr [rbp + 0x448]
1407e5537: 468934a0 mov dword ptr [rax + r12*4], r14d
1407e553b: 49ffc4 inc r12
1407e553e: 4c89a550040000 mov qword ptr [rbp + 0x450], r12
1407e5545: 483bb5b8040000 cmp rsi, qword ptr [rbp + 0x4b8]
1407e554c: 0f84df000000 je 0x1407e5631
1407e5552: 4989f7 mov r15, rsi
1407e5555: 4883c650 add rsi, 0x50
1407e5559: 488b8d10060000 mov rcx, qword ptr [rbp + 0x610]
1407e5560: 488b4108 mov rax, qword ptr [rcx + 8]
1407e5564: 488b4910 mov rcx, qword ptr [rcx + 0x10]
1407e5568: 458b7740 mov r14d, dword ptr [r15 + 0x40]
1407e556c: 48c1e102 shl rcx, 2
1407e5570: 31d2 xor edx, edx
1407e5572: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e5580: 4839d1 cmp rcx, rdx
1407e5583: 741b je 0x1407e55a0
1407e5585: 4c8d4204 lea r8, [rdx + 4]
1407e5589: 443b3410 cmp r14d, dword ptr [rax + rdx]
1407e558d: 4c89c2 mov rdx, r8
1407e5590: 75ee jne 0x1407e5580
1407e5592: 4c3ba540040000 cmp r12, qword ptr [rbp + 0x440]
1407e5599: 7595 jne 0x1407e5530
1407e559b: eb7c jmp 0x1407e5619 ; unknown
1407e559d: 0f1f00 nop dword ptr [rax]
1407e55a0: 418b4718 mov eax, dword ptr [r15 + 0x18]
1407e55a4: 894538 mov dword ptr [rbp + 0x38], eax
1407e55a7: 498b4710 mov rax, qword ptr [r15 + 0x10]
1407e55ab: 48894530 mov qword ptr [rbp + 0x30], rax
1407e55af: 44892f mov dword ptr [rdi], r13d
1407e55b2: 4c8b7330 mov r14, qword ptr [rbx + 0x30]
1407e55b6: 4c8b6340 mov r12, qword ptr [rbx + 0x40]
1407e55ba: 4c3b7320 cmp r14, qword ptr [rbx + 0x20]
1407e55be: 7513 jne 0x1407e55d3
1407e55c0: 488b8dd8040000 mov rcx, qword ptr [rbp + 0x4d8]
1407e55c7: 488d15b2c94302 lea rdx, [rip + 0x243c9b2]
1407e55ce: e8ddd7d7ff call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<tuple$<ref$<str$>,enum2$<country_core::resources::gui::SettingsMenuState> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpmc::waker::Entry,alloc::alloc::Global> | 
1407e55d3: 488b4328 mov rax, qword ptr [rbx + 0x28]
1407e55d7: 4b8d0c76 lea rcx, [r14 + r14*2]
1407e55db: 4c8924c8 mov qword ptr [rax + rcx*8], r12
1407e55df: 488b5530 mov rdx, qword ptr [rbp + 0x30]
1407e55e3: 488954c808 mov qword ptr [rax + rcx*8 + 8], rdx
1407e55e8: 8b5538 mov edx, dword ptr [rbp + 0x38]
1407e55eb: 8954c810 mov dword ptr [rax + rcx*8 + 0x10], edx
1407e55ef: 66c744c8141000 mov word ptr [rax + rcx*8 + 0x14], 0x10
1407e55f6: 49ffc6 inc r14
1407e55f9: 4c897330 mov qword ptr [rbx + 0x30], r14
1407e55fd: 48ff4340 inc qword ptr [rbx + 0x40]
1407e5601: 458b7740 mov r14d, dword ptr [r15 + 0x40]
1407e5605: 4c8ba550040000 mov r12, qword ptr [rbp + 0x450]
1407e560c: 4c3ba540040000 cmp r12, qword ptr [rbp + 0x440]
1407e5613: 0f8517ffffff jne 0x1407e5530
1407e5619: 488d8d40040000 lea rcx, [rbp + 0x440]
1407e5620: 488d15a13f2a02 lea rdx, [rip + 0x22a3fa1]
1407e5627: e844a23700 call 0x140b5f870 ; alloc::raw_vec::RawVec::grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<ash::vk::bitflags::PipelineStageFlags,alloc::alloc::Global> | alloc::raw_vec
1407e562c: e9fffeffff jmp 0x1407e5530 ; unknown
1407e5631: 4c8ba510060000 mov r12, qword ptr [rbp + 0x610]
1407e5638: 4c8badb0030000 mov r13, qword ptr [rbp + 0x3b0]
1407e563f: eb48 jmp 0x1407e5689 ; unknown
1407e5641: 48c7854004000000000000 mov qword ptr [rbp + 0x440], 0
1407e564c: 48c7854804000004000000 mov qword ptr [rbp + 0x448], 4
1407e5657: 48c7855004000000000000 mov qword ptr [rbp + 0x450], 0
1407e5662: 488b8d10030000 mov rcx, qword ptr [rbp + 0x310]
1407e5669: 4a8d04ad00000000 lea rax, [r13*4]
1407e5671: 4c01e8 add rax, r13
1407e5674: 48c1e004 shl rax, 4
1407e5678: 48898dc0040000 mov qword ptr [rbp + 0x4c0], rcx
1407e567f: 4801c8 add rax, rcx
1407e5682: 488985b8040000 mov qword ptr [rbp + 0x4b8], rax
1407e5689: 488bbd28060000 mov rdi, qword ptr [rbp + 0x628]
1407e5690: 488bb500060000 mov rsi, qword ptr [rbp + 0x600]
1407e5697: 488b8550040000 mov rax, qword ptr [rbp + 0x450]
1407e569e: 48894540 mov qword ptr [rbp + 0x40], rax
1407e56a2: 488b8540040000 mov rax, qword ptr [rbp + 0x440]
1407e56a9: 48894530 mov qword ptr [rbp + 0x30], rax
1407e56ad: 488b8548040000 mov rax, qword ptr [rbp + 0x448]
1407e56b4: 48894538 mov qword ptr [rbp + 0x38], rax
1407e56b8: 498b1424 mov rdx, qword ptr [r12]
1407e56bc: 4885d2 test rdx, rdx
1407e56bf: 7414 je 0x1407e56d5
1407e56c1: 498b4c2408 mov rcx, qword ptr [r12 + 8]
1407e56c6: 48c1e202 shl rdx, 2
1407e56ca: 41b804000000 mov r8d, 4
1407e56d0: e84be5e2ff call 0x140613c20 ; unknown
1407e56d5: 488b4540 mov rax, qword ptr [rbp + 0x40]
1407e56d9: 4989442410 mov qword ptr [r12 + 0x10], rax
1407e56de: 488b4530 mov rax, qword ptr [rbp + 0x30]
1407e56e2: 49890424 mov qword ptr [r12], rax
1407e56e6: 488b4538 mov rax, qword ptr [rbp + 0x38]
1407e56ea: 4989442408 mov qword ptr [r12 + 8], rax
1407e56ef: 4c8b0f mov r9, qword ptr [rdi]
1407e56f2: 488b36 mov rsi, qword ptr [rsi]
1407e56f5: 4889742448 mov qword ptr [rsp + 0x48], rsi
1407e56fa: c744244000000000 mov dword ptr [rsp + 0x40], 0
1407e5702: c74424380000c040 mov dword ptr [rsp + 0x38], 0x40c00000
1407e570a: c744243000001041 mov dword ptr [rsp + 0x30], 0x41100000
1407e5712: c644242801 mov byte ptr [rsp + 0x28], 1
1407e5717: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1407e5720: 488d4d30 lea rcx, [rbp + 0x30]
1407e5724: 488b95c0040000 mov rdx, qword ptr [rbp + 0x4c0]
1407e572b: 4d89e8 mov r8, r13
1407e572e: e8fdbcffff call 0x1407e1430 ; system_decorator::decorator_visual::flag_connections::greedily_connect
1407e5733: 488bbd20060000 mov rdi, qword ptr [rbp + 0x620]
1407e573a: 488b8508060000 mov rax, qword ptr [rbp + 0x608]
1407e5741: 488b5d30 mov rbx, qword ptr [rbp + 0x30]
1407e5745: 488b4d38 mov rcx, qword ptr [rbp + 0x38]
1407e5749: 48898db8030000 mov qword ptr [rbp + 0x3b8], rcx
1407e5750: 4c8b7d40 mov r15, qword ptr [rbp + 0x40]
1407e5754: 4c8b7548 mov r14, qword ptr [rbp + 0x48]
1407e5758: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
1407e575c: 48898d38040000 mov qword ptr [rbp + 0x438], rcx
1407e5763: 488b4d58 mov rcx, qword ptr [rbp + 0x58]
1407e5767: 48898d48030000 mov qword ptr [rbp + 0x348], rcx
1407e576e: 488b9570040000 mov rdx, qword ptr [rbp + 0x470]
1407e5775: 8b4a1c mov ecx, dword ptr [rdx + 0x1c]
1407e5778: 488b5210 mov rdx, qword ptr [rdx + 0x10]
1407e577c: 890a mov dword ptr [rdx], ecx
1407e577e: 488985f0020000 mov qword ptr [rbp + 0x2f0], rax
1407e5785: 4889b5f8020000 mov qword ptr [rbp + 0x2f8], rsi
1407e578c: 488b85c8040000 mov rax, qword ptr [rbp + 0x4c8]
1407e5793: 48898500030000 mov qword ptr [rbp + 0x300], rax
1407e579a: 48c785a004000000000000 mov qword ptr [rbp + 0x4a0], 0
1407e57a5: 48c785a804000010000000 mov qword ptr [rbp + 0x4a8], 0x10
1407e57b0: 48c785b004000000000000 mov qword ptr [rbp + 0x4b0], 0
1407e57bb: 498d442418 lea rax, [r12 + 0x18]
1407e57c0: 488985a8030000 mov qword ptr [rbp + 0x3a8], rax
1407e57c7: 49837c243000 cmp qword ptr [r12 + 0x30], 0
1407e57cd: 7447 je 0x1407e5816
1407e57cf: 498b742420 mov rsi, qword ptr [r12 + 0x20]
1407e57d4: 4885f6 test rsi, rsi
1407e57d7: 7415 je 0x1407e57ee
1407e57d9: 488b85a8030000 mov rax, qword ptr [rbp + 0x3a8]
1407e57e0: 488b08 mov rcx, qword ptr [rax]
1407e57e3: 4c8d4611 lea r8, [rsi + 0x11]
1407e57e7: b2ff mov dl, 0xff
1407e57e9: e832e41302 call 0x142923c20 ; unknown
1407e57ee: 49c744243000000000 mov qword ptr [r12 + 0x30], 0
1407e57f7: 488d4601 lea rax, [rsi + 1]
1407e57fb: 4889c1 mov rcx, rax
1407e57fe: 48c1e903 shr rcx, 3
1407e5802: 4883e0f8 and rax, 0xfffffffffffffff8
1407e5806: 4829c8 sub rax, rcx
1407e5809: 4883fe08 cmp rsi, 8
1407e580d: 480f42c6 cmovb rax, rsi
1407e5811: 4989442428 mov qword ptr [r12 + 0x28], rax
1407e5816: 4c89f8 mov rax, r15
1407e5819: 48c1e004 shl rax, 4
1407e581d: 4c8bbdb8030000 mov r15, qword ptr [rbp + 0x3b8]
1407e5824: 4c01f8 add rax, r15
1407e5827: 48898538020000 mov qword ptr [rbp + 0x238], rax
1407e582e: 498d442448 lea rax, [r12 + 0x48]
1407e5833: 48898570020000 mov qword ptr [rbp + 0x270], rax
1407e583a: 498d442468 lea rax, [r12 + 0x68]
1407e583f: 48898540020000 mov qword ptr [rbp + 0x240], rax
1407e5846: 488b07 mov rax, qword ptr [rdi]
1407e5849: 48898560030000 mov qword ptr [rbp + 0x360], rax
1407e5850: 488b8d18060000 mov rcx, qword ptr [rbp + 0x618]
1407e5857: 8b411c mov eax, dword ptr [rcx + 0x1c]
1407e585a: 89859c030000 mov dword ptr [rbp + 0x39c], eax
1407e5860: 4c8b21 mov r12, qword ptr [rcx]
1407e5863: 488b4110 mov rax, qword ptr [rcx + 0x10]
1407e5867: 48898548020000 mov qword ptr [rbp + 0x248], rax
1407e586e: 498d442420 lea rax, [r12 + 0x20]
1407e5873: 48898550020000 mov qword ptr [rbp + 0x250], rax
1407e587a: 48899d50030000 mov qword ptr [rbp + 0x350], rbx
1407e5881: 4c89b558030000 mov qword ptr [rbp + 0x358], r14
1407e5888: e990000000 jmp 0x1407e591d ; unknown
1407e588d: 0f1f00 nop dword ptr [rax]
1407e5890: 4983c710 add r15, 0x10
1407e5894: 01fe add esi, edi
1407e5896: 4401f6 add esi, r14d
1407e5899: 4401ee add esi, r13d
1407e589c: 488b85a8040000 mov rax, qword ptr [rbp + 0x4a8]
1407e58a3: 486bcb70 imul rcx, rbx, 0x70
1407e58a7: 0f284520 movaps xmm0, xmmword ptr [rbp + 0x20]
1407e58ab: 0f290408 movaps xmmword ptr [rax + rcx], xmm0
1407e58af: 0f137c0810 movlps qword ptr [rax + rcx + 0x10], xmm7
1407e58b4: f3440f11740818 movss dword ptr [rax + rcx + 0x18], xmm14
1407e58bb: 0f288510020000 movaps xmm0, xmmword ptr [rbp + 0x210]
1407e58c2: 0f1144081c movups xmmword ptr [rax + rcx + 0x1c], xmm0
1407e58c7: 8b9520020000 mov edx, dword ptr [rbp + 0x220]
1407e58cd: 8954082c mov dword ptr [rax + rcx + 0x2c], edx
1407e58d1: 0f284510 movaps xmm0, xmmword ptr [rbp + 0x10]
1407e58d5: 0f29440830 movaps xmmword ptr [rax + rcx + 0x30], xmm0
1407e58da: 0f13740840 movlps qword ptr [rax + rcx + 0x40], xmm6
1407e58df: f3440f11540848 movss dword ptr [rax + rcx + 0x48], xmm10
1407e58e6: 660f6f85f0010000 movdqa xmm0, xmmword ptr [rbp + 0x1f0]
1407e58ee: f30f7f44084c movdqu xmmword ptr [rax + rcx + 0x4c], xmm0
1407e58f4: 8b9500020000 mov edx, dword ptr [rbp + 0x200]
1407e58fa: 8954085c mov dword ptr [rax + rcx + 0x5c], edx
1407e58fe: 4489740860 mov dword ptr [rax + rcx + 0x60], r14d
1407e5903: 44896c0864 mov dword ptr [rax + rcx + 0x64], r13d
1407e5908: 89740868 mov dword ptr [rax + rcx + 0x68], esi
1407e590c: 48ffc3 inc rbx
1407e590f: 48899db0040000 mov qword ptr [rbp + 0x4b0], rbx
1407e5916: 4c8badb0030000 mov r13, qword ptr [rbp + 0x3b0]
1407e591d: 4c3bbd38020000 cmp r15, qword ptr [rbp + 0x238]
1407e5924: 0f84330a0000 je 0x1407e635d
1407e592a: 498b0f mov rcx, qword ptr [r15]
1407e592d: 4c39e9 cmp rcx, r13
1407e5930: 0f83d5190000 jae 0x1407e730b
1407e5936: 498b4708 mov rax, qword ptr [r15 + 8]
1407e593a: 4c39e8 cmp rax, r13
1407e593d: 4c8b8dc0040000 mov r9, qword ptr [rbp + 0x4c0]
1407e5944: 0f83a2190000 jae 0x1407e72ec
1407e594a: 4c8d2c89 lea r13, [rcx + rcx*4]
1407e594e: 49c1e504 shl r13, 4
1407e5952: 4f8d3429 lea r14, [r9 + r13]
1407e5956: 488d3c80 lea rdi, [rax + rax*4]
1407e595a: 48c1e704 shl rdi, 4
1407e595e: f2450f104610 movsd xmm8, qword ptr [r14 + 0x10]
1407e5964: f3410f107e18 movss xmm7, dword ptr [r14 + 0x18]
1407e596a: 458b4640 mov r8d, dword ptr [r14 + 0x40]
1407e596e: 418b443940 mov eax, dword ptr [r9 + rdi + 0x40]
1407e5973: 4439c0 cmp eax, r8d
1407e5976: 4489c2 mov edx, r8d
1407e5979: 0f42d0 cmovb edx, eax
1407e597c: f2450f10643910 movsd xmm12, qword ptr [r9 + rdi + 0x10]
1407e5983: 440f47c0 cmova r8d, eax
1407e5987: f3450f106c3918 movss xmm13, dword ptr [r9 + rdi + 0x18]
1407e598e: 8995a0030000 mov dword ptr [rbp + 0x3a0], edx
1407e5994: 448985a4030000 mov dword ptr [rbp + 0x3a4], r8d
1407e599b: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e59a2: 488b8da8030000 mov rcx, qword ptr [rbp + 0x3a8]
1407e59a9: e8922caeff call 0x1402c8640 ; hashbrown::map::HashMap::insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global> | hashbrown::map::HashMap::insert<country_core::resources::stair
1407e59ae: 488b8510060000 mov rax, qword ptr [rbp + 0x610]
1407e59b5: 4883786000 cmp qword ptr [rax + 0x60], 0
1407e59ba: 0f84e0000000 je 0x1407e5aa0
1407e59c0: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e59c7: 488b8d40020000 mov rcx, qword ptr [rbp + 0x240]
1407e59ce: 488d95a0030000 lea rdx, [rbp + 0x3a0]
1407e59d5: e806c2dcff call 0x1405b1be0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core:
1407e59da: 488b9510060000 mov rdx, qword ptr [rbp + 0x610]
1407e59e1: 488b4a48 mov rcx, qword ptr [rdx + 0x48]
1407e59e5: 488b5250 mov rdx, qword ptr [rdx + 0x50]
1407e59e9: 4989d0 mov r8, rdx
1407e59ec: 4921c0 and r8, rax
1407e59ef: 48c1e839 shr rax, 0x39
1407e59f3: 660f6ec0 movd xmm0, eax
1407e59f7: 660f60c0 punpcklbw xmm0, xmm0
1407e59fb: f20f70c000 pshuflw xmm0, xmm0, 0
1407e5a00: 660f70c000 pshufd xmm0, xmm0, 0
1407e5a05: f30f7e8da0030000 movq xmm1, qword ptr [rbp + 0x3a0]
1407e5a0d: 31c0 xor eax, eax
1407e5a0f: f3420f6f1401 movdqu xmm2, xmmword ptr [rcx + r8]
1407e5a15: 660f6fda movdqa xmm3, xmm2
1407e5a19: 660f74d8 pcmpeqb xmm3, xmm0
1407e5a1d: 66440fd7cb pmovmskb r9d, xmm3
1407e5a22: 4585c9 test r9d, r9d
1407e5a25: 7449 je 0x1407e5a70
1407e5a27: f3450fbcd1 tzcnt r10d, r9d
1407e5a2c: 4d01c2 add r10, r8
1407e5a2f: 4921d2 and r10, rdx
1407e5a32: 49c1e203 shl r10, 3
1407e5a36: 4989cb mov r11, rcx
1407e5a39: 4d29d3 sub r11, r10
1407e5a3c: f3410f7e5bf8 movq xmm3, qword ptr [r11 - 8]
1407e5a42: 660f76d9 pcmpeqd xmm3, xmm1
1407e5a46: 660f70db50 pshufd xmm3, xmm3, 0x50
1407e5a4b: 66440f50d3 movmskpd r10d, xmm3
1407e5a50: 4183fa03 cmp r10d, 3
1407e5a54: 0f84df000000 je 0x1407e5b39
1407e5a5a: 458d51ff lea r10d, [r9 - 1]
1407e5a5e: 664521ca and r10w, r9w
1407e5a62: 4589d1 mov r9d, r10d
1407e5a65: 75c0 jne 0x1407e5a27
1407e5a67: 660f1f840000000000 nop word ptr [rax + rax]
1407e5a70: 660f741518ff1302 pcmpeqb xmm2, xmmword ptr [rip + 0x213ff18]
1407e5a78: 66440fd7ca pmovmskb r9d, xmm2
1407e5a7d: 4585c9 test r9d, r9d
1407e5a80: 751e jne 0x1407e5aa0
1407e5a82: 4901c0 add r8, rax
1407e5a85: 4983c010 add r8, 0x10
1407e5a89: 4883c010 add rax, 0x10
1407e5a8d: 4921d0 and r8, rdx
1407e5a90: e97affffff jmp 0x1407e5a0f ; unknown
1407e5a95: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e5aa0: 8b859c030000 mov eax, dword ptr [rbp + 0x39c]
1407e5aa6: 488b8d48020000 mov rcx, qword ptr [rbp + 0x248]
1407e5aad: 8901 mov dword ptr [rcx], eax
1407e5aaf: 498b742430 mov rsi, qword ptr [r12 + 0x30]
1407e5ab4: 498b5c2440 mov rbx, qword ptr [r12 + 0x40]
1407e5ab9: 493b742420 cmp rsi, qword ptr [r12 + 0x20]
1407e5abe: 751a jne 0x1407e5ada
1407e5ac0: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e5ac7: 488b8d50020000 mov rcx, qword ptr [rbp + 0x250]
1407e5ace: 488d15abc44302 lea rdx, [rip + 0x243c4ab]
1407e5ad5: e8d6d2d7ff call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<tuple$<ref$<str$>,enum2$<country_core::resources::gui::SettingsMenuState> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<std::sync::mpmc::waker::Entry,alloc::alloc::Global> | 
1407e5ada: 0f28c7 movaps xmm0, xmm7
1407e5add: f30f101d8bfd1302 movss xmm3, dword ptr [rip + 0x213fd8b]
1407e5ae5: f30f59c3 mulss xmm0, xmm3
1407e5ae9: 410f28c8 movaps xmm1, xmm8
1407e5aed: 0f2825ecfd1302 movaps xmm4, xmmword ptr [rip + 0x213fdec]
1407e5af4: 0f59cc mulps xmm1, xmm4
1407e5af7: 410f28d4 movaps xmm2, xmm12
1407e5afb: 0f59d4 mulps xmm2, xmm4
1407e5afe: 0f58d1 addps xmm2, xmm1
1407e5b01: 410f28cd movaps xmm1, xmm13
1407e5b05: f30f59cb mulss xmm1, xmm3
1407e5b09: f30f58c8 addss xmm1, xmm0
1407e5b0d: 498b442428 mov rax, qword ptr [r12 + 0x28]
1407e5b12: 488d0c76 lea rcx, [rsi + rsi*2]
1407e5b16: 48891cc8 mov qword ptr [rax + rcx*8], rbx
1407e5b1a: 0f1354c808 movlps qword ptr [rax + rcx*8 + 8], xmm2
1407e5b1f: f30f114cc810 movss dword ptr [rax + rcx*8 + 0x10], xmm1
1407e5b25: 66c744c8141000 mov word ptr [rax + rcx*8 + 0x14], 0x10
1407e5b2c: 48ffc6 inc rsi
1407e5b2f: 4989742430 mov qword ptr [r12 + 0x30], rsi
1407e5b34: 49ff442440 inc qword ptr [r12 + 0x40]
1407e5b39: 488b85c0040000 mov rax, qword ptr [rbp + 0x4c0]
1407e5b40: 488d1c38 lea rbx, [rax + rdi]
1407e5b44: f30f11bdd8040000 movss dword ptr [rbp + 0x4d8], xmm7
1407e5b4c: 450f281e movaps xmm11, xmmword ptr [r14]
1407e5b50: 450f28fb movaps xmm15, xmm11
1407e5b54: 450f59fb mulps xmm15, xmm11
1407e5b58: f3410f16c7 movshdup xmm0, xmm15
1407e5b5d: f3410f58c7 addss xmm0, xmm15
1407e5b62: 410f28cf movaps xmm1, xmm15
1407e5b66: 66410f15cf unpckhpd xmm1, xmm15
1407e5b6b: f30f58c8 addss xmm1, xmm0
1407e5b6f: 0fc6c900 shufps xmm1, xmm1, 0
1407e5b73: 450fc6ffff shufps xmm15, xmm15, 0xff
1407e5b78: 440f5cf9 subps xmm15, xmm1
1407e5b7c: 410f28f3 movaps xmm6, xmm11
1407e5b80: 410fc6f3d2 shufps xmm6, xmm11, 0xd2
1407e5b85: 410f28e3 movaps xmm4, xmm11
1407e5b89: 410f58e3 addps xmm4, xmm11
1407e5b8d: 0fc6e4ff shufps xmm4, xmm4, 0xff
1407e5b91: 0f2803 movaps xmm0, xmmword ptr [rbx]
1407e5b94: 0f28c8 movaps xmm1, xmm0
1407e5b97: 0f59c8 mulps xmm1, xmm0
1407e5b9a: f30f16d1 movshdup xmm2, xmm1
1407e5b9e: f30f58d1 addss xmm2, xmm1
1407e5ba2: 0f28d9 movaps xmm3, xmm1
1407e5ba5: 660f15d9 unpckhpd xmm3, xmm1
1407e5ba9: f30f58da addss xmm3, xmm2
1407e5bad: 0fc6db00 shufps xmm3, xmm3, 0
1407e5bb1: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407e5bb5: 0f5ccb subps xmm1, xmm3
1407e5bb8: 0f28e8 movaps xmm5, xmm0
1407e5bbb: 0fc6e8d2 shufps xmm5, xmm0, 0xd2
1407e5bbf: 440f28d0 movaps xmm10, xmm0
1407e5bc3: 440f58d0 addps xmm10, xmm0
1407e5bc7: 450fc6d2ff shufps xmm10, xmm10, 0xff
1407e5bcc: f3440f104c3838 movss xmm9, dword ptr [rax + rdi + 0x38]
1407e5bd3: f3410f12d1 movsldup xmm2, xmm9
1407e5bd8: 0f299580030000 movaps xmmword ptr [rbp + 0x380], xmm2
1407e5bdf: f3420f107c2838 movss xmm7, dword ptr [rax + r13 + 0x38]
1407e5be6: f30f12df movsldup xmm3, xmm7
1407e5bea: 0fc6ff80 shufps xmm7, xmm7, 0x80
1407e5bee: 0f2815ab341c02 movaps xmm2, xmmword ptr [rip + 0x21c34ab]
1407e5bf5: 0f59fa mulps xmm7, xmm2
1407e5bf8: 440f2835b0352a02 movaps xmm14, xmmword ptr [rip + 0x22a35b0]
1407e5c00: 410f59fe mulps xmm7, xmm14
1407e5c04: 0f28d7 movaps xmm2, xmm7
1407e5c07: 0fc6d764 shufps xmm2, xmm7, 0x64
1407e5c0b: 440f29bd70030000 movaps xmmword ptr [rbp + 0x370], xmm15
1407e5c13: 410f59d7 mulps xmm2, xmm15
1407e5c17: 440f29a5c0030000 movaps xmmword ptr [rbp + 0x3c0], xmm12
1407e5c1f: 450f28e3 movaps xmm12, xmm11
1407e5c23: 440f59e7 mulps xmm12, xmm7
1407e5c27: 450f28fd movaps xmm15, xmm13
1407e5c2b: f3450f16ec movshdup xmm13, xmm12
1407e5c30: f3450f58ec addss xmm13, xmm12
1407e5c35: 450f12e4 movhlps xmm12, xmm12
1407e5c39: f3450f58e5 addss xmm12, xmm13
1407e5c3e: f3450f58e4 addss xmm12, xmm12
1407e5c43: 450fc6e400 shufps xmm12, xmm12, 0
1407e5c48: 450f59e3 mulps xmm12, xmm11
1407e5c4c: 440f58e2 addps xmm12, xmm2
1407e5c50: 0f29b510040000 movaps xmmword ptr [rbp + 0x410], xmm6
1407e5c57: 0f28d6 movaps xmm2, xmm6
1407e5c5a: 0f59d7 mulps xmm2, xmm7
1407e5c5d: 0fc6ffd2 shufps xmm7, xmm7, 0xd2
1407e5c61: 410f59fb mulps xmm7, xmm11
1407e5c65: 0f5cd7 subps xmm2, xmm7
1407e5c68: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1407e5c6c: 0f59d4 mulps xmm2, xmm4
1407e5c6f: 410f58d4 addps xmm2, xmm12
1407e5c73: 440f298570040000 movaps xmmword ptr [rbp + 0x470], xmm8
1407e5c7b: 410f28f8 movaps xmm7, xmm8
1407e5c7f: 0f58fa addps xmm7, xmm2
1407e5c82: 0f12d2 movhlps xmm2, xmm2
1407e5c85: f30f5895d8040000 addss xmm2, dword ptr [rbp + 0x4d8]
1407e5c8d: 450fc6c980 shufps xmm9, xmm9, 0x80
1407e5c92: 440f590d06341c02 mulps xmm9, xmmword ptr [rip + 0x21c3406]
1407e5c9a: 450f59ce mulps xmm9, xmm14
1407e5c9e: 450f28e1 movaps xmm12, xmm9
1407e5ca2: 450fc6e164 shufps xmm12, xmm9, 0x64
1407e5ca7: 0f298de0020000 movaps xmmword ptr [rbp + 0x2e0], xmm1
1407e5cae: 440f59e1 mulps xmm12, xmm1
1407e5cb2: 440f28e8 movaps xmm13, xmm0
1407e5cb6: 450f59e9 mulps xmm13, xmm9
1407e5cba: f3450f16f5 movshdup xmm14, xmm13
1407e5cbf: f3450f58f5 addss xmm14, xmm13
1407e5cc4: 450f12ed movhlps xmm13, xmm13
1407e5cc8: f3450f58ee addss xmm13, xmm14
1407e5ccd: f3450f58ed addss xmm13, xmm13
1407e5cd2: 450fc6ed00 shufps xmm13, xmm13, 0
1407e5cd7: 440f59e8 mulps xmm13, xmm0
1407e5cdb: 450f58ec addps xmm13, xmm12
1407e5cdf: 0f29ad80040000 movaps xmmword ptr [rbp + 0x480], xmm5
1407e5ce6: 440f28e5 movaps xmm12, xmm5
1407e5cea: 450f59e1 mulps xmm12, xmm9
1407e5cee: 450fc6c9d2 shufps xmm9, xmm9, 0xd2
1407e5cf3: 0f2985e0030000 movaps xmmword ptr [rbp + 0x3e0], xmm0
1407e5cfa: 440f59c8 mulps xmm9, xmm0
1407e5cfe: 0f2885c0030000 movaps xmm0, xmmword ptr [rbp + 0x3c0]
1407e5d05: 450f5ce1 subps xmm12, xmm9
1407e5d09: 450fc6e4d2 shufps xmm12, xmm12, 0xd2
1407e5d0e: 440f2995d0020000 movaps xmmword ptr [rbp + 0x2d0], xmm10
1407e5d16: 450f59e2 mulps xmm12, xmm10
1407e5d1a: 450f58e5 addps xmm12, xmm13
1407e5d1e: 440f28c8 movaps xmm9, xmm0
1407e5d22: 450f58cc addps xmm9, xmm12
1407e5d26: 450f12e4 movhlps xmm12, xmm12
1407e5d2a: f3440f11bdc8040000 movss dword ptr [rbp + 0x4c8], xmm15
1407e5d33: f3450f58e7 addss xmm12, xmm15
1407e5d38: 440f28ef movaps xmm13, xmm7
1407e5d3c: 450f5ce9 subps xmm13, xmm9
1407e5d40: 450f59ed mulps xmm13, xmm13
1407e5d44: f3450f16f5 movshdup xmm14, xmm13
1407e5d49: f3450f58f5 addss xmm14, xmm13
1407e5d4e: 440f28ea movaps xmm13, xmm2
1407e5d52: f3450f5cec subss xmm13, xmm12
1407e5d57: f3450f59ed mulss xmm13, xmm13
1407e5d5c: f3450f58ee addss xmm13, xmm14
1407e5d61: 4d8b4630 mov r8, qword ptr [r14 + 0x30]
1407e5d65: 488b4330 mov rax, qword ptr [rbx + 0x30]
1407e5d69: f3450f51ed sqrtss xmm13, xmm13
1407e5d6e: 4939c0 cmp r8, rax
1407e5d71: 0f85b9000000 jne 0x1407e5e30
1407e5d77: 0f57c9 xorps xmm1, xmm1
1407e5d7a: 450f57f6 xorps xmm14, xmm14
1407e5d7e: f3440f11b5d0040000 movss dword ptr [rbp + 0x4d0], xmm14
1407e5d87: 440f2ee9 ucomiss xmm13, xmm1
1407e5d8b: f3440f10bdc8040000 movss xmm15, dword ptr [rbp + 0x4c8]
1407e5d94: f30f108dd8040000 movss xmm1, dword ptr [rbp + 0x4d8]
1407e5d9c: f30f118df0030000 movss dword ptr [rbp + 0x3f0], xmm1
1407e5da4: f3440f103563b62902 movss xmm14, dword ptr [rip + 0x229b663]
1407e5dad: f3440f11b5d0030000 movss dword ptr [rbp + 0x3d0], xmm14
1407e5db6: 0f288d70040000 movaps xmm1, xmmword ptr [rbp + 0x470]
1407e5dbd: 0f298d00040000 movaps xmmword ptr [rbp + 0x400], xmm1
1407e5dc4: 0f298520040000 movaps xmmword ptr [rbp + 0x420], xmm0
1407e5dcb: 0f288d80040000 movaps xmm1, xmmword ptr [rbp + 0x480]
1407e5dd2: 440f2895e0030000 movaps xmm10, xmmword ptr [rbp + 0x3e0]
1407e5dda: 440f288510040000 movaps xmm8, xmmword ptr [rbp + 0x410]
1407e5de2: 0f28ad70030000 movaps xmm5, xmmword ptr [rbp + 0x370]
1407e5de9: 0f28b580030000 movaps xmm6, xmmword ptr [rbp + 0x380]
1407e5df0: 0f86ed000000 jbe 0x1407e5ee3
1407e5df6: 450f28fc movaps xmm15, xmm12
1407e5dfa: f30f1195f0030000 movss dword ptr [rbp + 0x3f0], xmm2
1407e5e02: f3440f11add0040000 movss dword ptr [rbp + 0x4d0], xmm13
1407e5e0b: f3440f11b5d0030000 movss dword ptr [rbp + 0x3d0], xmm14
1407e5e14: 0f29bd00040000 movaps xmmword ptr [rbp + 0x400], xmm7
1407e5e1b: 440f298d20040000 movaps xmmword ptr [rbp + 0x420], xmm9
1407e5e23: e9bb000000 jmp 0x1407e5ee3 ; unknown
1407e5e28: 0f1f840000000000 nop dword ptr [rax + rax]
1407e5e30: 450f57f6 xorps xmm14, xmm14
1407e5e34: f3440f11b5d0040000 movss dword ptr [rbp + 0x4d0], xmm14
1407e5e3d: f3440f1035cab52902 movss xmm14, dword ptr [rip + 0x229b5ca]
1407e5e46: 450f2ef5 ucomiss xmm14, xmm13
1407e5e4a: 7634 jbe 0x1407e5e80
1407e5e4c: 450f28fc movaps xmm15, xmm12
1407e5e50: f30f1195f0030000 movss dword ptr [rbp + 0x3f0], xmm2
1407e5e58: f3440f11add0030000 movss dword ptr [rbp + 0x3d0], xmm13
1407e5e61: 0f29bd00040000 movaps xmmword ptr [rbp + 0x400], xmm7
1407e5e68: 440f298d20040000 movaps xmmword ptr [rbp + 0x420], xmm9
1407e5e70: 0f2885c0030000 movaps xmm0, xmmword ptr [rbp + 0x3c0]
1407e5e77: eb45 jmp 0x1407e5ebe ; unknown
1407e5e79: 0f1f8000000000 nop dword ptr [rax]
1407e5e80: f3440f10bdc8040000 movss xmm15, dword ptr [rbp + 0x4c8]
1407e5e89: f30f1085d8040000 movss xmm0, dword ptr [rbp + 0x4d8]
1407e5e91: f30f1185f0030000 movss dword ptr [rbp + 0x3f0], xmm0
1407e5e99: f3440f11b5d0030000 movss dword ptr [rbp + 0x3d0], xmm14
1407e5ea2: 0f288570040000 movaps xmm0, xmmword ptr [rbp + 0x470]
1407e5ea9: 0f298500040000 movaps xmmword ptr [rbp + 0x400], xmm0
1407e5eb0: 0f2885c0030000 movaps xmm0, xmmword ptr [rbp + 0x3c0]
1407e5eb7: 0f298520040000 movaps xmmword ptr [rbp + 0x420], xmm0
1407e5ebe: 0f288d80040000 movaps xmm1, xmmword ptr [rbp + 0x480]
1407e5ec5: 440f2895e0030000 movaps xmm10, xmmword ptr [rbp + 0x3e0]
1407e5ecd: 440f288510040000 movaps xmm8, xmmword ptr [rbp + 0x410]
1407e5ed5: 0f28ad70030000 movaps xmm5, xmmword ptr [rbp + 0x370]
1407e5edc: 0f28b580030000 movaps xmm6, xmmword ptr [rbp + 0x380]
1407e5ee3: f3440f11bd10040000 movss dword ptr [rbp + 0x410], xmm15
1407e5eec: 0f28d3 movaps xmm2, xmm3
1407e5eef: 0fc6d3d4 shufps xmm2, xmm3, 0xd4
1407e5ef3: 0f5915a6311c02 mulps xmm2, xmmword ptr [rip + 0x21c31a6]
1407e5efa: 440f283dae322a02 movaps xmm15, xmmword ptr [rip + 0x22a32ae]
1407e5f02: 410f59d7 mulps xmm2, xmm15
1407e5f06: 0f28fa movaps xmm7, xmm2
1407e5f09: 0fc6fa64 shufps xmm7, xmm2, 0x64
1407e5f0d: 0f59fd mulps xmm7, xmm5
1407e5f10: 450f28cb movaps xmm9, xmm11
1407e5f14: 440f59ca mulps xmm9, xmm2
1407e5f18: f3450f16e1 movshdup xmm12, xmm9
1407e5f1d: f3450f58e1 addss xmm12, xmm9
1407e5f22: 450f12c9 movhlps xmm9, xmm9
1407e5f26: f3450f58cc addss xmm9, xmm12
1407e5f2b: f3450f58c9 addss xmm9, xmm9
1407e5f30: 450fc6c900 shufps xmm9, xmm9, 0
1407e5f35: 450f59cb mulps xmm9, xmm11
1407e5f39: 440f58cf addps xmm9, xmm7
1407e5f3d: 450f28f0 movaps xmm14, xmm8
1407e5f41: 440f59f2 mulps xmm14, xmm2
1407e5f45: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1407e5f49: 410f59d3 mulps xmm2, xmm11
1407e5f4d: 440f5cf2 subps xmm14, xmm2
1407e5f51: 450fc6f6d2 shufps xmm14, xmm14, 0xd2
1407e5f56: 440f59f4 mulps xmm14, xmm4
1407e5f5a: 450f58f1 addps xmm14, xmm9
1407e5f5e: 440f28ad70040000 movaps xmm13, xmmword ptr [rbp + 0x470]
1407e5f66: 450f58ee addps xmm13, xmm14
1407e5f6a: 450f12f6 movhlps xmm14, xmm14
1407e5f6e: f3440f58b5d8040000 addss xmm14, dword ptr [rbp + 0x4d8]
1407e5f77: 0f28fe movaps xmm7, xmm6
1407e5f7a: f30f101582f91302 movss xmm2, dword ptr [rip + 0x213f982]
1407e5f82: 0f59fa mulps xmm7, xmm2
1407e5f85: 0fc6ffd4 shufps xmm7, xmm7, 0xd4
1407e5f89: 410f59ff mulps xmm7, xmm15
1407e5f8d: 0f28d7 movaps xmm2, xmm7
1407e5f90: 0fc6d764 shufps xmm2, xmm7, 0x64
1407e5f94: 0f5995e0020000 mulps xmm2, xmmword ptr [rbp + 0x2e0]
1407e5f9b: 450f28ca movaps xmm9, xmm10
1407e5f9f: 440f59cf mulps xmm9, xmm7
1407e5fa3: f3450f16e1 movshdup xmm12, xmm9
1407e5fa8: f3450f58e1 addss xmm12, xmm9
1407e5fad: 450f12c9 movhlps xmm9, xmm9
1407e5fb1: f3450f58cc addss xmm9, xmm12
1407e5fb6: f3450f58c9 addss xmm9, xmm9
1407e5fbb: 450fc6c900 shufps xmm9, xmm9, 0
1407e5fc0: 450f59ca mulps xmm9, xmm10
1407e5fc4: 440f58ca addps xmm9, xmm2
1407e5fc8: 0f28d1 movaps xmm2, xmm1
1407e5fcb: 0f59d7 mulps xmm2, xmm7
1407e5fce: 0fc6ffd2 shufps xmm7, xmm7, 0xd2
1407e5fd2: 410f59fa mulps xmm7, xmm10
1407e5fd6: 0f5cd7 subps xmm2, xmm7
1407e5fd9: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1407e5fdd: 0f5995d0020000 mulps xmm2, xmmword ptr [rbp + 0x2d0]
1407e5fe4: 410f58d1 addps xmm2, xmm9
1407e5fe8: 440f28c8 movaps xmm9, xmm0
1407e5fec: 440f58ca addps xmm9, xmm2
1407e5ff0: 440f28e2 movaps xmm12, xmm2
1407e5ff4: 66440f15e2 unpckhpd xmm12, xmm2
1407e5ff9: f30f10bdc8040000 movss xmm7, dword ptr [rbp + 0x4c8]
1407e6001: 440f29a5e0030000 movaps xmmword ptr [rbp + 0x3e0], xmm12
1407e6009: f3410f58fc addss xmm7, xmm12
1407e600e: 440f29ad80040000 movaps xmmword ptr [rbp + 0x480], xmm13
1407e6016: 450f5ce9 subps xmm13, xmm9
1407e601a: 450f59ed mulps xmm13, xmm13
1407e601e: f3450f16e5 movshdup xmm12, xmm13
1407e6023: f3450f58e5 addss xmm12, xmm13
1407e6028: 450f28ee movaps xmm13, xmm14
1407e602c: f3440f5cef subss xmm13, xmm7
1407e6031: f3450f59ed mulss xmm13, xmm13
1407e6036: f3450f58ec addss xmm13, xmm12
1407e603b: f3450f51fd sqrtss xmm15, xmm13
1407e6040: 4939c0 cmp r8, rax
1407e6043: 752b jne 0x1407e6070
1407e6045: 440f2ebdd0040000 ucomiss xmm15, dword ptr [rbp + 0x4d0]
1407e604d: f30f1085d0030000 movss xmm0, dword ptr [rbp + 0x3d0]
1407e6055: 7627 jbe 0x1407e607e
1407e6057: f3440f11bdd0040000 movss dword ptr [rbp + 0x4d0], xmm15
1407e6060: 440f28f8 movaps xmm15, xmm0
1407e6064: eb43 jmp 0x1407e60a9 ; unknown
1407e6066: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e6070: f30f1085d0030000 movss xmm0, dword ptr [rbp + 0x3d0]
1407e6078: 410f2ec7 ucomiss xmm0, xmm15
1407e607c: 772b ja 0x1407e60a9
1407e607e: f30f10bd10040000 movss xmm7, dword ptr [rbp + 0x410]
1407e6086: f3440f10b5f0030000 movss xmm14, dword ptr [rbp + 0x3f0]
1407e608f: 440f28f8 movaps xmm15, xmm0
1407e6093: 0f288500040000 movaps xmm0, xmmword ptr [rbp + 0x400]
1407e609a: 0f298580040000 movaps xmmword ptr [rbp + 0x480], xmm0
1407e60a1: 440f288d20040000 movaps xmm9, xmmword ptr [rbp + 0x420]
1407e60a9: f3440f102552f81302 movss xmm12, dword ptr [rip + 0x213f852]
1407e60b2: 410f59dc mulps xmm3, xmm12
1407e60b6: 0fc6dbd4 shufps xmm3, xmm3, 0xd4
1407e60ba: 0f2805ef302a02 movaps xmm0, xmmword ptr [rip + 0x22a30ef]
1407e60c1: 0f59d8 mulps xmm3, xmm0
1407e60c4: 440f28e3 movaps xmm12, xmm3
1407e60c8: 440fc6e364 shufps xmm12, xmm3, 0x64
1407e60cd: 410f59ec mulps xmm5, xmm12
1407e60d1: 450f28e3 movaps xmm12, xmm11
1407e60d5: 440f59e3 mulps xmm12, xmm3
1407e60d9: f3450f16ec movshdup xmm13, xmm12
1407e60de: f3450f58ec addss xmm13, xmm12
1407e60e3: 450f12e4 movhlps xmm12, xmm12
1407e60e7: f3450f58e5 addss xmm12, xmm13
1407e60ec: f3450f58e4 addss xmm12, xmm12
1407e60f1: 450fc6e400 shufps xmm12, xmm12, 0
1407e60f6: 450f59e3 mulps xmm12, xmm11
1407e60fa: 440f58e5 addps xmm12, xmm5
1407e60fe: 440f59c3 mulps xmm8, xmm3
1407e6102: 0fc6dbd2 shufps xmm3, xmm3, 0xd2
1407e6106: 410f59db mulps xmm3, xmm11
1407e610a: 440f5cc3 subps xmm8, xmm3
1407e610e: 450fc6c0d2 shufps xmm8, xmm8, 0xd2
1407e6113: 410f59e0 mulps xmm4, xmm8
1407e6117: 410f58e4 addps xmm4, xmm12
1407e611b: 440f288570040000 movaps xmm8, xmmword ptr [rbp + 0x470]
1407e6123: 440f58c4 addps xmm8, xmm4
1407e6127: 0f12e4 movhlps xmm4, xmm4
1407e612a: f3440f109dd8040000 movss xmm11, dword ptr [rbp + 0x4d8]
1407e6133: f3440f58dc addss xmm11, xmm4
1407e6138: 0fc6f6d4 shufps xmm6, xmm6, 0xd4
1407e613c: 0f59355d2f1c02 mulps xmm6, xmmword ptr [rip + 0x21c2f5d]
1407e6143: 0f59f0 mulps xmm6, xmm0
1407e6146: 0f28e6 movaps xmm4, xmm6
1407e6149: 0fc6e664 shufps xmm4, xmm6, 0x64
1407e614d: 0f2885e0020000 movaps xmm0, xmmword ptr [rbp + 0x2e0]
1407e6154: 0f59c4 mulps xmm0, xmm4
1407e6157: 410f28e2 movaps xmm4, xmm10
1407e615b: 0f59e6 mulps xmm4, xmm6
1407e615e: f30f16ec movshdup xmm5, xmm4
1407e6162: f30f58ec addss xmm5, xmm4
1407e6166: 0f12e4 movhlps xmm4, xmm4
1407e6169: f30f58e5 addss xmm4, xmm5
1407e616d: f30f58e4 addss xmm4, xmm4
1407e6171: 0fc6e400 shufps xmm4, xmm4, 0
1407e6175: 410f59e2 mulps xmm4, xmm10
1407e6179: 0f58e0 addps xmm4, xmm0
1407e617c: 0f59ce mulps xmm1, xmm6
1407e617f: 0fc6f6d2 shufps xmm6, xmm6, 0xd2
1407e6183: 410f59f2 mulps xmm6, xmm10
1407e6187: 0f5cce subps xmm1, xmm6
1407e618a: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
1407e618e: 440f2895d0020000 movaps xmm10, xmmword ptr [rbp + 0x2d0]
1407e6196: 440f59d1 mulps xmm10, xmm1
1407e619a: 440f58d4 addps xmm10, xmm4
1407e619e: 0f28a5c0030000 movaps xmm4, xmmword ptr [rbp + 0x3c0]
1407e61a5: 0f28f4 movaps xmm6, xmm4
1407e61a8: 410f58f2 addps xmm6, xmm10
1407e61ac: 450f12d2 movhlps xmm10, xmm10
1407e61b0: f30f10adc8040000 movss xmm5, dword ptr [rbp + 0x4c8]
1407e61b8: f3440f58d5 addss xmm10, xmm5
1407e61bd: 410f28c0 movaps xmm0, xmm8
1407e61c1: 0f5cc6 subps xmm0, xmm6
1407e61c4: 0f59c0 mulps xmm0, xmm0
1407e61c7: f30f16c8 movshdup xmm1, xmm0
1407e61cb: f30f58c8 addss xmm1, xmm0
1407e61cf: 410f28c3 movaps xmm0, xmm11
1407e61d3: f3410f5cc2 subss xmm0, xmm10
1407e61d8: f30f59c0 mulss xmm0, xmm0
1407e61dc: f30f58c1 addss xmm0, xmm1
1407e61e0: f30f51c0 sqrtss xmm0, xmm0
1407e61e4: 4939c0 cmp r8, rax
1407e61e7: 7527 jne 0x1407e6210
1407e61e9: 0f2e85d0040000 ucomiss xmm0, dword ptr [rbp + 0x4d0]
1407e61f0: 762e jbe 0x1407e6220
1407e61f2: 450f28f3 movaps xmm14, xmm11
1407e61f6: f30f1185d0040000 movss dword ptr [rbp + 0x4d0], xmm0
1407e61fe: 410f28c7 movaps xmm0, xmm15
1407e6202: eb16 jmp 0x1407e621a ; unknown
1407e6204: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e6210: 440f2ef8 ucomiss xmm15, xmm0
1407e6214: 760a jbe 0x1407e6220
1407e6216: 450f28f3 movaps xmm14, xmm11
1407e621a: 410f28f8 movaps xmm7, xmm8
1407e621e: eb13 jmp 0x1407e6233 ; unknown
1407e6220: 440f28d7 movaps xmm10, xmm7
1407e6224: 410f28c7 movaps xmm0, xmm15
1407e6228: 410f28f1 movaps xmm6, xmm9
1407e622c: 0f28bd80040000 movaps xmm7, xmmword ptr [rbp + 0x480]
1407e6233: 0f58e2 addps xmm4, xmm2
1407e6236: f30f58ade0030000 addss xmm5, dword ptr [rbp + 0x3e0]
1407e623e: 410f28c8 movaps xmm1, xmm8
1407e6242: 0f5ccc subps xmm1, xmm4
1407e6245: 410f28d3 movaps xmm2, xmm11
1407e6249: f30f5cd5 subss xmm2, xmm5
1407e624d: 0f59c9 mulps xmm1, xmm1
1407e6250: f30f16d9 movshdup xmm3, xmm1
1407e6254: f30f58d9 addss xmm3, xmm1
1407e6258: f30f59d2 mulss xmm2, xmm2
1407e625c: f30f58d3 addss xmm2, xmm3
1407e6260: 0f57c9 xorps xmm1, xmm1
1407e6263: f30f51ca sqrtss xmm1, xmm2
1407e6267: 4939c0 cmp r8, rax
1407e626a: 7574 jne 0x1407e62e0
1407e626c: 0f2e8dd0040000 ucomiss xmm1, dword ptr [rbp + 0x4d0]
1407e6273: 760f jbe 0x1407e6284
1407e6275: 440f28d5 movaps xmm10, xmm5
1407e6279: 450f28f3 movaps xmm14, xmm11
1407e627d: 410f28f8 movaps xmm7, xmm8
1407e6281: 0f28f4 movaps xmm6, xmm4
1407e6284: 410f2806 movaps xmm0, xmmword ptr [r14]
1407e6288: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
1407e628c: 418b462c mov eax, dword ptr [r14 + 0x2c]
1407e6290: 898520020000 mov dword ptr [rbp + 0x220], eax
1407e6296: 410f10461c movups xmm0, xmmword ptr [r14 + 0x1c]
1407e629b: 0f298510020000 movaps xmmword ptr [rbp + 0x210], xmm0
1407e62a2: 0f2803 movaps xmm0, xmmword ptr [rbx]
1407e62a5: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
1407e62a9: 8b432c mov eax, dword ptr [rbx + 0x2c]
1407e62ac: 898500020000 mov dword ptr [rbp + 0x200], eax
1407e62b2: 0f10431c movups xmm0, xmmword ptr [rbx + 0x1c]
1407e62b6: 0f2985f0010000 movaps xmmword ptr [rbp + 0x1f0], xmm0
1407e62bd: 458b4e40 mov r9d, dword ptr [r14 + 0x40]
1407e62c1: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e62c8: 488b8d60030000 mov rcx, qword ptr [rbp + 0x360]
1407e62cf: 31d2 xor edx, edx
1407e62d1: e81a632300 call 0x140a1c5f0 ; country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info
1407e62d6: 4885c0 test rax, rax
1407e62d9: 7415 je 0x1407e62f0
1407e62db: 8b782c mov edi, dword ptr [rax + 0x2c]
1407e62de: eb12 jmp 0x1407e62f2 ; unknown
1407e62e0: 0f2ec1 ucomiss xmm0, xmm1
1407e62e3: 7790 ja 0x1407e6275
1407e62e5: eb9d jmp 0x1407e6284 ; unknown
1407e62e7: 660f1f840000000000 nop word ptr [rax + rax]
1407e62f0: 31ff xor edi, edi
1407e62f2: 4c8b4330 mov r8, qword ptr [rbx + 0x30]
1407e62f6: 448b4b40 mov r9d, dword ptr [rbx + 0x40]
1407e62fa: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e6301: 488b8d60030000 mov rcx, qword ptr [rbp + 0x360]
1407e6308: 31d2 xor edx, edx
1407e630a: e8e1622300 call 0x140a1c5f0 ; country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info
1407e630f: 4885c0 test rax, rax
1407e6312: 740c je 0x1407e6320
1407e6314: 8b702c mov esi, dword ptr [rax + 0x2c]
1407e6317: eb09 jmp 0x1407e6322 ; unknown
1407e6319: 0f1f8000000000 nop dword ptr [rax]
1407e6320: 31f6 xor esi, esi
1407e6322: 458b7640 mov r14d, dword ptr [r14 + 0x40]
1407e6326: 448b6b40 mov r13d, dword ptr [rbx + 0x40]
1407e632a: 488b9db0040000 mov rbx, qword ptr [rbp + 0x4b0]
1407e6331: 483b9da0040000 cmp rbx, qword ptr [rbp + 0x4a0]
1407e6338: 0f8552f5ffff jne 0x1407e5890
1407e633e: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e6345: 488d8da0040000 lea rcx, [rbp + 0x4a0]
1407e634c: 488d155d322a02 lea rdx, [rip + 0x22a325d]
1407e6353: e8d8510400 call 0x14082b530 ; alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,utils::components::transform::Transform,utils::components::transform::
1407e6358: e933f5ffff jmp 0x1407e5890 ; unknown
1407e635d: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e6364: 488b8d70020000 mov rcx, qword ptr [rbp + 0x270]
1407e636b: 488b95a8030000 mov rdx, qword ptr [rbp + 0x3a8]
1407e6372: e839cca700 call 0x141262fb0 ; hashbrown::raw::impl$13::clone_from<tuple$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple$<> >,alloc::alloc::Global> | hashbrown::raw::impl$13:
1407e6377: 488b8510060000 mov rax, qword ptr [rbp + 0x610]
1407e637e: f30f6f4038 movdqu xmm0, xmmword ptr [rax + 0x38]
1407e6383: f30f7f4068 movdqu xmmword ptr [rax + 0x68], xmm0
1407e6388: 31c9 xor ecx, ecx
1407e638a: 0f57f6 xorps xmm6, xmm6
1407e638d: f30f103d03e31402 movss xmm7, dword ptr [rip + 0x214e303]
1407e6395: f3440f10054a3d1702 movss xmm8, dword ptr [rip + 0x2173d4a]
1407e639e: 4c8d3dc3312a02 lea r15, [rip + 0x22a31c3]
1407e63a5: 4c8db5a0040000 lea r14, [rbp + 0x4a0]
1407e63ac: 488b8538040000 mov rax, qword ptr [rbp + 0x438]
1407e63b3: 488bb5c0040000 mov rsi, qword ptr [rbp + 0x4c0]
1407e63ba: e983010000 jmp 0x1407e6542 ; unknown
1407e63bf: 90 nop 
1407e63c0: 410f28c6 movaps xmm0, xmm14
1407e63c4: 410fc6c664 shufps xmm0, xmm14, 0x64
1407e63c9: 410f28d3 movaps xmm2, xmm11
1407e63cd: 410f59d3 mulps xmm2, xmm11
1407e63d1: f30f16ca movshdup xmm1, xmm2
1407e63d5: f30f58ca addss xmm1, xmm2
1407e63d9: 0f28da movaps xmm3, xmm2
1407e63dc: 660f15da unpckhpd xmm3, xmm2
1407e63e0: f30f58d9 addss xmm3, xmm1
1407e63e4: 0fc6db00 shufps xmm3, xmm3, 0
1407e63e8: 0fc6d2ff shufps xmm2, xmm2, 0xff
1407e63ec: 0f5cd3 subps xmm2, xmm3
1407e63ef: 0f59c2 mulps xmm0, xmm2
1407e63f2: 410f28db movaps xmm3, xmm11
1407e63f6: 410f59de mulps xmm3, xmm14
1407e63fa: f30f16cb movshdup xmm1, xmm3
1407e63fe: f30f58cb addss xmm1, xmm3
1407e6402: 0f12db movhlps xmm3, xmm3
1407e6405: f30f58d9 addss xmm3, xmm1
1407e6409: f30f58db addss xmm3, xmm3
1407e640d: 0fc6db00 shufps xmm3, xmm3, 0
1407e6411: 410f59db mulps xmm3, xmm11
1407e6415: 0f58d8 addps xmm3, xmm0
1407e6418: 410f28c3 movaps xmm0, xmm11
1407e641c: 410fc6c3d2 shufps xmm0, xmm11, 0xd2
1407e6421: 0f28c8 movaps xmm1, xmm0
1407e6424: 410f59ce mulps xmm1, xmm14
1407e6428: 450fc6f6d2 shufps xmm14, xmm14, 0xd2
1407e642d: 450f59f3 mulps xmm14, xmm11
1407e6431: 410f5cce subps xmm1, xmm14
1407e6435: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
1407e6439: 410f28e3 movaps xmm4, xmm11
1407e643d: 410f58e3 addps xmm4, xmm11
1407e6441: 0fc6e4ff shufps xmm4, xmm4, 0xff
1407e6445: 0f59cc mulps xmm1, xmm4
1407e6448: 0f58cb addps xmm1, xmm3
1407e644b: 410f28d9 movaps xmm3, xmm9
1407e644f: 0f58d9 addps xmm3, xmm1
1407e6452: 0f12c9 movhlps xmm1, xmm1
1407e6455: f3410f58ca addss xmm1, xmm10
1407e645a: 450f14e5 unpcklps xmm12, xmm13
1407e645e: 410f28ec movaps xmm5, xmm12
1407e6462: 410f16ed movlhps xmm5, xmm13
1407e6466: 450fc6e504 shufps xmm12, xmm13, 4
1407e646b: 440f59e2 mulps xmm12, xmm2
1407e646f: 410f28d3 movaps xmm2, xmm11
1407e6473: 0f59d5 mulps xmm2, xmm5
1407e6476: f3440f16ea movshdup xmm13, xmm2
1407e647b: f3440f58ea addss xmm13, xmm2
1407e6480: 0f12d2 movhlps xmm2, xmm2
1407e6483: f3410f58d5 addss xmm2, xmm13
1407e6488: f30f58d2 addss xmm2, xmm2
1407e648c: 0fc6d200 shufps xmm2, xmm2, 0
1407e6490: 410f59d3 mulps xmm2, xmm11
1407e6494: 410f58d4 addps xmm2, xmm12
1407e6498: 0f59c5 mulps xmm0, xmm5
1407e649b: 0fc6edd2 shufps xmm5, xmm5, 0xd2
1407e649f: 410f59eb mulps xmm5, xmm11
1407e64a3: 0f5cc5 subps xmm0, xmm5
1407e64a6: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407e64aa: 0f59c4 mulps xmm0, xmm4
1407e64ad: 0f58c2 addps xmm0, xmm2
1407e64b0: 440f58c8 addps xmm9, xmm0
1407e64b4: 0f12c0 movhlps xmm0, xmm0
1407e64b7: f3410f58c2 addss xmm0, xmm10
1407e64bc: 4401ef add edi, r13d
1407e64bf: 488b85a8040000 mov rax, qword ptr [rbp + 0x4a8]
1407e64c6: 496bcc70 imul rcx, r12, 0x70
1407e64ca: 0f285500 movaps xmm2, xmmword ptr [rbp]
1407e64ce: 0f291408 movaps xmmword ptr [rax + rcx], xmm2
1407e64d2: 0f135c0810 movlps qword ptr [rax + rcx + 0x10], xmm3
1407e64d7: f30f114c0818 movss dword ptr [rax + rcx + 0x18], xmm1
1407e64dd: 0f288dd0010000 movaps xmm1, xmmword ptr [rbp + 0x1d0]
1407e64e4: 0f114c081c movups xmmword ptr [rax + rcx + 0x1c], xmm1
1407e64e9: 8b95e0010000 mov edx, dword ptr [rbp + 0x1e0]
1407e64ef: 8954082c mov dword ptr [rax + rcx + 0x2c], edx
1407e64f3: 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
1407e64f7: 0f294c0830 movaps xmmword ptr [rax + rcx + 0x30], xmm1
1407e64fc: 440f134c0840 movlps qword ptr [rax + rcx + 0x40], xmm9
1407e6502: f30f11440848 movss dword ptr [rax + rcx + 0x48], xmm0
1407e6508: 660f6f85b0010000 movdqa xmm0, xmmword ptr [rbp + 0x1b0]
1407e6510: f30f7f44084c movdqu xmmword ptr [rax + rcx + 0x4c], xmm0
1407e6516: 8b95c0010000 mov edx, dword ptr [rbp + 0x1c0]
1407e651c: 8954085c mov dword ptr [rax + rcx + 0x5c], edx
1407e6520: 44896c0860 mov dword ptr [rax + rcx + 0x60], r13d
1407e6525: 44896c0864 mov dword ptr [rax + rcx + 0x64], r13d
1407e652a: 897c0868 mov dword ptr [rax + rcx + 0x68], edi
1407e652e: 49ffc4 inc r12
1407e6531: 4c89a5b0040000 mov qword ptr [rbp + 0x4b0], r12
1407e6538: 4889d9 mov rcx, rbx
1407e653b: 488b8538040000 mov rax, qword ptr [rbp + 0x438]
1407e6542: 488b9548030000 mov rdx, qword ptr [rbp + 0x348]
1407e6549: 483bb5b8040000 cmp rsi, qword ptr [rbp + 0x4b8]
1407e6550: 0f842c010000 je 0x1407e6682
1407e6556: 4839d1 cmp rcx, rdx
1407e6559: 0f83c40d0000 jae 0x1407e7323
1407e655f: 4989f4 mov r12, rsi
1407e6562: 4883c650 add rsi, 0x50
1407e6566: 488d5901 lea rbx, [rcx + 1]
1407e656a: 48833cc800 cmp qword ptr [rax + rcx*8], 0
1407e656f: 4889d9 mov rcx, rbx
1407e6572: 75d5 jne 0x1407e6549
1407e6574: 410f280424 movaps xmm0, xmmword ptr [r12]
1407e6579: 0f294500 movaps xmmword ptr [rbp], xmm0
1407e657d: f2450f104c2410 movsd xmm9, qword ptr [r12 + 0x10]
1407e6584: f3450f10542418 movss xmm10, dword ptr [r12 + 0x18]
1407e658b: 418b44242c mov eax, dword ptr [r12 + 0x2c]
1407e6590: 8985e0010000 mov dword ptr [rbp + 0x1e0], eax
1407e6596: 410f1044241c movups xmm0, xmmword ptr [r12 + 0x1c]
1407e659c: 0f2985d0010000 movaps xmmword ptr [rbp + 0x1d0], xmm0
1407e65a3: 450f281c24 movaps xmm11, xmmword ptr [r12]
1407e65a8: f3450f10642438 movss xmm12, dword ptr [r12 + 0x38]
1407e65af: 410f28c4 movaps xmm0, xmm12
1407e65b3: f30f59c6 mulss xmm0, xmm6
1407e65b7: 450f28f4 movaps xmm14, xmm12
1407e65bb: 440f14f0 unpcklps xmm14, xmm0
1407e65bf: 450fc6f6d4 shufps xmm14, xmm14, 0xd4
1407e65c4: 440f5935e42b2a02 mulps xmm14, xmmword ptr [rip + 0x22a2be4]
1407e65cc: 410f280424 movaps xmm0, xmmword ptr [r12]
1407e65d1: 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
1407e65d5: 410f1044241c movups xmm0, xmmword ptr [r12 + 0x1c]
1407e65db: 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
1407e65e2: 418b44242c mov eax, dword ptr [r12 + 0x2c]
1407e65e7: 8985c0010000 mov dword ptr [rbp + 0x1c0], eax
1407e65ed: 450f28ec movaps xmm13, xmm12
1407e65f1: f3440f59ef mulss xmm13, xmm7
1407e65f6: f3450f59e0 mulss xmm12, xmm8
1407e65fb: f3440f592d6cf21302 mulss xmm13, dword ptr [rip + 0x213f26c]
1407e6604: 4d8b442430 mov r8, qword ptr [r12 + 0x30]
1407e6609: 458b4c2440 mov r9d, dword ptr [r12 + 0x40]
1407e660e: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e6615: 488b8d60030000 mov rcx, qword ptr [rbp + 0x360]
1407e661c: 31d2 xor edx, edx
1407e661e: e8cd5f2300 call 0x140a1c5f0 ; country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info
1407e6623: 4885c0 test rax, rax
1407e6626: 7428 je 0x1407e6650
1407e6628: 8b782c mov edi, dword ptr [rax + 0x2c]
1407e662b: 458b6c2440 mov r13d, dword ptr [r12 + 0x40]
1407e6630: 4c8ba5b0040000 mov r12, qword ptr [rbp + 0x4b0]
1407e6637: 4c3ba5a0040000 cmp r12, qword ptr [rbp + 0x4a0]
1407e663e: 0f857cfdffff jne 0x1407e63c0
1407e6644: eb25 jmp 0x1407e666b ; unknown
1407e6646: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e6650: 31ff xor edi, edi
1407e6652: 458b6c2440 mov r13d, dword ptr [r12 + 0x40]
1407e6657: 4c8ba5b0040000 mov r12, qword ptr [rbp + 0x4b0]
1407e665e: 4c3ba5a0040000 cmp r12, qword ptr [rbp + 0x4a0]
1407e6665: 0f8555fdffff jne 0x1407e63c0
1407e666b: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e6672: 4c89f1 mov rcx, r14
1407e6675: 4c89fa mov rdx, r15
1407e6678: e8b34e0400 call 0x14082b530 ; alloc::raw_vec::RawVec::grow_one<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,utils::components::transform::Transform,utils::components::transform::
1407e667d: e93efdffff jmp 0x1407e63c0 ; unknown
1407e6682: 488b85a0040000 mov rax, qword ptr [rbp + 0x4a0]
1407e6689: 48898520040000 mov qword ptr [rbp + 0x420], rax
1407e6690: 488b8da8040000 mov rcx, qword ptr [rbp + 0x4a8]
1407e6697: 486b85b004000070 imul rax, qword ptr [rbp + 0x4b0], 0x70
1407e669f: 4801c8 add rax, rcx
1407e66a2: 48898500040000 mov qword ptr [rbp + 0x400], rax
1407e66a9: 488b8568020000 mov rax, qword ptr [rbp + 0x268]
1407e66b0: 488b00 mov rax, qword ptr [rax]
1407e66b3: 488985c8040000 mov qword ptr [rbp + 0x4c8], rax
1407e66ba: 488b8560020000 mov rax, qword ptr [rbp + 0x260]
1407e66c1: 488b00 mov rax, qword ptr [rax]
1407e66c4: 488985c0030000 mov qword ptr [rbp + 0x3c0], rax
1407e66cb: 488b8558020000 mov rax, qword ptr [rbp + 0x258]
1407e66d2: 488b00 mov rax, qword ptr [rax]
1407e66d5: 488985d0040000 mov qword ptr [rbp + 0x4d0], rax
1407e66dc: 48898db8040000 mov qword ptr [rbp + 0x4b8], rcx
1407e66e3: 48898dc0040000 mov qword ptr [rbp + 0x4c0], rcx
1407e66ea: 660f1f440000 nop word ptr [rax + rax]
1407e66f0: 488b8dc0040000 mov rcx, qword ptr [rbp + 0x4c0]
1407e66f7: 483b8d00040000 cmp rcx, qword ptr [rbp + 0x400]
1407e66fe: 0f84580a0000 je 0x1407e715c
1407e6704: 440f2809 movaps xmm9, xmmword ptr [rcx]
1407e6708: f2440f105910 movsd xmm11, qword ptr [rcx + 0x10]
1407e670e: f3440f106118 movss xmm12, dword ptr [rcx + 0x18]
1407e6714: 8b412c mov eax, dword ptr [rcx + 0x2c]
1407e6717: 8985c0020000 mov dword ptr [rbp + 0x2c0], eax
1407e671d: 0f10411c movups xmm0, xmmword ptr [rcx + 0x1c]
1407e6721: 0f2985b0020000 movaps xmmword ptr [rbp + 0x2b0], xmm0
1407e6728: 440f285130 movaps xmm10, xmmword ptr [rcx + 0x30]
1407e672d: f2440f106940 movsd xmm13, qword ptr [rcx + 0x40]
1407e6733: f3440f107148 movss xmm14, dword ptr [rcx + 0x48]
1407e6739: 8b415c mov eax, dword ptr [rcx + 0x5c]
1407e673c: 8985a0020000 mov dword ptr [rbp + 0x2a0], eax
1407e6742: 0f10414c movups xmm0, xmmword ptr [rcx + 0x4c]
1407e6746: 0f298590020000 movaps xmmword ptr [rbp + 0x290], xmm0
1407e674d: 8b4160 mov eax, dword ptr [rcx + 0x60]
1407e6750: 8b7164 mov esi, dword ptr [rcx + 0x64]
1407e6753: 8b4968 mov ecx, dword ptr [rcx + 0x68]
1407e6756: 48898dd0020000 mov qword ptr [rbp + 0x2d0], rcx
1407e675d: 89f1 mov ecx, esi
1407e675f: 898570040000 mov dword ptr [rbp + 0x470], eax
1407e6765: 0fafc8 imul ecx, eax
1407e6768: e8d3fa4a00 call 0x140c96240 ; utils::rng::init_rng
1407e676d: 488385c004000070 add qword ptr [rbp + 0x4c0], 0x70
1407e6775: 48898598040000 mov qword ptr [rbp + 0x498], rax
1407e677c: 8b85c0020000 mov eax, dword ptr [rbp + 0x2c0]
1407e6782: 488d4d4c lea rcx, [rbp + 0x4c]
1407e6786: 894110 mov dword ptr [rcx + 0x10], eax
1407e6789: 0f2885b0020000 movaps xmm0, xmmword ptr [rbp + 0x2b0]
1407e6790: 0f1101 movups xmmword ptr [rcx], xmm0
1407e6793: 8b85a0020000 mov eax, dword ptr [rbp + 0x2a0]
1407e6799: 894150 mov dword ptr [rcx + 0x50], eax
1407e679c: 0f288590020000 movaps xmm0, xmmword ptr [rbp + 0x290]
1407e67a3: 0f114140 movups xmmword ptr [rcx + 0x40], xmm0
1407e67a7: 440f294d30 movaps xmmword ptr [rbp + 0x30], xmm9
1407e67ac: 440f135d40 movlps qword ptr [rbp + 0x40], xmm11
1407e67b1: f3440f116548 movss dword ptr [rbp + 0x48], xmm12
1407e67b7: 8b8570040000 mov eax, dword ptr [rbp + 0x470]
1407e67bd: 894560 mov dword ptr [rbp + 0x60], eax
1407e67c0: 440f295570 movaps xmmword ptr [rbp + 0x70], xmm10
1407e67c5: 440f13ad80000000 movlps qword ptr [rbp + 0x80], xmm13
1407e67cd: f3440f11b588000000 movss dword ptr [rbp + 0x88], xmm14
1407e67d6: 89b5a0000000 mov dword ptr [rbp + 0xa0], esi
1407e67dc: 4531f6 xor r14d, r14d
1407e67df: 90 nop 
1407e67e0: 4983fe02 cmp r14, 2
1407e67e4: 0f84b6010000 je 0x1407e69a0
1407e67ea: 498d5e01 lea rbx, [r14 + 1]
1407e67ee: 48899db0000000 mov qword ptr [rbp + 0xb0], rbx
1407e67f5: 49c1e606 shl r14, 6
1407e67f9: 420f28743530 movaps xmm6, xmmword ptr [rbp + r14 + 0x30]
1407e67ff: f2420f107c3540 movsd xmm7, qword ptr [rbp + r14 + 0x40]
1407e6806: f3460f10443548 movss xmm8, dword ptr [rbp + r14 + 0x48]
1407e680d: 428b743560 mov esi, dword ptr [rbp + r14 + 0x60]
1407e6812: 488bbdf0020000 mov rdi, qword ptr [rbp + 0x2f0]
1407e6819: e8625385ff call 0x14003bb80 ; bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin> | bevy_app::plugin::Plugin::finish<country_core::systems::render_loop::painterly::PainterlyPlugin> | minidump_unwind::instruction_see
1407e681e: 488d8d40040000 lea rcx, [rbp + 0x440]
1407e6825: 4889fa mov rdx, rdi
1407e6828: e873de5200 call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1407e682d: 4c8bbd48040000 mov r15, qword ptr [rbp + 0x448]
1407e6834: 4d85ff test r15, r15
1407e6837: 7507 jne 0x1407e6840
1407e6839: 4c8bbd50040000 mov r15, qword ptr [rbp + 0x450]
1407e6840: 4c8ba560040000 mov r12, qword ptr [rbp + 0x460]
1407e6847: 498b07 mov rax, qword ptr [r15]
1407e684a: 498b7f10 mov rdi, qword ptr [r15 + 0x10]
1407e684e: 4829f8 sub rax, rdi
1407e6851: 483d87000000 cmp rax, 0x87
1407e6857: 0f861c010000 jbe 0x1407e6979
1407e685d: 498d042e lea rax, [r14 + rbp]
1407e6861: 4883c04c add rax, 0x4c
1407e6865: 0f28ce movaps xmm1, xmm6
1407e6868: 0f59ce mulps xmm1, xmm6
1407e686b: f30f16c1 movshdup xmm0, xmm1
1407e686f: f30f58c1 addss xmm0, xmm1
1407e6873: 0f28d1 movaps xmm2, xmm1
1407e6876: 660f15d1 unpckhpd xmm2, xmm1
1407e687a: f30f58d0 addss xmm2, xmm0
1407e687e: 0fc6d200 shufps xmm2, xmm2, 0
1407e6882: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407e6886: 0f5cca subps xmm1, xmm2
1407e6889: 0f281530292a02 movaps xmm2, xmmword ptr [rip + 0x22a2930]
1407e6890: 0f59ca mulps xmm1, xmm2
1407e6893: 0f28c6 movaps xmm0, xmm6
1407e6896: 0f59c2 mulps xmm0, xmm2
1407e6899: f30f16d0 movshdup xmm2, xmm0
1407e689d: f30f58d0 addss xmm2, xmm0
1407e68a1: 0f12c0 movhlps xmm0, xmm0
1407e68a4: f30f58c2 addss xmm0, xmm2
1407e68a8: f30f58c0 addss xmm0, xmm0
1407e68ac: 0fc6c000 shufps xmm0, xmm0, 0
1407e68b0: 0f59c6 mulps xmm0, xmm6
1407e68b3: 0f58c1 addps xmm0, xmm1
1407e68b6: 0f28ce movaps xmm1, xmm6
1407e68b9: f30f101d0f292a02 movss xmm3, dword ptr [rip + 0x22a290f]
1407e68c1: 0f59cb mulps xmm1, xmm3
1407e68c4: 0f28d6 movaps xmm2, xmm6
1407e68c7: 0fc6d6c9 shufps xmm2, xmm6, 0xc9
1407e68cb: 0f59d3 mulps xmm2, xmm3
1407e68ce: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
1407e68d2: 0f5cd1 subps xmm2, xmm1
1407e68d5: 0f28ce movaps xmm1, xmm6
1407e68d8: 0f58ce addps xmm1, xmm6
1407e68db: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407e68df: 0f59ca mulps xmm1, xmm2
1407e68e2: 0f58c8 addps xmm1, xmm0
1407e68e5: 0f5cf9 subps xmm7, xmm1
1407e68e8: 0f12c9 movhlps xmm1, xmm1
1407e68eb: f3440f5cc1 subss xmm8, xmm1
1407e68f0: 498b4f08 mov rcx, qword ptr [r15 + 8]
1407e68f4: 488d156563a800 lea rdx, [rip + 0xa86365]
1407e68fb: 48891439 mov qword ptr [rcx + rdi], rdx
1407e68ff: 0f11743908 movups xmmword ptr [rcx + rdi + 8], xmm6
1407e6904: 0f137c3918 movlps qword ptr [rcx + rdi + 0x18], xmm7
1407e6909: f3440f11443920 movss dword ptr [rcx + rdi + 0x20], xmm8
1407e6910: 0f1000 movups xmm0, xmmword ptr [rax]
1407e6913: 0f11443924 movups xmmword ptr [rcx + rdi + 0x24], xmm0
1407e6918: 8b4010 mov eax, dword ptr [rax + 0x10]
1407e691b: 89443934 mov dword ptr [rcx + rdi + 0x34], eax
1407e691f: c744393800000000 mov dword ptr [rcx + rdi + 0x38], 0
1407e6927: 48b8eb992ca1f3efa720 movabs rax, 0x20a7eff3a12c99eb
1407e6931: 4889443940 mov qword ptr [rcx + rdi + 0x40], rax
1407e6936: c644394803 mov byte ptr [rcx + rdi + 0x48], 3
1407e693b: c644395800 mov byte ptr [rcx + rdi + 0x58], 0
1407e6940: 4c89643968 mov qword ptr [rcx + rdi + 0x68], r12
1407e6945: 488d0524b7a900 lea rax, [rip + 0xa9b724]
1407e694c: 4889443978 mov qword ptr [rcx + rdi + 0x78], rax
1407e6951: 4881c788000000 add rdi, 0x88
1407e6958: 49897f10 mov qword ptr [r15 + 0x10], rdi
1407e695c: 488b8d00030000 mov rcx, qword ptr [rbp + 0x300]
1407e6963: 4883c148 add rcx, 0x48
1407e6967: 89f2 mov edx, esi
1407e6969: 4d89e0 mov r8, r12
1407e696c: e8bfb20200 call 0x140811c30 ; system_decorator::decorator_visual_state::impl$5::add_decorator_visual_entity
1407e6971: 4989de mov r14, rbx
1407e6974: e967feffff jmp 0x1407e67e0 ; unknown
1407e6979: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1407e6982: 41b888000000 mov r8d, 0x88
1407e6988: 41b901000000 mov r9d, 1
1407e698e: 4c89f9 mov rcx, r15
1407e6991: 4889fa mov rdx, rdi
1407e6994: e887150c02 call 0x1428a7f20 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1407e6999: e9bffeffff jmp 0x1407e685d ; unknown
1407e699e: 6690 nop 
1407e69a0: 410f28c9 movaps xmm1, xmm9
1407e69a4: 410f59c9 mulps xmm1, xmm9
1407e69a8: f30f16c1 movshdup xmm0, xmm1
1407e69ac: f30f58c1 addss xmm0, xmm1
1407e69b0: 0f28d1 movaps xmm2, xmm1
1407e69b3: 660f15d1 unpckhpd xmm2, xmm1
1407e69b7: f30f58d0 addss xmm2, xmm0
1407e69bb: 0fc6d200 shufps xmm2, xmm2, 0
1407e69bf: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407e69c3: 0f5cca subps xmm1, xmm2
1407e69c6: 0f281d13282a02 movaps xmm3, xmmword ptr [rip + 0x22a2813]
1407e69cd: 0f59cb mulps xmm1, xmm3
1407e69d0: 410f28c1 movaps xmm0, xmm9
1407e69d4: 0f59c3 mulps xmm0, xmm3
1407e69d7: f30f16d0 movshdup xmm2, xmm0
1407e69db: f30f58d0 addss xmm2, xmm0
1407e69df: 0f12c0 movhlps xmm0, xmm0
1407e69e2: f30f58c2 addss xmm0, xmm2
1407e69e6: f30f58c0 addss xmm0, xmm0
1407e69ea: 0fc6c000 shufps xmm0, xmm0, 0
1407e69ee: 410f59c1 mulps xmm0, xmm9
1407e69f2: 0f58c1 addps xmm0, xmm1
1407e69f5: 410f28c9 movaps xmm1, xmm9
1407e69f9: f30f102567661e02 movss xmm4, dword ptr [rip + 0x21e6667]
1407e6a01: 0f59cc mulps xmm1, xmm4
1407e6a04: 410f28d1 movaps xmm2, xmm9
1407e6a08: 410fc6d1c9 shufps xmm2, xmm9, 0xc9
1407e6a0d: 0f59d4 mulps xmm2, xmm4
1407e6a10: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
1407e6a14: 0f5cd1 subps xmm2, xmm1
1407e6a17: 410f28c9 movaps xmm1, xmm9
1407e6a1b: 410f58c9 addps xmm1, xmm9
1407e6a1f: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407e6a23: 0f59ca mulps xmm1, xmm2
1407e6a26: 0f58c8 addps xmm1, xmm0
1407e6a29: 440f58d9 addps xmm11, xmm1
1407e6a2d: 0f12c9 movhlps xmm1, xmm1
1407e6a30: f3440f58e1 addss xmm12, xmm1
1407e6a35: 410f28ca movaps xmm1, xmm10
1407e6a39: 410f59ca mulps xmm1, xmm10
1407e6a3d: f30f16c1 movshdup xmm0, xmm1
1407e6a41: f30f58c1 addss xmm0, xmm1
1407e6a45: 0f28d1 movaps xmm2, xmm1
1407e6a48: 660f15d1 unpckhpd xmm2, xmm1
1407e6a4c: f30f58d0 addss xmm2, xmm0
1407e6a50: 0fc6d200 shufps xmm2, xmm2, 0
1407e6a54: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407e6a58: 0f5cca subps xmm1, xmm2
1407e6a5b: 0f59cb mulps xmm1, xmm3
1407e6a5e: 410f28c2 movaps xmm0, xmm10
1407e6a62: 0f59c3 mulps xmm0, xmm3
1407e6a65: f30f16d0 movshdup xmm2, xmm0
1407e6a69: f30f58d0 addss xmm2, xmm0
1407e6a6d: 0f12c0 movhlps xmm0, xmm0
1407e6a70: f30f58c2 addss xmm0, xmm2
1407e6a74: f30f58c0 addss xmm0, xmm0
1407e6a78: 0fc6c000 shufps xmm0, xmm0, 0
1407e6a7c: 410f59c2 mulps xmm0, xmm10
1407e6a80: 0f58c1 addps xmm0, xmm1
1407e6a83: 410f28ca movaps xmm1, xmm10
1407e6a87: 0f59cc mulps xmm1, xmm4
1407e6a8a: 410f28d2 movaps xmm2, xmm10
1407e6a8e: 410fc6d2c9 shufps xmm2, xmm10, 0xc9
1407e6a93: 0f59d4 mulps xmm2, xmm4
1407e6a96: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
1407e6a9a: 0f5cd1 subps xmm2, xmm1
1407e6a9d: 410f28ca movaps xmm1, xmm10
1407e6aa1: 410f58ca addps xmm1, xmm10
1407e6aa5: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407e6aa9: 0f59ca mulps xmm1, xmm2
1407e6aac: 0f58c8 addps xmm1, xmm0
1407e6aaf: 440f58e9 addps xmm13, xmm1
1407e6ab3: 0f12c9 movhlps xmm1, xmm1
1407e6ab6: f3440f58f1 addss xmm14, xmm1
1407e6abb: 410f28c4 movaps xmm0, xmm12
1407e6abf: f3410f5cc6 subss xmm0, xmm14
1407e6ac4: 410f28cb movaps xmm1, xmm11
1407e6ac8: 410f5ccd subps xmm1, xmm13
1407e6acc: 0f59c9 mulps xmm1, xmm1
1407e6acf: f30f16d1 movshdup xmm2, xmm1
1407e6ad3: f30f58d1 addss xmm2, xmm1
1407e6ad7: f30f59c0 mulss xmm0, xmm0
1407e6adb: f30f58c2 addss xmm0, xmm2
1407e6adf: f30f51f0 sqrtss xmm6, xmm0
1407e6ae3: 0f28c6 movaps xmm0, xmm6
1407e6ae6: f30f5e0542651e02 divss xmm0, dword ptr [rip + 0x21e6542]
1407e6aee: e8add41302 call 0x142923fa0 ; unknown
1407e6af3: f3480f2cc0 cvttss2si rax, xmm0
1407e6af8: 4889c1 mov rcx, rax
1407e6afb: 48c1f93f sar rcx, 0x3f
1407e6aff: 0f28c8 movaps xmm1, xmm0
1407e6b02: f30f5c0dd2351702 subss xmm1, dword ptr [rip + 0x21735d2]
1407e6b0a: f34c0f2cc1 cvttss2si r8, xmm1
1407e6b0f: 4921c8 and r8, rcx
1407e6b12: 4909c0 or r8, rax
1407e6b15: 0f2e05749b1702 ucomiss xmm0, dword ptr [rip + 0x2179b74]
1407e6b1c: ba00000000 mov edx, 0
1407e6b21: 490f43d0 cmovae rdx, r8
1407e6b25: 0f2e05b4351702 ucomiss xmm0, dword ptr [rip + 0x21735b4]
1407e6b2c: 48c7c0ffffffff mov rax, 0xffffffffffffffff
1407e6b33: 480f47d0 cmova rdx, rax
1407e6b37: 4883fa01 cmp rdx, 1
1407e6b3b: 4883d201 adc rdx, 1
1407e6b3f: f30f101521651e02 movss xmm2, dword ptr [rip + 0x21e6521]
1407e6b47: f30f5ed6 divss xmm2, xmm6
1407e6b4b: 488d0566292a02 lea rax, [rip + 0x22a2966]
1407e6b52: 4889442420 mov qword ptr [rsp + 0x20], rax
1407e6b57: 488d8d78020000 lea rcx, [rbp + 0x278]
1407e6b5e: 4c8d8d98040000 lea r9, [rbp + 0x498]
1407e6b65: e8e6974a00 call 0x140c90350 ; utils::random_splits
1407e6b6a: 440f2995e0030000 movaps xmmword ptr [rbp + 0x3e0], xmm10
1407e6b72: 440f298d80040000 movaps xmmword ptr [rbp + 0x480], xmm9
1407e6b7a: 488b8580020000 mov rax, qword ptr [rbp + 0x280]
1407e6b81: 488b8d88020000 mov rcx, qword ptr [rbp + 0x288]
1407e6b88: 488d0c88 lea rcx, [rax + rcx*4]
1407e6b8c: 48898dd8040000 mov qword ptr [rbp + 0x4d8], rcx
1407e6b93: 410f28c5 movaps xmm0, xmm13
1407e6b97: f3410f5cc3 subss xmm0, xmm11
1407e6b9c: 410f28ce movaps xmm1, xmm14
1407e6ba0: f3410f5ccc subss xmm1, xmm12
1407e6ba5: 0f28f9 movaps xmm7, xmm1
1407e6ba8: 0f573d41ed1302 xorps xmm7, xmmword ptr [rip + 0x213ed41]
1407e6baf: f30f59c9 mulss xmm1, xmm1
1407e6bb3: 0f28d0 movaps xmm2, xmm0
1407e6bb6: f30f59d0 mulss xmm2, xmm0
1407e6bba: f30f58d1 addss xmm2, xmm1
1407e6bbe: 0f57c9 xorps xmm1, xmm1
1407e6bc1: f30f51ca sqrtss xmm1, xmm2
1407e6bc5: f3440f100536ed1302 movss xmm8, dword ptr [rip + 0x213ed36]
1407e6bce: f3440f5ec1 divss xmm8, xmm1
1407e6bd3: f3410f59f8 mulss xmm7, xmm8
1407e6bd8: f3440f59c0 mulss xmm8, xmm0
1407e6bdd: f3450f16fb movshdup xmm15, xmm11
1407e6be2: f3410f16c5 movshdup xmm0, xmm13
1407e6be7: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
1407e6bee: 0f29b5e0020000 movaps xmmword ptr [rbp + 0x2e0], xmm6
1407e6bf5: f30f12c6 movsldup xmm0, xmm6
1407e6bf9: 0f2985d0030000 movaps xmmword ptr [rbp + 0x3d0], xmm0
1407e6c00: 410f28f4 movaps xmm6, xmm12
1407e6c04: 440f29b5f0030000 movaps xmmword ptr [rbp + 0x3f0], xmm14
1407e6c0c: 450f16f5 movlhps xmm14, xmm13
1407e6c10: 440f29ad80030000 movaps xmmword ptr [rbp + 0x380], xmm13
1407e6c18: 450fc6f5e2 shufps xmm14, xmm13, 0xe2
1407e6c1d: 440f29a570030000 movaps xmmword ptr [rbp + 0x370], xmm12
1407e6c25: 410f16f3 movlhps xmm6, xmm11
1407e6c29: 440f299d10040000 movaps xmmword ptr [rbp + 0x410], xmm11
1407e6c31: 410fc6f3e2 shufps xmm6, xmm11, 0xe2
1407e6c36: 31ff xor edi, edi
1407e6c38: b101 mov cl, 1
1407e6c3a: 483b85d8040000 cmp rax, qword ptr [rbp + 0x4d8]
1407e6c41: 0f84db030000 je 0x1407e7022
1407e6c47: 660f1f840000000000 nop word ptr [rax + rax]
1407e6c50: 4c8d7804 lea r15, [rax + 4]
1407e6c54: f6c101 test cl, 1
1407e6c57: 7417 je 0x1407e6c70
1407e6c59: 4c3bbdd8040000 cmp r15, qword ptr [rbp + 0x4d8]
1407e6c60: 0f84bc030000 je 0x1407e7022
1407e6c66: 4c8d6008 lea r12, [rax + 8]
1407e6c6a: 4989c5 mov r13, rax
1407e6c6d: eb07 jmp 0x1407e6c76 ; unknown
1407e6c6f: 90 nop 
1407e6c70: 4d89fc mov r12, r15
1407e6c73: 4989c7 mov r15, rax
1407e6c76: f3410f104d00 movss xmm1, dword ptr [r13]
1407e6c7c: f3410f1017 movss xmm2, dword ptr [r15]
1407e6c81: 0f28c1 movaps xmm0, xmm1
1407e6c84: f30f58c2 addss xmm0, xmm2
1407e6c88: f30f1025e0eb1302 movss xmm4, dword ptr [rip + 0x213ebe0]
1407e6c90: f30f59c4 mulss xmm0, xmm4
1407e6c94: f30f5cd1 subss xmm2, xmm1
1407e6c98: f30f591550252a02 mulss xmm2, dword ptr [rip + 0x22a2550]
1407e6ca0: 440f2895e0020000 movaps xmm10, xmmword ptr [rbp + 0x2e0]
1407e6ca8: f3440f59d2 mulss xmm10, xmm2
1407e6cad: f30f59d4 mulss xmm2, xmm4
1407e6cb1: 0f28c8 movaps xmm1, xmm0
1407e6cb4: f30f5cca subss xmm1, xmm2
1407e6cb8: f30f58d0 addss xmm2, xmm0
1407e6cbc: 0f14ca unpcklps xmm1, xmm2
1407e6cbf: 440f2825e9791402 movaps xmm12, xmmword ptr [rip + 0x21479e9]
1407e6cc7: 410f28d4 movaps xmm2, xmm12
1407e6ccb: 0f5cd1 subps xmm2, xmm1
1407e6cce: 410f59d7 mulps xmm2, xmm15
1407e6cd2: 0f289d60030000 movaps xmm3, xmmword ptr [rbp + 0x360]
1407e6cd9: 0f59d9 mulps xmm3, xmm1
1407e6cdc: 0f58da addps xmm3, xmm2
1407e6cdf: 0f58c9 addps xmm1, xmm1
1407e6ce2: 0f580de7791402 addps xmm1, xmmword ptr [rip + 0x21479e7]
1407e6ce9: 0f59c9 mulps xmm1, xmm1
1407e6cec: 440f5ce1 subps xmm12, xmm1
1407e6cf0: 440f5415887c1402 andps xmm10, xmmword ptr [rip + 0x2147c88]
1407e6cf8: f3440f591563631e02 mulss xmm10, dword ptr [rip + 0x21e6363]
1407e6d01: 410f5cdf subps xmm3, xmm15
1407e6d05: 440f59a5d0030000 mulps xmm12, xmmword ptr [rbp + 0x3d0]
1407e6d0d: 440f5925eb242a02 mulps xmm12, xmmword ptr [rip + 0x22a24eb]
1407e6d15: 440f58e3 addps xmm12, xmm3
1407e6d19: f30f100de3eb1302 movss xmm1, dword ptr [rip + 0x213ebe3]
1407e6d21: f30f5cc8 subss xmm1, xmm0
1407e6d25: f30f12c0 movsldup xmm0, xmm0
1407e6d29: 410f59c6 mulps xmm0, xmm14
1407e6d2d: f30f12c9 movsldup xmm1, xmm1
1407e6d31: 0f59ce mulps xmm1, xmm6
1407e6d34: 0f58c8 addps xmm1, xmm0
1407e6d37: f3450f16ec movshdup xmm13, xmm12
1407e6d3c: f3450f58ec addss xmm13, xmm12
1407e6d41: f3440f59ec mulss xmm13, xmm4
1407e6d46: 488b8d98040000 mov rcx, qword ptr [rbp + 0x498]
1407e6d4d: 48b82f64bd78641d76a0 movabs rax, 0xa0761d6478bd642f
1407e6d57: 4801c1 add rcx, rax
1407e6d5a: 4889c8 mov rax, rcx
1407e6d5d: 48badb28b4a0d17e03e7 movabs rdx, 0xe7037ed1a0b428db
1407e6d67: 4831d0 xor rax, rdx
1407e6d6a: 48f7e1 mul rcx
1407e6d6d: 31d0 xor eax, edx
1407e6d6f: 31db xor ebx, ebx
1407e6d71: a9000000c0 test eax, 0xc0000000
1407e6d76: 4c8d35a2272a02 lea r14, [rip + 0x22a27a2]
1407e6d7d: 488d0573272a02 lea rax, [rip + 0x22a2773]
1407e6d84: 4c0f44f0 cmove r14, rax
1407e6d88: 48898d98040000 mov qword ptr [rbp + 0x498], rcx
1407e6d8f: 49bdf1b2889749cdfbb9 movabs r13, 0xb9fbcd499788b2f1
1407e6d99: 48b874cfbeea90781fe2 movabs rax, 0xe21f7890eabecf74
1407e6da3: 4c0f44e8 cmove r13, rax
1407e6da7: 400f94c6 sete sil
1407e6dab: 0f138d98010000 movlps qword ptr [rbp + 0x198], xmm1
1407e6db2: 410f28d7 movaps xmm2, xmm15
1407e6db6: f3410f58d5 addss xmm2, xmm13
1407e6dbb: 488d8d40040000 lea rcx, [rbp + 0x440]
1407e6dc2: 488d9598010000 lea rdx, [rbp + 0x198]
1407e6dc9: e812604b00 call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
1407e6dce: 488d4d30 lea rcx, [rbp + 0x30]
1407e6dd2: 0f28cf movaps xmm1, xmm7
1407e6dd5: 410f28d0 movaps xmm2, xmm8
1407e6dd9: e8f2a14a00 call 0x140c90fd0 ; utils::xz_rotation_bad
1407e6dde: 440f285d30 movaps xmm11, xmmword ptr [rbp + 0x30]
1407e6de3: 488b8540040000 mov rax, qword ptr [rbp + 0x440]
1407e6dea: 48898520030000 mov qword ptr [rbp + 0x320], rax
1407e6df1: 8b8548040000 mov eax, dword ptr [rbp + 0x448]
1407e6df7: 898528030000 mov dword ptr [rbp + 0x328], eax
1407e6dfd: 488d8d98040000 lea rcx, [rbp + 0x498]
1407e6e04: e877854c00 call 0x140caf380 ; fastrand::Rng::f32
1407e6e09: 440f28c8 movaps xmm9, xmm0
1407e6e0d: 488d8d98040000 lea rcx, [rbp + 0x498]
1407e6e14: e867854c00 call 0x140caf380 ; fastrand::Rng::f32
1407e6e19: f3410f12cd movsldup xmm1, xmm13
1407e6e1e: 440f5ce1 subps xmm12, xmm1
1407e6e22: 4088f3 mov bl, sil
1407e6e25: 488d045d26000000 lea rax, [rbx*2 + 0x26]
1407e6e2d: 410f28d1 movaps xmm2, xmm9
1407e6e31: f30f5915ef621e02 mulss xmm2, dword ptr [rip + 0x21e62ef]
1407e6e39: f30f101dc3ea1302 movss xmm3, dword ptr [rip + 0x213eac3]
1407e6e41: 0f28cb movaps xmm1, xmm3
1407e6e44: f3410f5cc9 subss xmm1, xmm9
1407e6e49: f30f590db7ea1302 mulss xmm1, dword ptr [rip + 0x213eab7]
1407e6e51: f30f58ca addss xmm1, xmm2
1407e6e55: f3410f59ca mulss xmm1, xmm10
1407e6e5a: f30f5dcb minss xmm1, xmm3
1407e6e5e: 0f28d3 movaps xmm2, xmm3
1407e6e61: f30f5cd0 subss xmm2, xmm0
1407e6e65: f30f101d4bfe2202 movss xmm3, dword ptr [rip + 0x222fe4b]
1407e6e6d: f30f59d3 mulss xmm2, xmm3
1407e6e71: f30f590597232a02 mulss xmm0, dword ptr [rip + 0x22a2397]
1407e6e79: f30f58c2 addss xmm0, xmm2
1407e6e7d: f30f5dc3 minss xmm0, xmm3
1407e6e81: f3410f59c2 mulss xmm0, xmm10
1407e6e86: 8b8d28030000 mov ecx, dword ptr [rbp + 0x328]
1407e6e8c: 4c8d454c lea r8, [rbp + 0x4c]
1407e6e90: 418988bc000000 mov dword ptr [r8 + 0xbc], ecx
1407e6e97: 488b8d20030000 mov rcx, qword ptr [rbp + 0x320]
1407e6e9e: 498988b4000000 mov qword ptr [r8 + 0xb4], rcx
1407e6ea5: 488d8d5c040000 lea rcx, [rbp + 0x45c]
1407e6eac: c741ef00000000 mov dword ptr [rcx - 0x11], 0
1407e6eb3: 48c741e700000000 mov qword ptr [rcx - 0x19], 0
1407e6ebb: 488b8d40040000 mov rcx, qword ptr [rbp + 0x440]
1407e6ec2: 488b9547040000 mov rdx, qword ptr [rbp + 0x447]
1407e6ec9: 49899028010000 mov qword ptr [r8 + 0x128], rdx
1407e6ed0: 49898821010000 mov qword ptr [r8 + 0x121], rcx
1407e6ed7: 488b8dd0020000 mov rcx, qword ptr [rbp + 0x2d0]
1407e6ede: 8d0c0f lea ecx, [rdi + rcx]
1407e6ee1: 48c7453001000000 mov qword ptr [rbp + 0x30], 1
1407e6ee9: 4c896d38 mov qword ptr [rbp + 0x38], r13
1407e6eed: 48c7454000000000 mov qword ptr [rbp + 0x40], 0
1407e6ef5: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
1407e6efd: c7456001000000 mov dword ptr [rbp + 0x60], 1
1407e6f04: f30f114564 movss dword ptr [rbp + 0x64], xmm0
1407e6f09: f30f114d68 movss dword ptr [rbp + 0x68], xmm1
1407e6f0e: 48c7456c0000803f mov qword ptr [rbp + 0x6c], 0x3f800000
1407e6f16: c785b000000001000000 mov dword ptr [rbp + 0xb0], 1
1407e6f20: 440f13a5b4000000 movlps qword ptr [rbp + 0xb4], xmm12
1407e6f28: 898dbc000000 mov dword ptr [rbp + 0xbc], ecx
1407e6f2e: c785c000000000000000 mov dword ptr [rbp + 0xc0], 0
1407e6f38: 4c89b5d0000000 mov qword ptr [rbp + 0xd0], r14
1407e6f3f: 488985d8000000 mov qword ptr [rbp + 0xd8], rax
1407e6f46: 488d05f8252a02 lea rax, [rip + 0x22a25f8]
1407e6f4d: 488985e0000000 mov qword ptr [rbp + 0xe0], rax
1407e6f54: 48c785e80000000a000000 mov qword ptr [rbp + 0xe8], 0xa
1407e6f5f: 440f299df0000000 movaps xmmword ptr [rbp + 0xf0], xmm11
1407e6f67: 48b80000803f0000803f movabs rax, 0x3f8000003f800000
1407e6f71: 498980c0000000 mov qword ptr [r8 + 0xc0], rax
1407e6f78: 41c780c80000000000803f mov dword ptr [r8 + 0xc8], 0x3f800000
1407e6f83: c7852001000000000000 mov dword ptr [rbp + 0x120], 0
1407e6f8d: c7852801000000000000 mov dword ptr [rbp + 0x128], 0
1407e6f97: c6854401000002 mov byte ptr [rbp + 0x144], 2
1407e6f9e: c7854801000000000000 mov dword ptr [rbp + 0x148], 0
1407e6fa8: c6856c01000002 mov byte ptr [rbp + 0x16c], 2
1407e6faf: c6857c01000002 mov byte ptr [rbp + 0x17c], 2
1407e6fb6: 66c7857e0100000000 mov word ptr [rbp + 0x17e], 0
1407e6fbf: c6858001000009 mov byte ptr [rbp + 0x180], 9
1407e6fc6: 488b85f0020000 mov rax, qword ptr [rbp + 0x2f0]
1407e6fcd: 488b8d00030000 mov rcx, qword ptr [rbp + 0x300]
1407e6fd4: 4883c148 add rcx, 0x48
1407e6fd8: 48894c2430 mov qword ptr [rsp + 0x30], rcx
1407e6fdd: 4889442428 mov qword ptr [rsp + 0x28], rax
1407e6fe2: 488b85d0040000 mov rax, qword ptr [rbp + 0x4d0]
1407e6fe9: 4889442420 mov qword ptr [rsp + 0x20], rax
1407e6fee: 488d4d30 lea rcx, [rbp + 0x30]
1407e6ff2: 8b9570040000 mov edx, dword ptr [rbp + 0x470]
1407e6ff8: 4c8b85c8040000 mov r8, qword ptr [rbp + 0x4c8]
1407e6fff: 4c8b8dc0030000 mov r9, qword ptr [rbp + 0x3c0]
1407e7006: e86542ac00 call 0x1412ab270 ; system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder::build<system_decorator::decorator_visual_state::DecoratorTransientVisualState>
1407e700b: ffc7 inc edi
1407e700d: 31c9 xor ecx, ecx
1407e700f: 4c89e0 mov rax, r12
1407e7012: 4d89fd mov r13, r15
1407e7015: 483b85d8040000 cmp rax, qword ptr [rbp + 0x4d8]
1407e701c: 0f852efcffff jne 0x1407e6c50
1407e7022: 0f288580040000 movaps xmm0, xmmword ptr [rbp + 0x480]
1407e7029: 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
1407e7030: 0f288510040000 movaps xmm0, xmmword ptr [rbp + 0x410]
1407e7037: 0f138550040000 movlps qword ptr [rbp + 0x450], xmm0
1407e703e: 0f288570030000 movaps xmm0, xmmword ptr [rbp + 0x370]
1407e7045: f30f118558040000 movss dword ptr [rbp + 0x458], xmm0
1407e704d: 8b85c0020000 mov eax, dword ptr [rbp + 0x2c0]
1407e7053: 488d8d5c040000 lea rcx, [rbp + 0x45c]
1407e705a: 894110 mov dword ptr [rcx + 0x10], eax
1407e705d: 0f2885b0020000 movaps xmm0, xmmword ptr [rbp + 0x2b0]
1407e7064: 0f1101 movups xmmword ptr [rcx], xmm0
1407e7067: 0f2885e0030000 movaps xmm0, xmmword ptr [rbp + 0x3e0]
1407e706e: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
1407e7072: 0f288580030000 movaps xmm0, xmmword ptr [rbp + 0x380]
1407e7079: 0f134540 movlps qword ptr [rbp + 0x40], xmm0
1407e707d: 0f2885f0030000 movaps xmm0, xmmword ptr [rbp + 0x3f0]
1407e7084: f30f114548 movss dword ptr [rbp + 0x48], xmm0
1407e7089: 8b85a0020000 mov eax, dword ptr [rbp + 0x2a0]
1407e708f: 488d4d4c lea rcx, [rbp + 0x4c]
1407e7093: 894110 mov dword ptr [rcx + 0x10], eax
1407e7096: 0f288590020000 movaps xmm0, xmmword ptr [rbp + 0x290]
1407e709d: 0f1101 movups xmmword ptr [rcx], xmm0
1407e70a0: c7859801000000000000 mov dword ptr [rbp + 0x198], 0
1407e70aa: 488d85f0020000 lea rax, [rbp + 0x2f0]
1407e70b1: 4889442468 mov qword ptr [rsp + 0x68], rax
1407e70b6: 488d8598010000 lea rax, [rbp + 0x198]
1407e70bd: 4889442460 mov qword ptr [rsp + 0x60], rax
1407e70c2: 488b85d0040000 mov rax, qword ptr [rbp + 0x4d0]
1407e70c9: 4889442450 mov qword ptr [rsp + 0x50], rax
1407e70ce: 488b85c0030000 mov rax, qword ptr [rbp + 0x3c0]
1407e70d5: 4889442448 mov qword ptr [rsp + 0x48], rax
1407e70da: 488b85c8040000 mov rax, qword ptr [rbp + 0x4c8]
1407e70e1: 4889442440 mov qword ptr [rsp + 0x40], rax
1407e70e6: 8b8570040000 mov eax, dword ptr [rbp + 0x470]
1407e70ec: 89442438 mov dword ptr [rsp + 0x38], eax
1407e70f0: 488d4530 lea rax, [rbp + 0x30]
1407e70f4: 4889442430 mov qword ptr [rsp + 0x30], rax
1407e70f9: 488d8540040000 lea rax, [rbp + 0x440]
1407e7100: 4889442428 mov qword ptr [rsp + 0x28], rax
1407e7105: c744245800000000 mov dword ptr [rsp + 0x58], 0
1407e710d: c74424200000803f mov dword ptr [rsp + 0x20], 0x3f800000
1407e7115: ba27000000 mov edx, 0x27
1407e711a: 488d0daf232a02 lea rcx, [rip + 0x22a23af]
1407e7121: f30f1015afe71302 movss xmm2, dword ptr [rip + 0x213e7af]
1407e7129: 41b101 mov r9b, 1
1407e712c: e8dfb3ffff call 0x1407e2510 ; system_decorator::decorator_visual::flag_connections::clothline
1407e7131: 488b9578020000 mov rdx, qword ptr [rbp + 0x278]
1407e7138: 4885d2 test rdx, rdx
1407e713b: 0f84aff5ffff je 0x1407e66f0
1407e7141: 488b8d80020000 mov rcx, qword ptr [rbp + 0x280]
1407e7148: 48c1e202 shl rdx, 2
1407e714c: 41b804000000 mov r8d, 4
1407e7152: e8c9cae2ff call 0x140613c20 ; unknown
1407e7157: e994f5ffff jmp 0x1407e66f0 ; unknown
1407e715c: 488b8520040000 mov rax, qword ptr [rbp + 0x420]
1407e7163: 4885c0 test rax, rax
1407e7166: 7416 je 0x1407e717e
1407e7168: 486bd070 imul rdx, rax, 0x70
1407e716c: 41b810000000 mov r8d, 0x10
1407e7172: 488b8db8040000 mov rcx, qword ptr [rbp + 0x4b8]
1407e7179: e8a2cae2ff call 0x140613c20 ; unknown
1407e717e: 488b9558030000 mov rdx, qword ptr [rbp + 0x358]
1407e7185: 4885d2 test rdx, rdx
1407e7188: 488bb550030000 mov rsi, qword ptr [rbp + 0x350]
1407e718f: 7416 je 0x1407e71a7
1407e7191: 48c1e203 shl rdx, 3
1407e7195: 41b808000000 mov r8d, 8
1407e719b: 488b8d38040000 mov rcx, qword ptr [rbp + 0x438]
1407e71a2: e879cae2ff call 0x140613c20 ; unknown
1407e71a7: 4885f6 test rsi, rsi
1407e71aa: 7419 je 0x1407e71c5
1407e71ac: 48c1e604 shl rsi, 4
1407e71b0: 41b808000000 mov r8d, 8
1407e71b6: 488b8db8030000 mov rcx, qword ptr [rbp + 0x3b8]
1407e71bd: 4889f2 mov rdx, rsi
1407e71c0: e85bcae2ff call 0x140613c20 ; unknown
1407e71c5: 488b8508030000 mov rax, qword ptr [rbp + 0x308]
1407e71cc: 4885c0 test rax, rax
1407e71cf: 741a je 0x1407e71eb
1407e71d1: 488b8d10030000 mov rcx, qword ptr [rbp + 0x310]
1407e71d8: 48c1e004 shl rax, 4
1407e71dc: 488d1480 lea rdx, [rax + rax*4]
1407e71e0: 41b810000000 mov r8d, 0x10
1407e71e6: e835cae2ff call 0x140613c20 ; unknown
1407e71eb: 4883bd3803000000 cmp qword ptr [rbp + 0x338], 0
1407e71f3: 744b je 0x1407e7240
1407e71f5: e8360f4300 call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1407e71fa: 4889c1 mov rcx, rax
1407e71fd: 488b00 mov rax, qword ptr [rax]
1407e7200: 4883f801 cmp rax, 1
1407e7204: 0f8598000000 jne 0x1407e72a2
1407e720a: 4883c108 add rcx, 8
1407e720e: 48833900 cmp qword ptr [rcx], 0
1407e7212: 0f85c8000000 jne 0x1407e72e0
1407e7218: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
1407e721f: 48898dd8040000 mov qword ptr [rbp + 0x4d8], rcx
1407e7226: 4883c108 add rcx, 8
1407e722a: 488b9540030000 mov rdx, qword ptr [rbp + 0x340]
1407e7231: e8bafe4200 call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
1407e7236: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
1407e723d: 48ff00 inc qword ptr [rax]
1407e7240: 0f28b5f0040000 movaps xmm6, xmmword ptr [rbp + 0x4f0]
1407e7247: 0f28bd00050000 movaps xmm7, xmmword ptr [rbp + 0x500]
1407e724e: 440f288510050000 movaps xmm8, xmmword ptr [rbp + 0x510]
1407e7256: 440f288d20050000 movaps xmm9, xmmword ptr [rbp + 0x520]
1407e725e: 440f289530050000 movaps xmm10, xmmword ptr [rbp + 0x530]
1407e7266: 440f289d40050000 movaps xmm11, xmmword ptr [rbp + 0x540]
1407e726e: 440f28a550050000 movaps xmm12, xmmword ptr [rbp + 0x550]
1407e7276: 440f28ad60050000 movaps xmm13, xmmword ptr [rbp + 0x560]
1407e727e: 440f28b570050000 movaps xmm14, xmmword ptr [rbp + 0x570]
1407e7286: 440f28bd80050000 movaps xmm15, xmmword ptr [rbp + 0x580]
1407e728e: 4881c418060000 add rsp, 0x618
1407e7295: 5b pop rbx
1407e7296: 5f pop rdi
1407e7297: 5e pop rsi
1407e7298: 415c pop r12
1407e729a: 415d pop r13
1407e729c: 415e pop r14
1407e729e: 415f pop r15
1407e72a0: 5d pop rbp
1407e72a1: c3 ret 
1407e72a2: 4885c0 test rax, rax
1407e72a5: 752d jne 0x1407e72d4
1407e72a7: 31d2 xor edx, edx
1407e72a9: e8d2b21002 call 0x1428f2580 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1407e72ae: 4889c1 mov rcx, rax
1407e72b1: 4885c0 test rax, rax
1407e72b4: 0f8554ffffff jne 0x1407e720e
1407e72ba: eb18 jmp 0x1407e72d4 ; unknown
1407e72bc: 4885c0 test rax, rax
1407e72bf: 7513 jne 0x1407e72d4
1407e72c1: 31d2 xor edx, edx
1407e72c3: e8b8b21002 call 0x1428f2580 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
1407e72c8: 4889c1 mov rcx, rax
1407e72cb: 4885c0 test rax, rax
1407e72ce: 0f8516e1ffff jne 0x1407e53ea
1407e72d4: 488d0deda04302 lea rcx, [rip + 0x243a0ed]
1407e72db: e8f00d0f02 call 0x1428d80d0 ; std::thread::local::panic_access_error
1407e72e0: 488d0d51a14302 lea rcx, [rip + 0x243a151]
1407e72e7: e8641f0f02 call 0x1428d9250 ; core::cell::panic_already_borrowed
1407e72ec: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e72f3: 4c8d059e222a02 lea r8, [rip + 0x22a229e]
1407e72fa: 4889c1 mov rcx, rax
1407e72fd: 488b95b0030000 mov rdx, qword ptr [rbp + 0x3b0]
1407e7304: e80f220f02 call 0x1428d9518 ; core::panicking::panic_bounds_check
1407e7309: eb41 jmp 0x1407e734c ; unknown
1407e730b: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e7312: 4c8d0567222a02 lea r8, [rip + 0x22a2267]
1407e7319: 4c89ea mov rdx, r13
1407e731c: e8f7210f02 call 0x1428d9518 ; core::panicking::panic_bounds_check
1407e7321: eb29 jmp 0x1407e734c ; unknown
1407e7323: c685e704000001 mov byte ptr [rbp + 0x4e7], 1
1407e732a: 4c8d051f222a02 lea r8, [rip + 0x22a221f]
1407e7331: e8e2210f02 call 0x1428d9518 ; core::panicking::panic_bounds_check
1407e7336: eb14 jmp 0x1407e734c ; unknown
1407e7338: 4c8d0561212a02 lea r8, [rip + 0x22a2161]
1407e733f: b904000000 mov ecx, 4
1407e7344: 4889f2 mov rdx, rsi
1407e7347: e8971c0f02 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1407e734c: 0f0b ud2 
1407e734e: 6690 nop 
1407e7350: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e7355: 55 push rbp
1407e7356: 4157 push r15
1407e7358: 4156 push r14
1407e735a: 4155 push r13
1407e735c: 4154 push r12
1407e735e: 56 push rsi
1407e735f: 57 push rdi
1407e7360: 53 push rbx
1407e7361: 4881ec18010000 sub rsp, 0x118
1407e7368: 488daa80000000 lea rbp, [rdx + 0x80]
1407e736f: 440f297c2470 movaps xmmword ptr [rsp + 0x70], xmm15
1407e7375: 440f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm14
1407e737e: 440f29ac2490000000 movaps xmmword ptr [rsp + 0x90], xmm13
1407e7387: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
1407e7390: 440f299c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm11
1407e7399: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1407e73a2: 440f298c24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm9
1407e73ab: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1407e73b4: 0f29bc24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm7
1407e73bc: 0f29b42400010000 movaps xmmword ptr [rsp + 0x100], xmm6
1407e73c4: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
1407e73cb: 48ff00 inc qword ptr [rax]
1407e73ce: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e73d6: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e73de: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e73e7: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e73f0: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e73f9: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e7402: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e740b: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e7414: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e741d: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e7423: 4881c418010000 add rsp, 0x118
1407e742a: 5b pop rbx
1407e742b: 5f pop rdi
1407e742c: 5e pop rsi
1407e742d: 415c pop r12
1407e742f: 415d pop r13
1407e7431: 415e pop r14
1407e7433: 415f pop r15
1407e7435: 5d pop rbp
1407e7436: c3 ret 
1407e7437: 660f1f840000000000 nop word ptr [rax + rax]
1407e7440: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e7445: 55 push rbp
1407e7446: 4157 push r15
1407e7448: 4156 push r14
1407e744a: 4155 push r13
1407e744c: 4154 push r12
1407e744e: 56 push rsi
1407e744f: 57 push rdi
1407e7450: 53 push rbx
1407e7451: 4881ec18010000 sub rsp, 0x118
1407e7458: 488daa80000000 lea rbp, [rdx + 0x80]
1407e745f: 440f297c2470 movaps xmmword ptr [rsp + 0x70], xmm15
1407e7465: 440f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm14
1407e746e: 440f29ac2490000000 movaps xmmword ptr [rsp + 0x90], xmm13
1407e7477: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
1407e7480: 440f299c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm11
1407e7489: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1407e7492: 440f298c24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm9
1407e749b: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1407e74a4: 0f29bc24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm7
1407e74ac: 0f29b42400010000 movaps xmmword ptr [rsp + 0x100], xmm6
1407e74b4: 4883bd3003000000 cmp qword ptr [rbp + 0x330], 0
1407e74bc: 742f je 0x1407e74ed
1407e74be: 488d8540030000 lea rax, [rbp + 0x340]
1407e74c5: 48898530020000 mov qword ptr [rbp + 0x230], rax
1407e74cc: 488d8530020000 lea rax, [rbp + 0x230]
1407e74d3: 48898528020000 mov qword ptr [rbp + 0x228], rax
1407e74da: 488d0daf1e2a02 lea rcx, [rip + 0x22a1eaf]
1407e74e1: 488d9528020000 lea rdx, [rbp + 0x228]
1407e74e8: e8d306a700 call 0x141257bc0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
1407e74ed: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e74f5: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e74fd: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e7506: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e750f: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e7518: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e7521: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e752a: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e7533: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e753c: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e7542: 4881c418010000 add rsp, 0x118
1407e7549: 5b pop rbx
1407e754a: 5f pop rdi
1407e754b: 5e pop rsi
1407e754c: 415c pop r12
1407e754e: 415d pop r13
1407e7550: 415e pop r14
1407e7552: 415f pop r15
1407e7554: 5d pop rbp
1407e7555: c3 ret 
1407e7556: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e7560: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e7565: 55 push rbp
1407e7566: 4157 push r15
1407e7568: 4156 push r14
1407e756a: 4155 push r13
1407e756c: 4154 push r12
1407e756e: 56 push rsi
1407e756f: 57 push rdi
1407e7570: 53 push rbx
1407e7571: 4881ec18010000 sub rsp, 0x118
1407e7578: 488daa80000000 lea rbp, [rdx + 0x80]
1407e757f: 440f297c2470 movaps xmmword ptr [rsp + 0x70], xmm15
1407e7585: 440f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm14
1407e758e: 440f29ac2490000000 movaps xmmword ptr [rsp + 0x90], xmm13
1407e7597: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
1407e75a0: 440f299c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm11
1407e75a9: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1407e75b2: 440f298c24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm9
1407e75bb: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1407e75c4: 0f29bc24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm7
1407e75cc: 0f29b42400010000 movaps xmmword ptr [rsp + 0x100], xmm6
1407e75d4: 488b8508030000 mov rax, qword ptr [rbp + 0x308]
1407e75db: 4885c0 test rax, rax
1407e75de: 741a je 0x1407e75fa
1407e75e0: 488b8d10030000 mov rcx, qword ptr [rbp + 0x310]
1407e75e7: 48c1e004 shl rax, 4
1407e75eb: 488d1480 lea rdx, [rax + rax*4]
1407e75ef: 41b810000000 mov r8d, 0x10
1407e75f5: e826c6e2ff call 0x140613c20 ; unknown
1407e75fa: 488b8538030000 mov rax, qword ptr [rbp + 0x338]
1407e7601: 48898530030000 mov qword ptr [rbp + 0x330], rax
1407e7608: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e7610: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e7618: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e7621: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e762a: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e7633: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e763c: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e7645: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e764e: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e7657: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e765d: 4881c418010000 add rsp, 0x118
1407e7664: 5b pop rbx
1407e7665: 5f pop rdi
1407e7666: 5e pop rsi
1407e7667: 415c pop r12
1407e7669: 415d pop r13
1407e766b: 415e pop r14
1407e766d: 415f pop r15
1407e766f: 5d pop rbp
1407e7670: c3 ret 
1407e7671: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e7680: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e7685: 55 push rbp
1407e7686: 4157 push r15
1407e7688: 4156 push r14
1407e768a: 4155 push r13
1407e768c: 4154 push r12
1407e768e: 56 push rsi
1407e768f: 57 push rdi
1407e7690: 53 push rbx
1407e7691: 4881ec18010000 sub rsp, 0x118
1407e7698: 488daa80000000 lea rbp, [rdx + 0x80]
1407e769f: 440f297c2470 movaps xmmword ptr [rsp + 0x70], xmm15
1407e76a5: 440f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm14
1407e76ae: 440f29ac2490000000 movaps xmmword ptr [rsp + 0x90], xmm13
1407e76b7: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
1407e76c0: 440f299c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm11
1407e76c9: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1407e76d2: 440f298c24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm9
1407e76db: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1407e76e4: 0f29bc24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm7
1407e76ec: 0f29b42400010000 movaps xmmword ptr [rsp + 0x100], xmm6
1407e76f4: 488b9558030000 mov rdx, qword ptr [rbp + 0x358]
1407e76fb: 4885d2 test rdx, rdx
1407e76fe: 7416 je 0x1407e7716
1407e7700: 48c1e203 shl rdx, 3
1407e7704: 41b808000000 mov r8d, 8
1407e770a: 488b8d38040000 mov rcx, qword ptr [rbp + 0x438]
1407e7711: e80ac5e2ff call 0x140613c20 ; unknown
1407e7716: 488b9550030000 mov rdx, qword ptr [rbp + 0x350]
1407e771d: 4885d2 test rdx, rdx
1407e7720: 7416 je 0x1407e7738
1407e7722: 48c1e204 shl rdx, 4
1407e7726: 41b808000000 mov r8d, 8
1407e772c: 488b8db8030000 mov rcx, qword ptr [rbp + 0x3b8]
1407e7733: e8e8c4e2ff call 0x140613c20 ; unknown
1407e7738: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e7740: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e7748: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e7751: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e775a: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e7763: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e776c: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e7775: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e777e: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e7787: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e778d: 4881c418010000 add rsp, 0x118
1407e7794: 5b pop rbx
1407e7795: 5f pop rdi
1407e7796: 5e pop rsi
1407e7797: 415c pop r12
1407e7799: 415d pop r13
1407e779b: 415e pop r14
1407e779d: 415f pop r15
1407e779f: 5d pop rbp
1407e77a0: c3 ret 
1407e77a1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e77b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e77b5: 55 push rbp
1407e77b6: 4157 push r15
1407e77b8: 4156 push r14
1407e77ba: 4155 push r13
1407e77bc: 4154 push r12
1407e77be: 56 push rsi
1407e77bf: 57 push rdi
1407e77c0: 53 push rbx
1407e77c1: 4881ec18010000 sub rsp, 0x118
1407e77c8: 488daa80000000 lea rbp, [rdx + 0x80]
1407e77cf: 440f297c2470 movaps xmmword ptr [rsp + 0x70], xmm15
1407e77d5: 440f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm14
1407e77de: 440f29ac2490000000 movaps xmmword ptr [rsp + 0x90], xmm13
1407e77e7: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
1407e77f0: 440f299c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm11
1407e77f9: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1407e7802: 440f298c24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm9
1407e780b: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1407e7814: 0f29bc24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm7
1407e781c: 0f29b42400010000 movaps xmmword ptr [rsp + 0x100], xmm6
1407e7824: 488b8520040000 mov rax, qword ptr [rbp + 0x420]
1407e782b: 4885c0 test rax, rax
1407e782e: 7416 je 0x1407e7846
1407e7830: 486bd070 imul rdx, rax, 0x70
1407e7834: 41b810000000 mov r8d, 0x10
1407e783a: 488b8db8040000 mov rcx, qword ptr [rbp + 0x4b8]
1407e7841: e8dac3e2ff call 0x140613c20 ; unknown
1407e7846: c685e704000000 mov byte ptr [rbp + 0x4e7], 0
1407e784d: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e7855: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e785d: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e7866: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e786f: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e7878: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e7881: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e788a: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e7893: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e789c: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e78a2: 4881c418010000 add rsp, 0x118
1407e78a9: 5b pop rbx
1407e78aa: 5f pop rdi
1407e78ab: 5e pop rsi
1407e78ac: 415c pop r12
1407e78ae: 415d pop r13
1407e78b0: 415e pop r14
1407e78b2: 415f pop r15
1407e78b4: 5d pop rbp
1407e78b5: c3 ret 
1407e78b6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e78c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e78c5: 55 push rbp
1407e78c6: 4157 push r15
1407e78c8: 4156 push r14
1407e78ca: 4155 push r13
1407e78cc: 4154 push r12
1407e78ce: 56 push rsi
1407e78cf: 57 push rdi
1407e78d0: 53 push rbx
1407e78d1: 4881ec18010000 sub rsp, 0x118
1407e78d8: 488daa80000000 lea rbp, [rdx + 0x80]
1407e78df: 440f297c2470 movaps xmmword ptr [rsp + 0x70], xmm15
1407e78e5: 440f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm14
1407e78ee: 440f29ac2490000000 movaps xmmword ptr [rsp + 0x90], xmm13
1407e78f7: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
1407e7900: 440f299c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm11
1407e7909: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1407e7912: 440f298c24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm9
1407e791b: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1407e7924: 0f29bc24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm7
1407e792c: 0f29b42400010000 movaps xmmword ptr [rsp + 0x100], xmm6
1407e7934: 488b85d8040000 mov rax, qword ptr [rbp + 0x4d8]
1407e793b: 48ff00 inc qword ptr [rax]
1407e793e: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e7946: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e794e: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e7957: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e7960: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e7969: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e7972: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e797b: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e7984: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e798d: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e7993: 4881c418010000 add rsp, 0x118
1407e799a: 5b pop rbx
1407e799b: 5f pop rdi
1407e799c: 5e pop rsi
1407e799d: 415c pop r12
1407e799f: 415d pop r13
1407e79a1: 415e pop r14
1407e79a3: 415f pop r15
1407e79a5: 5d pop rbp
1407e79a6: c3 ret 
1407e79a7: 660f1f840000000000 nop word ptr [rax + rax]
1407e79b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e79b5: 55 push rbp
1407e79b6: 4157 push r15
1407e79b8: 4156 push r14
1407e79ba: 4155 push r13
1407e79bc: 4154 push r12
1407e79be: 56 push rsi
1407e79bf: 57 push rdi
1407e79c0: 53 push rbx
1407e79c1: 4881ec18010000 sub rsp, 0x118
1407e79c8: 488daa80000000 lea rbp, [rdx + 0x80]
1407e79cf: 440f297c2470 movaps xmmword ptr [rsp + 0x70], xmm15
1407e79d5: 440f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm14
1407e79de: 440f29ac2490000000 movaps xmmword ptr [rsp + 0x90], xmm13
1407e79e7: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
1407e79f0: 440f299c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm11
1407e79f9: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1407e7a02: 440f298c24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm9
1407e7a0b: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1407e7a14: 0f29bc24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm7
1407e7a1c: 0f29b42400010000 movaps xmmword ptr [rsp + 0x100], xmm6
1407e7a24: 488b9578020000 mov rdx, qword ptr [rbp + 0x278]
1407e7a2b: 4885d2 test rdx, rdx
1407e7a2e: 7416 je 0x1407e7a46
1407e7a30: 488b8d80020000 mov rcx, qword ptr [rbp + 0x280]
1407e7a37: 48c1e202 shl rdx, 2
1407e7a3b: 41b804000000 mov r8d, 4
1407e7a41: e8dac1e2ff call 0x140613c20 ; unknown
1407e7a46: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e7a4e: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e7a56: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e7a5f: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e7a68: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e7a71: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e7a7a: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e7a83: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e7a8c: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e7a95: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e7a9b: 4881c418010000 add rsp, 0x118
1407e7aa2: 5b pop rbx
1407e7aa3: 5f pop rdi
1407e7aa4: 5e pop rsi
1407e7aa5: 415c pop r12
1407e7aa7: 415d pop r13
1407e7aa9: 415e pop r14
1407e7aab: 415f pop r15
1407e7aad: 5d pop rbp
1407e7aae: c3 ret 
1407e7aaf: 90 nop 
1407e7ab0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e7ab5: 55 push rbp
1407e7ab6: 4157 push r15
1407e7ab8: 4156 push r14
1407e7aba: 4155 push r13
1407e7abc: 4154 push r12
1407e7abe: 56 push rsi
1407e7abf: 57 push rdi
1407e7ac0: 53 push rbx
1407e7ac1: 4881ec18010000 sub rsp, 0x118
1407e7ac8: 488daa80000000 lea rbp, [rdx + 0x80]
1407e7acf: 440f297c2470 movaps xmmword ptr [rsp + 0x70], xmm15
1407e7ad5: 440f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm14
1407e7ade: 440f29ac2490000000 movaps xmmword ptr [rsp + 0x90], xmm13
1407e7ae7: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
1407e7af0: 440f299c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm11
1407e7af9: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1407e7b02: 440f298c24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm9
1407e7b0b: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1407e7b14: 0f29bc24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm7
1407e7b1c: 0f29b42400010000 movaps xmmword ptr [rsp + 0x100], xmm6
1407e7b24: 80bde704000000 cmp byte ptr [rbp + 0x4e7], 0
1407e7b2b: 0f848b000000 je 0x1407e7bbc
1407e7b31: 488b85a0040000 mov rax, qword ptr [rbp + 0x4a0]
1407e7b38: 4885c0 test rax, rax
1407e7b3b: 7416 je 0x1407e7b53
1407e7b3d: 488b8da8040000 mov rcx, qword ptr [rbp + 0x4a8]
1407e7b44: 486bd070 imul rdx, rax, 0x70
1407e7b48: 41b810000000 mov r8d, 0x10
1407e7b4e: e8cdc0e2ff call 0x140613c20 ; unknown
1407e7b53: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e7b5b: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e7b63: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e7b6c: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e7b75: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e7b7e: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e7b87: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e7b90: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e7b99: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e7ba2: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e7ba8: 4881c418010000 add rsp, 0x118
1407e7baf: 5b pop rbx
1407e7bb0: 5f pop rdi
1407e7bb1: 5e pop rsi
1407e7bb2: 415c pop r12
1407e7bb4: 415d pop r13
1407e7bb6: 415e pop r14
1407e7bb8: 415f pop r15
1407e7bba: 5d pop rbp
1407e7bbb: c3 ret 
1407e7bbc: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e7bc4: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e7bcc: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e7bd5: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e7bde: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e7be7: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e7bf0: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e7bf9: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e7c02: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e7c0b: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e7c11: 4881c418010000 add rsp, 0x118
1407e7c18: 5b pop rbx
1407e7c19: 5f pop rdi
1407e7c1a: 5e pop rsi
1407e7c1b: 415c pop r12
1407e7c1d: 415d pop r13
1407e7c1f: 415e pop r14
1407e7c21: 415f pop r15
1407e7c23: 5d pop rbp
1407e7c24: c3 ret 
1407e7c25: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
1407e7c30: 4889542410 mov qword ptr [rsp + 0x10], rdx
1407e7c35: 55 push rbp
1407e7c36: 4157 push r15
1407e7c38: 4156 push r14
1407e7c3a: 4155 push r13
1407e7c3c: 4154 push r12
1407e7c3e: 56 push rsi
1407e7c3f: 57 push rdi
1407e7c40: 53 push rbx
1407e7c41: 4881ec18010000 sub rsp, 0x118
1407e7c48: 488daa80000000 lea rbp, [rdx + 0x80]
1407e7c4f: 440f297c2470 movaps xmmword ptr [rsp + 0x70], xmm15
1407e7c55: 440f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm14
1407e7c5e: 440f29ac2490000000 movaps xmmword ptr [rsp + 0x90], xmm13
1407e7c67: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
1407e7c70: 440f299c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm11
1407e7c79: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1407e7c82: 440f298c24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm9
1407e7c8b: 440f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm8
1407e7c94: 0f29bc24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm7
1407e7c9c: 0f29b42400010000 movaps xmmword ptr [rsp + 0x100], xmm6
1407e7ca4: 488b9540040000 mov rdx, qword ptr [rbp + 0x440]
1407e7cab: 4885d2 test rdx, rdx
1407e7cae: 7416 je 0x1407e7cc6
1407e7cb0: 488b8d48040000 mov rcx, qword ptr [rbp + 0x448]
1407e7cb7: 48c1e202 shl rdx, 2
1407e7cbb: 41b804000000 mov r8d, 4
1407e7cc1: e85abfe2ff call 0x140613c20 ; unknown
1407e7cc6: 0f28b42400010000 movaps xmm6, xmmword ptr [rsp + 0x100]
1407e7cce: 0f28bc24f0000000 movaps xmm7, xmmword ptr [rsp + 0xf0]
1407e7cd6: 440f288424e0000000 movaps xmm8, xmmword ptr [rsp + 0xe0]
1407e7cdf: 440f288c24d0000000 movaps xmm9, xmmword ptr [rsp + 0xd0]
1407e7ce8: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1407e7cf1: 440f289c24b0000000 movaps xmm11, xmmword ptr [rsp + 0xb0]
1407e7cfa: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
1407e7d03: 440f28ac2490000000 movaps xmm13, xmmword ptr [rsp + 0x90]
1407e7d0c: 440f28b42480000000 movaps xmm14, xmmword ptr [rsp + 0x80]
1407e7d15: 440f287c2470 movaps xmm15, xmmword ptr [rsp + 0x70]
1407e7d1b: 4881c418010000 add rsp, 0x118
1407e7d22: 5b pop rbx
1407e7d23: 5f pop rdi
1407e7d24: 5e pop rsi
1407e7d25: 415c pop r12
1407e7d27: 415d pop r13
1407e7d29: 415e pop r14
1407e7d2b: 415f pop r15
1407e7d2d: 5d pop rbp
1407e7d2e: c3 ret 
