140008c90: push rbp
140008c91: push r15
140008c93: push r14
140008c95: push r13
140008c97: push r12
140008c99: push rsi
140008c9a: push rdi
140008c9b: push rbx
140008c9c: sub rsp, 0x948
140008ca3: lea rbp, [rsp + 0x80]
140008cab: movdqa xmmword ptr [rbp + 0x8b0], xmm15
140008cb4: movaps xmmword ptr [rbp + 0x8a0], xmm14
140008cbc: movdqa xmmword ptr [rbp + 0x890], xmm13
140008cc5: movdqa xmmword ptr [rbp + 0x880], xmm12
140008cce: movaps xmmword ptr [rbp + 0x870], xmm11
140008cd6: movaps xmmword ptr [rbp + 0x860], xmm10
140008cde: movaps xmmword ptr [rbp + 0x850], xmm9
140008ce6: movaps xmmword ptr [rbp + 0x840], xmm8
140008cee: movaps xmmword ptr [rbp + 0x830], xmm7
140008cf5: movaps xmmword ptr [rbp + 0x820], xmm6
140008cfc: mov qword ptr [rbp + 0x818], 0xfffffffffffffffe
140008d07: mov r15, r9
140008d0a: mov qword ptr [rbp + 0x378], r8
140008d11: mov qword ptr [rbp + 0x268], rdx
140008d18: mov rdi, rcx
140008d1b: mov rax, qword ptr [rbp + 0x930]
140008d22: mov rcx, qword ptr [rax]
140008d25: mov qword ptr [rbp + 0x7c0], rcx
140008d2c: mov r12, qword ptr [rax + 0x10]
140008d30: mov r13d, dword ptr [rax + 0x1c]
140008d34: mov rcx, qword ptr [rax + 0x20]
140008d38: mov qword ptr [rbp + 0x2f0], rcx
140008d3f: mov rcx, qword ptr [rax + 0x30]
140008d43: mov qword ptr [rbp + 0x90], rcx
140008d4a: mov ecx, dword ptr [rax + 0x3c]
140008d4d: mov dword ptr [rbp + 0x238], ecx
140008d53: mov rcx, qword ptr [rax + 0x40]
140008d57: mov qword ptr [rbp + 0x370], rcx
140008d5e: mov rcx, qword ptr [rax + 0x50]
140008d62: mov qword ptr [rbp + 0x340], rcx
140008d69: mov ecx, dword ptr [rax + 0x5c]
140008d6c: mov dword ptr [rbp + 0x53c], ecx
140008d72: mov rcx, qword ptr [rax + 0x60]
140008d76: mov qword ptr [rbp + 0x348], rcx
140008d7d: mov rcx, qword ptr [rax + 0x70]
140008d81: mov qword ptr [rbp + 0x258], rcx
140008d88: mov ecx, dword ptr [rax + 0x7c]
140008d8b: mov dword ptr [rbp + 0x424], ecx
140008d91: mov rcx, qword ptr [rax + 0x80]
140008d98: mov qword ptr [rbp + 0x240], rcx
140008d9f: mov rax, qword ptr [rax + 0xa0]
140008da6: mov qword ptr [rbp + 0x338], rax
140008dad: mov rax, qword ptr [rbp + 0x960]
140008db4: mov rcx, qword ptr [rax]
140008db7: mov qword ptr [rbp + 0xe8], rcx
140008dbe: mov rcx, qword ptr [rax + 0x20]
140008dc2: mov qword ptr [rbp + 0x260], rcx
140008dc9: mov rax, qword ptr [rax + 0x28]
140008dcd: mov qword ptr [rbp + 0x250], rax
140008dd4: call 0x140c16a00 ; puffin::are_scopes_on
140008dd9: mov byte ptr [rbp + 0x797], al
140008ddf: test al, al
140008de1: je 0x140009017
140008de7: mov r14d, 0x4f
140008ded: mov ecx, 0x4f
140008df2: call 0x1400299b0 ; core::str::impl$0::rfind<ref$<str$> >
140008df7: lea rsi, [rip + 0x291e5b2]
140008dfe: cmp rax, 1
140008e02: jne 0x140008ea9
140008e08: mov r9, rdx
140008e0b: test rdx, rdx
140008e0e: je 0x140008e46
140008e10: cmp r9, 0x4f
140008e14: jae 0x140008e44
140008e16: lea rax, [rip + 0x291e593]
140008e1d: cmp byte ptr [r9 + rax], 0xbf
140008e22: jg 0x140008e46
140008e24: lea rax, [rip + 0x291e64d]
140008e2b: mov qword ptr [rsp + 0x20], rax
140008e30: lea rcx, [rip + 0x291e579]
140008e37: mov edx, 0x4f
140008e3c: xor r8d, r8d
140008e3f: call 0x1428d9940 ; core::str::slice_error_fail
140008e44: jne 0x140008e24
140008e46: mov rcx, r9
140008e49: call 0x1400299b0 ; core::str::impl$0::rfind<ref$<str$> >
140008e4e: cmp rax, 1
140008e52: jne 0x140008ea9
140008e54: mov r8, rdx
140008e57: add r8, 2
140008e5b: je 0x140008e96
140008e5d: cmp r8, 0x4f
140008e61: jae 0x140008e94
140008e63: lea rax, [rip + 0x291e546]
140008e6a: cmp byte ptr [r8 + rax], 0xbf
140008e6f: jg 0x140008e96
140008e71: lea rax, [rip + 0x291e618]
140008e78: mov qword ptr [rsp + 0x20], rax
140008e7d: lea rcx, [rip + 0x291e52c]
140008e84: mov edx, 0x4f
140008e89: mov r9d, 0x4f
140008e8f: call 0x1428d9940 ; core::str::slice_error_fail
140008e94: jne 0x140008e71
140008e96: mov r14d, 0x4d
140008e9c: sub r14, rdx
140008e9f: lea rsi, [rip + 0x291e50a]
140008ea6: add rsi, r8
140008ea9: mov qword ptr [rbp + 0x800], r15
140008eb0: lea r8, [rip + 0x291d5f1]
140008eb7: lea r15, [rip + 0x291d5aa]
140008ebe: nop 
140008ec0: movsx eax, byte ptr [r8 - 1]
140008ec5: test eax, eax
140008ec7: js 0x140008ee0
140008ec9: dec r8
140008ecc: cmp eax, 0x5c
140008ecf: jne 0x140008f34
140008ed1: jmp 0x140008f4c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140008ed3: nop word ptr cs:[rax + rax]
140008ee0: movzx ecx, byte ptr [r8 - 2]
140008ee5: cmp cl, 0xc0
140008ee8: jge 0x140008f0d
140008eea: movzx edx, byte ptr [r8 - 3]
140008eef: cmp dl, 0xc0
140008ef2: jge 0x140008f16
140008ef4: movzx r9d, byte ptr [r8 - 4]
140008ef9: add r8, -4
140008efd: and r9d, 7
140008f01: shl r9d, 6
140008f05: and edx, 0x3f
140008f08: or edx, r9d
140008f0b: jmp 0x140008f1d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140008f0d: add r8, -2
140008f11: and ecx, 0x1f
140008f14: jmp 0x140008f25 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140008f16: add r8, -3
140008f1a: and edx, 0xf
140008f1d: shl edx, 6
140008f20: and ecx, 0x3f
140008f23: or ecx, edx
140008f25: shl ecx, 6
140008f28: and al, 0x3f
140008f2a: movzx eax, al
140008f2d: or eax, ecx
140008f2f: cmp eax, 0x5c
140008f32: je 0x140008f4c
140008f34: cmp eax, 0x2f
140008f37: je 0x140008f4c
140008f39: cmp r8, r15
140008f3c: jne 0x140008ec0
140008f3e: mov ebx, r13d
140008f41: mov r13, r12
140008f44: mov r12d, 0x40
140008f4a: jmp 0x140008f9f ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140008f4c: lea r15, [rip + 0x291d515]
140008f53: sub r8, r15
140008f56: inc r8
140008f59: je 0x140008f8d
140008f5b: cmp r8, 0x40
140008f5f: jae 0x140008f8b
140008f61: cmp byte ptr [r8 + r15], 0xbf
140008f66: jg 0x140008f8d
140008f68: lea rax, [rip + 0x291e4e9]
140008f6f: mov qword ptr [rsp + 0x20], rax
140008f74: lea rcx, [rip + 0x291d4ed]
140008f7b: mov edx, 0x40
140008f80: mov r9d, 0x40
140008f86: call 0x1428d9940 ; core::str::slice_error_fail
140008f8b: jne 0x140008f68
140008f8d: mov ebx, r13d
140008f90: mov r13, r12
140008f93: mov r12d, 0x40
140008f99: sub r12, r8
140008f9c: add r15, r8
140008f9f: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
140008fa4: mov rcx, rax
140008fa7: mov rax, qword ptr [rax]
140008faa: cmp rax, 1
140008fae: jne 0x140010f9e
140008fb4: add rcx, 8
140008fb8: cmp qword ptr [rcx], 0
140008fbc: jne 0x1400111fb
140008fc2: mov qword ptr [rcx], 0xffffffffffffffff
140008fc9: mov qword ptr [rbp + 0x780], rcx
140008fd0: add rcx, 8
140008fd4: mov qword ptr [rsp + 0x20], r12
140008fd9: mov qword ptr [rsp + 0x30], 0
140008fe2: mov qword ptr [rsp + 0x28], 1
140008feb: mov rdx, rsi
140008fee: mov r8, r14
140008ff1: mov r9, r15
140008ff4: call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
140008ff9: mov qword ptr [rbp + 0x270], rax
140009000: mov rax, qword ptr [rbp + 0x780]
140009007: inc qword ptr [rax]
14000900a: mov r12, r13
14000900d: mov r13d, ebx
140009010: mov r15, qword ptr [rbp + 0x800]
140009017: lea rdx, [rip + 0x29a51a2]
14000901e: mov rcx, rdi
140009021: call 0x1405ca870 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
140009026: mov rbx, rax
140009029: test rax, rax
14000902c: je 0x1400114f7
140009032: lea rax, [rip + 0x29f3e78]
140009039: mov qword ptr [rbp + 0x6a0], rax
140009040: mov qword ptr [rbp + 0x6a8], 0x3a
14000904b: mov r14, rbx
14000904e: add r14, 0x10
140009052: cmp qword ptr [rbx + 0x18], 0x3a
140009057: jne 0x140011134
14000905d: mov rdx, qword ptr [r14]
140009060: lea rcx, [rip + 0x29f3e4a]
140009067: mov r8d, 0x3a
14000906d: call 0x142923bf0 ; ?
140009072: test eax, eax
140009074: jne 0x140011134
14000907a: mov rcx, qword ptr [rbx]
14000907d: mov rax, qword ptr [rbx + 8]
140009081: call qword ptr [rax + 0x18]
140009084: mov qword ptr [rbp + 0x120], rax
14000908b: cmp qword ptr [rbp + 0x120], 0
140009093: je 0x1400114f7
140009099: lea rdx, [rip + 0x29a5118]
1400090a0: mov rcx, rdi
1400090a3: call 0x1405ca870 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
1400090a8: mov rbx, rax
1400090ab: test rax, rax
1400090ae: je 0x140011508
1400090b4: lea rax, [rip + 0x29f3b9a]
1400090bb: mov qword ptr [rbp + 0x6a0], rax
1400090c2: mov qword ptr [rbp + 0x6a8], 0x3d
1400090cd: mov r14, rbx
1400090d0: add r14, 0x10
1400090d4: cmp qword ptr [rbx + 0x18], 0x3d
1400090d9: jne 0x140011168
1400090df: mov rdx, qword ptr [r14]
1400090e2: lea rcx, [rip + 0x29f3b6c]
1400090e9: mov r8d, 0x3d
1400090ef: call 0x142923bf0 ; ?
1400090f4: test eax, eax
1400090f6: jne 0x140011168
1400090fc: mov rcx, qword ptr [rbx]
1400090ff: mov rax, qword ptr [rbx + 8]
140009103: call qword ptr [rax + 0x18]
140009106: mov qword ptr [rbp + 0x128], rax
14000910d: cmp qword ptr [rbp + 0x128], 0
140009115: je 0x140011508
14000911b: lea rdx, [rip + 0x29a506e]
140009122: mov rcx, rdi
140009125: call 0x1405ca870 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
14000912a: mov rdi, rax
14000912d: test rax, rax
140009130: je 0x140011519
140009136: lea rax, [rip + 0x29f3aeb]
14000913d: mov qword ptr [rbp + 0x6a0], rax
140009144: mov qword ptr [rbp + 0x6a8], 0x2d
14000914f: mov rbx, rdi
140009152: add rbx, 0x10
140009156: cmp qword ptr [rdi + 0x18], 0x2d
14000915b: jne 0x14001119c
140009161: mov rdx, qword ptr [rbx]
140009164: lea rcx, [rip + 0x29f3abd]
14000916b: mov r8d, 0x2d
140009171: call 0x142923bf0 ; ?
140009176: test eax, eax
140009178: jne 0x14001119c
14000917e: mov rcx, qword ptr [rdi]
140009181: mov rax, qword ptr [rdi + 8]
140009185: call qword ptr [rax + 0x18]
140009188: mov qword ptr [rbp + 0x130], rax
14000918f: cmp qword ptr [rbp + 0x130], 0
140009197: je 0x140011519
14000919d: mov rax, qword ptr [rbp + 0x948]
1400091a4: mov rcx, qword ptr [rbp + 0x940]
1400091ab: mov dword ptr [r12], r13d
1400091af: mov r13, rcx
1400091b2: mov rcx, qword ptr [rbp + 0x7c0]
1400091b9: lea rdx, [rcx + 0x198]
1400091c0: mov qword ptr [rbp + 0x2e0], rdx
1400091c7: mov rdi, qword ptr [rcx + 8]
1400091cb: imul rdx, qword ptr [rcx + 0x10], 0x150
1400091d3: add rdx, rdi
1400091d6: mov qword ptr [rbp + 0xf0], rdx
1400091dd: mov edx, dword ptr [rax + 0x1c]
1400091e0: mov dword ptr [rbp + 0x23c], edx
1400091e6: mov rdx, qword ptr [rax]
1400091e9: mov qword ptr [rbp + 0x98], rdx
1400091f0: mov rax, qword ptr [rax + 0x10]
1400091f4: mov qword ptr [rbp + 0xa0], rax
1400091fb: mov eax, dword ptr [r15 + 0x1c]
1400091ff: mov dword ptr [rbp + 0x32c], eax
140009205: mov rdx, qword ptr [r15]
140009208: mov rax, qword ptr [r15 + 0x10]
14000920c: mov qword ptr [rbp + 0x110], rax
140009213: xorps xmm8, xmm8
140009217: mov rax, qword ptr [rbp + 0x2f0]
14000921e: movzx eax, byte ptr [rax + 0x30]
140009222: mov byte ptr [rbp + 0x648], al
140009228: lea rax, [rcx + 0x18]
14000922c: mov qword ptr [rbp + 0x248], rax
140009233: lea rax, [rcx + 0x1b8]
14000923a: mov qword ptr [rbp + 0xc0], rax
140009241: lea rax, [rdx + 0x60]
140009245: mov qword ptr [rbp + 0xb0], rax
14000924c: mov qword ptr [rbp + 0xb8], rdx
140009253: lea rax, [rdx + 0x30]
140009257: mov qword ptr [rbp + 0xa8], rax
14000925e: mov rax, qword ptr [rbp + 0x338]
140009265: add rax, 0x20
140009269: mov qword ptr [rbp + 0xf8], rax
140009270: lea rax, [rcx + 0x60]
140009274: mov qword ptr [rbp + 0x350], rax
14000927b: xor r14d, r14d
14000927e: lea rbx, [rbp + 0x590]
140009285: lea r12, [rbp + 0x460]
14000928c: jmp 0x140009330 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009291: nop word ptr cs:[rax + rax]
1400092a0: movdqu xmm1, xmmword ptr [rcx + r8]
1400092a6: movdqa xmm2, xmm1
1400092aa: pcmpeqb xmm2, xmm0
1400092ae: pmovmskb r9d, xmm2
1400092b3: test r9d, r9d
1400092b6: je 0x140009300
1400092b8: mov r10, qword ptr [rbp + 0x678]
1400092bf: movq xmm2, qword ptr [r10]
1400092c4: tzcnt r10d, r9d
1400092c9: add r10, r8
1400092cc: and r10, rdx
1400092cf: shl r10, 3
1400092d3: mov r11, rcx
1400092d6: sub r11, r10
1400092d9: movq xmm3, qword ptr [r11 - 8]
1400092df: pcmpeqd xmm3, xmm2
1400092e3: pshufd xmm3, xmm3, 0x50
1400092e8: movmskpd r10d, xmm3
1400092ed: cmp r10d, 3
1400092f1: je 0x140009330
1400092f3: lea r10d, [r9 - 1]
1400092f7: and r10w, r9w
1400092fb: mov r9d, r10d
1400092fe: jne 0x1400092c4
140009300: pcmpeqb xmm1, xmmword ptr [rip + 0x291c688]
140009308: pmovmskb r9d, xmm1
14000930d: test r9d, r9d
140009310: jne 0x1400093b9
140009316: add r8, rax
140009319: add r8, 0x10
14000931d: add rax, 0x10
140009321: and r8, rdx
140009324: jmp 0x1400092a0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009329: nop dword ptr [rax]
140009330: cmp rdi, qword ptr [rbp + 0xf0]
140009337: je 0x140010604
14000933d: mov rax, rdi
140009340: add rdi, 0x150
140009347: cmp byte ptr [rax + 0x148], 1
14000934e: jne 0x140009330
140009350: cmp r14, qword ptr [rax + 0x98]
140009357: jo 0x140009330
140009359: mov qword ptr [rbp + 0x7e0], rax
140009360: lea rdx, [rax + 0x128]
140009367: mov rax, qword ptr [rbp + 0x338]
14000936e: cmp qword ptr [rax + 0x18], 0
140009373: mov qword ptr [rbp + 0x678], rdx
14000937a: je 0x1400093b9
14000937c: mov rcx, qword ptr [rbp + 0xf8]
140009383: call 0x1405b1be0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge> >
140009388: mov rdx, qword ptr [rbp + 0x338]
14000938f: mov rcx, qword ptr [rdx]
140009392: mov rdx, qword ptr [rdx + 8]
140009396: mov r8, rdx
140009399: and r8, rax
14000939c: shr rax, 0x39
1400093a0: movd xmm0, eax
1400093a4: punpcklbw xmm0, xmm0
1400093a8: pshuflw xmm0, xmm0, 0
1400093ad: pshufd xmm0, xmm0, 0
1400093b2: xor eax, eax
1400093b4: jmp 0x1400092a0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400093b9: mov rax, qword ptr [rbp + 0x7e0]
1400093c0: mov ecx, dword ptr [rax + 0x128]
1400093c6: mov edx, dword ptr [rax + 0x12c]
1400093cc: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
1400093d1: mov qword ptr [rbp + 0x438], rax
1400093d8: mov rcx, qword ptr [rbp + 0x438]
1400093df: call 0x140c96270 ; utils::rng::init_rng_u64
1400093e4: mov qword ptr [rbp + 0x428], rax
1400093eb: mov rdx, qword ptr [rbp + 0x7e0]
1400093f2: cmp r14, qword ptr [rdx + 0x98]
1400093f9: jo 0x1400114c2
1400093ff: mov rax, qword ptr [rdx + 0xa0]
140009406: mov qword ptr [rbp + 0x578], rax
14000940d: mov rax, qword ptr [rdx + 0xa8]
140009414: mov qword ptr [rbp + 0x570], rax
14000941b: mov rax, qword ptr [rbp + 0x958]
140009422: mov rcx, qword ptr [rax]
140009425: mov edx, dword ptr [rdx + 0x128]
14000942b: call 0x140a1c930 ; country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
140009430: test rax, rax
140009433: je 0x14000943a
140009435: mov eax, dword ptr [rax + 0x28]
140009438: jmp 0x14000943f ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000943a: mov eax, 0x186a3
14000943f: mov qword ptr [rbp + 0x758], rax
140009446: mov rax, qword ptr [rbp + 0x958]
14000944d: mov rcx, qword ptr [rax]
140009450: mov rax, qword ptr [rbp + 0x678]
140009457: mov edx, dword ptr [rax]
140009459: call 0x140a1c930 ; country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
14000945e: mov dword ptr [rbp + 0x528], 0
140009468: test rax, rax
14000946b: je 0x140009487
14000946d: add rax, 0x30
140009471: mov rcx, rax
140009474: call 0x140839490 ; country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_stair
140009479: test rax, rax
14000947c: je 0x140009487
14000947e: mov eax, dword ptr [rax + 8]
140009481: mov dword ptr [rbp + 0x528], eax
140009487: mov rax, qword ptr [rbp + 0x958]
14000948e: mov rcx, qword ptr [rax]
140009491: mov rax, qword ptr [rbp + 0x678]
140009498: mov edx, dword ptr [rax]
14000949a: call 0x140a1c930 ; country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info
14000949f: test rax, rax
1400094a2: je 0x1400094dc
1400094a4: add rax, 0x30
1400094a8: mov rcx, rax
1400094ab: call 0x140839490 ; country_core::resources::walls::decorator::DecoratorExtraInfo::try_as_stair
1400094b0: test rax, rax
1400094b3: je 0x1400094dc
1400094b5: movzx eax, byte ptr [rax + 0x10]
1400094b9: mov dword ptr [rbp + 0x7b8], eax
1400094bf: mov dword ptr [rbp + 0x588], 0
1400094c9: mov rax, qword ptr [rbp + 0x338]
1400094d0: cmp qword ptr [rax + 0x18], 0
1400094d5: jne 0x140009502
1400094d7: jmp 0x1400095d5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400094dc: mov dword ptr [rbp + 0x7b8], 0
1400094e6: mov dword ptr [rbp + 0x588], 0
1400094f0: mov rax, qword ptr [rbp + 0x338]
1400094f7: cmp qword ptr [rax + 0x18], 0
1400094fc: je 0x1400095d5
140009502: mov rcx, qword ptr [rbp + 0xf8]
140009509: mov rdx, qword ptr [rbp + 0x678]
140009510: call 0x1405b1be0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::stairs::stairs_state::StairGraphEdge> >
140009515: mov rdx, qword ptr [rbp + 0x338]
14000951c: mov rcx, qword ptr [rdx]
14000951f: mov rdx, qword ptr [rdx + 8]
140009523: mov r8, rdx
140009526: and r8, rax
140009529: shr rax, 0x39
14000952d: movd xmm0, eax
140009531: punpcklbw xmm0, xmm0
140009535: pshuflw xmm0, xmm0, 0
14000953a: pshufd xmm0, xmm0, 0
14000953f: xor eax, eax
140009541: movdqu xmm1, xmmword ptr [rcx + r8]
140009547: movdqa xmm2, xmm1
14000954b: pcmpeqb xmm2, xmm0
14000954f: pmovmskb r9d, xmm2
140009554: test r9d, r9d
140009557: je 0x1400095b0
140009559: mov r10, qword ptr [rbp + 0x678]
140009560: movq xmm2, qword ptr [r10]
140009565: tzcnt r10d, r9d
14000956a: add r10, r8
14000956d: and r10, rdx
140009570: shl r10, 3
140009574: mov r11, rcx
140009577: sub r11, r10
14000957a: movq xmm3, qword ptr [r11 - 8]
140009580: pcmpeqd xmm3, xmm2
140009584: pshufd xmm3, xmm3, 0x50
140009589: movmskpd r10d, xmm3
14000958e: mov r11b, 1
140009591: mov dword ptr [rbp + 0x538], r11d
140009598: cmp r10d, 3
14000959c: je 0x1400095df
14000959e: lea r10d, [r9 - 1]
1400095a2: and r10w, r9w
1400095a6: mov r9d, r10d
1400095a9: jne 0x140009565
1400095ab: nop dword ptr [rax + rax]
1400095b0: pcmpeqb xmm1, xmmword ptr [rip + 0x291c3d8]
1400095b8: pmovmskb r9d, xmm1
1400095bd: test r9d, r9d
1400095c0: jne 0x1400095d5
1400095c2: add r8, rax
1400095c5: add r8, 0x10
1400095c9: add rax, 0x10
1400095cd: and r8, rdx
1400095d0: jmp 0x140009541 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400095d5: mov dword ptr [rbp + 0x538], 0
1400095df: mov rdx, qword ptr [rbp + 0x7e0]
1400095e6: lea rax, [rdx + 0xd0]
1400095ed: mov qword ptr [rbp + 0x2e8], rax
1400095f4: cmp dword ptr [rax], 1
1400095f7: jne 0x140009613
1400095f9: movss xmm0, dword ptr [rdx + 0xd4]
140009601: movss dword ptr [rbp + 0x560], xmm0
140009609: mov dword ptr [rbp + 0x588], 1
140009613: xor eax, eax
140009615: cmp dword ptr [rdx + 0xf0], 1
14000961c: jne 0x140009633
14000961e: movss xmm0, dword ptr [rdx + 0xf4]
140009626: movss dword ptr [rbp + 0x568], xmm0
14000962e: mov eax, 1
140009633: mov dword ptr [rbp + 0x580], eax
140009639: mov rax, qword ptr [rbp + 0x90]
140009640: mov ecx, dword ptr [rbp + 0x238]
140009646: mov dword ptr [rax], ecx
140009648: mov r8d, dword ptr [rdx + 0x128]
14000964f: mov r9d, dword ptr [rdx + 0x12c]
140009656: mov rax, qword ptr [rbp + 0x128]
14000965d: mov qword ptr [rsp + 0x38], rax
140009662: mov rax, qword ptr [rbp + 0x130]
140009669: mov qword ptr [rsp + 0x30], rax
14000966e: mov rax, qword ptr [rbp + 0x120]
140009675: mov qword ptr [rsp + 0x28], rax
14000967a: mov qword ptr [rsp + 0x20], r13
14000967f: lea rcx, [rbp + 0x140]
140009686: mov rdx, qword ptr [rbp + 0x2f0]
14000968d: call 0x140aa8a60 ; country_core::resources::stairs::visual::StairsVisualState::reset_edge
140009692: mov r13, qword ptr [rbp + 0x7e0]
140009699: lea r15, [r13 + 0x130]
1400096a0: lea rax, [r13 + 0xf0]
1400096a7: mov qword ptr [rbp + 0x118], rax
1400096ae: mov r9, qword ptr [rbp + 0x570]
1400096b5: lea rax, [r9 + r9*2]
1400096b9: mov r10, qword ptr [rbp + 0x578]
1400096c0: lea rax, [r10 + rax*4]
1400096c4: mov rdx, qword ptr [r13 + 0x58]
1400096c8: mov rcx, qword ptr [r13 + 0x60]
1400096cc: lea r8, [rdx + rcx*4]
1400096d0: cmp rcx, r9
1400096d3: cmovae rcx, r9
1400096d7: mov qword ptr [rbp + 0x6a0], r10
1400096de: mov qword ptr [rbp + 0x6a8], rax
1400096e5: mov qword ptr [rbp + 0x6b0], rdx
1400096ec: mov qword ptr [rbp + 0x6b8], r8
1400096f3: mov qword ptr [rbp + 0x6c8], rcx
1400096fa: mov qword ptr [rbp + 0x6d0], r9
140009701: mov qword ptr [rbp + 0x6d8], 0
14000970c: xor eax, eax
14000970e: jmp 0x14000971e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009710: mov rax, qword ptr [rbp + 0x6c0]
140009717: mov rcx, qword ptr [rbp + 0x6c8]
14000971e: cmp rax, rcx
140009721: jae 0x1400099f3
140009727: lea rcx, [rax + 1]
14000972b: mov qword ptr [rbp + 0x6c0], rcx
140009732: lea rcx, [rax + rax*2]
140009736: shl rcx, 2
14000973a: add rcx, qword ptr [rbp + 0x6a0]
140009741: shl rax, 2
140009745: add rax, qword ptr [rbp + 0x6b0]
14000974c: cmp qword ptr [rbp + 0x6d8], 0
140009754: je 0x140009790
140009756: mov rdx, qword ptr [rbp + 0x6e8]
14000975d: mov r8, qword ptr [rbp + 0x6f0]
140009764: mov qword ptr [rbp + 0x6e8], rcx
14000976b: mov qword ptr [rbp + 0x6f0], rax
140009772: mov qword ptr [rbp + 0x6d8], rdx
140009779: mov qword ptr [rbp + 0x6e0], r8
140009780: jmp 0x140009805 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009785: nop word ptr cs:[rax + rax]
140009790: mov qword ptr [rbp + 0x460], 1
14000979b: mov qword ptr [rbp + 0x468], rcx
1400097a2: mov qword ptr [rbp + 0x470], rax
1400097a9: lea rax, [rbp + 0x6a0]
1400097b0: mov qword ptr [rbp + 0x478], rax
1400097b7: mov rcx, rbx
1400097ba: mov rdx, r12
1400097bd: call 0x1402e13b0 ; itertools::tuple_impl::impl$31::collect_from_iter_no_buf<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> >,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple$<ref$<glam::f32::vec3::Vec3>,ref$<f32> > >,ref_mut$<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<f32> > > > >
1400097c2: movups xmm0, xmmword ptr [rbp + 0x590]
1400097c9: movdqu xmm1, xmmword ptr [rbp + 0x5a0]
1400097d1: lea rax, [rbp + 0x6d8]
1400097d8: movdqu xmmword ptr [rax + 0x10], xmm1
1400097dd: movups xmmword ptr [rax], xmm0
1400097e0: mov rdx, qword ptr [rbp + 0x6d8]
1400097e7: test rdx, rdx
1400097ea: je 0x1400099f3
1400097f0: mov r8, qword ptr [rbp + 0x6e0]
1400097f7: mov rcx, qword ptr [rbp + 0x6e8]
1400097fe: mov rax, qword ptr [rbp + 0x6f0]
140009805: movd xmm0, dword ptr [rdx]
140009809: movss xmm1, dword ptr [rdx + 8]
14000980e: movd xmm2, dword ptr [rcx]
140009812: movss xmm3, dword ptr [rcx + 8]
140009817: movss xmm4, dword ptr [r8]
14000981c: movss xmm5, dword ptr [rax]
140009820: movaps xmm6, xmm4
140009823: cmpunordss xmm6, xmm4
140009828: movaps xmm7, xmm6
14000982b: andps xmm7, xmm5
14000982e: maxss xmm5, xmm4
140009832: andnps xmm6, xmm5
140009835: orps xmm6, xmm7
140009838: mulss xmm6, dword ptr [rip + 0x291c030]
140009840: mov rax, qword ptr [rbp + 0x1f8]
140009847: mov qword ptr [rbp + 0x460], rax
14000984e: mulss xmm6, dword ptr [rip + 0x291c01e]
140009856: addss xmm6, dword ptr [rip + 0x291c01a]
14000985e: movd dword ptr [rbp + 0x590], xmm0
140009866: movss dword ptr [rbp + 0x594], xmm1
14000986e: movd dword ptr [rbp + 0x598], xmm2
140009876: movss dword ptr [rbp + 0x59c], xmm3
14000987e: xor eax, eax
140009880: xor esi, esi
140009882: nop word ptr cs:[rax + rax]
140009890: cmp rsi, 2
140009894: je 0x140009710
14000989a: lea rcx, [rsi + 1]
14000989e: movd xmm0, dword ptr [rbp + rsi*8 + 0x590]
1400098a7: movd xmm2, dword ptr [rbp + rsi*8 + 0x594]
1400098b0: test al, 1
1400098b2: je 0x1400098d0
1400098b4: mov rsi, rcx
1400098b7: movdqa xmm12, xmm0
1400098bc: movdqa xmm13, xmm11
1400098c1: movdqa xmm0, xmm1
1400098c5: movdqa xmm11, xmm2
1400098ca: jmp 0x140009903 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400098cc: nop dword ptr [rax]
1400098d0: cmp rcx, 2
1400098d4: je 0x140009710
1400098da: mov qword ptr [rbp + 0x5a0], 2
1400098e5: movd xmm12, dword ptr [rbp + rsi*8 + 0x598]
1400098ef: movd xmm11, dword ptr [rbp + rsi*8 + 0x59c]
1400098f9: mov esi, 2
1400098fe: movdqa xmm13, xmm2
140009903: mov byte ptr [rbp + 0x380], 0
14000990a: movss xmm7, dword ptr [rip + 0x291bf62]
140009912: movaps xmm1, xmm7
140009915: movd xmm8, dword ptr [rip + 0x291bf5e]
14000991e: movdqa xmm2, xmm8
140009923: call 0x142923ef0 ; ?
140009928: movdqa xmm14, xmm0
14000992d: movdqa xmm0, xmm13
140009932: movaps xmm1, xmm7
140009935: movdqa xmm2, xmm8
14000993a: call 0x142923ef0 ; ?
14000993f: movd ecx, xmm14
140009944: movd eax, xmm0
140009948: shl rax, 0x20
14000994c: or rcx, rax
14000994f: call 0x140b84e40 ; conservative_line_raster::impl$0::from
140009954: movdqa xmm13, xmm0
140009959: movaps xmm14, xmm1
14000995d: movdqa xmm0, xmm12
140009962: movss xmm7, dword ptr [rip + 0x291bf0a]
14000996a: movaps xmm1, xmm7
14000996d: movss xmm8, dword ptr [rip + 0x291bf06]
140009976: movaps xmm2, xmm8
14000997a: call 0x142923ef0 ; ?
14000997f: movdqa xmm15, xmm0
140009984: movdqa xmm0, xmm11
140009989: movaps xmm1, xmm7
14000998c: movaps xmm2, xmm8
140009990: call 0x142923ef0 ; ?
140009995: movd ecx, xmm15
14000999a: xorps xmm8, xmm8
14000999e: movd eax, xmm0
1400099a2: shl rax, 0x20
1400099a6: or rcx, rax
1400099a9: call 0x140b84e40 ; conservative_line_raster::impl$0::from
1400099ae: movdqa xmm2, xmm0
1400099b2: movaps xmm3, xmm1
1400099b5: lea rax, [rbp + 0x380]
1400099bc: mov qword ptr [rsp + 0x30], rax
1400099c1: mov qword ptr [rsp + 0x28], r12
1400099c6: movss dword ptr [rsp + 0x20], xmm6
1400099cc: movdqa xmm0, xmm13
1400099d1: movaps xmm1, xmm14
1400099d5: call 0x1402582d0 ; conservative_line_raster::conservative_plot_line<country_core::utils::dirty_tiles::impl$0::iter_world_space_stroke_cells::closure_env$0<array$<glam::f32::vec2::Vec2,2>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$6<false> > >
1400099da: mov al, 1
1400099dc: cmp byte ptr [rbp + 0x380], 1
1400099e3: movdqa xmm1, xmm12
1400099e8: jne 0x140009890
1400099ee: jmp 0x140009710 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400099f3: mov eax, dword ptr [rbp + 0x23c]
1400099f9: mov rcx, qword ptr [rbp + 0xa0]
140009a00: mov dword ptr [rcx], eax
140009a02: mov edx, 1
140009a07: mov rcx, qword ptr [rbp + 0x98]
140009a0e: mov r8, qword ptr [rbp + 0x438]
140009a15: call 0x1400cce50 ; country_core::resources::rt::RtWorld::get_mut<enum2$<country_core::resources::rt::RtMeshHandle> >
140009a1a: mov qword ptr [rbp + 0x648], rax
140009a21: mov rax, qword ptr [rbp + 0x648]
140009a28: mov qword ptr [rax + 0x50], 0
140009a30: mov rax, qword ptr [rbp + 0x758]
140009a37: add eax, 0xfffe7960
140009a3c: cmp eax, 0xb
140009a3f: ja 0x140009a70
140009a41: lea rcx, [rip + 0x291de38]
140009a48: movsxd rax, dword ptr [rcx + rax*4]
140009a4c: add rax, rcx
140009a4f: jmp rax
140009a51: movsd xmm0, qword ptr [rip + 0x291be67]
140009a59: movsd qword ptr [rbp + 0x590], xmm0
140009a61: mov dword ptr [rbp + 0x598], 0x3e2e147b
140009a6b: jmp 0x140009af5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009a70: mov dword ptr [rbp + 0x598], 0
140009a7a: mov qword ptr [rbp + 0x590], 0
140009a85: jmp 0x140009af5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009a87: movsd xmm0, qword ptr [rip + 0x291be11]
140009a8f: movsd qword ptr [rbp + 0x590], xmm0
140009a97: mov dword ptr [rbp + 0x598], 0x3e051eb8
140009aa1: jmp 0x140009af5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009aa3: movsd xmm0, qword ptr [rip + 0x291be05]
140009aab: movsd qword ptr [rbp + 0x590], xmm0
140009ab3: mov dword ptr [rbp + 0x598], 0x3e0f5c29
140009abd: jmp 0x140009af5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009abf: movsd xmm0, qword ptr [rip + 0x291bdc9]
140009ac7: movsd qword ptr [rbp + 0x590], xmm0
140009acf: mov dword ptr [rbp + 0x598], 0x3e570a3d
140009ad9: jmp 0x140009af5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009adb: movsd xmm0, qword ptr [rip + 0x291bd9d]
140009ae3: movsd qword ptr [rbp + 0x590], xmm0
140009aeb: mov dword ptr [rbp + 0x598], 0x3e2147ae
140009af5: lea rcx, [rbp + 0x6a0]
140009afc: mov rdx, rbx
140009aff: call 0x1408bc460 ; country_core::resources::rt::impl$1::from
140009b04: test byte ptr [rbp + 0x6a0], 1
140009b0b: je 0x140009b2a
140009b0d: mov rax, qword ptr [rbp + 0x6a8]
140009b14: mov qword ptr [rbp + 0x460], rax
140009b1b: mov rcx, r12
140009b1e: call 0x1400885d0 ; minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction
140009b23: or eax, 0x1000000
140009b28: jmp 0x140009b30 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009b2a: mov eax, dword ptr [rbp + 0x6a4]
140009b30: mov rcx, qword ptr [rbp + 0x648]
140009b37: mov dword ptr [rcx + 0x58], eax
140009b3a: mov qword ptr [rbp + 0x440], 0
140009b45: lea rax, [rbp + 0x450]
140009b4c: movups xmmword ptr [rax], xmm8
140009b50: mov qword ptr [rbp + 0x448], 8
140009b5b: mov qword ptr [rbp + 0x278], 0
140009b66: mov qword ptr [rbp + 0x280], 0x10
140009b71: mov qword ptr [rbp + 0x288], 0
140009b7c: mov rax, qword ptr [r13 + 0x140]
140009b83: cmp rax, 2
140009b87: jb 0x140009b97
140009b89: mov r15, qword ptr [r13 + 0x130]
140009b90: mov rax, qword ptr [r13 + 0x138]
140009b97: add qword ptr [rbp + 0x648], 0x40
140009b9f: lea rax, [r15 + rax*4]
140009ba3: mov qword ptr [rbp + 0x520], rax
140009baa: mov rax, qword ptr [rbp + 0x570]
140009bb1: dec rax
140009bb4: mov qword ptr [rbp + 0x668], rax
140009bbb: mov rax, r15
140009bbe: xor r15d, r15d
140009bc1: mov qword ptr [rbp + 0x88], rdi
140009bc8: jmp 0x140009be5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009bca: nop word ptr [rax + rax]
140009bd0: mov r13, qword ptr [rbp + 0x7e0]
140009bd7: mov rax, qword ptr [rbp + 0x778]
140009bde: mov r15, qword ptr [rbp + 0x7b0]
140009be5: cmp rax, qword ptr [rbp + 0x520]
140009bec: je 0x14000df2d
140009bf2: movzx r8d, word ptr [rax + 2]
140009bf7: lea r11, [r15 + r8]
140009bfb: test r15, r15
140009bfe: sete sil
140009c02: cmp r11, qword ptr [rbp + 0x668]
140009c09: sete bl
140009c0c: cmp r11, -1
140009c10: je 0x140010f19
140009c16: lea rcx, [r11 + 1]
140009c1a: mov rdi, rcx
140009c1d: sub rdi, r15
140009c20: jb 0x140010f31
140009c26: cmp r11, qword ptr [rbp + 0x570]
140009c2d: jae 0x140010f4f
140009c33: mov rdx, qword ptr [r13 + 0x78]
140009c37: cmp r11, rdx
140009c3a: jae 0x140010f6e
140009c40: mov rdx, qword ptr [r13 + 0x60]
140009c44: cmp r11, rdx
140009c47: jae 0x140010f86
140009c4d: lea rdx, [rax + 4]
140009c51: mov qword ptr [rbp + 0x778], rdx
140009c58: lea rdx, [r15 + r15*2]
140009c5c: mov r9, qword ptr [rbp + 0x578]
140009c63: lea r12, [r9 + rdx*4]
140009c67: lea rdx, [r15*8]
140009c6f: add rdx, qword ptr [r13 + 0x70]
140009c73: mov qword ptr [rbp + 0x800], rdx
140009c7a: lea rdx, [r15*4]
140009c82: add rdx, qword ptr [r13 + 0x58]
140009c86: mov qword ptr [rbp + 0x780], rdx
140009c8d: mov rdx, qword ptr [r13 + 0x90]
140009c94: test rdx, rdx
140009c97: mov qword ptr [rbp + 0x7b0], r11
140009c9e: mov byte ptr [rbp + 0x7a0], bl
140009ca4: mov byte ptr [rbp + 0x7a8], sil
140009cab: je 0x140009d20
140009cad: cmp r11, rdx
140009cb0: pxor xmm1, xmm1
140009cb4: mov r10, qword ptr [rbp + 0x678]
140009cbb: jae 0x140010fe9
140009cc1: mov r9, qword ptr [r13 + 0x88]
140009cc8: movzx edx, byte ptr [rax]
140009ccb: movzx eax, byte ptr [rax + 1]
140009ccf: mov byte ptr [rbp + 0x80f], dl
140009cd5: mov byte ptr [rbp + 0x810], al
140009cdb: cmp rcx, r15
140009cde: je 0x140009d3a
140009ce0: movzx eax, dl
140009ce3: lea rcx, [r9 + r15*4]
140009ce7: lea r8, [r8*4 + 4]
140009cef: xor r9d, r9d
140009cf2: nop word ptr cs:[rax + rax]
140009d00: cmp r8, r9
140009d03: je 0x14000a2b4
140009d09: movss xmm0, dword ptr [rcx + r9]
140009d0f: add r9, 4
140009d13: ucomiss xmm0, xmm1
140009d16: ja 0x140009d00
140009d18: jmp 0x140009d3a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009d1a: nop word ptr [rax + rax]
140009d20: movzx edx, byte ptr [rax]
140009d23: movzx eax, byte ptr [rax + 1]
140009d27: mov byte ptr [rbp + 0x80f], dl
140009d2d: mov byte ptr [rbp + 0x810], al
140009d33: mov r10, qword ptr [rbp + 0x678]
140009d3a: lea rax, [rbp + 0x80f]
140009d41: mov qword ptr [rbp + 0x2a0], rax
140009d48: mov rax, qword ptr [rbp + 0x958]
140009d4f: mov qword ptr [rbp + 0x2a8], rax
140009d56: mov qword ptr [rbp + 0x2b0], r10
140009d5d: mov rax, qword ptr [rbp + 0x248]
140009d64: mov qword ptr [rbp + 0x2b8], rax
140009d6b: cmp dl, 3
140009d6e: je 0x14000a79b
140009d74: mov qword ptr [rbp + 0x670], r15
140009d7b: mov rax, qword ptr [rbp + 0x200]
140009d82: mov byte ptr [rbp + 0x817], 1
140009d89: mov qword ptr [rsp + 0x48], rax
140009d8e: mov rax, qword ptr [rbp + 0x648]
140009d95: mov qword ptr [rsp + 0x40], rax
140009d9a: lea rax, [rbp + 0x278]
140009da1: mov qword ptr [rsp + 0x38], rax
140009da6: mov qword ptr [rsp + 0x28], rdi
140009dab: mov rax, qword ptr [rbp + 0x780]
140009db2: mov qword ptr [rsp + 0x20], rax
140009db7: mov qword ptr [rsp + 0x30], 1
140009dc0: mov r14, r12
140009dc3: mov rcx, r12
140009dc6: mov rdx, rdi
140009dc9: mov r8, qword ptr [rbp + 0x800]
140009dd0: mov r9, rdi
140009dd3: call 0x1407de8b0 ; system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo
140009dd8: mov rax, rdi
140009ddb: mov ecx, 0xc
140009de0: mul rcx
140009de3: mov r12, rax
140009de6: jo 0x140011358
140009dec: mov ebx, 0
140009df1: movabs rax, 0x7ffffffffffffffc
140009dfb: cmp r12, rax
140009dfe: ja 0x14001135a
140009e04: movzx eax, byte ptr [rbp + 0x80f]
140009e0b: mov byte ptr [rbp + 0x760], al
140009e11: test r12, r12
140009e14: je 0x140009e40
140009e16: movzx eax, byte ptr [rip + 0x362adec]
140009e1d: mov ebx, 4
140009e22: mov edx, 4
140009e27: mov rcx, r12
140009e2a: call 0x140613c10 ; ?
140009e2f: test rax, rax
140009e32: je 0x14001135a
140009e38: mov rsi, rax
140009e3b: mov rbx, rdi
140009e3e: jmp 0x140009e47 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009e40: mov esi, 4
140009e45: xor ebx, ebx
140009e47: mov rcx, rsi
140009e4a: mov rdx, r14
140009e4d: mov r8, r12
140009e50: call 0x142923c00 ; ?
140009e55: mov qword ptr [rbp + 0x590], rbx
140009e5c: mov qword ptr [rbp + 0x598], rsi
140009e63: mov qword ptr [rbp + 0x5a0], rdi
140009e6a: mov byte ptr [rbp + 0x817], 1
140009e71: lea rcx, [rbp + 0x6a0]
140009e78: lea rdx, [rbp + 0x590]
140009e7f: mov r8d, 1
140009e85: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
140009e8a: xor r14d, r14d
140009e8d: cmp r14, qword ptr [rbp + 0x6a0]
140009e94: jo 0x140009bd0
140009e9a: mov rax, qword ptr [rbp + 0x6d0]
140009ea1: mov qword ptr [rbp + 0x3b0], rax
140009ea8: movups xmm0, xmmword ptr [rbp + 0x6a0]
140009eaf: movups xmm1, xmmword ptr [rbp + 0x6b0]
140009eb6: movdqu xmm2, xmmword ptr [rbp + 0x6c0]
140009ebe: movdqa xmmword ptr [rbp + 0x3a0], xmm2
140009ec6: movaps xmmword ptr [rbp + 0x390], xmm1
140009ecd: movaps xmmword ptr [rbp + 0x380], xmm0
140009ed4: mov rbx, qword ptr [rbp + 0x390]
140009edb: mov rax, rbx
140009ede: mov ecx, 0xc
140009ee3: mul rcx
140009ee6: mov r12, rax
140009ee9: jo 0x1400113ae
140009eef: mov esi, 0
140009ef4: movabs rax, 0x7ffffffffffffffc
140009efe: cmp r12, rax
140009f01: ja 0x1400113b0
140009f07: mov r14, qword ptr [rbp + 0x388]
140009f0e: test r12, r12
140009f11: je 0x140009f3e
140009f13: movzx eax, byte ptr [rip + 0x362acef]
140009f1a: mov esi, 4
140009f1f: mov edx, 4
140009f24: mov rcx, r12
140009f27: call 0x140613c10 ; ?
140009f2c: test rax, rax
140009f2f: je 0x1400113b0
140009f35: mov qword ptr [rbp + 0x7f0], rbx
140009f3c: jmp 0x140009f4e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009f3e: mov eax, 4
140009f43: mov qword ptr [rbp + 0x7f0], 0
140009f4e: mov qword ptr [rbp + 0x798], rax
140009f55: mov rcx, rax
140009f58: mov rdx, r14
140009f5b: mov r8, r12
140009f5e: call 0x142923c00 ; ?
140009f63: mov r15, qword ptr [rbp + 0x3a8]
140009f6a: lea r12, [r15*4]
140009f72: mov rax, r15
140009f75: shr rax, 0x3e
140009f79: jne 0x1400113c7
140009f7f: mov r14d, 0
140009f85: movabs rax, 0x7ffffffffffffffc
140009f8f: cmp r12, rax
140009f92: ja 0x1400113ca
140009f98: mov r13, qword ptr [rbp + 0x3a0]
140009f9f: test r12, r12
140009fa2: je 0x140009fcf
140009fa4: movzx eax, byte ptr [rip + 0x362ac5e]
140009fab: mov r14d, 4
140009fb1: mov edx, 4
140009fb6: mov rcx, r12
140009fb9: call 0x140613c10 ; ?
140009fbe: test rax, rax
140009fc1: je 0x1400113ca
140009fc7: mov rsi, rax
140009fca: mov r14, r15
140009fcd: jmp 0x140009fd7 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140009fcf: mov esi, 4
140009fd4: xor r14d, r14d
140009fd7: mov rcx, rsi
140009fda: mov rdx, r13
140009fdd: mov r8, r12
140009fe0: call 0x142923c00 ; ?
140009fe5: movss xmm0, dword ptr [rbp + 0x3b0]
140009fed: mov rax, qword ptr [rbp + 0x7f0]
140009ff4: mov qword ptr [rbp + 0x6a0], rax
140009ffb: mov rax, qword ptr [rbp + 0x798]
14000a002: mov qword ptr [rbp + 0x6a8], rax
14000a009: mov qword ptr [rbp + 0x6b0], rbx
14000a010: mov qword ptr [rbp + 0x6b8], r14
14000a017: mov qword ptr [rbp + 0x6c0], rsi
14000a01e: mov qword ptr [rbp + 0x6c8], r15
14000a025: movss dword ptr [rbp + 0x6d0], xmm0
14000a02d: lea rcx, [rbp + 0x590]
14000a034: lea rdx, [rbp + 0x6a0]
14000a03b: movd xmm2, dword ptr [rip + 0x291b82d]
14000a043: mov r9d, 1
14000a049: call 0x140152b00 ; utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000a04e: mov rax, qword ptr [rbp + 0x590]
14000a055: mov rcx, rax
14000a058: neg rcx
14000a05b: mov r14d, 0
14000a061: mov r13, qword ptr [rbp + 0x7e0]
14000a068: jno 0x14000a110
14000a06e: mov rax, qword ptr [rbp + 0x6a0]
14000a075: test rax, rax
14000a078: je 0x14000a094
14000a07a: mov rcx, qword ptr [rbp + 0x6a8]
14000a081: shl rax, 2
14000a085: lea rdx, [rax + rax*2]
14000a089: mov r8d, 4
14000a08f: call 0x140613c20 ; ?
14000a094: mov rdx, qword ptr [rbp + 0x6b8]
14000a09b: test rdx, rdx
14000a09e: je 0x14000a0b6
14000a0a0: mov rcx, qword ptr [rbp + 0x6c0]
14000a0a7: shl rdx, 2
14000a0ab: mov r8d, 4
14000a0b1: call 0x140613c20 ; ?
14000a0b6: mov rax, qword ptr [rbp + 0x380]
14000a0bd: test rax, rax
14000a0c0: je 0x14000a0dc
14000a0c2: mov rcx, qword ptr [rbp + 0x388]
14000a0c9: shl rax, 2
14000a0cd: lea rdx, [rax + rax*2]
14000a0d1: mov r8d, 4
14000a0d7: call 0x140613c20 ; ?
14000a0dc: mov rdx, qword ptr [rbp + 0x398]
14000a0e3: test rdx, rdx
14000a0e6: je 0x140009bd7
14000a0ec: mov rcx, qword ptr [rbp + 0x3a0]
14000a0f3: shl rdx, 2
14000a0f7: mov r8d, 4
14000a0fd: call 0x140613c20 ; ?
14000a102: jmp 0x140009bd7 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000a107: nop word ptr [rax + rax]
14000a110: lea rcx, [rbp + 0x598]
14000a117: movups xmm0, xmmword ptr [rcx]
14000a11a: movups xmm1, xmmword ptr [rcx + 0x10]
14000a11e: movdqu xmm2, xmmword ptr [rcx + 0x20]
14000a123: lea rcx, [rbp + 0x468]
14000a12a: movdqu xmmword ptr [rcx + 0x20], xmm2
14000a12f: movups xmmword ptr [rcx + 0x10], xmm1
14000a133: movups xmmword ptr [rcx], xmm0
14000a136: mov qword ptr [rbp + 0x460], rax
14000a13d: mov rax, qword ptr [rbp + 0x6a0]
14000a144: test rax, rax
14000a147: je 0x14000a163
14000a149: mov rcx, qword ptr [rbp + 0x6a8]
14000a150: shl rax, 2
14000a154: lea rdx, [rax + rax*2]
14000a158: mov r8d, 4
14000a15e: call 0x140613c20 ; ?
14000a163: mov rdx, qword ptr [rbp + 0x6b8]
14000a16a: test rdx, rdx
14000a16d: je 0x14000a185
14000a16f: mov rcx, qword ptr [rbp + 0x6c0]
14000a176: shl rdx, 2
14000a17a: mov r8d, 4
14000a180: call 0x140613c20 ; ?
14000a185: mov r14, qword ptr [rbp + 0x470]
14000a18c: test r14, r14
14000a18f: je 0x14000a30d
14000a195: lea rsi, [r14*4]
14000a19d: movzx eax, byte ptr [rip + 0x362aa65]
14000a1a4: mov edx, 4
14000a1a9: mov rcx, rsi
14000a1ac: call 0x140613c10 ; ?
14000a1b1: test rax, rax
14000a1b4: je 0x1400113e1
14000a1ba: mov rbx, qword ptr [rbp + 0x470]
14000a1c1: mov qword ptr [rbp + 0x650], r14
14000a1c8: mov qword ptr [rbp + 0x658], rax
14000a1cf: mov qword ptr [rbp + 0x660], 0
14000a1da: test rbx, rbx
14000a1dd: je 0x14000a32e
14000a1e3: lea rsi, [rbx*8]
14000a1eb: movzx eax, byte ptr [rip + 0x362aa17]
14000a1f2: mov edx, 4
14000a1f7: mov rcx, rsi
14000a1fa: call 0x140613c10 ; ?
14000a1ff: test rax, rax
14000a202: je 0x14001145c
14000a208: mov r14, qword ptr [rbp + 0x470]
14000a20f: mov qword ptr [rbp + 0x680], rbx
14000a216: mov qword ptr [rbp + 0x688], rax
14000a21d: mov qword ptr [rbp + 0x690], 0
14000a228: test r14, r14
14000a22b: je 0x14000a34f
14000a231: lea rax, [r14*4]
14000a239: lea rsi, [rax + rax*2]
14000a23d: movzx eax, byte ptr [rip + 0x362a9c5]
14000a244: mov edx, 4
14000a249: mov rcx, rsi
14000a24c: call 0x140613c10 ; ?
14000a251: test rax, rax
14000a254: je 0x140011490
14000a25a: mov rbx, qword ptr [rbp + 0x470]
14000a261: mov qword ptr [rbp + 0x500], r14
14000a268: mov qword ptr [rbp + 0x508], rax
14000a26f: mov qword ptr [rbp + 0x510], 0
14000a27a: test rbx, rbx
14000a27d: je 0x14000a370
14000a283: lea rsi, [rbx*4]
14000a28b: movzx eax, byte ptr [rip + 0x362a977]
14000a292: mov edx, 4
14000a297: mov rcx, rsi
14000a29a: call 0x140613c10 ; ?
14000a29f: test rax, rax
14000a2a2: je 0x1400114a9
14000a2a8: mov rsi, qword ptr [rbp + 0x470]
14000a2af: jmp 0x14000a379 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000a2b4: lea ecx, [rdx - 2]
14000a2b7: dec rax
14000a2ba: cmp cl, 3
14000a2bd: cmovae rax, r14
14000a2c1: test rax, rax
14000a2c4: je 0x14000a2d0
14000a2c6: cmp rax, 3
14000a2ca: jne 0x140009d3a
14000a2d0: mov byte ptr [rbp + 0x80f], 2
14000a2d7: lea rax, [rbp + 0x80f]
14000a2de: mov qword ptr [rbp + 0x2a0], rax
14000a2e5: mov rax, qword ptr [rbp + 0x958]
14000a2ec: mov qword ptr [rbp + 0x2a8], rax
14000a2f3: mov qword ptr [rbp + 0x2b0], r10
14000a2fa: mov rax, qword ptr [rbp + 0x248]
14000a301: mov qword ptr [rbp + 0x2b8], rax
14000a308: jmp 0x140009d74 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000a30d: mov qword ptr [rbp + 0x650], 0
14000a318: mov qword ptr [rbp + 0x658], 4
14000a323: mov qword ptr [rbp + 0x660], 0
14000a32e: mov qword ptr [rbp + 0x680], 0
14000a339: mov qword ptr [rbp + 0x688], 4
14000a344: mov qword ptr [rbp + 0x690], 0
14000a34f: mov qword ptr [rbp + 0x500], 0
14000a35a: mov qword ptr [rbp + 0x508], 4
14000a365: mov qword ptr [rbp + 0x510], 0
14000a370: mov eax, 4
14000a375: xor ebx, ebx
14000a377: xor esi, esi
14000a379: mov qword ptr [rbp + 0x590], rbx
14000a380: mov qword ptr [rbp + 0x598], rax
14000a387: mov qword ptr [rbp + 0x5a0], 0
14000a392: mov r14, qword ptr [rbp + 0x468]
14000a399: mov rax, qword ptr [rbp + 0x480]
14000a3a0: mov qword ptr [rbp + 0x7f0], rax
14000a3a7: mov rax, qword ptr [rbp + 0x488]
14000a3ae: cmp rax, rsi
14000a3b1: cmovb rsi, rax
14000a3b5: xor ebx, ebx
14000a3b7: xor r13d, r13d
14000a3ba: jmp 0x14000a3db ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000a3bc: nop dword ptr [rax]
14000a3c0: inc r13
14000a3c3: mov rax, qword ptr [rbp + 0x598]
14000a3ca: mov dword ptr [rax + rbx*4], 0
14000a3d1: inc rbx
14000a3d4: mov qword ptr [rbp + 0x5a0], rbx
14000a3db: cmp r13, rsi
14000a3de: jae 0x14000a5d0
14000a3e4: lea rax, [r13*2]
14000a3ec: add rax, r13
14000a3ef: mov ebx, dword ptr [r14 + rax*4]
14000a3f3: movss xmm6, dword ptr [r14 + rax*4 + 4]
14000a3fa: mov r15d, dword ptr [r14 + rax*4 + 8]
14000a3ff: mov rax, qword ptr [rbp + 0x7f0]
14000a406: movss xmm1, dword ptr [rax + r13*4]
14000a40c: lea rcx, [rbp + 0x380]
14000a413: call 0x140153380 ; utils::curve::Curve::get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D> >
14000a418: mov ecx, eax
14000a41a: cmp rdi, rcx
14000a41d: jbe 0x1400111e7
14000a423: lea rax, [rcx + 1]
14000a427: cmp rax, rdi
14000a42a: jae 0x1400111d0
14000a430: movaps xmm11, xmm0
14000a434: mov rax, qword ptr [rbp + 0x800]
14000a43b: movsd xmm7, qword ptr [rax + rcx*8]
14000a440: movsd xmm8, qword ptr [rax + rcx*8 + 8]
14000a447: mov rax, qword ptr [rbp + 0x780]
14000a44e: movss xmm10, dword ptr [rax + rcx*4]
14000a454: movss xmm9, dword ptr [rax + rcx*4 + 4]
14000a45b: cmp byte ptr [rbp + 0x760], 2
14000a462: jae 0x14000a4c9
14000a464: movd xmm1, ebx
14000a468: movd xmm2, r15d
14000a46d: mov rdx, qword ptr [rbp + 0x240]
14000a474: mov rax, qword ptr [rdx + 8]
14000a478: mov rcx, qword ptr [rdx + 0x10]
14000a47c: add rcx, rcx
14000a47f: mov qword ptr [rbp + 0x6a0], rax
14000a486: mov qword ptr [rbp + 0x6a8], rcx
14000a48d: movsd xmm0, qword ptr [rdx + 0x50]
14000a492: movsd qword ptr [rbp + 0x6b0], xmm0
14000a49a: lea rcx, [rbp + 0x6a0]
14000a4a1: call 0x140257ba0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightFloatGrid>
14000a4a6: addss xmm0, dword ptr [rip + 0x291b472]
14000a4ae: movaps xmm1, xmm6
14000a4b1: cmpunordss xmm1, xmm6
14000a4b6: movaps xmm2, xmm1
14000a4b9: andps xmm2, xmm0
14000a4bc: maxss xmm0, xmm6
14000a4c0: andnps xmm1, xmm0
14000a4c3: orps xmm1, xmm2
14000a4c6: movaps xmm6, xmm1
14000a4c9: mov r12, qword ptr [rbp + 0x660]
14000a4d0: cmp r12, qword ptr [rbp + 0x650]
14000a4d7: jne 0x14000a4ec
14000a4d9: lea rcx, [rbp + 0x650]
14000a4e0: lea rdx, [rip + 0x291bec1]
14000a4e7: call 0x140650050 ; alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>
14000a4ec: subss xmm9, xmm10
14000a4f1: mulss xmm9, xmm11
14000a4f6: addss xmm9, xmm10
14000a4fb: mov rax, qword ptr [rbp + 0x658]
14000a502: movss dword ptr [rax + r12*4], xmm9
14000a508: inc r12
14000a50b: mov qword ptr [rbp + 0x660], r12
14000a512: mov r12, qword ptr [rbp + 0x690]
14000a519: cmp r12, qword ptr [rbp + 0x680]
14000a520: jne 0x14000a535
14000a522: lea rcx, [rbp + 0x680]
14000a529: lea rdx, [rip + 0x291be90]
14000a530: call 0x140b81f90 ; alloc::raw_vec::RawVec::grow_one<tuple$<u8,u32>,alloc::alloc::Global>
14000a535: subps xmm8, xmm7
14000a539: movsldup xmm0, xmm11
14000a53e: mulps xmm0, xmm8
14000a542: addps xmm0, xmm7
14000a545: mov rax, qword ptr [rbp + 0x688]
14000a54c: movlps qword ptr [rax + r12*8], xmm0
14000a551: inc r12
14000a554: mov qword ptr [rbp + 0x690], r12
14000a55b: mov r12, qword ptr [rbp + 0x510]
14000a562: cmp r12, qword ptr [rbp + 0x500]
14000a569: jne 0x14000a57e
14000a56b: lea rcx, [rbp + 0x500]
14000a572: lea rdx, [rip + 0x291be5f]
14000a579: call 0x140b9f180 ; alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>
14000a57e: mov rax, qword ptr [rbp + 0x508]
14000a585: lea rcx, [r12 + r12*2]
14000a589: mov dword ptr [rax + rcx*4], ebx
14000a58c: movss dword ptr [rax + rcx*4 + 4], xmm6
14000a592: mov dword ptr [rax + rcx*4 + 8], r15d
14000a597: inc r12
14000a59a: mov qword ptr [rbp + 0x510], r12
14000a5a1: mov rbx, qword ptr [rbp + 0x5a0]
14000a5a8: cmp rbx, qword ptr [rbp + 0x590]
14000a5af: jne 0x14000a3c0
14000a5b5: lea rcx, [rbp + 0x590]
14000a5bc: lea rdx, [rip + 0x291be2d]
14000a5c3: call 0x140650050 ; alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>
14000a5c8: jmp 0x14000a3c0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000a5cd: nop dword ptr [rax]
14000a5d0: mov rsi, qword ptr [rbp + 0x650]
14000a5d7: lea rax, [rbp + 0x658]
14000a5de: movups xmm0, xmmword ptr [rax]
14000a5e1: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000a5e8: mov rax, qword ptr [rbp + 0x690]
14000a5ef: lea rcx, [rbp + 0x6d8]
14000a5f6: mov qword ptr [rcx - 0x18], rax
14000a5fa: movups xmm0, xmmword ptr [rbp + 0x680]
14000a601: movups xmmword ptr [rcx - 0x28], xmm0
14000a605: movups xmm0, xmmword ptr [rbp + 0x500]
14000a60c: movaps xmmword ptr [rbp + 0x310], xmm0
14000a613: mov rax, qword ptr [rbp + 0x510]
14000a61a: mov qword ptr [rbp + 0x320], rax
14000a621: mov rdi, qword ptr [rbp + 0x590]
14000a628: mov r15, qword ptr [rbp + 0x598]
14000a62f: movaps xmm0, xmmword ptr [rbp + 0x6a0]
14000a636: movaps xmm1, xmmword ptr [rbp + 0x6b0]
14000a63d: movaps xmmword ptr [rbp + 0x3e0], xmm0
14000a644: movaps xmmword ptr [rbp + 0x3f0], xmm1
14000a64b: mov rax, qword ptr [rbp + 0x6c0]
14000a652: mov qword ptr [rbp + 0x400], rax
14000a659: mov rax, qword ptr [rbp + 0x460]
14000a660: test rax, rax
14000a663: je 0x14000a67f
14000a665: mov rcx, qword ptr [rbp + 0x468]
14000a66c: shl rax, 2
14000a670: lea rdx, [rax + rax*2]
14000a674: mov r8d, 4
14000a67a: call 0x140613c20 ; ?
14000a67f: mov rdx, qword ptr [rbp + 0x478]
14000a686: test rdx, rdx
14000a689: mov r14d, 0
14000a68f: mov r13, qword ptr [rbp + 0x7e0]
14000a696: je 0x14000a6ae
14000a698: mov rcx, qword ptr [rbp + 0x480]
14000a69f: shl rdx, 2
14000a6a3: mov r8d, 4
14000a6a9: call 0x140613c20 ; ?
14000a6ae: mov rax, qword ptr [rbp + 0x380]
14000a6b5: test rax, rax
14000a6b8: je 0x14000a6d4
14000a6ba: mov rcx, qword ptr [rbp + 0x388]
14000a6c1: shl rax, 2
14000a6c5: lea rdx, [rax + rax*2]
14000a6c9: mov r8d, 4
14000a6cf: call 0x140613c20 ; ?
14000a6d4: mov rdx, qword ptr [rbp + 0x398]
14000a6db: test rdx, rdx
14000a6de: je 0x14000a6f6
14000a6e0: mov rcx, qword ptr [rbp + 0x3a0]
14000a6e7: shl rdx, 2
14000a6eb: mov r8d, 4
14000a6f1: call 0x140613c20 ; ?
14000a6f6: mov rax, rsi
14000a6f9: neg rax
14000a6fc: jo 0x140009bd7
14000a702: movaps xmm0, xmmword ptr [rbp + 0x3e0]
14000a709: lea rax, [rbp + 0x550]
14000a710: movups xmmword ptr [rax], xmm0
14000a713: lea rcx, [rbp + 0x3f0]
14000a71a: mov rax, qword ptr [rcx + 0x10]
14000a71e: mov qword ptr [rbp + 0x660], rax
14000a725: movups xmm0, xmmword ptr [rcx]
14000a728: movaps xmmword ptr [rbp + 0x650], xmm0
14000a72f: mov rax, qword ptr [rbp + 0x320]
14000a736: mov qword ptr [rbp + 0x690], rax
14000a73d: movaps xmm0, xmmword ptr [rbp + 0x310]
14000a744: movaps xmmword ptr [rbp + 0x680], xmm0
14000a74b: mov qword ptr [rbp + 0x548], rsi
14000a752: movzx esi, byte ptr [rbp + 0x80f]
14000a759: lea eax, [rsi - 2]
14000a75c: mov rcx, rsi
14000a75f: dec rcx
14000a762: cmp al, 3
14000a764: cmovae rcx, r14
14000a768: lea rdx, [rip + 0x291d141]
14000a76f: movsxd rax, dword ptr [rdx + rcx*4]
14000a773: add rax, rdx
14000a776: mov qword ptr [rbp + 0x798], rdi
14000a77d: mov qword ptr [rbp + 0x7f0], r15
14000a784: jmp rax
14000a786: cmp sil, 4
14000a78a: jne 0x14000add8
14000a790: xor r15d, r15d
14000a793: xor r12d, r12d
14000a796: jmp 0x14000b1d6 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000a79b: mov qword ptr [rbp + 0x78], rdi
14000a79f: cmp rdi, 2
14000a7a3: jne 0x1400110f8
14000a7a9: movsd xmm14, qword ptr [r12]
14000a7af: movss xmm15, dword ptr [r12 + 8]
14000a7b6: movsd xmm12, qword ptr [r12 + 0xc]
14000a7bd: movss xmm13, dword ptr [r12 + 0x14]
14000a7c4: mov rax, qword ptr [rbp + 0x950]
14000a7cb: mov rax, qword ptr [rax]
14000a7ce: mov rdx, qword ptr [rbp + 0x7e0]
14000a7d5: mov ecx, dword ptr [rdx + 0x128]
14000a7db: mov edx, dword ptr [rdx + 0x12c]
14000a7e1: movsd qword ptr [rbp + 0x590], xmm14
14000a7ea: movss dword ptr [rbp + 0x598], xmm15
14000a7f3: movsd qword ptr [rbp + 0x6a0], xmm12
14000a7fc: movss dword ptr [rbp + 0x6a8], xmm13
14000a805: mov byte ptr [rbp + 0x817], 1
14000a80c: mov dword ptr [rsp + 0x38], edx
14000a810: mov dword ptr [rsp + 0x30], ecx
14000a814: mov rcx, qword ptr [rbp + 0x2e0]
14000a81b: mov qword ptr [rsp + 0x28], rcx
14000a820: mov qword ptr [rsp + 0x20], rax
14000a825: lea rcx, [rbp + 0x650]
14000a82c: mov rdx, qword ptr [rbp + 0x2e8]
14000a833: lea r8, [rbp + 0x590]
14000a83a: lea r9, [rbp + 0x6a0]
14000a841: call 0x1407d8fe0 ; system_decorator::decorator_visual::stairs_assemble::calculate_adjusted_ladder_positions
14000a846: mov r10, qword ptr [rbp + 0x7e0]
14000a84d: mov r8d, dword ptr [r10 + 0x128]
14000a854: mov r9d, dword ptr [r10 + 0x12c]
14000a85b: mov dword ptr [rbp + 0x590], r8d
14000a862: mov dword ptr [rbp + 0x594], r9d
14000a869: mov rax, qword ptr [rbp + 0x7c0]
14000a870: mov rdi, qword ptr [rax + 0x70]
14000a874: test rdi, rdi
14000a877: lea rsi, [rbp + 0x590]
14000a87e: je 0x14000a98c
14000a884: cmp rdi, 1
14000a888: jne 0x14000a8ad
14000a88a: mov rax, qword ptr [rbp + 0x7c0]
14000a891: mov rax, qword ptr [rax + 0x68]
14000a895: cmp dword ptr [rax + 8], r8d
14000a899: jne 0x14000a98c
14000a89f: cmp dword ptr [rax + 0xc], r9d
14000a8a3: jne 0x14000a98c
14000a8a9: xor edx, edx
14000a8ab: jmp 0x14000a905 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000a8ad: mov rax, qword ptr [rbp + 0x7c0]
14000a8b4: mov rcx, qword ptr [rax + 0x98]
14000a8bb: mov rdx, qword ptr [rax + 0xa0]
14000a8c2: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000a8c7: mov byte ptr [rbp + 0x817], 1
14000a8ce: mov rcx, qword ptr [rbp + 0x350]
14000a8d5: mov rdx, rax
14000a8d8: mov r8, rsi
14000a8db: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000a8e0: cmp rax, 1
14000a8e4: mov r10, qword ptr [rbp + 0x7e0]
14000a8eb: jne 0x14000a98c
14000a8f1: cmp rdx, rdi
14000a8f4: jae 0x140011604
14000a8fa: mov rax, qword ptr [rbp + 0x7c0]
14000a901: mov rax, qword ptr [rax + 0x68]
14000a905: lea rcx, [rbp + 0x460]
14000a90c: lea rdx, [rdx + rdx*4]
14000a910: movups xmm0, xmmword ptr [rax + rdx*8 + 0x10]
14000a915: movaps xmmword ptr [rbp + 0x460], xmm0
14000a91c: mov eax, dword ptr [rax + rdx*8 + 0x20]
14000a920: mov dword ptr [rbp + 0x470], eax
14000a926: movzx edx, byte ptr [rbp + 0x460]
14000a92d: mov byte ptr [rbp + 0x35c], 2
14000a934: mov eax, dword ptr [rbp + 0x36c]
14000a93a: mov dword ptr [rbp + 0x5a0], eax
14000a940: movzx eax, byte ptr [rbp + 0x35c]
14000a947: mov byte ptr [rbp + 0x590], al
14000a94d: mov rax, qword ptr [rbp + 0x35d]
14000a954: mov qword ptr [rbp + 0x591], rax
14000a95b: mov eax, dword ptr [rbp + 0x365]
14000a961: mov dword ptr [rbp + 0x599], eax
14000a967: movzx eax, word ptr [rbp + 0x369]
14000a96e: mov word ptr [rbp + 0x59d], ax
14000a975: movzx eax, byte ptr [rbp + 0x36b]
14000a97c: mov byte ptr [rbp + 0x59f], al
14000a982: mov rax, rcx
14000a985: cmp dl, 3
14000a988: je 0x14000a9ef
14000a98a: jmp 0x14000a9f2 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000a98c: mov byte ptr [rbp + 0x460], 3
14000a993: mov byte ptr [rbp + 0x35c], 2
14000a99a: mov eax, dword ptr [rbp + 0x36c]
14000a9a0: mov dword ptr [rbp + 0x5a0], eax
14000a9a6: movzx eax, byte ptr [rbp + 0x35c]
14000a9ad: mov byte ptr [rbp + 0x590], al
14000a9b3: mov rax, qword ptr [rbp + 0x35d]
14000a9ba: mov qword ptr [rbp + 0x591], rax
14000a9c1: mov eax, dword ptr [rbp + 0x365]
14000a9c7: mov dword ptr [rbp + 0x599], eax
14000a9cd: movzx eax, word ptr [rbp + 0x369]
14000a9d4: mov word ptr [rbp + 0x59d], ax
14000a9db: movzx eax, byte ptr [rbp + 0x36b]
14000a9e2: mov byte ptr [rbp + 0x59f], al
14000a9e8: lea rcx, [rbp + 0x460]
14000a9ef: mov rax, rsi
14000a9f2: mov edx, dword ptr [rax + 0x10]
14000a9f5: mov dword ptr [rbp + 0x390], edx
14000a9fb: movups xmm0, xmmword ptr [rax]
14000a9fe: movaps xmmword ptr [rbp + 0x380], xmm0
14000aa05: mov r9d, dword ptr [r10 + 0x128]
14000aa0c: mov r8d, dword ptr [r10 + 0x12c]
14000aa13: mov dword ptr [rbp + 0x590], r8d
14000aa1a: mov dword ptr [rbp + 0x594], r9d
14000aa21: mov rax, qword ptr [rbp + 0x7c0]
14000aa28: mov rdi, qword ptr [rax + 0x70]
14000aa2c: test rdi, rdi
14000aa2f: je 0x14000aae3
14000aa35: cmp rdi, 1
14000aa39: jne 0x14000aa5e
14000aa3b: mov rax, qword ptr [rbp + 0x7c0]
14000aa42: mov rax, qword ptr [rax + 0x68]
14000aa46: cmp dword ptr [rax + 8], r8d
14000aa4a: jne 0x14000aae3
14000aa50: cmp dword ptr [rax + 0xc], r9d
14000aa54: jne 0x14000aae3
14000aa5a: xor edx, edx
14000aa5c: jmp 0x14000aab2 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000aa5e: mov rax, qword ptr [rbp + 0x7c0]
14000aa65: mov rcx, qword ptr [rax + 0x98]
14000aa6c: mov rdx, qword ptr [rax + 0xa0]
14000aa73: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000aa78: mov byte ptr [rbp + 0x817], 1
14000aa7f: mov rcx, qword ptr [rbp + 0x350]
14000aa86: mov rdx, rax
14000aa89: mov r8, rsi
14000aa8c: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000aa91: cmp rax, 1
14000aa95: lea rcx, [rbp + 0x460]
14000aa9c: jne 0x14000aae3
14000aa9e: cmp rdx, rdi
14000aaa1: jae 0x1400115e6
14000aaa7: mov rax, qword ptr [rbp + 0x7c0]
14000aaae: mov rax, qword ptr [rax + 0x68]
14000aab2: lea rdx, [rdx + rdx*4]
14000aab6: movups xmm0, xmmword ptr [rax + rdx*8 + 0x10]
14000aabb: movaps xmmword ptr [rbp + 0x590], xmm0
14000aac2: mov eax, dword ptr [rax + rdx*8 + 0x20]
14000aac6: cmp byte ptr [rbp + 0x590], 3
14000aacd: mov dword ptr [rbp + 0x5a0], eax
14000aad3: mov rax, rsi
14000aad6: lea rdx, [rbp + 0x35c]
14000aadd: cmove rax, rdx
14000aae1: jmp 0x14000aaf1 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000aae3: mov byte ptr [rbp + 0x590], 3
14000aaea: lea rax, [rbp + 0x35c]
14000aaf1: mov r8d, dword ptr [rax + 0x10]
14000aaf5: lea rdx, [rbp + 0x6d8]
14000aafc: mov dword ptr [rdx - 0x14], r8d
14000ab00: movups xmm0, xmmword ptr [rax]
14000ab03: movups xmmword ptr [rdx - 0x24], xmm0
14000ab07: movaps xmm0, xmmword ptr [rbp + 0x380]
14000ab0e: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000ab15: mov eax, dword ptr [rbp + 0x390]
14000ab1b: mov dword ptr [rbp + 0x6b0], eax
14000ab21: movlps qword ptr [rbp + 0x460], xmm14
14000ab29: movss dword ptr [rbp + 0x468], xmm15
14000ab32: movlps qword ptr [rbp + 0x590], xmm12
14000ab3a: movss dword ptr [rbp + 0x598], xmm13
14000ab43: mov byte ptr [rbp + 0x817], 1
14000ab4a: mov rdx, rsi
14000ab4d: mov r8, qword ptr [rbp + 0x2e8]
14000ab54: lea r9, [rbp + 0x6a0]
14000ab5b: call 0x1407de800 ; system_decorator::decorator_visual::stairs_assemble::determine_ladder_support
14000ab60: mov byte ptr [rbp + 0x7ef], al
14000ab66: mov rax, qword ptr [rbp + 0x800]
14000ab6d: movsd xmm6, qword ptr [rax]
14000ab71: movsd xmm11, qword ptr [rax + 8]
14000ab77: movups xmm0, xmmword ptr [rax]
14000ab7a: movaps xmmword ptr [rbp + 0x310], xmm0
14000ab81: mov rax, qword ptr [rbp + 0x780]
14000ab88: movsd xmm0, qword ptr [rax]
14000ab8c: movsd qword ptr [rbp + 0x540], xmm0
14000ab94: cmp byte ptr [rbp + 0x538], 0
14000ab9b: je 0x14000d04f
14000aba1: mov byte ptr [rbp + 0x7ef], 0
14000aba8: movzx eax, byte ptr [rip + 0x362a05a]
14000abaf: mov ecx, 0x18
14000abb4: mov edx, 4
14000abb9: call 0x140613c10 ; ?
14000abbe: test rax, rax
14000abc1: je 0x1400115ac
14000abc7: mov rcx, qword ptr [rbp + 0x660]
14000abce: mov qword ptr [rax + 0x10], rcx
14000abd2: movups xmm0, xmmword ptr [rbp + 0x650]
14000abd9: movups xmmword ptr [rax], xmm0
14000abdc: mov qword ptr [rbp + 0x6a0], 2
14000abe7: mov qword ptr [rbp + 0x6a8], rax
14000abee: mov qword ptr [rbp + 0x6b0], 2
14000abf9: mov byte ptr [rbp + 0x817], 1
14000ac00: lea rcx, [rbp + 0x590]
14000ac07: lea rdx, [rbp + 0x6a0]
14000ac0e: xor r8d, r8d
14000ac11: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000ac16: xor esi, esi
14000ac18: cmp rsi, qword ptr [rbp + 0x590]
14000ac1f: jo 0x14000d04f
14000ac25: mov rax, qword ptr [rbp + 0x5c0]
14000ac2c: mov qword ptr [rbp + 0x6d0], rax
14000ac33: movups xmm0, xmmword ptr [rbp + 0x590]
14000ac3a: movups xmm1, xmmword ptr [rbp + 0x5a0]
14000ac41: movups xmm2, xmmword ptr [rbp + 0x5b0]
14000ac48: movaps xmmword ptr [rbp + 0x6c0], xmm2
14000ac4f: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000ac56: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000ac5d: lea rcx, [rbp + 0x460]
14000ac64: lea rdx, [rbp + 0x6a0]
14000ac6b: movd xmm2, dword ptr [rip + 0x291abfd]
14000ac73: mov r9d, 1
14000ac79: call 0x140152b00 ; utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000ac7e: mov rax, qword ptr [rbp + 0x6a0]
14000ac85: test rax, rax
14000ac88: je 0x14000aca4
14000ac8a: mov rcx, qword ptr [rbp + 0x6a8]
14000ac91: shl rax, 2
14000ac95: lea rdx, [rax + rax*2]
14000ac99: mov r8d, 4
14000ac9f: call 0x140613c20 ; ?
14000aca4: mov rdx, qword ptr [rbp + 0x6b8]
14000acab: test rdx, rdx
14000acae: je 0x14000acc6
14000acb0: mov rcx, qword ptr [rbp + 0x6c0]
14000acb7: shl rdx, 2
14000acbb: mov r8d, 4
14000acc1: call 0x140613c20 ; ?
14000acc6: cmp rsi, qword ptr [rbp + 0x460]
14000accd: jo 0x14000d04f
14000acd3: movups xmm0, xmmword ptr [rbp + 0x460]
14000acda: movups xmm1, xmmword ptr [rbp + 0x470]
14000ace1: movups xmm2, xmmword ptr [rbp + 0x480]
14000ace8: movaps xmmword ptr [rbp + 0x5a0], xmm1
14000acef: movaps xmmword ptr [rbp + 0x5b0], xmm2
14000acf6: mov rax, qword ptr [rbp + 0x490]
14000acfd: mov qword ptr [rbp + 0x5c0], rax
14000ad04: movaps xmmword ptr [rbp + 0x590], xmm0
14000ad0b: mov rbx, qword ptr [rbp + 0x5a0]
14000ad12: mov rax, qword ptr [rbp + 0x5b8]
14000ad19: cmp rax, rbx
14000ad1c: cmovb rbx, rax
14000ad20: lea rdi, [rbx*8]
14000ad28: mov rax, rbx
14000ad2b: shr rax, 0x3d
14000ad2f: mov r13, qword ptr [rbp + 0x348]
14000ad36: xorps xmm9, xmm9
14000ad3a: jne 0x140011763
14000ad40: mov esi, 0
14000ad45: movabs rax, 0x7ffffffffffffffc
14000ad4f: cmp rdi, rax
14000ad52: movss xmm8, dword ptr [rip + 0x291aba9]
14000ad5b: movaps xmm10, xmmword ptr [rip + 0x291abcd]
14000ad63: ja 0x140011765
14000ad69: mov r14, qword ptr [rbp + 0x5b0]
14000ad70: test rdi, rdi
14000ad73: je 0x14000ae0c
14000ad79: movzx eax, byte ptr [rip + 0x3629e89]
14000ad80: mov esi, 4
14000ad85: mov edx, 4
14000ad8a: mov rcx, rdi
14000ad8d: call 0x140613c10 ; ?
14000ad92: test rax, rax
14000ad95: je 0x140011765
14000ad9b: mov r12, rax
14000ad9e: mov qword ptr [rbp + 0x800], rbx
14000ada5: test rbx, rbx
14000ada8: jne 0x14000ae26
14000adaa: jmp 0x14000aef3 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000adaf: cmp byte ptr [rbp + 0x7b8], 0
14000adb6: mov rax, qword ptr [rbp + 0x670]
14000adbd: je 0x14000b942
14000adc3: cmp sil, 4
14000adc7: jne 0x14000b55b
14000adcd: xor r15d, r15d
14000add0: xor r12d, r12d
14000add3: jmp 0x14000b852 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000add8: cmp qword ptr [rbp + 0x670], 0
14000ade0: je 0x14000b1ad
14000ade6: xor eax, eax
14000ade8: mov rcx, qword ptr [rbp + 0x7b0]
14000adef: cmp rcx, qword ptr [rbp + 0x668]
14000adf6: jne 0x14000b1cc
14000adfc: cmp byte ptr [r13 + 0x108], 1
14000ae04: sete cl
14000ae07: jmp 0x14000b1ce ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000ae0c: mov r12d, 4
14000ae12: mov qword ptr [rbp + 0x800], 0
14000ae1d: test rbx, rbx
14000ae20: je 0x14000aef3
14000ae26: cmp rbx, 8
14000ae2a: jae 0x14000afe4
14000ae30: xor eax, eax
14000ae32: mov rcx, rax
14000ae35: or rcx, 1
14000ae39: test bl, 1
14000ae3c: je 0x14000ae75
14000ae3e: movss xmm0, dword ptr [r14 + rax*4]
14000ae44: movaps xmm1, xmm8
14000ae48: subss xmm1, xmm0
14000ae4c: movsd xmm2, qword ptr [rbp + 0x310]
14000ae54: movsd xmm3, qword ptr [rbp + 0x318]
14000ae5c: movsldup xmm1, xmm1
14000ae60: mulps xmm1, xmm2
14000ae63: movsldup xmm0, xmm0
14000ae67: mulps xmm0, xmm3
14000ae6a: addps xmm0, xmm1
14000ae6d: movlps qword ptr [r12 + rax*8], xmm0
14000ae72: mov rax, rcx
14000ae75: cmp rbx, rcx
14000ae78: je 0x14000aef3
14000ae7a: nop word ptr [rax + rax]
14000ae80: movss xmm0, dword ptr [r14 + rax*4]
14000ae86: movaps xmm1, xmm8
14000ae8a: subss xmm1, xmm0
14000ae8e: movsd xmm2, qword ptr [rbp + 0x310]
14000ae96: movsd xmm3, qword ptr [rbp + 0x318]
14000ae9e: movsldup xmm1, xmm1
14000aea2: mulps xmm1, xmm2
14000aea5: movsldup xmm0, xmm0
14000aea9: mulps xmm0, xmm3
14000aeac: addps xmm0, xmm1
14000aeaf: movlps qword ptr [r12 + rax*8], xmm0
14000aeb4: movss xmm0, dword ptr [r14 + rax*4 + 4]
14000aebb: movaps xmm1, xmm8
14000aebf: subss xmm1, xmm0
14000aec3: movsd xmm2, qword ptr [rbp + 0x310]
14000aecb: movsd xmm3, qword ptr [rbp + 0x318]
14000aed3: movsldup xmm1, xmm1
14000aed7: mulps xmm1, xmm2
14000aeda: movsldup xmm0, xmm0
14000aede: mulps xmm0, xmm3
14000aee1: addps xmm0, xmm1
14000aee4: movlps qword ptr [r12 + rax*8 + 8], xmm0
14000aeea: add rax, 2
14000aeee: cmp rbx, rax
14000aef1: jne 0x14000ae80
14000aef3: mov r14, qword ptr [rbp + 0x5a0]
14000aefa: mov rax, qword ptr [rbp + 0x5b8]
14000af01: cmp rax, r14
14000af04: cmovb r14, rax
14000af08: lea rdi, [r14*4]
14000af10: mov rax, r14
14000af13: shr rax, 0x3e
14000af17: mov qword ptr [rbp + 0x780], r12
14000af1e: jne 0x1400117f1
14000af24: mov esi, 0
14000af29: movabs rax, 0x7ffffffffffffffc
14000af33: cmp rdi, rax
14000af36: ja 0x1400117f3
14000af3c: mov r15, qword ptr [rbp + 0x5b0]
14000af43: test rdi, rdi
14000af46: je 0x14000af7e
14000af48: movzx eax, byte ptr [rip + 0x3629cba]
14000af4f: mov esi, 4
14000af54: mov edx, 4
14000af59: mov rcx, rdi
14000af5c: call 0x140613c10 ; ?
14000af61: test rax, rax
14000af64: je 0x1400117f3
14000af6a: mov r10, rax
14000af6d: mov qword ptr [rbp + 0x760], r14
14000af74: test r14, r14
14000af77: jne 0x14000af98
14000af79: jmp 0x14000cd9a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000af7e: mov r10d, 4
14000af84: mov qword ptr [rbp + 0x760], 0
14000af8f: test r14, r14
14000af92: je 0x14000cd9a
14000af98: cmp r14, 8
14000af9c: jae 0x14000b0f6
14000afa2: xor eax, eax
14000afa4: mov rcx, rax
14000afa7: or rcx, 1
14000afab: test r14b, 1
14000afaf: je 0x14000cd36
14000afb5: movss xmm0, dword ptr [r15 + rax*4]
14000afbb: movaps xmm1, xmm8
14000afbf: subss xmm1, xmm0
14000afc3: movsd xmm2, qword ptr [rbp + 0x540]
14000afcb: unpcklps xmm1, xmm0
14000afce: mulps xmm1, xmm2
14000afd1: movshdup xmm0, xmm1
14000afd5: addss xmm0, xmm1
14000afd9: movss dword ptr [r10 + rax*4], xmm0
14000afdf: jmp 0x14000cd95 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000afe4: lea rax, [r12 + rbx*8]
14000afe8: add rax, -8
14000afec: cmp rax, r12
14000afef: jb 0x14000ae30
14000aff5: lea rax, [r12 + 4]
14000affa: lea rcx, [r12 + rbx*8]
14000affe: add rcx, -4
14000b002: cmp rcx, rax
14000b005: jb 0x14000ae30
14000b00b: lea rax, [rbx - 1]
14000b00f: shr rax, 0x3d
14000b013: mov eax, 0
14000b018: jne 0x14000ae32
14000b01e: add rdi, r12
14000b021: lea rax, [r14 + rbx*4]
14000b025: cmp rax, r12
14000b028: seta al
14000b02b: cmp r14, rdi
14000b02e: setb r8b
14000b032: lea rcx, [rbp + 0x320]
14000b039: cmp rcx, r12
14000b03c: seta cl
14000b03f: lea rdx, [rbp + 0x310]
14000b046: cmp rdx, rdi
14000b049: setb dl
14000b04c: test al, r8b
14000b04f: jne 0x14000ae30
14000b055: mov eax, 0
14000b05a: and cl, dl
14000b05c: jne 0x14000ae32
14000b062: movaps xmmword ptr [rbp + 0x760], xmm6
14000b069: mov rax, rbx
14000b06c: movabs rcx, 0x1ffffffffffffffc
14000b076: and rax, rcx
14000b079: movaps xmm0, xmmword ptr [rbp + 0x310]
14000b080: movaps xmm1, xmm0
14000b083: shufps xmm1, xmm0, 0xff
14000b087: movaps xmm2, xmm0
14000b08a: shufps xmm2, xmm0, 0xaa
14000b08e: movaps xmm3, xmm0
14000b091: shufps xmm3, xmm0, 0x55
14000b095: shufps xmm0, xmm0, 0
14000b099: xor ecx, ecx
14000b09b: nop dword ptr [rax + rax]
14000b0a0: movups xmm4, xmmword ptr [r14 + rcx*4]
14000b0a5: movaps xmm5, xmm10
14000b0a9: subps xmm5, xmm4
14000b0ac: movaps xmm6, xmm5
14000b0af: mulps xmm6, xmm0
14000b0b2: mulps xmm5, xmm3
14000b0b5: movaps xmm7, xmm4
14000b0b8: mulps xmm7, xmm2
14000b0bb: addps xmm7, xmm6
14000b0be: mulps xmm4, xmm1
14000b0c1: addps xmm4, xmm5
14000b0c4: movaps xmm5, xmm7
14000b0c7: unpcklps xmm5, xmm4
14000b0ca: unpckhps xmm7, xmm4
14000b0cd: movups xmmword ptr [r12 + rcx*8 + 0x10], xmm7
14000b0d3: movups xmmword ptr [r12 + rcx*8], xmm5
14000b0d8: add rcx, 4
14000b0dc: cmp rax, rcx
14000b0df: jne 0x14000b0a0
14000b0e1: cmp rbx, rax
14000b0e4: movaps xmm6, xmmword ptr [rbp + 0x760]
14000b0eb: jne 0x14000ae32
14000b0f1: jmp 0x14000aef3 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b0f6: lea rax, [r10 + rdi]
14000b0fa: add rdi, r15
14000b0fd: lea rcx, [rbp + 0x548]
14000b104: cmp rcx, r10
14000b107: seta r8b
14000b10b: lea rcx, [rbp + 0x540]
14000b112: cmp rcx, rax
14000b115: setb r9b
14000b119: cmp rdi, r10
14000b11c: seta cl
14000b11f: cmp r15, rax
14000b122: setb dl
14000b125: test r8b, r9b
14000b128: jne 0x14000afa2
14000b12e: mov eax, 0
14000b133: and cl, dl
14000b135: jne 0x14000afa4
14000b13b: mov rax, r14
14000b13e: movabs rcx, 0x3ffffffffffffff8
14000b148: and rax, rcx
14000b14b: movaps xmm0, xmmword ptr [rbp + 0x540]
14000b152: movaps xmm1, xmm0
14000b155: shufps xmm1, xmm0, 0
14000b159: shufps xmm0, xmm0, 0x55
14000b15d: xor ecx, ecx
14000b15f: nop 
14000b160: movups xmm2, xmmword ptr [r15 + rcx*4]
14000b165: movups xmm3, xmmword ptr [r15 + rcx*4 + 0x10]
14000b16b: movaps xmm4, xmm10
14000b16f: subps xmm4, xmm2
14000b172: movaps xmm5, xmm10
14000b176: subps xmm5, xmm3
14000b179: mulps xmm4, xmm1
14000b17c: mulps xmm5, xmm1
14000b17f: mulps xmm2, xmm0
14000b182: addps xmm2, xmm4
14000b185: mulps xmm3, xmm0
14000b188: addps xmm3, xmm5
14000b18b: movups xmmword ptr [r10 + rcx*4], xmm2
14000b190: movups xmmword ptr [r10 + rcx*4 + 0x10], xmm3
14000b196: add rcx, 8
14000b19a: cmp rax, rcx
14000b19d: jne 0x14000b160
14000b19f: cmp r14, rax
14000b1a2: jne 0x14000afa4
14000b1a8: jmp 0x14000cd9a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b1ad: cmp byte ptr [r13 + 0xe8], 1
14000b1b5: sete al
14000b1b8: mov rcx, qword ptr [rbp + 0x7b0]
14000b1bf: cmp rcx, qword ptr [rbp + 0x668]
14000b1c6: je 0x14000adfc
14000b1cc: xor ecx, ecx
14000b1ce: movzx r15d, al
14000b1d2: movzx r12d, cl
14000b1d6: mov rax, qword ptr [rbp + 0x938]
14000b1dd: mov rdi, qword ptr [rax]
14000b1e0: mov r14b, byte ptr [rbp + 0x810]
14000b1e7: mov byte ptr [rbp + 0x816], 1
14000b1ee: mov byte ptr [rbp + 0x815], 1
14000b1f5: lea rcx, [rbp + 0x2a0]
14000b1fc: call 0x1400284b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$8<true>
14000b201: movzx ecx, r12w
14000b205: shl ecx, 8
14000b208: movzx r8d, r15w
14000b20c: or r8d, ecx
14000b20f: mov ecx, dword ptr [rbp + 0x588]
14000b215: mov dword ptr [rbp + 0x6a0], ecx
14000b21b: movss xmm0, dword ptr [rbp + 0x560]
14000b223: movss dword ptr [rbp + 0x6a4], xmm0
14000b22b: mov ecx, dword ptr [rbp + 0x580]
14000b231: mov dword ptr [rbp + 0x6a8], ecx
14000b237: movss xmm0, dword ptr [rbp + 0x568]
14000b23f: movss dword ptr [rbp + 0x6ac], xmm0
14000b247: mov byte ptr [rbp + 0x816], 1
14000b24e: mov byte ptr [rbp + 0x815], 1
14000b255: and dl, 1
14000b258: mov byte ptr [rsp + 0x68], dl
14000b25c: and al, 1
14000b25e: mov byte ptr [rsp + 0x60], al
14000b262: mov byte ptr [rsp + 0x58], r14b
14000b267: mov qword ptr [rsp + 0x80], rbx
14000b26f: mov rax, qword ptr [rbp + 0x7f0]
14000b276: mov qword ptr [rsp + 0x78], rax
14000b27b: lea rax, [rbp + 0x6a0]
14000b282: mov qword ptr [rsp + 0x70], rax
14000b287: mov byte ptr [rsp + 0x50], sil
14000b28c: mov eax, dword ptr [rbp + 0x7b8]
14000b292: mov byte ptr [rsp + 0x48], al
14000b296: mov word ptr [rsp + 0x40], r8w
14000b29c: mov qword ptr [rsp + 0x38], rdi
14000b2a1: mov eax, dword ptr [rbp + 0x528]
14000b2a7: mov dword ptr [rsp + 0x30], eax
14000b2ab: mov rax, qword ptr [rbp + 0x758]
14000b2b2: mov dword ptr [rsp + 0x28], eax
14000b2b6: lea rax, [rbp + 0x428]
14000b2bd: mov qword ptr [rsp + 0x20], rax
14000b2c2: lea rcx, [rbp + 0x140]
14000b2c9: lea rdx, [rbp + 0x548]
14000b2d0: lea r8, [rbp + 0x650]
14000b2d7: lea r9, [rbp + 0x680]
14000b2de: call 0x1407f98e0 ; system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings
14000b2e3: mov byte ptr [rbp + 0x816], 1
14000b2ea: mov byte ptr [rbp + 0x815], 1
14000b2f1: mov rcx, qword ptr [rbp + 0x268]
14000b2f8: call 0x140987ca0 ; country_core::render::impl$0::deref
14000b2fd: mov rdx, qword ptr [rbp + 0x550]
14000b304: mov r8, qword ptr [rbp + 0x558]
14000b30b: mov r9, qword ptr [rbp + 0x658]
14000b312: mov rcx, qword ptr [rbp + 0x660]
14000b319: movups xmm0, xmmword ptr [rbp + 0x688]
14000b320: mov r10d, dword ptr [rbp + 0x588]
14000b327: mov dword ptr [rbp + 0x6a0], r10d
14000b32e: movss xmm1, dword ptr [rbp + 0x560]
14000b336: movss dword ptr [rbp + 0x6a4], xmm1
14000b33e: mov r10d, dword ptr [rbp + 0x580]
14000b345: mov dword ptr [rbp + 0x6a8], r10d
14000b34c: movss xmm1, dword ptr [rbp + 0x568]
14000b354: movss dword ptr [rbp + 0x6ac], xmm1
14000b35c: mov byte ptr [rbp + 0x816], 1
14000b363: mov byte ptr [rbp + 0x815], 1
14000b36a: mov qword ptr [rsp + 0x60], rbx
14000b36f: mov r10, qword ptr [rbp + 0x7f0]
14000b376: mov qword ptr [rsp + 0x58], r10
14000b37b: lea r10, [rbp + 0x6a0]
14000b382: mov qword ptr [rsp + 0x50], r10
14000b387: mov r10, qword ptr [rbp + 0x758]
14000b38e: mov dword ptr [rsp + 0x48], r10d
14000b393: mov r10, qword ptr [rbp + 0x940]
14000b39a: mov qword ptr [rsp + 0x40], r10
14000b39f: lea r10, [rbp + 0x140]
14000b3a6: mov qword ptr [rsp + 0x38], r10
14000b3ab: movups xmmword ptr [rsp + 0x28], xmm0
14000b3b0: mov qword ptr [rsp + 0x20], rcx
14000b3b5: mov rcx, rax
14000b3b8: call 0x1407dc170 ; system_decorator::decorator_visual::stairs_assemble::gen_stair_floors
14000b3bd: mov rax, qword ptr [rbp + 0x7e0]
14000b3c4: cmp byte ptr [rax + 0xe8], 0
14000b3cb: je 0x14000b51f
14000b3d1: xor eax, eax
14000b3d3: mov r10, qword ptr [rbp + 0x7f0]
14000b3da: movd xmm0, dword ptr [rip + 0x291a4f2]
14000b3e2: test al, al
14000b3e4: jne 0x14000b3ee
14000b3e6: movd xmm0, dword ptr [rip + 0x291a4ea]
14000b3ee: mov rcx, qword ptr [rbp + 0x550]
14000b3f5: mov rdx, qword ptr [rbp + 0x558]
14000b3fc: mov r8, qword ptr [rbp + 0x658]
14000b403: mov r9, qword ptr [rbp + 0x660]
14000b40a: movups xmm1, xmmword ptr [rbp + 0x688]
14000b411: mov rax, qword ptr [rbp + 0x938]
14000b418: mov rax, qword ptr [rax]
14000b41b: mov byte ptr [rbp + 0x816], 1
14000b422: mov byte ptr [rbp + 0x815], 1
14000b429: mov qword ptr [rsp + 0x78], rbx
14000b42e: mov qword ptr [rsp + 0x70], r10
14000b433: mov r10, qword ptr [rbp + 0x240]
14000b43a: mov qword ptr [rsp + 0x68], r10
14000b43f: movd dword ptr [rsp + 0x60], xmm0
14000b445: mov r10d, dword ptr [rbp + 0x7b8]
14000b44c: mov byte ptr [rsp + 0x58], r10b
14000b451: mov qword ptr [rsp + 0x48], rax
14000b456: mov rax, qword ptr [rbp + 0x758]
14000b45d: mov dword ptr [rsp + 0x40], eax
14000b461: lea rax, [rbp + 0x140]
14000b468: mov qword ptr [rsp + 0x38], rax
14000b46d: lea rax, [rbp + 0x428]
14000b474: mov qword ptr [rsp + 0x30], rax
14000b479: movups xmmword ptr [rsp + 0x20], xmm1
14000b47e: mov byte ptr [rsp + 0x80], 1
14000b486: mov word ptr [rsp + 0x50], 0
14000b48d: call 0x1407da0b0 ; system_decorator::decorator_visual::stairs_assemble::construct_stair_steps
14000b492: cmp byte ptr [rbp + 0x7b8], 0
14000b499: je 0x14000bb1d
14000b49f: movzx esi, byte ptr [rbp + 0x80f]
14000b4a6: cmp sil, 1
14000b4aa: ja 0x14000b545
14000b4b0: movzx r14d, byte ptr [rbp + 0x810]
14000b4b8: mov r10, qword ptr [rbp + 0x7e0]
14000b4bf: mov r8d, dword ptr [r10 + 0x128]
14000b4c6: mov r9d, dword ptr [r10 + 0x12c]
14000b4cd: mov dword ptr [rbp + 0x6a0], r8d
14000b4d4: mov dword ptr [rbp + 0x6a4], r9d
14000b4db: mov rax, qword ptr [rbp + 0x7c0]
14000b4e2: mov rdi, qword ptr [rax + 0x70]
14000b4e6: test rdi, rdi
14000b4e9: je 0x14000b5f8
14000b4ef: cmp rdi, 1
14000b4f3: jne 0x14000b58a
14000b4f9: mov rax, qword ptr [rbp + 0x7c0]
14000b500: mov rax, qword ptr [rax + 0x68]
14000b504: cmp dword ptr [rax + 8], r8d
14000b508: jne 0x14000b5f8
14000b50e: cmp dword ptr [rax + 0xc], r9d
14000b512: jne 0x14000b5f8
14000b518: xor edx, edx
14000b51a: jmp 0x14000b5e9 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b51f: cmp byte ptr [rax + 0x108], 0
14000b526: sete al
14000b529: mov r10, qword ptr [rbp + 0x7f0]
14000b530: movd xmm0, dword ptr [rip + 0x291a39c]
14000b538: test al, al
14000b53a: je 0x14000b3e6
14000b540: jmp 0x14000b3ee ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b545: xor r14d, r14d
14000b548: mov r13, qword ptr [rbp + 0x7e0]
14000b54f: mov r15, qword ptr [rbp + 0x7f0]
14000b556: jmp 0x14000bd9c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b55b: test rax, rax
14000b55e: je 0x14000b829
14000b564: xor eax, eax
14000b566: mov rcx, qword ptr [rbp + 0x7b0]
14000b56d: cmp rcx, qword ptr [rbp + 0x668]
14000b574: jne 0x14000b848
14000b57a: cmp byte ptr [r13 + 0x108], 1
14000b582: sete cl
14000b585: jmp 0x14000b84a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b58a: mov rax, qword ptr [rbp + 0x7c0]
14000b591: mov rcx, qword ptr [rax + 0x98]
14000b598: mov rdx, qword ptr [rax + 0xa0]
14000b59f: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000b5a4: mov byte ptr [rbp + 0x816], 1
14000b5ab: mov byte ptr [rbp + 0x815], 1
14000b5b2: mov rcx, qword ptr [rbp + 0x350]
14000b5b9: mov rdx, rax
14000b5bc: lea r8, [rbp + 0x6a0]
14000b5c3: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000b5c8: cmp rax, 1
14000b5cc: mov r10, qword ptr [rbp + 0x7e0]
14000b5d3: jne 0x14000b5f8
14000b5d5: cmp rdx, rdi
14000b5d8: jae 0x1400119ec
14000b5de: mov rax, qword ptr [rbp + 0x7c0]
14000b5e5: mov rax, qword ptr [rax + 0x68]
14000b5e9: lea rcx, [rdx + rdx*4]
14000b5ed: cmp byte ptr [rax + rcx*8 + 0x10], 0
14000b5f2: je 0x14000b6bc
14000b5f8: mov r9d, dword ptr [r10 + 0x128]
14000b5ff: mov r8d, dword ptr [r10 + 0x12c]
14000b606: mov dword ptr [rbp + 0x6a0], r8d
14000b60d: mov dword ptr [rbp + 0x6a4], r9d
14000b614: mov rax, qword ptr [rbp + 0x7c0]
14000b61b: mov rdi, qword ptr [rax + 0x70]
14000b61f: test rdi, rdi
14000b622: je 0x14000b742
14000b628: cmp rdi, 1
14000b62c: jne 0x14000b651
14000b62e: mov rax, qword ptr [rbp + 0x7c0]
14000b635: mov rax, qword ptr [rax + 0x68]
14000b639: cmp dword ptr [rax + 8], r8d
14000b63d: jne 0x14000b742
14000b643: cmp dword ptr [rax + 0xc], r9d
14000b647: jne 0x14000b742
14000b64d: xor edx, edx
14000b64f: jmp 0x14000b6ad ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b651: mov rax, qword ptr [rbp + 0x7c0]
14000b658: mov rcx, qword ptr [rax + 0x98]
14000b65f: mov rdx, qword ptr [rax + 0xa0]
14000b666: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000b66b: mov byte ptr [rbp + 0x816], 1
14000b672: mov byte ptr [rbp + 0x815], 1
14000b679: mov rcx, qword ptr [rbp + 0x350]
14000b680: mov rdx, rax
14000b683: lea r8, [rbp + 0x6a0]
14000b68a: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000b68f: cmp rax, 1
14000b693: jne 0x14000b742
14000b699: cmp rdx, rdi
14000b69c: jae 0x140011a0e
14000b6a2: mov rax, qword ptr [rbp + 0x7c0]
14000b6a9: mov rax, qword ptr [rax + 0x68]
14000b6ad: lea rcx, [rdx + rdx*4]
14000b6b1: cmp byte ptr [rax + rcx*8 + 0x10], 0
14000b6b6: jne 0x14000b742
14000b6bc: lea rax, [rax + rcx*8]
14000b6c0: add rax, 0x10
14000b6c4: mov rcx, qword ptr [rbp + 0x938]
14000b6cb: mov rcx, qword ptr [rcx]
14000b6ce: movdqu xmm0, xmmword ptr [rbp + 0x550]
14000b6d6: movups xmm1, xmmword ptr [rbp + 0x658]
14000b6dd: movdqu xmm2, xmmword ptr [rbp + 0x688]
14000b6e5: movss xmm3, dword ptr [rax + 4]
14000b6ea: movss xmm4, dword ptr [rax + 8]
14000b6ef: mov byte ptr [rbp + 0x816], 1
14000b6f6: mov byte ptr [rbp + 0x815], 1
14000b6fd: movss dword ptr [rsp + 0x68], xmm4
14000b703: movss dword ptr [rsp + 0x60], xmm3
14000b709: mov byte ptr [rsp + 0x58], r14b
14000b70e: and sil, 1
14000b712: mov byte ptr [rsp + 0x50], sil
14000b717: movdqu xmmword ptr [rsp + 0x40], xmm2
14000b71d: movups xmmword ptr [rsp + 0x30], xmm1
14000b722: movdqu xmmword ptr [rsp + 0x20], xmm0
14000b728: mov rdx, qword ptr [rbp + 0x758]
14000b72f: mov r8d, dword ptr [rbp + 0x7b8]
14000b736: lea r9, [rbp + 0x140]
14000b73d: call 0x1407febb0 ; system_decorator::decorator_visual::stairs_railings_assemble::add_corner_stone_railing_piece
14000b742: xor edx, edx
14000b744: mov rax, qword ptr [rbp + 0x7b0]
14000b74b: cmp rax, qword ptr [rbp + 0x668]
14000b752: sete dl
14000b755: xor eax, eax
14000b757: cmp qword ptr [rbp + 0x670], 0
14000b75f: sete al
14000b762: mov rcx, qword ptr [rbp + 0x950]
14000b769: mov rcx, qword ptr [rcx]
14000b76c: movdqu xmm0, xmmword ptr [rbp + 0x550]
14000b774: movups xmm1, xmmword ptr [rbp + 0x658]
14000b77b: movdqu xmm2, xmmword ptr [rbp + 0x688]
14000b783: mov r8, qword ptr [rbp + 0x378]
14000b78a: mov r8, qword ptr [r8]
14000b78d: mov r9, qword ptr [rbp + 0x958]
14000b794: mov r9, qword ptr [r9]
14000b797: mov r10, qword ptr [rbp + 0x938]
14000b79e: mov r10, qword ptr [r10]
14000b7a1: shl edx, 8
14000b7a4: or eax, edx
14000b7a6: mov byte ptr [rbp + 0x816], 1
14000b7ad: mov byte ptr [rbp + 0x815], 1
14000b7b4: mov qword ptr [rsp + 0x80], r10
14000b7bc: mov qword ptr [rsp + 0x78], r9
14000b7c1: mov qword ptr [rsp + 0x70], r8
14000b7c6: movdqu xmmword ptr [rsp + 0x60], xmm2
14000b7cc: movups xmmword ptr [rsp + 0x50], xmm1
14000b7d1: movdqu xmmword ptr [rsp + 0x40], xmm0
14000b7d7: lea rdx, [rbp + 0x140]
14000b7de: mov qword ptr [rsp + 0x38], rdx
14000b7e3: mov edx, dword ptr [rbp + 0x7b8]
14000b7e9: mov byte ptr [rsp + 0x30], dl
14000b7ed: mov edx, dword ptr [rbp + 0x528]
14000b7f3: mov dword ptr [rsp + 0x28], edx
14000b7f7: mov rdx, qword ptr [rbp + 0x758]
14000b7fe: mov dword ptr [rsp + 0x20], edx
14000b802: mov word ptr [rsp + 0x88], ax
14000b80a: mov rdx, qword ptr [rbp + 0x7c0]
14000b811: mov r8, qword ptr [rbp + 0x678]
14000b818: lea r9, [rbp + 0x428]
14000b81f: call 0x1407fcc80 ; system_decorator::decorator_visual::stairs_railings_assemble::railings_tips
14000b824: jmp 0x14000bb1d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b829: cmp byte ptr [r13 + 0xe8], 1
14000b831: sete al
14000b834: mov rcx, qword ptr [rbp + 0x7b0]
14000b83b: cmp rcx, qword ptr [rbp + 0x668]
14000b842: je 0x14000b57a
14000b848: xor ecx, ecx
14000b84a: movzx r15d, al
14000b84e: movzx r12d, cl
14000b852: mov rax, qword ptr [rbp + 0x938]
14000b859: mov rdi, qword ptr [rax]
14000b85c: mov r14b, byte ptr [rbp + 0x810]
14000b863: mov byte ptr [rbp + 0x816], 1
14000b86a: mov byte ptr [rbp + 0x815], 1
14000b871: lea rcx, [rbp + 0x2a0]
14000b878: call 0x1400284b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$8<true>
14000b87d: mov dword ptr [rbp + 0x6a0], 0
14000b887: mov dword ptr [rbp + 0x6a8], 0
14000b891: movzx ecx, r12w
14000b895: shl ecx, 8
14000b898: movzx r8d, r15w
14000b89c: or r8d, ecx
14000b89f: mov byte ptr [rbp + 0x816], 1
14000b8a6: mov byte ptr [rbp + 0x815], 1
14000b8ad: and dl, 1
14000b8b0: mov byte ptr [rsp + 0x68], dl
14000b8b4: and al, 1
14000b8b6: mov byte ptr [rsp + 0x60], al
14000b8ba: mov byte ptr [rsp + 0x58], r14b
14000b8bf: mov qword ptr [rsp + 0x80], rbx
14000b8c7: mov rax, qword ptr [rbp + 0x7f0]
14000b8ce: mov qword ptr [rsp + 0x78], rax
14000b8d3: lea rax, [rbp + 0x6a0]
14000b8da: mov qword ptr [rsp + 0x70], rax
14000b8df: mov byte ptr [rsp + 0x50], sil
14000b8e4: mov eax, dword ptr [rbp + 0x7b8]
14000b8ea: mov byte ptr [rsp + 0x48], al
14000b8ee: mov word ptr [rsp + 0x40], r8w
14000b8f4: mov qword ptr [rsp + 0x38], rdi
14000b8f9: mov eax, dword ptr [rbp + 0x528]
14000b8ff: mov dword ptr [rsp + 0x30], eax
14000b903: mov rax, qword ptr [rbp + 0x758]
14000b90a: mov dword ptr [rsp + 0x28], eax
14000b90e: lea rax, [rbp + 0x428]
14000b915: mov qword ptr [rsp + 0x20], rax
14000b91a: lea rcx, [rbp + 0x140]
14000b921: lea rdx, [rbp + 0x548]
14000b928: lea r8, [rbp + 0x650]
14000b92f: lea r9, [rbp + 0x680]
14000b936: call 0x1407f98e0 ; system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings
14000b93b: mov r13, qword ptr [rbp + 0x7e0]
14000b942: cmp byte ptr [r13 + 0xe8], 0
14000b94a: je 0x14000b958
14000b94c: mov dword ptr [rbp + 0x760], 0
14000b956: jmp 0x14000b969 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b958: cmp byte ptr [r13 + 0x108], 0
14000b960: sete al
14000b963: mov dword ptr [rbp + 0x760], eax
14000b969: mov rax, qword ptr [rbp + 0x550]
14000b970: mov qword ptr [rbp + 0x800], rax
14000b977: mov rax, qword ptr [rbp + 0x558]
14000b97e: mov qword ptr [rbp + 0x780], rax
14000b985: mov rdi, qword ptr [rbp + 0x658]
14000b98c: mov r15, qword ptr [rbp + 0x660]
14000b993: mov rsi, qword ptr [rbp + 0x688]
14000b99a: mov r14, qword ptr [rbp + 0x690]
14000b9a1: mov rax, qword ptr [rbp + 0x938]
14000b9a8: mov r12, qword ptr [rax]
14000b9ab: mov byte ptr [rbp + 0x816], 1
14000b9b2: mov byte ptr [rbp + 0x815], 1
14000b9b9: mov rcx, qword ptr [rbp + 0x2e8]
14000b9c0: call 0x140959ef0 ; country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
14000b9c5: test rax, rax
14000b9c8: je 0x14000b9f0
14000b9ca: cmp qword ptr [rbp + 0x558], 0
14000b9d2: je 0x140011837
14000b9d8: mov rax, qword ptr [rbp + 0x550]
14000b9df: movss xmm0, dword ptr [rip + 0x2919ee9]
14000b9e7: ucomiss xmm0, dword ptr [rax]
14000b9ea: setae r13b
14000b9ee: jmp 0x14000b9f3 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000b9f0: xor r13d, r13d
14000b9f3: mov byte ptr [rbp + 0x816], 1
14000b9fa: mov byte ptr [rbp + 0x815], 1
14000ba01: mov rcx, qword ptr [rbp + 0x118]
14000ba08: call 0x140959ef0 ; country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
14000ba0d: test rax, rax
14000ba10: je 0x14000ba66
14000ba12: mov rax, qword ptr [rbp + 0x558]
14000ba19: test rax, rax
14000ba1c: je 0x140011818
14000ba22: mov rcx, qword ptr [rbp + 0x550]
14000ba29: lea rax, [rcx + rax*4]
14000ba2d: add rax, -4
14000ba31: test rax, rax
14000ba34: je 0x140011818
14000ba3a: movd xmm0, dword ptr [rip + 0x2919e92]
14000ba42: cmp byte ptr [rbp + 0x760], 0
14000ba49: jne 0x14000ba53
14000ba4b: movd xmm0, dword ptr [rip + 0x2919e85]
14000ba53: movss xmm1, dword ptr [rip + 0x2919e75]
14000ba5b: ucomiss xmm1, dword ptr [rax]
14000ba5e: jb 0x14000ba7f
14000ba60: mov ax, 0x100
14000ba64: jmp 0x14000ba81 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000ba66: movd xmm0, dword ptr [rip + 0x2919e66]
14000ba6e: cmp byte ptr [rbp + 0x760], 0
14000ba75: jne 0x14000ba7f
14000ba77: movd xmm0, dword ptr [rip + 0x2919e59]
14000ba7f: xor eax, eax
14000ba81: movzx ecx, r13b
14000ba85: movzx eax, ax
14000ba88: or eax, ecx
14000ba8a: mov byte ptr [rbp + 0x816], 1
14000ba91: mov byte ptr [rbp + 0x815], 1
14000ba98: mov word ptr [rsp + 0x50], ax
14000ba9d: mov qword ptr [rsp + 0x48], r12
14000baa2: mov qword ptr [rsp + 0x28], r14
14000baa7: mov qword ptr [rsp + 0x20], rsi
14000baac: mov qword ptr [rsp + 0x78], rbx
14000bab1: mov rax, qword ptr [rbp + 0x7f0]
14000bab8: mov qword ptr [rsp + 0x70], rax
14000babd: mov rax, qword ptr [rbp + 0x240]
14000bac4: mov qword ptr [rsp + 0x68], rax
14000bac9: movd dword ptr [rsp + 0x60], xmm0
14000bacf: mov eax, dword ptr [rbp + 0x7b8]
14000bad5: mov byte ptr [rsp + 0x58], al
14000bad9: mov rax, qword ptr [rbp + 0x758]
14000bae0: mov dword ptr [rsp + 0x40], eax
14000bae4: lea rax, [rbp + 0x140]
14000baeb: mov qword ptr [rsp + 0x38], rax
14000baf0: lea rax, [rbp + 0x428]
14000baf7: mov qword ptr [rsp + 0x30], rax
14000bafc: mov byte ptr [rsp + 0x80], 0
14000bb04: mov r8, rdi
14000bb07: mov r9, r15
14000bb0a: mov rcx, qword ptr [rbp + 0x800]
14000bb11: mov rdx, qword ptr [rbp + 0x780]
14000bb18: call 0x1407da0b0 ; system_decorator::decorator_visual::stairs_assemble::construct_stair_steps
14000bb1d: movzx esi, byte ptr [rbp + 0x80f]
14000bb24: cmp sil, 2
14000bb28: mov r14d, 0
14000bb2e: mov r13, qword ptr [rbp + 0x7e0]
14000bb35: mov r15, qword ptr [rbp + 0x7f0]
14000bb3c: jae 0x14000bd9c
14000bb42: mov r8d, dword ptr [r13 + 0x128]
14000bb49: mov r9d, dword ptr [r13 + 0x12c]
14000bb50: mov dword ptr [rbp + 0x6a0], r8d
14000bb57: mov dword ptr [rbp + 0x6a4], r9d
14000bb5e: mov rax, qword ptr [rbp + 0x7c0]
14000bb65: mov rdi, qword ptr [rax + 0x70]
14000bb69: test rdi, rdi
14000bb6c: je 0x14000bd95
14000bb72: cmp rdi, 1
14000bb76: jne 0x14000bb9b
14000bb78: mov rax, qword ptr [rbp + 0x7c0]
14000bb7f: mov rax, qword ptr [rax + 0x68]
14000bb83: cmp dword ptr [rax + 8], r8d
14000bb87: jne 0x14000bd95
14000bb8d: cmp dword ptr [rax + 0xc], r9d
14000bb91: jne 0x14000bd95
14000bb97: xor edx, edx
14000bb99: jmp 0x14000bc0b ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000bb9b: mov rax, qword ptr [rbp + 0x7c0]
14000bba2: mov rcx, qword ptr [rax + 0x98]
14000bba9: mov rdx, qword ptr [rax + 0xa0]
14000bbb0: call 0x140106840 ; indexmap::map::IndexMap::hash<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,std::hash::random::RandomState,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000bbb5: mov byte ptr [rbp + 0x816], 1
14000bbbc: mov byte ptr [rbp + 0x815], 1
14000bbc3: mov rcx, qword ptr [rbp + 0x350]
14000bbca: mov rdx, rax
14000bbcd: lea r8, [rbp + 0x6a0]
14000bbd4: call 0x1403ab1c0 ; indexmap::map::core::IndexMapCore::get_index_of<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >
14000bbd9: cmp rax, 1
14000bbdd: mov r14d, 0
14000bbe3: mov r13, qword ptr [rbp + 0x7e0]
14000bbea: mov r15, qword ptr [rbp + 0x7f0]
14000bbf1: jne 0x14000bd95
14000bbf7: cmp rdx, rdi
14000bbfa: jae 0x14001191b
14000bc00: mov rax, qword ptr [rbp + 0x7c0]
14000bc07: mov rax, qword ptr [rax + 0x68]
14000bc0b: lea rcx, [rdx + rdx*4]
14000bc0f: cmp byte ptr [rax + rcx*8 + 0x10], 0
14000bc14: jne 0x14000bd95
14000bc1a: movzx edx, byte ptr [rax + rcx*8 + 0x12]
14000bc1f: or dl, byte ptr [rax + rcx*8 + 0x11]
14000bc23: test dl, 1
14000bc26: je 0x14000bd95
14000bc2c: mov ecx, dword ptr [r13 + 0x128]
14000bc33: mov edx, dword ptr [r13 + 0x12c]
14000bc3a: mov byte ptr [rbp + 0x816], 1
14000bc41: mov byte ptr [rbp + 0x815], 1
14000bc48: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
14000bc4d: mov rdi, rax
14000bc50: mov qword ptr [rbp + 0x6a0], rax
14000bc57: mov rax, qword ptr [rbp + 0x7c0]
14000bc5e: cmp qword ptr [rax + 0x1b0], 0
14000bc66: mov r14d, 0
14000bc6c: mov r15, qword ptr [rbp + 0x7f0]
14000bc73: je 0x14000bd8e
14000bc79: mov rcx, qword ptr [rbp + 0xc0]
14000bc80: lea rdx, [rbp + 0x6a0]
14000bc87: call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> >
14000bc8c: mov rdx, qword ptr [rbp + 0x7c0]
14000bc93: mov rcx, qword ptr [rdx + 0x198]
14000bc9a: mov rdx, qword ptr [rdx + 0x1a0]
14000bca1: mov r8, rdx
14000bca4: and r8, rax
14000bca7: shr rax, 0x39
14000bcab: movd xmm0, eax
14000bcaf: punpcklbw xmm0, xmm0
14000bcb3: pshuflw xmm0, xmm0, 0
14000bcb8: pshufd xmm0, xmm0, 0
14000bcbd: lea rax, [rcx - 0x20]
14000bcc1: xor r9d, r9d
14000bcc4: movdqu xmm1, xmmword ptr [rcx + r8]
14000bcca: movdqa xmm2, xmm1
14000bcce: pcmpeqb xmm2, xmm0
14000bcd2: pmovmskb r10d, xmm2
14000bcd7: test r10d, r10d
14000bcda: je 0x14000bd0c
14000bcdc: tzcnt r11d, r10d
14000bce1: add r11, r8
14000bce4: and r11, rdx
14000bce7: mov rsi, r11
14000bcea: shl rsi, 5
14000bcee: mov r14, rax
14000bcf1: sub r14, rsi
14000bcf4: cmp rdi, qword ptr [r14]
14000bcf7: je 0x14000bd2e
14000bcf9: lea r11d, [r10 - 1]
14000bcfd: and r11w, r10w
14000bd01: mov r10d, r11d
14000bd04: mov r14d, 0
14000bd0a: jne 0x14000bcdc
14000bd0c: pcmpeqb xmm1, xmmword ptr [rip + 0x2919c7c]
14000bd14: pmovmskb r10d, xmm1
14000bd19: test r10d, r10d
14000bd1c: jne 0x14000bd8e
14000bd1e: add r8, r9
14000bd21: add r8, 0x10
14000bd25: add r9, 0x10
14000bd29: and r8, rdx
14000bd2c: jmp 0x14000bcc4 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000bd2e: neg r11
14000bd31: shl r11, 5
14000bd35: mov rax, qword ptr [rcx + r11 - 8]
14000bd3a: cmp rax, 2
14000bd3e: mov r14d, 0
14000bd44: jb 0x14000bd52
14000bd46: mov rdx, qword ptr [rcx + r11 - 0x18]
14000bd4b: mov rax, qword ptr [rcx + r11 - 0x10]
14000bd50: jmp 0x14000bd5a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000bd52: lea rdx, [rcx + r11]
14000bd56: add rdx, -0x18
14000bd5a: shl rax, 2
14000bd5e: xor ecx, ecx
14000bd60: cmp rax, rcx
14000bd63: je 0x14000bd8e
14000bd65: cmp byte ptr [rdx + rcx], 3
14000bd69: lea rcx, [rcx + 4]
14000bd6d: jne 0x14000bd60
14000bd6f: mov sil, 1
14000bd72: mov r13, qword ptr [rbp + 0x7e0]
14000bd79: mov rdx, qword ptr [rbp + 0x798]
14000bd80: test rdx, rdx
14000bd83: jne 0x14000c308
14000bd89: jmp 0x14000c31a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000bd8e: mov r13, qword ptr [rbp + 0x7e0]
14000bd95: movzx esi, byte ptr [rbp + 0x80f]
14000bd9c: cmp sil, 4
14000bda0: jne 0x14000c281
14000bda6: mov rcx, qword ptr [rbp + 0x558]
14000bdad: test rcx, rcx
14000bdb0: mov rdx, qword ptr [rbp + 0x798]
14000bdb7: je 0x140011622
14000bdbd: mov rax, qword ptr [rbp + 0x550]
14000bdc4: lea rcx, [rax + rcx*4]
14000bdc8: add rcx, -4
14000bdcc: je 0x140011645
14000bdd2: movss xmm0, dword ptr [rax]
14000bdd6: movss xmm1, dword ptr [rcx]
14000bdda: movaps xmm2, xmm0
14000bddd: cmpunordss xmm2, xmm0
14000bde2: movaps xmm3, xmm2
14000bde5: andps xmm3, xmm1
14000bde8: maxss xmm1, xmm0
14000bdec: andnps xmm2, xmm1
14000bdef: orps xmm2, xmm3
14000bdf2: ucomiss xmm2, dword ptr [rip + 0x2919ad7]
14000bdf9: setbe al
14000bdfc: or al, byte ptr [rbp + 0x538]
14000be02: mov sil, 1
14000be05: jne 0x14000c303
14000be0b: mov qword ptr [rbp + 0x460], 0
14000be16: mov qword ptr [rbp + 0x468], 1
14000be21: movq xmm0, qword ptr [rbp + 0x690]
14000be29: pshufd xmm0, xmm0, 0x44
14000be2e: paddq xmm0, xmmword ptr [rip + 0x2919ada]
14000be36: movdqu xmmword ptr [rbp + 0x470], xmm0
14000be3e: xor r13d, r13d
14000be41: xor esi, esi
14000be43: nop word ptr cs:[rax + rax]
14000be50: cmp r13, 0x20
14000be54: je 0x14000c39c
14000be5a: mov r14, qword ptr [rbp + r13 + 0x468]
14000be62: cmp r14, -1
14000be66: je 0x14001108e
14000be6c: mov rcx, qword ptr [rbp + r13 + 0x460]
14000be74: lea r9, [r14 + 1]
14000be78: mov rdi, r9
14000be7b: mov qword ptr [rbp + 0x800], rcx
14000be82: sub rdi, rcx
14000be85: jb 0x1400110ad
14000be8b: mov rdx, qword ptr [rbp + 0x690]
14000be92: cmp r14, rdx
14000be95: jae 0x1400110d6
14000be9b: mov qword ptr [rbp + 0x760], r9
14000bea2: mov qword ptr [rbp + 0x780], rsi
14000bea9: mov rax, rdi
14000beac: mov ecx, 0xc
14000beb1: mul rcx
14000beb4: jo 0x14001156a
14000beba: mov esi, 0
14000bebf: movabs rcx, 0x7ffffffffffffffc
14000bec9: cmp rax, rcx
14000becc: mov r15, qword ptr [rbp + 0x108]
14000bed3: ja 0x140011573
14000bed9: mov r12, qword ptr [rbp + 0x688]
14000bee0: test rax, rax
14000bee3: je 0x14000bf20
14000bee5: movzx ecx, byte ptr [rip + 0x3628d1d]
14000beec: mov esi, 4
14000bef1: mov edx, 4
14000bef6: mov rcx, rax
14000bef9: mov r15, rax
14000befc: call 0x140613c10 ; ?
14000bf01: test rax, rax
14000bf04: je 0x140011573
14000bf0a: mov rsi, rdi
14000bf0d: mov r15, rax
14000bf10: jmp 0x14000bf28 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000bf12: nop word ptr cs:[rax + rax]
14000bf20: mov r15d, 4
14000bf26: xor esi, esi
14000bf28: mov rax, qword ptr [rbp + 0x800]
14000bf2f: lea rax, [rax + rax*2]
14000bf33: lea rdx, [r12 + rax*4]
14000bf37: lea r12, [rdi*4]
14000bf3f: lea r8, [r12 + r12*2]
14000bf43: mov rcx, r15
14000bf46: call 0x142923c00 ; ?
14000bf4b: mov qword ptr [rbp + 0x6a0], rsi
14000bf52: mov qword ptr [rbp + 0x108], r15
14000bf59: mov qword ptr [rbp + 0x6a8], r15
14000bf60: mov qword ptr [rbp + 0x6b0], rdi
14000bf67: mov byte ptr [rbp + 0x816], 1
14000bf6e: mov byte ptr [rbp + 0x815], 1
14000bf75: lea rcx, [rbp + 0x18]
14000bf79: lea rdx, [rbp + 0x6a0]
14000bf80: mov r8d, 1
14000bf86: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000bf8b: add r13, 0x10
14000bf8f: mov rax, qword ptr [rbp + 0x780]
14000bf96: lea rsi, [rax + 1]
14000bf9a: xor eax, eax
14000bf9c: cmp rax, qword ptr [rbp + 0x18]
14000bfa0: jo 0x14000be50
14000bfa6: mov rax, qword ptr [rbp + 0x340]
14000bfad: mov ecx, dword ptr [rbp + 0x53c]
14000bfb3: mov dword ptr [rax], ecx
14000bfb5: mov rax, qword ptr [rbp + 0x48]
14000bfb9: mov qword ptr [rbp + 0x5c0], rax
14000bfc0: movups xmm0, xmmword ptr [rbp + 0x18]
14000bfc4: movups xmm1, xmmword ptr [rbp + 0x28]
14000bfc8: movdqu xmm2, xmmword ptr [rbp + 0x38]
14000bfcd: movdqa xmmword ptr [rbp + 0x5b0], xmm2
14000bfd5: movaps xmmword ptr [rbp + 0x5a0], xmm1
14000bfdc: movaps xmmword ptr [rbp + 0x590], xmm0
14000bfe3: mov rdx, qword ptr [rbp + 0x558]
14000bfea: cmp r14, rdx
14000bfed: jae 0x1400112b1
14000bff3: mov rax, qword ptr [rbp + 0x800]
14000bffa: lea r15, [rax*4]
14000c002: add r15, qword ptr [rbp + 0x550]
14000c009: test rdi, rdi
14000c00c: je 0x14000c034
14000c00e: movzx eax, byte ptr [rip + 0x3628bf4]
14000c015: mov edx, 4
14000c01a: mov rcx, r12
14000c01d: call 0x140613c10 ; ?
14000c022: test rax, rax
14000c025: je 0x1400118de
14000c02b: mov qword ptr [rbp + 0x7a0], rdi
14000c032: jmp 0x14000c044 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000c034: mov eax, 4
14000c039: mov qword ptr [rbp + 0x7a0], 0
14000c044: mov qword ptr [rbp + 0x138], rax
14000c04b: mov rcx, rax
14000c04e: mov rdx, r15
14000c051: mov r8, r12
14000c054: call 0x142923c00 ; ?
14000c059: mov rdx, qword ptr [rbp + 0x660]
14000c060: cmp r14, rdx
14000c063: jae 0x1400112c9
14000c069: mov rax, rdi
14000c06c: shr rax, 0x3c
14000c070: jne 0x140011811
14000c076: mov rax, qword ptr [rbp + 0x800]
14000c07d: lea rdx, [rax*8]
14000c085: add rdx, qword ptr [rbp + 0x658]
14000c08c: lea r8, [rdi*8]
14000c094: test rdi, rdi
14000c097: je 0x14000c0d3
14000c099: mov qword ptr [rbp + 0x670], rdx
14000c0a0: movzx eax, byte ptr [rip + 0x3628b62]
14000c0a7: mov edx, 4
14000c0ac: mov r15, r8
14000c0af: mov rcx, r8
14000c0b2: call 0x140613c10 ; ?
14000c0b7: test rax, rax
14000c0ba: je 0x1400118f7
14000c0c0: mov qword ptr [rbp + 0x7a8], rdi
14000c0c7: mov r8, r15
14000c0ca: mov rdx, qword ptr [rbp + 0x670]
14000c0d1: jmp 0x14000c0e3 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000c0d3: mov eax, 4
14000c0d8: mov qword ptr [rbp + 0x7a8], 0
14000c0e3: mov qword ptr [rbp + 0x308], rax
14000c0ea: mov rcx, rax
14000c0ed: call 0x142923c00 ; ?
14000c0f2: cmp r14, rbx
14000c0f5: jae 0x1400112e1
14000c0fb: test rdi, rdi
14000c0fe: je 0x14000c125
14000c100: movzx eax, byte ptr [rip + 0x3628b02]
14000c107: mov edx, 4
14000c10c: mov rcx, r12
14000c10f: call 0x140613c10 ; ?
14000c114: test rax, rax
14000c117: je 0x1400118c5
14000c11d: mov r15, rax
14000c120: mov r14, rdi
14000c123: jmp 0x14000c12e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000c125: mov r15d, 4
14000c12b: xor r14d, r14d
14000c12e: mov rax, qword ptr [rbp + 0x800]
14000c135: mov rdx, qword ptr [rbp + 0x7f0]
14000c13c: lea rdx, [rdx + rax*4]
14000c140: mov rcx, r15
14000c143: mov r8, r12
14000c146: call 0x142923c00 ; ?
14000c14b: cmp qword ptr [rbp + 0x780], 1
14000c153: mov al, 2
14000c155: sbb al, 0
14000c157: movaps xmm0, xmmword ptr [rbp + 0x590]
14000c15e: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14000c165: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14000c16d: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000c174: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000c17b: movdqa xmmword ptr [rbp + 0x6c0], xmm2
14000c183: mov rcx, qword ptr [rbp + 0x5c0]
14000c18a: mov qword ptr [rbp + 0x6d0], rcx
14000c191: mov rcx, qword ptr [rbp + 0x7a8]
14000c198: mov qword ptr [rbp + 0x6d8], rcx
14000c19f: mov rcx, qword ptr [rbp + 0x308]
14000c1a6: mov qword ptr [rbp + 0x6e0], rcx
14000c1ad: mov qword ptr [rbp + 0x6e8], rdi
14000c1b4: mov rcx, qword ptr [rbp + 0x7a0]
14000c1bb: mov qword ptr [rbp + 0x6f0], rcx
14000c1c2: mov rcx, qword ptr [rbp + 0x138]
14000c1c9: mov qword ptr [rbp + 0x6f8], rcx
14000c1d0: mov qword ptr [rbp + 0x700], rdi
14000c1d7: mov qword ptr [rbp + 0x708], r14
14000c1de: mov qword ptr [rbp + 0x710], r15
14000c1e5: mov qword ptr [rbp + 0x718], rdi
14000c1ec: mov rcx, qword ptr [rbp + 0x438]
14000c1f3: mov qword ptr [rbp + 0x720], rcx
14000c1fa: mov word ptr [rbp + 0x734], 0
14000c203: mov rcx, qword ptr [rbp + 0x758]
14000c20a: mov dword ptr [rbp + 0x730], ecx
14000c210: mov rcx, qword ptr [rbp + 0x678]
14000c217: mov rcx, qword ptr [rcx]
14000c21a: mov qword ptr [rbp + 0x728], rcx
14000c221: mov byte ptr [rbp + 0x73b], al
14000c227: mov dword ptr [rbp + 0x736], 0
14000c231: mov byte ptr [rbp + 0x73a], 0
14000c238: mov r14, qword ptr [rbp + 0x370]
14000c23f: mov rdi, qword ptr [r14 + 0x10]
14000c243: cmp rdi, qword ptr [r14]
14000c246: jne 0x14000c257
14000c248: mov rcx, r14
14000c24b: lea rdx, [rip + 0x291a856]
14000c252: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
14000c257: lea rcx, [rdi + rdi*4]
14000c25b: shl rcx, 5
14000c25f: add rcx, qword ptr [r14 + 8]
14000c263: mov r8d, 0xa0
14000c269: lea rdx, [rbp + 0x6a0]
14000c270: call 0x142923c10 ; ?
14000c275: inc rdi
14000c278: mov qword ptr [r14 + 0x10], rdi
14000c27c: jmp 0x14000be50 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000c281: cmp byte ptr [r13 + 0xe8], 0
14000c289: lea rcx, [rbp + 0x460]
14000c290: jne 0x14000c2a0
14000c292: cmp byte ptr [r13 + 0x108], 0
14000c29a: je 0x14000c678
14000c2a0: mov rax, qword ptr [rbp + 0x690]
14000c2a7: mov qword ptr [rbp + 0x6b0], rax
14000c2ae: movaps xmm0, xmmword ptr [rbp + 0x680]
14000c2b5: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000c2bc: mov byte ptr [rbp + 0x816], 0
14000c2c3: mov byte ptr [rbp + 0x815], 1
14000c2ca: lea rdx, [rbp + 0x6a0]
14000c2d1: mov r8d, 1
14000c2d7: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000c2dc: xor r14d, r14d
14000c2df: cmp r14, qword ptr [rbp + 0x460]
14000c2e6: jno 0x14000c3c5
14000c2ec: xor esi, esi
14000c2ee: mov r13, qword ptr [rbp + 0x7e0]
14000c2f5: mov rdx, qword ptr [rbp + 0x798]
14000c2fc: mov r15, qword ptr [rbp + 0x7f0]
14000c303: test rdx, rdx
14000c306: je 0x14000c31a
14000c308: shl rdx, 2
14000c30c: mov r8d, 4
14000c312: mov rcx, r15
14000c315: call 0x140613c20 ; ?
14000c31a: test sil, sil
14000c31d: mov r15, qword ptr [rbp + 0x7b0]
14000c324: je 0x14000c34c
14000c326: mov rax, qword ptr [rbp + 0x680]
14000c32d: test rax, rax
14000c330: je 0x14000c34c
14000c332: mov rcx, qword ptr [rbp + 0x688]
14000c339: shl rax, 2
14000c33d: lea rdx, [rax + rax*2]
14000c341: mov r8d, 4
14000c347: call 0x140613c20 ; ?
14000c34c: mov rdx, qword ptr [rbp + 0x650]
14000c353: test rdx, rdx
14000c356: je 0x14000c36e
14000c358: mov rcx, qword ptr [rbp + 0x658]
14000c35f: shl rdx, 3
14000c363: mov r8d, 4
14000c369: call 0x140613c20 ; ?
14000c36e: mov rdx, qword ptr [rbp + 0x548]
14000c375: test rdx, rdx
14000c378: je 0x14000c390
14000c37a: mov rcx, qword ptr [rbp + 0x550]
14000c381: shl rdx, 2
14000c385: mov r8d, 4
14000c38b: call 0x140613c20 ; ?
14000c390: mov rax, qword ptr [rbp + 0x778]
14000c397: jmp 0x140009be5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000c39c: xor r14d, r14d
14000c39f: mov r13, qword ptr [rbp + 0x7e0]
14000c3a6: mov rdx, qword ptr [rbp + 0x798]
14000c3ad: mov r15, qword ptr [rbp + 0x7f0]
14000c3b4: mov sil, 1
14000c3b7: test rdx, rdx
14000c3ba: jne 0x14000c308
14000c3c0: jmp 0x14000c31a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000c3c5: mov rax, qword ptr [rbp + 0x490]
14000c3cc: mov qword ptr [rbp + 0x5c0], rax
14000c3d3: movups xmm0, xmmword ptr [rbp + 0x460]
14000c3da: movups xmm1, xmmword ptr [rbp + 0x470]
14000c3e1: movdqu xmm2, xmmword ptr [rbp + 0x480]
14000c3e9: movdqa xmmword ptr [rbp + 0x5b0], xmm2
14000c3f1: movaps xmmword ptr [rbp + 0x5a0], xmm1
14000c3f8: movaps xmmword ptr [rbp + 0x590], xmm0
14000c3ff: mov rax, qword ptr [rbp + 0x548]
14000c406: mov qword ptr [rbp + 0x800], rax
14000c40d: mov rcx, qword ptr [rbp + 0x550]
14000c414: mov rsi, qword ptr [rbp + 0x558]
14000c41b: mov rax, qword ptr [rbp + 0x650]
14000c422: mov qword ptr [rbp + 0x780], rax
14000c429: mov rdx, qword ptr [rbp + 0x658]
14000c430: mov r14, qword ptr [rbp + 0x660]
14000c437: mov dil, 1
14000c43a: mov r13, qword ptr [rbp + 0x7e0]
14000c441: cmp byte ptr [r13 + 0xe8], 1
14000c449: mov r15b, 1
14000c44c: mov r12, qword ptr [rbp + 0x678]
14000c453: mov qword ptr [rbp + 0x760], rcx
14000c45a: mov qword ptr [rbp + 0x530], rdx
14000c461: je 0x14000c495
14000c463: mov r8d, dword ptr [r13 + 0x128]
14000c46a: mov r9d, dword ptr [r13 + 0x12c]
14000c471: mov rcx, qword ptr [rbp + 0x950]
14000c478: mov rdx, qword ptr [rbp + 0x2e0]
14000c47f: call 0x1400275b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$9<true>
14000c484: mov r15d, eax
14000c487: mov rcx, qword ptr [rbp + 0x760]
14000c48e: mov rdx, qword ptr [rbp + 0x530]
14000c495: cmp byte ptr [r13 + 0x108], 1
14000c49d: je 0x14000c4d0
14000c49f: mov r9d, dword ptr [r13 + 0x128]
14000c4a6: mov r8d, dword ptr [r13 + 0x12c]
14000c4ad: mov rcx, qword ptr [rbp + 0x950]
14000c4b4: mov rdx, qword ptr [rbp + 0x2e0]
14000c4bb: call 0x1400275b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$9<true>
14000c4c0: mov edi, eax
14000c4c2: mov rcx, qword ptr [rbp + 0x760]
14000c4c9: mov rdx, qword ptr [rbp + 0x530]
14000c4d0: mov rax, qword ptr [rbp + 0x5c0]
14000c4d7: mov qword ptr [rbp + 0x6d0], rax
14000c4de: movaps xmm0, xmmword ptr [rbp + 0x590]
14000c4e5: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14000c4ec: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14000c4f4: movdqa xmmword ptr [rbp + 0x6c0], xmm2
14000c4fc: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000c503: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000c50a: mov rax, qword ptr [rbp + 0x780]
14000c511: mov qword ptr [rbp + 0x6d8], rax
14000c518: mov qword ptr [rbp + 0x6e0], rdx
14000c51f: mov qword ptr [rbp + 0x6e8], r14
14000c526: mov rax, qword ptr [rbp + 0x800]
14000c52d: mov qword ptr [rbp + 0x6f0], rax
14000c534: mov qword ptr [rbp + 0x6f8], rcx
14000c53b: mov qword ptr [rbp + 0x700], rsi
14000c542: mov rax, qword ptr [rbp + 0x798]
14000c549: mov qword ptr [rbp + 0x708], rax
14000c550: mov rax, qword ptr [rbp + 0x7f0]
14000c557: mov qword ptr [rbp + 0x710], rax
14000c55e: mov qword ptr [rbp + 0x718], rbx
14000c565: mov rax, qword ptr [rbp + 0x438]
14000c56c: mov qword ptr [rbp + 0x720], rax
14000c573: mov rax, qword ptr [r12]
14000c577: mov qword ptr [rbp + 0x728], rax
14000c57e: mov rax, qword ptr [rbp + 0x758]
14000c585: mov dword ptr [rbp + 0x730], eax
14000c58b: mov byte ptr [rbp + 0x734], r15b
14000c592: mov byte ptr [rbp + 0x735], dil
14000c599: mov word ptr [rbp + 0x736], 0
14000c5a2: movzx eax, byte ptr [rbp + 0x7a8]
14000c5a9: mov byte ptr [rbp + 0x738], al
14000c5af: movzx eax, byte ptr [rbp + 0x7a0]
14000c5b6: mov byte ptr [rbp + 0x739], al
14000c5bc: mov word ptr [rbp + 0x73a], 0x300
14000c5c5: mov rax, qword ptr [rbp + 0x670]
14000c5cc: mov qword ptr [rbp + 0x740], rax
14000c5d3: mov rax, qword ptr [rbp + 0x7b0]
14000c5da: mov qword ptr [rbp + 0x748], rax
14000c5e1: mov byte ptr [rbp + 0x750], 0
14000c5e8: mov rax, qword ptr [rbp + 0x440]
14000c5ef: mov rsi, qword ptr [rbp + 0x458]
14000c5f6: cmp rsi, rax
14000c5f9: mov r14d, 0
14000c5ff: jne 0x14000c622
14000c601: lea rcx, [rbp + 0x440]
14000c608: lea rdx, [rip + 0x291a3f1]
14000c60f: call 0x1405114a0 ; alloc::collections::vec_deque::VecDeque::grow<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
14000c614: mov rax, qword ptr [rbp + 0x440]
14000c61b: mov rsi, qword ptr [rbp + 0x458]
14000c622: mov rcx, qword ptr [rbp + 0x450]
14000c629: add rcx, rsi
14000c62c: cmp rcx, rax
14000c62f: mov edx, 0
14000c634: cmovae rdx, rax
14000c638: sub rcx, rdx
14000c63b: imul rcx, rcx, 0xb8
14000c642: add rcx, qword ptr [rbp + 0x448]
14000c649: mov r8d, 0xb8
14000c64f: lea rdx, [rbp + 0x6a0]
14000c656: call 0x142923c10 ; ?
14000c65b: inc rsi
14000c65e: mov qword ptr [rbp + 0x458], rsi
14000c665: mov r15, qword ptr [rbp + 0x7b0]
14000c66c: mov rax, qword ptr [rbp + 0x778]
14000c673: jmp 0x140009be5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000c678: mov rsi, qword ptr [rbp + 0x688]
14000c67f: mov rax, qword ptr [rbp + 0x690]
14000c686: test rax, rax
14000c689: sete dl
14000c68c: lea rax, [rax + rax*2]
14000c690: lea rdi, [rsi + rax*4]
14000c694: cmp rdi, 0xc
14000c698: sete al
14000c69b: or al, dl
14000c69d: jne 0x14000cb45
14000c6a3: mov rdx, qword ptr [rbp + 0x660]
14000c6aa: cmp rdx, 1
14000c6ae: jbe 0x140011940
14000c6b4: movss xmm6, dword ptr [rsi + 8]
14000c6b9: movss xmm0, dword ptr [rip + 0x29191af]
14000c6c1: mulss xmm6, xmm0
14000c6c5: movsd xmm8, qword ptr [rsi]
14000c6ca: movsd xmm7, qword ptr [rdi - 0xc]
14000c6cf: movss xmm9, dword ptr [rdi - 4]
14000c6d5: mulss xmm9, xmm0
14000c6da: mov rax, qword ptr [rbp + 0x658]
14000c6e1: movss xmm0, dword ptr [rax + 8]
14000c6e6: movss xmm1, dword ptr [rax + 0xc]
14000c6eb: xorps xmm1, xmmword ptr [rip + 0x29191fe]
14000c6f2: movss dword ptr [rbp + 0x590], xmm1
14000c6fa: movss dword ptr [rbp + 0x594], xmm0
14000c702: mov byte ptr [rbp + 0x816], 1
14000c709: mov byte ptr [rbp + 0x815], 1
14000c710: lea rcx, [rbp + 0x6a0]
14000c717: lea rdx, [rbp + 0x590]
14000c71e: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14000c723: mov rdx, qword ptr [rbp + 0x558]
14000c72a: cmp rdx, 1
14000c72e: jbe 0x140011986
14000c734: mov rax, qword ptr [rbp + 0x550]
14000c73b: movss xmm12, dword ptr [rax + 4]
14000c741: mulss xmm12, dword ptr [rip + 0x2919126]
14000c74a: addss xmm12, dword ptr [rip + 0x29191ad]
14000c753: mov rdx, qword ptr [rbp + 0x660]
14000c75a: cmp rdx, 1
14000c75e: jbe 0x1400119aa
14000c764: movsd xmm10, qword ptr [rbp + 0x6a0]
14000c76d: movss xmm11, dword ptr [rbp + 0x6a8]
14000c776: mulss xmm11, xmm12
14000c77b: mov rax, qword ptr [rbp + 0x658]
14000c782: movss xmm0, dword ptr [rax + 8]
14000c787: movss xmm1, dword ptr [rax + 0xc]
14000c78c: xorps xmm1, xmmword ptr [rip + 0x291915d]
14000c793: movss dword ptr [rbp + 0x590], xmm1
14000c79b: movss dword ptr [rbp + 0x594], xmm0
14000c7a3: mov byte ptr [rbp + 0x816], 1
14000c7aa: mov byte ptr [rbp + 0x815], 1
14000c7b1: lea rcx, [rbp + 0x6a0]
14000c7b8: lea rdx, [rbp + 0x590]
14000c7bf: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14000c7c4: mov rdx, qword ptr [rbp + 0x558]
14000c7cb: cmp rdx, 1
14000c7cf: jbe 0x1400119cb
14000c7d5: mov rax, qword ptr [rbp + 0x550]
14000c7dc: movss xmm14, dword ptr [rax + 4]
14000c7e2: movsd xmm13, qword ptr [rbp + 0x6a0]
14000c7eb: movss xmm15, dword ptr [rbp + 0x6a8]
14000c7f4: movzx eax, byte ptr [rip + 0x362840e]
14000c7fb: mov ecx, 0x24
14000c800: mov edx, 4
14000c805: call 0x140613c10 ; ?
14000c80a: test rax, rax
14000c80d: lea rcx, [rbp + 0x460]
14000c814: je 0x140011964
14000c81a: movaps xmm0, xmmword ptr [rip + 0x29190bf]
14000c821: mulps xmm8, xmm0
14000c825: mulps xmm7, xmm0
14000c828: addps xmm7, xmm8
14000c82c: addss xmm6, xmm9
14000c831: movsldup xmm0, xmm12
14000c836: mulps xmm10, xmm0
14000c83a: addps xmm10, xmm7
14000c83e: addss xmm11, xmm6
14000c843: mulss xmm14, dword ptr [rip + 0x2919024]
14000c84c: addss xmm14, dword ptr [rip + 0x29190ab]
14000c855: mulss xmm15, xmm14
14000c85a: movlps qword ptr [rax], xmm10
14000c85e: movss dword ptr [rax + 8], xmm11
14000c864: movlps qword ptr [rax + 0xc], xmm7
14000c868: movss dword ptr [rax + 0x14], xmm6
14000c86d: subss xmm6, xmm15
14000c872: movsldup xmm0, xmm14
14000c877: mulps xmm13, xmm0
14000c87b: subps xmm7, xmm13
14000c87f: movlps qword ptr [rax + 0x18], xmm7
14000c883: movss dword ptr [rax + 0x20], xmm6
14000c888: movss xmm6, dword ptr [rsi]
14000c88c: subss xmm6, dword ptr [rdi - 0xc]
14000c891: movss xmm7, dword ptr [rsi + 8]
14000c896: subss xmm7, dword ptr [rdi - 4]
14000c89b: mov qword ptr [rbp + 0x6a0], 3
14000c8a6: mov qword ptr [rbp + 0x6a8], rax
14000c8ad: mov qword ptr [rbp + 0x6b0], 3
14000c8b8: mov byte ptr [rbp + 0x816], 1
14000c8bf: mov byte ptr [rbp + 0x815], 1
14000c8c6: lea rdx, [rbp + 0x6a0]
14000c8cd: mov r8d, 1
14000c8d3: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000c8d8: xor eax, eax
14000c8da: cmp rax, qword ptr [rbp + 0x460]
14000c8e1: jo 0x14000cb3e
14000c8e7: mov rax, qword ptr [rbp + 0x490]
14000c8ee: mov qword ptr [rbp + 0x6d0], rax
14000c8f5: movups xmm0, xmmword ptr [rbp + 0x460]
14000c8fc: movups xmm1, xmmword ptr [rbp + 0x470]
14000c903: movups xmm2, xmmword ptr [rbp + 0x480]
14000c90a: movaps xmmword ptr [rbp + 0x6c0], xmm2
14000c911: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000c918: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000c91f: mov r8d, 3
14000c925: lea rcx, [rbp + 0x500]
14000c92c: movss xmm1, dword ptr [rip + 0x2918fd4]
14000c934: lea r9, [rip + 0x291a035]
14000c93b: call 0x1402b6ad0 ; alloc::vec::from_elem<f32>
14000c940: test rbx, rbx
14000c943: movss xmm8, dword ptr [rip + 0x2918fb8]
14000c94c: je 0x140011a30
14000c952: mov rax, qword ptr [rbp + 0x7f0]
14000c959: movss xmm1, dword ptr [rax]
14000c95d: mov r8d, 3
14000c963: lea rcx, [rbp + 0x3e0]
14000c96a: lea r9, [rip + 0x291a02f]
14000c971: call 0x1402b6ad0 ; alloc::vec::from_elem<f32>
14000c976: mov rsi, qword ptr [rbp + 0x678]
14000c97d: movaps xmm1, xmm7
14000c980: xorps xmm1, xmmword ptr [rip + 0x2918f69]
14000c987: mulss xmm7, xmm7
14000c98b: movaps xmm0, xmm6
14000c98e: mulss xmm0, xmm6
14000c992: addss xmm0, xmm7
14000c996: sqrtss xmm0, xmm0
14000c99a: movaps xmm2, xmm8
14000c99e: divss xmm2, xmm0
14000c9a2: mulss xmm1, xmm2
14000c9a6: mulss xmm2, xmm6
14000c9aa: lea rax, [rip + 0x291a007]
14000c9b1: mov qword ptr [rsp + 0x20], rax
14000c9b6: mov r9d, 3
14000c9bc: lea rcx, [rbp + 0x380]
14000c9c3: call 0x1402b6c60 ; alloc::vec::from_elem<glam::f32::vec2::Vec2>
14000c9c8: mov rax, qword ptr [rbp + 0x6d0]
14000c9cf: mov qword ptr [rbp + 0x5c0], rax
14000c9d6: movaps xmm0, xmmword ptr [rbp + 0x6a0]
14000c9dd: movaps xmm1, xmmword ptr [rbp + 0x6b0]
14000c9e4: movaps xmm2, xmmword ptr [rbp + 0x6c0]
14000c9eb: movaps xmmword ptr [rbp + 0x5b0], xmm2
14000c9f2: movaps xmmword ptr [rbp + 0x5a0], xmm1
14000c9f9: movaps xmmword ptr [rbp + 0x590], xmm0
14000ca00: mov rax, qword ptr [rbp + 0x390]
14000ca07: lea rcx, [rbp + 0x598]
14000ca0e: mov qword ptr [rcx + 0x40], rax
14000ca12: movups xmm0, xmmword ptr [rbp + 0x380]
14000ca19: movups xmmword ptr [rcx + 0x30], xmm0
14000ca1d: mov rax, qword ptr [rbp + 0x510]
14000ca24: mov qword ptr [rcx + 0x58], rax
14000ca28: movups xmm0, xmmword ptr [rbp + 0x500]
14000ca2f: movups xmmword ptr [rcx + 0x48], xmm0
14000ca33: mov rax, qword ptr [rbp + 0x3f0]
14000ca3a: mov qword ptr [rcx + 0x70], rax
14000ca3e: movups xmm0, xmmword ptr [rbp + 0x3e0]
14000ca45: movups xmmword ptr [rcx + 0x60], xmm0
14000ca49: movaps xmm0, xmmword ptr [rbp + 0x590]
14000ca50: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14000ca57: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14000ca5f: movapd xmm3, xmmword ptr [rbp + 0x5c0]
14000ca67: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000ca6e: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000ca75: movdqa xmmword ptr [rbp + 0x6c0], xmm2
14000ca7d: movapd xmmword ptr [rbp + 0x6d0], xmm3
14000ca85: movaps xmm0, xmmword ptr [rbp + 0x5d0]
14000ca8c: movaps xmmword ptr [rbp + 0x6e0], xmm0
14000ca93: movaps xmm0, xmmword ptr [rbp + 0x5e0]
14000ca9a: movaps xmmword ptr [rbp + 0x6f0], xmm0
14000caa1: movaps xmm0, xmmword ptr [rbp + 0x5f0]
14000caa8: movaps xmmword ptr [rbp + 0x700], xmm0
14000caaf: movaps xmm0, xmmword ptr [rbp + 0x600]
14000cab6: movaps xmmword ptr [rbp + 0x710], xmm0
14000cabd: mov rax, qword ptr [rbp + 0x438]
14000cac4: mov qword ptr [rbp + 0x720], rax
14000cacb: mov rax, qword ptr [rsi]
14000cace: mov qword ptr [rbp + 0x728], rax
14000cad5: mov rax, qword ptr [rbp + 0x758]
14000cadc: mov dword ptr [rbp + 0x730], eax
14000cae2: mov dword ptr [rbp + 0x734], 0
14000caec: movzx eax, byte ptr [rbp + 0x7a8]
14000caf3: mov byte ptr [rbp + 0x738], al
14000caf9: movzx eax, byte ptr [rbp + 0x7a0]
14000cb00: mov byte ptr [rbp + 0x739], al
14000cb06: mov word ptr [rbp + 0x73a], 0x301
14000cb0f: mov byte ptr [rbp + 0x750], 2
14000cb16: mov byte ptr [rbp + 0x816], 1
14000cb1d: mov byte ptr [rbp + 0x815], 1
14000cb24: lea rcx, [rbp + 0x440]
14000cb2b: lea rdx, [rbp + 0x6a0]
14000cb32: lea r8, [rip + 0x2919e97]
14000cb39: call 0x140511a10 ; alloc::collections::vec_deque::VecDeque::push_back<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
14000cb3e: lea rcx, [rbp + 0x460]
14000cb45: mov rax, qword ptr [rbp + 0x690]
14000cb4c: mov qword ptr [rbp + 0x6b0], rax
14000cb53: movaps xmm0, xmmword ptr [rbp + 0x680]
14000cb5a: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000cb61: mov byte ptr [rbp + 0x816], 0
14000cb68: mov byte ptr [rbp + 0x815], 1
14000cb6f: lea rdx, [rbp + 0x6a0]
14000cb76: mov r8d, 1
14000cb7c: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000cb81: xor r14d, r14d
14000cb84: cmp r14, qword ptr [rbp + 0x460]
14000cb8b: jo 0x14000c2ec
14000cb91: mov rax, qword ptr [rbp + 0x490]
14000cb98: mov qword ptr [rbp + 0x5c0], rax
14000cb9f: movups xmm0, xmmword ptr [rbp + 0x460]
14000cba6: movups xmm1, xmmword ptr [rbp + 0x470]
14000cbad: movups xmm2, xmmword ptr [rbp + 0x480]
14000cbb4: movaps xmmword ptr [rbp + 0x5b0], xmm2
14000cbbb: movaps xmmword ptr [rbp + 0x5a0], xmm1
14000cbc2: movaps xmmword ptr [rbp + 0x590], xmm0
14000cbc9: mov rax, qword ptr [rbp + 0x558]
14000cbd0: lea rcx, [rbp + 0x598]
14000cbd7: mov qword ptr [rcx + 0x58], rax
14000cbdb: movups xmm0, xmmword ptr [rbp + 0x548]
14000cbe2: movups xmmword ptr [rcx + 0x48], xmm0
14000cbe6: mov rax, qword ptr [rbp + 0x660]
14000cbed: mov qword ptr [rcx + 0x40], rax
14000cbf1: movaps xmm0, xmmword ptr [rbp + 0x650]
14000cbf8: movups xmmword ptr [rcx + 0x30], xmm0
14000cbfc: movaps xmm0, xmmword ptr [rbp + 0x590]
14000cc03: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14000cc0a: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
14000cc12: mov rax, qword ptr [rbp + 0x5c0]
14000cc19: mov rcx, qword ptr [rbp + 0x5c8]
14000cc20: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000cc27: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000cc2e: movdqa xmmword ptr [rbp + 0x6c0], xmm2
14000cc36: mov qword ptr [rbp + 0x6d0], rax
14000cc3d: mov qword ptr [rbp + 0x6d8], rcx
14000cc44: movaps xmm0, xmmword ptr [rbp + 0x5d0]
14000cc4b: movaps xmmword ptr [rbp + 0x6e0], xmm0
14000cc52: movaps xmm0, xmmword ptr [rbp + 0x5e0]
14000cc59: movaps xmmword ptr [rbp + 0x6f0], xmm0
14000cc60: mov rax, qword ptr [rbp + 0x5f0]
14000cc67: mov qword ptr [rbp + 0x700], rax
14000cc6e: mov rax, qword ptr [rbp + 0x798]
14000cc75: mov qword ptr [rbp + 0x708], rax
14000cc7c: mov rax, qword ptr [rbp + 0x7f0]
14000cc83: mov qword ptr [rbp + 0x710], rax
14000cc8a: mov qword ptr [rbp + 0x718], rbx
14000cc91: mov rax, qword ptr [rbp + 0x438]
14000cc98: mov qword ptr [rbp + 0x720], rax
14000cc9f: mov rax, qword ptr [rbp + 0x678]
14000cca6: mov rax, qword ptr [rax]
14000cca9: mov qword ptr [rbp + 0x728], rax
14000ccb0: mov rax, qword ptr [rbp + 0x758]
14000ccb7: mov dword ptr [rbp + 0x730], eax
14000ccbd: mov dword ptr [rbp + 0x734], 0
14000ccc7: movzx eax, byte ptr [rbp + 0x7a8]
14000ccce: mov byte ptr [rbp + 0x738], al
14000ccd4: movzx eax, byte ptr [rbp + 0x7a0]
14000ccdb: mov byte ptr [rbp + 0x739], al
14000cce1: mov word ptr [rbp + 0x73a], 0x301
14000ccea: mov byte ptr [rbp + 0x750], 2
14000ccf1: mov byte ptr [rbp + 0x816], 0
14000ccf8: mov byte ptr [rbp + 0x815], 0
14000ccff: lea rcx, [rbp + 0x440]
14000cd06: lea rdx, [rbp + 0x6a0]
14000cd0d: lea r8, [rip + 0x2919cd4]
14000cd14: call 0x140511a10 ; alloc::collections::vec_deque::VecDeque::push_back<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
14000cd19: xor r14d, r14d
14000cd1c: mov r13, qword ptr [rbp + 0x7e0]
14000cd23: mov r15, qword ptr [rbp + 0x7b0]
14000cd2a: mov rax, qword ptr [rbp + 0x778]
14000cd31: jmp 0x140009be5 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000cd36: cmp r14, rcx
14000cd39: je 0x14000cd9a
14000cd3b: movss xmm0, dword ptr [r15 + rax*4]
14000cd41: movaps xmm1, xmm8
14000cd45: subss xmm1, xmm0
14000cd49: movsd xmm2, qword ptr [rbp + 0x540]
14000cd51: unpcklps xmm1, xmm0
14000cd54: mulps xmm1, xmm2
14000cd57: movshdup xmm0, xmm1
14000cd5b: addss xmm0, xmm1
14000cd5f: movss dword ptr [r10 + rax*4], xmm0
14000cd65: movss xmm0, dword ptr [r15 + rax*4 + 4]
14000cd6c: movaps xmm1, xmm8
14000cd70: subss xmm1, xmm0
14000cd74: movsd xmm2, qword ptr [rbp + 0x540]
14000cd7c: unpcklps xmm1, xmm0
14000cd7f: mulps xmm1, xmm2
14000cd82: movshdup xmm0, xmm1
14000cd86: addss xmm0, xmm1
14000cd8a: movss dword ptr [r10 + rax*4 + 4], xmm0
14000cd91: lea rcx, [rax + 2]
14000cd95: mov rax, rcx
14000cd98: jmp 0x14000cd36 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000cd9a: mov qword ptr [rbp + 0x7f0], r10
14000cda1: mov r15, qword ptr [rbp + 0x5a0]
14000cda8: mov rax, qword ptr [rbp + 0x5b8]
14000cdaf: cmp rax, r15
14000cdb2: cmovb r15, rax
14000cdb6: lea rdi, [r15*4]
14000cdbe: mov rax, r15
14000cdc1: shr rax, 0x3e
14000cdc5: jne 0x1400117aa
14000cdcb: mov esi, 0
14000cdd0: movabs rax, 0x7ffffffffffffffc
14000cdda: cmp rdi, rax
14000cddd: ja 0x1400117ac
14000cde3: mov r12, qword ptr [rbp + 0x5b0]
14000cdea: test rdi, rdi
14000cded: je 0x14000ce25
14000cdef: movzx eax, byte ptr [rip + 0x3627e13]
14000cdf6: mov esi, 4
14000cdfb: mov edx, 4
14000ce00: mov rcx, rdi
14000ce03: call 0x140613c10 ; ?
14000ce08: test rax, rax
14000ce0b: je 0x1400117ac
14000ce11: mov rcx, r15
14000ce14: test r15, r15
14000ce17: mov r9, qword ptr [rbp + 0x7f0]
14000ce1e: jne 0x14000ce3c
14000ce20: jmp 0x14000cf3d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000ce25: mov eax, 4
14000ce2a: xor ecx, ecx
14000ce2c: test r15, r15
14000ce2f: mov r9, qword ptr [rbp + 0x7f0]
14000ce36: je 0x14000cf3d
14000ce3c: cmp r15, 8
14000ce40: jb 0x14000ceb9
14000ce42: mov r8, rax
14000ce45: sub r8, r12
14000ce48: mov edx, 0
14000ce4d: cmp r8, 0x20
14000ce51: jb 0x14000cebb
14000ce53: mov rdx, r15
14000ce56: movabs r8, 0x3ffffffffffffff8
14000ce60: and rdx, r8
14000ce63: xor r8d, r8d
14000ce66: xorps xmm4, xmm4
14000ce69: nop dword ptr [rax]
14000ce70: movups xmm0, xmmword ptr [r12 + r8*4]
14000ce75: movups xmm1, xmmword ptr [r12 + r8*4 + 0x10]
14000ce7b: movaps xmm2, xmm10
14000ce7f: subps xmm2, xmm0
14000ce82: movaps xmm3, xmm10
14000ce86: subps xmm3, xmm1
14000ce89: mulps xmm2, xmm4
14000ce8c: mulps xmm3, xmm4
14000ce8f: mulps xmm0, xmm4
14000ce92: addps xmm0, xmm2
14000ce95: mulps xmm1, xmm4
14000ce98: addps xmm1, xmm3
14000ce9b: movups xmmword ptr [rax + r8*4], xmm0
14000cea0: movups xmmword ptr [rax + r8*4 + 0x10], xmm1
14000cea6: add r8, 8
14000ceaa: cmp rdx, r8
14000cead: jne 0x14000ce70
14000ceaf: cmp r15, rdx
14000ceb2: jne 0x14000cebb
14000ceb4: jmp 0x14000cf3d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000ceb9: xor edx, edx
14000cebb: mov r8, rdx
14000cebe: or r8, 1
14000cec2: test r15b, 1
14000cec6: je 0x14000ceeb
14000cec8: movss xmm0, dword ptr [r12 + rdx*4]
14000cece: movaps xmm1, xmm8
14000ced2: subss xmm1, xmm0
14000ced6: mulss xmm1, xmm9
14000cedb: mulss xmm0, xmm9
14000cee0: addss xmm0, xmm1
14000cee4: movss dword ptr [rax + rdx*4], xmm0
14000cee9: jmp 0x14000cf38 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000ceeb: cmp r15, r8
14000ceee: je 0x14000cf3d
14000cef0: movss xmm0, dword ptr [r12 + rdx*4]
14000cef6: movaps xmm1, xmm8
14000cefa: subss xmm1, xmm0
14000cefe: mulss xmm1, xmm9
14000cf03: mulss xmm0, xmm9
14000cf08: addss xmm0, xmm1
14000cf0c: movss dword ptr [rax + rdx*4], xmm0
14000cf11: movss xmm0, dword ptr [r12 + rdx*4 + 4]
14000cf18: movaps xmm1, xmm8
14000cf1c: subss xmm1, xmm0
14000cf20: mulss xmm1, xmm9
14000cf25: mulss xmm0, xmm9
14000cf2a: addss xmm0, xmm1
14000cf2e: movss dword ptr [rax + rdx*4 + 4], xmm0
14000cf34: lea r8, [rdx + 2]
14000cf38: mov rdx, r8
14000cf3b: jmp 0x14000ceeb ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000cf3d: mov rdx, qword ptr [rbp + 0x258]
14000cf44: mov r8d, dword ptr [rbp + 0x424]
14000cf4b: mov dword ptr [rdx], r8d
14000cf4e: mov rdx, qword ptr [rbp + 0x5c0]
14000cf55: mov qword ptr [rbp + 0x6d0], rdx
14000cf5c: movaps xmm0, xmmword ptr [rbp + 0x590]
14000cf63: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14000cf6a: movaps xmm2, xmmword ptr [rbp + 0x5b0]
14000cf71: movaps xmmword ptr [rbp + 0x6c0], xmm2
14000cf78: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000cf7f: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000cf86: mov rdx, qword ptr [rbp + 0x800]
14000cf8d: mov qword ptr [rbp + 0x6d8], rdx
14000cf94: mov rdx, qword ptr [rbp + 0x780]
14000cf9b: mov qword ptr [rbp + 0x6e0], rdx
14000cfa2: mov qword ptr [rbp + 0x6e8], rbx
14000cfa9: mov rdx, qword ptr [rbp + 0x760]
14000cfb0: mov qword ptr [rbp + 0x708], rdx
14000cfb7: mov qword ptr [rbp + 0x710], r9
14000cfbe: mov qword ptr [rbp + 0x718], r14
14000cfc5: mov qword ptr [rbp + 0x6f0], rcx
14000cfcc: mov qword ptr [rbp + 0x6f8], rax
14000cfd3: mov qword ptr [rbp + 0x700], r15
14000cfda: mov rax, qword ptr [rbp + 0x758]
14000cfe1: mov dword ptr [rbp + 0x740], eax
14000cfe7: mov rax, qword ptr [rbp + 0x678]
14000cfee: mov rax, qword ptr [rax]
14000cff1: mov qword ptr [rbp + 0x738], rax
14000cff8: mov word ptr [rbp + 0x744], 0
14000d001: movabs rax, 0x8000000000000000
14000d00b: mov qword ptr [rbp + 0x720], rax
14000d012: mov rsi, qword ptr [r13 + 0x10]
14000d016: cmp rsi, qword ptr [r13]
14000d01a: jne 0x14000d02b
14000d01c: mov rcx, r13
14000d01f: lea rdx, [rip + 0x2919802]
14000d026: call 0x1405636c0 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsBracketAssemblyParams,alloc::alloc::Global>
14000d02b: imul rcx, rsi, 0xa8
14000d032: add rcx, qword ptr [r13 + 8]
14000d036: mov r8d, 0xa8
14000d03c: lea rdx, [rbp + 0x6a0]
14000d043: call 0x142923c10 ; ?
14000d048: inc rsi
14000d04b: mov qword ptr [r13 + 0x10], rsi
14000d04f: mov rax, qword ptr [rbp + 0x200]
14000d056: mov byte ptr [rbp + 0x817], 1
14000d05d: mov qword ptr [rsp + 0x48], rax
14000d062: mov rax, qword ptr [rbp + 0x648]
14000d069: mov qword ptr [rsp + 0x40], rax
14000d06e: lea rax, [rbp + 0x278]
14000d075: mov qword ptr [rsp + 0x38], rax
14000d07a: lea rax, [rbp + 0x540]
14000d081: mov qword ptr [rsp + 0x20], rax
14000d086: mov qword ptr [rsp + 0x30], 1
14000d08f: mov qword ptr [rsp + 0x28], 2
14000d098: mov edx, 2
14000d09d: mov r9d, 2
14000d0a3: lea rcx, [rbp + 0x650]
14000d0aa: lea r8, [rbp + 0x310]
14000d0b1: call 0x1407de8b0 ; system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo
14000d0b6: mov byte ptr [rbp + 0x817], 1
14000d0bd: lea rax, [rbp + 0x428]
14000d0c4: mov qword ptr [rsp + 0x38], rax
14000d0c9: mov eax, dword ptr [rbp + 0x528]
14000d0cf: mov dword ptr [rsp + 0x30], eax
14000d0d3: lea rax, [rbp + 0x7ef]
14000d0da: mov qword ptr [rsp + 0x28], rax
14000d0df: mov qword ptr [rsp + 0x20], 2
14000d0e8: mov r8d, 2
14000d0ee: lea rcx, [rbp + 0x198]
14000d0f5: lea rdx, [rbp + 0x650]
14000d0fc: lea r9, [rbp + 0x540]
14000d103: call 0x1407db3e0 ; system_decorator::decorator_visual::stairs_assemble::construct_ladder
14000d108: movzx eax, byte ptr [rbp + 0x7ef]
14000d10f: cmp eax, 1
14000d112: mov r14d, 0
14000d118: jbe 0x140009bd0
14000d11e: cmp eax, 3
14000d121: je 0x14000d222
14000d127: cmp eax, 4
14000d12a: movss xmm3, dword ptr [rip + 0x29187d2]
14000d132: jne 0x14000d259
14000d138: movaps xmm0, xmm13
14000d13c: movlhps xmm0, xmm12
14000d140: shufps xmm0, xmm12, 0xe2
14000d145: movaps xmm1, xmm15
14000d149: movlhps xmm1, xmm14
14000d14d: shufps xmm1, xmm14, 0xe2
14000d152: subps xmm0, xmm1
14000d155: movaps xmm1, xmm0
14000d158: mulps xmm1, xmm0
14000d15b: movshdup xmm2, xmm1
14000d15f: addss xmm2, xmm1
14000d163: xorps xmm1, xmm1
14000d166: sqrtss xmm1, xmm2
14000d16a: movaps xmm2, xmm3
14000d16d: divss xmm2, xmm1
14000d171: movd eax, xmm2
14000d175: test eax, eax
14000d177: setns cl
14000d17a: mov edx, eax
14000d17c: and edx, 0x7fffffff
14000d182: add edx, 0xff800000
14000d188: cmp edx, 0x7f000000
14000d18e: setb dl
14000d191: and dl, cl
14000d193: dec eax
14000d195: cmp eax, 0x7fffff
14000d19a: setb al
14000d19d: or al, dl
14000d19f: movsldup xmm1, xmm2
14000d1a3: mulps xmm1, xmm0
14000d1a6: movzx eax, al
14000d1a9: movd xmm0, eax
14000d1ad: pshufd xmm11, xmm0, 0x50
14000d1b3: pslld xmm11, 0x1f
14000d1b9: psrad xmm11, 0x1f
14000d1bf: pand xmm11, xmm1
14000d1c4: movq qword ptr [rbp + 0x6a0], xmm11
14000d1cd: mov byte ptr [rbp + 0x817], 1
14000d1d4: lea rcx, [rbp + 0x68]
14000d1d8: lea rdx, [rbp + 0x6a0]
14000d1df: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14000d1e4: movsd xmm0, qword ptr [rbp + 0x68]
14000d1e9: mulps xmm0, xmmword ptr [rip + 0x2918750]
14000d1f0: movss xmm1, dword ptr [rbp + 0x70]
14000d1f5: mulss xmm1, dword ptr [rip + 0x2918753]
14000d1fd: subss xmm15, xmm1
14000d202: subps xmm14, xmm0
14000d206: addps xmm12, xmm0
14000d20a: addss xmm13, xmm1
14000d20f: mov rbx, qword ptr [rbp + 0x540]
14000d216: lea rcx, [rbp + 0x460]
14000d21d: jmp 0x14000d4a2 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d222: movaps xmm0, xmm14
14000d226: cmpltps xmm0, xmm12
14000d22b: pextrw eax, xmm0, 2
14000d230: test al, 1
14000d232: movss xmm3, dword ptr [rip + 0x29186ca]
14000d23a: je 0x14000d26b
14000d23c: mov esi, dword ptr [rbp + 0x540]
14000d242: movaps xmm1, xmm6
14000d245: mulps xmm1, xmm6
14000d248: movshdup xmm0, xmm1
14000d24c: addss xmm0, xmm1
14000d250: mov dil, 1
14000d253: movaps xmm11, xmm6
14000d257: jmp 0x14000d283 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d259: movaps xmm0, xmm12
14000d25d: cmpltps xmm0, xmm14
14000d262: pextrw eax, xmm0, 2
14000d267: test al, 1
14000d269: jne 0x14000d23c
14000d26b: mov esi, dword ptr [rbp + 0x544]
14000d271: movaps xmm1, xmm11
14000d275: mulps xmm1, xmm11
14000d279: movshdup xmm0, xmm1
14000d27d: addss xmm0, xmm1
14000d281: xor edi, edi
14000d283: sqrtss xmm0, xmm0
14000d287: movaps xmm1, xmm3
14000d28a: divss xmm1, xmm0
14000d28e: movd eax, xmm1
14000d292: test eax, eax
14000d294: setns cl
14000d297: mov edx, eax
14000d299: and edx, 0x7fffffff
14000d29f: add edx, 0xff800000
14000d2a5: cmp edx, 0x7f000000
14000d2ab: setb dl
14000d2ae: and dl, cl
14000d2b0: dec eax
14000d2b2: cmp eax, 0x7fffff
14000d2b7: setb al
14000d2ba: or al, dl
14000d2bc: movsldup xmm0, xmm1
14000d2c0: mulps xmm0, xmm11
14000d2c4: movzx eax, al
14000d2c7: movd xmm1, eax
14000d2cb: pshufd xmm1, xmm1, 0x50
14000d2d0: pslld xmm1, 0x1f
14000d2d5: psrad xmm1, 0x1f
14000d2da: pand xmm1, xmm0
14000d2de: movq qword ptr [rbp + 0x590], xmm1
14000d2e6: movshdup xmm0, xmm14
14000d2eb: movshdup xmm1, xmm12
14000d2f0: movaps xmm11, xmm0
14000d2f4: cmpunordss xmm11, xmm0
14000d2fa: movaps xmm2, xmm11
14000d2fe: andps xmm2, xmm1
14000d301: minss xmm1, xmm0
14000d305: andnps xmm11, xmm1
14000d309: orps xmm11, xmm2
14000d30d: movss dword ptr [rbp + 0x6a0], xmm14
14000d316: movss dword ptr [rbp + 0x6a4], xmm15
14000d31f: mov byte ptr [rbp + 0x817], 1
14000d326: lea rcx, [rbp + 0x228]
14000d32d: lea rdx, [rbp + 0x6a0]
14000d334: movaps xmm2, xmm11
14000d338: call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
14000d33d: movss dword ptr [rbp + 0x6a0], xmm12
14000d346: movss dword ptr [rbp + 0x6a4], xmm13
14000d34f: mov byte ptr [rbp + 0x817], 1
14000d356: lea rcx, [rbp + 0x218]
14000d35d: lea rdx, [rbp + 0x6a0]
14000d364: movaps xmm2, xmm11
14000d368: call 0x140c9cde0 ; utils::swizzlers::impl$1::xvy
14000d36d: test dil, dil
14000d370: je 0x14000d3ce
14000d372: mov byte ptr [rbp + 0x817], 1
14000d379: lea rcx, [rbp + 0x6a0]
14000d380: lea rdx, [rbp + 0x590]
14000d387: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14000d38c: movsd xmm12, qword ptr [rbp + 0x6a0]
14000d395: movss xmm13, dword ptr [rbp + 0x6a8]
14000d39e: cmp byte ptr [rbp + 0x7ef], 3
14000d3a5: jne 0x14000d440
14000d3ab: mulps xmm12, xmmword ptr [rip + 0x291852d]
14000d3b3: mulss xmm13, dword ptr [rip + 0x29184b4]
14000d3bc: movsd xmm0, qword ptr [rbp + 0x228]
14000d3c4: movss xmm1, dword ptr [rbp + 0x230]
14000d3cc: jmp 0x14000d424 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d3ce: mov byte ptr [rbp + 0x817], 1
14000d3d5: lea rcx, [rbp + 0x6a0]
14000d3dc: lea rdx, [rbp + 0x590]
14000d3e3: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14000d3e8: movsd xmm12, qword ptr [rbp + 0x6a0]
14000d3f1: movss xmm13, dword ptr [rbp + 0x6a8]
14000d3fa: cmp byte ptr [rbp + 0x7ef], 3
14000d401: jne 0x14000d45d
14000d403: mulps xmm12, xmmword ptr [rip + 0x29184d5]
14000d40b: mulss xmm13, dword ptr [rip + 0x291845c]
14000d414: movsd xmm0, qword ptr [rbp + 0x218]
14000d41c: movss xmm1, dword ptr [rbp + 0x220]
14000d424: movaps xmm14, xmm0
14000d428: subps xmm14, xmm12
14000d42c: movaps xmm15, xmm1
14000d430: subss xmm15, xmm13
14000d435: addps xmm12, xmm0
14000d439: addss xmm13, xmm1
14000d43e: jmp 0x14000d486 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d440: movsd xmm14, qword ptr [rbp + 0x228]
14000d449: movss xmm15, dword ptr [rbp + 0x230]
14000d452: addps xmm12, xmm14
14000d456: addss xmm13, xmm15
14000d45b: jmp 0x14000d486 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d45d: movsd xmm0, qword ptr [rbp + 0x218]
14000d465: movss xmm1, dword ptr [rbp + 0x220]
14000d46d: movaps xmm14, xmm0
14000d471: subps xmm14, xmm12
14000d475: movaps xmm15, xmm1
14000d479: subss xmm15, xmm13
14000d47e: movaps xmm13, xmm1
14000d482: movaps xmm12, xmm0
14000d486: lea rcx, [rbp + 0x460]
14000d48d: movq xmm11, qword ptr [rbp + 0x590]
14000d496: mov eax, esi
14000d498: mov rbx, rax
14000d49b: shl rbx, 0x20
14000d49f: or rbx, rax
14000d4a2: movlps qword ptr [rbp + 0x680], xmm14
14000d4aa: movss dword ptr [rbp + 0x688], xmm15
14000d4b3: movlps qword ptr [rbp + 0x68c], xmm12
14000d4bb: movss dword ptr [rbp + 0x694], xmm13
14000d4c4: movshdup xmm0, xmm14
14000d4c9: minss xmm0, dword ptr [rip + 0x2918483]
14000d4d1: movshdup xmm1, xmm12
14000d4d6: minss xmm1, xmm0
14000d4da: movss dword ptr [rbp + 0x548], xmm1
14000d4e2: lea rax, [rbp + 0x680]
14000d4e9: mov qword ptr [rbp + 0x6a0], rax
14000d4f0: lea rax, [rbp + 0x698]
14000d4f7: mov qword ptr [rbp + 0x6a8], rax
14000d4fe: lea rax, [rbp + 0x548]
14000d505: mov qword ptr [rbp + 0x6b0], rax
14000d50c: mov byte ptr [rbp + 0x817], 1
14000d513: lea rdx, [rbp + 0x6a0]
14000d51a: lea r8, [rip + 0x2919e0f]
14000d521: call 0x1402befa0 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$16<false> > >
14000d526: mov byte ptr [rbp + 0x817], 1
14000d52d: lea rcx, [rbp + 0x590]
14000d534: lea rdx, [rbp + 0x460]
14000d53b: xor r8d, r8d
14000d53e: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000d543: xor r14d, r14d
14000d546: cmp r14, qword ptr [rbp + 0x590]
14000d54d: jo 0x140009bd0
14000d553: mov rax, qword ptr [rbp + 0x5c0]
14000d55a: mov qword ptr [rbp + 0x6d0], rax
14000d561: movups xmm0, xmmword ptr [rbp + 0x590]
14000d568: movups xmm1, xmmword ptr [rbp + 0x5a0]
14000d56f: movups xmm2, xmmword ptr [rbp + 0x5b0]
14000d576: movaps xmmword ptr [rbp + 0x6c0], xmm2
14000d57d: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000d584: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000d58b: lea rcx, [rbp + 0x380]
14000d592: lea rdx, [rbp + 0x6a0]
14000d599: movd xmm2, dword ptr [rip + 0x29182cf]
14000d5a1: mov r9d, 1
14000d5a7: call 0x140152b00 ; utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000d5ac: mov rax, qword ptr [rbp + 0x6a0]
14000d5b3: test rax, rax
14000d5b6: je 0x14000d5d2
14000d5b8: mov rcx, qword ptr [rbp + 0x6a8]
14000d5bf: shl rax, 2
14000d5c3: lea rdx, [rax + rax*2]
14000d5c7: mov r8d, 4
14000d5cd: call 0x140613c20 ; ?
14000d5d2: mov rdx, qword ptr [rbp + 0x6b8]
14000d5d9: test rdx, rdx
14000d5dc: je 0x14000d5f4
14000d5de: mov rcx, qword ptr [rbp + 0x6c0]
14000d5e5: shl rdx, 2
14000d5e9: mov r8d, 4
14000d5ef: call 0x140613c20 ; ?
14000d5f4: cmp r14, qword ptr [rbp + 0x380]
14000d5fb: jo 0x140009bd0
14000d601: mov rax, qword ptr [rbp + 0x3b0]
14000d608: mov qword ptr [rbp + 0x490], rax
14000d60f: movupd xmm0, xmmword ptr [rbp + 0x380]
14000d617: movups xmm1, xmmword ptr [rbp + 0x390]
14000d61e: movups xmm2, xmmword ptr [rbp + 0x3a0]
14000d625: movaps xmmword ptr [rbp + 0x480], xmm2
14000d62c: movaps xmmword ptr [rbp + 0x470], xmm1
14000d633: movapd xmmword ptr [rbp + 0x460], xmm0
14000d63b: mov rdi, qword ptr [rbp + 0x470]
14000d642: test rdi, rdi
14000d645: je 0x14000d6fb
14000d64b: lea rsi, [rdi*8]
14000d653: movzx eax, byte ptr [rip + 0x36275af]
14000d65a: mov edx, 4
14000d65f: mov rcx, rsi
14000d662: call 0x140613c10 ; ?
14000d667: test rax, rax
14000d66a: je 0x140011696
14000d670: mov r8d, 1
14000d676: mov rcx, rax
14000d679: cmp rdi, 1
14000d67d: xorps xmm7, xmm7
14000d680: movss xmm8, dword ptr [rip + 0x291827b]
14000d689: xorps xmm9, xmm9
14000d68d: movaps xmm10, xmmword ptr [rip + 0x291829b]
14000d695: je 0x14000d6f4
14000d697: mov edx, 1
14000d69c: mov rcx, rax
14000d69f: cmp rdi, 4
14000d6a3: jbe 0x14000d6e0
14000d6a5: lea r9, [rdi - 1]
14000d6a9: mov r10, r9
14000d6ac: and r10, 0xfffffffffffffffc
14000d6b0: lea rcx, [rax + r10*8]
14000d6b4: lea rdx, [r10 + 1]
14000d6b8: movddup xmm0, xmm11
14000d6bd: xor r8d, r8d
14000d6c0: movupd xmmword ptr [rax + r8*8], xmm0
14000d6c6: movupd xmmword ptr [rax + r8*8 + 0x10], xmm0
14000d6cd: add r8, 4
14000d6d1: cmp r10, r8
14000d6d4: jne 0x14000d6c0
14000d6d6: mov r8, rdi
14000d6d9: cmp r9, r10
14000d6dc: je 0x14000d6f4
14000d6de: nop 
14000d6e0: inc rdx
14000d6e3: movlpd qword ptr [rcx], xmm11
14000d6e8: add rcx, 8
14000d6ec: cmp rdi, rdx
14000d6ef: jne 0x14000d6e0
14000d6f1: mov r8, rdi
14000d6f4: movlpd qword ptr [rcx], xmm11
14000d6f9: jmp 0x14000d71b ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d6fb: mov eax, 4
14000d700: xor r8d, r8d
14000d703: xorps xmm7, xmm7
14000d706: movss xmm8, dword ptr [rip + 0x29181f5]
14000d70f: xorps xmm9, xmm9
14000d713: movaps xmm10, xmmword ptr [rip + 0x2918215]
14000d71b: mov qword ptr [rbp + 0x500], r8
14000d722: mov qword ptr [rbp + 0x508], rax
14000d729: mov r15, qword ptr [rbp + 0x470]
14000d730: mov rax, qword ptr [rbp + 0x488]
14000d737: cmp rax, r15
14000d73a: cmovb r15, rax
14000d73e: mov qword ptr [rbp + 0x510], rdi
14000d745: lea rdi, [r15*4]
14000d74d: mov rax, r15
14000d750: shr rax, 0x3e
14000d754: jne 0x140011783
14000d75a: mov esi, 0
14000d75f: movabs rax, 0x7ffffffffffffffc
14000d769: cmp rdi, rax
14000d76c: ja 0x140011785
14000d772: mov r14, qword ptr [rbp + 0x480]
14000d779: test rdi, rdi
14000d77c: je 0x14000d7ad
14000d77e: movzx eax, byte ptr [rip + 0x3627484]
14000d785: mov esi, 4
14000d78a: mov edx, 4
14000d78f: mov rcx, rdi
14000d792: call 0x140613c10 ; ?
14000d797: test rax, rax
14000d79a: je 0x140011785
14000d7a0: mov rcx, r15
14000d7a3: test r15, r15
14000d7a6: jne 0x14000d7bd
14000d7a8: jmp 0x14000d8dd ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d7ad: mov eax, 4
14000d7b2: xor ecx, ecx
14000d7b4: test r15, r15
14000d7b7: je 0x14000d8dd
14000d7bd: mov rdx, rbx
14000d7c0: shr rdx, 0x20
14000d7c4: movd xmm1, ebx
14000d7c8: movd xmm0, edx
14000d7cc: punpckldq xmm0, xmm1
14000d7d0: cmp r15, 8
14000d7d4: jb 0x14000d859
14000d7da: mov r8, rax
14000d7dd: sub r8, r14
14000d7e0: mov edx, 0
14000d7e5: cmp r8, 0x20
14000d7e9: jb 0x14000d85b
14000d7eb: mov rdx, r15
14000d7ee: movabs r8, 0x3ffffffffffffff8
14000d7f8: and rdx, r8
14000d7fb: movdqa xmm1, xmm0
14000d7ff: shufps xmm1, xmm0, 0x55
14000d803: movaps xmm2, xmm0
14000d806: shufps xmm2, xmm0, 0
14000d80a: xor r8d, r8d
14000d80d: nop dword ptr [rax]
14000d810: movups xmm3, xmmword ptr [r14 + r8*4]
14000d815: movups xmm4, xmmword ptr [r14 + r8*4 + 0x10]
14000d81b: movaps xmm5, xmm10
14000d81f: subps xmm5, xmm3
14000d822: movaps xmm6, xmm10
14000d826: subps xmm6, xmm4
14000d829: mulps xmm5, xmm1
14000d82c: mulps xmm6, xmm1
14000d82f: mulps xmm3, xmm2
14000d832: addps xmm3, xmm5
14000d835: mulps xmm4, xmm2
14000d838: addps xmm4, xmm6
14000d83b: movups xmmword ptr [rax + r8*4], xmm3
14000d840: movups xmmword ptr [rax + r8*4 + 0x10], xmm4
14000d846: add r8, 8
14000d84a: cmp rdx, r8
14000d84d: jne 0x14000d810
14000d84f: cmp r15, rdx
14000d852: jne 0x14000d85b
14000d854: jmp 0x14000d8dd ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d859: xor edx, edx
14000d85b: mov r8, rdx
14000d85e: or r8, 1
14000d862: test r15b, 1
14000d866: je 0x14000d88b
14000d868: movss xmm1, dword ptr [r14 + rdx*4]
14000d86e: movaps xmm2, xmm8
14000d872: subss xmm2, xmm1
14000d876: unpcklps xmm1, xmm2
14000d879: mulps xmm1, xmm0
14000d87c: movshdup xmm2, xmm1
14000d880: addss xmm2, xmm1
14000d884: movss dword ptr [rax + rdx*4], xmm2
14000d889: jmp 0x14000d8d8 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d88b: cmp r15, r8
14000d88e: je 0x14000d8dd
14000d890: movss xmm1, dword ptr [r14 + rdx*4]
14000d896: movaps xmm2, xmm8
14000d89a: subss xmm2, xmm1
14000d89e: unpcklps xmm1, xmm2
14000d8a1: mulps xmm1, xmm0
14000d8a4: movshdup xmm2, xmm1
14000d8a8: addss xmm2, xmm1
14000d8ac: movss dword ptr [rax + rdx*4], xmm2
14000d8b1: movss xmm1, dword ptr [r14 + rdx*4 + 4]
14000d8b8: movaps xmm2, xmm8
14000d8bc: subss xmm2, xmm1
14000d8c0: unpcklps xmm1, xmm2
14000d8c3: mulps xmm1, xmm0
14000d8c6: movshdup xmm2, xmm1
14000d8ca: addss xmm2, xmm1
14000d8ce: movss dword ptr [rax + rdx*4 + 4], xmm2
14000d8d4: lea r8, [rdx + 2]
14000d8d8: mov rdx, r8
14000d8db: jmp 0x14000d88b ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d8dd: mov qword ptr [rbp + 0x3e0], rcx
14000d8e4: mov qword ptr [rbp + 0x3e8], rax
14000d8eb: mov r14, qword ptr [rbp + 0x470]
14000d8f2: mov rax, qword ptr [rbp + 0x488]
14000d8f9: cmp rax, r14
14000d8fc: cmovb r14, rax
14000d900: mov qword ptr [rbp + 0x3f0], r15
14000d907: lea rdi, [r14*4]
14000d90f: mov rax, r14
14000d912: shr rax, 0x3e
14000d916: jne 0x1400117ca
14000d91c: mov esi, 0
14000d921: movabs rax, 0x7ffffffffffffffc
14000d92b: cmp rdi, rax
14000d92e: ja 0x1400117cc
14000d934: mov rbx, qword ptr [rbp + 0x480]
14000d93b: test rdi, rdi
14000d93e: je 0x14000d973
14000d940: movzx eax, byte ptr [rip + 0x36272c2]
14000d947: mov esi, 4
14000d94c: mov edx, 4
14000d951: mov rcx, rdi
14000d954: call 0x140613c10 ; ?
14000d959: test rax, rax
14000d95c: je 0x1400117cc
14000d962: mov qword ptr [rbp + 0x780], r14
14000d969: test r14, r14
14000d96c: jne 0x14000d98c
14000d96e: jmp 0x14000da86 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000d973: mov eax, 4
14000d978: mov qword ptr [rbp + 0x780], 0
14000d983: test r14, r14
14000d986: je 0x14000da86
14000d98c: cmp r14, 8
14000d990: jb 0x14000da05
14000d992: mov rcx, rax
14000d995: sub rcx, rbx
14000d998: mov edx, 0
14000d99d: cmp rcx, 0x20
14000d9a1: jb 0x14000da07
14000d9a3: mov rdx, r14
14000d9a6: movabs rcx, 0x3ffffffffffffff8
14000d9b0: and rdx, rcx
14000d9b3: xor ecx, ecx
14000d9b5: nop word ptr cs:[rax + rax]
14000d9c0: movups xmm0, xmmword ptr [rbx + rcx*4]
14000d9c4: movups xmm1, xmmword ptr [rbx + rcx*4 + 0x10]
14000d9c9: movaps xmm2, xmm10
14000d9cd: subps xmm2, xmm0
14000d9d0: movaps xmm3, xmm10
14000d9d4: subps xmm3, xmm1
14000d9d7: mulps xmm2, xmm7
14000d9da: mulps xmm3, xmm7
14000d9dd: mulps xmm0, xmm7
14000d9e0: addps xmm0, xmm2
14000d9e3: mulps xmm1, xmm7
14000d9e6: addps xmm1, xmm3
14000d9e9: movups xmmword ptr [rax + rcx*4], xmm0
14000d9ed: movups xmmword ptr [rax + rcx*4 + 0x10], xmm1
14000d9f2: add rcx, 8
14000d9f6: cmp rdx, rcx
14000d9f9: jne 0x14000d9c0
14000d9fb: cmp r14, rdx
14000d9fe: jne 0x14000da07
14000da00: jmp 0x14000da86 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000da05: xor edx, edx
14000da07: mov rcx, rdx
14000da0a: or rcx, 1
14000da0e: test r14b, 1
14000da12: je 0x14000da36
14000da14: movss xmm0, dword ptr [rbx + rdx*4]
14000da19: movaps xmm1, xmm8
14000da1d: subss xmm1, xmm0
14000da21: mulss xmm1, xmm9
14000da26: mulss xmm0, xmm9
14000da2b: addss xmm0, xmm1
14000da2f: movss dword ptr [rax + rdx*4], xmm0
14000da34: jmp 0x14000da81 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000da36: cmp r14, rcx
14000da39: je 0x14000da86
14000da3b: movss xmm0, dword ptr [rbx + rdx*4]
14000da40: movaps xmm1, xmm8
14000da44: subss xmm1, xmm0
14000da48: mulss xmm1, xmm9
14000da4d: mulss xmm0, xmm9
14000da52: addss xmm0, xmm1
14000da56: movss dword ptr [rax + rdx*4], xmm0
14000da5b: movss xmm0, dword ptr [rbx + rdx*4 + 4]
14000da61: movaps xmm1, xmm8
14000da65: subss xmm1, xmm0
14000da69: mulss xmm1, xmm9
14000da6e: mulss xmm0, xmm9
14000da73: addss xmm0, xmm1
14000da77: movss dword ptr [rax + rdx*4 + 4], xmm0
14000da7d: lea rcx, [rdx + 2]
14000da81: mov rdx, rcx
14000da84: jmp 0x14000da36 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000da86: mov qword ptr [rbp + 0x800], rax
14000da8d: mov byte ptr [rbp + 0x812], 1
14000da94: mov byte ptr [rbp + 0x811], 1
14000da9b: mov rcx, qword ptr [rbp + 0x268]
14000daa2: call 0x140987ca0 ; country_core::render::impl$0::deref
14000daa7: mov rdi, rax
14000daaa: mov r12, qword ptr [rbp + 0x3e8]
14000dab1: mov r8, qword ptr [rbp + 0x3f0]
14000dab8: mov r13, qword ptr [rbp + 0x508]
14000dabf: mov r11, qword ptr [rbp + 0x510]
14000dac6: mov rsi, qword ptr [rbp + 0x468]
14000dacd: mov rbx, qword ptr [rbp + 0x470]
14000dad4: test r8, r8
14000dad7: je 0x14000db2f
14000dad9: mov qword ptr [rbp + 0x7f0], rbx
14000dae0: mov rbx, r11
14000dae3: mov r15, r8
14000dae6: lea rcx, [r8*4]
14000daee: movzx eax, byte ptr [rip + 0x3627114]
14000daf5: mov edx, 4
14000dafa: mov qword ptr [rbp + 0x670], rcx
14000db01: call 0x140613c40 ; ?
14000db06: test rax, rax
14000db09: je 0x1400116cf
14000db0f: mov r10, rax
14000db12: mov ecx, dword ptr [rbp + 0x580]
14000db18: movss xmm1, dword ptr [rbp + 0x568]
14000db20: mov r8, r15
14000db23: mov r11, rbx
14000db26: mov rbx, qword ptr [rbp + 0x7f0]
14000db2d: jmp 0x14000db43 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000db2f: mov r10d, 4
14000db35: mov ecx, dword ptr [rbp + 0x580]
14000db3b: movss xmm1, dword ptr [rbp + 0x568]
14000db43: mov qword ptr [rbp + 0x760], r8
14000db4a: mov eax, dword ptr [rbp + 0x588]
14000db50: mov dword ptr [rbp + 0x6a0], eax
14000db56: movss xmm0, dword ptr [rbp + 0x560]
14000db5e: movss dword ptr [rbp + 0x6a4], xmm0
14000db66: mov dword ptr [rbp + 0x6a8], ecx
14000db6c: movss dword ptr [rbp + 0x6ac], xmm1
14000db74: mov qword ptr [rsp + 0x60], r8
14000db79: mov qword ptr [rbp + 0x7f0], r10
14000db80: mov qword ptr [rsp + 0x58], r10
14000db85: lea rax, [rbp + 0x6a0]
14000db8c: mov qword ptr [rsp + 0x50], rax
14000db91: mov rax, qword ptr [rbp + 0x758]
14000db98: mov dword ptr [rsp + 0x48], eax
14000db9c: mov rax, qword ptr [rbp + 0x940]
14000dba3: mov qword ptr [rsp + 0x40], rax
14000dba8: lea rax, [rbp + 0x140]
14000dbaf: mov qword ptr [rsp + 0x38], rax
14000dbb4: mov qword ptr [rsp + 0x30], rbx
14000dbb9: mov qword ptr [rsp + 0x28], rsi
14000dbbe: mov qword ptr [rsp + 0x20], r11
14000dbc3: mov rcx, rdi
14000dbc6: mov rdx, r12
14000dbc9: mov r9, r13
14000dbcc: call 0x1407dc170 ; system_decorator::decorator_visual::stairs_assemble::gen_stair_floors
14000dbd1: mov rdx, qword ptr [rbp + 0x760]
14000dbd8: test rdx, rdx
14000dbdb: lea r15, [rbp + 0x460]
14000dbe2: mov r12, qword ptr [rbp + 0x678]
14000dbe9: je 0x14000dc01
14000dbeb: shl rdx, 2
14000dbef: mov r8d, 4
14000dbf5: mov rcx, qword ptr [rbp + 0x7f0]
14000dbfc: call 0x140613c20 ; ?
14000dc01: mov rax, qword ptr [rbp + 0x938]
14000dc08: mov rsi, qword ptr [rax]
14000dc0b: mov bl, byte ptr [rbp + 0x80f]
14000dc11: mov dil, byte ptr [rbp + 0x810]
14000dc18: mov byte ptr [rbp + 0x812], 1
14000dc1f: mov byte ptr [rbp + 0x811], 1
14000dc26: lea rcx, [rbp + 0x2a0]
14000dc2d: call 0x1400284b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$8<true>
14000dc32: mov dword ptr [rbp + 0x6a0], 0
14000dc3c: mov dword ptr [rbp + 0x6a8], 0
14000dc46: mov byte ptr [rbp + 0x812], 1
14000dc4d: mov byte ptr [rbp + 0x811], 1
14000dc54: mov qword ptr [rsp + 0x80], r14
14000dc5c: mov rcx, qword ptr [rbp + 0x800]
14000dc63: mov qword ptr [rsp + 0x78], rcx
14000dc68: lea rcx, [rbp + 0x6a0]
14000dc6f: mov qword ptr [rsp + 0x70], rcx
14000dc74: and dl, 1
14000dc77: mov byte ptr [rsp + 0x68], dl
14000dc7b: and al, 1
14000dc7d: mov byte ptr [rsp + 0x60], al
14000dc81: mov byte ptr [rsp + 0x58], dil
14000dc86: mov byte ptr [rsp + 0x50], bl
14000dc8a: mov eax, dword ptr [rbp + 0x7b8]
14000dc90: mov byte ptr [rsp + 0x48], al
14000dc94: mov qword ptr [rsp + 0x38], rsi
14000dc99: mov eax, dword ptr [rbp + 0x528]
14000dc9f: mov dword ptr [rsp + 0x30], eax
14000dca3: mov rax, qword ptr [rbp + 0x758]
14000dcaa: mov dword ptr [rsp + 0x28], eax
14000dcae: lea rax, [rbp + 0x428]
14000dcb5: mov qword ptr [rsp + 0x20], rax
14000dcba: mov word ptr [rsp + 0x40], 0x101
14000dcc1: lea rcx, [rbp + 0x140]
14000dcc8: lea rdx, [rbp + 0x3e0]
14000dccf: lea r8, [rbp + 0x500]
14000dcd6: mov r9, r15
14000dcd9: call 0x1407f98e0 ; system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings
14000dcde: mov rax, qword ptr [rbp + 0x490]
14000dce5: mov qword ptr [rbp + 0x6d0], rax
14000dcec: movaps xmm0, xmmword ptr [rbp + 0x460]
14000dcf3: movaps xmm1, xmmword ptr [rbp + 0x470]
14000dcfa: movaps xmm2, xmmword ptr [rbp + 0x480]
14000dd01: movaps xmmword ptr [rbp + 0x6c0], xmm2
14000dd08: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000dd0f: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000dd16: mov rdi, qword ptr [rbp + 0x3f0]
14000dd1d: test rdi, rdi
14000dd20: je 0x14000dd4e
14000dd22: lea rsi, [rdi*4]
14000dd2a: movzx eax, byte ptr [rip + 0x3626ed8]
14000dd31: mov edx, 4
14000dd36: mov rcx, rsi
14000dd39: call 0x140613c40 ; ?
14000dd3e: test rax, rax
14000dd41: mov r14d, 0
14000dd47: jne 0x14000dd56
14000dd49: jmp 0x1400116b6 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000dd4e: mov eax, 4
14000dd53: xor r14d, r14d
14000dd56: mov rcx, qword ptr [rbp + 0x3f0]
14000dd5d: lea rdx, [rbp + 0x598]
14000dd64: mov qword ptr [rdx + 0x58], rcx
14000dd68: movups xmm0, xmmword ptr [rbp + 0x3e0]
14000dd6f: movups xmmword ptr [rdx + 0x48], xmm0
14000dd73: mov rcx, qword ptr [rbp + 0x510]
14000dd7a: mov qword ptr [rdx + 0x40], rcx
14000dd7e: movups xmm0, xmmword ptr [rbp + 0x500]
14000dd85: movups xmmword ptr [rdx + 0x30], xmm0
14000dd89: mov rcx, qword ptr [rbp + 0x6d0]
14000dd90: mov qword ptr [rbp + 0x5c0], rcx
14000dd97: movaps xmm0, xmmword ptr [rbp + 0x6a0]
14000dd9e: movaps xmm1, xmmword ptr [rbp + 0x6b0]
14000dda5: movaps xmm2, xmmword ptr [rbp + 0x6c0]
14000ddac: movaps xmmword ptr [rbp + 0x590], xmm0
14000ddb3: movaps xmmword ptr [rbp + 0x5a0], xmm1
14000ddba: movaps xmmword ptr [rbp + 0x5b0], xmm2
14000ddc1: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000ddc8: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000ddcf: movaps xmmword ptr [rbp + 0x6c0], xmm2
14000ddd6: movaps xmm0, xmmword ptr [rbp + 0x5d0]
14000dddd: movaps xmm1, xmmword ptr [rbp + 0x5e0]
14000dde4: movaps xmmword ptr [rbp + 0x6e0], xmm0
14000ddeb: movaps xmmword ptr [rbp + 0x6f0], xmm1
14000ddf2: mov rcx, qword ptr [rbp + 0x5f0]
14000ddf9: mov qword ptr [rbp + 0x700], rcx
14000de00: mov rcx, qword ptr [rbp + 0x5c0]
14000de07: mov qword ptr [rbp + 0x6d0], rcx
14000de0e: mov rcx, qword ptr [rbp + 0x5c8]
14000de15: mov qword ptr [rbp + 0x6d8], rcx
14000de1c: mov qword ptr [rbp + 0x708], rdi
14000de23: mov qword ptr [rbp + 0x710], rax
14000de2a: mov qword ptr [rbp + 0x718], rdi
14000de31: mov rax, qword ptr [rbp + 0x438]
14000de38: mov qword ptr [rbp + 0x720], rax
14000de3f: mov rax, qword ptr [r12]
14000de43: mov qword ptr [rbp + 0x728], rax
14000de4a: mov rax, qword ptr [rbp + 0x758]
14000de51: mov dword ptr [rbp + 0x730], eax
14000de57: mov dword ptr [rbp + 0x734], 0x101
14000de61: movzx eax, byte ptr [rbp + 0x7a8]
14000de68: mov byte ptr [rbp + 0x738], al
14000de6e: movzx eax, byte ptr [rbp + 0x7a0]
14000de75: mov byte ptr [rbp + 0x739], al
14000de7b: mov word ptr [rbp + 0x73a], 0x300
14000de84: mov byte ptr [rbp + 0x750], 2
14000de8b: mov rax, qword ptr [rbp + 0x440]
14000de92: mov rsi, qword ptr [rbp + 0x458]
14000de99: cmp rsi, rax
14000de9c: jne 0x14000debf
14000de9e: lea rcx, [rbp + 0x440]
14000dea5: lea rdx, [rip + 0x29189dc]
14000deac: call 0x1405114a0 ; alloc::collections::vec_deque::VecDeque::grow<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global>
14000deb1: mov rax, qword ptr [rbp + 0x440]
14000deb8: mov rsi, qword ptr [rbp + 0x458]
14000debf: mov rcx, qword ptr [rbp + 0x450]
14000dec6: add rcx, rsi
14000dec9: cmp rcx, rax
14000decc: mov edx, 0
14000ded1: cmovae rdx, rax
14000ded5: sub rcx, rdx
14000ded8: imul rcx, rcx, 0xb8
14000dedf: add rcx, qword ptr [rbp + 0x448]
14000dee6: mov r8d, 0xb8
14000deec: lea rdx, [rbp + 0x6a0]
14000def3: call 0x142923c10 ; ?
14000def8: inc rsi
14000defb: mov qword ptr [rbp + 0x458], rsi
14000df02: mov rdx, qword ptr [rbp + 0x780]
14000df09: test rdx, rdx
14000df0c: je 0x140009bd0
14000df12: shl rdx, 2
14000df16: mov r8d, 4
14000df1c: mov rcx, qword ptr [rbp + 0x800]
14000df23: call 0x140613c20 ; ?
14000df28: jmp 0x140009bd0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000df2d: mov eax, dword ptr [rbp + 0x32c]
14000df33: mov rcx, qword ptr [rbp + 0x110]
14000df3a: mov dword ptr [rcx], eax
14000df3c: mov rdx, qword ptr [rbp + 0x378]
14000df43: mov eax, dword ptr [rdx + 0x1c]
14000df46: mov rcx, qword ptr [rdx + 0x10]
14000df4a: mov dword ptr [rcx], eax
14000df4c: mov rdx, qword ptr [rdx]
14000df4f: mov r9d, dword ptr [r13 + 0x128]
14000df56: mov eax, dword ptr [r13 + 0x12c]
14000df5d: mov byte ptr [rbp + 0x817], 1
14000df64: mov dword ptr [rsp + 0x20], eax
14000df68: lea rcx, [rbp + 0x6a0]
14000df6f: mov r8, qword ptr [rbp + 0xa8]
14000df76: call 0x140252ad0 ; country_core::systems::collision::world::RaycastWorld::remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>
14000df7b: cmp byte ptr [rbp + 0x6c0], 0x11
14000df82: je 0x14000dfa4
14000df84: mov rax, qword ptr [rbp + 0x6d8]
14000df8b: lock dec qword ptr [rax]
14000df8f: jne 0x14000dfa4
14000df91: mov byte ptr [rbp + 0x817], 1
14000df98: lea rcx, [rbp + 0x6d8]
14000df9f: call 0x140b5c620 ; alloc::sync::Arc::drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
14000dfa4: mov rdx, qword ptr [rbp + 0x378]
14000dfab: mov eax, dword ptr [rdx + 0x1c]
14000dfae: mov rcx, qword ptr [rdx + 0x10]
14000dfb2: mov dword ptr [rcx], eax
14000dfb4: mov rdx, qword ptr [rdx]
14000dfb7: mov rax, qword ptr [rbp + 0x7e0]
14000dfbe: mov r9d, dword ptr [rax + 0x128]
14000dfc5: mov eax, dword ptr [rax + 0x12c]
14000dfcb: mov byte ptr [rbp + 0x817], 1
14000dfd2: mov dword ptr [rsp + 0x20], eax
14000dfd6: lea rcx, [rbp + 0x6a0]
14000dfdd: mov r8, qword ptr [rbp + 0xb0]
14000dfe4: call 0x140252ad0 ; country_core::systems::collision::world::RaycastWorld::remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>
14000dfe9: cmp byte ptr [rbp + 0x6c0], 0x11
14000dff0: je 0x14000e012
14000dff2: mov rax, qword ptr [rbp + 0x6d8]
14000dff9: lock dec qword ptr [rax]
14000dffd: jne 0x14000e012
14000dfff: mov byte ptr [rbp + 0x817], 1
14000e006: lea rcx, [rbp + 0x6d8]
14000e00d: call 0x140b5c620 ; alloc::sync::Arc::drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
14000e012: cmp qword ptr [rbp + 0x288], 0
14000e01a: je 0x14000e120
14000e020: mov rax, qword ptr [rbp + 0x288]
14000e027: mov qword ptr [rbp + 0x470], rax
14000e02e: movups xmm0, xmmword ptr [rbp + 0x278]
14000e035: movaps xmmword ptr [rbp + 0x460], xmm0
14000e03c: movaps xmm0, xmmword ptr [rip + 0x291858d]
14000e043: movaps xmmword ptr [rbp + 0x5a0], xmm0
14000e04a: movaps xmm0, xmmword ptr [rip + 0x291856f]
14000e051: movaps xmmword ptr [rbp + 0x590], xmm0
14000e058: mov byte ptr [rbp + 0x817], 0
14000e05f: lea rcx, [rbp + 0x6b0]
14000e066: lea rdx, [rbp + 0x460]
14000e06d: lea r8, [rbp + 0x590]
14000e074: call 0x1408d0230 ; country_core::systems::collision::world::RaycastMeshData::new_compound
14000e079: mov qword ptr [rbp + 0x6a0], 1
14000e084: mov qword ptr [rbp + 0x6a8], 1
14000e08f: movzx eax, byte ptr [rip + 0x3626b73]
14000e096: mov ecx, 0x90
14000e09b: mov edx, 0x10
14000e0a0: call 0x140613c10 ; ?
14000e0a5: test rax, rax
14000e0a8: je 0x140011598
14000e0ae: mov rsi, rax
14000e0b1: mov r8d, 0x90
14000e0b7: mov rcx, rax
14000e0ba: lea rdi, [rbp + 0x6a0]
14000e0c1: mov rdx, rdi
14000e0c4: call 0x142923c00 ; ?
14000e0c9: mov rax, qword ptr [rbp + 0x678]
14000e0d0: mov rax, qword ptr [rax]
14000e0d3: mov qword ptr [rbp + 0x6a4], rax
14000e0da: mov byte ptr [rbp + 0x6a0], 0xd
14000e0e1: mov byte ptr [rbp + 0x817], 0
14000e0e8: lea rcx, [rbp + 0x500]
14000e0ef: mov rdx, rsi
14000e0f2: mov r8, rdi
14000e0f5: call 0x1408ceb10 ; country_core::systems::collision::world::RaycastMesh::new
14000e0fa: xorps xmm8, xmm8
14000e0fe: xor esi, esi
14000e100: mov rdi, qword ptr [rbp + 0x7e0]
14000e107: cmp byte ptr [rbp + 0x500], 0x11
14000e10e: lea rbx, [rbp + 0x500]
14000e115: cmove rbx, rsi
14000e119: jne 0x14000e175
14000e11b: jmp 0x14000eb95 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000e120: mov rdx, qword ptr [rbp + 0x278]
14000e127: mov rcx, qword ptr [rbp + 0x280]
14000e12e: mov byte ptr [rbp + 0x500], 0x11
14000e135: test rdx, rdx
14000e138: xorps xmm8, xmm8
14000e13c: mov esi, 0
14000e141: mov rdi, qword ptr [rbp + 0x7e0]
14000e148: je 0x14000eb95
14000e14e: shl rdx, 7
14000e152: mov r8d, 0x10
14000e158: call 0x140613c20 ; ?
14000e15d: cmp byte ptr [rbp + 0x500], 0x11
14000e164: lea rbx, [rbp + 0x500]
14000e16b: cmove rbx, rsi
14000e16f: je 0x14000eb95
14000e175: mov byte ptr [rbp + 0x7ff], 1
14000e17c: xor ecx, ecx
14000e17e: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14000e183: test rax, rax
14000e186: lea rsi, [rbp + 0x460]
14000e18d: je 0x1400115ce
14000e193: movups xmm0, xmmword ptr [rax]
14000e196: inc qword ptr [rax]
14000e199: movaps xmmword ptr [rbp + 0x480], xmm0
14000e1a0: movups xmm0, xmmword ptr [rip + 0x29a1e79]
14000e1a7: movaps xmmword ptr [rbp + 0x460], xmm0
14000e1ae: movups xmm0, xmmword ptr [rip + 0x29a1e7b]
14000e1b5: movaps xmmword ptr [rbp + 0x470], xmm0
14000e1bc: mov rax, qword ptr [rbp + 0x378]
14000e1c3: mov rdi, qword ptr [rax]
14000e1c6: movsd xmm0, qword ptr [rip + 0x2917792]
14000e1ce: movsd qword ptr [rbp + 0x6a0], xmm0
14000e1d6: mov dword ptr [rbp + 0x6a8], 0
14000e1e0: mov byte ptr [rbp + 0x7e9], 1
14000e1e7: lea rcx, [rbp + 0x590]
14000e1ee: lea rdx, [rbp + 0x6a0]
14000e1f5: call 0x140c91b30 ; utils::IsoTransform::from_translation
14000e1fa: mov byte ptr [rbp + 0x7e9], 1
14000e201: lea rcx, [rbp + 0x500]
14000e208: call 0x1408ceb30 ; country_core::systems::collision::world::RaycastMesh::data
14000e20d: movaps xmm0, xmmword ptr [rbp + 0x590]
14000e214: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14000e21b: movaps xmmword ptr [rbp + 0x6b0], xmm1
14000e222: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000e229: mov qword ptr [rbp + 0x6c0], rax
14000e230: mov byte ptr [rbp + 0x7e9], 1
14000e237: mov qword ptr [rsp + 0x20], rsi
14000e23c: mov rcx, rdi
14000e23f: lea rdx, [rbp + 0x6a0]
14000e246: mov r8d, 2
14000e24c: xor r9d, r9d
14000e24f: call 0x140254890 ; country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps<tuple$<utils::IsoTransform,ref$<country_core::systems::collision::world::RaycastMeshData> >,bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >),system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$21<true> >
14000e254: mov r14, qword ptr [rbp + 0x460]
14000e25b: mov rcx, qword ptr [rbp + 0x468]
14000e262: test rcx, rcx
14000e265: mov qword ptr [rbp + 0x668], rcx
14000e26c: je 0x14000e2a3
14000e26e: lea rax, [rcx*8 + 0x17]
14000e276: and rax, 0xfffffffffffffff0
14000e27a: add rcx, rax
14000e27d: add rcx, 0x11
14000e281: mov qword ptr [rbp + 0x7a0], rcx
14000e288: mov rcx, r14
14000e28b: sub rcx, rax
14000e28e: mov qword ptr [rbp + 0x670], rcx
14000e295: mov eax, 0x10
14000e29a: mov qword ptr [rbp + 0x7a8], rax
14000e2a1: jmp 0x14000e2ae ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000e2a3: mov qword ptr [rbp + 0x7a8], 0
14000e2ae: mov rax, qword ptr [rbp + 0x478]
14000e2b5: mov qword ptr [rbp + 0x520], rax
14000e2bc: movdqa xmm0, xmmword ptr [r14]
14000e2c1: lea r13, [r14 + 0x10]
14000e2c5: pmovmskb eax, xmm0
14000e2c9: not eax
14000e2cb: jmp 0x14000e2d6 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000e2cd: nop dword ptr [rax]
14000e2d0: mov eax, dword ptr [rbp + 0x578]
14000e2d6: cmp qword ptr [rbp + 0x520], 0
14000e2de: je 0x14000eb65
14000e2e4: test ax, ax
14000e2e7: je 0x14000e300
14000e2e9: lea ecx, [rax - 1]
14000e2ec: and ecx, eax
14000e2ee: mov dword ptr [rbp + 0x578], ecx
14000e2f4: jmp 0x14000e32c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000e2f6: nop word ptr cs:[rax + rax]
14000e300: movdqa xmm0, xmmword ptr [r13]
14000e306: pmovmskb eax, xmm0
14000e30a: add r14, -0x80
14000e30e: add r13, 0x10
14000e312: cmp eax, 0xffff
14000e317: je 0x14000e300
14000e319: mov ecx, 0xfffffffe
14000e31e: sub ecx, eax
14000e320: not eax
14000e322: mov edx, eax
14000e324: and edx, ecx
14000e326: mov dword ptr [rbp + 0x578], edx
14000e32c: tzcnt eax, eax
14000e330: shl eax, 3
14000e333: mov rcx, r14
14000e336: sub rcx, rax
14000e339: mov rdx, qword ptr [rcx - 8]
14000e33d: mov rcx, qword ptr [rbp + 0xe8]
14000e344: mov qword ptr [rbp + 0x648], rdx
14000e34b: lea r8, [rip + 0x291823e]
14000e352: call 0x140a91ef0 ; country_core::resources::walls::public_walls::PublicWalls::get
14000e357: mov r15, rax
14000e35a: dec qword ptr [rbp + 0x520]
14000e361: mov eax, dword ptr [rbp + 0x578]
14000e367: test r15, r15
14000e36a: je 0x14000e2d6
14000e370: mov rcx, qword ptr [rbp + 0xe8]
14000e377: mov rdx, qword ptr [rbp + 0x648]
14000e37e: call 0x140a922b0 ; country_core::resources::walls::public_walls::PublicWalls::roof_entity
14000e383: mov rcx, rax
14000e386: shr rcx, 0x20
14000e38a: je 0x14000e480
14000e390: mov eax, eax
14000e392: mov rdx, qword ptr [rbp + 0x250]
14000e399: cmp rax, qword ptr [rdx + 0x10]
14000e39d: jae 0x14000e480
14000e3a3: mov rdx, qword ptr [rbp + 0x250]
14000e3aa: mov rdx, qword ptr [rdx + 8]
14000e3ae: lea rax, [rax + rax*4]
14000e3b2: cmp dword ptr [rdx + rax*4], ecx
14000e3b5: jne 0x14000e480
14000e3bb: lea rax, [rdx + rax*4]
14000e3bf: mov ecx, dword ptr [rax + 4]
14000e3c2: mov edx, 0xffffffff
14000e3c7: cmp rcx, rdx
14000e3ca: je 0x14000e480
14000e3d0: mov rdx, qword ptr [rbp + 0x260]
14000e3d7: cmp qword ptr [rdx + 0x38], rcx
14000e3db: jbe 0x14000e480
14000e3e1: mov rdx, qword ptr [rbp + 0x260]
14000e3e8: mov rdx, qword ptr [rdx + 0x28]
14000e3ec: mov r8d, ecx
14000e3ef: shr r8d, 6
14000e3f3: mov rdx, qword ptr [rdx + r8*8]
14000e3f7: bt rdx, rcx
14000e3fb: jae 0x14000e480
14000e401: mov ecx, dword ptr [rax + 0xc]
14000e404: mov rdx, qword ptr [rbp + 0x250]
14000e40b: mov rdx, qword ptr [rdx + 0x1a8]
14000e412: lea r8, [rcx + rcx*8]
14000e416: mov rcx, qword ptr [rbp + 0x260]
14000e41d: mov rcx, qword ptr [rcx + 0x110]
14000e424: cmp qword ptr [rdx + r8*8 + 0x40], rcx
14000e429: jbe 0x14000e452
14000e42b: lea rdx, [rdx + r8*8]
14000e42f: mov r8, qword ptr [rdx + 0x38]
14000e433: mov rcx, qword ptr [r8 + rcx*8]
14000e437: test rcx, rcx
14000e43a: je 0x14000e452
14000e43c: mov rdx, qword ptr [rdx + 0x18]
14000e440: not rcx
14000e443: lea rcx, [rcx + rcx*2]
14000e447: shl rcx, 4
14000e44b: mov rcx, qword ptr [rdx + rcx + 0x10]
14000e450: jmp 0x14000e454 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000e452: xor ecx, ecx
14000e454: mov eax, dword ptr [rax + 0x10]
14000e457: imul rax, rax, 0x58
14000e45b: mov edx, dword ptr [rcx + rax + 0x40]
14000e45f: xor esi, esi
14000e461: cmp dl, 3
14000e464: cmove edx, esi
14000e467: cmp dl, 2
14000e46a: je 0x14000eb59
14000e470: movzx eax, dl
14000e473: cmp eax, 1
14000e476: je 0x14000e48e
14000e478: nop dword ptr [rax + rax]
14000e480: lea rcx, [r15 + 0x299]
14000e487: call 0x1408c5670 ; country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered
14000e48c: mov esi, eax
14000e48e: mov rcx, r15
14000e491: call 0x140b2a600 ; country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape
14000e496: mov r12, rax
14000e499: mov eax, dword ptr [rbp + 0x578]
14000e49f: test r12, r12
14000e4a2: je 0x14000e2d6
14000e4a8: test byte ptr [r12], 1
14000e4ad: je 0x14000e539
14000e4b3: mov rcx, r15
14000e4b6: call 0x140b2a6e0 ; country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y
14000e4bb: lea rdx, [r12 + 4]
14000e4c0: addss xmm0, dword ptr [rip + 0x2917438]
14000e4c8: lea rcx, [rbp + 0x6a0]
14000e4cf: movaps xmm2, xmm0
14000e4d2: call 0x140c98b90 ; utils::geometry::rectangle::Rectangle2d::as_points3
14000e4d7: movzx eax, byte ptr [rip + 0x362672b]
14000e4de: mov ecx, 0x30
14000e4e3: mov edx, 4
14000e4e8: call 0x140613c10 ; ?
14000e4ed: test rax, rax
14000e4f0: je 0x140011475
14000e4f6: movups xmm0, xmmword ptr [rbp + 0x6a0]
14000e4fd: movups xmm1, xmmword ptr [rbp + 0x6b0]
14000e504: movups xmm2, xmmword ptr [rbp + 0x6c0]
14000e50b: movups xmmword ptr [rax + 0x20], xmm2
14000e50f: movups xmmword ptr [rax + 0x10], xmm1
14000e513: mov qword ptr [rbp + 0x780], rax
14000e51a: movups xmmword ptr [rax], xmm0
14000e51d: lea rdi, [r12 + 0xc]
14000e522: add r12, 0x10
14000e526: mov eax, 4
14000e52b: mov qword ptr [rbp + 0x800], rax
14000e532: mov ecx, 4
14000e537: jmp 0x14000e59d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000e539: mov rcx, r15
14000e53c: call 0x140b2a6e0 ; country_core::resources::walls::public_wall_state::PublicWallState::flat_roof_y
14000e541: lea rdi, [r12 + 4]
14000e546: addss xmm0, dword ptr [rip + 0x29173b2]
14000e54e: lea rcx, [rbp + 0x6a0]
14000e555: mov rdx, rdi
14000e558: movss xmm2, dword ptr [rip + 0x2917410]
14000e560: movaps xmm3, xmm0
14000e563: call 0x140c97c50 ; utils::geometry::circle::Circle2d::as_points3
14000e568: lea rcx, [rbp + 0x50]
14000e56c: lea rdx, [rbp + 0x6a0]
14000e573: lea r8, [rip + 0x29cfb0e]
14000e57a: call 0x1402bd6b0 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::ops::range::Range<usize>,utils::geometry::circle::impl$0::as_points2_ptcount::closure_env$0>,utils::geometry::circle::impl$0::as_points3::closure_env$0> >
14000e57f: add r12, 8
14000e583: mov rax, qword ptr [rbp + 0x50]
14000e587: mov qword ptr [rbp + 0x800], rax
14000e58e: mov rax, qword ptr [rbp + 0x58]
14000e592: mov qword ptr [rbp + 0x780], rax
14000e599: mov rcx, qword ptr [rbp + 0x60]
14000e59d: mov dword ptr [rbp + 0x760], esi
14000e5a3: movss xmm0, dword ptr [r12]
14000e5a9: movss xmm1, dword ptr [rdi]
14000e5ad: movss dword ptr [rbp + 0x680], xmm1
14000e5b5: movss dword ptr [rbp + 0x684], xmm0
14000e5bd: lea rax, [rcx*4]
14000e5c5: lea rdi, [rax + rax*2]
14000e5c9: mov qword ptr [rbp + 0x7f0], rcx
14000e5d0: test rcx, rcx
14000e5d3: je 0x14000e5f7
14000e5d5: movzx eax, byte ptr [rip + 0x362662d]
14000e5dc: mov edx, 4
14000e5e1: mov rcx, rdi
14000e5e4: call 0x140613c10 ; ?
14000e5e9: test rax, rax
14000e5ec: je 0x140011443
14000e5f2: mov rsi, rax
14000e5f5: jmp 0x14000e5fc ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000e5f7: mov esi, 4
14000e5fc: mov r12, qword ptr [rbp + 0x780]
14000e603: lea rax, [r12 + rdi]
14000e607: mov qword ptr [rbp + 0x7b0], rax
14000e60e: mov rcx, rsi
14000e611: mov rdx, r12
14000e614: mov r8, rdi
14000e617: call 0x142923c00 ; ?
14000e61c: add rdi, rsi
14000e61f: mov rax, qword ptr [rbp + 0x7f0]
14000e626: mov qword ptr [rbp + 0x6a0], rax
14000e62d: mov qword ptr [rbp + 0x6a8], rsi
14000e634: mov qword ptr [rbp + 0x6b0], rsi
14000e63b: mov qword ptr [rbp + 0x6b8], rax
14000e642: mov qword ptr [rbp + 0x6c0], rdi
14000e649: mov qword ptr [rbp + 0x6c8], r12
14000e650: mov qword ptr [rbp + 0x6d0], r12
14000e657: mov rax, qword ptr [rbp + 0x800]
14000e65e: mov qword ptr [rbp + 0x6d8], rax
14000e665: mov rax, qword ptr [rbp + 0x7b0]
14000e66c: mov qword ptr [rbp + 0x6e0], rax
14000e673: mov dword ptr [rbp + 0x6e8], 0
14000e67d: movss xmm0, dword ptr [rip + 0x29172ef]
14000e685: movss dword ptr [rbp + 0x570], xmm0
14000e68d: test byte ptr [rbp + 0x760], 1
14000e694: jne 0x14000e6a6
14000e696: movss xmm0, dword ptr [rip + 0x29172da]
14000e69e: movss dword ptr [rbp + 0x570], xmm0
14000e6a6: xorps xmm0, xmm0
14000e6a9: jmp 0x14000e704 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000e6ab: nop dword ptr [rax + rax]
14000e6b0: movss xmm0, dword ptr [rbp + 0x778]
14000e6b8: subss xmm0, xmm7
14000e6bc: mov rax, qword ptr [r12 + 8]
14000e6c1: lea rcx, [rsi + rsi*2]
14000e6c5: mov rdx, qword ptr [rbp + 0x648]
14000e6cc: mov qword ptr [rax + rcx*8], rdx
14000e6d0: movss dword ptr [rax + rcx*8 + 8], xmm12
14000e6d7: movss dword ptr [rax + rcx*8 + 0xc], xmm11
14000e6de: movss dword ptr [rax + rcx*8 + 0x10], xmm0
14000e6e4: mov dword ptr [rax + rcx*8 + 0x14], 0x41200000
14000e6ec: inc rsi
14000e6ef: mov qword ptr [r12 + 0x10], rsi
14000e6f4: movss xmm0, dword ptr [rbp + 0x7f0]
14000e6fc: addss xmm0, dword ptr [rbp + 0x7b0]
14000e704: movss dword ptr [rbp + 0x7f0], xmm0
14000e70c: lea rcx, [rbp + 0x380]
14000e713: lea rdx, [rbp + 0x6a0]
14000e71a: call 0x1404c2650 ; itertools::tuple_impl::impl$10::next<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> >
14000e71f: test byte ptr [rbp + 0x380], 1
14000e726: je 0x14000eb00
14000e72c: movss xmm0, dword ptr [rbp + 0x38c]
14000e734: movss dword ptr [rbp + 0x800], xmm0
14000e73c: movsd xmm0, qword ptr [rbp + 0x384]
14000e744: movaps xmmword ptr [rbp + 0x780], xmm0
14000e74b: movsd xmm0, qword ptr [rbp + 0x390]
14000e753: movaps xmmword ptr [rbp + 0x760], xmm0
14000e75a: movss xmm8, dword ptr [rbp + 0x398]
14000e763: lea rcx, [rbp + 0x590]
14000e76a: lea rdx, [rbp + 0x680]
14000e771: call 0x140c9cdc0 ; utils::swizzlers::impl$0::x0y
14000e776: movss xmm11, dword ptr [rbp + 0x800]
14000e77f: addss xmm11, xmm8
14000e784: mulss xmm11, dword ptr [rip + 0x29170e3]
14000e78d: subss xmm11, dword ptr [rbp + 0x598]
14000e796: movaps xmm1, xmm11
14000e79a: mulss xmm1, xmm11
14000e79f: movaps xmm0, xmmword ptr [rbp + 0x780]
14000e7a6: addps xmm0, xmmword ptr [rbp + 0x760]
14000e7ad: mulps xmm0, xmmword ptr [rip + 0x291712c]
14000e7b4: movsd xmm2, qword ptr [rbp + 0x590]
14000e7bc: subps xmm0, xmm2
14000e7bf: movaps xmm2, xmm0
14000e7c2: mulps xmm2, xmm0
14000e7c5: movshdup xmm3, xmm2
14000e7c9: addss xmm3, xmm2
14000e7cd: addss xmm3, xmm1
14000e7d1: xorps xmm2, xmm2
14000e7d4: sqrtss xmm2, xmm3
14000e7d8: movss xmm1, dword ptr [rip + 0x2917124]
14000e7e0: divss xmm1, xmm2
14000e7e4: movd eax, xmm1
14000e7e8: test eax, eax
14000e7ea: setns cl
14000e7ed: mov edx, eax
14000e7ef: and edx, 0x7fffffff
14000e7f5: add edx, 0xff800000
14000e7fb: cmp edx, 0x7f000000
14000e801: setb dl
14000e804: and dl, cl
14000e806: dec eax
14000e808: cmp eax, 0x7fffff
14000e80d: setb al
14000e810: or al, dl
14000e812: jne 0x14000e820
14000e814: xorps xmm11, xmm11
14000e818: jmp 0x14000e825 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000e81a: nop word ptr [rax + rax]
14000e820: mulss xmm11, xmm1
14000e825: movaps xmm4, xmmword ptr [rbp + 0x760]
14000e82c: movaps xmm9, xmm4
14000e830: movaps xmm12, xmmword ptr [rbp + 0x780]
14000e838: subps xmm9, xmm12
14000e83c: movaps xmm6, xmm8
14000e840: movss xmm10, dword ptr [rbp + 0x800]
14000e849: subss xmm6, xmm10
14000e84e: movaps xmm2, xmm9
14000e852: mulps xmm2, xmm9
14000e856: movshdup xmm3, xmm2
14000e85a: addss xmm3, xmm2
14000e85e: movaps xmm2, xmm6
14000e861: mulss xmm2, xmm6
14000e865: addss xmm2, xmm3
14000e869: sqrtss xmm2, xmm2
14000e86d: movss dword ptr [rbp + 0x7b0], xmm2
14000e875: movsldup xmm1, xmm1
14000e879: mulps xmm0, xmm1
14000e87c: movzx eax, al
14000e87f: movd xmm1, eax
14000e883: pshufd xmm14, xmm1, 0x50
14000e889: pslld xmm14, 0x1f
14000e88f: psrad xmm14, 0x1f
14000e895: pand xmm14, xmm0
14000e89a: mov qword ptr [rbp + 0x2a8], 2
14000e8a5: movabs rax, 0x3e99999a00000000
14000e8af: mov qword ptr [rbp + 0x2b0], rax
14000e8b6: subps xmm12, xmm4
14000e8ba: subss xmm10, xmm8
14000e8bf: xor esi, esi
14000e8c1: movss xmm7, dword ptr [rip + 0x29170b7]
14000e8c9: movss xmm0, dword ptr [rip + 0x29170ab]
14000e8d1: movss dword ptr [rbp + 0x778], xmm0
14000e8d9: nop dword ptr [rax]
14000e8e0: mov rax, rsi
14000e8e3: cmp rsi, 2
14000e8e7: je 0x14000ea60
14000e8ed: lea rsi, [rax + 1]
14000e8f1: mov qword ptr [rbp + 0x2a0], rsi
14000e8f8: movss xmm15, dword ptr [rbp + rax*4 + 0x2b0]
14000e902: movsldup xmm13, xmm15
14000e907: mulps xmm13, xmm14
14000e90b: mulss xmm15, xmm11
14000e910: movss xmm0, dword ptr [rbp + 0x800]
14000e918: addss xmm0, xmm15
14000e91d: movaps xmm1, xmmword ptr [rbp + 0x780]
14000e924: addps xmm1, xmm13
14000e928: movlps qword ptr [rbp + 0x3e0], xmm1
14000e92f: movss dword ptr [rbp + 0x3e8], xmm0
14000e937: movlps qword ptr [rbp + 0x3ec], xmm9
14000e93f: movss dword ptr [rbp + 0x3f4], xmm6
14000e947: mov dword ptr [rsp + 0x20], 0x3f800000
14000e94f: lea rcx, [rbp + 0x590]
14000e956: mov rdx, rbx
14000e959: lea r8, [rip + 0x2917c60]
14000e960: lea r9, [rbp + 0x3e0]
14000e967: call 0x1408ceb40 ; country_core::systems::collision::world::RaycastMesh::raycast
14000e96c: cmp dword ptr [rbp + 0x5c4], 2
14000e973: je 0x14000e9a8
14000e975: movss xmm0, dword ptr [rbp + 0x5c0]
14000e97d: mulss xmm0, dword ptr [rbp + 0x7b0]
14000e985: addss xmm0, dword ptr [rbp + 0x7f0]
14000e98d: movaps xmm1, xmm7
14000e990: cmpunordss xmm1, xmm7
14000e995: movaps xmm2, xmm1
14000e998: andps xmm2, xmm0
14000e99b: minss xmm0, xmm7
14000e99f: andnps xmm1, xmm0
14000e9a2: orps xmm1, xmm2
14000e9a5: movaps xmm7, xmm1
14000e9a8: addss xmm15, xmm8
14000e9ad: addps xmm13, xmmword ptr [rbp + 0x760]
14000e9b5: movlps qword ptr [rbp + 0x3e0], xmm13
14000e9bd: movss dword ptr [rbp + 0x3e8], xmm15
14000e9c6: movlps qword ptr [rbp + 0x3ec], xmm12
14000e9ce: movss dword ptr [rbp + 0x3f4], xmm10
14000e9d7: mov dword ptr [rsp + 0x20], 0x3f800000
14000e9df: lea rcx, [rbp + 0x590]
14000e9e6: mov rdx, rbx
14000e9e9: lea r8, [rip + 0x2917bd0]
14000e9f0: lea r9, [rbp + 0x3e0]
14000e9f7: call 0x1408ceb40 ; country_core::systems::collision::world::RaycastMesh::raycast
14000e9fc: cmp dword ptr [rbp + 0x5c4], 2
14000ea03: je 0x14000e8e0
14000ea09: movss xmm0, dword ptr [rip + 0x2916ef3]
14000ea11: subss xmm0, dword ptr [rbp + 0x5c0]
14000ea19: mulss xmm0, dword ptr [rbp + 0x7b0]
14000ea21: addss xmm0, dword ptr [rbp + 0x7f0]
14000ea29: movss xmm3, dword ptr [rbp + 0x778]
14000ea31: movaps xmm1, xmm3
14000ea34: cmpunordss xmm1, xmm3
14000ea39: movaps xmm2, xmm1
14000ea3c: andps xmm2, xmm0
14000ea3f: maxss xmm0, xmm3
14000ea43: andnps xmm1, xmm0
14000ea46: orps xmm1, xmm2
14000ea49: movss dword ptr [rbp + 0x778], xmm1
14000ea51: jmp 0x14000e8e0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000ea56: nop word ptr cs:[rax + rax]
14000ea60: movaps xmm0, xmm7
14000ea63: addss xmm0, dword ptr [rip + 0x2916f19]
14000ea6b: movss xmm1, dword ptr [rbp + 0x778]
14000ea73: ucomiss xmm1, xmm0
14000ea76: jbe 0x14000e6f4
14000ea7c: mov rcx, r15
14000ea7f: call 0x140b2a620 ; country_core::resources::walls::public_wall_state::PublicWallState::max_y
14000ea84: movaps xmm11, xmm0
14000ea88: mov r12, qword ptr [rbp + 0x208]
14000ea8f: mov rcx, r15
14000ea92: call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
14000ea97: subss xmm11, dword ptr [rbp + 0x570]
14000eaa0: addss xmm11, dword ptr [rip + 0x2916edf]
14000eaa9: movaps xmm1, xmm7
14000eaac: addss xmm1, dword ptr [rbp + 0x778]
14000eab4: mulss xmm1, dword ptr [rip + 0x2916db4]
14000eabc: mov dword ptr [rbp + 0x590], eax
14000eac2: movss dword ptr [rbp + 0x594], xmm0
14000eaca: lea rcx, [rbp + 0x590]
14000ead1: call 0x140c91eb0 ; utils::wall_space::impl$1::project_clamp
14000ead6: movaps xmm12, xmm0
14000eada: mov rsi, qword ptr [r12 + 0x10]
14000eadf: cmp rsi, qword ptr [r12]
14000eae3: jne 0x14000e6b0
14000eae9: mov rcx, r12
14000eaec: lea rdx, [rip + 0x2917ab5]
14000eaf3: call 0x140562db0 ; alloc::raw_vec::RawVec::grow_one<tuple$<ref$<str$>,enum2$<country_core::resources::gui::SettingsMenuState> >,alloc::alloc::Global>
14000eaf8: jmp 0x14000e6b0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000eafd: nop dword ptr [rax]
14000eb00: mov rax, qword ptr [rbp + 0x6b8]
14000eb07: test rax, rax
14000eb0a: je 0x14000eb26
14000eb0c: mov rcx, qword ptr [rbp + 0x6a8]
14000eb13: shl rax, 2
14000eb17: lea rdx, [rax + rax*2]
14000eb1b: mov r8d, 4
14000eb21: call 0x140613c20 ; ?
14000eb26: mov rax, qword ptr [rbp + 0x6d8]
14000eb2d: test rax, rax
14000eb30: xorps xmm8, xmm8
14000eb34: je 0x14000e2d0
14000eb3a: mov rcx, qword ptr [rbp + 0x6c8]
14000eb41: shl rax, 2
14000eb45: lea rdx, [rax + rax*2]
14000eb49: mov r8d, 4
14000eb4f: call 0x140613c20 ; ?
14000eb54: jmp 0x14000e2d0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000eb59: add rcx, rax
14000eb5c: movzx esi, byte ptr [rcx + 0x41]
14000eb60: jmp 0x14000e48e ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000eb65: cmp qword ptr [rbp + 0x668], 0
14000eb6d: mov rdx, qword ptr [rbp + 0x7a0]
14000eb74: je 0x14000eb8e
14000eb76: test rdx, rdx
14000eb79: je 0x14000eb8e
14000eb7b: mov rcx, qword ptr [rbp + 0x670]
14000eb82: mov r8, qword ptr [rbp + 0x7a8]
14000eb89: call 0x140613c20 ; ?
14000eb8e: mov rdi, qword ptr [rbp + 0x7e0]
14000eb95: mov rdx, qword ptr [rbp + 0x378]
14000eb9c: mov eax, dword ptr [rdx + 0x1c]
14000eb9f: mov rcx, qword ptr [rdx + 0x10]
14000eba3: mov dword ptr [rcx], eax
14000eba5: mov rcx, qword ptr [rdx]
14000eba8: mov eax, dword ptr [rbp + 0x32c]
14000ebae: mov rdx, qword ptr [rbp + 0x110]
14000ebb5: mov dword ptr [rdx], eax
14000ebb7: movzx eax, byte ptr [rbp + 0x500]
14000ebbe: mov rdx, qword ptr [rbp + 0x501]
14000ebc5: mov r8d, dword ptr [rbp + 0x509]
14000ebcc: movzx r9d, word ptr [rbp + 0x50d]
14000ebd4: movzx r10d, byte ptr [rbp + 0x50f]
14000ebdc: movups xmm0, xmmword ptr [rbp + 0x510]
14000ebe3: movaps xmmword ptr [rbp + 0x6b0], xmm0
14000ebea: mov byte ptr [rbp + 0x6a0], al
14000ebf0: mov qword ptr [rbp + 0x6a1], rdx
14000ebf7: mov dword ptr [rbp + 0x6a9], r8d
14000ebfe: mov word ptr [rbp + 0x6ad], r9w
14000ec06: mov byte ptr [rbp + 0x6af], r10b
14000ec0d: mov r9d, dword ptr [rdi + 0x128]
14000ec14: mov eax, dword ptr [rdi + 0x12c]
14000ec1a: mov byte ptr [rbp + 0x7ff], 0
14000ec21: mov dword ptr [rsp + 0x20], eax
14000ec25: mov rdx, qword ptr [rbp + 0xb8]
14000ec2c: lea r8, [rbp + 0x6a0]
14000ec33: call 0x1407deb60 ; system_decorator::decorator_visual::stairs_assemble::create_stairs_collider
14000ec38: mov rax, qword ptr [rbp + 0x458]
14000ec3f: test rax, rax
14000ec42: mov r13, qword ptr [rbp + 0x940]
14000ec49: je 0x14000eca7
14000ec4b: mov rcx, qword ptr [rbp + 0x440]
14000ec52: mov rdx, qword ptr [rbp + 0x450]
14000ec59: lea r8, [rdx + 1]
14000ec5d: cmp r8, rcx
14000ec60: mov r8d, 0
14000ec66: cmovae r8, rcx
14000ec6a: neg r8
14000ec6d: lea rcx, [rdx + r8]
14000ec71: inc rcx
14000ec74: mov qword ptr [rbp + 0x450], rcx
14000ec7b: dec rax
14000ec7e: mov qword ptr [rbp + 0x458], rax
14000ec85: imul rdx, rdx, 0xb8
14000ec8c: add rdx, qword ptr [rbp + 0x448]
14000ec93: mov r8d, 0xb8
14000ec99: lea rcx, [rbp + 0x6a0]
14000eca0: call 0x142923c00 ; ?
14000eca5: jmp 0x14000ecb8 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000eca7: movabs rax, 0x8000000000000000
14000ecb1: mov qword ptr [rbp + 0x6a0], rax
14000ecb8: lea rbx, [rbp + 0x590]
14000ecbf: lea r12, [rbp + 0x460]
14000ecc6: jmp 0x14000ecf0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000ecc8: imul rcx, rsi, 0xa8
14000eccf: add rcx, qword ptr [rdi + 8]
14000ecd3: mov r8d, 0xa8
14000ecd9: mov rdx, rbx
14000ecdc: call 0x142923c10 ; ?
14000ece1: inc rsi
14000ece4: mov qword ptr [rdi + 0x10], rsi
14000ece8: nop dword ptr [rax + rax]
14000ecf0: mov rcx, qword ptr [rbp + 0x6a0]
14000ecf7: movabs rax, 0x8000000000000000
14000ed01: mov qword ptr [rbp + 0x6a0], rax
14000ed08: mov qword ptr [rbp + 0x800], rcx
14000ed0f: mov rax, rcx
14000ed12: neg rax
14000ed15: jo 0x140010592
14000ed1b: mov rax, qword ptr [rbp + 0x6a8]
14000ed22: mov qword ptr [rbp + 0x7f0], rax
14000ed29: mov r14, qword ptr [rbp + 0x6b0]
14000ed30: mov r15, qword ptr [rbp + 0x6b8]
14000ed37: mov rax, qword ptr [rbp + 0x6c0]
14000ed3e: mov qword ptr [rbp + 0x528], rax
14000ed45: lea rcx, [rbp + 0x6d8]
14000ed4c: movdqu xmm0, xmmword ptr [rcx - 0x10]
14000ed51: movdqa xmmword ptr [rbp + 0x310], xmm0
14000ed59: mov r8, qword ptr [rbp + 0x6d8]
14000ed60: mov rax, qword ptr [rbp + 0x6e0]
14000ed67: mov qword ptr [rbp + 0x7b0], rax
14000ed6e: mov rbx, qword ptr [rbp + 0x6e8]
14000ed75: mov rax, qword ptr [rbp + 0x6f0]
14000ed7c: mov qword ptr [rbp + 0x780], rax
14000ed83: mov rax, qword ptr [rbp + 0x6f8]
14000ed8a: mov qword ptr [rbp + 0x760], rax
14000ed91: mov rdi, qword ptr [rbp + 0x700]
14000ed98: mov rdx, qword ptr [rbp + 0x708]
14000ed9f: mov rax, qword ptr [rbp + 0x710]
14000eda6: mov qword ptr [rbp + 0x778], rax
14000edad: movzx eax, word ptr [rcx + 0x66]
14000edb1: mov word ptr [rbp + 0x54c], ax
14000edb8: mov eax, dword ptr [rcx + 0x62]
14000edbb: mov dword ptr [rbp + 0x548], eax
14000edc1: mov rsi, qword ptr [rbp + 0x718]
14000edc8: movss xmm6, dword ptr [rbp + 0x734]
14000edd0: mov rax, qword ptr [rbp + 0x720]
14000edd7: mov qword ptr [rbp + 0x7b8], rax
14000edde: mov eax, dword ptr [rbp + 0x728]
14000ede4: mov dword ptr [rbp + 0x588], eax
14000edea: mov r10d, dword ptr [rbp + 0x72c]
14000edf1: mov r9d, dword ptr [rbp + 0x730]
14000edf8: movzx r11d, word ptr [rbp + 0x738]
14000ee00: mov rcx, qword ptr [rbp + 0x458]
14000ee07: test rcx, rcx
14000ee0a: mov qword ptr [rbp + 0x570], r15
14000ee11: mov qword ptr [rbp + 0x7e0], rdx
14000ee18: mov qword ptr [rbp + 0x648], r8
14000ee1f: je 0x14000f5c5
14000ee25: mov dword ptr [rbp + 0x560], r10d
14000ee2c: mov dword ptr [rbp + 0x580], r9d
14000ee33: mov rdx, qword ptr [rbp + 0x450]
14000ee3a: mov r8, qword ptr [rbp + 0x440]
14000ee41: mov rax, qword ptr [rbp + 0x448]
14000ee48: cmp rdx, r8
14000ee4b: mov r9d, 0
14000ee51: cmovae r9, r8
14000ee55: mov r10, rdx
14000ee58: sub r10, r9
14000ee5b: imul r9, r10, 0xb8
14000ee62: cmp byte ptr [rax + r9 + 0xb0], 2
14000ee6b: je 0x14000f51e
14000ee71: cmp byte ptr [rbp + 0x750], 2
14000ee78: je 0x14000f51e
14000ee7e: mov r10, qword ptr [rbp + 0x748]
14000ee85: add r9, rax
14000ee88: cmp r10, qword ptr [r9 + 0xa0]
14000ee8f: jne 0x14000f51e
14000ee95: mov r10, qword ptr [rbp + 0x740]
14000ee9c: mov qword ptr [rbp + 0x330], r10
14000eea3: mov r9, qword ptr [r9 + 0xa8]
14000eeaa: mov qword ptr [rbp + 0x530], r9
14000eeb1: lea r9, [rdx + 1]
14000eeb5: cmp r9, r8
14000eeb8: mov r9d, 0
14000eebe: cmovae r9, r8
14000eec2: neg r9
14000eec5: lea r8, [rdx + r9]
14000eec9: inc r8
14000eecc: mov qword ptr [rbp + 0x450], r8
14000eed3: dec rcx
14000eed6: mov qword ptr [rbp + 0x458], rcx
14000eedd: imul r9, rdx, 0xb8
14000eee4: mov rcx, qword ptr [rax + r9]
14000eee8: mov rdx, rcx
14000eeeb: neg rdx
14000eeee: jo 0x1400112fc
14000eef4: mov word ptr [rbp + 0x430], r11w
14000eefc: mov rdx, qword ptr [rax + r9 + 8]
14000ef01: mov r8, qword ptr [rax + r9 + 0x10]
14000ef06: mov r10, qword ptr [rax + r9 + 0x18]
14000ef0b: mov qword ptr [rbp + 0x7a8], r10
14000ef12: mov r10, qword ptr [rax + r9 + 0x20]
14000ef17: mov qword ptr [rbp + 0x568], r10
14000ef1e: mov r10, qword ptr [rax + r9 + 0x38]
14000ef23: mov qword ptr [rbp + 0x578], r10
14000ef2a: mov r10, qword ptr [rax + r9 + 0x40]
14000ef2f: mov qword ptr [rbp + 0x668], r10
14000ef36: mov r13, qword ptr [rax + r9 + 0x48]
14000ef3b: mov r10, qword ptr [rax + r9 + 0x50]
14000ef40: mov qword ptr [rbp + 0x520], r10
14000ef47: mov r10, qword ptr [rax + r9 + 0x58]
14000ef4c: mov qword ptr [rbp + 0x7a0], r10
14000ef53: mov r12, qword ptr [rax + r9 + 0x60]
14000ef58: mov r10, qword ptr [rax + r9 + 0x68]
14000ef5d: mov qword ptr [rbp + 0x670], r10
14000ef64: mov r10, qword ptr [rax + r9 + 0x70]
14000ef69: mov qword ptr [rbp + 0x798], r10
14000ef70: mov r15, qword ptr [rax + r9 + 0x78]
14000ef75: mov rax, qword ptr [rbp + 0x800]
14000ef7c: mov qword ptr [rbp + 0x650], rax
14000ef83: mov rax, qword ptr [rbp + 0x7f0]
14000ef8a: mov qword ptr [rbp + 0x658], rax
14000ef91: mov qword ptr [rbp + 0x660], r14
14000ef98: test r14, r14
14000ef9b: je 0x14000efa7
14000ef9d: dec r14
14000efa0: mov qword ptr [rbp + 0x660], r14
14000efa7: lea rax, [r8 + r8*2]
14000efab: lea rax, [rdx + rax*4]
14000efaf: mov qword ptr [rbp + 0x590], rdx
14000efb6: mov qword ptr [rbp + 0x598], rdx
14000efbd: mov qword ptr [rbp + 0x5a0], rcx
14000efc4: mov qword ptr [rbp + 0x5a8], rax
14000efcb: mov qword ptr [rbp + 0x5b0], 1
14000efd6: mov byte ptr [rbp + 0x7dd], 1
14000efdd: mov byte ptr [rbp + 0x7dc], 1
14000efe4: mov byte ptr [rbp + 0x7db], 1
14000efeb: mov byte ptr [rbp + 0x7da], 1
14000eff2: lea rcx, [rbp + 0x650]
14000eff9: lea rdx, [rbp + 0x590]
14000f000: lea r8, [rip + 0x29175f1]
14000f007: call 0x1402b9e00 ; alloc::vec::spec_extend::impl$0::spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::skip::Skip<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >,alloc::alloc::Global>
14000f00c: mov rax, qword ptr [rbp + 0x648]
14000f013: mov qword ptr [rbp + 0x680], rax
14000f01a: mov rax, qword ptr [rbp + 0x7b0]
14000f021: mov qword ptr [rbp + 0x688], rax
14000f028: mov qword ptr [rbp + 0x690], rbx
14000f02f: test rbx, rbx
14000f032: mov r14d, 0
14000f038: je 0x14000f044
14000f03a: dec rbx
14000f03d: mov qword ptr [rbp + 0x690], rbx
14000f044: mov rcx, qword ptr [rbp + 0x668]
14000f04b: lea rax, [rcx + r13*8]
14000f04f: mov qword ptr [rbp + 0x590], rcx
14000f056: mov qword ptr [rbp + 0x598], rcx
14000f05d: mov rcx, qword ptr [rbp + 0x578]
14000f064: mov qword ptr [rbp + 0x5a0], rcx
14000f06b: mov qword ptr [rbp + 0x5a8], rax
14000f072: mov qword ptr [rbp + 0x5b0], 1
14000f07d: mov byte ptr [rbp + 0x7d9], 1
14000f084: mov byte ptr [rbp + 0x7d8], 1
14000f08b: mov byte ptr [rbp + 0x7d7], 1
14000f092: mov byte ptr [rbp + 0x7d6], 1
14000f099: lea rcx, [rbp + 0x680]
14000f0a0: lea rbx, [rbp + 0x590]
14000f0a7: mov rdx, rbx
14000f0aa: lea r8, [rip + 0x291755f]
14000f0b1: call 0x1402ba000 ; alloc::vec::spec_extend::impl$0::spec_extend<glam::f32::vec2::Vec2,core::iter::adapters::skip::Skip<alloc::vec::into_iter::IntoIter<glam::f32::vec2::Vec2,alloc::alloc::Global> >,alloc::alloc::Global>
14000f0b6: mov rax, qword ptr [rbp + 0x780]
14000f0bd: mov qword ptr [rbp + 0x2a0], rax
14000f0c4: mov rax, qword ptr [rbp + 0x760]
14000f0cb: mov qword ptr [rbp + 0x2a8], rax
14000f0d2: mov qword ptr [rbp + 0x2b0], rdi
14000f0d9: test rdi, rdi
14000f0dc: mov r13, qword ptr [rbp + 0x940]
14000f0e3: je 0x14000f0ef
14000f0e5: dec rdi
14000f0e8: mov qword ptr [rbp + 0x2b0], rdi
14000f0ef: mov rcx, qword ptr [rbp + 0x7a0]
14000f0f6: lea rax, [rcx + r12*4]
14000f0fa: mov qword ptr [rbp + 0x590], rcx
14000f101: mov qword ptr [rbp + 0x598], rcx
14000f108: mov rcx, qword ptr [rbp + 0x520]
14000f10f: mov qword ptr [rbp + 0x5a0], rcx
14000f116: mov qword ptr [rbp + 0x5a8], rax
14000f11d: mov qword ptr [rbp + 0x5b0], 1
14000f128: mov byte ptr [rbp + 0x7d5], 1
14000f12f: mov byte ptr [rbp + 0x7d4], 1
14000f136: mov byte ptr [rbp + 0x7d3], 1
14000f13d: lea rcx, [rbp + 0x2a0]
14000f144: mov rdx, rbx
14000f147: lea r8, [rip + 0x29174da]
14000f14e: call 0x1402b9c50 ; alloc::vec::spec_extend::impl$0::spec_extend<f32,core::iter::adapters::skip::Skip<alloc::vec::into_iter::IntoIter<f32,alloc::alloc::Global> >,alloc::alloc::Global>
14000f153: mov rax, qword ptr [rbp + 0x7e0]
14000f15a: mov qword ptr [rbp + 0x3e0], rax
14000f161: mov rax, qword ptr [rbp + 0x778]
14000f168: mov qword ptr [rbp + 0x3e8], rax
14000f16f: mov qword ptr [rbp + 0x3f0], rsi
14000f176: test rsi, rsi
14000f179: lea r12, [rbp + 0x460]
14000f180: je 0x14000f18c
14000f182: dec rsi
14000f185: mov qword ptr [rbp + 0x3f0], rsi
14000f18c: mov rcx, qword ptr [rbp + 0x798]
14000f193: lea rax, [rcx + r15*4]
14000f197: mov qword ptr [rbp + 0x590], rcx
14000f19e: mov qword ptr [rbp + 0x598], rcx
14000f1a5: mov rcx, qword ptr [rbp + 0x670]
14000f1ac: mov qword ptr [rbp + 0x5a0], rcx
14000f1b3: mov qword ptr [rbp + 0x5a8], rax
14000f1ba: mov qword ptr [rbp + 0x5b0], 1
14000f1c5: mov byte ptr [rbp + 0x7d2], 1
14000f1cc: mov byte ptr [rbp + 0x7d1], 1
14000f1d3: lea rcx, [rbp + 0x3e0]
14000f1da: mov rdx, rbx
14000f1dd: lea r8, [rip + 0x291745c]
14000f1e4: call 0x1402b9c50 ; alloc::vec::spec_extend::impl$0::spec_extend<f32,core::iter::adapters::skip::Skip<alloc::vec::into_iter::IntoIter<f32,alloc::alloc::Global> >,alloc::alloc::Global>
14000f1e9: mov rax, qword ptr [rbp + 0x660]
14000f1f0: mov qword ptr [rbp + 0x5a0], rax
14000f1f7: movdqu xmm0, xmmword ptr [rbp + 0x650]
14000f1ff: movdqa xmmword ptr [rbp + 0x590], xmm0
14000f207: mov byte ptr [rbp + 0x7d2], 1
14000f20e: mov byte ptr [rbp + 0x7d1], 0
14000f215: lea rcx, [rbp + 0x380]
14000f21c: mov rdx, rbx
14000f21f: mov r8d, 1
14000f225: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000f22a: cmp r14, qword ptr [rbp + 0x380]
14000f231: jno 0x14000f2c0
14000f237: mov rdx, qword ptr [rbp + 0x3e0]
14000f23e: test rdx, rdx
14000f241: je 0x14000f259
14000f243: mov rcx, qword ptr [rbp + 0x3e8]
14000f24a: shl rdx, 2
14000f24e: mov r8d, 4
14000f254: call 0x140613c20 ; ?
14000f259: mov rdx, qword ptr [rbp + 0x2a0]
14000f260: test rdx, rdx
14000f263: mov rsi, qword ptr [rbp + 0x570]
14000f26a: mov rdi, qword ptr [rbp + 0x7a8]
14000f271: je 0x14000f289
14000f273: mov rcx, qword ptr [rbp + 0x2a8]
14000f27a: shl rdx, 2
14000f27e: mov r8d, 4
14000f284: call 0x140613c20 ; ?
14000f289: mov rdx, qword ptr [rbp + 0x680]
14000f290: test rdx, rdx
14000f293: je 0x14000f4d9
14000f299: mov rcx, qword ptr [rbp + 0x688]
14000f2a0: shl rdx, 3
14000f2a4: mov r8d, 4
14000f2aa: call 0x140613c20 ; ?
14000f2af: jmp 0x14000f4d9 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000f2b4: nop word ptr cs:[rax + rax]
14000f2c0: movaps xmmword ptr [rbp + 0x290], xmm6
14000f2c7: mov rax, qword ptr [rbp + 0x3b0]
14000f2ce: mov qword ptr [rbp + 0x5c0], rax
14000f2d5: movups xmm0, xmmword ptr [rbp + 0x380]
14000f2dc: movups xmm1, xmmword ptr [rbp + 0x390]
14000f2e3: movups xmm2, xmmword ptr [rbp + 0x3a0]
14000f2ea: movaps xmmword ptr [rbp + 0x5b0], xmm2
14000f2f1: movaps xmmword ptr [rbp + 0x5a0], xmm1
14000f2f8: movaps xmmword ptr [rbp + 0x590], xmm0
14000f2ff: mov rax, qword ptr [rbp + 0x690]
14000f306: lea rcx, [rbp + 0x598]
14000f30d: mov qword ptr [rcx + 0x40], rax
14000f311: movups xmm0, xmmword ptr [rbp + 0x680]
14000f318: movups xmmword ptr [rcx + 0x30], xmm0
14000f31c: mov rax, qword ptr [rbp + 0x2b0]
14000f323: mov qword ptr [rcx + 0x58], rax
14000f327: movups xmm0, xmmword ptr [rbp + 0x2a0]
14000f32e: movups xmmword ptr [rcx + 0x48], xmm0
14000f332: mov rax, qword ptr [rbp + 0x3f0]
14000f339: mov qword ptr [rcx + 0x70], rax
14000f33d: movups xmm0, xmmword ptr [rbp + 0x3e0]
14000f344: movups xmmword ptr [rcx + 0x60], xmm0
14000f348: movaps xmm0, xmmword ptr [rbp + 0x590]
14000f34f: movaps xmm1, xmmword ptr [rbp + 0x5a0]
14000f356: movaps xmm2, xmmword ptr [rbp + 0x5b0]
14000f35d: movaps xmm3, xmmword ptr [rbp + 0x5c0]
14000f364: movaps xmmword ptr [rbp + 0x460], xmm0
14000f36b: movaps xmmword ptr [rbp + 0x470], xmm1
14000f372: movaps xmmword ptr [rbp + 0x480], xmm2
14000f379: movaps xmmword ptr [rbp + 0x490], xmm3
14000f380: movaps xmm0, xmmword ptr [rbp + 0x5d0]
14000f387: movaps xmmword ptr [rbp + 0x4a0], xmm0
14000f38e: movaps xmm0, xmmword ptr [rbp + 0x5e0]
14000f395: movaps xmmword ptr [rbp + 0x4b0], xmm0
14000f39c: movaps xmm0, xmmword ptr [rbp + 0x5f0]
14000f3a3: movaps xmmword ptr [rbp + 0x4c0], xmm0
14000f3aa: movaps xmm0, xmmword ptr [rbp + 0x600]
14000f3b1: movaps xmmword ptr [rbp + 0x4d0], xmm0
14000f3b8: cmp r14, qword ptr [rbp + 0x6a0]
14000f3bf: jo 0x14000f3cd
14000f3c1: lea rcx, [rbp + 0x6a0]
14000f3c8: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
14000f3cd: movaps xmm0, xmmword ptr [rbp + 0x4d0]
14000f3d4: movaps xmmword ptr [rbp + 0x710], xmm0
14000f3db: movaps xmm0, xmmword ptr [rbp + 0x4c0]
14000f3e2: movaps xmmword ptr [rbp + 0x700], xmm0
14000f3e9: movaps xmm0, xmmword ptr [rbp + 0x4b0]
14000f3f0: movaps xmmword ptr [rbp + 0x6f0], xmm0
14000f3f7: movaps xmm0, xmmword ptr [rbp + 0x4a0]
14000f3fe: movaps xmmword ptr [rbp + 0x6e0], xmm0
14000f405: movaps xmm0, xmmword ptr [rbp + 0x460]
14000f40c: movdqa xmm1, xmmword ptr [rbp + 0x470]
14000f414: movdqa xmm2, xmmword ptr [rbp + 0x480]
14000f41c: movapd xmm3, xmmword ptr [rbp + 0x490]
14000f424: movapd xmmword ptr [rbp + 0x6d0], xmm3
14000f42c: movdqa xmmword ptr [rbp + 0x6c0], xmm2
14000f434: movdqa xmmword ptr [rbp + 0x6b0], xmm1
14000f43c: movaps xmmword ptr [rbp + 0x6a0], xmm0
14000f443: mov rax, qword ptr [rbp + 0x7b8]
14000f44a: mov qword ptr [rbp + 0x720], rax
14000f451: mov eax, dword ptr [rbp + 0x588]
14000f457: mov dword ptr [rbp + 0x728], eax
14000f45d: mov eax, dword ptr [rbp + 0x560]
14000f463: mov dword ptr [rbp + 0x72c], eax
14000f469: mov eax, dword ptr [rbp + 0x580]
14000f46f: mov dword ptr [rbp + 0x730], eax
14000f475: movdqa xmm0, xmmword ptr [rbp + 0x290]
14000f47d: movd eax, xmm0
14000f481: mov word ptr [rbp + 0x734], ax
14000f488: mov word ptr [rbp + 0x736], 0
14000f491: movzx eax, word ptr [rbp + 0x430]
14000f498: mov word ptr [rbp + 0x738], ax
14000f49f: mov word ptr [rbp + 0x73a], 0x300
14000f4a8: mov rax, qword ptr [rbp + 0x330]
14000f4af: mov qword ptr [rbp + 0x740], rax
14000f4b6: mov rax, qword ptr [rbp + 0x530]
14000f4bd: mov qword ptr [rbp + 0x748], rax
14000f4c4: mov byte ptr [rbp + 0x750], 0
14000f4cb: mov rsi, qword ptr [rbp + 0x570]
14000f4d2: mov rdi, qword ptr [rbp + 0x7a8]
14000f4d9: test rdi, rdi
14000f4dc: je 0x14000f4f7
14000f4de: shl rdi, 2
14000f4e2: mov r8d, 4
14000f4e8: mov rcx, qword ptr [rbp + 0x568]
14000f4ef: mov rdx, rdi
14000f4f2: call 0x140613c20 ; ?
14000f4f7: test rsi, rsi
14000f4fa: je 0x14000ecf0
14000f500: shl rsi, 2
14000f504: mov r8d, 4
14000f50a: mov rcx, qword ptr [rbp + 0x528]
14000f511: mov rdx, rsi
14000f514: call 0x140613c20 ; ?
14000f519: jmp 0x14000ecf0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000f51e: lea r9, [rdx + 1]
14000f522: cmp r9, r8
14000f525: mov r9d, 0
14000f52b: cmovae r9, r8
14000f52f: neg r9
14000f532: lea r8, [rdx + r9]
14000f536: inc r8
14000f539: mov qword ptr [rbp + 0x450], r8
14000f540: dec rcx
14000f543: mov qword ptr [rbp + 0x458], rcx
14000f54a: imul rcx, rdx, 0xb8
14000f551: mov r15, qword ptr [rax + rcx]
14000f555: lea rdx, [rax + rcx]
14000f559: add rdx, 8
14000f55d: mov r8d, 0xb0
14000f563: lea r12, [rbp + 0x590]
14000f56a: mov rcx, r12
14000f56d: mov r13d, r11d
14000f570: call 0x142923c00 ; ?
14000f575: mov qword ptr [rbp + 0x6a0], r15
14000f57c: mov r15, qword ptr [rbp + 0x570]
14000f583: mov r8d, 0xb0
14000f589: lea rcx, [rbp + 0x6a8]
14000f590: mov rdx, r12
14000f593: lea r12, [rbp + 0x460]
14000f59a: call 0x142923c00 ; ?
14000f59f: mov r11d, r13d
14000f5a2: mov r13, qword ptr [rbp + 0x940]
14000f5a9: mov rdx, qword ptr [rbp + 0x7e0]
14000f5b0: mov r8, qword ptr [rbp + 0x648]
14000f5b7: mov r9d, dword ptr [rbp + 0x580]
14000f5be: mov r10d, dword ptr [rbp + 0x560]
14000f5c5: mov rax, qword ptr [rbp + 0x800]
14000f5cc: mov qword ptr [rbp + 0x460], rax
14000f5d3: mov rax, qword ptr [rbp + 0x7f0]
14000f5da: mov qword ptr [rbp + 0x468], rax
14000f5e1: mov qword ptr [rbp + 0x470], r14
14000f5e8: mov qword ptr [rbp + 0x478], r15
14000f5ef: mov rax, qword ptr [rbp + 0x528]
14000f5f6: mov qword ptr [rbp + 0x480], rax
14000f5fd: movaps xmm0, xmmword ptr [rbp + 0x310]
14000f604: lea rcx, [rbp + 0x468]
14000f60b: movups xmmword ptr [rcx + 0x20], xmm0
14000f60f: mov qword ptr [rbp + 0x498], r8
14000f616: mov rax, qword ptr [rbp + 0x7b0]
14000f61d: mov qword ptr [rbp + 0x4a0], rax
14000f624: mov qword ptr [rbp + 0x4a8], rbx
14000f62b: mov rax, qword ptr [rbp + 0x780]
14000f632: mov qword ptr [rbp + 0x4b0], rax
14000f639: mov r8, qword ptr [rbp + 0x760]
14000f640: mov qword ptr [rbp + 0x4b8], r8
14000f647: mov qword ptr [rbp + 0x4c0], rdi
14000f64e: mov qword ptr [rbp + 0x4c8], rdx
14000f655: mov rax, qword ptr [rbp + 0x778]
14000f65c: mov qword ptr [rbp + 0x4d0], rax
14000f663: mov qword ptr [rbp + 0x4d8], rsi
14000f66a: mov rax, qword ptr [rbp + 0x7b8]
14000f671: mov qword ptr [rbp + 0x4e0], rax
14000f678: mov eax, dword ptr [rbp + 0x588]
14000f67e: mov dword ptr [rbp + 0x4e8], eax
14000f684: mov dword ptr [rbp + 0x4ec], r10d
14000f68b: mov dword ptr [rbp + 0x4f0], r9d
14000f692: movss dword ptr [rbp + 0x4f4], xmm6
14000f69a: mov word ptr [rbp + 0x4f8], r11w
14000f6a2: movzx eax, word ptr [rbp + 0x54c]
14000f6a9: mov word ptr [rcx + 0x96], ax
14000f6b0: mov eax, dword ptr [rbp + 0x548]
14000f6b6: mov dword ptr [rcx + 0x92], eax
14000f6bc: test rdi, rdi
14000f6bf: je 0x1400113fa
14000f6c5: lea rax, [r8 + rdi*4]
14000f6c9: add rax, -4
14000f6cd: lea rbx, [rbp + 0x590]
14000f6d4: je 0x140011412
14000f6da: cmp byte ptr [rbp + 0x538], 0
14000f6e1: je 0x14000f7cb
14000f6e7: mov rax, qword ptr [rbp + 0x258]
14000f6ee: mov ecx, dword ptr [rbp + 0x424]
14000f6f4: mov dword ptr [rax], ecx
14000f6f6: mov rax, qword ptr [rbp + 0x490]
14000f6fd: mov qword ptr [rbp + 0x5c0], rax
14000f704: movups xmm0, xmmword ptr [rbp + 0x460]
14000f70b: movups xmm1, xmmword ptr [rbp + 0x470]
14000f712: movups xmm2, xmmword ptr [rbp + 0x480]
14000f719: movaps xmmword ptr [rbp + 0x5b0], xmm2
14000f720: movaps xmmword ptr [rbp + 0x5a0], xmm1
14000f727: movaps xmmword ptr [rbp + 0x590], xmm0
14000f72e: lea rdx, [rbp + 0x468]
14000f735: mov rax, qword ptr [rdx + 0x40]
14000f739: lea rcx, [rbp + 0x598]
14000f740: mov qword ptr [rcx + 0x40], rax
14000f744: movups xmm0, xmmword ptr [rdx + 0x30]
14000f748: movups xmmword ptr [rcx + 0x30], xmm0
14000f74c: mov rax, qword ptr [rdx + 0x58]
14000f750: mov qword ptr [rcx + 0x70], rax
14000f754: movups xmm0, xmmword ptr [rdx + 0x48]
14000f758: movups xmmword ptr [rcx + 0x60], xmm0
14000f75c: mov rax, qword ptr [rdx + 0x70]
14000f760: mov qword ptr [rcx + 0x58], rax
14000f764: movups xmm0, xmmword ptr [rdx + 0x60]
14000f768: movups xmmword ptr [rcx + 0x48], xmm0
14000f76c: mov rax, qword ptr [rbp + 0x758]
14000f773: mov dword ptr [rbp + 0x630], eax
14000f779: mov rax, qword ptr [rbp + 0x678]
14000f780: mov rax, qword ptr [rax]
14000f783: mov qword ptr [rbp + 0x628], rax
14000f78a: mov word ptr [rbp + 0x634], r11w
14000f792: movabs rax, 0x8000000000000000
14000f79c: mov qword ptr [rbp + 0x610], rax
14000f7a3: mov rdi, qword ptr [rbp + 0x348]
14000f7aa: mov rsi, qword ptr [rdi + 0x10]
14000f7ae: cmp rsi, qword ptr [rdi]
14000f7b1: jne 0x14000ecc8
14000f7b7: mov rcx, rdi
14000f7ba: lea rdx, [rip + 0x2916fcf]
14000f7c1: call 0x1405636c0 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsBracketAssemblyParams,alloc::alloc::Global>
14000f7c6: jmp 0x14000ecc8 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000f7cb: movss xmm0, dword ptr [r8]
14000f7d0: movss xmm1, dword ptr [rax]
14000f7d4: movaps xmm2, xmm0
14000f7d7: cmpunordss xmm2, xmm0
14000f7dc: movaps xmm3, xmm2
14000f7df: andps xmm3, xmm1
14000f7e2: maxss xmm1, xmm0
14000f7e6: andnps xmm2, xmm1
14000f7e9: orps xmm2, xmm3
14000f7ec: movss xmm0, dword ptr [rip + 0x29160dc]
14000f7f4: ucomiss xmm0, xmm2
14000f7f7: jb 0x14000f832
14000f7f9: mov byte ptr [rbp + 0x814], 1
14000f800: mov rcx, qword ptr [rbp + 0x2e8]
14000f807: call 0x140959ef0 ; country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
14000f80c: mov rdi, rax
14000f80f: mov byte ptr [rbp + 0x814], 1
14000f816: mov rcx, qword ptr [rbp + 0x118]
14000f81d: call 0x140959ef0 ; country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
14000f822: test rdi, rdi
14000f825: sete cl
14000f828: test rax, rax
14000f82b: sete dl
14000f82e: or dl, cl
14000f830: je 0x14000f888
14000f832: mov rax, qword ptr [rbp + 0x340]
14000f839: mov ecx, dword ptr [rbp + 0x53c]
14000f83f: mov dword ptr [rax], ecx
14000f841: mov r8d, 0xa0
14000f847: mov rcx, rbx
14000f84a: mov rdx, r12
14000f84d: call 0x142923c00 ; ?
14000f852: mov rdi, qword ptr [rbp + 0x370]
14000f859: mov rsi, qword ptr [rdi + 0x10]
14000f85d: cmp rsi, qword ptr [rdi]
14000f860: jne 0x14000f871
14000f862: mov rcx, rdi
14000f865: lea rdx, [rip + 0x2916f0c]
14000f86c: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
14000f871: lea rcx, [rsi + rsi*4]
14000f875: shl rcx, 5
14000f879: add rcx, qword ptr [rdi + 8]
14000f87d: mov r8d, 0xa0
14000f883: jmp 0x14000ecd9 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000f888: mov rcx, qword ptr [rdi]
14000f88b: cmp rcx, qword ptr [rax]
14000f88e: jne 0x14000f89d
14000f890: movzx r11d, word ptr [rbp + 0x4f8]
14000f898: jmp 0x14000f6e7 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000f89d: mov byte ptr [rbp + 0x814], 1
14000f8a4: mov qword ptr [rsp + 0x20], rdi
14000f8a9: mov rcx, rbx
14000f8ac: mov rdx, qword ptr [rbp + 0x378]
14000f8b3: mov r8, qword ptr [rbp + 0x938]
14000f8ba: mov r9, r12
14000f8bd: call 0x1407d94a0 ; system_decorator::decorator_visual::stairs_assemble::split_segment_into_supported_and_unsupported_subsegments
14000f8c2: mov rax, qword ptr [rbp + 0x590]
14000f8c9: mov qword ptr [rbp + 0x7a8], rax
14000f8d0: mov r13, qword ptr [rbp + 0x598]
14000f8d7: mov rcx, qword ptr [rbp + 0x5a0]
14000f8de: mov rax, qword ptr [rbp + 0x5a8]
14000f8e5: mov qword ptr [rbp + 0x560], rax
14000f8ec: mov rax, qword ptr [rbp + 0x5b0]
14000f8f3: mov qword ptr [rbp + 0x530], rax
14000f8fa: mov rax, qword ptr [rbp + 0x5b8]
14000f901: mov qword ptr [rbp + 0x670], rax
14000f908: mov rax, qword ptr [rbp + 0x5c0]
14000f90f: mov qword ptr [rbp + 0x580], rax
14000f916: mov rax, qword ptr [rbp + 0x5c8]
14000f91d: mov qword ptr [rbp + 0x568], rax
14000f924: mov rax, qword ptr [rbp + 0x5d0]
14000f92b: mov qword ptr [rbp + 0x330], rax
14000f932: shl rcx, 4
14000f936: add rcx, r13
14000f939: mov qword ptr [rbp + 0x520], rcx
14000f940: mov qword ptr [rbp + 0x798], r13
14000f947: nop word ptr [rax + rax]
14000f950: mov r12, r13
14000f953: cmp r12, qword ptr [rbp + 0x520]
14000f95a: mov qword ptr [rbp + 0x578], r12
14000f961: je 0x14000fe23
14000f967: mov r14, qword ptr [r12 + 8]
14000f96c: cmp r14, -1
14000f970: je 0x140011001
14000f976: mov r13, qword ptr [r12]
14000f97a: lea rax, [r14 + 1]
14000f97e: mov rdi, rax
14000f981: sub rdi, r13
14000f984: mov r15, qword ptr [rbp + 0x80]
14000f98b: jb 0x140011012
14000f991: mov rdx, qword ptr [rbp + 0x470]
14000f998: cmp r14, rdx
14000f99b: jae 0x140011029
14000f9a1: mov qword ptr [rbp + 0x7a0], rax
14000f9a8: mov rax, rdi
14000f9ab: mov ecx, 0xc
14000f9b0: mul rcx
14000f9b3: jo 0x14001142a
14000f9b9: mov esi, 0
14000f9be: movabs rcx, 0x7ffffffffffffffc
14000f9c8: cmp rax, rcx
14000f9cb: ja 0x14001142c
14000f9d1: mov rbx, qword ptr [rbp + 0x468]
14000f9d8: test rax, rax
14000f9db: je 0x14000fa10
14000f9dd: movzx ecx, byte ptr [rip + 0x3625225]
14000f9e4: mov esi, 4
14000f9e9: mov edx, 4
14000f9ee: mov rcx, rax
14000f9f1: mov r15, rax
14000f9f4: call 0x140613c10 ; ?
14000f9f9: test rax, rax
14000f9fc: je 0x14001142c
14000fa02: mov rsi, rdi
14000fa05: mov r15, rax
14000fa08: jmp 0x14000fa18 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000fa0a: nop word ptr [rax + rax]
14000fa10: mov r15d, 4
14000fa16: xor esi, esi
14000fa18: mov qword ptr [rbp + 0x668], r13
14000fa1f: lea rax, [r13*2]
14000fa27: add rax, r13
14000fa2a: lea rdx, [rbx + rax*4]
14000fa2e: lea r12, [rdi*4]
14000fa36: lea r8, [r12 + r12*2]
14000fa3a: mov rcx, r15
14000fa3d: call 0x142923c00 ; ?
14000fa42: mov qword ptr [rbp + 0x380], rsi
14000fa49: mov qword ptr [rbp + 0x388], r15
14000fa50: mov qword ptr [rbp + 0x390], rdi
14000fa57: lea rcx, [rbp + 0x590]
14000fa5e: lea rdx, [rbp + 0x380]
14000fa65: mov r8d, 1
14000fa6b: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
14000fa70: mov qword ptr [rbp + 0x80], r15
14000fa77: mov rax, qword ptr [rbp + 0x578]
14000fa7e: lea r13, [rax + 0x10]
14000fa82: mov rax, qword ptr [rbp + 0x590]
14000fa89: mov rcx, rax
14000fa8c: neg rcx
14000fa8f: jo 0x14000f950
14000fa95: mov rcx, qword ptr [rbp + 0x598]
14000fa9c: mov r15, qword ptr [rbp + 0x5a8]
14000faa3: mov rbx, qword ptr [rbp + 0x5b0]
14000faaa: movss xmm0, dword ptr [rbp + 0x5c0]
14000fab2: ucomiss xmm0, dword ptr [rip + 0x2915ed3]
14000fab9: ja 0x14000faf9
14000fabb: test rax, rax
14000fabe: je 0x14000fad3
14000fac0: shl rax, 2
14000fac4: lea rdx, [rax + rax*2]
14000fac8: mov r8d, 4
14000face: call 0x140613c20 ; ?
14000fad3: mov r12, r13
14000fad6: test r15, r15
14000fad9: je 0x14000f953
14000fadf: shl r15, 2
14000fae3: mov r8d, 4
14000fae9: mov rcx, rbx
14000faec: mov rdx, r15
14000faef: call 0x140613c20 ; ?
14000faf4: jmp 0x14000f950 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000faf9: mov rdx, qword ptr [rbp + 0x5a0]
14000fb00: mov r8, qword ptr [rbp + 0x5b8]
14000fb07: mov r9d, dword ptr [rbp + 0x5c4]
14000fb0e: mov r10, qword ptr [rbp + 0x340]
14000fb15: mov r11d, dword ptr [rbp + 0x53c]
14000fb1c: mov dword ptr [r10], r11d
14000fb1f: mov qword ptr [rbp + 0x380], rax
14000fb26: mov qword ptr [rbp + 0x388], rcx
14000fb2d: mov qword ptr [rbp + 0x390], rdx
14000fb34: mov qword ptr [rbp + 0x398], r15
14000fb3b: mov qword ptr [rbp + 0x3a0], rbx
14000fb42: mov qword ptr [rbp + 0x3a8], r8
14000fb49: movss dword ptr [rbp + 0x3b0], xmm0
14000fb51: mov dword ptr [rbp + 0x3b4], r9d
14000fb58: mov rdx, qword ptr [rbp + 0x4a8]
14000fb5f: cmp r14, rdx
14000fb62: jae 0x140011207
14000fb68: mov rax, rdi
14000fb6b: shr rax, 0x3c
14000fb6f: jne 0x140011664
14000fb75: mov rax, qword ptr [rbp + 0x668]
14000fb7c: lea r15, [rax*8]
14000fb84: add r15, qword ptr [rbp + 0x4a0]
14000fb8b: lea rbx, [rdi*8]
14000fb93: test rdi, rdi
14000fb96: je 0x14000fbc3
14000fb98: movzx eax, byte ptr [rip + 0x362506a]
14000fb9f: mov esi, 4
14000fba4: mov edx, 4
14000fba9: mov rcx, rbx
14000fbac: call 0x140613c10 ; ?
14000fbb1: test rax, rax
14000fbb4: je 0x140011741
14000fbba: mov qword ptr [rbp + 0x588], rdi
14000fbc1: jmp 0x14000fbd3 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000fbc3: mov eax, 4
14000fbc8: mov qword ptr [rbp + 0x588], 0
14000fbd3: mov qword ptr [rbp + 0x2f8], rax
14000fbda: mov rcx, rax
14000fbdd: mov rdx, r15
14000fbe0: mov r8, rbx
14000fbe3: call 0x142923c00 ; ?
14000fbe8: mov rdx, qword ptr [rbp + 0x4c0]
14000fbef: cmp r14, rdx
14000fbf2: jae 0x14001121f
14000fbf8: mov rax, qword ptr [rbp + 0x668]
14000fbff: lea rsi, [rax*4]
14000fc07: add rsi, qword ptr [rbp + 0x4b8]
14000fc0e: test rdi, rdi
14000fc11: je 0x14000fc8b
14000fc13: movzx eax, byte ptr [rip + 0x3624fef]
14000fc1a: mov edx, 4
14000fc1f: mov rcx, r12
14000fc22: call 0x140613c10 ; ?
14000fc27: test rax, rax
14000fc2a: je 0x140011701
14000fc30: mov rbx, rax
14000fc33: mov rcx, rax
14000fc36: mov rdx, rsi
14000fc39: mov r8, r12
14000fc3c: call 0x142923c00 ; ?
14000fc41: mov rdx, qword ptr [rbp + 0x4d8]
14000fc48: cmp r14, rdx
14000fc4b: jae 0x140011332
14000fc51: mov rsi, rbx
14000fc54: mov rbx, qword ptr [rbp + 0x4d0]
14000fc5b: movzx eax, byte ptr [rip + 0x3624fa7]
14000fc62: mov edx, 4
14000fc67: mov rcx, r12
14000fc6a: call 0x140613c10 ; ?
14000fc6f: test rax, rax
14000fc72: je 0x14001171a
14000fc78: mov r15, rax
14000fc7b: mov qword ptr [rbp + 0x7a0], rdi
14000fc82: mov qword ptr [rbp + 0x7b8], rsi
14000fc89: jmp 0x14000fcd4 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000fc8b: mov ebx, 4
14000fc90: mov ecx, 4
14000fc95: mov rdx, rsi
14000fc98: mov r8, r12
14000fc9b: call 0x142923c00 ; ?
14000fca0: mov rdx, qword ptr [rbp + 0x4d8]
14000fca7: cmp r14, rdx
14000fcaa: jae 0x140011330
14000fcb0: mov eax, 4
14000fcb5: mov qword ptr [rbp + 0x7b8], rax
14000fcbc: mov rbx, qword ptr [rbp + 0x4d0]
14000fcc3: mov qword ptr [rbp + 0x7a0], 0
14000fcce: mov r15d, 4
14000fcd4: mov rsi, qword ptr [rbp + 0x668]
14000fcdb: lea rdx, [rbx + rsi*4]
14000fcdf: mov rcx, r15
14000fce2: mov r8, r12
14000fce5: call 0x142923c00 ; ?
14000fcea: mov rax, qword ptr [rbp + 0x470]
14000fcf1: dec rax
14000fcf4: test rsi, rsi
14000fcf7: movups xmm0, xmmword ptr [rbp + 0x380]
14000fcfe: movups xmm1, xmmword ptr [rbp + 0x390]
14000fd05: movups xmm2, xmmword ptr [rbp + 0x3a0]
14000fd0c: movaps xmmword ptr [rbp + 0x590], xmm0
14000fd13: movaps xmmword ptr [rbp + 0x5a0], xmm1
14000fd1a: movaps xmmword ptr [rbp + 0x5b0], xmm2
14000fd21: mov rcx, qword ptr [rbp + 0x3b0]
14000fd28: mov qword ptr [rbp + 0x5c0], rcx
14000fd2f: mov rcx, qword ptr [rbp + 0x588]
14000fd36: mov qword ptr [rbp + 0x5c8], rcx
14000fd3d: mov rcx, qword ptr [rbp + 0x2f8]
14000fd44: mov qword ptr [rbp + 0x5d0], rcx
14000fd4b: mov qword ptr [rbp + 0x5d8], rdi
14000fd52: mov rcx, qword ptr [rbp + 0x7a0]
14000fd59: mov qword ptr [rbp + 0x5e0], rcx
14000fd60: mov rdx, qword ptr [rbp + 0x7b8]
14000fd67: mov qword ptr [rbp + 0x5e8], rdx
14000fd6e: mov qword ptr [rbp + 0x5f0], rdi
14000fd75: mov qword ptr [rbp + 0x5f8], rcx
14000fd7c: mov qword ptr [rbp + 0x600], r15
14000fd83: mov qword ptr [rbp + 0x608], rdi
14000fd8a: mov rcx, qword ptr [rbp + 0x438]
14000fd91: mov qword ptr [rbp + 0x610], rcx
14000fd98: mov word ptr [rbp + 0x624], 0x101
14000fda1: mov rcx, qword ptr [rbp + 0x758]
14000fda8: mov dword ptr [rbp + 0x620], ecx
14000fdae: mov rcx, qword ptr [rbp + 0x678]
14000fdb5: mov rcx, qword ptr [rcx]
14000fdb8: setne byte ptr [rbp + 0x626]
14000fdbf: cmp r14, rax
14000fdc2: mov qword ptr [rbp + 0x618], rcx
14000fdc9: setne byte ptr [rbp + 0x627]
14000fdd0: mov dword ptr [rbp + 0x628], 0x3000000
14000fdda: mov rdi, qword ptr [rbp + 0x370]
14000fde1: mov rsi, qword ptr [rdi + 0x10]
14000fde5: cmp rsi, qword ptr [rdi]
14000fde8: jne 0x14000fdf9
14000fdea: mov rcx, rdi
14000fded: lea rdx, [rip + 0x291696c]
14000fdf4: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
14000fdf9: lea rcx, [rsi + rsi*4]
14000fdfd: shl rcx, 5
14000fe01: add rcx, qword ptr [rdi + 8]
14000fe05: mov r8d, 0xa0
14000fe0b: lea rdx, [rbp + 0x590]
14000fe12: call 0x142923c10 ; ?
14000fe17: inc rsi
14000fe1a: mov qword ptr [rdi + 0x10], rsi
14000fe1e: jmp 0x14000f950 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000fe23: mov rdx, qword ptr [rbp + 0x7a8]
14000fe2a: test rdx, rdx
14000fe2d: lea rbx, [rbp + 0x590]
14000fe34: mov rsi, qword ptr [rbp + 0x330]
14000fe3b: je 0x14000fe53
14000fe3d: shl rdx, 4
14000fe41: mov r8d, 8
14000fe47: mov rcx, qword ptr [rbp + 0x798]
14000fe4e: call 0x140613c20 ; ?
14000fe53: mov rdx, qword ptr [rbp + 0x670]
14000fe5a: shl rdx, 4
14000fe5e: mov rcx, qword ptr [rbp + 0x530]
14000fe65: add rdx, rcx
14000fe68: lea rax, [rsi + rsi*2]
14000fe6c: mov r13, qword ptr [rbp + 0x568]
14000fe73: lea r8, [rax*8]
14000fe7b: add r8, r13
14000fe7e: lea rax, [rbp + 0x3c0]
14000fe85: movups xmmword ptr [rax], xmm8
14000fe89: mov qword ptr [rax + 0x10], 0
14000fe91: mov qword ptr [rbp + 0x380], rcx
14000fe98: mov qword ptr [rbp + 0x388], rcx
14000fe9f: mov rax, qword ptr [rbp + 0x560]
14000fea6: mov qword ptr [rbp + 0x390], rax
14000fead: mov qword ptr [rbp + 0x670], rdx
14000feb4: mov qword ptr [rbp + 0x398], rdx
14000febb: mov qword ptr [rbp + 0x3a0], r13
14000fec2: mov qword ptr [rbp + 0x3a8], r13
14000fec9: mov rax, qword ptr [rbp + 0x580]
14000fed0: mov qword ptr [rbp + 0x3b0], rax
14000fed7: mov qword ptr [rbp + 0x798], r8
14000fede: mov qword ptr [rbp + 0x3b8], r8
14000fee5: mov r12, r13
14000fee8: mov r14, rcx
14000feeb: nop dword ptr [rax + rax]
14000fef0: mov rax, r14
14000fef3: cmp rax, qword ptr [rbp + 0x670]
14000fefa: je 0x140010409
14000ff00: lea r13, [rax + 0x10]
14000ff04: mov qword ptr [rbp + 0x388], r13
14000ff0b: cmp r12, qword ptr [rbp + 0x798]
14000ff12: je 0x140010402
14000ff18: mov rbx, qword ptr [rax]
14000ff1b: mov r15, qword ptr [rax + 8]
14000ff1f: lea rdx, [r12 + 0x18]
14000ff24: mov qword ptr [rbp + 0x3a8], rdx
14000ff2b: mov rcx, qword ptr [r12]
14000ff2f: mov rax, rcx
14000ff32: neg rax
14000ff35: jo 0x14001056c
14000ff3b: mov rax, qword ptr [r12 + 8]
14000ff40: mov qword ptr [rbp + 0x7a8], rax
14000ff47: cmp r15, -1
14000ff4b: mov qword ptr [rbp + 0x668], rcx
14000ff52: je 0x14001103d
14000ff58: mov qword ptr [rbp + 0x7a0], rdx
14000ff5f: lea rax, [r15 + 1]
14000ff63: mov rdi, rax
14000ff66: sub rdi, rbx
14000ff69: jb 0x140011055
14000ff6f: mov rdx, qword ptr [rbp + 0x470]
14000ff76: cmp r15, rdx
14000ff79: jae 0x140011073
14000ff7f: mov qword ptr [rbp + 0x7b8], rax
14000ff86: mov rax, rdi
14000ff89: mov ecx, 0xc
14000ff8e: mul rcx
14000ff91: jo 0x1400114d3
14000ff97: mov esi, 0
14000ff9c: movabs rcx, 0x7ffffffffffffffc
14000ffa6: cmp rax, rcx
14000ffa9: ja 0x1400114d5
14000ffaf: mov rcx, qword ptr [r12 + 0x10]
14000ffb4: mov qword ptr [rbp + 0x588], rcx
14000ffbb: mov r12, qword ptr [rbp + 0x468]
14000ffc2: test rax, rax
14000ffc5: je 0x140010000
14000ffc7: movzx ecx, byte ptr [rip + 0x3624c3b]
14000ffce: mov esi, 4
14000ffd3: mov edx, 4
14000ffd8: mov rcx, rax
14000ffdb: mov r14, rax
14000ffde: call 0x140613c10 ; ?
14000ffe3: mov qword ptr [rbp + 0x100], r14
14000ffea: test rax, rax
14000ffed: je 0x1400114d5
14000fff3: mov rsi, rdi
14000fff6: mov r14, rax
14000fff9: jmp 0x140010008 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14000fffb: nop dword ptr [rax + rax]
140010000: mov r14d, 4
140010006: xor esi, esi
140010008: lea rax, [rbx + rbx*2]
14001000c: lea rdx, [r12 + rax*4]
140010010: lea r12, [rdi*4]
140010018: lea r8, [r12 + r12*2]
14001001c: mov rcx, r14
14001001f: call 0x142923c00 ; ?
140010024: mov qword ptr [rbp + 0x3e0], rsi
14001002b: mov qword ptr [rbp + 0x100], r14
140010032: mov qword ptr [rbp + 0x3e8], r14
140010039: mov qword ptr [rbp + 0x3f0], rdi
140010040: mov byte ptr [rbp + 0x813], 1
140010047: lea rcx, [rbp + 0x590]
14001004e: lea rdx, [rbp + 0x3e0]
140010055: mov r8d, 1
14001005b: call 0x140153640 ; utils::curve::Curve::try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
140010060: xor eax, eax
140010062: cmp rax, qword ptr [rbp + 0x590]
140010069: mov r14, r13
14001006c: jno 0x1400100b0
14001006e: mov rsi, qword ptr [rbp + 0x7a0]
140010075: mov r12, rsi
140010078: mov rax, r14
14001007b: mov rdx, qword ptr [rbp + 0x668]
140010082: test rdx, rdx
140010085: lea rbx, [rbp + 0x590]
14001008c: je 0x14000fef3
140010092: shl rdx, 2
140010096: mov r8d, 4
14001009c: mov rcx, qword ptr [rbp + 0x7a8]
1400100a3: call 0x140613c20 ; ?
1400100a8: mov r12, rsi
1400100ab: jmp 0x14000fef0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400100b0: mov rax, qword ptr [rbp + 0x5c0]
1400100b7: mov qword ptr [rbp + 0x2d0], rax
1400100be: movups xmm0, xmmword ptr [rbp + 0x590]
1400100c5: movups xmm1, xmmword ptr [rbp + 0x5a0]
1400100cc: movups xmm2, xmmword ptr [rbp + 0x5b0]
1400100d3: movaps xmmword ptr [rbp + 0x2c0], xmm2
1400100da: movaps xmmword ptr [rbp + 0x2b0], xmm1
1400100e1: movaps xmmword ptr [rbp + 0x2a0], xmm0
1400100e8: mov rax, qword ptr [rbp + 0x258]
1400100ef: mov ecx, dword ptr [rbp + 0x424]
1400100f5: mov dword ptr [rax], ecx
1400100f7: mov rax, qword ptr [rbp + 0x2d0]
1400100fe: mov qword ptr [rbp + 0x410], rax
140010105: movaps xmm0, xmmword ptr [rbp + 0x2a0]
14001010c: movaps xmm1, xmmword ptr [rbp + 0x2b0]
140010113: movaps xmm2, xmmword ptr [rbp + 0x2c0]
14001011a: movaps xmmword ptr [rbp + 0x400], xmm2
140010121: movaps xmmword ptr [rbp + 0x3f0], xmm1
140010128: movaps xmmword ptr [rbp + 0x3e0], xmm0
14001012f: mov rdx, qword ptr [rbp + 0x4a8]
140010136: cmp r15, rdx
140010139: jae 0x140011281
14001013f: mov rax, rdi
140010142: shr rax, 0x3c
140010146: jne 0x1400116fa
14001014c: lea r13, [rbx*8]
140010154: add r13, qword ptr [rbp + 0x4a0]
14001015b: lea r8, [rdi*8]
140010163: test rdi, rdi
140010166: je 0x140010194
140010168: movzx eax, byte ptr [rip + 0x3624a9a]
14001016f: mov edx, 4
140010174: mov rsi, r8
140010177: mov rcx, r8
14001017a: call 0x140613c10 ; ?
14001017f: test rax, rax
140010182: je 0x14001185a
140010188: mov qword ptr [rbp + 0x430], rdi
14001018f: mov r8, rsi
140010192: jmp 0x1400101a4 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010194: mov eax, 4
140010199: mov qword ptr [rbp + 0x430], 0
1400101a4: mov qword ptr [rbp + 0x300], rax
1400101ab: mov rcx, rax
1400101ae: mov rdx, r13
1400101b1: call 0x142923c00 ; ?
1400101b6: mov rdx, qword ptr [rbp + 0x4c0]
1400101bd: cmp r15, rdx
1400101c0: jae 0x140011299
1400101c6: lea rsi, [rbx*4]
1400101ce: add rsi, qword ptr [rbp + 0x4b8]
1400101d5: test rdi, rdi
1400101d8: je 0x140010250
1400101da: mov r13, qword ptr [rbp + 0x430]
1400101e1: movzx eax, byte ptr [rip + 0x3624a21]
1400101e8: mov edx, 4
1400101ed: mov rcx, r12
1400101f0: call 0x140613c10 ; ?
1400101f5: test rax, rax
1400101f8: je 0x14001187e
1400101fe: mov qword ptr [rbp + 0x290], rax
140010205: mov rcx, rax
140010208: mov rdx, rsi
14001020b: mov r8, r12
14001020e: call 0x142923c00 ; ?
140010213: mov rdx, qword ptr [rbp + 0x4d8]
14001021a: cmp r15, rdx
14001021d: jae 0x140011381
140010223: mov rsi, qword ptr [rbp + 0x4d0]
14001022a: movzx eax, byte ptr [rip + 0x36249d8]
140010231: mov edx, 4
140010236: mov rcx, r12
140010239: call 0x140613c10 ; ?
14001023e: test rax, rax
140010241: je 0x140011897
140010247: mov qword ptr [rbp + 0x7b8], rdi
14001024e: jmp 0x140010295 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010250: mov r13, qword ptr [rbp + 0x430]
140010257: mov ecx, 4
14001025c: mov rdx, rsi
14001025f: mov r8, r12
140010262: call 0x142923c00 ; ?
140010267: mov rdx, qword ptr [rbp + 0x4d8]
14001026e: mov eax, 4
140010273: cmp r15, rdx
140010276: jae 0x140011378
14001027c: mov qword ptr [rbp + 0x290], rax
140010283: mov rsi, qword ptr [rbp + 0x4d0]
14001028a: mov qword ptr [rbp + 0x7b8], 0
140010295: lea rdx, [rsi + rbx*4]
140010299: mov rsi, rax
14001029c: mov rcx, rax
14001029f: mov r8, r12
1400102a2: call 0x142923c00 ; ?
1400102a7: xor eax, eax
1400102a9: mov ecx, 0
1400102ae: test rbx, rbx
1400102b1: jne 0x1400102b9
1400102b3: mov ecx, dword ptr [rbp + 0x4f8]
1400102b9: mov rdx, qword ptr [rbp + 0x470]
1400102c0: dec rdx
1400102c3: cmp r15, rdx
1400102c6: movzx edx, byte ptr [rbp + 0x4f9]
1400102cd: cmove eax, edx
1400102d0: mov rdx, qword ptr [rbp + 0x668]
1400102d7: mov qword ptr [rbp + 0x610], rdx
1400102de: mov rdx, qword ptr [rbp + 0x7a8]
1400102e5: mov qword ptr [rbp + 0x618], rdx
1400102ec: mov rdx, qword ptr [rbp + 0x588]
1400102f3: mov qword ptr [rbp + 0x620], rdx
1400102fa: mov rdx, qword ptr [rbp + 0x410]
140010301: mov qword ptr [rbp + 0x5c0], rdx
140010308: movaps xmm0, xmmword ptr [rbp + 0x3e0]
14001030f: movaps xmm1, xmmword ptr [rbp + 0x3f0]
140010316: movaps xmm2, xmmword ptr [rbp + 0x400]
14001031d: movaps xmmword ptr [rbp + 0x590], xmm0
140010324: movaps xmmword ptr [rbp + 0x5a0], xmm1
14001032b: movaps xmmword ptr [rbp + 0x5b0], xmm2
140010332: mov qword ptr [rbp + 0x5c8], r13
140010339: mov rdx, qword ptr [rbp + 0x300]
140010340: mov qword ptr [rbp + 0x5d0], rdx
140010347: mov qword ptr [rbp + 0x5d8], rdi
14001034e: mov rdx, qword ptr [rbp + 0x7b8]
140010355: mov qword ptr [rbp + 0x5e0], rdx
14001035c: mov qword ptr [rbp + 0x5e8], rsi
140010363: mov qword ptr [rbp + 0x5f0], rdi
14001036a: mov qword ptr [rbp + 0x5f8], rdx
140010371: mov rdx, qword ptr [rbp + 0x290]
140010378: mov qword ptr [rbp + 0x600], rdx
14001037f: mov qword ptr [rbp + 0x608], rdi
140010386: mov rdx, qword ptr [rbp + 0x758]
14001038d: mov dword ptr [rbp + 0x630], edx
140010393: mov rdx, qword ptr [rbp + 0x678]
14001039a: mov rdx, qword ptr [rdx]
14001039d: mov qword ptr [rbp + 0x628], rdx
1400103a4: mov byte ptr [rbp + 0x634], cl
1400103aa: mov byte ptr [rbp + 0x635], al
1400103b0: mov rdi, qword ptr [rbp + 0x348]
1400103b7: mov rsi, qword ptr [rdi + 0x10]
1400103bb: cmp rsi, qword ptr [rdi]
1400103be: lea rbx, [rbp + 0x590]
1400103c5: jne 0x1400103d6
1400103c7: mov rcx, rdi
1400103ca: lea rdx, [rip + 0x2916317]
1400103d1: call 0x1405636c0 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsBracketAssemblyParams,alloc::alloc::Global>
1400103d6: imul rcx, rsi, 0xa8
1400103dd: add rcx, qword ptr [rdi + 8]
1400103e1: mov r8d, 0xa8
1400103e7: mov rdx, rbx
1400103ea: call 0x142923c10 ; ?
1400103ef: inc rsi
1400103f2: mov qword ptr [rdi + 0x10], rsi
1400103f6: mov r12, qword ptr [rbp + 0x7a0]
1400103fd: jmp 0x14000fef0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010402: mov r12, qword ptr [rbp + 0x798]
140010409: mov rdx, qword ptr [rbp + 0x560]
140010410: test rdx, rdx
140010413: mov r13, qword ptr [rbp + 0x940]
14001041a: je 0x140010432
14001041c: shl rdx, 4
140010420: mov r8d, 8
140010426: mov rcx, qword ptr [rbp + 0x530]
14001042d: call 0x140613c20 ; ?
140010432: mov r8, qword ptr [rbp + 0x798]
140010439: mov rax, r8
14001043c: sub rax, r12
14001043f: movabs rcx, 0xaaaaaaaaaaaaaaab
140010449: mul rcx
14001044c: cmp r8, r12
14001044f: je 0x140010488
140010451: mov rdi, rdx
140010454: shr rdi, 4
140010458: add r12, 8
14001045c: jmp 0x140010469 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001045e: nop 
140010460: add r12, 0x18
140010464: dec rdi
140010467: je 0x140010488
140010469: mov rdx, qword ptr [r12 - 8]
14001046e: test rdx, rdx
140010471: je 0x140010460
140010473: mov rcx, qword ptr [r12]
140010477: shl rdx, 2
14001047b: mov r8d, 4
140010481: call 0x140613c20 ; ?
140010486: jmp 0x140010460 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010488: mov rax, qword ptr [rbp + 0x580]
14001048f: test rax, rax
140010492: je 0x1400104ae
140010494: shl rax, 3
140010498: lea rdx, [rax + rax*2]
14001049c: mov r8d, 8
1400104a2: mov rcx, qword ptr [rbp + 0x568]
1400104a9: call 0x140613c20 ; ?
1400104ae: mov rax, qword ptr [rbp + 0x460]
1400104b5: test rax, rax
1400104b8: lea r12, [rbp + 0x460]
1400104bf: je 0x1400104db
1400104c1: mov rcx, qword ptr [rbp + 0x468]
1400104c8: shl rax, 2
1400104cc: lea rdx, [rax + rax*2]
1400104d0: mov r8d, 4
1400104d6: call 0x140613c20 ; ?
1400104db: mov rdx, qword ptr [rbp + 0x478]
1400104e2: test rdx, rdx
1400104e5: je 0x1400104fd
1400104e7: mov rcx, qword ptr [rbp + 0x480]
1400104ee: shl rdx, 2
1400104f2: mov r8d, 4
1400104f8: call 0x140613c20 ; ?
1400104fd: mov rdx, qword ptr [rbp + 0x498]
140010504: test rdx, rdx
140010507: je 0x14001051f
140010509: mov rcx, qword ptr [rbp + 0x4a0]
140010510: shl rdx, 3
140010514: mov r8d, 4
14001051a: call 0x140613c20 ; ?
14001051f: mov rdx, qword ptr [rbp + 0x4b0]
140010526: test rdx, rdx
140010529: je 0x140010541
14001052b: mov rcx, qword ptr [rbp + 0x4b8]
140010532: shl rdx, 2
140010536: mov r8d, 4
14001053c: call 0x140613c20 ; ?
140010541: mov rdx, qword ptr [rbp + 0x4c8]
140010548: test rdx, rdx
14001054b: je 0x14000ecf0
140010551: mov rcx, qword ptr [rbp + 0x4d0]
140010558: shl rdx, 2
14001055c: mov r8d, 4
140010562: call 0x140613c20 ; ?
140010567: jmp 0x14000ecf0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001056c: mov r12, rdx
14001056f: lea rbx, [rbp + 0x590]
140010576: mov rdx, qword ptr [rbp + 0x560]
14001057d: test rdx, rdx
140010580: mov r13, qword ptr [rbp + 0x940]
140010587: jne 0x14001041c
14001058d: jmp 0x140010432 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010592: lea rcx, [rbp + 0x440]
140010599: call 0x1405dec70 ; core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global> >
14001059e: mov rdi, qword ptr [rbp + 0x88]
1400105a5: xor r14d, r14d
1400105a8: mov rsi, qword ptr [rbp + 0x160]
1400105af: add rsi, 0x18
1400105b3: mov rcx, rsi
1400105b6: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
1400105bb: mov al, 1
1400105bd: lock cmpxchg byte ptr [rsi], r14b
1400105c2: jne 0x1400105ec
1400105c4: mov rsi, qword ptr [rbp + 0x1b8]
1400105cb: add rsi, 0x18
1400105cf: mov rcx, rsi
1400105d2: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
1400105d7: mov al, 1
1400105d9: lock cmpxchg byte ptr [rsi], r14b
1400105de: jne 0x1400105f8
1400105e0: mov byte ptr [rbp + 0x648], 1
1400105e7: jmp 0x140009330 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400105ec: mov rcx, rsi
1400105ef: xor edx, edx
1400105f1: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
1400105f6: jmp 0x1400105c4 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400105f8: mov rcx, rsi
1400105fb: xor edx, edx
1400105fd: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
140010602: jmp 0x1400105e0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010604: mov rax, qword ptr [rbp + 0x370]
14001060b: mov rsi, qword ptr [rax + 8]
14001060f: mov rax, qword ptr [rax + 0x10]
140010613: lea rdi, [rax + rax*4]
140010617: shl rdi, 5
14001061b: add rdi, rsi
14001061e: add qword ptr [rbp + 0x7c0], 0x168
140010629: nop dword ptr [rax]
140010630: cmp rsi, rdi
140010633: je 0x140010661
140010635: mov ecx, dword ptr [rsi + 0x88]
14001063b: mov edx, dword ptr [rsi + 0x8c]
140010641: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
140010646: add rsi, 0xa0
14001064d: mov rcx, qword ptr [rbp + 0x7c0]
140010654: mov rdx, rax
140010657: mov r8b, 1
14001065a: call 0x1402c8dc0 ; hashbrown::map::HashMap::insert<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,country_core::resources::stairs::stairs_assembly_params::StairArchWallSupportType,std::hash::random::RandomState,alloc::alloc::Global>
14001065f: jmp 0x140010630 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010661: mov rax, qword ptr [rbp + 0x348]
140010668: mov rsi, qword ptr [rax + 8]
14001066c: imul rdi, qword ptr [rax + 0x10], 0xa8
140010674: add rdi, rsi
140010677: nop word ptr [rax + rax]
140010680: cmp rsi, rdi
140010683: je 0x1400106b1
140010685: mov ecx, dword ptr [rsi + 0x98]
14001068b: mov edx, dword ptr [rsi + 0x9c]
140010691: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
140010696: add rsi, 0xa8
14001069d: mov rcx, qword ptr [rbp + 0x7c0]
1400106a4: mov rdx, rax
1400106a7: xor r8d, r8d
1400106aa: call 0x1402c8dc0 ; hashbrown::map::HashMap::insert<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,country_core::resources::stairs::stairs_assembly_params::StairArchWallSupportType,std::hash::random::RandomState,alloc::alloc::Global>
1400106af: jmp 0x140010680 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400106b1: mov rax, qword ptr [rbp + 0x348]
1400106b8: mov r12, qword ptr [rax + 8]
1400106bc: imul r14, qword ptr [rax + 0x10], 0xa8
1400106c4: add r14, r12
1400106c7: lea rsi, [rbp + 0x460]
1400106ce: lea rdi, [rbp + 0x6a0]
1400106d5: lea rbx, [rbp + 0x380]
1400106dc: xor r15d, r15d
1400106df: cmp r12, r14
1400106e2: jne 0x140010700
1400106e4: jmp 0x140010c35 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400106e9: nop dword ptr [rax]
1400106f0: add r12, 0xa8
1400106f7: cmp r12, r14
1400106fa: je 0x140010c35
140010700: lea rax, [r12 + 0x68]
140010705: lea rcx, [r12 + 0x38]
14001070a: mov rdx, qword ptr [rbp + 0x950]
140010711: mov qword ptr [rbp + 0x460], rdx
140010718: mov rdx, qword ptr [rbp + 0x7c0]
14001071f: mov qword ptr [rbp + 0x468], rdx
140010726: mov rdx, qword ptr [rbp + 0x248]
14001072d: mov qword ptr [rbp + 0x470], rdx
140010734: mov rdx, qword ptr [rbp + 0x2e0]
14001073b: mov qword ptr [rbp + 0x478], rdx
140010742: mov qword ptr [rbp + 0x380], r12
140010749: mov qword ptr [rbp + 0x388], rax
140010750: mov qword ptr [rbp + 0x390], rcx
140010757: cmp byte ptr [r12 + 0xa4], 0
140010760: je 0x1400109c0
140010766: mov edx, dword ptr [r12 + 0x98]
14001076e: mov r8d, dword ptr [r12 + 0x9c]
140010776: mov rcx, rsi
140010779: mov r9b, 1
14001077c: call 0x1400286d0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$24<false>
140010781: test al, al
140010783: je 0x1400109c0
140010789: mov rcx, rdi
14001078c: mov rdx, rbx
14001078f: mov r8b, 1
140010792: call 0x140027840 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$25<false>
140010797: cmp r15, qword ptr [rbp + 0x6a0]
14001079e: jo 0x1400109c0
1400107a4: mov rax, qword ptr [rbp + 0x6f0]
1400107ab: mov qword ptr [rbp + 0x800], rax
1400107b2: mov rax, qword ptr [rbp + 0x6f8]
1400107b9: mov qword ptr [rbp + 0x760], rax
1400107c0: mov rdx, qword ptr [rbp + 0x700]
1400107c7: mov rax, qword ptr [rbp + 0x340]
1400107ce: mov ecx, dword ptr [rbp + 0x53c]
1400107d4: mov dword ptr [rax], ecx
1400107d6: mov rax, qword ptr [rbp + 0x6d0]
1400107dd: mov qword ptr [rbp + 0x5c0], rax
1400107e4: movups xmm0, xmmword ptr [rbp + 0x6a0]
1400107eb: movdqu xmm1, xmmword ptr [rbp + 0x6b0]
1400107f3: movdqu xmm2, xmmword ptr [rbp + 0x6c0]
1400107fb: movdqa xmmword ptr [rbp + 0x5b0], xmm2
140010803: movdqa xmmword ptr [rbp + 0x5a0], xmm1
14001080b: movaps xmmword ptr [rbp + 0x590], xmm0
140010812: mov rax, qword ptr [rbp + 0x6d8]
140010819: mov qword ptr [rbp + 0x780], rax
140010820: mov rax, qword ptr [rbp + 0x6e0]
140010827: mov qword ptr [rbp + 0x7b0], rax
14001082e: mov r15, qword ptr [rbp + 0x6e8]
140010835: test rdx, rdx
140010838: mov qword ptr [rbp + 0x7f0], rdx
14001083f: je 0x140010867
140010841: lea r13, [rdx*4]
140010849: movzx eax, byte ptr [rip + 0x36243b9]
140010850: mov edx, 4
140010855: mov rcx, r13
140010858: call 0x140613c40 ; ?
14001085d: test rax, rax
140010860: jne 0x14001086c
140010862: jmp 0x14001152a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010867: mov eax, 4
14001086c: mov qword ptr [rbp + 0x778], rax
140010873: mov ecx, dword ptr [r12 + 0x98]
14001087b: mov edx, dword ptr [r12 + 0x9c]
140010883: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
140010888: mov ecx, dword ptr [r12 + 0xa0]
140010890: mov rdx, qword ptr [rbp + 0x5c0]
140010897: mov qword ptr [rbp + 0x170], rdx
14001089e: movaps xmm0, xmmword ptr [rbp + 0x590]
1400108a5: movdqa xmm1, xmmword ptr [rbp + 0x5a0]
1400108ad: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
1400108b5: movdqa xmmword ptr [rbp + 0x160], xmm2
1400108bd: movdqa xmmword ptr [rbp + 0x150], xmm1
1400108c5: movaps xmmword ptr [rbp + 0x140], xmm0
1400108cc: mov rdx, qword ptr [rbp + 0x780]
1400108d3: mov qword ptr [rbp + 0x178], rdx
1400108da: mov rdx, qword ptr [rbp + 0x7b0]
1400108e1: mov qword ptr [rbp + 0x180], rdx
1400108e8: mov qword ptr [rbp + 0x188], r15
1400108ef: mov rdx, qword ptr [rbp + 0x800]
1400108f6: mov qword ptr [rbp + 0x190], rdx
1400108fd: mov rdx, qword ptr [rbp + 0x760]
140010904: mov qword ptr [rbp + 0x198], rdx
14001090b: mov rdx, qword ptr [rbp + 0x7f0]
140010912: mov qword ptr [rbp + 0x1a0], rdx
140010919: mov qword ptr [rbp + 0x1a8], rdx
140010920: mov r8, qword ptr [rbp + 0x778]
140010927: mov qword ptr [rbp + 0x1b0], r8
14001092e: mov qword ptr [rbp + 0x1b8], rdx
140010935: mov qword ptr [rbp + 0x1c0], rax
14001093c: mov word ptr [rbp + 0x1d4], 0
140010945: mov dword ptr [rbp + 0x1d0], ecx
14001094b: mov rax, qword ptr [r12 + 0x98]
140010953: mov qword ptr [rbp + 0x1c8], rax
14001095a: mov dword ptr [rbp + 0x1d6], 0
140010964: mov word ptr [rbp + 0x1da], 0x100
14001096d: mov r13, qword ptr [rbp + 0x370]
140010974: mov r15, qword ptr [r13 + 0x10]
140010978: cmp r15, qword ptr [r13]
14001097c: jne 0x14001098d
14001097e: mov rcx, r13
140010981: lea rdx, [rip + 0x2915ba8]
140010988: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
14001098d: lea rcx, [r15 + r15*4]
140010991: shl rcx, 5
140010995: add rcx, qword ptr [r13 + 8]
140010999: mov r8d, 0xa0
14001099f: lea rdx, [rbp + 0x140]
1400109a6: call 0x142923c10 ; ?
1400109ab: inc r15
1400109ae: mov qword ptr [r13 + 0x10], r15
1400109b2: mov r13, qword ptr [rbp + 0x940]
1400109b9: xor r15d, r15d
1400109bc: nop dword ptr [rax]
1400109c0: cmp byte ptr [r12 + 0xa5], 0
1400109c9: je 0x1400106f0
1400109cf: mov r8d, dword ptr [r12 + 0x98]
1400109d7: mov edx, dword ptr [r12 + 0x9c]
1400109df: mov rcx, rsi
1400109e2: xor r9d, r9d
1400109e5: call 0x1400286d0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$24<false>
1400109ea: test al, al
1400109ec: je 0x1400106f0
1400109f2: mov rcx, rdi
1400109f5: mov rdx, rbx
1400109f8: xor r8d, r8d
1400109fb: call 0x140027840 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure$25<false>
140010a00: cmp r15, qword ptr [rbp + 0x6a0]
140010a07: jo 0x1400106f0
140010a0d: mov rax, qword ptr [rbp + 0x6f0]
140010a14: mov qword ptr [rbp + 0x800], rax
140010a1b: mov rax, qword ptr [rbp + 0x6f8]
140010a22: mov qword ptr [rbp + 0x760], rax
140010a29: mov rdx, qword ptr [rbp + 0x700]
140010a30: mov rax, qword ptr [rbp + 0x340]
140010a37: mov ecx, dword ptr [rbp + 0x53c]
140010a3d: mov dword ptr [rax], ecx
140010a3f: mov rax, qword ptr [rbp + 0x6d0]
140010a46: mov qword ptr [rbp + 0x5c0], rax
140010a4d: movups xmm0, xmmword ptr [rbp + 0x6a0]
140010a54: movdqu xmm1, xmmword ptr [rbp + 0x6b0]
140010a5c: movdqu xmm2, xmmword ptr [rbp + 0x6c0]
140010a64: movdqa xmmword ptr [rbp + 0x5b0], xmm2
140010a6c: movdqa xmmword ptr [rbp + 0x5a0], xmm1
140010a74: movaps xmmword ptr [rbp + 0x590], xmm0
140010a7b: mov rax, qword ptr [rbp + 0x6d8]
140010a82: mov qword ptr [rbp + 0x780], rax
140010a89: mov rax, qword ptr [rbp + 0x6e0]
140010a90: mov qword ptr [rbp + 0x7b0], rax
140010a97: mov r15, qword ptr [rbp + 0x6e8]
140010a9e: test rdx, rdx
140010aa1: mov qword ptr [rbp + 0x7f0], rdx
140010aa8: je 0x140010ad0
140010aaa: lea r13, [rdx*4]
140010ab2: movzx eax, byte ptr [rip + 0x3624150]
140010ab9: mov edx, 4
140010abe: mov rcx, r13
140010ac1: call 0x140613c40 ; ?
140010ac6: test rax, rax
140010ac9: jne 0x140010ad5
140010acb: jmp 0x14001154a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010ad0: mov eax, 4
140010ad5: mov qword ptr [rbp + 0x778], rax
140010adc: mov ecx, dword ptr [r12 + 0x98]
140010ae4: mov edx, dword ptr [r12 + 0x9c]
140010aec: call 0x140959e70 ; country_core::resources::stairs::stairs_assembly_params::impl$2::from
140010af1: mov ecx, dword ptr [r12 + 0xa0]
140010af9: mov rdx, qword ptr [rbp + 0x5c0]
140010b00: mov qword ptr [rbp + 0x170], rdx
140010b07: movaps xmm0, xmmword ptr [rbp + 0x590]
140010b0e: movdqa xmm1, xmmword ptr [rbp + 0x5a0]
140010b16: movdqa xmm2, xmmword ptr [rbp + 0x5b0]
140010b1e: movdqa xmmword ptr [rbp + 0x160], xmm2
140010b26: movdqa xmmword ptr [rbp + 0x150], xmm1
140010b2e: movaps xmmword ptr [rbp + 0x140], xmm0
140010b35: mov rdx, qword ptr [rbp + 0x780]
140010b3c: mov qword ptr [rbp + 0x178], rdx
140010b43: mov rdx, qword ptr [rbp + 0x7b0]
140010b4a: mov qword ptr [rbp + 0x180], rdx
140010b51: mov qword ptr [rbp + 0x188], r15
140010b58: mov rdx, qword ptr [rbp + 0x800]
140010b5f: mov qword ptr [rbp + 0x190], rdx
140010b66: mov rdx, qword ptr [rbp + 0x760]
140010b6d: mov qword ptr [rbp + 0x198], rdx
140010b74: mov rdx, qword ptr [rbp + 0x7f0]
140010b7b: mov qword ptr [rbp + 0x1a0], rdx
140010b82: mov qword ptr [rbp + 0x1a8], rdx
140010b89: mov r8, qword ptr [rbp + 0x778]
140010b90: mov qword ptr [rbp + 0x1b0], r8
140010b97: mov qword ptr [rbp + 0x1b8], rdx
140010b9e: mov qword ptr [rbp + 0x1c0], rax
140010ba5: mov word ptr [rbp + 0x1d4], 0
140010bae: mov dword ptr [rbp + 0x1d0], ecx
140010bb4: mov rax, qword ptr [r12 + 0x98]
140010bbc: mov qword ptr [rbp + 0x1c8], rax
140010bc3: mov dword ptr [rbp + 0x1d6], 0
140010bcd: mov word ptr [rbp + 0x1da], 0x200
140010bd6: mov r13, qword ptr [rbp + 0x370]
140010bdd: mov r15, qword ptr [r13 + 0x10]
140010be1: cmp r15, qword ptr [r13]
140010be5: jne 0x140010bf6
140010be7: mov rcx, r13
140010bea: lea rdx, [rip + 0x291596f]
140010bf1: call 0x140563480 ; alloc::raw_vec::RawVec::grow_one<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams,alloc::alloc::Global>
140010bf6: lea rcx, [r15 + r15*4]
140010bfa: shl rcx, 5
140010bfe: add rcx, qword ptr [r13 + 8]
140010c02: mov r8d, 0xa0
140010c08: lea rdx, [rbp + 0x140]
140010c0f: call 0x142923c10 ; ?
140010c14: inc r15
140010c17: mov qword ptr [r13 + 0x10], r15
140010c1b: mov r13, qword ptr [rbp + 0x940]
140010c22: xor r15d, r15d
140010c25: add r12, 0xa8
140010c2c: cmp r12, r14
140010c2f: jne 0x140010700
140010c35: test byte ptr [rbp + 0x648], 1
140010c3c: je 0x140010daf
140010c42: mov r8, qword ptr [rbp + 0x2f0]
140010c49: mov rax, qword ptr [r8]
140010c4c: mov rcx, qword ptr [r8 + 8]
140010c50: add rcx, rax
140010c53: inc rcx
140010c56: movdqa xmm0, xmmword ptr [rax]
140010c5a: pmovmskb edx, xmm0
140010c5e: not edx
140010c60: mov r8, qword ptr [r8 + 0x18]
140010c64: mov qword ptr [rbp + 0x140], 0
140010c6f: mov qword ptr [rbp + 0x150], 0
140010c7a: mov qword ptr [rbp + 0x160], rax
140010c81: add rax, 0x10
140010c85: mov qword ptr [rbp + 0x168], rax
140010c8c: mov qword ptr [rbp + 0x170], rcx
140010c93: mov word ptr [rbp + 0x178], dx
140010c9a: mov qword ptr [rbp + 0x180], r8
140010ca1: lea r8, [rip + 0x2916688]
140010ca8: lea rcx, [rbp + 0x590]
140010caf: lea rdx, [rbp + 0x140]
140010cb6: call 0x1402bcd50 ; alloc::vec::spec_from_iter::impl$0::from_iter<glam::f32::sse2::vec4::Vec4,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::visual::StairsEdgeVisualState>,core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::sse2::vec4::Vec4> >,system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::closure_env$26<false> > >
140010cbb: mov rax, qword ptr [rbp + 0x598]
140010cc2: mov qword ptr [rbp + 0x800], rax
140010cc9: mov r14, qword ptr [rbp + 0x5a0]
140010cd0: test r14, r14
140010cd3: je 0x140010dfb
140010cd9: mov rcx, qword ptr [rbp + 0x268]
140010ce0: call 0x140987ca0 ; country_core::render::impl$0::deref
140010ce5: mov rsi, rax
140010ce8: mov rdi, r14
140010ceb: shl rdi, 4
140010cef: lea rbx, [rbp + 0x6a0]
140010cf6: mov rcx, rbx
140010cf9: mov rdx, rdi
140010cfc: mov r8d, 0x20
140010d02: call 0x140bd4e20 ; rhapsody::raw::buffer::BufferDesc::new_gpu_only
140010d07: mov qword ptr [rsp + 0x30], rdi
140010d0c: mov rax, qword ptr [rbp + 0x800]
140010d13: mov qword ptr [rsp + 0x28], rax
140010d18: mov qword ptr [rsp + 0x20], rbx
140010d1d: lea r8, [rip + 0x29157cc]
140010d24: lea rcx, [rbp + 0x140]
140010d2b: mov r9d, 0xf
140010d31: mov rdx, rsi
140010d34: call 0x1405b93b0 ; rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
140010d39: cmp dword ptr [rbp + 0x140], 0xb
140010d40: jne 0x140010e35
140010d46: mov rax, qword ptr [rbp + 0x168]
140010d4d: mov qword ptr [rbp + 0x6c0], rax
140010d54: movdqu xmm0, xmmword ptr [rbp + 0x148]
140010d5c: movdqu xmm1, xmmword ptr [rbp + 0x158]
140010d64: movdqa xmmword ptr [rbp + 0x6b0], xmm1
140010d6c: movdqa xmmword ptr [rbp + 0x6a0], xmm0
140010d74: mov dword ptr [rbp + 0x6c8], r14d
140010d7b: lea r8, [rip + 0x291577e]
140010d82: lea rdx, [rbp + 0x6a0]
140010d89: mov rcx, r13
140010d8c: call 0x14014dd30 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>
140010d91: cmp dword ptr [rbp + 0x140], 0xb
140010d98: je 0x140010e41
140010d9e: lea rcx, [rbp + 0x140]
140010da5: call 0x1400297d0 ; core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > > >
140010daa: jmp 0x140010e41 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010daf: mov rax, qword ptr [rbp + 0x2f0]
140010db6: cmp qword ptr [rax + 0x18], 0
140010dbb: jne 0x140010e63
140010dc1: mov rsi, qword ptr [r13 + 8]
140010dc5: test rsi, rsi
140010dc8: jne 0x140010dce
140010dca: mov rsi, qword ptr [r13 + 0x10]
140010dce: mov rax, qword ptr [rsi]
140010dd1: mov rdi, qword ptr [rsi + 0x10]
140010dd5: sub rax, rdi
140010dd8: cmp rax, 7
140010ddc: jbe 0x140011237
140010de2: mov rax, qword ptr [rsi + 8]
140010de6: lea rcx, [rip + 0x13c7d3]
140010ded: mov qword ptr [rax + rdi], rcx
140010df1: add rdi, 8
140010df5: mov qword ptr [rsi + 0x10], rdi
140010df9: jmp 0x140010e63 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010dfb: mov rsi, qword ptr [r13 + 8]
140010dff: test rsi, rsi
140010e02: jne 0x140010e08
140010e04: mov rsi, qword ptr [r13 + 0x10]
140010e08: mov rax, qword ptr [rsi]
140010e0b: mov rdi, qword ptr [rsi + 0x10]
140010e0f: sub rax, rdi
140010e12: cmp rax, 7
140010e16: jbe 0x14001125c
140010e1c: mov rax, qword ptr [rsi + 8]
140010e20: lea rcx, [rip + 0x13c799]
140010e27: mov qword ptr [rax + rdi], rcx
140010e2b: add rdi, 8
140010e2f: mov qword ptr [rsi + 0x10], rdi
140010e33: jmp 0x140010e41 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010e35: lea rcx, [rbp + 0x140]
140010e3c: call 0x1400297d0 ; core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > > >
140010e41: mov rdx, qword ptr [rbp + 0x590]
140010e48: test rdx, rdx
140010e4b: je 0x140010e63
140010e4d: shl rdx, 4
140010e51: mov r8d, 0x10
140010e57: mov rcx, qword ptr [rbp + 0x800]
140010e5e: call 0x140613c20 ; ?
140010e63: cmp byte ptr [rbp + 0x797], 0
140010e6a: je 0x140010eb7
140010e6c: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
140010e71: mov rcx, rax
140010e74: mov rax, qword ptr [rax]
140010e77: cmp rax, 1
140010e7b: jne 0x140010fb8
140010e81: add rcx, 8
140010e85: mov rdx, qword ptr [rbp + 0x270]
140010e8c: cmp qword ptr [rcx], 0
140010e90: jne 0x1400111fb
140010e96: mov qword ptr [rcx], 0xffffffffffffffff
140010e9d: mov qword ptr [rbp + 0x800], rcx
140010ea4: add rcx, 8
140010ea8: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
140010ead: mov rax, qword ptr [rbp + 0x800]
140010eb4: inc qword ptr [rax]
140010eb7: movaps xmm6, xmmword ptr [rbp + 0x820]
140010ebe: movaps xmm7, xmmword ptr [rbp + 0x830]
140010ec5: movaps xmm8, xmmword ptr [rbp + 0x840]
140010ecd: movaps xmm9, xmmword ptr [rbp + 0x850]
140010ed5: movaps xmm10, xmmword ptr [rbp + 0x860]
140010edd: movaps xmm11, xmmword ptr [rbp + 0x870]
140010ee5: movaps xmm12, xmmword ptr [rbp + 0x880]
140010eed: movaps xmm13, xmmword ptr [rbp + 0x890]
140010ef5: movaps xmm14, xmmword ptr [rbp + 0x8a0]
140010efd: movaps xmm15, xmmword ptr [rbp + 0x8b0]
140010f05: add rsp, 0x948
140010f0c: pop rbx
140010f0d: pop rdi
140010f0e: pop rsi
140010f0f: pop r12
140010f11: pop r13
140010f13: pop r14
140010f15: pop r15
140010f17: pop rbp
140010f18: ret 
140010f19: mov byte ptr [rbp + 0x817], 1
140010f20: lea rcx, [rip + 0x2915881]
140010f27: call 0x1428d98a0 ; core::slice::index::slice_end_index_overflow_fail
140010f2c: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010f31: mov byte ptr [rbp + 0x817], 1
140010f38: lea r8, [rip + 0x2915869]
140010f3f: mov rdx, rcx
140010f42: mov rcx, r15
140010f45: call 0x1428d9850 ; core::slice::index::slice_index_order_fail
140010f4a: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010f4f: mov byte ptr [rbp + 0x817], 1
140010f56: lea r8, [rip + 0x291584b]
140010f5d: mov rdx, qword ptr [rbp + 0x570]
140010f64: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140010f69: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010f6e: mov byte ptr [rbp + 0x817], 1
140010f75: lea r8, [rip + 0x2915844]
140010f7c: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140010f81: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010f86: mov byte ptr [rbp + 0x817], 1
140010f8d: lea r8, [rip + 0x2915844]
140010f94: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140010f99: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010f9e: test rax, rax
140010fa1: jne 0x140010fdd
140010fa3: xor edx, edx
140010fa5: call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
140010faa: mov rcx, rax
140010fad: test rax, rax
140010fb0: jne 0x140008fb8
140010fb6: jmp 0x140010fdd ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140010fb8: test rax, rax
140010fbb: mov rax, qword ptr [rbp + 0x270]
140010fc2: jne 0x140010fdd
140010fc4: mov rsi, rax
140010fc7: xor edx, edx
140010fc9: call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
140010fce: mov rdx, rsi
140010fd1: mov rcx, rax
140010fd4: test rax, rax
140010fd7: jne 0x140010e8c
140010fdd: lea rcx, [rip + 0x2921a2c]
140010fe4: call 0x1428d80d0 ; std::thread::local::panic_access_error
140010fe9: mov byte ptr [rbp + 0x817], 1
140010ff0: lea r8, [rip + 0x29157f9]
140010ff7: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140010ffc: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011001: lea rcx, [rip + 0x29156f8]
140011008: call 0x1428d98a0 ; core::slice::index::slice_end_index_overflow_fail
14001100d: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011012: lea r8, [rip + 0x29156e7]
140011019: mov rcx, r13
14001101c: mov rdx, rax
14001101f: call 0x1428d9850 ; core::slice::index::slice_index_order_fail
140011024: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011029: lea r8, [rip + 0x29156d0]
140011030: mov rcx, rax
140011033: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140011038: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001103d: mov byte ptr [rbp + 0x813], 1
140011044: lea rcx, [rip + 0x291563d]
14001104b: call 0x1428d98a0 ; core::slice::index::slice_end_index_overflow_fail
140011050: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011055: mov byte ptr [rbp + 0x813], 1
14001105c: lea r8, [rip + 0x2915625]
140011063: mov rcx, rbx
140011066: mov rdx, rax
140011069: call 0x1428d9850 ; core::slice::index::slice_index_order_fail
14001106e: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011073: mov byte ptr [rbp + 0x813], 1
14001107a: lea r8, [rip + 0x2915607]
140011081: mov rcx, rax
140011084: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140011089: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001108e: mov byte ptr [rbp + 0x816], 1
140011095: mov byte ptr [rbp + 0x815], 1
14001109c: lea rcx, [rip + 0x29159a5]
1400110a3: call 0x1428d98a0 ; core::slice::index::slice_end_index_overflow_fail
1400110a8: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400110ad: mov byte ptr [rbp + 0x816], 1
1400110b4: mov byte ptr [rbp + 0x815], 1
1400110bb: lea r8, [rip + 0x2915986]
1400110c2: mov rcx, qword ptr [rbp + 0x800]
1400110c9: mov rdx, r9
1400110cc: call 0x1428d9850 ; core::slice::index::slice_index_order_fail
1400110d1: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400110d6: mov byte ptr [rbp + 0x816], 1
1400110dd: mov byte ptr [rbp + 0x815], 1
1400110e4: lea r8, [rip + 0x291595d]
1400110eb: mov rcx, r9
1400110ee: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400110f3: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400110f8: mov qword ptr [rbp + 0x6a0], 0
140011103: mov byte ptr [rbp + 0x817], 1
14001110a: lea rax, [rip + 0x29156ff]
140011111: mov qword ptr [rsp + 0x20], rax
140011116: lea r8, [rip + 0x29156eb]
14001111d: lea rdx, [rbp + 0x78]
140011121: lea r9, [rbp + 0x6a0]
140011128: xor ecx, ecx
14001112a: call 0x1428d9596 ; core::panicking::assert_failed<usize,usize>
14001112f: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011134: mov qword ptr [rbp + 0x140], 0
14001113f: lea rax, [rip + 0x299d052]
140011146: mov qword ptr [rsp + 0x20], rax
14001114b: lea rdx, [rbp + 0x6a0]
140011152: lea r9, [rbp + 0x140]
140011159: xor ecx, ecx
14001115b: mov r8, r14
14001115e: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
140011163: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011168: mov qword ptr [rbp + 0x140], 0
140011173: lea rax, [rip + 0x299d01e]
14001117a: mov qword ptr [rsp + 0x20], rax
14001117f: lea rdx, [rbp + 0x6a0]
140011186: lea r9, [rbp + 0x140]
14001118d: xor ecx, ecx
14001118f: mov r8, r14
140011192: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
140011197: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001119c: mov qword ptr [rbp + 0x140], 0
1400111a7: lea rax, [rip + 0x299cfea]
1400111ae: mov qword ptr [rsp + 0x20], rax
1400111b3: lea rdx, [rbp + 0x6a0]
1400111ba: lea r9, [rbp + 0x140]
1400111c1: xor ecx, ecx
1400111c3: mov r8, rbx
1400111c6: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
1400111cb: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400111d0: lea r8, [rip + 0x29151b9]
1400111d7: mov rcx, rax
1400111da: mov rdx, rdi
1400111dd: call 0x1428d9518 ; core::panicking::panic_bounds_check
1400111e2: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400111e7: lea r8, [rip + 0x29151a2]
1400111ee: mov rdx, rdi
1400111f1: call 0x1428d9518 ; core::panicking::panic_bounds_check
1400111f6: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400111fb: lea rcx, [rip + 0x29218b6]
140011202: call 0x1428d9250 ; core::cell::panic_already_borrowed
140011207: lea r8, [rip + 0x291550a]
14001120e: mov rcx, qword ptr [rbp + 0x7a0]
140011215: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
14001121a: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001121f: lea r8, [rip + 0x291550a]
140011226: mov rcx, qword ptr [rbp + 0x7a0]
14001122d: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140011232: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011237: mov qword ptr [rsp + 0x20], 1
140011240: mov r8d, 8
140011246: mov r9d, 1
14001124c: mov rcx, rsi
14001124f: mov rdx, rdi
140011252: call 0x1428a29e0 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140011257: jmp 0x140010de2 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001125c: mov qword ptr [rsp + 0x20], 1
140011265: mov r8d, 8
14001126b: mov r9d, 1
140011271: mov rcx, rsi
140011274: mov rdx, rdi
140011277: call 0x1428a29e0 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
14001127c: jmp 0x140010e1c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011281: lea r8, [rip + 0x2915418]
140011288: mov rcx, qword ptr [rbp + 0x7b8]
14001128f: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140011294: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011299: lea r8, [rip + 0x2915418]
1400112a0: mov rcx, qword ptr [rbp + 0x7b8]
1400112a7: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400112ac: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400112b1: lea r8, [rip + 0x29157a8]
1400112b8: mov rcx, qword ptr [rbp + 0x760]
1400112bf: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400112c4: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400112c9: lea r8, [rip + 0x29157a8]
1400112d0: mov rcx, qword ptr [rbp + 0x760]
1400112d7: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400112dc: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400112e1: lea r8, [rip + 0x29157a8]
1400112e8: mov rcx, qword ptr [rbp + 0x760]
1400112ef: mov rdx, rbx
1400112f2: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400112f7: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400112fc: mov byte ptr [rbp + 0x7ee], 1
140011303: mov byte ptr [rbp + 0x7ed], 1
14001130a: mov byte ptr [rbp + 0x7ec], 1
140011311: mov byte ptr [rbp + 0x7eb], 0
140011318: mov byte ptr [rbp + 0x7ea], 0
14001131f: lea rcx, [rip + 0x29152ba]
140011326: call 0x1428d9310 ; core::option::unwrap_failed
14001132b: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011330: xor edi, edi
140011332: mov qword ptr [rbp + 0xd0], rbx
140011339: mov qword ptr [rbp + 0xc8], rdi
140011340: lea r8, [rip + 0x2915401]
140011347: mov rcx, qword ptr [rbp + 0x7a0]
14001134e: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
140011353: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011358: xor ebx, ebx
14001135a: mov byte ptr [rbp + 0x817], 1
140011361: lea r8, [rip + 0x29a2bc8]
140011368: mov rcx, rbx
14001136b: mov rdx, r12
14001136e: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011373: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011378: mov qword ptr [rbp + 0x290], rax
14001137f: xor edi, edi
140011381: mov rax, qword ptr [rbp + 0x290]
140011388: mov qword ptr [rbp + 0xe0], rax
14001138f: mov qword ptr [rbp + 0xd8], rdi
140011396: lea r8, [rip + 0x2915333]
14001139d: mov rcx, qword ptr [rbp + 0x7b8]
1400113a4: call 0x1428d9840 ; core::slice::index::slice_end_index_len_fail
1400113a9: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400113ae: xor esi, esi
1400113b0: lea r8, [rip + 0x299d081]
1400113b7: mov rcx, rsi
1400113ba: mov rdx, r12
1400113bd: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400113c2: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400113c7: xor r14d, r14d
1400113ca: lea r8, [rip + 0x299d067]
1400113d1: mov rcx, r14
1400113d4: mov rdx, r12
1400113d7: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400113dc: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400113e1: lea r8, [rip + 0x2914f48]
1400113e8: mov ecx, 4
1400113ed: mov rdx, rsi
1400113f0: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400113f5: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400113fa: mov byte ptr [rbp + 0x814], 1
140011401: lea rcx, [rip + 0x2915250]
140011408: call 0x1428d9310 ; core::option::unwrap_failed
14001140d: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011412: mov byte ptr [rbp + 0x814], 1
140011419: lea rcx, [rip + 0x2915250]
140011420: call 0x1428d9310 ; core::option::unwrap_failed
140011425: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001142a: xor esi, esi
14001142c: lea r8, [rip + 0x2915f65]
140011433: mov rcx, rsi
140011436: mov rdx, r15
140011439: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14001143e: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011443: lea r8, [rip + 0x29c37ce]
14001144a: mov ecx, 4
14001144f: mov rdx, rdi
140011452: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011457: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001145c: lea r8, [rip + 0x2914ee5]
140011463: mov ecx, 4
140011468: mov rdx, rsi
14001146b: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011470: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011475: lea r8, [rip + 0x2915f1c]
14001147c: mov ecx, 4
140011481: mov edx, 0x30
140011486: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14001148b: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011490: lea r8, [rip + 0x2914ec9]
140011497: mov ecx, 4
14001149c: mov rdx, rsi
14001149f: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400114a4: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400114a9: lea r8, [rip + 0x2914ec8]
1400114b0: mov ecx, 4
1400114b5: mov rdx, rsi
1400114b8: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400114bd: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400114c2: lea rcx, [rip + 0x29150af]
1400114c9: call 0x1428d9310 ; core::option::unwrap_failed
1400114ce: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400114d3: xor esi, esi
1400114d5: mov byte ptr [rbp + 0x813], 1
1400114dc: lea r8, [rip + 0x2915eb5]
1400114e3: mov rcx, rsi
1400114e6: mov rdx, qword ptr [rbp + 0x100]
1400114ed: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400114f2: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400114f7: lea rcx, [rip + 0x2914faa]
1400114fe: call 0x1428d9310 ; core::option::unwrap_failed
140011503: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011508: lea rcx, [rip + 0x2914fb1]
14001150f: call 0x1428d9310 ; core::option::unwrap_failed
140011514: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011519: lea rcx, [rip + 0x2914fb8]
140011520: call 0x1428d9310 ; core::option::unwrap_failed
140011525: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001152a: mov byte ptr [rbp + 0x796], 1
140011531: lea r8, [rip + 0x2914fe0]
140011538: mov ecx, 4
14001153d: mov rdx, r13
140011540: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011545: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001154a: mov byte ptr [rbp + 0x795], 1
140011551: lea r8, [rip + 0x2914ff0]
140011558: mov ecx, 4
14001155d: mov rdx, r13
140011560: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011565: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001156a: xor esi, esi
14001156c: mov r15, qword ptr [rbp + 0x108]
140011573: mov byte ptr [rbp + 0x816], 1
14001157a: mov byte ptr [rbp + 0x815], 1
140011581: lea r8, [rip + 0x2915e10]
140011588: mov rcx, rsi
14001158b: mov rdx, r15
14001158e: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011593: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011598: mov ecx, 0x10
14001159d: mov edx, 0x90
1400115a2: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
1400115a7: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400115ac: mov byte ptr [rbp + 0x817], 1
1400115b3: lea r8, [rip + 0x2915dde]
1400115ba: mov ecx, 4
1400115bf: mov edx, 0x18
1400115c4: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400115c9: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400115ce: mov byte ptr [rbp + 0x7ff], 1
1400115d5: lea rcx, [rip + 0x2921434]
1400115dc: call 0x1428d80d0 ; std::thread::local::panic_access_error
1400115e1: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400115e6: mov byte ptr [rbp + 0x817], 1
1400115ed: lea r8, [rip + 0x293cd8c]
1400115f4: mov rcx, rdx
1400115f7: mov rdx, rdi
1400115fa: call 0x1428d9518 ; core::panicking::panic_bounds_check
1400115ff: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011604: mov byte ptr [rbp + 0x817], 1
14001160b: lea r8, [rip + 0x293cd6e]
140011612: mov rcx, rdx
140011615: mov rdx, rdi
140011618: call 0x1428d9518 ; core::panicking::panic_bounds_check
14001161d: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011622: mov byte ptr [rbp + 0x816], 1
140011629: mov byte ptr [rbp + 0x815], 1
140011630: lea r8, [rip + 0x29153e1]
140011637: xor ecx, ecx
140011639: xor edx, edx
14001163b: call 0x1428d9518 ; core::panicking::panic_bounds_check
140011640: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011645: mov byte ptr [rbp + 0x816], 1
14001164c: mov byte ptr [rbp + 0x815], 1
140011653: lea rcx, [rip + 0x29153d6]
14001165a: call 0x1428d9310 ; core::option::unwrap_failed
14001165f: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011664: xor esi, esi
140011666: jmp 0x140011748 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001166b: mov byte ptr [rbp + 0x816], 1
140011672: mov byte ptr [rbp + 0x815], 1
140011679: lea rcx, [rip + 0x2915250]
140011680: lea r8, [rip + 0x2915271]
140011687: mov edx, 0x28
14001168c: call 0x1428d9430 ; core::panicking::panic
140011691: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011696: mov byte ptr [rbp + 0x7cd], 1
14001169d: lea r8, [rip + 0x291519c]
1400116a4: mov ecx, 4
1400116a9: mov rdx, rsi
1400116ac: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400116b1: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400116b6: lea r8, [rip + 0x29151b3]
1400116bd: mov ecx, 4
1400116c2: mov rdx, rsi
1400116c5: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400116ca: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400116cf: mov byte ptr [rbp + 0x812], 1
1400116d6: mov byte ptr [rbp + 0x811], 1
1400116dd: lea r8, [rip + 0x2915174]
1400116e4: mov ecx, 4
1400116e9: mov rdx, qword ptr [rbp + 0x670]
1400116f0: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400116f5: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400116fa: xor ecx, ecx
1400116fc: jmp 0x140011866 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011701: lea r8, [rip + 0x2915c90]
140011708: mov ecx, 4
14001170d: mov rdx, r12
140011710: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011715: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001171a: mov qword ptr [rbp + 0xd0], rsi
140011721: mov qword ptr [rbp + 0xc8], rdi
140011728: lea r8, [rip + 0x2915c69]
14001172f: mov ecx, 4
140011734: mov rdx, r12
140011737: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14001173c: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011741: mov qword ptr [rbp + 0x2f8], rbx
140011748: lea r8, [rip + 0x2915c49]
14001174f: mov rcx, rsi
140011752: mov rdx, qword ptr [rbp + 0x2f8]
140011759: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14001175e: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011763: xor esi, esi
140011765: mov byte ptr [rbp + 0x7d0], 1
14001176c: lea r8, [rip + 0x2915bbd]
140011773: mov rcx, rsi
140011776: mov rdx, rdi
140011779: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14001177e: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011783: xor esi, esi
140011785: mov byte ptr [rbp + 0x7cc], 1
14001178c: mov byte ptr [rbp + 0x7cb], 1
140011793: lea r8, [rip + 0x2915b96]
14001179a: mov rcx, rsi
14001179d: mov rdx, rdi
1400117a0: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400117a5: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400117aa: xor esi, esi
1400117ac: mov byte ptr [rbp + 0x7ce], 1
1400117b3: lea r8, [rip + 0x2915b76]
1400117ba: mov rcx, rsi
1400117bd: mov rdx, rdi
1400117c0: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400117c5: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400117ca: xor esi, esi
1400117cc: mov byte ptr [rbp + 0x794], 1
1400117d3: mov byte ptr [rbp + 0x793], 1
1400117da: lea r8, [rip + 0x2915b4f]
1400117e1: mov rcx, rsi
1400117e4: mov rdx, rdi
1400117e7: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400117ec: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400117f1: xor esi, esi
1400117f3: mov byte ptr [rbp + 0x7cf], 1
1400117fa: lea r8, [rip + 0x2915b2f]
140011801: mov rcx, rsi
140011804: mov rdx, rdi
140011807: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14001180c: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011811: xor ecx, ecx
140011813: jmp 0x140011903 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011818: mov byte ptr [rbp + 0x816], 1
14001181f: mov byte ptr [rbp + 0x815], 1
140011826: lea rcx, [rip + 0x291508b]
14001182d: call 0x1428d9310 ; core::option::unwrap_failed
140011832: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011837: mov byte ptr [rbp + 0x816], 1
14001183e: mov byte ptr [rbp + 0x815], 1
140011845: lea r8, [rip + 0x2915054]
14001184c: xor ecx, ecx
14001184e: xor edx, edx
140011850: call 0x1428d9518 ; core::panicking::panic_bounds_check
140011855: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001185a: mov ecx, 4
14001185f: mov qword ptr [rbp + 0x300], rsi
140011866: lea r8, [rip + 0x2915b2b]
14001186d: mov rdx, qword ptr [rbp + 0x300]
140011874: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011879: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001187e: lea r8, [rip + 0x2915b13]
140011885: mov ecx, 4
14001188a: mov rdx, r12
14001188d: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011892: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011897: mov rax, qword ptr [rbp + 0x290]
14001189e: mov qword ptr [rbp + 0xe0], rax
1400118a5: mov qword ptr [rbp + 0xd8], rdi
1400118ac: lea r8, [rip + 0x2915ae5]
1400118b3: mov ecx, 4
1400118b8: mov rdx, r12
1400118bb: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400118c0: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400118c5: lea r8, [rip + 0x2915acc]
1400118cc: mov ecx, 4
1400118d1: mov rdx, r12
1400118d4: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400118d9: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400118de: lea r8, [rip + 0x2915ab3]
1400118e5: mov ecx, 4
1400118ea: mov rdx, r12
1400118ed: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1400118f2: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400118f7: mov ecx, 4
1400118fc: mov qword ptr [rbp + 0x308], r15
140011903: lea r8, [rip + 0x2915a8e]
14001190a: mov rdx, qword ptr [rbp + 0x308]
140011911: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140011916: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
14001191b: mov byte ptr [rbp + 0x816], 1
140011922: mov byte ptr [rbp + 0x815], 1
140011929: lea r8, [rip + 0x293ca50]
140011930: mov rcx, rdx
140011933: mov rdx, rdi
140011936: call 0x1428d9518 ; core::panicking::panic_bounds_check
14001193b: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011940: mov byte ptr [rbp + 0x816], 1
140011947: mov byte ptr [rbp + 0x815], 1
14001194e: lea r8, [rip + 0x2914fbb]
140011955: mov ecx, 1
14001195a: call 0x1428d9518 ; core::panicking::panic_bounds_check
14001195f: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011964: mov byte ptr [rbp + 0x816], 1
14001196b: mov byte ptr [rbp + 0x815], 1
140011972: mov ecx, 4
140011977: mov edx, 0x24
14001197c: call 0x1428d9000 ; alloc::alloc::handle_alloc_error
140011981: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011986: mov byte ptr [rbp + 0x816], 1
14001198d: mov byte ptr [rbp + 0x815], 1
140011994: lea r8, [rip + 0x2914f8d]
14001199b: mov ecx, 1
1400119a0: call 0x1428d9518 ; core::panicking::panic_bounds_check
1400119a5: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400119aa: mov byte ptr [rbp + 0x816], 1
1400119b1: mov byte ptr [rbp + 0x815], 1
1400119b8: lea r8, [rip + 0x2914f81]
1400119bf: mov ecx, 1
1400119c4: call 0x1428d9518 ; core::panicking::panic_bounds_check
1400119c9: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400119cb: mov byte ptr [rbp + 0x816], 1
1400119d2: mov byte ptr [rbp + 0x815], 1
1400119d9: lea r8, [rip + 0x2914f78]
1400119e0: mov ecx, 1
1400119e5: call 0x1428d9518 ; core::panicking::panic_bounds_check
1400119ea: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400119ec: mov byte ptr [rbp + 0x816], 1
1400119f3: mov byte ptr [rbp + 0x815], 1
1400119fa: lea r8, [rip + 0x293c97f]
140011a01: mov rcx, rdx
140011a04: mov rdx, rdi
140011a07: call 0x1428d9518 ; core::panicking::panic_bounds_check
140011a0c: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011a0e: mov byte ptr [rbp + 0x816], 1
140011a15: mov byte ptr [rbp + 0x815], 1
140011a1c: lea r8, [rip + 0x293c95d]
140011a23: mov rcx, rdx
140011a26: mov rdx, rdi
140011a29: call 0x1428d9518 ; core::panicking::panic_bounds_check
140011a2e: jmp 0x140011a40 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140011a30: lea r8, [rip + 0x2914f51]
140011a37: xor ecx, ecx
140011a39: xor edx, edx
140011a3b: call 0x1428d9518 ; core::panicking::panic_bounds_check
140011a40: ud2 
140011a42: nop word ptr cs:[rax + rax]
140011a50: mov qword ptr [rsp + 0x10], rdx
140011a55: push rbp
140011a56: push r15
140011a58: push r14
140011a5a: push r13
140011a5c: push r12
140011a5e: push rsi
140011a5f: push rdi
140011a60: push rbx
140011a61: sub rsp, 0x138
140011a68: lea rbp, [rdx + 0x80]
140011a6f: movaps xmmword ptr [rsp + 0x90], xmm15
140011a78: movaps xmmword ptr [rsp + 0xa0], xmm14
140011a81: movaps xmmword ptr [rsp + 0xb0], xmm13
140011a8a: movaps xmmword ptr [rsp + 0xc0], xmm12
140011a93: movaps xmmword ptr [rsp + 0xd0], xmm11
140011a9c: movaps xmmword ptr [rsp + 0xe0], xmm10
140011aa5: movaps xmmword ptr [rsp + 0xf0], xmm9
140011aae: movaps xmmword ptr [rsp + 0x100], xmm8
140011ab7: movaps xmmword ptr [rsp + 0x110], xmm7
140011abf: movaps xmmword ptr [rsp + 0x120], xmm6
140011ac7: mov rdx, qword ptr [rbp + 0x3e0]
140011ace: test rdx, rdx
140011ad1: je 0x140011ae9
140011ad3: mov rcx, qword ptr [rbp + 0x3e8]
140011ada: shl rdx, 2
140011ade: mov r8d, 4
140011ae4: call 0x140613c20 ; ?
140011ae9: movaps xmm6, xmmword ptr [rsp + 0x120]
140011af1: movaps xmm7, xmmword ptr [rsp + 0x110]
140011af9: movaps xmm8, xmmword ptr [rsp + 0x100]
140011b02: movaps xmm9, xmmword ptr [rsp + 0xf0]
140011b0b: movaps xmm10, xmmword ptr [rsp + 0xe0]
140011b14: movaps xmm11, xmmword ptr [rsp + 0xd0]
140011b1d: movaps xmm12, xmmword ptr [rsp + 0xc0]
140011b26: movaps xmm13, xmmword ptr [rsp + 0xb0]
140011b2f: movaps xmm14, xmmword ptr [rsp + 0xa0]
140011b38: movaps xmm15, xmmword ptr [rsp + 0x90]
140011b41: add rsp, 0x138
140011b48: pop rbx
140011b49: pop rdi
140011b4a: pop rsi
140011b4b: pop r12
140011b4d: pop r13
140011b4f: pop r14
140011b51: pop r15
140011b53: pop rbp
140011b54: ret 
140011b55: nop word ptr cs:[rax + rax]
140011b60: mov qword ptr [rsp + 0x10], rdx
140011b65: push rbp
140011b66: push r15
140011b68: push r14
140011b6a: push r13
140011b6c: push r12
140011b6e: push rsi
140011b6f: push rdi
140011b70: push rbx
140011b71: sub rsp, 0x138
140011b78: lea rbp, [rdx + 0x80]
140011b7f: movaps xmmword ptr [rsp + 0x90], xmm15
140011b88: movaps xmmword ptr [rsp + 0xa0], xmm14
140011b91: movaps xmmword ptr [rsp + 0xb0], xmm13
140011b9a: movaps xmmword ptr [rsp + 0xc0], xmm12
140011ba3: movaps xmmword ptr [rsp + 0xd0], xmm11
140011bac: movaps xmmword ptr [rsp + 0xe0], xmm10
140011bb5: movaps xmmword ptr [rsp + 0xf0], xmm9
140011bbe: movaps xmmword ptr [rsp + 0x100], xmm8
140011bc7: movaps xmmword ptr [rsp + 0x110], xmm7
140011bcf: movaps xmmword ptr [rsp + 0x120], xmm6
140011bd7: mov rdx, qword ptr [rbp + 0x500]
140011bde: test rdx, rdx
140011be1: je 0x140011bf9
140011be3: mov rcx, qword ptr [rbp + 0x508]
140011bea: shl rdx, 2
140011bee: mov r8d, 4
140011bf4: call 0x140613c20 ; ?
140011bf9: movaps xmm6, xmmword ptr [rsp + 0x120]
140011c01: movaps xmm7, xmmword ptr [rsp + 0x110]
140011c09: movaps xmm8, xmmword ptr [rsp + 0x100]
140011c12: movaps xmm9, xmmword ptr [rsp + 0xf0]
140011c1b: movaps xmm10, xmmword ptr [rsp + 0xe0]
140011c24: movaps xmm11, xmmword ptr [rsp + 0xd0]
140011c2d: movaps xmm12, xmmword ptr [rsp + 0xc0]
140011c36: movaps xmm13, xmmword ptr [rsp + 0xb0]
140011c3f: movaps xmm14, xmmword ptr [rsp + 0xa0]
140011c48: movaps xmm15, xmmword ptr [rsp + 0x90]
140011c51: add rsp, 0x138
140011c58: pop rbx
140011c59: pop rdi
140011c5a: pop rsi
140011c5b: pop r12
140011c5d: pop r13
140011c5f: pop r14
140011c61: pop r15
140011c63: pop rbp
140011c64: ret 
140011c65: nop word ptr cs:[rax + rax]
140011c70: mov qword ptr [rsp + 0x10], rdx
140011c75: push rbp
140011c76: push r15
140011c78: push r14
140011c7a: push r13
140011c7c: push r12
140011c7e: push rsi
140011c7f: push rdi
140011c80: push rbx
140011c81: sub rsp, 0x138
140011c88: lea rbp, [rdx + 0x80]
140011c8f: movaps xmmword ptr [rsp + 0x90], xmm15
140011c98: movaps xmmword ptr [rsp + 0xa0], xmm14
140011ca1: movaps xmmword ptr [rsp + 0xb0], xmm13
140011caa: movaps xmmword ptr [rsp + 0xc0], xmm12
140011cb3: movaps xmmword ptr [rsp + 0xd0], xmm11
140011cbc: movaps xmmword ptr [rsp + 0xe0], xmm10
140011cc5: movaps xmmword ptr [rsp + 0xf0], xmm9
140011cce: movaps xmmword ptr [rsp + 0x100], xmm8
140011cd7: movaps xmmword ptr [rsp + 0x110], xmm7
140011cdf: movaps xmmword ptr [rsp + 0x120], xmm6
140011ce7: lea rcx, [rbp + 0x6a0]
140011cee: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140011cf3: mov byte ptr [rbp + 0x816], 1
140011cfa: mov byte ptr [rbp + 0x815], 1
140011d01: movaps xmm6, xmmword ptr [rsp + 0x120]
140011d09: movaps xmm7, xmmword ptr [rsp + 0x110]
140011d11: movaps xmm8, xmmword ptr [rsp + 0x100]
140011d1a: movaps xmm9, xmmword ptr [rsp + 0xf0]
140011d23: movaps xmm10, xmmword ptr [rsp + 0xe0]
140011d2c: movaps xmm11, xmmword ptr [rsp + 0xd0]
140011d35: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140011d3f: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140011d49: movaps xmm14, xmmword ptr [rsp + 0xa0]
140011d52: movdqa xmm15, xmmword ptr [rsp + 0x90]
140011d5c: add rsp, 0x138
140011d63: pop rbx
140011d64: pop rdi
140011d65: pop rsi
140011d66: pop r12
140011d68: pop r13
140011d6a: pop r14
140011d6c: pop r15
140011d6e: pop rbp
140011d6f: ret 
140011d70: mov qword ptr [rsp + 0x10], rdx
140011d75: push rbp
140011d76: push r15
140011d78: push r14
140011d7a: push r13
140011d7c: push r12
140011d7e: push rsi
140011d7f: push rdi
140011d80: push rbx
140011d81: sub rsp, 0x138
140011d88: lea rbp, [rdx + 0x80]
140011d8f: movdqa xmmword ptr [rsp + 0x90], xmm15
140011d99: movaps xmmword ptr [rsp + 0xa0], xmm14
140011da2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140011dac: movdqa xmmword ptr [rsp + 0xc0], xmm12
140011db6: movaps xmmword ptr [rsp + 0xd0], xmm11
140011dbf: movaps xmmword ptr [rsp + 0xe0], xmm10
140011dc8: movaps xmmword ptr [rsp + 0xf0], xmm9
140011dd1: movaps xmmword ptr [rsp + 0x100], xmm8
140011dda: movaps xmmword ptr [rsp + 0x110], xmm7
140011de2: movaps xmmword ptr [rsp + 0x120], xmm6
140011dea: mov byte ptr [rbp + 0x816], 0
140011df1: mov byte ptr [rbp + 0x815], 0
140011df8: lea rcx, [rbp + 0x6a0]
140011dff: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140011e04: mov byte ptr [rbp + 0x816], 0
140011e0b: mov byte ptr [rbp + 0x815], 0
140011e12: movaps xmm6, xmmword ptr [rsp + 0x120]
140011e1a: movaps xmm7, xmmword ptr [rsp + 0x110]
140011e22: movaps xmm8, xmmword ptr [rsp + 0x100]
140011e2b: movaps xmm9, xmmword ptr [rsp + 0xf0]
140011e34: movaps xmm10, xmmword ptr [rsp + 0xe0]
140011e3d: movaps xmm11, xmmword ptr [rsp + 0xd0]
140011e46: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140011e50: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140011e5a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140011e63: movdqa xmm15, xmmword ptr [rsp + 0x90]
140011e6d: add rsp, 0x138
140011e74: pop rbx
140011e75: pop rdi
140011e76: pop rsi
140011e77: pop r12
140011e79: pop r13
140011e7b: pop r14
140011e7d: pop r15
140011e7f: pop rbp
140011e80: ret 
140011e81: nop word ptr cs:[rax + rax]
140011e90: mov qword ptr [rsp + 0x10], rdx
140011e95: push rbp
140011e96: push r15
140011e98: push r14
140011e9a: push r13
140011e9c: push r12
140011e9e: push rsi
140011e9f: push rdi
140011ea0: push rbx
140011ea1: sub rsp, 0x138
140011ea8: lea rbp, [rdx + 0x80]
140011eaf: movdqa xmmword ptr [rsp + 0x90], xmm15
140011eb9: movaps xmmword ptr [rsp + 0xa0], xmm14
140011ec2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140011ecc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140011ed6: movaps xmmword ptr [rsp + 0xd0], xmm11
140011edf: movaps xmmword ptr [rsp + 0xe0], xmm10
140011ee8: movaps xmmword ptr [rsp + 0xf0], xmm9
140011ef1: movaps xmmword ptr [rsp + 0x100], xmm8
140011efa: movaps xmmword ptr [rsp + 0x110], xmm7
140011f02: movaps xmmword ptr [rsp + 0x120], xmm6
140011f0a: mov byte ptr [rbp + 0x816], 1
140011f11: mov byte ptr [rbp + 0x815], 1
140011f18: lea rcx, [rbp + 0x6a0]
140011f1f: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140011f24: mov byte ptr [rbp + 0x816], 1
140011f2b: mov byte ptr [rbp + 0x815], 1
140011f32: movaps xmm6, xmmword ptr [rsp + 0x120]
140011f3a: movaps xmm7, xmmword ptr [rsp + 0x110]
140011f42: movaps xmm8, xmmword ptr [rsp + 0x100]
140011f4b: movaps xmm9, xmmword ptr [rsp + 0xf0]
140011f54: movaps xmm10, xmmword ptr [rsp + 0xe0]
140011f5d: movaps xmm11, xmmword ptr [rsp + 0xd0]
140011f66: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140011f70: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140011f7a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140011f83: movdqa xmm15, xmmword ptr [rsp + 0x90]
140011f8d: add rsp, 0x138
140011f94: pop rbx
140011f95: pop rdi
140011f96: pop rsi
140011f97: pop r12
140011f99: pop r13
140011f9b: pop r14
140011f9d: pop r15
140011f9f: pop rbp
140011fa0: ret 
140011fa1: nop word ptr cs:[rax + rax]
140011fb0: mov qword ptr [rsp + 0x10], rdx
140011fb5: push rbp
140011fb6: push r15
140011fb8: push r14
140011fba: push r13
140011fbc: push r12
140011fbe: push rsi
140011fbf: push rdi
140011fc0: push rbx
140011fc1: sub rsp, 0x138
140011fc8: lea rbp, [rdx + 0x80]
140011fcf: movdqa xmmword ptr [rsp + 0x90], xmm15
140011fd9: movaps xmmword ptr [rsp + 0xa0], xmm14
140011fe2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140011fec: movdqa xmmword ptr [rsp + 0xc0], xmm12
140011ff6: movaps xmmword ptr [rsp + 0xd0], xmm11
140011fff: movaps xmmword ptr [rsp + 0xe0], xmm10
140012008: movaps xmmword ptr [rsp + 0xf0], xmm9
140012011: movaps xmmword ptr [rsp + 0x100], xmm8
14001201a: movaps xmmword ptr [rsp + 0x110], xmm7
140012022: movaps xmmword ptr [rsp + 0x120], xmm6
14001202a: mov byte ptr [rbp + 0x813], 0
140012031: lea rcx, [rbp + 0x590]
140012038: call 0x140607a30 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsBracketAssemblyParams>
14001203d: mov byte ptr [rbp + 0x813], 0
140012044: movaps xmm6, xmmword ptr [rsp + 0x120]
14001204c: movaps xmm7, xmmword ptr [rsp + 0x110]
140012054: movaps xmm8, xmmword ptr [rsp + 0x100]
14001205d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012066: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001206f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012078: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140012082: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001208c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012095: movdqa xmm15, xmmword ptr [rsp + 0x90]
14001209f: add rsp, 0x138
1400120a6: pop rbx
1400120a7: pop rdi
1400120a8: pop rsi
1400120a9: pop r12
1400120ab: pop r13
1400120ad: pop r14
1400120af: pop r15
1400120b1: pop rbp
1400120b2: ret 
1400120b3: nop word ptr cs:[rax + rax]
1400120c0: mov qword ptr [rsp + 0x10], rdx
1400120c5: push rbp
1400120c6: push r15
1400120c8: push r14
1400120ca: push r13
1400120cc: push r12
1400120ce: push rsi
1400120cf: push rdi
1400120d0: push rbx
1400120d1: sub rsp, 0x138
1400120d8: lea rbp, [rdx + 0x80]
1400120df: movdqa xmmword ptr [rsp + 0x90], xmm15
1400120e9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400120f2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400120fc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140012106: movaps xmmword ptr [rsp + 0xd0], xmm11
14001210f: movaps xmmword ptr [rsp + 0xe0], xmm10
140012118: movaps xmmword ptr [rsp + 0xf0], xmm9
140012121: movaps xmmword ptr [rsp + 0x100], xmm8
14001212a: movaps xmmword ptr [rsp + 0x110], xmm7
140012132: movaps xmmword ptr [rsp + 0x120], xmm6
14001213a: cmp qword ptr [rbp + 0x798], 0
140012142: jne 0x140012206
140012148: cmp qword ptr [rbp + 0x780], 0
140012150: jne 0x140012235
140012156: cmp qword ptr [rbp + 0x800], 0
14001215e: je 0x14001217d
140012160: mov rdx, qword ptr [rbp + 0x800]
140012167: shl rdx, 2
14001216b: mov r8d, 4
140012171: mov rcx, qword ptr [rbp + 0x760]
140012178: call 0x140613c20 ; ?
14001217d: lea rcx, [rbp + 0x590]
140012184: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140012189: mov byte ptr [rbp + 0x816], 0
140012190: mov byte ptr [rbp + 0x815], 0
140012197: movaps xmm6, xmmword ptr [rsp + 0x120]
14001219f: movaps xmm7, xmmword ptr [rsp + 0x110]
1400121a7: movaps xmm8, xmmword ptr [rsp + 0x100]
1400121b0: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400121b9: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400121c2: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400121cb: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400121d5: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400121df: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400121e8: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400121f2: add rsp, 0x138
1400121f9: pop rbx
1400121fa: pop rdi
1400121fb: pop rsi
1400121fc: pop r12
1400121fe: pop r13
140012200: pop r14
140012202: pop r15
140012204: pop rbp
140012205: ret 
140012206: mov rax, qword ptr [rbp + 0x798]
14001220d: lea rdx, [rax*4]
140012215: mov r8d, 4
14001221b: mov rcx, qword ptr [rbp + 0x7f0]
140012222: call 0x140613c20 ; ?
140012227: cmp qword ptr [rbp + 0x780], 0
14001222f: je 0x140012156
140012235: mov rdx, qword ptr [rbp + 0x780]
14001223c: shl rdx, 3
140012240: mov r8d, 4
140012246: mov rcx, qword ptr [rbp + 0x530]
14001224d: call 0x140613c20 ; ?
140012252: cmp qword ptr [rbp + 0x800], 0
14001225a: jne 0x140012160
140012260: jmp 0x14001217d ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140012265: nop word ptr cs:[rax + rax]
140012270: mov qword ptr [rsp + 0x10], rdx
140012275: push rbp
140012276: push r15
140012278: push r14
14001227a: push r13
14001227c: push r12
14001227e: push rsi
14001227f: push rdi
140012280: push rbx
140012281: sub rsp, 0x138
140012288: lea rbp, [rdx + 0x80]
14001228f: movdqa xmmword ptr [rsp + 0x90], xmm15
140012299: movaps xmmword ptr [rsp + 0xa0], xmm14
1400122a2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400122ac: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400122b6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400122bf: movaps xmmword ptr [rsp + 0xe0], xmm10
1400122c8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400122d1: movaps xmmword ptr [rsp + 0x100], xmm8
1400122da: movaps xmmword ptr [rsp + 0x110], xmm7
1400122e2: movaps xmmword ptr [rsp + 0x120], xmm6
1400122ea: lea rcx, [rbp + 0x590]
1400122f1: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
1400122f6: movaps xmm6, xmmword ptr [rsp + 0x120]
1400122fe: movaps xmm7, xmmword ptr [rsp + 0x110]
140012306: movaps xmm8, xmmword ptr [rsp + 0x100]
14001230f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012318: movaps xmm10, xmmword ptr [rsp + 0xe0]
140012321: movaps xmm11, xmmword ptr [rsp + 0xd0]
14001232a: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140012334: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001233e: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012347: movdqa xmm15, xmmword ptr [rsp + 0x90]
140012351: add rsp, 0x138
140012358: pop rbx
140012359: pop rdi
14001235a: pop rsi
14001235b: pop r12
14001235d: pop r13
14001235f: pop r14
140012361: pop r15
140012363: pop rbp
140012364: ret 
140012365: nop word ptr cs:[rax + rax]
140012370: mov qword ptr [rsp + 0x10], rdx
140012375: push rbp
140012376: push r15
140012378: push r14
14001237a: push r13
14001237c: push r12
14001237e: push rsi
14001237f: push rdi
140012380: push rbx
140012381: sub rsp, 0x138
140012388: lea rbp, [rdx + 0x80]
14001238f: movaps xmmword ptr [rsp + 0x90], xmm15
140012398: movaps xmmword ptr [rsp + 0xa0], xmm14
1400123a1: movaps xmmword ptr [rsp + 0xb0], xmm13
1400123aa: movaps xmmword ptr [rsp + 0xc0], xmm12
1400123b3: movaps xmmword ptr [rsp + 0xd0], xmm11
1400123bc: movaps xmmword ptr [rsp + 0xe0], xmm10
1400123c5: movaps xmmword ptr [rsp + 0xf0], xmm9
1400123ce: movaps xmmword ptr [rsp + 0x100], xmm8
1400123d7: movaps xmmword ptr [rsp + 0x110], xmm7
1400123df: movaps xmmword ptr [rsp + 0x120], xmm6
1400123e7: mov byte ptr [rbp + 0x812], 0
1400123ee: mov byte ptr [rbp + 0x811], 0
1400123f5: lea rcx, [rbp + 0x6a0]
1400123fc: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
140012401: mov byte ptr [rbp + 0x812], 0
140012408: mov byte ptr [rbp + 0x811], 0
14001240f: movaps xmm6, xmmword ptr [rsp + 0x120]
140012417: movaps xmm7, xmmword ptr [rsp + 0x110]
14001241f: movaps xmm8, xmmword ptr [rsp + 0x100]
140012428: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012431: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001243a: movapd xmm11, xmmword ptr [rsp + 0xd0]
140012444: movaps xmm12, xmmword ptr [rsp + 0xc0]
14001244d: movaps xmm13, xmmword ptr [rsp + 0xb0]
140012456: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001245f: movaps xmm15, xmmword ptr [rsp + 0x90]
140012468: add rsp, 0x138
14001246f: pop rbx
140012470: pop rdi
140012471: pop rsi
140012472: pop r12
140012474: pop r13
140012476: pop r14
140012478: pop r15
14001247a: pop rbp
14001247b: ret 
14001247c: nop dword ptr [rax]
140012480: mov qword ptr [rsp + 0x10], rdx
140012485: push rbp
140012486: push r15
140012488: push r14
14001248a: push r13
14001248c: push r12
14001248e: push rsi
14001248f: push rdi
140012490: push rbx
140012491: sub rsp, 0x138
140012498: lea rbp, [rdx + 0x80]
14001249f: movaps xmmword ptr [rsp + 0x90], xmm15
1400124a8: movaps xmmword ptr [rsp + 0xa0], xmm14
1400124b1: movaps xmmword ptr [rsp + 0xb0], xmm13
1400124ba: movaps xmmword ptr [rsp + 0xc0], xmm12
1400124c3: movaps xmmword ptr [rsp + 0xd0], xmm11
1400124cc: movaps xmmword ptr [rsp + 0xe0], xmm10
1400124d5: movaps xmmword ptr [rsp + 0xf0], xmm9
1400124de: movaps xmmword ptr [rsp + 0x100], xmm8
1400124e7: movaps xmmword ptr [rsp + 0x110], xmm7
1400124ef: movaps xmmword ptr [rsp + 0x120], xmm6
1400124f7: mov byte ptr [rbp + 0x7ce], 0
1400124fe: lea rcx, [rbp + 0x6a0]
140012505: call 0x140607a30 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsBracketAssemblyParams>
14001250a: mov byte ptr [rbp + 0x7ce], 0
140012511: movaps xmm6, xmmword ptr [rsp + 0x120]
140012519: movaps xmm7, xmmword ptr [rsp + 0x110]
140012521: movaps xmm8, xmmword ptr [rsp + 0x100]
14001252a: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012533: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001253c: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012545: movaps xmm12, xmmword ptr [rsp + 0xc0]
14001254e: movaps xmm13, xmmword ptr [rsp + 0xb0]
140012557: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012560: movaps xmm15, xmmword ptr [rsp + 0x90]
140012569: add rsp, 0x138
140012570: pop rbx
140012571: pop rdi
140012572: pop rsi
140012573: pop r12
140012575: pop r13
140012577: pop r14
140012579: pop r15
14001257b: pop rbp
14001257c: ret 
14001257d: nop dword ptr [rax]
140012580: mov qword ptr [rsp + 0x10], rdx
140012585: push rbp
140012586: push r15
140012588: push r14
14001258a: push r13
14001258c: push r12
14001258e: push rsi
14001258f: push rdi
140012590: push rbx
140012591: sub rsp, 0x138
140012598: lea rbp, [rdx + 0x80]
14001259f: movdqa xmmword ptr [rsp + 0x90], xmm15
1400125a9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400125b2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400125bc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400125c6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400125cf: movaps xmmword ptr [rsp + 0xe0], xmm10
1400125d8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400125e1: movaps xmmword ptr [rsp + 0x100], xmm8
1400125ea: movaps xmmword ptr [rsp + 0x110], xmm7
1400125f2: movaps xmmword ptr [rsp + 0x120], xmm6
1400125fa: cmp dword ptr [rbp + 0x140], 0xb
140012601: je 0x14001267e
140012603: lea rcx, [rbp + 0x140]
14001260a: call 0x1400297d0 ; core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > > >
14001260f: movaps xmm6, xmmword ptr [rsp + 0x120]
140012617: movaps xmm7, xmmword ptr [rsp + 0x110]
14001261f: movaps xmm8, xmmword ptr [rsp + 0x100]
140012628: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012631: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001263a: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012643: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001264d: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140012657: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012660: movdqa xmm15, xmmword ptr [rsp + 0x90]
14001266a: add rsp, 0x138
140012671: pop rbx
140012672: pop rdi
140012673: pop rsi
140012674: pop r12
140012676: pop r13
140012678: pop r14
14001267a: pop r15
14001267c: pop rbp
14001267d: ret 
14001267e: movaps xmm6, xmmword ptr [rsp + 0x120]
140012686: movaps xmm7, xmmword ptr [rsp + 0x110]
14001268e: movaps xmm8, xmmword ptr [rsp + 0x100]
140012697: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400126a0: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400126a9: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400126b2: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400126bc: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400126c6: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400126cf: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400126d9: add rsp, 0x138
1400126e0: pop rbx
1400126e1: pop rdi
1400126e2: pop rsi
1400126e3: pop r12
1400126e5: pop r13
1400126e7: pop r14
1400126e9: pop r15
1400126eb: pop rbp
1400126ec: ret 
1400126ed: nop dword ptr [rax]
1400126f0: mov qword ptr [rsp + 0x10], rdx
1400126f5: push rbp
1400126f6: push r15
1400126f8: push r14
1400126fa: push r13
1400126fc: push r12
1400126fe: push rsi
1400126ff: push rdi
140012700: push rbx
140012701: sub rsp, 0x138
140012708: lea rbp, [rdx + 0x80]
14001270f: movaps xmmword ptr [rsp + 0x90], xmm15
140012718: movaps xmmword ptr [rsp + 0xa0], xmm14
140012721: movaps xmmword ptr [rsp + 0xb0], xmm13
14001272a: movaps xmmword ptr [rsp + 0xc0], xmm12
140012733: movaps xmmword ptr [rsp + 0xd0], xmm11
14001273c: movaps xmmword ptr [rsp + 0xe0], xmm10
140012745: movaps xmmword ptr [rsp + 0xf0], xmm9
14001274e: movaps xmmword ptr [rsp + 0x100], xmm8
140012757: movaps xmmword ptr [rsp + 0x110], xmm7
14001275f: movaps xmmword ptr [rsp + 0x120], xmm6
140012767: lea rcx, [rbp + 0x6a0]
14001276e: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140012773: mov byte ptr [rbp + 0x812], 1
14001277a: mov byte ptr [rbp + 0x811], 0
140012781: movaps xmm6, xmmword ptr [rsp + 0x120]
140012789: movaps xmm7, xmmword ptr [rsp + 0x110]
140012791: movaps xmm8, xmmword ptr [rsp + 0x100]
14001279a: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400127a3: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400127ac: movapd xmm11, xmmword ptr [rsp + 0xd0]
1400127b6: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400127bf: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400127c8: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400127d1: movaps xmm15, xmmword ptr [rsp + 0x90]
1400127da: add rsp, 0x138
1400127e1: pop rbx
1400127e2: pop rdi
1400127e3: pop rsi
1400127e4: pop r12
1400127e6: pop r13
1400127e8: pop r14
1400127ea: pop r15
1400127ec: pop rbp
1400127ed: ret 
1400127ee: nop 
1400127f0: mov qword ptr [rsp + 0x10], rdx
1400127f5: push rbp
1400127f6: push r15
1400127f8: push r14
1400127fa: push r13
1400127fc: push r12
1400127fe: push rsi
1400127ff: push rdi
140012800: push rbx
140012801: sub rsp, 0x138
140012808: lea rbp, [rdx + 0x80]
14001280f: movaps xmmword ptr [rsp + 0x90], xmm15
140012818: movaps xmmword ptr [rsp + 0xa0], xmm14
140012821: movaps xmmword ptr [rsp + 0xb0], xmm13
14001282a: movaps xmmword ptr [rsp + 0xc0], xmm12
140012833: movaps xmmword ptr [rsp + 0xd0], xmm11
14001283c: movaps xmmword ptr [rsp + 0xe0], xmm10
140012845: movaps xmmword ptr [rsp + 0xf0], xmm9
14001284e: movaps xmmword ptr [rsp + 0x100], xmm8
140012857: movaps xmmword ptr [rsp + 0x110], xmm7
14001285f: movaps xmmword ptr [rsp + 0x120], xmm6
140012867: mov rax, qword ptr [rbp + 0x760]
14001286e: test rax, rax
140012871: je 0x14001288c
140012873: mov rdx, rax
140012876: shl rdx, 2
14001287a: mov r8d, 4
140012880: mov rcx, qword ptr [rbp + 0x7f0]
140012887: call 0x140613c20 ; ?
14001288c: mov byte ptr [rbp + 0x812], 1
140012893: mov byte ptr [rbp + 0x811], 1
14001289a: movaps xmm6, xmmword ptr [rsp + 0x120]
1400128a2: movaps xmm7, xmmword ptr [rsp + 0x110]
1400128aa: movaps xmm8, xmmword ptr [rsp + 0x100]
1400128b3: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400128bc: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400128c5: movapd xmm11, xmmword ptr [rsp + 0xd0]
1400128cf: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400128d8: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400128e1: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400128ea: movaps xmm15, xmmword ptr [rsp + 0x90]
1400128f3: add rsp, 0x138
1400128fa: pop rbx
1400128fb: pop rdi
1400128fc: pop rsi
1400128fd: pop r12
1400128ff: pop r13
140012901: pop r14
140012903: pop r15
140012905: pop rbp
140012906: ret 
140012907: nop word ptr [rax + rax]
140012910: mov qword ptr [rsp + 0x10], rdx
140012915: push rbp
140012916: push r15
140012918: push r14
14001291a: push r13
14001291c: push r12
14001291e: push rsi
14001291f: push rdi
140012920: push rbx
140012921: sub rsp, 0x138
140012928: lea rbp, [rdx + 0x80]
14001292f: movdqa xmmword ptr [rsp + 0x90], xmm15
140012939: movaps xmmword ptr [rsp + 0xa0], xmm14
140012942: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001294c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140012956: movaps xmmword ptr [rsp + 0xd0], xmm11
14001295f: movaps xmmword ptr [rsp + 0xe0], xmm10
140012968: movaps xmmword ptr [rsp + 0xf0], xmm9
140012971: movaps xmmword ptr [rsp + 0x100], xmm8
14001297a: movaps xmmword ptr [rsp + 0x110], xmm7
140012982: movaps xmmword ptr [rsp + 0x120], xmm6
14001298a: mov rax, qword ptr [rbp + 0x800]
140012991: inc qword ptr [rax]
140012994: movaps xmm6, xmmword ptr [rsp + 0x120]
14001299c: movaps xmm7, xmmword ptr [rsp + 0x110]
1400129a4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400129ad: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400129b6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400129bf: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400129c8: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400129d1: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400129da: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400129e3: movaps xmm15, xmmword ptr [rsp + 0x90]
1400129ec: add rsp, 0x138
1400129f3: pop rbx
1400129f4: pop rdi
1400129f5: pop rsi
1400129f6: pop r12
1400129f8: pop r13
1400129fa: pop r14
1400129fc: pop r15
1400129fe: pop rbp
1400129ff: ret 
140012a00: mov qword ptr [rsp + 0x10], rdx
140012a05: push rbp
140012a06: push r15
140012a08: push r14
140012a0a: push r13
140012a0c: push r12
140012a0e: push rsi
140012a0f: push rdi
140012a10: push rbx
140012a11: sub rsp, 0x138
140012a18: lea rbp, [rdx + 0x80]
140012a1f: movaps xmmword ptr [rsp + 0x90], xmm15
140012a28: movaps xmmword ptr [rsp + 0xa0], xmm14
140012a31: movaps xmmword ptr [rsp + 0xb0], xmm13
140012a3a: movaps xmmword ptr [rsp + 0xc0], xmm12
140012a43: movdqa xmmword ptr [rsp + 0xd0], xmm11
140012a4d: movaps xmmword ptr [rsp + 0xe0], xmm10
140012a56: movaps xmmword ptr [rsp + 0xf0], xmm9
140012a5f: movaps xmmword ptr [rsp + 0x100], xmm8
140012a68: movaps xmmword ptr [rsp + 0x110], xmm7
140012a70: movaps xmmword ptr [rsp + 0x120], xmm6
140012a78: mov byte ptr [rbp + 0x817], 1
140012a7f: lea rcx, [rbp + 0x6a0]
140012a86: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140012a8b: mov byte ptr [rbp + 0x817], 1
140012a92: movaps xmm6, xmmword ptr [rsp + 0x120]
140012a9a: movaps xmm7, xmmword ptr [rsp + 0x110]
140012aa2: movaps xmm8, xmmword ptr [rsp + 0x100]
140012aab: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012ab4: movaps xmm10, xmmword ptr [rsp + 0xe0]
140012abd: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012ac6: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140012ad0: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140012ada: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012ae3: movdqa xmm15, xmmword ptr [rsp + 0x90]
140012aed: add rsp, 0x138
140012af4: pop rbx
140012af5: pop rdi
140012af6: pop rsi
140012af7: pop r12
140012af9: pop r13
140012afb: pop r14
140012afd: pop r15
140012aff: pop rbp
140012b00: ret 
140012b01: nop word ptr cs:[rax + rax]
140012b10: mov qword ptr [rsp + 0x10], rdx
140012b15: push rbp
140012b16: push r15
140012b18: push r14
140012b1a: push r13
140012b1c: push r12
140012b1e: push rsi
140012b1f: push rdi
140012b20: push rbx
140012b21: sub rsp, 0x138
140012b28: lea rbp, [rdx + 0x80]
140012b2f: movaps xmmword ptr [rsp + 0x90], xmm15
140012b38: movaps xmmword ptr [rsp + 0xa0], xmm14
140012b41: movaps xmmword ptr [rsp + 0xb0], xmm13
140012b4a: movaps xmmword ptr [rsp + 0xc0], xmm12
140012b53: movaps xmmword ptr [rsp + 0xd0], xmm11
140012b5c: movaps xmmword ptr [rsp + 0xe0], xmm10
140012b65: movaps xmmword ptr [rsp + 0xf0], xmm9
140012b6e: movaps xmmword ptr [rsp + 0x100], xmm8
140012b77: movaps xmmword ptr [rsp + 0x110], xmm7
140012b7f: movaps xmmword ptr [rsp + 0x120], xmm6
140012b87: mov byte ptr [rbp + 0x817], 1
140012b8e: lea rcx, [rbp + 0x6a0]
140012b95: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140012b9a: mov byte ptr [rbp + 0x817], 1
140012ba1: movaps xmm6, xmmword ptr [rsp + 0x120]
140012ba9: movaps xmm7, xmmword ptr [rsp + 0x110]
140012bb1: movaps xmm8, xmmword ptr [rsp + 0x100]
140012bba: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012bc3: movaps xmm10, xmmword ptr [rsp + 0xe0]
140012bcc: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012bd5: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140012bdf: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140012be9: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012bf2: movdqa xmm15, xmmword ptr [rsp + 0x90]
140012bfc: add rsp, 0x138
140012c03: pop rbx
140012c04: pop rdi
140012c05: pop rsi
140012c06: pop r12
140012c08: pop r13
140012c0a: pop r14
140012c0c: pop r15
140012c0e: pop rbp
140012c0f: ret 
140012c10: mov qword ptr [rsp + 0x10], rdx
140012c15: push rbp
140012c16: push r15
140012c18: push r14
140012c1a: push r13
140012c1c: push r12
140012c1e: push rsi
140012c1f: push rdi
140012c20: push rbx
140012c21: sub rsp, 0x138
140012c28: lea rbp, [rdx + 0x80]
140012c2f: movaps xmmword ptr [rsp + 0x90], xmm15
140012c38: movaps xmmword ptr [rsp + 0xa0], xmm14
140012c41: movaps xmmword ptr [rsp + 0xb0], xmm13
140012c4a: movaps xmmword ptr [rsp + 0xc0], xmm12
140012c53: movaps xmmword ptr [rsp + 0xd0], xmm11
140012c5c: movaps xmmword ptr [rsp + 0xe0], xmm10
140012c65: movaps xmmword ptr [rsp + 0xf0], xmm9
140012c6e: movaps xmmword ptr [rsp + 0x100], xmm8
140012c77: movaps xmmword ptr [rsp + 0x110], xmm7
140012c7f: movaps xmmword ptr [rsp + 0x120], xmm6
140012c87: movzx ebx, byte ptr [rbp + 0x7ce]
140012c8e: test bl, 1
140012c91: je 0x140012d30
140012c97: cmp qword ptr [rbp + 0x760], 0
140012c9f: je 0x140012cbe
140012ca1: mov rdx, qword ptr [rbp + 0x760]
140012ca8: shl rdx, 2
140012cac: mov r8d, 4
140012cb2: mov rcx, qword ptr [rbp + 0x7f0]
140012cb9: call 0x140613c20 ; ?
140012cbe: mov byte ptr [rbp + 0x7cf], bl
140012cc4: movaps xmm6, xmmword ptr [rsp + 0x120]
140012ccc: movaps xmm7, xmmword ptr [rsp + 0x110]
140012cd4: movaps xmm8, xmmword ptr [rsp + 0x100]
140012cdd: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012ce6: movaps xmm10, xmmword ptr [rsp + 0xe0]
140012cef: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012cf8: movaps xmm12, xmmword ptr [rsp + 0xc0]
140012d01: movaps xmm13, xmmword ptr [rsp + 0xb0]
140012d0a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012d13: movaps xmm15, xmmword ptr [rsp + 0x90]
140012d1c: add rsp, 0x138
140012d23: pop rbx
140012d24: pop rdi
140012d25: pop rsi
140012d26: pop r12
140012d28: pop r13
140012d2a: pop r14
140012d2c: pop r15
140012d2e: pop rbp
140012d2f: ret 
140012d30: mov byte ptr [rbp + 0x7cf], bl
140012d36: movaps xmm6, xmmword ptr [rsp + 0x120]
140012d3e: movaps xmm7, xmmword ptr [rsp + 0x110]
140012d46: movaps xmm8, xmmword ptr [rsp + 0x100]
140012d4f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012d58: movaps xmm10, xmmword ptr [rsp + 0xe0]
140012d61: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012d6a: movaps xmm12, xmmword ptr [rsp + 0xc0]
140012d73: movaps xmm13, xmmword ptr [rsp + 0xb0]
140012d7c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012d85: movaps xmm15, xmmword ptr [rsp + 0x90]
140012d8e: add rsp, 0x138
140012d95: pop rbx
140012d96: pop rdi
140012d97: pop rsi
140012d98: pop r12
140012d9a: pop r13
140012d9c: pop r14
140012d9e: pop r15
140012da0: pop rbp
140012da1: ret 
140012da2: nop word ptr cs:[rax + rax]
140012db0: mov qword ptr [rsp + 0x10], rdx
140012db5: push rbp
140012db6: push r15
140012db8: push r14
140012dba: push r13
140012dbc: push r12
140012dbe: push rsi
140012dbf: push rdi
140012dc0: push rbx
140012dc1: sub rsp, 0x138
140012dc8: lea rbp, [rdx + 0x80]
140012dcf: movdqa xmmword ptr [rsp + 0x90], xmm15
140012dd9: movaps xmmword ptr [rsp + 0xa0], xmm14
140012de2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140012dec: movdqa xmmword ptr [rsp + 0xc0], xmm12
140012df6: movaps xmmword ptr [rsp + 0xd0], xmm11
140012dff: movaps xmmword ptr [rsp + 0xe0], xmm10
140012e08: movaps xmmword ptr [rsp + 0xf0], xmm9
140012e11: movaps xmmword ptr [rsp + 0x100], xmm8
140012e1a: movaps xmmword ptr [rsp + 0x110], xmm7
140012e22: movaps xmmword ptr [rsp + 0x120], xmm6
140012e2a: mov rax, qword ptr [rbp + 0x780]
140012e31: inc qword ptr [rax]
140012e34: movaps xmm6, xmmword ptr [rsp + 0x120]
140012e3c: movaps xmm7, xmmword ptr [rsp + 0x110]
140012e44: movaps xmm8, xmmword ptr [rsp + 0x100]
140012e4d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012e56: movaps xmm10, xmmword ptr [rsp + 0xe0]
140012e5f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012e68: movaps xmm12, xmmword ptr [rsp + 0xc0]
140012e71: movaps xmm13, xmmword ptr [rsp + 0xb0]
140012e7a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012e83: movaps xmm15, xmmword ptr [rsp + 0x90]
140012e8c: add rsp, 0x138
140012e93: pop rbx
140012e94: pop rdi
140012e95: pop rsi
140012e96: pop r12
140012e98: pop r13
140012e9a: pop r14
140012e9c: pop r15
140012e9e: pop rbp
140012e9f: ret 
140012ea0: mov qword ptr [rsp + 0x10], rdx
140012ea5: push rbp
140012ea6: push r15
140012ea8: push r14
140012eaa: push r13
140012eac: push r12
140012eae: push rsi
140012eaf: push rdi
140012eb0: push rbx
140012eb1: sub rsp, 0x138
140012eb8: lea rbp, [rdx + 0x80]
140012ebf: movaps xmmword ptr [rsp + 0x90], xmm15
140012ec8: movaps xmmword ptr [rsp + 0xa0], xmm14
140012ed1: movaps xmmword ptr [rsp + 0xb0], xmm13
140012eda: movaps xmmword ptr [rsp + 0xc0], xmm12
140012ee3: movaps xmmword ptr [rsp + 0xd0], xmm11
140012eec: movaps xmmword ptr [rsp + 0xe0], xmm10
140012ef5: movaps xmmword ptr [rsp + 0xf0], xmm9
140012efe: movaps xmmword ptr [rsp + 0x100], xmm8
140012f07: movaps xmmword ptr [rsp + 0x110], xmm7
140012f0f: movaps xmmword ptr [rsp + 0x120], xmm6
140012f17: movzx ebx, byte ptr [rbp + 0x7cf]
140012f1e: test bl, 1
140012f21: je 0x140012fc0
140012f27: cmp qword ptr [rbp + 0x800], 0
140012f2f: je 0x140012f4e
140012f31: mov rdx, qword ptr [rbp + 0x800]
140012f38: shl rdx, 3
140012f3c: mov r8d, 4
140012f42: mov rcx, qword ptr [rbp + 0x780]
140012f49: call 0x140613c20 ; ?
140012f4e: mov byte ptr [rbp + 0x7d0], bl
140012f54: movaps xmm6, xmmword ptr [rsp + 0x120]
140012f5c: movaps xmm7, xmmword ptr [rsp + 0x110]
140012f64: movaps xmm8, xmmword ptr [rsp + 0x100]
140012f6d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012f76: movaps xmm10, xmmword ptr [rsp + 0xe0]
140012f7f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012f88: movaps xmm12, xmmword ptr [rsp + 0xc0]
140012f91: movaps xmm13, xmmword ptr [rsp + 0xb0]
140012f9a: movaps xmm14, xmmword ptr [rsp + 0xa0]
140012fa3: movaps xmm15, xmmword ptr [rsp + 0x90]
140012fac: add rsp, 0x138
140012fb3: pop rbx
140012fb4: pop rdi
140012fb5: pop rsi
140012fb6: pop r12
140012fb8: pop r13
140012fba: pop r14
140012fbc: pop r15
140012fbe: pop rbp
140012fbf: ret 
140012fc0: mov byte ptr [rbp + 0x7d0], bl
140012fc6: movaps xmm6, xmmword ptr [rsp + 0x120]
140012fce: movaps xmm7, xmmword ptr [rsp + 0x110]
140012fd6: movaps xmm8, xmmword ptr [rsp + 0x100]
140012fdf: movaps xmm9, xmmword ptr [rsp + 0xf0]
140012fe8: movaps xmm10, xmmword ptr [rsp + 0xe0]
140012ff1: movaps xmm11, xmmword ptr [rsp + 0xd0]
140012ffa: movaps xmm12, xmmword ptr [rsp + 0xc0]
140013003: movaps xmm13, xmmword ptr [rsp + 0xb0]
14001300c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013015: movaps xmm15, xmmword ptr [rsp + 0x90]
14001301e: add rsp, 0x138
140013025: pop rbx
140013026: pop rdi
140013027: pop rsi
140013028: pop r12
14001302a: pop r13
14001302c: pop r14
14001302e: pop r15
140013030: pop rbp
140013031: ret 
140013032: nop word ptr cs:[rax + rax]
140013040: mov qword ptr [rsp + 0x10], rdx
140013045: push rbp
140013046: push r15
140013048: push r14
14001304a: push r13
14001304c: push r12
14001304e: push rsi
14001304f: push rdi
140013050: push rbx
140013051: sub rsp, 0x138
140013058: lea rbp, [rdx + 0x80]
14001305f: movdqa xmmword ptr [rsp + 0x90], xmm15
140013069: movaps xmmword ptr [rsp + 0xa0], xmm14
140013072: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001307c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140013086: movaps xmmword ptr [rsp + 0xd0], xmm11
14001308f: movaps xmmword ptr [rsp + 0xe0], xmm10
140013098: movaps xmmword ptr [rsp + 0xf0], xmm9
1400130a1: movaps xmmword ptr [rsp + 0x100], xmm8
1400130aa: movaps xmmword ptr [rsp + 0x110], xmm7
1400130b2: movaps xmmword ptr [rsp + 0x120], xmm6
1400130ba: mov rdx, qword ptr [rbp + 0x590]
1400130c1: test rdx, rdx
1400130c4: je 0x1400130dc
1400130c6: shl rdx, 4
1400130ca: mov r8d, 0x10
1400130d0: mov rcx, qword ptr [rbp + 0x800]
1400130d7: call 0x140613c20 ; ?
1400130dc: movaps xmm6, xmmword ptr [rsp + 0x120]
1400130e4: movaps xmm7, xmmword ptr [rsp + 0x110]
1400130ec: movaps xmm8, xmmword ptr [rsp + 0x100]
1400130f5: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400130fe: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013107: movaps xmm11, xmmword ptr [rsp + 0xd0]
140013110: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001311a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140013124: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001312d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140013137: add rsp, 0x138
14001313e: pop rbx
14001313f: pop rdi
140013140: pop rsi
140013141: pop r12
140013143: pop r13
140013145: pop r14
140013147: pop r15
140013149: pop rbp
14001314a: ret 
14001314b: nop dword ptr [rax + rax]
140013150: mov qword ptr [rsp + 0x10], rdx
140013155: push rbp
140013156: push r15
140013158: push r14
14001315a: push r13
14001315c: push r12
14001315e: push rsi
14001315f: push rdi
140013160: push rbx
140013161: sub rsp, 0x138
140013168: lea rbp, [rdx + 0x80]
14001316f: movaps xmmword ptr [rsp + 0x90], xmm15
140013178: movaps xmmword ptr [rsp + 0xa0], xmm14
140013181: movaps xmmword ptr [rsp + 0xb0], xmm13
14001318a: movaps xmmword ptr [rsp + 0xc0], xmm12
140013193: movaps xmmword ptr [rsp + 0xd0], xmm11
14001319c: movaps xmmword ptr [rsp + 0xe0], xmm10
1400131a5: movaps xmmword ptr [rsp + 0xf0], xmm9
1400131ae: movaps xmmword ptr [rsp + 0x100], xmm8
1400131b7: movaps xmmword ptr [rsp + 0x110], xmm7
1400131bf: movaps xmmword ptr [rsp + 0x120], xmm6
1400131c7: test byte ptr [rbp + 0x7d0], 1
1400131ce: je 0x140013256
1400131d4: lea rcx, [rbp + 0x590]
1400131db: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
1400131e0: mov byte ptr [rbp + 0x817], 1
1400131e7: movaps xmm6, xmmword ptr [rsp + 0x120]
1400131ef: movaps xmm7, xmmword ptr [rsp + 0x110]
1400131f7: movaps xmm8, xmmword ptr [rsp + 0x100]
140013200: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013209: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013212: movaps xmm11, xmmword ptr [rsp + 0xd0]
14001321b: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140013225: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001322f: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013238: movdqa xmm15, xmmword ptr [rsp + 0x90]
140013242: add rsp, 0x138
140013249: pop rbx
14001324a: pop rdi
14001324b: pop rsi
14001324c: pop r12
14001324e: pop r13
140013250: pop r14
140013252: pop r15
140013254: pop rbp
140013255: ret 
140013256: mov byte ptr [rbp + 0x817], 1
14001325d: movaps xmm6, xmmword ptr [rsp + 0x120]
140013265: movaps xmm7, xmmword ptr [rsp + 0x110]
14001326d: movaps xmm8, xmmword ptr [rsp + 0x100]
140013276: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001327f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013288: movaps xmm11, xmmword ptr [rsp + 0xd0]
140013291: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001329b: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400132a5: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400132ae: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400132b8: add rsp, 0x138
1400132bf: pop rbx
1400132c0: pop rdi
1400132c1: pop rsi
1400132c2: pop r12
1400132c4: pop r13
1400132c6: pop r14
1400132c8: pop r15
1400132ca: pop rbp
1400132cb: ret 
1400132cc: nop dword ptr [rax]
1400132d0: mov qword ptr [rsp + 0x10], rdx
1400132d5: push rbp
1400132d6: push r15
1400132d8: push r14
1400132da: push r13
1400132dc: push r12
1400132de: push rsi
1400132df: push rdi
1400132e0: push rbx
1400132e1: sub rsp, 0x138
1400132e8: lea rbp, [rdx + 0x80]
1400132ef: movaps xmmword ptr [rsp + 0x90], xmm15
1400132f8: movaps xmmword ptr [rsp + 0xa0], xmm14
140013301: movaps xmmword ptr [rsp + 0xb0], xmm13
14001330a: movaps xmmword ptr [rsp + 0xc0], xmm12
140013313: movaps xmmword ptr [rsp + 0xd0], xmm11
14001331c: movaps xmmword ptr [rsp + 0xe0], xmm10
140013325: movaps xmmword ptr [rsp + 0xf0], xmm9
14001332e: movaps xmmword ptr [rsp + 0x100], xmm8
140013337: movaps xmmword ptr [rsp + 0x110], xmm7
14001333f: movaps xmmword ptr [rsp + 0x120], xmm6
140013347: movzx ebx, byte ptr [rbp + 0x811]
14001334e: movzx esi, byte ptr [rbp + 0x812]
140013355: cmp qword ptr [rbp + 0x780], 0
14001335d: je 0x14001337c
14001335f: mov rdx, qword ptr [rbp + 0x780]
140013366: shl rdx, 2
14001336a: mov r8d, 4
140013370: mov rcx, qword ptr [rbp + 0x800]
140013377: call 0x140613c20 ; ?
14001337c: mov byte ptr [rbp + 0x794], sil
140013383: and bl, 1
140013386: mov byte ptr [rbp + 0x793], bl
14001338c: movaps xmm6, xmmword ptr [rsp + 0x120]
140013394: movaps xmm7, xmmword ptr [rsp + 0x110]
14001339c: movaps xmm8, xmmword ptr [rsp + 0x100]
1400133a5: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400133ae: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400133b7: movapd xmm11, xmmword ptr [rsp + 0xd0]
1400133c1: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400133ca: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400133d3: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400133dc: movaps xmm15, xmmword ptr [rsp + 0x90]
1400133e5: add rsp, 0x138
1400133ec: pop rbx
1400133ed: pop rdi
1400133ee: pop rsi
1400133ef: pop r12
1400133f1: pop r13
1400133f3: pop r14
1400133f5: pop r15
1400133f7: pop rbp
1400133f8: ret 
1400133f9: nop dword ptr [rax]
140013400: mov qword ptr [rsp + 0x10], rdx
140013405: push rbp
140013406: push r15
140013408: push r14
14001340a: push r13
14001340c: push r12
14001340e: push rsi
14001340f: push rdi
140013410: push rbx
140013411: sub rsp, 0x138
140013418: lea rbp, [rdx + 0x80]
14001341f: movdqa xmmword ptr [rsp + 0x90], xmm15
140013429: movaps xmmword ptr [rsp + 0xa0], xmm14
140013432: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001343c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140013446: movaps xmmword ptr [rsp + 0xd0], xmm11
14001344f: movaps xmmword ptr [rsp + 0xe0], xmm10
140013458: movaps xmmword ptr [rsp + 0xf0], xmm9
140013461: movaps xmmword ptr [rsp + 0x100], xmm8
14001346a: movaps xmmword ptr [rsp + 0x110], xmm7
140013472: movaps xmmword ptr [rsp + 0x120], xmm6
14001347a: mov byte ptr [rbp + 0x817], 0
140013481: lea rcx, [rbp + 0x6b0]
140013488: call 0x140601be0 ; core::ptr::drop_in_place<country_core::systems::collision::world::RaycastMeshData>
14001348d: mov byte ptr [rbp + 0x817], 0
140013494: movaps xmm6, xmmword ptr [rsp + 0x120]
14001349c: movaps xmm7, xmmword ptr [rsp + 0x110]
1400134a4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400134ad: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400134b6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400134bf: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400134c8: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400134d2: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400134dc: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400134e5: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400134ef: add rsp, 0x138
1400134f6: pop rbx
1400134f7: pop rdi
1400134f8: pop rsi
1400134f9: pop r12
1400134fb: pop r13
1400134fd: pop r14
1400134ff: pop r15
140013501: pop rbp
140013502: ret 
140013503: nop word ptr cs:[rax + rax]
140013510: mov qword ptr [rsp + 0x10], rdx
140013515: push rbp
140013516: push r15
140013518: push r14
14001351a: push r13
14001351c: push r12
14001351e: push rsi
14001351f: push rdi
140013520: push rbx
140013521: sub rsp, 0x138
140013528: lea rbp, [rdx + 0x80]
14001352f: movdqa xmmword ptr [rsp + 0x90], xmm15
140013539: movaps xmmword ptr [rsp + 0xa0], xmm14
140013542: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001354c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140013556: movaps xmmword ptr [rsp + 0xd0], xmm11
14001355f: movaps xmmword ptr [rsp + 0xe0], xmm10
140013568: movaps xmmword ptr [rsp + 0xf0], xmm9
140013571: movaps xmmword ptr [rsp + 0x100], xmm8
14001357a: movaps xmmword ptr [rsp + 0x110], xmm7
140013582: movaps xmmword ptr [rsp + 0x120], xmm6
14001358a: mov byte ptr [rbp + 0x814], 0
140013591: lea rcx, [rbp + 0x590]
140013598: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
14001359d: mov byte ptr [rbp + 0x814], 0
1400135a4: movaps xmm6, xmmword ptr [rsp + 0x120]
1400135ac: movaps xmm7, xmmword ptr [rsp + 0x110]
1400135b4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400135bd: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400135c6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400135cf: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400135d8: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400135e2: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400135ec: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400135f5: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400135ff: add rsp, 0x138
140013606: pop rbx
140013607: pop rdi
140013608: pop rsi
140013609: pop r12
14001360b: pop r13
14001360d: pop r14
14001360f: pop r15
140013611: pop rbp
140013612: ret 
140013613: nop word ptr cs:[rax + rax]
140013620: mov qword ptr [rsp + 0x10], rdx
140013625: push rbp
140013626: push r15
140013628: push r14
14001362a: push r13
14001362c: push r12
14001362e: push rsi
14001362f: push rdi
140013630: push rbx
140013631: sub rsp, 0x138
140013638: lea rbp, [rdx + 0x80]
14001363f: movdqa xmmword ptr [rsp + 0x90], xmm15
140013649: movaps xmmword ptr [rsp + 0xa0], xmm14
140013652: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001365c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140013666: movaps xmmword ptr [rsp + 0xd0], xmm11
14001366f: movaps xmmword ptr [rsp + 0xe0], xmm10
140013678: movaps xmmword ptr [rsp + 0xf0], xmm9
140013681: movaps xmmword ptr [rsp + 0x100], xmm8
14001368a: movaps xmmword ptr [rsp + 0x110], xmm7
140013692: movaps xmmword ptr [rsp + 0x120], xmm6
14001369a: mov byte ptr [rbp + 0x7de], 0
1400136a1: lea rcx, [rbp + 0x140]
1400136a8: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
1400136ad: mov byte ptr [rbp + 0x7de], 0
1400136b4: movaps xmm6, xmmword ptr [rsp + 0x120]
1400136bc: movaps xmm7, xmmword ptr [rsp + 0x110]
1400136c4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400136cd: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400136d6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400136df: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400136e8: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400136f1: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400136fa: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013703: movaps xmm15, xmmword ptr [rsp + 0x90]
14001370c: add rsp, 0x138
140013713: pop rbx
140013714: pop rdi
140013715: pop rsi
140013716: pop r12
140013718: pop r13
14001371a: pop r14
14001371c: pop r15
14001371e: pop rbp
14001371f: ret 
140013720: mov qword ptr [rsp + 0x10], rdx
140013725: push rbp
140013726: push r15
140013728: push r14
14001372a: push r13
14001372c: push r12
14001372e: push rsi
14001372f: push rdi
140013730: push rbx
140013731: sub rsp, 0x138
140013738: lea rbp, [rdx + 0x80]
14001373f: movdqa xmmword ptr [rsp + 0x90], xmm15
140013749: movaps xmmword ptr [rsp + 0xa0], xmm14
140013752: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001375c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140013766: movaps xmmword ptr [rsp + 0xd0], xmm11
14001376f: movaps xmmword ptr [rsp + 0xe0], xmm10
140013778: movaps xmmword ptr [rsp + 0xf0], xmm9
140013781: movaps xmmword ptr [rsp + 0x100], xmm8
14001378a: movaps xmmword ptr [rsp + 0x110], xmm7
140013792: movaps xmmword ptr [rsp + 0x120], xmm6
14001379a: mov byte ptr [rbp + 0x7df], 0
1400137a1: lea rcx, [rbp + 0x140]
1400137a8: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
1400137ad: mov byte ptr [rbp + 0x7df], 0
1400137b4: movaps xmm6, xmmword ptr [rsp + 0x120]
1400137bc: movaps xmm7, xmmword ptr [rsp + 0x110]
1400137c4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400137cd: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400137d6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400137df: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400137e8: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400137f1: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400137fa: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013803: movaps xmm15, xmmword ptr [rsp + 0x90]
14001380c: add rsp, 0x138
140013813: pop rbx
140013814: pop rdi
140013815: pop rsi
140013816: pop r12
140013818: pop r13
14001381a: pop r14
14001381c: pop r15
14001381e: pop rbp
14001381f: ret 
140013820: mov qword ptr [rsp + 0x10], rdx
140013825: push rbp
140013826: push r15
140013828: push r14
14001382a: push r13
14001382c: push r12
14001382e: push rsi
14001382f: push rdi
140013830: push rbx
140013831: sub rsp, 0x138
140013838: lea rbp, [rdx + 0x80]
14001383f: movaps xmmword ptr [rsp + 0x90], xmm15
140013848: movaps xmmword ptr [rsp + 0xa0], xmm14
140013851: movaps xmmword ptr [rsp + 0xb0], xmm13
14001385a: movaps xmmword ptr [rsp + 0xc0], xmm12
140013863: movapd xmmword ptr [rsp + 0xd0], xmm11
14001386d: movaps xmmword ptr [rsp + 0xe0], xmm10
140013876: movaps xmmword ptr [rsp + 0xf0], xmm9
14001387f: movaps xmmword ptr [rsp + 0x100], xmm8
140013888: movaps xmmword ptr [rsp + 0x110], xmm7
140013890: movaps xmmword ptr [rsp + 0x120], xmm6
140013898: movzx ebx, byte ptr [rbp + 0x793]
14001389f: movzx esi, byte ptr [rbp + 0x794]
1400138a6: test sil, 1
1400138aa: je 0x14001394f
1400138b0: mov rdx, qword ptr [rbp + 0x3e0]
1400138b7: test rdx, rdx
1400138ba: je 0x1400138d2
1400138bc: mov rcx, qword ptr [rbp + 0x3e8]
1400138c3: shl rdx, 2
1400138c7: mov r8d, 4
1400138cd: call 0x140613c20 ; ?
1400138d2: mov byte ptr [rbp + 0x7cc], sil
1400138d9: and bl, 1
1400138dc: mov byte ptr [rbp + 0x7cb], bl
1400138e2: movaps xmm6, xmmword ptr [rsp + 0x120]
1400138ea: movaps xmm7, xmmword ptr [rsp + 0x110]
1400138f2: movaps xmm8, xmmword ptr [rsp + 0x100]
1400138fb: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013904: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001390d: movapd xmm11, xmmword ptr [rsp + 0xd0]
140013917: movaps xmm12, xmmword ptr [rsp + 0xc0]
140013920: movaps xmm13, xmmword ptr [rsp + 0xb0]
140013929: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013932: movaps xmm15, xmmword ptr [rsp + 0x90]
14001393b: add rsp, 0x138
140013942: pop rbx
140013943: pop rdi
140013944: pop rsi
140013945: pop r12
140013947: pop r13
140013949: pop r14
14001394b: pop r15
14001394d: pop rbp
14001394e: ret 
14001394f: mov byte ptr [rbp + 0x7cc], sil
140013956: and bl, 1
140013959: mov byte ptr [rbp + 0x7cb], bl
14001395f: movaps xmm6, xmmword ptr [rsp + 0x120]
140013967: movaps xmm7, xmmword ptr [rsp + 0x110]
14001396f: movaps xmm8, xmmword ptr [rsp + 0x100]
140013978: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013981: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001398a: movapd xmm11, xmmword ptr [rsp + 0xd0]
140013994: movaps xmm12, xmmword ptr [rsp + 0xc0]
14001399d: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400139a6: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400139af: movaps xmm15, xmmword ptr [rsp + 0x90]
1400139b8: add rsp, 0x138
1400139bf: pop rbx
1400139c0: pop rdi
1400139c1: pop rsi
1400139c2: pop r12
1400139c4: pop r13
1400139c6: pop r14
1400139c8: pop r15
1400139ca: pop rbp
1400139cb: ret 
1400139cc: nop dword ptr [rax]
1400139d0: mov qword ptr [rsp + 0x10], rdx
1400139d5: push rbp
1400139d6: push r15
1400139d8: push r14
1400139da: push r13
1400139dc: push r12
1400139de: push rsi
1400139df: push rdi
1400139e0: push rbx
1400139e1: sub rsp, 0x138
1400139e8: lea rbp, [rdx + 0x80]
1400139ef: movaps xmmword ptr [rsp + 0x90], xmm15
1400139f8: movaps xmmword ptr [rsp + 0xa0], xmm14
140013a01: movaps xmmword ptr [rsp + 0xb0], xmm13
140013a0a: movaps xmmword ptr [rsp + 0xc0], xmm12
140013a13: movapd xmmword ptr [rsp + 0xd0], xmm11
140013a1d: movaps xmmword ptr [rsp + 0xe0], xmm10
140013a26: movaps xmmword ptr [rsp + 0xf0], xmm9
140013a2f: movaps xmmword ptr [rsp + 0x100], xmm8
140013a38: movaps xmmword ptr [rsp + 0x110], xmm7
140013a40: movaps xmmword ptr [rsp + 0x120], xmm6
140013a48: movzx ebx, byte ptr [rbp + 0x7cb]
140013a4f: test byte ptr [rbp + 0x7cc], 1
140013a56: je 0x140013af4
140013a5c: mov rdx, qword ptr [rbp + 0x500]
140013a63: test rdx, rdx
140013a66: je 0x140013a7e
140013a68: mov rcx, qword ptr [rbp + 0x508]
140013a6f: shl rdx, 3
140013a73: mov r8d, 4
140013a79: call 0x140613c20 ; ?
140013a7e: and bl, 1
140013a81: mov byte ptr [rbp + 0x7cd], bl
140013a87: movaps xmm6, xmmword ptr [rsp + 0x120]
140013a8f: movaps xmm7, xmmword ptr [rsp + 0x110]
140013a97: movaps xmm8, xmmword ptr [rsp + 0x100]
140013aa0: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013aa9: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013ab2: movapd xmm11, xmmword ptr [rsp + 0xd0]
140013abc: movaps xmm12, xmmword ptr [rsp + 0xc0]
140013ac5: movaps xmm13, xmmword ptr [rsp + 0xb0]
140013ace: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013ad7: movaps xmm15, xmmword ptr [rsp + 0x90]
140013ae0: add rsp, 0x138
140013ae7: pop rbx
140013ae8: pop rdi
140013ae9: pop rsi
140013aea: pop r12
140013aec: pop r13
140013aee: pop r14
140013af0: pop r15
140013af2: pop rbp
140013af3: ret 
140013af4: and bl, 1
140013af7: mov byte ptr [rbp + 0x7cd], bl
140013afd: movaps xmm6, xmmword ptr [rsp + 0x120]
140013b05: movaps xmm7, xmmword ptr [rsp + 0x110]
140013b0d: movaps xmm8, xmmword ptr [rsp + 0x100]
140013b16: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013b1f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013b28: movapd xmm11, xmmword ptr [rsp + 0xd0]
140013b32: movaps xmm12, xmmword ptr [rsp + 0xc0]
140013b3b: movaps xmm13, xmmword ptr [rsp + 0xb0]
140013b44: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013b4d: movaps xmm15, xmmword ptr [rsp + 0x90]
140013b56: add rsp, 0x138
140013b5d: pop rbx
140013b5e: pop rdi
140013b5f: pop rsi
140013b60: pop r12
140013b62: pop r13
140013b64: pop r14
140013b66: pop r15
140013b68: pop rbp
140013b69: ret 
140013b6a: nop word ptr [rax + rax]
140013b70: mov qword ptr [rsp + 0x10], rdx
140013b75: push rbp
140013b76: push r15
140013b78: push r14
140013b7a: push r13
140013b7c: push r12
140013b7e: push rsi
140013b7f: push rdi
140013b80: push rbx
140013b81: sub rsp, 0x138
140013b88: lea rbp, [rdx + 0x80]
140013b8f: movdqa xmmword ptr [rsp + 0x90], xmm15
140013b99: movaps xmmword ptr [rsp + 0xa0], xmm14
140013ba2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140013bac: movdqa xmmword ptr [rsp + 0xc0], xmm12
140013bb6: movaps xmmword ptr [rsp + 0xd0], xmm11
140013bbf: movaps xmmword ptr [rsp + 0xe0], xmm10
140013bc8: movaps xmmword ptr [rsp + 0xf0], xmm9
140013bd1: movaps xmmword ptr [rsp + 0x100], xmm8
140013bda: movaps xmmword ptr [rsp + 0x110], xmm7
140013be2: movaps xmmword ptr [rsp + 0x120], xmm6
140013bea: mov rcx, qword ptr [rbp + 0x1b8]
140013bf1: call 0x1405df260 ; core::ptr::drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo> >
140013bf6: movaps xmm6, xmmword ptr [rsp + 0x120]
140013bfe: movaps xmm7, xmmword ptr [rsp + 0x110]
140013c06: movaps xmm8, xmmword ptr [rsp + 0x100]
140013c0f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013c18: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013c21: movaps xmm11, xmmword ptr [rsp + 0xd0]
140013c2a: movaps xmm12, xmmword ptr [rsp + 0xc0]
140013c33: movaps xmm13, xmmword ptr [rsp + 0xb0]
140013c3c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013c45: movaps xmm15, xmmword ptr [rsp + 0x90]
140013c4e: add rsp, 0x138
140013c55: pop rbx
140013c56: pop rdi
140013c57: pop rsi
140013c58: pop r12
140013c5a: pop r13
140013c5c: pop r14
140013c5e: pop r15
140013c60: pop rbp
140013c61: ret 
140013c62: nop word ptr cs:[rax + rax]
140013c70: mov qword ptr [rsp + 0x10], rdx
140013c75: push rbp
140013c76: push r15
140013c78: push r14
140013c7a: push r13
140013c7c: push r12
140013c7e: push rsi
140013c7f: push rdi
140013c80: push rbx
140013c81: sub rsp, 0x138
140013c88: lea rbp, [rdx + 0x80]
140013c8f: movdqa xmmword ptr [rsp + 0x90], xmm15
140013c99: movaps xmmword ptr [rsp + 0xa0], xmm14
140013ca2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140013cac: movdqa xmmword ptr [rsp + 0xc0], xmm12
140013cb6: movaps xmmword ptr [rsp + 0xd0], xmm11
140013cbf: movaps xmmword ptr [rsp + 0xe0], xmm10
140013cc8: movaps xmmword ptr [rsp + 0xf0], xmm9
140013cd1: movaps xmmword ptr [rsp + 0x100], xmm8
140013cda: movaps xmmword ptr [rsp + 0x110], xmm7
140013ce2: movaps xmmword ptr [rsp + 0x120], xmm6
140013cea: mov byte ptr [rbp + 0x814], 0
140013cf1: lea rcx, [rbp + 0x590]
140013cf8: call 0x140607a30 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsBracketAssemblyParams>
140013cfd: mov byte ptr [rbp + 0x814], 0
140013d04: movaps xmm6, xmmword ptr [rsp + 0x120]
140013d0c: movaps xmm7, xmmword ptr [rsp + 0x110]
140013d14: movaps xmm8, xmmword ptr [rsp + 0x100]
140013d1d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013d26: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013d2f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140013d38: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140013d42: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140013d4c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013d55: movdqa xmm15, xmmword ptr [rsp + 0x90]
140013d5f: add rsp, 0x138
140013d66: pop rbx
140013d67: pop rdi
140013d68: pop rsi
140013d69: pop r12
140013d6b: pop r13
140013d6d: pop r14
140013d6f: pop r15
140013d71: pop rbp
140013d72: ret 
140013d73: nop word ptr cs:[rax + rax]
140013d80: mov qword ptr [rsp + 0x10], rdx
140013d85: push rbp
140013d86: push r15
140013d88: push r14
140013d8a: push r13
140013d8c: push r12
140013d8e: push rsi
140013d8f: push rdi
140013d90: push rbx
140013d91: sub rsp, 0x138
140013d98: lea rbp, [rdx + 0x80]
140013d9f: movaps xmmword ptr [rsp + 0x90], xmm15
140013da8: movaps xmmword ptr [rsp + 0xa0], xmm14
140013db1: movaps xmmword ptr [rsp + 0xb0], xmm13
140013dba: movaps xmmword ptr [rsp + 0xc0], xmm12
140013dc3: movaps xmmword ptr [rsp + 0xd0], xmm11
140013dcc: movaps xmmword ptr [rsp + 0xe0], xmm10
140013dd5: movaps xmmword ptr [rsp + 0xf0], xmm9
140013dde: movaps xmmword ptr [rsp + 0x100], xmm8
140013de7: movaps xmmword ptr [rsp + 0x110], xmm7
140013def: movaps xmmword ptr [rsp + 0x120], xmm6
140013df7: cmp byte ptr [rbp + 0x7cd], 0
140013dfe: je 0x140013e86
140013e04: lea rcx, [rbp + 0x460]
140013e0b: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140013e10: mov byte ptr [rbp + 0x817], 1
140013e17: movaps xmm6, xmmword ptr [rsp + 0x120]
140013e1f: movaps xmm7, xmmword ptr [rsp + 0x110]
140013e27: movaps xmm8, xmmword ptr [rsp + 0x100]
140013e30: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013e39: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013e42: movaps xmm11, xmmword ptr [rsp + 0xd0]
140013e4b: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140013e55: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140013e5f: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013e68: movdqa xmm15, xmmword ptr [rsp + 0x90]
140013e72: add rsp, 0x138
140013e79: pop rbx
140013e7a: pop rdi
140013e7b: pop rsi
140013e7c: pop r12
140013e7e: pop r13
140013e80: pop r14
140013e82: pop r15
140013e84: pop rbp
140013e85: ret 
140013e86: mov byte ptr [rbp + 0x817], 1
140013e8d: movaps xmm6, xmmword ptr [rsp + 0x120]
140013e95: movaps xmm7, xmmword ptr [rsp + 0x110]
140013e9d: movaps xmm8, xmmword ptr [rsp + 0x100]
140013ea6: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013eaf: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013eb8: movaps xmm11, xmmword ptr [rsp + 0xd0]
140013ec1: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140013ecb: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140013ed5: movaps xmm14, xmmword ptr [rsp + 0xa0]
140013ede: movdqa xmm15, xmmword ptr [rsp + 0x90]
140013ee8: add rsp, 0x138
140013eef: pop rbx
140013ef0: pop rdi
140013ef1: pop rsi
140013ef2: pop r12
140013ef4: pop r13
140013ef6: pop r14
140013ef8: pop r15
140013efa: pop rbp
140013efb: ret 
140013efc: nop dword ptr [rax]
140013f00: mov qword ptr [rsp + 0x10], rdx
140013f05: push rbp
140013f06: push r15
140013f08: push r14
140013f0a: push r13
140013f0c: push r12
140013f0e: push rsi
140013f0f: push rdi
140013f10: push rbx
140013f11: sub rsp, 0x138
140013f18: lea rbp, [rdx + 0x80]
140013f1f: movdqa xmmword ptr [rsp + 0x90], xmm15
140013f29: movaps xmmword ptr [rsp + 0xa0], xmm14
140013f32: movdqa xmmword ptr [rsp + 0xb0], xmm13
140013f3c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140013f46: movaps xmmword ptr [rsp + 0xd0], xmm11
140013f4f: movaps xmmword ptr [rsp + 0xe0], xmm10
140013f58: movaps xmmword ptr [rsp + 0xf0], xmm9
140013f61: movaps xmmword ptr [rsp + 0x100], xmm8
140013f6a: movaps xmmword ptr [rsp + 0x110], xmm7
140013f72: movaps xmmword ptr [rsp + 0x120], xmm6
140013f7a: cmp qword ptr [rbp + 0x800], 0
140013f82: je 0x140013fa5
140013f84: mov rax, qword ptr [rbp + 0x800]
140013f8b: lea rdx, [rax*4]
140013f93: mov r8d, 4
140013f99: mov rcx, qword ptr [rbp + 0x760]
140013fa0: call 0x140613c20 ; ?
140013fa5: cmp qword ptr [rbp + 0x7f0], 0
140013fad: je 0x140013fcc
140013faf: mov rdx, qword ptr [rbp + 0x7f0]
140013fb6: shl rdx, 2
140013fba: mov r8d, 4
140013fc0: mov rcx, qword ptr [rbp + 0x778]
140013fc7: call 0x140613c20 ; ?
140013fcc: mov byte ptr [rbp + 0x795], 0
140013fd3: movaps xmm6, xmmword ptr [rsp + 0x120]
140013fdb: movaps xmm7, xmmword ptr [rsp + 0x110]
140013fe3: movaps xmm8, xmmword ptr [rsp + 0x100]
140013fec: movaps xmm9, xmmword ptr [rsp + 0xf0]
140013ff5: movaps xmm10, xmmword ptr [rsp + 0xe0]
140013ffe: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014007: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014011: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001401b: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014024: movdqa xmm15, xmmword ptr [rsp + 0x90]
14001402e: add rsp, 0x138
140014035: pop rbx
140014036: pop rdi
140014037: pop rsi
140014038: pop r12
14001403a: pop r13
14001403c: pop r14
14001403e: pop r15
140014040: pop rbp
140014041: ret 
140014042: nop word ptr cs:[rax + rax]
140014050: mov qword ptr [rsp + 0x10], rdx
140014055: push rbp
140014056: push r15
140014058: push r14
14001405a: push r13
14001405c: push r12
14001405e: push rsi
14001405f: push rdi
140014060: push rbx
140014061: sub rsp, 0x138
140014068: lea rbp, [rdx + 0x80]
14001406f: movdqa xmmword ptr [rsp + 0x90], xmm15
140014079: movaps xmmword ptr [rsp + 0xa0], xmm14
140014082: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001408c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140014096: movaps xmmword ptr [rsp + 0xd0], xmm11
14001409f: movaps xmmword ptr [rsp + 0xe0], xmm10
1400140a8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400140b1: movaps xmmword ptr [rsp + 0x100], xmm8
1400140ba: movaps xmmword ptr [rsp + 0x110], xmm7
1400140c2: movaps xmmword ptr [rsp + 0x120], xmm6
1400140ca: cmp qword ptr [rbp + 0x800], 0
1400140d2: je 0x1400140f5
1400140d4: mov rax, qword ptr [rbp + 0x800]
1400140db: lea rdx, [rax*4]
1400140e3: mov r8d, 4
1400140e9: mov rcx, qword ptr [rbp + 0x760]
1400140f0: call 0x140613c20 ; ?
1400140f5: cmp qword ptr [rbp + 0x7f0], 0
1400140fd: je 0x14001411c
1400140ff: mov rdx, qword ptr [rbp + 0x7f0]
140014106: shl rdx, 2
14001410a: mov r8d, 4
140014110: mov rcx, qword ptr [rbp + 0x778]
140014117: call 0x140613c20 ; ?
14001411c: mov byte ptr [rbp + 0x796], 0
140014123: movaps xmm6, xmmword ptr [rsp + 0x120]
14001412b: movaps xmm7, xmmword ptr [rsp + 0x110]
140014133: movaps xmm8, xmmword ptr [rsp + 0x100]
14001413c: movaps xmm9, xmmword ptr [rsp + 0xf0]
140014145: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001414e: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014157: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014161: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001416b: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014174: movdqa xmm15, xmmword ptr [rsp + 0x90]
14001417e: add rsp, 0x138
140014185: pop rbx
140014186: pop rdi
140014187: pop rsi
140014188: pop r12
14001418a: pop r13
14001418c: pop r14
14001418e: pop r15
140014190: pop rbp
140014191: ret 
140014192: nop word ptr cs:[rax + rax]
1400141a0: mov qword ptr [rsp + 0x10], rdx
1400141a5: push rbp
1400141a6: push r15
1400141a8: push r14
1400141aa: push r13
1400141ac: push r12
1400141ae: push rsi
1400141af: push rdi
1400141b0: push rbx
1400141b1: sub rsp, 0x138
1400141b8: lea rbp, [rdx + 0x80]
1400141bf: movdqa xmmword ptr [rsp + 0x90], xmm15
1400141c9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400141d2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400141dc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400141e6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400141ef: movaps xmmword ptr [rsp + 0xe0], xmm10
1400141f8: movaps xmmword ptr [rsp + 0xf0], xmm9
140014201: movaps xmmword ptr [rsp + 0x100], xmm8
14001420a: movaps xmmword ptr [rsp + 0x110], xmm7
140014212: movaps xmmword ptr [rsp + 0x120], xmm6
14001421a: movzx ebx, byte ptr [rbp + 0x795]
140014221: cmp qword ptr [rbp + 0x780], 0
140014229: je 0x140014248
14001422b: mov rdx, qword ptr [rbp + 0x780]
140014232: shl rdx, 3
140014236: mov r8d, 4
14001423c: mov rcx, qword ptr [rbp + 0x7b0]
140014243: call 0x140613c20 ; ?
140014248: lea rcx, [rbp + 0x590]
14001424f: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140014254: and bl, 1
140014257: mov byte ptr [rbp + 0x7de], bl
14001425d: movaps xmm6, xmmword ptr [rsp + 0x120]
140014265: movaps xmm7, xmmword ptr [rsp + 0x110]
14001426d: movaps xmm8, xmmword ptr [rsp + 0x100]
140014276: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001427f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140014288: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014291: movaps xmm12, xmmword ptr [rsp + 0xc0]
14001429a: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400142a3: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400142ac: movaps xmm15, xmmword ptr [rsp + 0x90]
1400142b5: add rsp, 0x138
1400142bc: pop rbx
1400142bd: pop rdi
1400142be: pop rsi
1400142bf: pop r12
1400142c1: pop r13
1400142c3: pop r14
1400142c5: pop r15
1400142c7: pop rbp
1400142c8: ret 
1400142c9: nop dword ptr [rax]
1400142d0: mov qword ptr [rsp + 0x10], rdx
1400142d5: push rbp
1400142d6: push r15
1400142d8: push r14
1400142da: push r13
1400142dc: push r12
1400142de: push rsi
1400142df: push rdi
1400142e0: push rbx
1400142e1: sub rsp, 0x138
1400142e8: lea rbp, [rdx + 0x80]
1400142ef: movdqa xmmword ptr [rsp + 0x90], xmm15
1400142f9: movaps xmmword ptr [rsp + 0xa0], xmm14
140014302: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001430c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140014316: movaps xmmword ptr [rsp + 0xd0], xmm11
14001431f: movaps xmmword ptr [rsp + 0xe0], xmm10
140014328: movaps xmmword ptr [rsp + 0xf0], xmm9
140014331: movaps xmmword ptr [rsp + 0x100], xmm8
14001433a: movaps xmmword ptr [rsp + 0x110], xmm7
140014342: movaps xmmword ptr [rsp + 0x120], xmm6
14001434a: movzx ebx, byte ptr [rbp + 0x796]
140014351: cmp qword ptr [rbp + 0x780], 0
140014359: je 0x140014378
14001435b: mov rdx, qword ptr [rbp + 0x780]
140014362: shl rdx, 3
140014366: mov r8d, 4
14001436c: mov rcx, qword ptr [rbp + 0x7b0]
140014373: call 0x140613c20 ; ?
140014378: lea rcx, [rbp + 0x590]
14001437f: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140014384: and bl, 1
140014387: mov byte ptr [rbp + 0x7df], bl
14001438d: movaps xmm6, xmmword ptr [rsp + 0x120]
140014395: movaps xmm7, xmmword ptr [rsp + 0x110]
14001439d: movaps xmm8, xmmword ptr [rsp + 0x100]
1400143a6: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400143af: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400143b8: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400143c1: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400143ca: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400143d3: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400143dc: movaps xmm15, xmmword ptr [rsp + 0x90]
1400143e5: add rsp, 0x138
1400143ec: pop rbx
1400143ed: pop rdi
1400143ee: pop rsi
1400143ef: pop r12
1400143f1: pop r13
1400143f3: pop r14
1400143f5: pop r15
1400143f7: pop rbp
1400143f8: ret 
1400143f9: nop dword ptr [rax]
140014400: mov qword ptr [rsp + 0x10], rdx
140014405: push rbp
140014406: push r15
140014408: push r14
14001440a: push r13
14001440c: push r12
14001440e: push rsi
14001440f: push rdi
140014410: push rbx
140014411: sub rsp, 0x138
140014418: lea rbp, [rdx + 0x80]
14001441f: movaps xmmword ptr [rsp + 0x90], xmm15
140014428: movaps xmmword ptr [rsp + 0xa0], xmm14
140014431: movaps xmmword ptr [rsp + 0xb0], xmm13
14001443a: movaps xmmword ptr [rsp + 0xc0], xmm12
140014443: movaps xmmword ptr [rsp + 0xd0], xmm11
14001444c: movaps xmmword ptr [rsp + 0xe0], xmm10
140014455: movaps xmmword ptr [rsp + 0xf0], xmm9
14001445e: movaps xmmword ptr [rsp + 0x100], xmm8
140014467: movaps xmmword ptr [rsp + 0x110], xmm7
14001446f: movaps xmmword ptr [rsp + 0x120], xmm6
140014477: cmp qword ptr [rbp + 0x800], 0
14001447f: setne al
140014482: test byte ptr [rbp + 0x7de], al
140014488: je 0x1400144a7
14001448a: mov rdx, qword ptr [rbp + 0x800]
140014491: shl rdx, 2
140014495: mov r8d, 4
14001449b: mov rcx, qword ptr [rbp + 0x760]
1400144a2: call 0x140613c20 ; ?
1400144a7: movaps xmm6, xmmword ptr [rsp + 0x120]
1400144af: movaps xmm7, xmmword ptr [rsp + 0x110]
1400144b7: movaps xmm8, xmmword ptr [rsp + 0x100]
1400144c0: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400144c9: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400144d2: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400144db: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400144e4: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400144ed: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400144f6: movaps xmm15, xmmword ptr [rsp + 0x90]
1400144ff: add rsp, 0x138
140014506: pop rbx
140014507: pop rdi
140014508: pop rsi
140014509: pop r12
14001450b: pop r13
14001450d: pop r14
14001450f: pop r15
140014511: pop rbp
140014512: ret 
140014513: nop word ptr cs:[rax + rax]
140014520: mov qword ptr [rsp + 0x10], rdx
140014525: push rbp
140014526: push r15
140014528: push r14
14001452a: push r13
14001452c: push r12
14001452e: push rsi
14001452f: push rdi
140014530: push rbx
140014531: sub rsp, 0x138
140014538: lea rbp, [rdx + 0x80]
14001453f: movaps xmmword ptr [rsp + 0x90], xmm15
140014548: movaps xmmword ptr [rsp + 0xa0], xmm14
140014551: movaps xmmword ptr [rsp + 0xb0], xmm13
14001455a: movaps xmmword ptr [rsp + 0xc0], xmm12
140014563: movaps xmmword ptr [rsp + 0xd0], xmm11
14001456c: movaps xmmword ptr [rsp + 0xe0], xmm10
140014575: movaps xmmword ptr [rsp + 0xf0], xmm9
14001457e: movaps xmmword ptr [rsp + 0x100], xmm8
140014587: movaps xmmword ptr [rsp + 0x110], xmm7
14001458f: movaps xmmword ptr [rsp + 0x120], xmm6
140014597: cmp qword ptr [rbp + 0x800], 0
14001459f: setne al
1400145a2: test byte ptr [rbp + 0x7df], al
1400145a8: je 0x1400145c7
1400145aa: mov rdx, qword ptr [rbp + 0x800]
1400145b1: shl rdx, 2
1400145b5: mov r8d, 4
1400145bb: mov rcx, qword ptr [rbp + 0x760]
1400145c2: call 0x140613c20 ; ?
1400145c7: movaps xmm6, xmmword ptr [rsp + 0x120]
1400145cf: movaps xmm7, xmmword ptr [rsp + 0x110]
1400145d7: movaps xmm8, xmmword ptr [rsp + 0x100]
1400145e0: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400145e9: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400145f2: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400145fb: movaps xmm12, xmmword ptr [rsp + 0xc0]
140014604: movaps xmm13, xmmword ptr [rsp + 0xb0]
14001460d: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014616: movaps xmm15, xmmword ptr [rsp + 0x90]
14001461f: add rsp, 0x138
140014626: pop rbx
140014627: pop rdi
140014628: pop rsi
140014629: pop r12
14001462b: pop r13
14001462d: pop r14
14001462f: pop r15
140014631: pop rbp
140014632: ret 
140014633: nop word ptr cs:[rax + rax]
140014640: mov qword ptr [rsp + 0x10], rdx
140014645: push rbp
140014646: push r15
140014648: push r14
14001464a: push r13
14001464c: push r12
14001464e: push rsi
14001464f: push rdi
140014650: push rbx
140014651: sub rsp, 0x138
140014658: lea rbp, [rdx + 0x80]
14001465f: movdqa xmmword ptr [rsp + 0x90], xmm15
140014669: movaps xmmword ptr [rsp + 0xa0], xmm14
140014672: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001467c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140014686: movaps xmmword ptr [rsp + 0xd0], xmm11
14001468f: movaps xmmword ptr [rsp + 0xe0], xmm10
140014698: movaps xmmword ptr [rsp + 0xf0], xmm9
1400146a1: movaps xmmword ptr [rsp + 0x100], xmm8
1400146aa: movaps xmmword ptr [rsp + 0x110], xmm7
1400146b2: movaps xmmword ptr [rsp + 0x120], xmm6
1400146ba: cmp qword ptr [rbp + 0x800], 0
1400146c2: je 0x1400146e5
1400146c4: mov rax, qword ptr [rbp + 0x800]
1400146cb: shl rax, 2
1400146cf: lea rdx, [rax + rax*2]
1400146d3: mov r8d, 4
1400146d9: mov rcx, qword ptr [rbp + 0x780]
1400146e0: call 0x140613c20 ; ?
1400146e5: movaps xmm6, xmmword ptr [rsp + 0x120]
1400146ed: movaps xmm7, xmmword ptr [rsp + 0x110]
1400146f5: movaps xmm8, xmmword ptr [rsp + 0x100]
1400146fe: movaps xmm9, xmmword ptr [rsp + 0xf0]
140014707: movaps xmm10, xmmword ptr [rsp + 0xe0]
140014710: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014719: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014723: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001472d: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014736: movdqa xmm15, xmmword ptr [rsp + 0x90]
140014740: add rsp, 0x138
140014747: pop rbx
140014748: pop rdi
140014749: pop rsi
14001474a: pop r12
14001474c: pop r13
14001474e: pop r14
140014750: pop r15
140014752: pop rbp
140014753: ret 
140014754: nop word ptr cs:[rax + rax]
140014760: mov qword ptr [rsp + 0x10], rdx
140014765: push rbp
140014766: push r15
140014768: push r14
14001476a: push r13
14001476c: push r12
14001476e: push rsi
14001476f: push rdi
140014770: push rbx
140014771: sub rsp, 0x138
140014778: lea rbp, [rdx + 0x80]
14001477f: movdqa xmmword ptr [rsp + 0x90], xmm15
140014789: movaps xmmword ptr [rsp + 0xa0], xmm14
140014792: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001479c: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400147a6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400147af: movaps xmmword ptr [rsp + 0xe0], xmm10
1400147b8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400147c1: movaps xmmword ptr [rsp + 0x100], xmm8
1400147ca: movaps xmmword ptr [rsp + 0x110], xmm7
1400147d2: movaps xmmword ptr [rsp + 0x120], xmm6
1400147da: movaps xmm0, xmmword ptr [rbp + 0x4d0]
1400147e1: movaps xmmword ptr [rbp + 0x710], xmm0
1400147e8: movaps xmm0, xmmword ptr [rbp + 0x4c0]
1400147ef: movaps xmmword ptr [rbp + 0x700], xmm0
1400147f6: movaps xmm0, xmmword ptr [rbp + 0x4b0]
1400147fd: movaps xmmword ptr [rbp + 0x6f0], xmm0
140014804: movaps xmm0, xmmword ptr [rbp + 0x4a0]
14001480b: movaps xmmword ptr [rbp + 0x6e0], xmm0
140014812: movaps xmm0, xmmword ptr [rbp + 0x460]
140014819: movdqa xmm1, xmmword ptr [rbp + 0x470]
140014821: movdqa xmm2, xmmword ptr [rbp + 0x480]
140014829: movapd xmm3, xmmword ptr [rbp + 0x490]
140014831: movapd xmmword ptr [rbp + 0x6d0], xmm3
140014839: movdqa xmmword ptr [rbp + 0x6c0], xmm2
140014841: movdqa xmmword ptr [rbp + 0x6b0], xmm1
140014849: movaps xmmword ptr [rbp + 0x6a0], xmm0
140014850: mov rax, qword ptr [rbp + 0x7b8]
140014857: mov qword ptr [rbp + 0x720], rax
14001485e: mov eax, dword ptr [rbp + 0x588]
140014864: mov dword ptr [rbp + 0x728], eax
14001486a: mov eax, dword ptr [rbp + 0x560]
140014870: mov dword ptr [rbp + 0x72c], eax
140014876: mov eax, dword ptr [rbp + 0x580]
14001487c: mov dword ptr [rbp + 0x730], eax
140014882: movdqa xmm0, xmmword ptr [rbp + 0x290]
14001488a: movd eax, xmm0
14001488e: mov word ptr [rbp + 0x734], ax
140014895: mov word ptr [rbp + 0x736], 0
14001489e: movzx eax, word ptr [rbp + 0x430]
1400148a5: mov word ptr [rbp + 0x738], ax
1400148ac: mov word ptr [rbp + 0x73a], 0x300
1400148b5: mov rax, qword ptr [rbp + 0x330]
1400148bc: mov qword ptr [rbp + 0x740], rax
1400148c3: mov rax, qword ptr [rbp + 0x530]
1400148ca: mov qword ptr [rbp + 0x748], rax
1400148d1: mov byte ptr [rbp + 0x750], 0
1400148d8: mov byte ptr [rbp + 0x7d2], 0
1400148df: mov byte ptr [rbp + 0x7d1], 0
1400148e6: movaps xmm6, xmmword ptr [rsp + 0x120]
1400148ee: movaps xmm7, xmmword ptr [rsp + 0x110]
1400148f6: movaps xmm8, xmmword ptr [rsp + 0x100]
1400148ff: movaps xmm9, xmmword ptr [rsp + 0xf0]
140014908: movaps xmm10, xmmword ptr [rsp + 0xe0]
140014911: movaps xmm11, xmmword ptr [rsp + 0xd0]
14001491a: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014924: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001492e: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014937: movdqa xmm15, xmmword ptr [rsp + 0x90]
140014941: add rsp, 0x138
140014948: pop rbx
140014949: pop rdi
14001494a: pop rsi
14001494b: pop r12
14001494d: pop r13
14001494f: pop r14
140014951: pop r15
140014953: pop rbp
140014954: ret 
140014955: nop word ptr cs:[rax + rax]
140014960: mov qword ptr [rsp + 0x10], rdx
140014965: push rbp
140014966: push r15
140014968: push r14
14001496a: push r13
14001496c: push r12
14001496e: push rsi
14001496f: push rdi
140014970: push rbx
140014971: sub rsp, 0x138
140014978: lea rbp, [rdx + 0x80]
14001497f: movdqa xmmword ptr [rsp + 0x90], xmm15
140014989: movaps xmmword ptr [rsp + 0xa0], xmm14
140014992: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001499c: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400149a6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400149af: movaps xmmword ptr [rsp + 0xe0], xmm10
1400149b8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400149c1: movaps xmmword ptr [rsp + 0x100], xmm8
1400149ca: movaps xmmword ptr [rsp + 0x110], xmm7
1400149d2: movaps xmmword ptr [rsp + 0x120], xmm6
1400149da: lea rcx, [rbp + 0x6a0]
1400149e1: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
1400149e6: movaps xmm6, xmmword ptr [rsp + 0x120]
1400149ee: movaps xmm7, xmmword ptr [rsp + 0x110]
1400149f6: movaps xmm8, xmmword ptr [rsp + 0x100]
1400149ff: movaps xmm9, xmmword ptr [rsp + 0xf0]
140014a08: movaps xmm10, xmmword ptr [rsp + 0xe0]
140014a11: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014a1a: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014a24: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140014a2e: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014a37: movdqa xmm15, xmmword ptr [rsp + 0x90]
140014a41: add rsp, 0x138
140014a48: pop rbx
140014a49: pop rdi
140014a4a: pop rsi
140014a4b: pop r12
140014a4d: pop r13
140014a4f: pop r14
140014a51: pop r15
140014a53: pop rbp
140014a54: ret 
140014a55: nop word ptr cs:[rax + rax]
140014a60: mov qword ptr [rsp + 0x10], rdx
140014a65: push rbp
140014a66: push r15
140014a68: push r14
140014a6a: push r13
140014a6c: push r12
140014a6e: push rsi
140014a6f: push rdi
140014a70: push rbx
140014a71: sub rsp, 0x138
140014a78: lea rbp, [rdx + 0x80]
140014a7f: movdqa xmmword ptr [rsp + 0x90], xmm15
140014a89: movaps xmmword ptr [rsp + 0xa0], xmm14
140014a92: movdqa xmmword ptr [rsp + 0xb0], xmm13
140014a9c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140014aa6: movaps xmmword ptr [rsp + 0xd0], xmm11
140014aaf: movaps xmmword ptr [rsp + 0xe0], xmm10
140014ab8: movaps xmmword ptr [rsp + 0xf0], xmm9
140014ac1: movaps xmmword ptr [rsp + 0x100], xmm8
140014aca: movaps xmmword ptr [rsp + 0x110], xmm7
140014ad2: movaps xmmword ptr [rsp + 0x120], xmm6
140014ada: cmp qword ptr [rbp + 0x7f0], 0
140014ae2: je 0x140014b05
140014ae4: mov rax, qword ptr [rbp + 0x7f0]
140014aeb: shl rax, 2
140014aef: lea rdx, [rax + rax*2]
140014af3: mov r8d, 4
140014af9: mov rcx, qword ptr [rbp + 0x798]
140014b00: call 0x140613c20 ; ?
140014b05: movaps xmm6, xmmword ptr [rsp + 0x120]
140014b0d: movaps xmm7, xmmword ptr [rsp + 0x110]
140014b15: movaps xmm8, xmmword ptr [rsp + 0x100]
140014b1e: movaps xmm9, xmmword ptr [rsp + 0xf0]
140014b27: movaps xmm10, xmmword ptr [rsp + 0xe0]
140014b30: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014b39: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014b43: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140014b4d: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014b56: movdqa xmm15, xmmword ptr [rsp + 0x90]
140014b60: add rsp, 0x138
140014b67: pop rbx
140014b68: pop rdi
140014b69: pop rsi
140014b6a: pop r12
140014b6c: pop r13
140014b6e: pop r14
140014b70: pop r15
140014b72: pop rbp
140014b73: ret 
140014b74: nop word ptr cs:[rax + rax]
140014b80: mov qword ptr [rsp + 0x10], rdx
140014b85: push rbp
140014b86: push r15
140014b88: push r14
140014b8a: push r13
140014b8c: push r12
140014b8e: push rsi
140014b8f: push rdi
140014b90: push rbx
140014b91: sub rsp, 0x138
140014b98: lea rbp, [rdx + 0x80]
140014b9f: movdqa xmmword ptr [rsp + 0x90], xmm15
140014ba9: movaps xmmword ptr [rsp + 0xa0], xmm14
140014bb2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140014bbc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140014bc6: movaps xmmword ptr [rsp + 0xd0], xmm11
140014bcf: movaps xmmword ptr [rsp + 0xe0], xmm10
140014bd8: movaps xmmword ptr [rsp + 0xf0], xmm9
140014be1: movaps xmmword ptr [rsp + 0x100], xmm8
140014bea: movaps xmmword ptr [rsp + 0x110], xmm7
140014bf2: movaps xmmword ptr [rsp + 0x120], xmm6
140014bfa: cmp qword ptr [rbp + 0x7a8], 0
140014c02: je 0x140014c21
140014c04: mov rdx, qword ptr [rbp + 0x7a8]
140014c0b: shl rdx, 3
140014c0f: mov r8d, 4
140014c15: mov rcx, qword ptr [rbp + 0x308]
140014c1c: call 0x140613c20 ; ?
140014c21: movaps xmm6, xmmword ptr [rsp + 0x120]
140014c29: movaps xmm7, xmmword ptr [rsp + 0x110]
140014c31: movaps xmm8, xmmword ptr [rsp + 0x100]
140014c3a: movaps xmm9, xmmword ptr [rsp + 0xf0]
140014c43: movaps xmm10, xmmword ptr [rsp + 0xe0]
140014c4c: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014c55: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014c5f: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140014c69: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014c72: movdqa xmm15, xmmword ptr [rsp + 0x90]
140014c7c: add rsp, 0x138
140014c83: pop rbx
140014c84: pop rdi
140014c85: pop rsi
140014c86: pop r12
140014c88: pop r13
140014c8a: pop r14
140014c8c: pop r15
140014c8e: pop rbp
140014c8f: ret 
140014c90: mov qword ptr [rsp + 0x10], rdx
140014c95: push rbp
140014c96: push r15
140014c98: push r14
140014c9a: push r13
140014c9c: push r12
140014c9e: push rsi
140014c9f: push rdi
140014ca0: push rbx
140014ca1: sub rsp, 0x138
140014ca8: lea rbp, [rdx + 0x80]
140014caf: movdqa xmmword ptr [rsp + 0x90], xmm15
140014cb9: movaps xmmword ptr [rsp + 0xa0], xmm14
140014cc2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140014ccc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140014cd6: movaps xmmword ptr [rsp + 0xd0], xmm11
140014cdf: movaps xmmword ptr [rsp + 0xe0], xmm10
140014ce8: movaps xmmword ptr [rsp + 0xf0], xmm9
140014cf1: movaps xmmword ptr [rsp + 0x100], xmm8
140014cfa: movaps xmmword ptr [rsp + 0x110], xmm7
140014d02: movaps xmmword ptr [rsp + 0x120], xmm6
140014d0a: mov rdx, qword ptr [rbp + 0xd8]
140014d11: test rdx, rdx
140014d14: je 0x140014d2c
140014d16: mov rcx, qword ptr [rbp + 0xe0]
140014d1d: shl rdx, 2
140014d21: mov r8d, 4
140014d27: call 0x140613c20 ; ?
140014d2c: movaps xmm6, xmmword ptr [rsp + 0x120]
140014d34: movaps xmm7, xmmword ptr [rsp + 0x110]
140014d3c: movaps xmm8, xmmword ptr [rsp + 0x100]
140014d45: movaps xmm9, xmmword ptr [rsp + 0xf0]
140014d4e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140014d57: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014d60: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014d6a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140014d74: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014d7d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140014d87: add rsp, 0x138
140014d8e: pop rbx
140014d8f: pop rdi
140014d90: pop rsi
140014d91: pop r12
140014d93: pop r13
140014d95: pop r14
140014d97: pop r15
140014d99: pop rbp
140014d9a: ret 
140014d9b: nop dword ptr [rax + rax]
140014da0: mov qword ptr [rsp + 0x10], rdx
140014da5: push rbp
140014da6: push r15
140014da8: push r14
140014daa: push r13
140014dac: push r12
140014dae: push rsi
140014daf: push rdi
140014db0: push rbx
140014db1: sub rsp, 0x138
140014db8: lea rbp, [rdx + 0x80]
140014dbf: movdqa xmmword ptr [rsp + 0x90], xmm15
140014dc9: movaps xmmword ptr [rsp + 0xa0], xmm14
140014dd2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140014ddc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140014de6: movaps xmmword ptr [rsp + 0xd0], xmm11
140014def: movaps xmmword ptr [rsp + 0xe0], xmm10
140014df8: movaps xmmword ptr [rsp + 0xf0], xmm9
140014e01: movaps xmmword ptr [rsp + 0x100], xmm8
140014e0a: movaps xmmword ptr [rsp + 0x110], xmm7
140014e12: movaps xmmword ptr [rsp + 0x120], xmm6
140014e1a: mov rdx, qword ptr [rbp + 0xc8]
140014e21: test rdx, rdx
140014e24: je 0x140014e3c
140014e26: mov rcx, qword ptr [rbp + 0xd0]
140014e2d: shl rdx, 2
140014e31: mov r8d, 4
140014e37: call 0x140613c20 ; ?
140014e3c: movaps xmm6, xmmword ptr [rsp + 0x120]
140014e44: movaps xmm7, xmmword ptr [rsp + 0x110]
140014e4c: movaps xmm8, xmmword ptr [rsp + 0x100]
140014e55: movaps xmm9, xmmword ptr [rsp + 0xf0]
140014e5e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140014e67: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014e70: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014e7a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140014e84: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014e8d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140014e97: add rsp, 0x138
140014e9e: pop rbx
140014e9f: pop rdi
140014ea0: pop rsi
140014ea1: pop r12
140014ea3: pop r13
140014ea5: pop r14
140014ea7: pop r15
140014ea9: pop rbp
140014eaa: ret 
140014eab: nop dword ptr [rax + rax]
140014eb0: mov qword ptr [rsp + 0x10], rdx
140014eb5: push rbp
140014eb6: push r15
140014eb8: push r14
140014eba: push r13
140014ebc: push r12
140014ebe: push rsi
140014ebf: push rdi
140014ec0: push rbx
140014ec1: sub rsp, 0x138
140014ec8: lea rbp, [rdx + 0x80]
140014ecf: movdqa xmmword ptr [rsp + 0x90], xmm15
140014ed9: movaps xmmword ptr [rsp + 0xa0], xmm14
140014ee2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140014eec: movdqa xmmword ptr [rsp + 0xc0], xmm12
140014ef6: movaps xmmword ptr [rsp + 0xd0], xmm11
140014eff: movaps xmmword ptr [rsp + 0xe0], xmm10
140014f08: movaps xmmword ptr [rsp + 0xf0], xmm9
140014f11: movaps xmmword ptr [rsp + 0x100], xmm8
140014f1a: movaps xmmword ptr [rsp + 0x110], xmm7
140014f22: movaps xmmword ptr [rsp + 0x120], xmm6
140014f2a: mov rdx, qword ptr [rbp + 0x7a0]
140014f31: test rdx, rdx
140014f34: je 0x140014f4c
140014f36: shl rdx, 2
140014f3a: mov r8d, 4
140014f40: mov rcx, qword ptr [rbp + 0x138]
140014f47: call 0x140613c20 ; ?
140014f4c: movaps xmm6, xmmword ptr [rsp + 0x120]
140014f54: movaps xmm7, xmmword ptr [rsp + 0x110]
140014f5c: movaps xmm8, xmmword ptr [rsp + 0x100]
140014f65: movaps xmm9, xmmword ptr [rsp + 0xf0]
140014f6e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140014f77: movaps xmm11, xmmword ptr [rsp + 0xd0]
140014f80: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140014f8a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140014f94: movaps xmm14, xmmword ptr [rsp + 0xa0]
140014f9d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140014fa7: add rsp, 0x138
140014fae: pop rbx
140014faf: pop rdi
140014fb0: pop rsi
140014fb1: pop r12
140014fb3: pop r13
140014fb5: pop r14
140014fb7: pop r15
140014fb9: pop rbp
140014fba: ret 
140014fbb: nop dword ptr [rax + rax]
140014fc0: mov qword ptr [rsp + 0x10], rdx
140014fc5: push rbp
140014fc6: push r15
140014fc8: push r14
140014fca: push r13
140014fcc: push r12
140014fce: push rsi
140014fcf: push rdi
140014fd0: push rbx
140014fd1: sub rsp, 0x138
140014fd8: lea rbp, [rdx + 0x80]
140014fdf: movdqa xmmword ptr [rsp + 0x90], xmm15
140014fe9: movaps xmmword ptr [rsp + 0xa0], xmm14
140014ff2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140014ffc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140015006: movaps xmmword ptr [rsp + 0xd0], xmm11
14001500f: movaps xmmword ptr [rsp + 0xe0], xmm10
140015018: movaps xmmword ptr [rsp + 0xf0], xmm9
140015021: movaps xmmword ptr [rsp + 0x100], xmm8
14001502a: movaps xmmword ptr [rsp + 0x110], xmm7
140015032: movaps xmmword ptr [rsp + 0x120], xmm6
14001503a: movzx ebx, byte ptr [rbp + 0x7d1]
140015041: movzx esi, byte ptr [rbp + 0x7d2]
140015048: test sil, 1
14001504c: je 0x1400150fa
140015052: mov rdx, qword ptr [rbp + 0x3e0]
140015059: test rdx, rdx
14001505c: je 0x140015074
14001505e: mov rcx, qword ptr [rbp + 0x3e8]
140015065: shl rdx, 2
140015069: mov r8d, 4
14001506f: call 0x140613c20 ; ?
140015074: mov byte ptr [rbp + 0x7d5], sil
14001507b: and bl, 1
14001507e: mov byte ptr [rbp + 0x7d4], bl
140015084: mov byte ptr [rbp + 0x7d3], 0
14001508b: movaps xmm6, xmmword ptr [rsp + 0x120]
140015093: movaps xmm7, xmmword ptr [rsp + 0x110]
14001509b: movaps xmm8, xmmword ptr [rsp + 0x100]
1400150a4: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400150ad: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400150b6: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400150bf: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400150c9: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400150d3: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400150dc: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400150e6: add rsp, 0x138
1400150ed: pop rbx
1400150ee: pop rdi
1400150ef: pop rsi
1400150f0: pop r12
1400150f2: pop r13
1400150f4: pop r14
1400150f6: pop r15
1400150f8: pop rbp
1400150f9: ret 
1400150fa: mov byte ptr [rbp + 0x7d5], sil
140015101: and bl, 1
140015104: mov byte ptr [rbp + 0x7d4], bl
14001510a: mov byte ptr [rbp + 0x7d3], 0
140015111: movaps xmm6, xmmword ptr [rsp + 0x120]
140015119: movaps xmm7, xmmword ptr [rsp + 0x110]
140015121: movaps xmm8, xmmword ptr [rsp + 0x100]
14001512a: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015133: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001513c: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015145: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001514f: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015159: movaps xmm14, xmmword ptr [rsp + 0xa0]
140015162: movdqa xmm15, xmmword ptr [rsp + 0x90]
14001516c: add rsp, 0x138
140015173: pop rbx
140015174: pop rdi
140015175: pop rsi
140015176: pop r12
140015178: pop r13
14001517a: pop r14
14001517c: pop r15
14001517e: pop rbp
14001517f: ret 
140015180: mov qword ptr [rsp + 0x10], rdx
140015185: push rbp
140015186: push r15
140015188: push r14
14001518a: push r13
14001518c: push r12
14001518e: push rsi
14001518f: push rdi
140015190: push rbx
140015191: sub rsp, 0x138
140015198: lea rbp, [rdx + 0x80]
14001519f: movdqa xmmword ptr [rsp + 0x90], xmm15
1400151a9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400151b2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400151bc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400151c6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400151cf: movaps xmmword ptr [rsp + 0xe0], xmm10
1400151d8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400151e1: movaps xmmword ptr [rsp + 0x100], xmm8
1400151ea: movaps xmmword ptr [rsp + 0x110], xmm7
1400151f2: movaps xmmword ptr [rsp + 0x120], xmm6
1400151fa: mov rdx, qword ptr [rbp + 0x430]
140015201: test rdx, rdx
140015204: je 0x14001521c
140015206: shl rdx, 3
14001520a: mov r8d, 4
140015210: mov rcx, qword ptr [rbp + 0x300]
140015217: call 0x140613c20 ; ?
14001521c: movaps xmm6, xmmword ptr [rsp + 0x120]
140015224: movaps xmm7, xmmword ptr [rsp + 0x110]
14001522c: movaps xmm8, xmmword ptr [rsp + 0x100]
140015235: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001523e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015247: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015250: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001525a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015264: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001526d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015277: add rsp, 0x138
14001527e: pop rbx
14001527f: pop rdi
140015280: pop rsi
140015281: pop r12
140015283: pop r13
140015285: pop r14
140015287: pop r15
140015289: pop rbp
14001528a: ret 
14001528b: nop dword ptr [rax + rax]
140015290: mov qword ptr [rsp + 0x10], rdx
140015295: push rbp
140015296: push r15
140015298: push r14
14001529a: push r13
14001529c: push r12
14001529e: push rsi
14001529f: push rdi
1400152a0: push rbx
1400152a1: sub rsp, 0x138
1400152a8: lea rbp, [rdx + 0x80]
1400152af: movdqa xmmword ptr [rsp + 0x90], xmm15
1400152b9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400152c2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400152cc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400152d6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400152df: movaps xmmword ptr [rsp + 0xe0], xmm10
1400152e8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400152f1: movaps xmmword ptr [rsp + 0x100], xmm8
1400152fa: movaps xmmword ptr [rsp + 0x110], xmm7
140015302: movaps xmmword ptr [rsp + 0x120], xmm6
14001530a: lea rcx, [rbp + 0x590]
140015311: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140015316: mov byte ptr [rbp + 0x816], 1
14001531d: mov byte ptr [rbp + 0x815], 1
140015324: movaps xmm6, xmmword ptr [rsp + 0x120]
14001532c: movaps xmm7, xmmword ptr [rsp + 0x110]
140015334: movaps xmm8, xmmword ptr [rsp + 0x100]
14001533d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015346: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001534f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015358: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140015362: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001536c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140015375: movdqa xmm15, xmmword ptr [rsp + 0x90]
14001537f: add rsp, 0x138
140015386: pop rbx
140015387: pop rdi
140015388: pop rsi
140015389: pop r12
14001538b: pop r13
14001538d: pop r14
14001538f: pop r15
140015391: pop rbp
140015392: ret 
140015393: nop word ptr cs:[rax + rax]
1400153a0: mov qword ptr [rsp + 0x10], rdx
1400153a5: push rbp
1400153a6: push r15
1400153a8: push r14
1400153aa: push r13
1400153ac: push r12
1400153ae: push rsi
1400153af: push rdi
1400153b0: push rbx
1400153b1: sub rsp, 0x138
1400153b8: lea rbp, [rdx + 0x80]
1400153bf: movdqa xmmword ptr [rsp + 0x90], xmm15
1400153c9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400153d2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400153dc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400153e6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400153ef: movaps xmmword ptr [rsp + 0xe0], xmm10
1400153f8: movaps xmmword ptr [rsp + 0xf0], xmm9
140015401: movaps xmmword ptr [rsp + 0x100], xmm8
14001540a: movaps xmmword ptr [rsp + 0x110], xmm7
140015412: movaps xmmword ptr [rsp + 0x120], xmm6
14001541a: movzx ebx, byte ptr [rbp + 0x7d3]
140015421: movzx esi, byte ptr [rbp + 0x7d4]
140015428: movzx edi, byte ptr [rbp + 0x7d5]
14001542f: test dil, 1
140015433: je 0x1400154ec
140015439: mov rdx, qword ptr [rbp + 0x2a0]
140015440: test rdx, rdx
140015443: je 0x14001545b
140015445: mov rcx, qword ptr [rbp + 0x2a8]
14001544c: shl rdx, 2
140015450: mov r8d, 4
140015456: call 0x140613c20 ; ?
14001545b: mov byte ptr [rbp + 0x7d9], dil
140015462: and sil, 1
140015466: mov byte ptr [rbp + 0x7d8], sil
14001546d: and bl, 1
140015470: mov byte ptr [rbp + 0x7d7], bl
140015476: mov byte ptr [rbp + 0x7d6], 0
14001547d: movaps xmm6, xmmword ptr [rsp + 0x120]
140015485: movaps xmm7, xmmword ptr [rsp + 0x110]
14001548d: movaps xmm8, xmmword ptr [rsp + 0x100]
140015496: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001549f: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400154a8: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400154b1: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400154bb: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400154c5: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400154ce: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400154d8: add rsp, 0x138
1400154df: pop rbx
1400154e0: pop rdi
1400154e1: pop rsi
1400154e2: pop r12
1400154e4: pop r13
1400154e6: pop r14
1400154e8: pop r15
1400154ea: pop rbp
1400154eb: ret 
1400154ec: mov byte ptr [rbp + 0x7d9], dil
1400154f3: and sil, 1
1400154f7: mov byte ptr [rbp + 0x7d8], sil
1400154fe: and bl, 1
140015501: mov byte ptr [rbp + 0x7d7], bl
140015507: mov byte ptr [rbp + 0x7d6], 0
14001550e: movaps xmm6, xmmword ptr [rsp + 0x120]
140015516: movaps xmm7, xmmword ptr [rsp + 0x110]
14001551e: movaps xmm8, xmmword ptr [rsp + 0x100]
140015527: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015530: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015539: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015542: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001554c: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015556: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001555f: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015569: add rsp, 0x138
140015570: pop rbx
140015571: pop rdi
140015572: pop rsi
140015573: pop r12
140015575: pop r13
140015577: pop r14
140015579: pop r15
14001557b: pop rbp
14001557c: ret 
14001557d: nop dword ptr [rax]
140015580: mov qword ptr [rsp + 0x10], rdx
140015585: push rbp
140015586: push r15
140015588: push r14
14001558a: push r13
14001558c: push r12
14001558e: push rsi
14001558f: push rdi
140015590: push rbx
140015591: sub rsp, 0x138
140015598: lea rbp, [rdx + 0x80]
14001559f: movdqa xmmword ptr [rsp + 0x90], xmm15
1400155a9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400155b2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400155bc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400155c6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400155cf: movaps xmmword ptr [rsp + 0xe0], xmm10
1400155d8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400155e1: movaps xmmword ptr [rsp + 0x100], xmm8
1400155ea: movaps xmmword ptr [rsp + 0x110], xmm7
1400155f2: movaps xmmword ptr [rsp + 0x120], xmm6
1400155fa: lea rcx, [rbp + 0x3e0]
140015601: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140015606: mov byte ptr [rbp + 0x813], 1
14001560d: movaps xmm6, xmmword ptr [rsp + 0x120]
140015615: movaps xmm7, xmmword ptr [rsp + 0x110]
14001561d: movaps xmm8, xmmword ptr [rsp + 0x100]
140015626: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001562f: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015638: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015641: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001564b: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015655: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001565e: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015668: add rsp, 0x138
14001566f: pop rbx
140015670: pop rdi
140015671: pop rsi
140015672: pop r12
140015674: pop r13
140015676: pop r14
140015678: pop r15
14001567a: pop rbp
14001567b: ret 
14001567c: nop dword ptr [rax]
140015680: mov qword ptr [rsp + 0x10], rdx
140015685: push rbp
140015686: push r15
140015688: push r14
14001568a: push r13
14001568c: push r12
14001568e: push rsi
14001568f: push rdi
140015690: push rbx
140015691: sub rsp, 0x138
140015698: lea rbp, [rdx + 0x80]
14001569f: movdqa xmmword ptr [rsp + 0x90], xmm15
1400156a9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400156b2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400156bc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400156c6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400156cf: movaps xmmword ptr [rsp + 0xe0], xmm10
1400156d8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400156e1: movaps xmmword ptr [rsp + 0x100], xmm8
1400156ea: movaps xmmword ptr [rsp + 0x110], xmm7
1400156f2: movaps xmmword ptr [rsp + 0x120], xmm6
1400156fa: mov rdx, qword ptr [rbp + 0x588]
140015701: test rdx, rdx
140015704: je 0x14001571c
140015706: shl rdx, 3
14001570a: mov r8d, 4
140015710: mov rcx, qword ptr [rbp + 0x2f8]
140015717: call 0x140613c20 ; ?
14001571c: movaps xmm6, xmmword ptr [rsp + 0x120]
140015724: movaps xmm7, xmmword ptr [rsp + 0x110]
14001572c: movaps xmm8, xmmword ptr [rsp + 0x100]
140015735: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001573e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015747: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015750: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001575a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015764: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001576d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015777: add rsp, 0x138
14001577e: pop rbx
14001577f: pop rdi
140015780: pop rsi
140015781: pop r12
140015783: pop r13
140015785: pop r14
140015787: pop r15
140015789: pop rbp
14001578a: ret 
14001578b: nop dword ptr [rax + rax]
140015790: mov qword ptr [rsp + 0x10], rdx
140015795: push rbp
140015796: push r15
140015798: push r14
14001579a: push r13
14001579c: push r12
14001579e: push rsi
14001579f: push rdi
1400157a0: push rbx
1400157a1: sub rsp, 0x138
1400157a8: lea rbp, [rdx + 0x80]
1400157af: movdqa xmmword ptr [rsp + 0x90], xmm15
1400157b9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400157c2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400157cc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400157d6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400157df: movaps xmmword ptr [rsp + 0xe0], xmm10
1400157e8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400157f1: movaps xmmword ptr [rsp + 0x100], xmm8
1400157fa: movaps xmmword ptr [rsp + 0x110], xmm7
140015802: movaps xmmword ptr [rsp + 0x120], xmm6
14001580a: movzx ebx, byte ptr [rbp + 0x7d6]
140015811: movzx esi, byte ptr [rbp + 0x7d7]
140015818: movzx edi, byte ptr [rbp + 0x7d8]
14001581f: test byte ptr [rbp + 0x7d9], 1
140015826: je 0x1400158e3
14001582c: mov rdx, qword ptr [rbp + 0x680]
140015833: test rdx, rdx
140015836: je 0x14001584e
140015838: mov rcx, qword ptr [rbp + 0x688]
14001583f: shl rdx, 3
140015843: mov r8d, 4
140015849: call 0x140613c20 ; ?
14001584e: and dil, 1
140015852: mov byte ptr [rbp + 0x7dd], dil
140015859: and sil, 1
14001585d: mov byte ptr [rbp + 0x7dc], sil
140015864: and bl, 1
140015867: mov byte ptr [rbp + 0x7db], bl
14001586d: mov byte ptr [rbp + 0x7da], 0
140015874: movaps xmm6, xmmword ptr [rsp + 0x120]
14001587c: movaps xmm7, xmmword ptr [rsp + 0x110]
140015884: movaps xmm8, xmmword ptr [rsp + 0x100]
14001588d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015896: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001589f: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400158a8: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400158b2: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400158bc: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400158c5: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400158cf: add rsp, 0x138
1400158d6: pop rbx
1400158d7: pop rdi
1400158d8: pop rsi
1400158d9: pop r12
1400158db: pop r13
1400158dd: pop r14
1400158df: pop r15
1400158e1: pop rbp
1400158e2: ret 
1400158e3: and dil, 1
1400158e7: mov byte ptr [rbp + 0x7dd], dil
1400158ee: and sil, 1
1400158f2: mov byte ptr [rbp + 0x7dc], sil
1400158f9: and bl, 1
1400158fc: mov byte ptr [rbp + 0x7db], bl
140015902: mov byte ptr [rbp + 0x7da], 0
140015909: movaps xmm6, xmmword ptr [rsp + 0x120]
140015911: movaps xmm7, xmmword ptr [rsp + 0x110]
140015919: movaps xmm8, xmmword ptr [rsp + 0x100]
140015922: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001592b: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015934: movaps xmm11, xmmword ptr [rsp + 0xd0]
14001593d: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140015947: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015951: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001595a: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015964: add rsp, 0x138
14001596b: pop rbx
14001596c: pop rdi
14001596d: pop rsi
14001596e: pop r12
140015970: pop r13
140015972: pop r14
140015974: pop r15
140015976: pop rbp
140015977: ret 
140015978: nop dword ptr [rax + rax]
140015980: mov qword ptr [rsp + 0x10], rdx
140015985: push rbp
140015986: push r15
140015988: push r14
14001598a: push r13
14001598c: push r12
14001598e: push rsi
14001598f: push rdi
140015990: push rbx
140015991: sub rsp, 0x138
140015998: lea rbp, [rdx + 0x80]
14001599f: movdqa xmmword ptr [rsp + 0x90], xmm15
1400159a9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400159b2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400159bc: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400159c6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400159cf: movaps xmmword ptr [rsp + 0xe0], xmm10
1400159d8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400159e1: movaps xmmword ptr [rsp + 0x100], xmm8
1400159ea: movaps xmmword ptr [rsp + 0x110], xmm7
1400159f2: movaps xmmword ptr [rsp + 0x120], xmm6
1400159fa: movzx ebx, byte ptr [rbp + 0x7da]
140015a01: movzx esi, byte ptr [rbp + 0x7db]
140015a08: movzx edi, byte ptr [rbp + 0x7dc]
140015a0f: cmp byte ptr [rbp + 0x7dd], 0
140015a16: je 0x140015a3e
140015a18: mov rax, qword ptr [rbp + 0x650]
140015a1f: test rax, rax
140015a22: je 0x140015a3e
140015a24: mov rcx, qword ptr [rbp + 0x658]
140015a2b: shl rax, 2
140015a2f: lea rdx, [rax + rax*2]
140015a33: mov r8d, 4
140015a39: call 0x140613c20 ; ?
140015a3e: mov rdx, qword ptr [rbp + 0x7a8]
140015a45: test rdx, rdx
140015a48: je 0x140015a60
140015a4a: shl rdx, 2
140015a4e: mov r8d, 4
140015a54: mov rcx, qword ptr [rbp + 0x568]
140015a5b: call 0x140613c20 ; ?
140015a60: mov rdx, qword ptr [rbp + 0x578]
140015a67: test rdx, rdx
140015a6a: setne al
140015a6d: test bl, al
140015a6f: je 0x140015a87
140015a71: shl rdx, 3
140015a75: mov r8d, 4
140015a7b: mov rcx, qword ptr [rbp + 0x668]
140015a82: call 0x140613c20 ; ?
140015a87: mov rdx, qword ptr [rbp + 0x520]
140015a8e: test rdx, rdx
140015a91: setne al
140015a94: test sil, al
140015a97: je 0x140015aaf
140015a99: shl rdx, 2
140015a9d: mov r8d, 4
140015aa3: mov rcx, qword ptr [rbp + 0x7a0]
140015aaa: call 0x140613c20 ; ?
140015aaf: test dil, 1
140015ab3: je 0x140015b73
140015ab9: mov rdx, qword ptr [rbp + 0x670]
140015ac0: test rdx, rdx
140015ac3: je 0x140015adb
140015ac5: shl rdx, 2
140015ac9: mov r8d, 4
140015acf: mov rcx, qword ptr [rbp + 0x798]
140015ad6: call 0x140613c20 ; ?
140015adb: mov byte ptr [rbp + 0x7ee], 1
140015ae2: and sil, 1
140015ae6: mov byte ptr [rbp + 0x7ed], sil
140015aed: and bl, 1
140015af0: mov byte ptr [rbp + 0x7ec], bl
140015af6: mov byte ptr [rbp + 0x7eb], 0
140015afd: mov byte ptr [rbp + 0x7ea], 1
140015b04: movaps xmm6, xmmword ptr [rsp + 0x120]
140015b0c: movaps xmm7, xmmword ptr [rsp + 0x110]
140015b14: movaps xmm8, xmmword ptr [rsp + 0x100]
140015b1d: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015b26: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015b2f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015b38: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140015b42: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015b4c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140015b55: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015b5f: add rsp, 0x138
140015b66: pop rbx
140015b67: pop rdi
140015b68: pop rsi
140015b69: pop r12
140015b6b: pop r13
140015b6d: pop r14
140015b6f: pop r15
140015b71: pop rbp
140015b72: ret 
140015b73: mov byte ptr [rbp + 0x7ee], 0
140015b7a: and sil, 1
140015b7e: mov byte ptr [rbp + 0x7ed], sil
140015b85: and bl, 1
140015b88: mov byte ptr [rbp + 0x7ec], bl
140015b8e: mov byte ptr [rbp + 0x7eb], 0
140015b95: mov byte ptr [rbp + 0x7ea], 1
140015b9c: movaps xmm6, xmmword ptr [rsp + 0x120]
140015ba4: movaps xmm7, xmmword ptr [rsp + 0x110]
140015bac: movaps xmm8, xmmword ptr [rsp + 0x100]
140015bb5: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015bbe: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015bc7: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015bd0: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140015bda: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015be4: movaps xmm14, xmmword ptr [rsp + 0xa0]
140015bed: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015bf7: add rsp, 0x138
140015bfe: pop rbx
140015bff: pop rdi
140015c00: pop rsi
140015c01: pop r12
140015c03: pop r13
140015c05: pop r14
140015c07: pop r15
140015c09: pop rbp
140015c0a: ret 
140015c0b: nop dword ptr [rax + rax]
140015c10: mov qword ptr [rsp + 0x10], rdx
140015c15: push rbp
140015c16: push r15
140015c18: push r14
140015c1a: push r13
140015c1c: push r12
140015c1e: push rsi
140015c1f: push rdi
140015c20: push rbx
140015c21: sub rsp, 0x138
140015c28: lea rbp, [rdx + 0x80]
140015c2f: movdqa xmmword ptr [rsp + 0x90], xmm15
140015c39: movaps xmmword ptr [rsp + 0xa0], xmm14
140015c42: movdqa xmmword ptr [rsp + 0xb0], xmm13
140015c4c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140015c56: movaps xmmword ptr [rsp + 0xd0], xmm11
140015c5f: movaps xmmword ptr [rsp + 0xe0], xmm10
140015c68: movaps xmmword ptr [rsp + 0xf0], xmm9
140015c71: movaps xmmword ptr [rsp + 0x100], xmm8
140015c7a: movaps xmmword ptr [rsp + 0x110], xmm7
140015c82: movaps xmmword ptr [rsp + 0x120], xmm6
140015c8a: lea rcx, [rbp + 0x380]
140015c91: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
140015c96: movaps xmm6, xmmword ptr [rsp + 0x120]
140015c9e: movaps xmm7, xmmword ptr [rsp + 0x110]
140015ca6: movaps xmm8, xmmword ptr [rsp + 0x100]
140015caf: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015cb8: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015cc1: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015cca: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140015cd4: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015cde: movaps xmm14, xmmword ptr [rsp + 0xa0]
140015ce7: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015cf1: add rsp, 0x138
140015cf8: pop rbx
140015cf9: pop rdi
140015cfa: pop rsi
140015cfb: pop r12
140015cfd: pop r13
140015cff: pop r14
140015d01: pop r15
140015d03: pop rbp
140015d04: ret 
140015d05: nop word ptr cs:[rax + rax]
140015d10: mov qword ptr [rsp + 0x10], rdx
140015d15: push rbp
140015d16: push r15
140015d18: push r14
140015d1a: push r13
140015d1c: push r12
140015d1e: push rsi
140015d1f: push rdi
140015d20: push rbx
140015d21: sub rsp, 0x138
140015d28: lea rbp, [rdx + 0x80]
140015d2f: movdqa xmmword ptr [rsp + 0x90], xmm15
140015d39: movaps xmmword ptr [rsp + 0xa0], xmm14
140015d42: movdqa xmmword ptr [rsp + 0xb0], xmm13
140015d4c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140015d56: movaps xmmword ptr [rsp + 0xd0], xmm11
140015d5f: movaps xmmword ptr [rsp + 0xe0], xmm10
140015d68: movaps xmmword ptr [rsp + 0xf0], xmm9
140015d71: movaps xmmword ptr [rsp + 0x100], xmm8
140015d7a: movaps xmmword ptr [rsp + 0x110], xmm7
140015d82: movaps xmmword ptr [rsp + 0x120], xmm6
140015d8a: mov rdx, qword ptr [rbp + 0x590]
140015d91: test rdx, rdx
140015d94: je 0x140015dac
140015d96: mov rcx, qword ptr [rbp + 0x598]
140015d9d: shl rdx, 2
140015da1: mov r8d, 4
140015da7: call 0x140613c20 ; ?
140015dac: movaps xmm6, xmmword ptr [rsp + 0x120]
140015db4: movaps xmm7, xmmword ptr [rsp + 0x110]
140015dbc: movaps xmm8, xmmword ptr [rsp + 0x100]
140015dc5: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015dce: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015dd7: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015de0: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140015dea: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015df4: movaps xmm14, xmmword ptr [rsp + 0xa0]
140015dfd: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015e07: add rsp, 0x138
140015e0e: pop rbx
140015e0f: pop rdi
140015e10: pop rsi
140015e11: pop r12
140015e13: pop r13
140015e15: pop r14
140015e17: pop r15
140015e19: pop rbp
140015e1a: ret 
140015e1b: nop dword ptr [rax + rax]
140015e20: mov qword ptr [rsp + 0x10], rdx
140015e25: push rbp
140015e26: push r15
140015e28: push r14
140015e2a: push r13
140015e2c: push r12
140015e2e: push rsi
140015e2f: push rdi
140015e30: push rbx
140015e31: sub rsp, 0x138
140015e38: lea rbp, [rdx + 0x80]
140015e3f: movdqa xmmword ptr [rsp + 0x90], xmm15
140015e49: movaps xmmword ptr [rsp + 0xa0], xmm14
140015e52: movdqa xmmword ptr [rsp + 0xb0], xmm13
140015e5c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140015e66: movaps xmmword ptr [rsp + 0xd0], xmm11
140015e6f: movaps xmmword ptr [rsp + 0xe0], xmm10
140015e78: movaps xmmword ptr [rsp + 0xf0], xmm9
140015e81: movaps xmmword ptr [rsp + 0x100], xmm8
140015e8a: movaps xmmword ptr [rsp + 0x110], xmm7
140015e92: movaps xmmword ptr [rsp + 0x120], xmm6
140015e9a: mov rax, qword ptr [rbp + 0x500]
140015ea1: test rax, rax
140015ea4: je 0x140015ec0
140015ea6: mov rcx, qword ptr [rbp + 0x508]
140015ead: shl rax, 2
140015eb1: lea rdx, [rax + rax*2]
140015eb5: mov r8d, 4
140015ebb: call 0x140613c20 ; ?
140015ec0: movaps xmm6, xmmword ptr [rsp + 0x120]
140015ec8: movaps xmm7, xmmword ptr [rsp + 0x110]
140015ed0: movaps xmm8, xmmword ptr [rsp + 0x100]
140015ed9: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015ee2: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015eeb: movaps xmm11, xmmword ptr [rsp + 0xd0]
140015ef4: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140015efe: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140015f08: movaps xmm14, xmmword ptr [rsp + 0xa0]
140015f11: movdqa xmm15, xmmword ptr [rsp + 0x90]
140015f1b: add rsp, 0x138
140015f22: pop rbx
140015f23: pop rdi
140015f24: pop rsi
140015f25: pop r12
140015f27: pop r13
140015f29: pop r14
140015f2b: pop r15
140015f2d: pop rbp
140015f2e: ret 
140015f2f: nop 
140015f30: mov qword ptr [rsp + 0x10], rdx
140015f35: push rbp
140015f36: push r15
140015f38: push r14
140015f3a: push r13
140015f3c: push r12
140015f3e: push rsi
140015f3f: push rdi
140015f40: push rbx
140015f41: sub rsp, 0x138
140015f48: lea rbp, [rdx + 0x80]
140015f4f: movdqa xmmword ptr [rsp + 0x90], xmm15
140015f59: movaps xmmword ptr [rsp + 0xa0], xmm14
140015f62: movdqa xmmword ptr [rsp + 0xb0], xmm13
140015f6c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140015f76: movaps xmmword ptr [rsp + 0xd0], xmm11
140015f7f: movaps xmmword ptr [rsp + 0xe0], xmm10
140015f88: movaps xmmword ptr [rsp + 0xf0], xmm9
140015f91: movaps xmmword ptr [rsp + 0x100], xmm8
140015f9a: movaps xmmword ptr [rsp + 0x110], xmm7
140015fa2: movaps xmmword ptr [rsp + 0x120], xmm6
140015faa: mov rdx, qword ptr [rbp + 0x680]
140015fb1: test rdx, rdx
140015fb4: je 0x140015fcc
140015fb6: mov rcx, qword ptr [rbp + 0x688]
140015fbd: shl rdx, 3
140015fc1: mov r8d, 4
140015fc7: call 0x140613c20 ; ?
140015fcc: movaps xmm6, xmmword ptr [rsp + 0x120]
140015fd4: movaps xmm7, xmmword ptr [rsp + 0x110]
140015fdc: movaps xmm8, xmmword ptr [rsp + 0x100]
140015fe5: movaps xmm9, xmmword ptr [rsp + 0xf0]
140015fee: movaps xmm10, xmmword ptr [rsp + 0xe0]
140015ff7: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016000: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001600a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140016014: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001601d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140016027: add rsp, 0x138
14001602e: pop rbx
14001602f: pop rdi
140016030: pop rsi
140016031: pop r12
140016033: pop r13
140016035: pop r14
140016037: pop r15
140016039: pop rbp
14001603a: ret 
14001603b: nop dword ptr [rax + rax]
140016040: mov qword ptr [rsp + 0x10], rdx
140016045: push rbp
140016046: push r15
140016048: push r14
14001604a: push r13
14001604c: push r12
14001604e: push rsi
14001604f: push rdi
140016050: push rbx
140016051: sub rsp, 0x138
140016058: lea rbp, [rdx + 0x80]
14001605f: movdqa xmmword ptr [rsp + 0x90], xmm15
140016069: movaps xmmword ptr [rsp + 0xa0], xmm14
140016072: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001607c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140016086: movaps xmmword ptr [rsp + 0xd0], xmm11
14001608f: movaps xmmword ptr [rsp + 0xe0], xmm10
140016098: movaps xmmword ptr [rsp + 0xf0], xmm9
1400160a1: movaps xmmword ptr [rsp + 0x100], xmm8
1400160aa: movaps xmmword ptr [rsp + 0x110], xmm7
1400160b2: movaps xmmword ptr [rsp + 0x120], xmm6
1400160ba: mov rdx, qword ptr [rbp + 0x650]
1400160c1: test rdx, rdx
1400160c4: je 0x1400160dc
1400160c6: mov rcx, qword ptr [rbp + 0x658]
1400160cd: shl rdx, 2
1400160d1: mov r8d, 4
1400160d7: call 0x140613c20 ; ?
1400160dc: movaps xmm6, xmmword ptr [rsp + 0x120]
1400160e4: movaps xmm7, xmmword ptr [rsp + 0x110]
1400160ec: movaps xmm8, xmmword ptr [rsp + 0x100]
1400160f5: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400160fe: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016107: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016110: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001611a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140016124: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001612d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140016137: add rsp, 0x138
14001613e: pop rbx
14001613f: pop rdi
140016140: pop rsi
140016141: pop r12
140016143: pop r13
140016145: pop r14
140016147: pop r15
140016149: pop rbp
14001614a: ret 
14001614b: nop dword ptr [rax + rax]
140016150: mov qword ptr [rsp + 0x10], rdx
140016155: push rbp
140016156: push r15
140016158: push r14
14001615a: push r13
14001615c: push r12
14001615e: push rsi
14001615f: push rdi
140016160: push rbx
140016161: sub rsp, 0x138
140016168: lea rbp, [rdx + 0x80]
14001616f: movdqa xmmword ptr [rsp + 0x90], xmm15
140016179: movaps xmmword ptr [rsp + 0xa0], xmm14
140016182: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001618c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140016196: movaps xmmword ptr [rsp + 0xd0], xmm11
14001619f: movaps xmmword ptr [rsp + 0xe0], xmm10
1400161a8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400161b1: movaps xmmword ptr [rsp + 0x100], xmm8
1400161ba: movaps xmmword ptr [rsp + 0x110], xmm7
1400161c2: movaps xmmword ptr [rsp + 0x120], xmm6
1400161ca: lea rcx, [rbp + 0x460]
1400161d1: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
1400161d6: movaps xmm6, xmmword ptr [rsp + 0x120]
1400161de: movaps xmm7, xmmword ptr [rsp + 0x110]
1400161e6: movaps xmm8, xmmword ptr [rsp + 0x100]
1400161ef: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400161f8: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016201: movaps xmm11, xmmword ptr [rsp + 0xd0]
14001620a: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140016214: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001621e: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016227: movdqa xmm15, xmmword ptr [rsp + 0x90]
140016231: add rsp, 0x138
140016238: pop rbx
140016239: pop rdi
14001623a: pop rsi
14001623b: pop r12
14001623d: pop r13
14001623f: pop r14
140016241: pop r15
140016243: pop rbp
140016244: ret 
140016245: nop word ptr cs:[rax + rax]
140016250: mov qword ptr [rsp + 0x10], rdx
140016255: push rbp
140016256: push r15
140016258: push r14
14001625a: push r13
14001625c: push r12
14001625e: push rsi
14001625f: push rdi
140016260: push rbx
140016261: sub rsp, 0x138
140016268: lea rbp, [rdx + 0x80]
14001626f: movdqa xmmword ptr [rsp + 0x90], xmm15
140016279: movaps xmmword ptr [rsp + 0xa0], xmm14
140016282: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001628c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140016296: movaps xmmword ptr [rsp + 0xd0], xmm11
14001629f: movaps xmmword ptr [rsp + 0xe0], xmm10
1400162a8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400162b1: movaps xmmword ptr [rsp + 0x100], xmm8
1400162ba: movaps xmmword ptr [rsp + 0x110], xmm7
1400162c2: movaps xmmword ptr [rsp + 0x120], xmm6
1400162ca: mov byte ptr [rbp + 0x817], 1
1400162d1: lea rcx, [rbp + 0x380]
1400162d8: call 0x14014ba20 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>
1400162dd: mov byte ptr [rbp + 0x817], 1
1400162e4: movaps xmm6, xmmword ptr [rsp + 0x120]
1400162ec: movaps xmm7, xmmword ptr [rsp + 0x110]
1400162f4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400162fd: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016306: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001630f: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016318: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140016322: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001632c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016335: movdqa xmm15, xmmword ptr [rsp + 0x90]
14001633f: add rsp, 0x138
140016346: pop rbx
140016347: pop rdi
140016348: pop rsi
140016349: pop r12
14001634b: pop r13
14001634d: pop r14
14001634f: pop r15
140016351: pop rbp
140016352: ret 
140016353: nop word ptr cs:[rax + rax]
140016360: mov qword ptr [rsp + 0x10], rdx
140016365: push rbp
140016366: push r15
140016368: push r14
14001636a: push r13
14001636c: push r12
14001636e: push rsi
14001636f: push rdi
140016370: push rbx
140016371: sub rsp, 0x138
140016378: lea rbp, [rdx + 0x80]
14001637f: movdqa xmmword ptr [rsp + 0x90], xmm15
140016389: movaps xmmword ptr [rsp + 0xa0], xmm14
140016392: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001639c: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400163a6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400163af: movaps xmmword ptr [rsp + 0xe0], xmm10
1400163b8: movaps xmmword ptr [rsp + 0xf0], xmm9
1400163c1: movaps xmmword ptr [rsp + 0x100], xmm8
1400163ca: movaps xmmword ptr [rsp + 0x110], xmm7
1400163d2: movaps xmmword ptr [rsp + 0x120], xmm6
1400163da: movzx eax, byte ptr [rbp + 0x815]
1400163e1: movzx ecx, byte ptr [rbp + 0x816]
1400163e8: mov byte ptr [rbp + 0x800], al
1400163ee: test al, al
1400163f0: mov byte ptr [rbp + 0x780], cl
1400163f6: je 0x14001648a
1400163fc: mov rdx, qword ptr [rbp + 0x798]
140016403: test rdx, rdx
140016406: je 0x14001641e
140016408: shl rdx, 2
14001640c: mov r8d, 4
140016412: mov rcx, qword ptr [rbp + 0x7f0]
140016419: call 0x140613c20 ; ?
14001641e: movaps xmm6, xmmword ptr [rsp + 0x120]
140016426: movaps xmm7, xmmword ptr [rsp + 0x110]
14001642e: movaps xmm8, xmmword ptr [rsp + 0x100]
140016437: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016440: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016449: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016452: movaps xmm12, xmmword ptr [rsp + 0xc0]
14001645b: movaps xmm13, xmmword ptr [rsp + 0xb0]
140016464: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001646d: movaps xmm15, xmmword ptr [rsp + 0x90]
140016476: add rsp, 0x138
14001647d: pop rbx
14001647e: pop rdi
14001647f: pop rsi
140016480: pop r12
140016482: pop r13
140016484: pop r14
140016486: pop r15
140016488: pop rbp
140016489: ret 
14001648a: movaps xmm6, xmmword ptr [rsp + 0x120]
140016492: movaps xmm7, xmmword ptr [rsp + 0x110]
14001649a: movaps xmm8, xmmword ptr [rsp + 0x100]
1400164a3: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400164ac: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400164b5: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400164be: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400164c7: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400164d0: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400164d9: movaps xmm15, xmmword ptr [rsp + 0x90]
1400164e2: add rsp, 0x138
1400164e9: pop rbx
1400164ea: pop rdi
1400164eb: pop rsi
1400164ec: pop r12
1400164ee: pop r13
1400164f0: pop r14
1400164f2: pop r15
1400164f4: pop rbp
1400164f5: ret 
1400164f6: nop word ptr cs:[rax + rax]
140016500: mov qword ptr [rsp + 0x10], rdx
140016505: push rbp
140016506: push r15
140016508: push r14
14001650a: push r13
14001650c: push r12
14001650e: push rsi
14001650f: push rdi
140016510: push rbx
140016511: sub rsp, 0x138
140016518: lea rbp, [rdx + 0x80]
14001651f: movaps xmmword ptr [rsp + 0x90], xmm15
140016528: movaps xmmword ptr [rsp + 0xa0], xmm14
140016531: movaps xmmword ptr [rsp + 0xb0], xmm13
14001653a: movaps xmmword ptr [rsp + 0xc0], xmm12
140016543: movaps xmmword ptr [rsp + 0xd0], xmm11
14001654c: movaps xmmword ptr [rsp + 0xe0], xmm10
140016555: movaps xmmword ptr [rsp + 0xf0], xmm9
14001655e: movaps xmmword ptr [rsp + 0x100], xmm8
140016567: movaps xmmword ptr [rsp + 0x110], xmm7
14001656f: movaps xmmword ptr [rsp + 0x120], xmm6
140016577: test byte ptr [rbp + 0x780], 1
14001657e: je 0x140016616
140016584: mov rax, qword ptr [rbp + 0x680]
14001658b: test rax, rax
14001658e: je 0x1400165aa
140016590: mov rcx, qword ptr [rbp + 0x688]
140016597: shl rax, 2
14001659b: lea rdx, [rax + rax*2]
14001659f: mov r8d, 4
1400165a5: call 0x140613c20 ; ?
1400165aa: movaps xmm6, xmmword ptr [rsp + 0x120]
1400165b2: movaps xmm7, xmmword ptr [rsp + 0x110]
1400165ba: movaps xmm8, xmmword ptr [rsp + 0x100]
1400165c3: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400165cc: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400165d5: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400165de: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400165e7: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400165f0: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400165f9: movaps xmm15, xmmword ptr [rsp + 0x90]
140016602: add rsp, 0x138
140016609: pop rbx
14001660a: pop rdi
14001660b: pop rsi
14001660c: pop r12
14001660e: pop r13
140016610: pop r14
140016612: pop r15
140016614: pop rbp
140016615: ret 
140016616: movaps xmm6, xmmword ptr [rsp + 0x120]
14001661e: movaps xmm7, xmmword ptr [rsp + 0x110]
140016626: movaps xmm8, xmmword ptr [rsp + 0x100]
14001662f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016638: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016641: movaps xmm11, xmmword ptr [rsp + 0xd0]
14001664a: movaps xmm12, xmmword ptr [rsp + 0xc0]
140016653: movaps xmm13, xmmword ptr [rsp + 0xb0]
14001665c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016665: movaps xmm15, xmmword ptr [rsp + 0x90]
14001666e: add rsp, 0x138
140016675: pop rbx
140016676: pop rdi
140016677: pop rsi
140016678: pop r12
14001667a: pop r13
14001667c: pop r14
14001667e: pop r15
140016680: pop rbp
140016681: ret 
140016682: nop word ptr cs:[rax + rax]
140016690: mov qword ptr [rsp + 0x10], rdx
140016695: push rbp
140016696: push r15
140016698: push r14
14001669a: push r13
14001669c: push r12
14001669e: push rsi
14001669f: push rdi
1400166a0: push rbx
1400166a1: sub rsp, 0x138
1400166a8: lea rbp, [rdx + 0x80]
1400166af: movaps xmmword ptr [rsp + 0x90], xmm15
1400166b8: movaps xmmword ptr [rsp + 0xa0], xmm14
1400166c1: movaps xmmword ptr [rsp + 0xb0], xmm13
1400166ca: movaps xmmword ptr [rsp + 0xc0], xmm12
1400166d3: movaps xmmword ptr [rsp + 0xd0], xmm11
1400166dc: movaps xmmword ptr [rsp + 0xe0], xmm10
1400166e5: movaps xmmword ptr [rsp + 0xf0], xmm9
1400166ee: movaps xmmword ptr [rsp + 0x100], xmm8
1400166f7: movaps xmmword ptr [rsp + 0x110], xmm7
1400166ff: movaps xmmword ptr [rsp + 0x120], xmm6
140016707: cmp byte ptr [rbp + 0x800], 0
14001670e: je 0x1400167a2
140016714: mov rdx, qword ptr [rbp + 0x650]
14001671b: test rdx, rdx
14001671e: je 0x140016736
140016720: mov rcx, qword ptr [rbp + 0x658]
140016727: shl rdx, 3
14001672b: mov r8d, 4
140016731: call 0x140613c20 ; ?
140016736: movaps xmm6, xmmword ptr [rsp + 0x120]
14001673e: movaps xmm7, xmmword ptr [rsp + 0x110]
140016746: movaps xmm8, xmmword ptr [rsp + 0x100]
14001674f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016758: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016761: movaps xmm11, xmmword ptr [rsp + 0xd0]
14001676a: movaps xmm12, xmmword ptr [rsp + 0xc0]
140016773: movaps xmm13, xmmword ptr [rsp + 0xb0]
14001677c: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016785: movaps xmm15, xmmword ptr [rsp + 0x90]
14001678e: add rsp, 0x138
140016795: pop rbx
140016796: pop rdi
140016797: pop rsi
140016798: pop r12
14001679a: pop r13
14001679c: pop r14
14001679e: pop r15
1400167a0: pop rbp
1400167a1: ret 
1400167a2: movaps xmm6, xmmword ptr [rsp + 0x120]
1400167aa: movaps xmm7, xmmword ptr [rsp + 0x110]
1400167b2: movaps xmm8, xmmword ptr [rsp + 0x100]
1400167bb: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400167c4: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400167cd: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400167d6: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400167df: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400167e8: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400167f1: movaps xmm15, xmmword ptr [rsp + 0x90]
1400167fa: add rsp, 0x138
140016801: pop rbx
140016802: pop rdi
140016803: pop rsi
140016804: pop r12
140016806: pop r13
140016808: pop r14
14001680a: pop r15
14001680c: pop rbp
14001680d: ret 
14001680e: nop 
140016810: mov qword ptr [rsp + 0x10], rdx
140016815: push rbp
140016816: push r15
140016818: push r14
14001681a: push r13
14001681c: push r12
14001681e: push rsi
14001681f: push rdi
140016820: push rbx
140016821: sub rsp, 0x138
140016828: lea rbp, [rdx + 0x80]
14001682f: movaps xmmword ptr [rsp + 0x90], xmm15
140016838: movaps xmmword ptr [rsp + 0xa0], xmm14
140016841: movaps xmmword ptr [rsp + 0xb0], xmm13
14001684a: movaps xmmword ptr [rsp + 0xc0], xmm12
140016853: movaps xmmword ptr [rsp + 0xd0], xmm11
14001685c: movaps xmmword ptr [rsp + 0xe0], xmm10
140016865: movaps xmmword ptr [rsp + 0xf0], xmm9
14001686e: movaps xmmword ptr [rsp + 0x100], xmm8
140016877: movaps xmmword ptr [rsp + 0x110], xmm7
14001687f: movaps xmmword ptr [rsp + 0x120], xmm6
140016887: cmp byte ptr [rbp + 0x800], 0
14001688e: je 0x14001692c
140016894: mov rdx, qword ptr [rbp + 0x548]
14001689b: test rdx, rdx
14001689e: je 0x1400168b6
1400168a0: mov rcx, qword ptr [rbp + 0x550]
1400168a7: shl rdx, 2
1400168ab: mov r8d, 4
1400168b1: call 0x140613c20 ; ?
1400168b6: mov byte ptr [rbp + 0x817], 1
1400168bd: movaps xmm6, xmmword ptr [rsp + 0x120]
1400168c5: movaps xmm7, xmmword ptr [rsp + 0x110]
1400168cd: movaps xmm8, xmmword ptr [rsp + 0x100]
1400168d6: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400168df: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400168e8: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400168f1: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400168fb: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140016905: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001690e: movdqa xmm15, xmmword ptr [rsp + 0x90]
140016918: add rsp, 0x138
14001691f: pop rbx
140016920: pop rdi
140016921: pop rsi
140016922: pop r12
140016924: pop r13
140016926: pop r14
140016928: pop r15
14001692a: pop rbp
14001692b: ret 
14001692c: mov byte ptr [rbp + 0x817], 1
140016933: movaps xmm6, xmmword ptr [rsp + 0x120]
14001693b: movaps xmm7, xmmword ptr [rsp + 0x110]
140016943: movaps xmm8, xmmword ptr [rsp + 0x100]
14001694c: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016955: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001695e: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016967: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140016971: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001697b: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016984: movdqa xmm15, xmmword ptr [rsp + 0x90]
14001698e: add rsp, 0x138
140016995: pop rbx
140016996: pop rdi
140016997: pop rsi
140016998: pop r12
14001699a: pop r13
14001699c: pop r14
14001699e: pop r15
1400169a0: pop rbp
1400169a1: ret 
1400169a2: nop word ptr cs:[rax + rax]
1400169b0: mov qword ptr [rsp + 0x10], rdx
1400169b5: push rbp
1400169b6: push r15
1400169b8: push r14
1400169ba: push r13
1400169bc: push r12
1400169be: push rsi
1400169bf: push rdi
1400169c0: push rbx
1400169c1: sub rsp, 0x138
1400169c8: lea rbp, [rdx + 0x80]
1400169cf: movdqa xmmword ptr [rsp + 0x90], xmm15
1400169d9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400169e2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400169ec: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400169f6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400169ff: movaps xmmword ptr [rsp + 0xe0], xmm10
140016a08: movaps xmmword ptr [rsp + 0xf0], xmm9
140016a11: movaps xmmword ptr [rsp + 0x100], xmm8
140016a1a: movaps xmmword ptr [rsp + 0x110], xmm7
140016a22: movaps xmmword ptr [rsp + 0x120], xmm6
140016a2a: cmp byte ptr [rbp + 0x813], 0
140016a31: je 0x140016ac5
140016a37: mov rdx, qword ptr [rbp + 0x668]
140016a3e: test rdx, rdx
140016a41: je 0x140016a59
140016a43: shl rdx, 2
140016a47: mov r8d, 4
140016a4d: mov rcx, qword ptr [rbp + 0x7a8]
140016a54: call 0x140613c20 ; ?
140016a59: movaps xmm6, xmmword ptr [rsp + 0x120]
140016a61: movaps xmm7, xmmword ptr [rsp + 0x110]
140016a69: movaps xmm8, xmmword ptr [rsp + 0x100]
140016a72: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016a7b: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016a84: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016a8d: movaps xmm12, xmmword ptr [rsp + 0xc0]
140016a96: movaps xmm13, xmmword ptr [rsp + 0xb0]
140016a9f: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016aa8: movaps xmm15, xmmword ptr [rsp + 0x90]
140016ab1: add rsp, 0x138
140016ab8: pop rbx
140016ab9: pop rdi
140016aba: pop rsi
140016abb: pop r12
140016abd: pop r13
140016abf: pop r14
140016ac1: pop r15
140016ac3: pop rbp
140016ac4: ret 
140016ac5: movaps xmm6, xmmword ptr [rsp + 0x120]
140016acd: movaps xmm7, xmmword ptr [rsp + 0x110]
140016ad5: movaps xmm8, xmmword ptr [rsp + 0x100]
140016ade: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016ae7: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016af0: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016af9: movaps xmm12, xmmword ptr [rsp + 0xc0]
140016b02: movaps xmm13, xmmword ptr [rsp + 0xb0]
140016b0b: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016b14: movaps xmm15, xmmword ptr [rsp + 0x90]
140016b1d: add rsp, 0x138
140016b24: pop rbx
140016b25: pop rdi
140016b26: pop rsi
140016b27: pop r12
140016b29: pop r13
140016b2b: pop r14
140016b2d: pop r15
140016b2f: pop rbp
140016b30: ret 
140016b31: nop word ptr cs:[rax + rax]
140016b40: mov qword ptr [rsp + 0x10], rdx
140016b45: push rbp
140016b46: push r15
140016b48: push r14
140016b4a: push r13
140016b4c: push r12
140016b4e: push rsi
140016b4f: push rdi
140016b50: push rbx
140016b51: sub rsp, 0x138
140016b58: lea rbp, [rdx + 0x80]
140016b5f: movaps xmmword ptr [rsp + 0x90], xmm15
140016b68: movaps xmmword ptr [rsp + 0xa0], xmm14
140016b71: movaps xmmword ptr [rsp + 0xb0], xmm13
140016b7a: movaps xmmword ptr [rsp + 0xc0], xmm12
140016b83: movaps xmmword ptr [rsp + 0xd0], xmm11
140016b8c: movaps xmmword ptr [rsp + 0xe0], xmm10
140016b95: movaps xmmword ptr [rsp + 0xf0], xmm9
140016b9e: movaps xmmword ptr [rsp + 0x100], xmm8
140016ba7: movaps xmmword ptr [rsp + 0x110], xmm7
140016baf: movaps xmmword ptr [rsp + 0x120], xmm6
140016bb7: lea rcx, [rbp + 0x380]
140016bbe: call 0x1405e16f0 ; core::ptr::drop_in_place<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<tuple$<usize,usize>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<alloc::vec::Vec<f32,alloc::alloc::Global>,alloc::alloc::Global> > >
140016bc3: movaps xmm6, xmmword ptr [rsp + 0x120]
140016bcb: movaps xmm7, xmmword ptr [rsp + 0x110]
140016bd3: movaps xmm8, xmmword ptr [rsp + 0x100]
140016bdc: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016be5: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016bee: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016bf7: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140016c01: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140016c0b: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016c14: movdqa xmm15, xmmword ptr [rsp + 0x90]
140016c1e: add rsp, 0x138
140016c25: pop rbx
140016c26: pop rdi
140016c27: pop rsi
140016c28: pop r12
140016c2a: pop r13
140016c2c: pop r14
140016c2e: pop r15
140016c30: pop rbp
140016c31: ret 
140016c32: nop word ptr cs:[rax + rax]
140016c40: mov qword ptr [rsp + 0x10], rdx
140016c45: push rbp
140016c46: push r15
140016c48: push r14
140016c4a: push r13
140016c4c: push r12
140016c4e: push rsi
140016c4f: push rdi
140016c50: push rbx
140016c51: sub rsp, 0x138
140016c58: lea rbp, [rdx + 0x80]
140016c5f: movdqa xmmword ptr [rsp + 0x90], xmm15
140016c69: movaps xmmword ptr [rsp + 0xa0], xmm14
140016c72: movdqa xmmword ptr [rsp + 0xb0], xmm13
140016c7c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140016c86: movaps xmmword ptr [rsp + 0xd0], xmm11
140016c8f: movaps xmmword ptr [rsp + 0xe0], xmm10
140016c98: movaps xmmword ptr [rsp + 0xf0], xmm9
140016ca1: movaps xmmword ptr [rsp + 0x100], xmm8
140016caa: movaps xmmword ptr [rsp + 0x110], xmm7
140016cb2: movaps xmmword ptr [rsp + 0x120], xmm6
140016cba: lea rcx, [rbp + 0x6a0]
140016cc1: call 0x1405e1b90 ; core::ptr::drop_in_place<itertools::tuple_impl::CircularTupleWindows<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>,tuple$<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3> > >
140016cc6: movaps xmm6, xmmword ptr [rsp + 0x120]
140016cce: movaps xmm7, xmmword ptr [rsp + 0x110]
140016cd6: movaps xmm8, xmmword ptr [rsp + 0x100]
140016cdf: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016ce8: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016cf1: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016cfa: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140016d04: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140016d0e: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016d17: movdqa xmm15, xmmword ptr [rsp + 0x90]
140016d21: add rsp, 0x138
140016d28: pop rbx
140016d29: pop rdi
140016d2a: pop rsi
140016d2b: pop r12
140016d2d: pop r13
140016d2f: pop r14
140016d31: pop r15
140016d33: pop rbp
140016d34: ret 
140016d35: nop word ptr cs:[rax + rax]
140016d40: mov qword ptr [rsp + 0x10], rdx
140016d45: push rbp
140016d46: push r15
140016d48: push r14
140016d4a: push r13
140016d4c: push r12
140016d4e: push rsi
140016d4f: push rdi
140016d50: push rbx
140016d51: sub rsp, 0x138
140016d58: lea rbp, [rdx + 0x80]
140016d5f: movdqa xmmword ptr [rsp + 0x90], xmm15
140016d69: movaps xmmword ptr [rsp + 0xa0], xmm14
140016d72: movdqa xmmword ptr [rsp + 0xb0], xmm13
140016d7c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140016d86: movaps xmmword ptr [rsp + 0xd0], xmm11
140016d8f: movaps xmmword ptr [rsp + 0xe0], xmm10
140016d98: movaps xmmword ptr [rsp + 0xf0], xmm9
140016da1: movaps xmmword ptr [rsp + 0x100], xmm8
140016daa: movaps xmmword ptr [rsp + 0x110], xmm7
140016db2: movaps xmmword ptr [rsp + 0x120], xmm6
140016dba: cmp qword ptr [rbp + 0x668], 0
140016dc2: mov rdx, qword ptr [rbp + 0x7a0]
140016dc9: je 0x140016de3
140016dcb: test rdx, rdx
140016dce: je 0x140016de3
140016dd0: mov rcx, qword ptr [rbp + 0x670]
140016dd7: mov r8, qword ptr [rbp + 0x7a8]
140016dde: call 0x140613c20 ; ?
140016de3: mov byte ptr [rbp + 0x7e9], 0
140016dea: movaps xmm6, xmmword ptr [rsp + 0x120]
140016df2: movaps xmm7, xmmword ptr [rsp + 0x110]
140016dfa: movaps xmm8, xmmword ptr [rsp + 0x100]
140016e03: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016e0c: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016e15: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016e1e: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140016e28: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140016e32: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016e3b: movdqa xmm15, xmmword ptr [rsp + 0x90]
140016e45: add rsp, 0x138
140016e4c: pop rbx
140016e4d: pop rdi
140016e4e: pop rsi
140016e4f: pop r12
140016e51: pop r13
140016e53: pop r14
140016e55: pop r15
140016e57: pop rbp
140016e58: ret 
140016e59: nop dword ptr [rax]
140016e60: mov qword ptr [rsp + 0x10], rdx
140016e65: push rbp
140016e66: push r15
140016e68: push r14
140016e6a: push r13
140016e6c: push r12
140016e6e: push rsi
140016e6f: push rdi
140016e70: push rbx
140016e71: sub rsp, 0x138
140016e78: lea rbp, [rdx + 0x80]
140016e7f: movdqa xmmword ptr [rsp + 0x90], xmm15
140016e89: movaps xmmword ptr [rsp + 0xa0], xmm14
140016e92: movdqa xmmword ptr [rsp + 0xb0], xmm13
140016e9c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140016ea6: movaps xmmword ptr [rsp + 0xd0], xmm11
140016eaf: movaps xmmword ptr [rsp + 0xe0], xmm10
140016eb8: movaps xmmword ptr [rsp + 0xf0], xmm9
140016ec1: movaps xmmword ptr [rsp + 0x100], xmm8
140016eca: movaps xmmword ptr [rsp + 0x110], xmm7
140016ed2: movaps xmmword ptr [rsp + 0x120], xmm6
140016eda: cmp byte ptr [rbp + 0x7e9], 0
140016ee1: je 0x140016f69
140016ee7: lea rcx, [rbp + 0x460]
140016eee: call 0x140149f50 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
140016ef3: mov byte ptr [rbp + 0x7ff], 1
140016efa: movaps xmm6, xmmword ptr [rsp + 0x120]
140016f02: movaps xmm7, xmmword ptr [rsp + 0x110]
140016f0a: movaps xmm8, xmmword ptr [rsp + 0x100]
140016f13: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016f1c: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016f25: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016f2e: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140016f38: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140016f42: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016f4b: movdqa xmm15, xmmword ptr [rsp + 0x90]
140016f55: add rsp, 0x138
140016f5c: pop rbx
140016f5d: pop rdi
140016f5e: pop rsi
140016f5f: pop r12
140016f61: pop r13
140016f63: pop r14
140016f65: pop r15
140016f67: pop rbp
140016f68: ret 
140016f69: mov byte ptr [rbp + 0x7ff], 1
140016f70: movaps xmm6, xmmword ptr [rsp + 0x120]
140016f78: movaps xmm7, xmmword ptr [rsp + 0x110]
140016f80: movaps xmm8, xmmword ptr [rsp + 0x100]
140016f89: movaps xmm9, xmmword ptr [rsp + 0xf0]
140016f92: movaps xmm10, xmmword ptr [rsp + 0xe0]
140016f9b: movaps xmm11, xmmword ptr [rsp + 0xd0]
140016fa4: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140016fae: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140016fb8: movaps xmm14, xmmword ptr [rsp + 0xa0]
140016fc1: movdqa xmm15, xmmword ptr [rsp + 0x90]
140016fcb: add rsp, 0x138
140016fd2: pop rbx
140016fd3: pop rdi
140016fd4: pop rsi
140016fd5: pop r12
140016fd7: pop r13
140016fd9: pop r14
140016fdb: pop r15
140016fdd: pop rbp
140016fde: ret 
140016fdf: nop 
140016fe0: mov qword ptr [rsp + 0x10], rdx
140016fe5: push rbp
140016fe6: push r15
140016fe8: push r14
140016fea: push r13
140016fec: push r12
140016fee: push rsi
140016fef: push rdi
140016ff0: push rbx
140016ff1: sub rsp, 0x138
140016ff8: lea rbp, [rdx + 0x80]
140016fff: movdqa xmmword ptr [rsp + 0x90], xmm15
140017009: movaps xmmword ptr [rsp + 0xa0], xmm14
140017012: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001701c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140017026: movaps xmmword ptr [rsp + 0xd0], xmm11
14001702f: movaps xmmword ptr [rsp + 0xe0], xmm10
140017038: movaps xmmword ptr [rsp + 0xf0], xmm9
140017041: movaps xmmword ptr [rsp + 0x100], xmm8
14001704a: movaps xmmword ptr [rsp + 0x110], xmm7
140017052: movaps xmmword ptr [rsp + 0x120], xmm6
14001705a: mov rdx, qword ptr [rbp + 0x7a8]
140017061: test rdx, rdx
140017064: je 0x14001707c
140017066: shl rdx, 4
14001706a: mov r8d, 8
140017070: mov rcx, qword ptr [rbp + 0x798]
140017077: call 0x140613c20 ; ?
14001707c: movaps xmm6, xmmword ptr [rsp + 0x120]
140017084: movaps xmm7, xmmword ptr [rsp + 0x110]
14001708c: movaps xmm8, xmmword ptr [rsp + 0x100]
140017095: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001709e: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400170a7: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400170b0: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400170ba: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400170c4: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400170cd: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400170d7: add rsp, 0x138
1400170de: pop rbx
1400170df: pop rdi
1400170e0: pop rsi
1400170e1: pop r12
1400170e3: pop r13
1400170e5: pop r14
1400170e7: pop r15
1400170e9: pop rbp
1400170ea: ret 
1400170eb: nop dword ptr [rax + rax]
1400170f0: mov qword ptr [rsp + 0x10], rdx
1400170f5: push rbp
1400170f6: push r15
1400170f8: push r14
1400170fa: push r13
1400170fc: push r12
1400170fe: push rsi
1400170ff: push rdi
140017100: push rbx
140017101: sub rsp, 0x138
140017108: lea rbp, [rdx + 0x80]
14001710f: movdqa xmmword ptr [rsp + 0x90], xmm15
140017119: movaps xmmword ptr [rsp + 0xa0], xmm14
140017122: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001712c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140017136: movaps xmmword ptr [rsp + 0xd0], xmm11
14001713f: movaps xmmword ptr [rsp + 0xe0], xmm10
140017148: movaps xmmword ptr [rsp + 0xf0], xmm9
140017151: movaps xmmword ptr [rsp + 0x100], xmm8
14001715a: movaps xmmword ptr [rsp + 0x110], xmm7
140017162: movaps xmmword ptr [rsp + 0x120], xmm6
14001716a: mov rax, qword ptr [rbp + 0x578]
140017171: cmp rax, qword ptr [rbp + 0x520]
140017178: jne 0x1400171f0
14001717a: mov byte ptr [rbp + 0x814], 1
140017181: movaps xmm6, xmmword ptr [rsp + 0x120]
140017189: movaps xmm7, xmmword ptr [rsp + 0x110]
140017191: movaps xmm8, xmmword ptr [rsp + 0x100]
14001719a: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400171a3: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400171ac: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400171b5: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400171bf: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400171c9: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400171d2: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400171dc: add rsp, 0x138
1400171e3: pop rbx
1400171e4: pop rdi
1400171e5: pop rsi
1400171e6: pop r12
1400171e8: pop r13
1400171ea: pop r14
1400171ec: pop r15
1400171ee: pop rbp
1400171ef: ret 
1400171f0: mov r14, qword ptr [rbp + 0x330]
1400171f7: test r14, r14
1400171fa: mov rdi, qword ptr [rbp + 0x580]
140017201: mov rbx, qword ptr [rbp + 0x560]
140017208: jne 0x1400172c1
14001720e: test rdi, rdi
140017211: je 0x14001722d
140017213: shl rdi, 3
140017217: lea rdx, [rdi + rdi*2]
14001721b: mov r8d, 8
140017221: mov rcx, qword ptr [rbp + 0x568]
140017228: call 0x140613c20 ; ?
14001722d: test rbx, rbx
140017230: je 0x14001724b
140017232: shl rbx, 4
140017236: mov r8d, 8
14001723c: mov rcx, qword ptr [rbp + 0x530]
140017243: mov rdx, rbx
140017246: call 0x140613c20 ; ?
14001724b: mov byte ptr [rbp + 0x814], 1
140017252: movaps xmm6, xmmword ptr [rsp + 0x120]
14001725a: movaps xmm7, xmmword ptr [rsp + 0x110]
140017262: movaps xmm8, xmmword ptr [rsp + 0x100]
14001726b: movaps xmm9, xmmword ptr [rsp + 0xf0]
140017274: movaps xmm10, xmmword ptr [rsp + 0xe0]
14001727d: movaps xmm11, xmmword ptr [rsp + 0xd0]
140017286: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140017290: movdqa xmm13, xmmword ptr [rsp + 0xb0]
14001729a: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400172a3: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400172ad: add rsp, 0x138
1400172b4: pop rbx
1400172b5: pop rdi
1400172b6: pop rsi
1400172b7: pop r12
1400172b9: pop r13
1400172bb: pop r14
1400172bd: pop r15
1400172bf: pop rbp
1400172c0: ret 
1400172c1: mov rax, qword ptr [rbp + 0x568]
1400172c8: lea rsi, [rax + 8]
1400172cc: jmp 0x1400172dd ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400172ce: nop 
1400172d0: add rsi, 0x18
1400172d4: dec r14
1400172d7: je 0x14001720e
1400172dd: mov rdx, qword ptr [rsi - 8]
1400172e1: test rdx, rdx
1400172e4: je 0x1400172d0
1400172e6: mov rcx, qword ptr [rsi]
1400172e9: shl rdx, 2
1400172ed: mov r8d, 4
1400172f3: call 0x140613c20 ; ?
1400172f8: jmp 0x1400172d0 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400172fa: nop word ptr [rax + rax]
140017300: mov qword ptr [rsp + 0x10], rdx
140017305: push rbp
140017306: push r15
140017308: push r14
14001730a: push r13
14001730c: push r12
14001730e: push rsi
14001730f: push rdi
140017310: push rbx
140017311: sub rsp, 0x138
140017318: lea rbp, [rdx + 0x80]
14001731f: movdqa xmmword ptr [rsp + 0x90], xmm15
140017329: movaps xmmword ptr [rsp + 0xa0], xmm14
140017332: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001733c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140017346: movaps xmmword ptr [rsp + 0xd0], xmm11
14001734f: movaps xmmword ptr [rsp + 0xe0], xmm10
140017358: movaps xmmword ptr [rsp + 0xf0], xmm9
140017361: movaps xmmword ptr [rsp + 0x100], xmm8
14001736a: movaps xmmword ptr [rsp + 0x110], xmm7
140017372: movaps xmmword ptr [rsp + 0x120], xmm6
14001737a: cmp byte ptr [rbp + 0x814], 0
140017381: je 0x14001747b
140017387: mov rax, qword ptr [rbp + 0x460]
14001738e: test rax, rax
140017391: jne 0x14001750d
140017397: mov rdx, qword ptr [rbp + 0x478]
14001739e: test rdx, rdx
1400173a1: jne 0x140017537
1400173a7: mov rdx, qword ptr [rbp + 0x498]
1400173ae: test rdx, rdx
1400173b1: jne 0x14001755d
1400173b7: mov rdx, qword ptr [rbp + 0x4b0]
1400173be: test rdx, rdx
1400173c1: jne 0x140017583
1400173c7: mov rdx, qword ptr [rbp + 0x4c8]
1400173ce: test rdx, rdx
1400173d1: je 0x1400173e9
1400173d3: mov rcx, qword ptr [rbp + 0x4d0]
1400173da: shl rdx, 2
1400173de: mov r8d, 4
1400173e4: call 0x140613c20 ; ?
1400173e9: mov byte ptr [rbp + 0x7ee], 0
1400173f0: mov byte ptr [rbp + 0x7ed], 0
1400173f7: mov byte ptr [rbp + 0x7ec], 0
1400173fe: mov byte ptr [rbp + 0x7eb], 1
140017405: mov byte ptr [rbp + 0x7ea], 1
14001740c: movaps xmm6, xmmword ptr [rsp + 0x120]
140017414: movaps xmm7, xmmword ptr [rsp + 0x110]
14001741c: movaps xmm8, xmmword ptr [rsp + 0x100]
140017425: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001742e: movaps xmm10, xmmword ptr [rsp + 0xe0]
140017437: movaps xmm11, xmmword ptr [rsp + 0xd0]
140017440: movdqa xmm12, xmmword ptr [rsp + 0xc0]
14001744a: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140017454: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001745d: movdqa xmm15, xmmword ptr [rsp + 0x90]
140017467: add rsp, 0x138
14001746e: pop rbx
14001746f: pop rdi
140017470: pop rsi
140017471: pop r12
140017473: pop r13
140017475: pop r14
140017477: pop r15
140017479: pop rbp
14001747a: ret 
14001747b: mov byte ptr [rbp + 0x7ee], 0
140017482: mov byte ptr [rbp + 0x7ed], 0
140017489: mov byte ptr [rbp + 0x7ec], 0
140017490: mov byte ptr [rbp + 0x7eb], 1
140017497: mov byte ptr [rbp + 0x7ea], 1
14001749e: movaps xmm6, xmmword ptr [rsp + 0x120]
1400174a6: movaps xmm7, xmmword ptr [rsp + 0x110]
1400174ae: movaps xmm8, xmmword ptr [rsp + 0x100]
1400174b7: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400174c0: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400174c9: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400174d2: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400174dc: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400174e6: movaps xmm14, xmmword ptr [rsp + 0xa0]
1400174ef: movdqa xmm15, xmmword ptr [rsp + 0x90]
1400174f9: add rsp, 0x138
140017500: pop rbx
140017501: pop rdi
140017502: pop rsi
140017503: pop r12
140017505: pop r13
140017507: pop r14
140017509: pop r15
14001750b: pop rbp
14001750c: ret 
14001750d: mov rcx, qword ptr [rbp + 0x468]
140017514: shl rax, 2
140017518: lea rdx, [rax + rax*2]
14001751c: mov r8d, 4
140017522: call 0x140613c20 ; ?
140017527: mov rdx, qword ptr [rbp + 0x478]
14001752e: test rdx, rdx
140017531: je 0x1400173a7
140017537: mov rcx, qword ptr [rbp + 0x480]
14001753e: shl rdx, 2
140017542: mov r8d, 4
140017548: call 0x140613c20 ; ?
14001754d: mov rdx, qword ptr [rbp + 0x498]
140017554: test rdx, rdx
140017557: je 0x1400173b7
14001755d: mov rcx, qword ptr [rbp + 0x4a0]
140017564: shl rdx, 3
140017568: mov r8d, 4
14001756e: call 0x140613c20 ; ?
140017573: mov rdx, qword ptr [rbp + 0x4b0]
14001757a: test rdx, rdx
14001757d: je 0x1400173c7
140017583: mov rcx, qword ptr [rbp + 0x4b8]
14001758a: shl rdx, 2
14001758e: mov r8d, 4
140017594: call 0x140613c20 ; ?
140017599: mov rdx, qword ptr [rbp + 0x4c8]
1400175a0: test rdx, rdx
1400175a3: jne 0x1400173d3
1400175a9: jmp 0x1400173e9 ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
1400175ae: nop 
1400175b0: mov qword ptr [rsp + 0x10], rdx
1400175b5: push rbp
1400175b6: push r15
1400175b8: push r14
1400175ba: push r13
1400175bc: push r12
1400175be: push rsi
1400175bf: push rdi
1400175c0: push rbx
1400175c1: sub rsp, 0x138
1400175c8: lea rbp, [rdx + 0x80]
1400175cf: movdqa xmmword ptr [rsp + 0x90], xmm15
1400175d9: movaps xmmword ptr [rsp + 0xa0], xmm14
1400175e2: movdqa xmmword ptr [rsp + 0xb0], xmm13
1400175ec: movdqa xmmword ptr [rsp + 0xc0], xmm12
1400175f6: movaps xmmword ptr [rsp + 0xd0], xmm11
1400175ff: movaps xmmword ptr [rsp + 0xe0], xmm10
140017608: movaps xmmword ptr [rsp + 0xf0], xmm9
140017611: movaps xmmword ptr [rsp + 0x100], xmm8
14001761a: movaps xmmword ptr [rsp + 0x110], xmm7
140017622: movaps xmmword ptr [rsp + 0x120], xmm6
14001762a: movzx r14d, byte ptr [rbp + 0x7eb]
140017632: movzx edi, byte ptr [rbp + 0x7ec]
140017639: movzx esi, byte ptr [rbp + 0x7ed]
140017640: movzx ebx, byte ptr [rbp + 0x7ee]
140017647: cmp qword ptr [rbp + 0x800], 0
14001764f: je 0x14001767b
140017651: test byte ptr [rbp + 0x7ea], 1
140017658: jne 0x14001767b
14001765a: mov rax, qword ptr [rbp + 0x800]
140017661: shl rax, 2
140017665: lea rdx, [rax + rax*2]
140017669: mov r8d, 4
14001766f: mov rcx, qword ptr [rbp + 0x7f0]
140017676: call 0x140613c20 ; ?
14001767b: mov rdx, qword ptr [rbp + 0x570]
140017682: test rdx, rdx
140017685: sete al
140017688: or r14b, al
14001768b: test r14b, 1
14001768f: jne 0x1400176a7
140017691: shl rdx, 2
140017695: mov r8d, 4
14001769b: mov rcx, qword ptr [rbp + 0x528]
1400176a2: call 0x140613c20 ; ?
1400176a7: mov rdx, qword ptr [rbp + 0x648]
1400176ae: test rdx, rdx
1400176b1: setne al
1400176b4: test dil, al
1400176b7: mov rdi, qword ptr [rbp + 0x780]
1400176be: mov r14, qword ptr [rbp + 0x760]
1400176c5: je 0x140017787
1400176cb: shl rdx, 3
1400176cf: mov r8d, 4
1400176d5: mov rcx, qword ptr [rbp + 0x7b0]
1400176dc: call 0x140613c20 ; ?
1400176e1: test rdi, rdi
1400176e4: setne al
1400176e7: test sil, al
1400176ea: jne 0x140017796
1400176f0: test bl, 1
1400176f3: je 0x1400177b4
1400176f9: mov rdx, qword ptr [rbp + 0x7e0]
140017700: test rdx, rdx
140017703: je 0x14001771b
140017705: shl rdx, 2
140017709: mov r8d, 4
14001770f: mov rcx, qword ptr [rbp + 0x778]
140017716: call 0x140613c20 ; ?
14001771b: movaps xmm6, xmmword ptr [rsp + 0x120]
140017723: movaps xmm7, xmmword ptr [rsp + 0x110]
14001772b: movaps xmm8, xmmword ptr [rsp + 0x100]
140017734: movaps xmm9, xmmword ptr [rsp + 0xf0]
14001773d: movaps xmm10, xmmword ptr [rsp + 0xe0]
140017746: movaps xmm11, xmmword ptr [rsp + 0xd0]
14001774f: movaps xmm12, xmmword ptr [rsp + 0xc0]
140017758: movaps xmm13, xmmword ptr [rsp + 0xb0]
140017761: movaps xmm14, xmmword ptr [rsp + 0xa0]
14001776a: movaps xmm15, xmmword ptr [rsp + 0x90]
140017773: add rsp, 0x138
14001777a: pop rbx
14001777b: pop rdi
14001777c: pop rsi
14001777d: pop r12
14001777f: pop r13
140017781: pop r14
140017783: pop r15
140017785: pop rbp
140017786: ret 
140017787: test rdi, rdi
14001778a: setne al
14001778d: test sil, al
140017790: je 0x1400176f0
140017796: shl rdi, 2
14001779a: mov r8d, 4
1400177a0: mov rcx, r14
1400177a3: mov rdx, rdi
1400177a6: call 0x140613c20 ; ?
1400177ab: test bl, 1
1400177ae: jne 0x1400176f9
1400177b4: movaps xmm6, xmmword ptr [rsp + 0x120]
1400177bc: movaps xmm7, xmmword ptr [rsp + 0x110]
1400177c4: movaps xmm8, xmmword ptr [rsp + 0x100]
1400177cd: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400177d6: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400177df: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400177e8: movaps xmm12, xmmword ptr [rsp + 0xc0]
1400177f1: movaps xmm13, xmmword ptr [rsp + 0xb0]
1400177fa: movaps xmm14, xmmword ptr [rsp + 0xa0]
140017803: movaps xmm15, xmmword ptr [rsp + 0x90]
14001780c: add rsp, 0x138
140017813: pop rbx
140017814: pop rdi
140017815: pop rsi
140017816: pop r12
140017818: pop r13
14001781a: pop r14
14001781c: pop r15
14001781e: pop rbp
14001781f: ret 
140017820: mov qword ptr [rsp + 0x10], rdx
140017825: push rbp
140017826: push r15
140017828: push r14
14001782a: push r13
14001782c: push r12
14001782e: push rsi
14001782f: push rdi
140017830: push rbx
140017831: sub rsp, 0x138
140017838: lea rbp, [rdx + 0x80]
14001783f: movaps xmmword ptr [rsp + 0x90], xmm15
140017848: movaps xmmword ptr [rsp + 0xa0], xmm14
140017851: movaps xmmword ptr [rsp + 0xb0], xmm13
14001785a: movaps xmmword ptr [rsp + 0xc0], xmm12
140017863: movaps xmmword ptr [rsp + 0xd0], xmm11
14001786c: movaps xmmword ptr [rsp + 0xe0], xmm10
140017875: movaps xmmword ptr [rsp + 0xf0], xmm9
14001787e: movaps xmmword ptr [rsp + 0x100], xmm8
140017887: movaps xmmword ptr [rsp + 0x110], xmm7
14001788f: movaps xmmword ptr [rsp + 0x120], xmm6
140017897: xor eax, eax
140017899: cmp rax, qword ptr [rbp + 0x6a0]
1400178a0: jo 0x1400178ae
1400178a2: lea rcx, [rbp + 0x6a0]
1400178a9: call 0x140609eb0 ; core::ptr::drop_in_place<country_core::resources::stairs_walls::StairsArchSupportAssemblyParams>
1400178ae: mov byte ptr [rbp + 0x7ff], 0
1400178b5: movaps xmm6, xmmword ptr [rsp + 0x120]
1400178bd: movaps xmm7, xmmword ptr [rsp + 0x110]
1400178c5: movaps xmm8, xmmword ptr [rsp + 0x100]
1400178ce: movaps xmm9, xmmword ptr [rsp + 0xf0]
1400178d7: movaps xmm10, xmmword ptr [rsp + 0xe0]
1400178e0: movaps xmm11, xmmword ptr [rsp + 0xd0]
1400178e9: movdqa xmm12, xmmword ptr [rsp + 0xc0]
1400178f3: movdqa xmm13, xmmword ptr [rsp + 0xb0]
1400178fd: movaps xmm14, xmmword ptr [rsp + 0xa0]
140017906: movdqa xmm15, xmmword ptr [rsp + 0x90]
140017910: add rsp, 0x138
140017917: pop rbx
140017918: pop rdi
140017919: pop rsi
14001791a: pop r12
14001791c: pop r13
14001791e: pop r14
140017920: pop r15
140017922: pop rbp
140017923: ret 
140017924: nop word ptr cs:[rax + rax]
140017930: mov qword ptr [rsp + 0x10], rdx
140017935: push rbp
140017936: push r15
140017938: push r14
14001793a: push r13
14001793c: push r12
14001793e: push rsi
14001793f: push rdi
140017940: push rbx
140017941: sub rsp, 0x138
140017948: lea rbp, [rdx + 0x80]
14001794f: movdqa xmmword ptr [rsp + 0x90], xmm15
140017959: movaps xmmword ptr [rsp + 0xa0], xmm14
140017962: movdqa xmmword ptr [rsp + 0xb0], xmm13
14001796c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140017976: movaps xmmword ptr [rsp + 0xd0], xmm11
14001797f: movaps xmmword ptr [rsp + 0xe0], xmm10
140017988: movaps xmmword ptr [rsp + 0xf0], xmm9
140017991: movaps xmmword ptr [rsp + 0x100], xmm8
14001799a: movaps xmmword ptr [rsp + 0x110], xmm7
1400179a2: movaps xmmword ptr [rsp + 0x120], xmm6
1400179aa: cmp byte ptr [rbp + 0x7ff], 0
1400179b1: je 0x140017a4f
1400179b7: cmp byte ptr [rbp + 0x500], 0x11
1400179be: je 0x1400179d9
1400179c0: mov rax, qword ptr [rbp + 0x518]
1400179c7: lock dec qword ptr [rax]
1400179cb: jne 0x1400179d9
1400179cd: lea rcx, [rbp + 0x518]
1400179d4: call 0x140b5c620 ; alloc::sync::Arc::drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
1400179d9: mov byte ptr [rbp + 0x817], 0
1400179e0: movaps xmm6, xmmword ptr [rsp + 0x120]
1400179e8: movaps xmm7, xmmword ptr [rsp + 0x110]
1400179f0: movaps xmm8, xmmword ptr [rsp + 0x100]
1400179f9: movaps xmm9, xmmword ptr [rsp + 0xf0]
140017a02: movaps xmm10, xmmword ptr [rsp + 0xe0]
140017a0b: movaps xmm11, xmmword ptr [rsp + 0xd0]
140017a14: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140017a1e: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140017a28: movaps xmm14, xmmword ptr [rsp + 0xa0]
140017a31: movdqa xmm15, xmmword ptr [rsp + 0x90]
140017a3b: add rsp, 0x138
140017a42: pop rbx
140017a43: pop rdi
140017a44: pop rsi
140017a45: pop r12
140017a47: pop r13
140017a49: pop r14
140017a4b: pop r15
140017a4d: pop rbp
140017a4e: ret 
140017a4f: mov byte ptr [rbp + 0x817], 0
140017a56: movaps xmm6, xmmword ptr [rsp + 0x120]
140017a5e: movaps xmm7, xmmword ptr [rsp + 0x110]
140017a66: movaps xmm8, xmmword ptr [rsp + 0x100]
140017a6f: movaps xmm9, xmmword ptr [rsp + 0xf0]
140017a78: movaps xmm10, xmmword ptr [rsp + 0xe0]
140017a81: movaps xmm11, xmmword ptr [rsp + 0xd0]
140017a8a: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140017a94: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140017a9e: movaps xmm14, xmmword ptr [rsp + 0xa0]
140017aa7: movdqa xmm15, xmmword ptr [rsp + 0x90]
140017ab1: add rsp, 0x138
140017ab8: pop rbx
140017ab9: pop rdi
140017aba: pop rsi
140017abb: pop r12
140017abd: pop r13
140017abf: pop r14
140017ac1: pop r15
140017ac3: pop rbp
140017ac4: ret 
140017ac5: nop word ptr cs:[rax + rax]
140017ad0: mov qword ptr [rsp + 0x10], rdx
140017ad5: push rbp
140017ad6: push r15
140017ad8: push r14
140017ada: push r13
140017adc: push r12
140017ade: push rsi
140017adf: push rdi
140017ae0: push rbx
140017ae1: sub rsp, 0x138
140017ae8: lea rbp, [rdx + 0x80]
140017aef: movdqa xmmword ptr [rsp + 0x90], xmm15
140017af9: movaps xmmword ptr [rsp + 0xa0], xmm14
140017b02: movdqa xmmword ptr [rsp + 0xb0], xmm13
140017b0c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140017b16: movaps xmmword ptr [rsp + 0xd0], xmm11
140017b1f: movaps xmmword ptr [rsp + 0xe0], xmm10
140017b28: movaps xmmword ptr [rsp + 0xf0], xmm9
140017b31: movaps xmmword ptr [rsp + 0x100], xmm8
140017b3a: movaps xmmword ptr [rsp + 0x110], xmm7
140017b42: movaps xmmword ptr [rsp + 0x120], xmm6
140017b4a: cmp byte ptr [rbp + 0x817], 0
140017b51: je 0x140017c11
140017b57: mov rsi, qword ptr [rbp + 0x280]
140017b5e: mov rdi, qword ptr [rbp + 0x288]
140017b65: xor ebx, ebx
140017b67: mov rcx, rsi
140017b6a: cmp rdi, rbx
140017b6d: je 0x140017b87
140017b6f: nop 
140017b70: inc rbx
140017b73: lea r14, [rcx + 0x80]
140017b7a: call 0x140601be0 ; core::ptr::drop_in_place<country_core::systems::collision::world::RaycastMeshData>
140017b7f: mov rcx, r14
140017b82: cmp rdi, rbx
140017b85: jne 0x140017b70
140017b87: mov rdx, qword ptr [rbp + 0x278]
140017b8e: test rdx, rdx
140017b91: je 0x140017ba5
140017b93: shl rdx, 7
140017b97: mov r8d, 0x10
140017b9d: mov rcx, rsi
140017ba0: call 0x140613c20 ; ?
140017ba5: movaps xmm6, xmmword ptr [rsp + 0x120]
140017bad: movaps xmm7, xmmword ptr [rsp + 0x110]
140017bb5: movaps xmm8, xmmword ptr [rsp + 0x100]
140017bbe: movaps xmm9, xmmword ptr [rsp + 0xf0]
140017bc7: movaps xmm10, xmmword ptr [rsp + 0xe0]
140017bd0: movaps xmm11, xmmword ptr [rsp + 0xd0]
140017bd9: movaps xmm12, xmmword ptr [rsp + 0xc0]
140017be2: movaps xmm13, xmmword ptr [rsp + 0xb0]
140017beb: movaps xmm14, xmmword ptr [rsp + 0xa0]
140017bf4: movaps xmm15, xmmword ptr [rsp + 0x90]
140017bfd: add rsp, 0x138
140017c04: pop rbx
140017c05: pop rdi
140017c06: pop rsi
140017c07: pop r12
140017c09: pop r13
140017c0b: pop r14
140017c0d: pop r15
140017c0f: pop rbp
140017c10: ret 
140017c11: movaps xmm6, xmmword ptr [rsp + 0x120]
140017c19: movaps xmm7, xmmword ptr [rsp + 0x110]
140017c21: movaps xmm8, xmmword ptr [rsp + 0x100]
140017c2a: movaps xmm9, xmmword ptr [rsp + 0xf0]
140017c33: movaps xmm10, xmmword ptr [rsp + 0xe0]
140017c3c: movaps xmm11, xmmword ptr [rsp + 0xd0]
140017c45: movaps xmm12, xmmword ptr [rsp + 0xc0]
140017c4e: movaps xmm13, xmmword ptr [rsp + 0xb0]
140017c57: movaps xmm14, xmmword ptr [rsp + 0xa0]
140017c60: movaps xmm15, xmmword ptr [rsp + 0x90]
140017c69: add rsp, 0x138
140017c70: pop rbx
140017c71: pop rdi
140017c72: pop rsi
140017c73: pop r12
140017c75: pop r13
140017c77: pop r14
140017c79: pop r15
140017c7b: pop rbp
140017c7c: ret 
140017c7d: nop dword ptr [rax]
140017c80: mov qword ptr [rsp + 0x10], rdx
140017c85: push rbp
140017c86: push r15
140017c88: push r14
140017c8a: push r13
140017c8c: push r12
140017c8e: push rsi
140017c8f: push rdi
140017c90: push rbx
140017c91: sub rsp, 0x138
140017c98: lea rbp, [rdx + 0x80]
140017c9f: movaps xmmword ptr [rsp + 0x90], xmm15
140017ca8: movaps xmmword ptr [rsp + 0xa0], xmm14
140017cb1: movaps xmmword ptr [rsp + 0xb0], xmm13
140017cba: movaps xmmword ptr [rsp + 0xc0], xmm12
140017cc3: movaps xmmword ptr [rsp + 0xd0], xmm11
140017ccc: movaps xmmword ptr [rsp + 0xe0], xmm10
140017cd5: movaps xmmword ptr [rsp + 0xf0], xmm9
140017cde: movaps xmmword ptr [rsp + 0x100], xmm8
140017ce7: movaps xmmword ptr [rsp + 0x110], xmm7
140017cef: movaps xmmword ptr [rsp + 0x120], xmm6
140017cf7: lea rcx, [rbp + 0x440]
140017cfe: call 0x1405dec70 ; core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble::ArchSegment,alloc::alloc::Global> >
140017d03: movaps xmm6, xmmword ptr [rsp + 0x120]
140017d0b: movaps xmm7, xmmword ptr [rsp + 0x110]
140017d13: movaps xmm8, xmmword ptr [rsp + 0x100]
140017d1c: movaps xmm9, xmmword ptr [rsp + 0xf0]
140017d25: movaps xmm10, xmmword ptr [rsp + 0xe0]
140017d2e: movaps xmm11, xmmword ptr [rsp + 0xd0]
140017d37: movdqa xmm12, xmmword ptr [rsp + 0xc0]
140017d41: movdqa xmm13, xmmword ptr [rsp + 0xb0]
140017d4b: movaps xmm14, xmmword ptr [rsp + 0xa0]
140017d54: movdqa xmm15, xmmword ptr [rsp + 0x90]
140017d5e: add rsp, 0x138
140017d65: pop rbx
140017d66: pop rdi
140017d67: pop rsi
140017d68: pop r12
140017d6a: pop r13
140017d6c: pop r14
140017d6e: pop r15
140017d70: pop rbp
140017d71: ret 
140017d72: nop word ptr cs:[rax + rax]
140017d80: mov qword ptr [rsp + 0x10], rdx
140017d85: push rbp
140017d86: push r15
140017d88: push r14
140017d8a: push r13
140017d8c: push r12
140017d8e: push rsi
140017d8f: push rdi
140017d90: push rbx
140017d91: sub rsp, 0x138
140017d98: lea rbp, [rdx + 0x80]
140017d9f: movdqa xmmword ptr [rsp + 0x90], xmm15
140017da9: movaps xmmword ptr [rsp + 0xa0], xmm14
140017db2: movdqa xmmword ptr [rsp + 0xb0], xmm13
140017dbc: movdqa xmmword ptr [rsp + 0xc0], xmm12
140017dc6: movaps xmmword ptr [rsp + 0xd0], xmm11
140017dcf: movaps xmmword ptr [rsp + 0xe0], xmm10
140017dd8: movaps xmmword ptr [rsp + 0xf0], xmm9
140017de1: movaps xmmword ptr [rsp + 0x100], xmm8
140017dea: movaps xmmword ptr [rsp + 0x110], xmm7
140017df2: movaps xmmword ptr [rsp + 0x120], xmm6
140017dfa: mov rdi, qword ptr [rbp + 0x160]
140017e01: lea rsi, [rdi + 0x18]
140017e05: mov rcx, rsi
140017e08: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
140017e0d: xor ecx, ecx
140017e0f: mov al, 1
140017e11: lock cmpxchg byte ptr [rdi + 0x18], cl
140017e16: jne 0x140017ea6
140017e1c: mov rdi, qword ptr [rbp + 0x1b8]
140017e23: lea rsi, [rdi + 0x18]
140017e27: mov rcx, rsi
140017e2a: call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
140017e2f: xor ecx, ecx
140017e31: mov al, 1
140017e33: lock cmpxchg byte ptr [rdi + 0x18], cl
140017e38: jne 0x140017eb5
140017e3a: movaps xmm6, xmmword ptr [rsp + 0x120]
140017e42: movaps xmm7, xmmword ptr [rsp + 0x110]
140017e4a: movaps xmm8, xmmword ptr [rsp + 0x100]
140017e53: movaps xmm9, xmmword ptr [rsp + 0xf0]
140017e5c: movaps xmm10, xmmword ptr [rsp + 0xe0]
140017e65: movaps xmm11, xmmword ptr [rsp + 0xd0]
140017e6e: movaps xmm12, xmmword ptr [rsp + 0xc0]
140017e77: movaps xmm13, xmmword ptr [rsp + 0xb0]
140017e80: movaps xmm14, xmmword ptr [rsp + 0xa0]
140017e89: movaps xmm15, xmmword ptr [rsp + 0x90]
140017e92: add rsp, 0x138
140017e99: pop rbx
140017e9a: pop rdi
140017e9b: pop rsi
140017e9c: pop r12
140017e9e: pop r13
140017ea0: pop r14
140017ea2: pop r15
140017ea4: pop rbp
140017ea5: ret 
140017ea6: mov rcx, rsi
140017ea9: xor edx, edx
140017eab: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
140017eb0: jmp 0x140017e1c ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140017eb5: mov rcx, rsi
140017eb8: xor edx, edx
140017eba: call 0x1428cdb90 ; parking_lot::raw_mutex::RawMutex::unlock_slow
140017ebf: jmp 0x140017e3a ; system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true>
140017ec4: nop word ptr cs:[rax + rax]
140017ed0: mov qword ptr [rsp + 0x10], rdx
140017ed5: push rbp
140017ed6: push r15
140017ed8: push r14
140017eda: push r13
140017edc: push r12
140017ede: push rsi
140017edf: push rdi
140017ee0: push rbx
140017ee1: sub rsp, 0x138
140017ee8: lea rbp, [rdx + 0x80]
140017eef: movdqa xmmword ptr [rsp + 0x90], xmm15
140017ef9: movaps xmmword ptr [rsp + 0xa0], xmm14
140017f02: movdqa xmmword ptr [rsp + 0xb0], xmm13
140017f0c: movdqa xmmword ptr [rsp + 0xc0], xmm12
140017f16: movaps xmmword ptr [rsp + 0xd0], xmm11
140017f1f: movaps xmmword ptr [rsp + 0xe0], xmm10
140017f28: movaps xmmword ptr [rsp + 0xf0], xmm9
140017f31: movaps xmmword ptr [rsp + 0x100], xmm8
140017f3a: movaps xmmword ptr [rsp + 0x110], xmm7
140017f42: movaps xmmword ptr [rsp + 0x120], xmm6
140017f4a: cmp byte ptr [rbp + 0x797], 0
140017f51: je 0x140017f90
140017f53: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
140017f58: mov rsi, rax
140017f5b: mov rax, qword ptr [rax]
140017f5e: cmp rax, 1
140017f62: mov rdx, qword ptr [rbp + 0x270]
140017f69: jne 0x140017ffc
140017f6f: add rsi, 8
140017f73: cmp qword ptr [rsi], 0
140017f77: jne 0x140018029
140017f7d: mov qword ptr [rsi], 0xffffffffffffffff
140017f84: lea rcx, [rsi + 8]
140017f88: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
140017f8d: inc qword ptr [rsi]
140017f90: movaps xmm6, xmmword ptr [rsp + 0x120]
140017f98: movaps xmm7, xmmword ptr [rsp + 0x110]
140017fa0: movaps xmm8, xmmword ptr [rsp + 0x100]
140017fa9: movaps xmm9, xmmword ptr [rsp + 0xf0]
140017fb2: movaps xmm10, xmmword ptr [rsp + 0xe0]
140017fbb: movaps xmm11, xmmword ptr [rsp + 0xd0]
140017fc4: movaps xmm12, xmmword ptr [rsp + 0xc0]
140017fcd: movaps xmm13, xmmword ptr [rsp + 0xb0]
140017fd6: movaps xmm14, xmmword ptr [rsp + 0xa0]
140017fdf: movaps xmm15, xmmword ptr [rsp + 0x90]
140017fe8: add rsp, 0x138
140017fef: pop rbx
140017ff0: pop rdi
140017ff1: pop rsi
140017ff2: pop r12
140017ff4: pop r13
140017ff6: pop r14
140017ff8: pop r15
140017ffa: pop rbp
140017ffb: ret 
140017ffc: test rax, rax
140017fff: jne 0x14001801d
140018001: mov rcx, rsi
140018004: mov rsi, rdx
140018007: xor edx, edx
140018009: call 0x1428a1640 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14001800e: mov rdx, rsi
140018011: mov rsi, rax
140018014: test rax, rax
140018017: jne 0x140017f73
14001801d: lea rcx, [rip + 0x291a9ec]
140018024: call 0x1428d80d0 ; std::thread::local::panic_access_error
140018029: lea rcx, [rip + 0x291aa88]
140018030: call 0x1428d9250 ; core::cell::panic_already_borrowed
140018035: int3 
