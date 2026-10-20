142318fa0: push rbp
142318fa1: push r15
142318fa3: push r14
142318fa5: push r13
142318fa7: push r12
142318fa9: push rsi
142318faa: push rdi
142318fab: push rbx
142318fac: sub rsp, 0x378
142318fb3: lea rbp, [rsp + 0x80]
142318fbb: movaps xmmword ptr [rbp + 0x2e0], xmm15
142318fc3: movaps xmmword ptr [rbp + 0x2d0], xmm14
142318fcb: movaps xmmword ptr [rbp + 0x2c0], xmm13
142318fd3: movaps xmmword ptr [rbp + 0x2b0], xmm12
142318fdb: movaps xmmword ptr [rbp + 0x2a0], xmm11
142318fe3: movaps xmmword ptr [rbp + 0x290], xmm10
142318feb: movaps xmmword ptr [rbp + 0x280], xmm9
142318ff3: movaps xmmword ptr [rbp + 0x270], xmm8
142318ffb: movaps xmmword ptr [rbp + 0x260], xmm7
142319002: movaps xmmword ptr [rbp + 0x250], xmm6
142319009: mov qword ptr [rbp + 0x248], 0xfffffffffffffffe
142319014: mov r13, r9
142319017: mov rdi, r8
14231901a: mov rbx, rdx
14231901d: mov r15, rcx
142319020: call 0x140c16a00 ; puffin::are_scopes_on
142319025: test al, al
142319027: je 0x142319089
142319029: lea r12, [rip + 0xd31f05]
142319030: mov r14d, 0x26
142319036: mov edx, 0x26
14231903b: mov rcx, r12
14231903e: call 0x142317580 ; core::str::impl$0::rfind<ref$<str$> >
142319043: cmp rax, 1
142319047: jne 0x1423190ff
14231904d: mov r9, rdx
142319050: test rdx, rdx
142319053: je 0x142319092
142319055: cmp r9, 0x26
142319059: jae 0x142319090
14231905b: lea rax, [rip + 0xd31ed3]
142319062: cmp byte ptr [r9 + rax], 0xbf
142319067: jg 0x142319092
142319069: lea rax, [rip + 0xd31f70]
142319070: mov qword ptr [rsp + 0x20], rax
142319075: lea rcx, [rip + 0xd31eb9]
14231907c: mov edx, 0x26
142319081: xor r8d, r8d
142319084: call 0x1428d9940 ; core::str::slice_error_fail
142319089: xor eax, eax
14231908b: jmp 0x14231926b
142319090: jne 0x142319069
142319092: lea r12, [rip + 0xd31e9c]
142319099: mov rcx, r12
14231909c: mov rdx, r9
14231909f: call 0x142317580 ; core::str::impl$0::rfind<ref$<str$> >
1423190a4: cmp rax, 1
1423190a8: jne 0x1423190ff
1423190aa: mov r8, rdx
1423190ad: add r8, 2
1423190b1: je 0x1423190ec
1423190b3: cmp r8, 0x26
1423190b7: jae 0x1423190ea
1423190b9: lea rax, [rip + 0xd31e75]
1423190c0: cmp byte ptr [r8 + rax], 0xbf
1423190c5: jg 0x1423190ec
1423190c7: lea rax, [rip + 0xd31f2a]
1423190ce: mov qword ptr [rsp + 0x20], rax
1423190d3: lea rcx, [rip + 0xd31e5b]
1423190da: mov edx, 0x26
1423190df: mov r9d, 0x26
1423190e5: call 0x1428d9940 ; core::str::slice_error_fail
1423190ea: jne 0x1423190c7
1423190ec: mov r14d, 0x24
1423190f2: sub r14, rdx
1423190f5: lea r12, [rip + 0xd31e39]
1423190fc: add r12, r8
1423190ff: mov qword ptr [rbp + 0x210], rdi
142319106: mov rdi, r13
142319109: lea r8, [rip + 0xd31f3d]
142319110: lea r13, [rip + 0xd31f11]
142319117: nop word ptr [rax + rax]
142319120: movsx eax, byte ptr [r8 - 1]
142319125: test eax, eax
142319127: js 0x142319140
142319129: dec r8
14231912c: cmp eax, 0x5c
14231912f: jne 0x142319194
142319131: jmp 0x1423191a5
142319133: nop word ptr cs:[rax + rax]
142319140: movzx ecx, byte ptr [r8 - 2]
142319145: cmp cl, 0xc0
142319148: jge 0x14231916d
14231914a: movzx edx, byte ptr [r8 - 3]
14231914f: cmp dl, 0xc0
142319152: jge 0x142319176
142319154: movzx r9d, byte ptr [r8 - 4]
142319159: add r8, -4
14231915d: and r9d, 7
142319161: shl r9d, 6
142319165: and edx, 0x3f
142319168: or edx, r9d
14231916b: jmp 0x14231917d
14231916d: add r8, -2
142319171: and ecx, 0x1f
142319174: jmp 0x142319185
142319176: add r8, -3
14231917a: and edx, 0xf
14231917d: shl edx, 6
142319180: and ecx, 0x3f
142319183: or ecx, edx
142319185: shl ecx, 6
142319188: and al, 0x3f
14231918a: movzx eax, al
14231918d: or eax, ecx
14231918f: cmp eax, 0x5c
142319192: je 0x1423191a5
142319194: cmp eax, 0x2f
142319197: je 0x1423191a5
142319199: cmp r8, r13
14231919c: jne 0x142319120
14231919e: mov esi, 0x25
1423191a3: jmp 0x1423191f1
1423191a5: lea r13, [rip + 0xd31e7c]
1423191ac: sub r8, r13
1423191af: inc r8
1423191b2: je 0x1423191e6
1423191b4: cmp r8, 0x25
1423191b8: jae 0x1423191e4
1423191ba: cmp byte ptr [r8 + r13], 0xbf
1423191bf: jg 0x1423191e6
1423191c1: lea rax, [rip + 0xd31df8]
1423191c8: mov qword ptr [rsp + 0x20], rax
1423191cd: lea rcx, [rip + 0xd31e54]
1423191d4: mov edx, 0x25
1423191d9: mov r9d, 0x25
1423191df: call 0x1428d9940 ; core::str::slice_error_fail
1423191e4: jne 0x1423191c1
1423191e6: mov esi, 0x25
1423191eb: sub rsi, r8
1423191ee: add r13, r8
1423191f1: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
1423191f6: mov rcx, rax
1423191f9: mov rax, qword ptr [rax]
1423191fc: cmp rax, 1
142319200: jne 0x14231a316
142319206: add rcx, 8
14231920a: cmp qword ptr [rcx], 0
14231920e: jne 0x14231a33a
142319214: mov qword ptr [rcx], 0xffffffffffffffff
14231921b: mov qword ptr [rbp + 0x218], rcx
142319222: add rcx, 8
142319226: mov qword ptr [rsp + 0x20], rsi
14231922b: mov qword ptr [rsp + 0x30], 0
142319234: mov qword ptr [rsp + 0x28], 1
14231923d: mov rdx, r12
142319240: mov r8, r14
142319243: mov r9, r13
142319246: call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
14231924b: mov rcx, qword ptr [rbp + 0x218]
142319252: inc qword ptr [rcx]
142319255: mov qword ptr [rbp + 0x178], rax
14231925c: mov eax, 1
142319261: mov r13, rdi
142319264: mov rdi, qword ptr [rbp + 0x210]
14231926b: mov rcx, qword ptr [rbp + 0x360]
142319272: mov qword ptr [rbp + 0x170], rax
142319279: mov rax, qword ptr [rbx]
14231927c: movss xmm14, dword ptr [rax + 0x28]
142319282: mov rcx, qword ptr [rcx]
142319285: lea rdx, [rip + 0xd3105c]
14231928c: call 0x14230e780 ; frozen::FrozenMap::get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId> >,country_core::asset_libraries::ssbo_library::SsboId>
142319291: mov rbx, rax
142319294: test rax, rax
142319297: je 0x14231a37f
14231929d: lea rax, [rip + 0xd3112c]
1423192a4: mov qword ptr [rbp + 0x1b0], rax
1423192ab: mov qword ptr [rbp + 0x1b8], 0x3f
1423192b6: mov r14, rbx
1423192b9: add r14, 0x10
1423192bd: cmp qword ptr [rbx + 0x18], 0x3f
1423192c2: jne 0x14231a346
1423192c8: mov rdx, qword ptr [r14]
1423192cb: lea rcx, [rip + 0xd310fe]
1423192d2: mov r8d, 0x3f
1423192d8: call 0x142923bf0 ; 
1423192dd: test eax, eax
1423192df: jne 0x14231a346
1423192e5: mov rcx, qword ptr [rbx]
1423192e8: mov rax, qword ptr [rbx + 8]
1423192ec: call qword ptr [rax + 0x18]
1423192ef: test rax, rax
1423192f2: je 0x14231a37f
1423192f8: mov rcx, qword ptr [rbp + 0x368]
1423192ff: mov rdx, qword ptr [rax]
142319302: mov qword ptr [rbp + 0x130], rdx
142319309: mov rax, qword ptr [rax + 8]
14231930d: mov qword ptr [rbp + 0x138], rax
142319314: mov rcx, qword ptr [rcx]
142319317: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
14231931c: mov rcx, qword ptr [rbp + 0x370]
142319323: lea rdx, [rax + 8]
142319327: lea r8, [rax + 0x20]
14231932b: mov r9, rax
14231932e: add r9, 0x28
142319332: cmp byte ptr [rax + 0x298], 0
142319339: cmovne rdx, r8
14231933d: lea rax, [rax + 0x10]
142319341: cmove r9, rax
142319345: mov rax, qword ptr [r9]
142319348: mov rdx, qword ptr [rdx]
14231934b: mov qword ptr [rbp + 0xe8], rdx
142319352: mov qword ptr [rbp + 0xf0], rax
142319359: mov rcx, qword ptr [rcx]
14231935c: call 0x14067dd80 ; country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
142319361: lea rcx, [rax + 8]
142319365: lea rdx, [rax + 0x20]
142319369: mov r8, rax
14231936c: add r8, 0x28
142319370: cmp byte ptr [rax + 0x298], 0
142319377: cmovne rcx, rdx
14231937b: lea rax, [rax + 0x10]
14231937f: cmove r8, rax
142319383: mov rax, qword ptr [r8]
142319386: mov rcx, qword ptr [rcx]
142319389: mov qword ptr [rbp + 0x118], rcx
142319390: mov qword ptr [rbp + 0x120], rax
142319397: mov r8, qword ptr [r15]
14231939a: mov rdx, qword ptr [r15 + 8]
14231939e: mov r9d, dword ptr [r15 + 0x10]
1423193a2: mov eax, dword ptr [r15 + 0x14]
1423193a6: lea rcx, [rbp + 0x10]
1423193aa: mov dword ptr [rsp + 0x20], eax
1423193ae: mov qword ptr [rbp + 0x210], rdx
1423193b5: mov qword ptr [rbp + 0x1a8], r8
1423193bc: call 0x14230ecf0 ; bevy_ecs::query::iter::QueryIterationCursor::init<tuple$<ref_mut$<utils::components::transform::Transform>,ref$<enum2$<country_core::resources::critters::CritterLocomotion> >,ref$<country_core::resources::critters::Velocity>,enum2$<core::option::Option<ref$<country_core::resources::critters::CritterVelocityMultiplier> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::Duck> > >,enum2$<core::option::Option<ref$<country_core::resources::critters::sheep::Sheep> > >,enum2$<core::option::Option<ref_mut$<country_core::resources::critters::BirdSpots> > > >,tuple$<> >
1423193c1: mov rsi, qword ptr [rbp + 0x378]
1423193c8: movss xmm0, dword ptr [rip + 0x6472a4]
1423193d0: movaps xmm1, xmm14
1423193d4: call 0x142923f70 ; 
1423193d9: movaps xmm7, xmm0
1423193dc: mov r15, qword ptr [rbp + 0x10]
1423193e0: mov rax, qword ptr [rbp + 0x18]
1423193e4: mov qword ptr [rbp + 0x180], rax
1423193eb: mov rax, qword ptr [rbp + 0x30]
1423193ef: mov qword ptr [rbp + 0x188], rax
1423193f6: mov rbx, qword ptr [rbp + 0x40]
1423193fa: mov rax, qword ptr [rbp + 0x48]
1423193fe: mov qword ptr [rbp + 0x218], rax
142319405: mov rax, qword ptr [rbp + 0x50]
142319409: mov qword ptr [rbp + 0x1f0], rax
142319410: movzx eax, byte ptr [rbp + 0x58]
142319414: mov dword ptr [rbp + 0x240], eax
14231941a: movzx eax, byte ptr [rbp + 0x68]
14231941e: mov dword ptr [rbp + 0x238], eax
142319424: movzx eax, byte ptr [rbp + 0x78]
142319428: mov dword ptr [rbp + 0x23c], eax
14231942e: mov rax, qword ptr [rbp + 0x80]
142319435: mov qword ptr [rbp + 0x1c8], rax
14231943c: mov rax, qword ptr [rdi]
14231943f: mov qword ptr [rbp + 0x150], rax
142319446: mov rax, qword ptr [r13]
14231944a: mov qword ptr [rbp + 0x128], rax
142319451: mov rax, qword ptr [rsi]
142319454: mov qword ptr [rbp + 0x158], rax
14231945b: movss xmm10, dword ptr [rip + 0x60c4a0]
142319464: movaps xmm8, xmm10
142319468: subss xmm8, xmm0
14231946d: shufps xmm8, xmm8, 0
142319472: shufps xmm7, xmm0, 0
142319476: movsldup xmm13, xmm14
14231947b: xorps xmm6, xmm6
14231947e: movss xmm15, dword ptr [rip + 0x7621d1]
142319487: movzx eax, byte ptr [rbp + 0xd8]
14231948e: mov byte ptr [rbp + 0x237], al
142319494: mov rax, qword ptr [rbp + 0x90]
14231949b: mov qword ptr [rbp + 0x1d0], rax
1423194a2: mov eax, dword ptr [rbp + 0x9c]
1423194a8: mov dword ptr [rbp + 0x1fc], eax
1423194ae: mov rax, qword ptr [rbp + 0xa0]
1423194b5: mov qword ptr [rbp + 0x220], rax
1423194bc: mov rax, qword ptr [rbp + 0xa8]
1423194c3: mov qword ptr [rbp + 0x228], rax
1423194ca: mov rax, qword ptr [rbp + 0xb0]
1423194d1: mov qword ptr [rbp + 0x1e0], rax
1423194d8: mov rax, qword ptr [rbp + 0xc0]
1423194df: mov qword ptr [rbp + 0x1e8], rax
1423194e6: mov eax, dword ptr [rbp + 0xcc]
1423194ec: mov dword ptr [rbp + 0x1a4], eax
1423194f2: movzx eax, byte ptr [rbp + 0xd0]
1423194f9: mov dword ptr [rbp + 0x244], eax
1423194ff: mov rdi, qword ptr [rbp + 0x1a8]
142319506: test byte ptr [rbp + 0x237], 1
14231950d: je 0x1423195c0
142319513: mov rax, qword ptr [rbp + 0x218]
14231951a: cmp rax, rbx
14231951d: mov rdx, qword ptr [rbp + 0x180]
142319524: jne 0x142319959
14231952a: nop word ptr [rax + rax]
142319530: cmp r15, rdx
142319533: je 0x14231a245
142319539: mov ecx, dword ptr [r15]
14231953c: add r15, 4
142319540: mov rax, qword ptr [rbp + 0x210]
142319547: mov rax, qword ptr [rax + 0x1a8]
14231954e: lea rcx, [rcx + rcx*8]
142319552: mov rbx, qword ptr [rax + rcx*8 + 0x10]
142319557: test rbx, rbx
14231955a: je 0x142319530
14231955c: lea rcx, [rax + rcx*8]
142319560: mov rdx, qword ptr [rcx + 0x40]
142319564: mov rax, qword ptr [rcx + 0x18]
142319568: mov rcx, qword ptr [rcx + 0x38]
14231956c: mov r8, qword ptr [rdi + 0x118]
142319573: cmp rdx, r8
142319576: jbe 0x14231967b
14231957c: mov r8, qword ptr [rcx + r8*8]
142319580: test r8, r8
142319583: je 0x14231967b
142319589: not r8
14231958c: lea r8, [r8 + r8*2]
142319590: shl r8, 4
142319594: mov r8, qword ptr [rax + r8 + 0x10]
142319599: mov qword ptr [rbp + 0x220], r8
1423195a0: mov r8, qword ptr [rdi + 0x120]
1423195a7: cmp rdx, r8
1423195aa: ja 0x142319696
1423195b0: jmp 0x142319727
1423195b5: nop word ptr cs:[rax + rax]
1423195c0: mov rax, qword ptr [rbp + 0x218]
1423195c7: cmp rax, rbx
1423195ca: mov rdx, qword ptr [rbp + 0x180]
1423195d1: jne 0x142319ad4
1423195d7: nop word ptr [rax + rax]
1423195e0: cmp r15, rdx
1423195e3: je 0x14231a245
1423195e9: mov ecx, dword ptr [r15]
1423195ec: add r15, 4
1423195f0: mov rax, qword ptr [rbp + 0x210]
1423195f7: mov rax, qword ptr [rax + 0x100]
1423195fe: lea rcx, [rcx + rcx*4]
142319602: shl rcx, 5
142319606: mov rbx, qword ptr [rax + rcx + 0x58]
14231960b: test rbx, rbx
14231960e: je 0x1423195e0
142319610: add rax, rcx
142319613: mov ecx, dword ptr [rax + 0x94]
142319619: mov rdx, qword ptr [rbp + 0x210]
142319620: mov rdx, qword ptr [rdx + 0x1a8]
142319627: lea r8, [rcx + rcx*8]
14231962b: mov r9, qword ptr [rdx + r8*8 + 0x40]
142319630: mov rcx, qword ptr [rdx + r8*8 + 0x18]
142319635: mov rdx, qword ptr [rdx + r8*8 + 0x38]
14231963a: mov r8, qword ptr [rdi + 0x118]
142319641: cmp r9, r8
142319644: jbe 0x1423196cc
14231964a: mov r8, qword ptr [rdx + r8*8]
14231964e: test r8, r8
142319651: je 0x1423196cc
142319653: not r8
142319656: lea r8, [r8 + r8*2]
14231965a: shl r8, 4
14231965e: mov r8, qword ptr [rcx + r8 + 0x10]
142319663: mov qword ptr [rbp + 0x220], r8
14231966a: mov r8, qword ptr [rdi + 0x120]
142319671: cmp r9, r8
142319674: ja 0x1423196e7
142319676: jmp 0x1423197af
14231967b: mov qword ptr [rbp + 0x220], 0
142319686: mov r8, qword ptr [rdi + 0x120]
14231968d: cmp rdx, r8
142319690: jbe 0x142319727
142319696: mov r8, qword ptr [rcx + r8*8]
14231969a: test r8, r8
14231969d: je 0x142319727
1423196a3: not r8
1423196a6: lea r8, [r8 + r8*2]
1423196aa: shl r8, 4
1423196ae: mov r8, qword ptr [rax + r8 + 0x10]
1423196b3: mov qword ptr [rbp + 0x228], r8
1423196ba: mov r8, qword ptr [rdi + 0x128]
1423196c1: cmp rdx, r8
1423196c4: jbe 0x14231982f
1423196ca: jmp 0x142319742
1423196cc: mov qword ptr [rbp + 0x220], 0
1423196d7: mov r8, qword ptr [rdi + 0x120]
1423196de: cmp r9, r8
1423196e1: jbe 0x1423197af
1423196e7: mov r8, qword ptr [rdx + r8*8]
1423196eb: test r8, r8
1423196ee: je 0x1423197af
1423196f4: not r8
1423196f7: lea r8, [r8 + r8*2]
1423196fb: shl r8, 4
1423196ff: mov r8, qword ptr [rcx + r8 + 0x10]
142319704: mov qword ptr [rbp + 0x228], r8
14231970b: mov r10, qword ptr [rdi + 0x128]
142319712: mov r8, qword ptr [rax + 0x88]
142319719: cmp r8, r10
14231971c: jbe 0x1423198ed
142319722: jmp 0x1423197d1
142319727: mov qword ptr [rbp + 0x228], 0
142319732: mov r8, qword ptr [rdi + 0x128]
142319739: cmp rdx, r8
14231973c: jbe 0x14231982f
142319742: mov r8, qword ptr [rcx + r8*8]
142319746: test r8, r8
142319749: je 0x14231982f
14231974f: not r8
142319752: lea r8, [r8 + r8*2]
142319756: shl r8, 4
14231975a: mov r8, qword ptr [rax + r8 + 0x10]
14231975f: mov qword ptr [rbp + 0x1f0], r8
142319766: mov r8b, 1
142319769: mov dword ptr [rbp + 0x240], r8d
142319770: mov r8, qword ptr [rdi + 0x130]
142319777: cmp rdx, r8
14231977a: ja 0x142319849
142319780: mov dword ptr [rbp + 0x238], 0
14231978a: mov r8, qword ptr [rdi + 0x138]
142319791: cmp rdx, r8
142319794: jbe 0x142319869
14231979a: cmp qword ptr [rcx + r8*8], 0
14231979f: setne r8b
1423197a3: mov dword ptr [rbp + 0x23c], r8d
1423197aa: jmp 0x142319873
1423197af: mov qword ptr [rbp + 0x228], 0
1423197ba: mov r10, qword ptr [rdi + 0x128]
1423197c1: mov r8, qword ptr [rax + 0x88]
1423197c8: cmp r8, r10
1423197cb: jbe 0x1423198ed
1423197d1: mov r11, qword ptr [rax + 0x80]
1423197d8: cmp qword ptr [r11 + r10*8], 0
1423197dd: je 0x1423198ed
1423197e3: mov r11b, 1
1423197e6: mov dword ptr [rbp + 0x240], r11d
1423197ed: cmp r9, r10
1423197f0: jbe 0x1423199f6
1423197f6: mov r9, qword ptr [rdx + r10*8]
1423197fa: test r9, r9
1423197fd: je 0x1423199f6
142319803: not r9
142319806: lea r9, [r9 + r9*2]
14231980a: shl r9, 4
14231980e: mov r9, qword ptr [rcx + r9 + 0x10]
142319813: mov qword ptr [rbp + 0x1f0], r9
14231981a: mov r9, qword ptr [rdi + 0x130]
142319821: cmp r8, r9
142319824: ja 0x142319907
14231982a: jmp 0x142319a11
14231982f: mov dword ptr [rbp + 0x240], 0
142319839: mov r8, qword ptr [rdi + 0x130]
142319840: cmp rdx, r8
142319843: jbe 0x142319780
142319849: cmp qword ptr [rcx + r8*8], 0
14231984e: setne r8b
142319852: mov dword ptr [rbp + 0x238], r8d
142319859: mov r8, qword ptr [rdi + 0x138]
142319860: cmp rdx, r8
142319863: ja 0x14231979a
142319869: mov dword ptr [rbp + 0x23c], 0
142319873: mov r8, qword ptr [rdi + 0x110]
14231987a: mov r8, qword ptr [rcx + r8*8]
14231987e: not r8
142319881: lea r8, [r8 + r8*2]
142319885: shl r8, 4
142319889: mov r9, qword ptr [rax + r8 + 0x10]
14231988e: mov qword ptr [rbp + 0x1c8], r9
142319895: mov r8, qword ptr [rax + r8 + 0x28]
14231989a: mov qword ptr [rbp + 0x1d0], r8
1423198a1: mov r8, qword ptr [rdi + 0x140]
1423198a8: cmp rdx, r8
1423198ab: jbe 0x14231994d
1423198b1: mov rcx, qword ptr [rcx + r8*8]
1423198b5: test rcx, rcx
1423198b8: je 0x14231994d
1423198be: not rcx
1423198c1: lea rcx, [rcx + rcx*2]
1423198c5: shl rcx, 4
1423198c9: mov rdx, qword ptr [rax + rcx + 0x10]
1423198ce: mov qword ptr [rbp + 0x1e0], rdx
1423198d5: mov rax, qword ptr [rax + rcx + 0x28]
1423198da: mov qword ptr [rbp + 0x1e8], rax
1423198e1: mov al, 1
1423198e3: mov dword ptr [rbp + 0x244], eax
1423198e9: xor eax, eax
1423198eb: jmp 0x142319959
1423198ed: mov dword ptr [rbp + 0x240], 0
1423198f7: mov r9, qword ptr [rdi + 0x130]
1423198fe: cmp r8, r9
142319901: jbe 0x142319a11
142319907: mov r10, qword ptr [rax + 0x80]
14231990e: cmp qword ptr [r10 + r9*8], 0
142319913: setne r9b
142319917: mov dword ptr [rbp + 0x238], r9d
14231991e: mov r9, qword ptr [rdi + 0x138]
142319925: cmp r8, r9
142319928: ja 0x142319a2b
14231992e: mov dword ptr [rbp + 0x23c], 0
142319938: mov r9, qword ptr [rdi + 0x140]
14231993f: cmp r8, r9
142319942: jbe 0x142319a8f
142319948: jmp 0x142319a4e
14231994d: xor eax, eax
14231994f: mov dword ptr [rbp + 0x244], 0
142319959: mov qword ptr [rbp + 0x218], rax
142319960: mov edx, eax
142319962: lea r8, [rdx + rdx*2]
142319966: mov rsi, r8
142319969: shl rsi, 4
14231996d: add rsi, qword ptr [rbp + 0x1c8]
142319974: mov rax, qword ptr [rbp + 0x1d0]
14231997b: lea rax, [rax + rdx*4]
14231997f: mov qword ptr [rbp + 0x190], rax
142319986: mov rax, qword ptr [rbp + 0x220]
14231998d: lea r13, [rax + rdx*8]
142319991: mov rax, qword ptr [rbp + 0x228]
142319998: lea rax, [rax + r8*4]
14231999c: mov rcx, qword ptr [rbp + 0x1f0]
1423199a3: lea rcx, [rcx + rdx*4]
1423199a7: test byte ptr [rbp + 0x240], 1
1423199ae: mov r9d, 0
1423199b4: cmove rcx, r9
1423199b8: test byte ptr [rbp + 0x244], 1
1423199bf: je 0x1423199ea
1423199c1: mov r9, qword ptr [rbp + 0x1e0]
1423199c8: lea r8, [r9 + r8*8]
1423199cc: mov qword ptr [rbp + 0x1d8], r8
1423199d3: mov r8, qword ptr [rbp + 0x1e8]
1423199da: lea rdx, [r8 + rdx*4]
1423199de: mov qword ptr [rbp + 0x140], rdx
1423199e5: jmp 0x142319b6d
1423199ea: mov r9, qword ptr [rbp + 0x140]
1423199f1: jmp 0x142319b79
1423199f6: mov qword ptr [rbp + 0x1f0], 0
142319a01: mov r9, qword ptr [rdi + 0x130]
142319a08: cmp r8, r9
142319a0b: ja 0x142319907
142319a11: mov dword ptr [rbp + 0x238], 0
142319a1b: mov r9, qword ptr [rdi + 0x138]
142319a22: cmp r8, r9
142319a25: jbe 0x14231992e
142319a2b: mov r10, qword ptr [rax + 0x80]
142319a32: cmp qword ptr [r10 + r9*8], 0
142319a37: setne r9b
142319a3b: mov dword ptr [rbp + 0x23c], r9d
142319a42: mov r9, qword ptr [rdi + 0x140]
142319a49: cmp r8, r9
142319a4c: jbe 0x142319a8f
142319a4e: mov r8, qword ptr [rax + 0x80]
142319a55: cmp qword ptr [r8 + r9*8], 0
142319a5a: je 0x142319a8f
142319a5c: mov r8, qword ptr [rdx + r9*8]
142319a60: not r8
142319a63: lea r8, [r8 + r8*2]
142319a67: shl r8, 4
142319a6b: mov r9, qword ptr [rcx + r8 + 0x10]
142319a70: mov qword ptr [rbp + 0x1e0], r9
142319a77: mov r8, qword ptr [rcx + r8 + 0x28]
142319a7c: mov qword ptr [rbp + 0x1e8], r8
142319a83: mov r8b, 1
142319a86: mov dword ptr [rbp + 0x244], r8d
142319a8d: jmp 0x142319a99
142319a8f: mov dword ptr [rbp + 0x244], 0
142319a99: mov r8, qword ptr [rdi + 0x110]
142319aa0: mov rdx, qword ptr [rdx + r8*8]
142319aa4: not rdx
142319aa7: lea rdx, [rdx + rdx*2]
142319aab: shl rdx, 4
142319aaf: mov r8, qword ptr [rcx + rdx + 0x10]
142319ab4: mov qword ptr [rbp + 0x1c8], r8
142319abb: mov rcx, qword ptr [rcx + rdx + 0x28]
142319ac0: mov qword ptr [rbp + 0x1d0], rcx
142319ac7: mov rax, qword ptr [rax + 0x50]
142319acb: mov qword ptr [rbp + 0x188], rax
142319ad2: xor eax, eax
142319ad4: mov qword ptr [rbp + 0x218], rax
142319adb: shl rax, 4
142319adf: mov rcx, qword ptr [rbp + 0x188]
142319ae6: mov edx, dword ptr [rcx + rax + 8]
142319aea: lea r8, [rdx + rdx*2]
142319aee: mov rsi, r8
142319af1: shl rsi, 4
142319af5: add rsi, qword ptr [rbp + 0x1c8]
142319afc: mov rax, qword ptr [rbp + 0x1d0]
142319b03: lea rax, [rax + rdx*4]
142319b07: mov qword ptr [rbp + 0x190], rax
142319b0e: mov rax, qword ptr [rbp + 0x220]
142319b15: lea r13, [rax + rdx*8]
142319b19: mov rax, qword ptr [rbp + 0x228]
142319b20: lea rax, [rax + r8*4]
142319b24: mov rcx, qword ptr [rbp + 0x1f0]
142319b2b: lea rcx, [rcx + rdx*4]
142319b2f: test byte ptr [rbp + 0x240], 1
142319b36: mov r9d, 0
142319b3c: cmove rcx, r9
142319b40: test byte ptr [rbp + 0x244], 1
142319b47: je 0x142319b72
142319b49: mov r9, qword ptr [rbp + 0x1e0]
142319b50: lea r8, [r9 + r8*8]
142319b54: mov qword ptr [rbp + 0x1d8], r8
142319b5b: mov r8, qword ptr [rbp + 0x1e8]
142319b62: lea rdx, [r8 + rdx*4]
142319b66: mov qword ptr [rbp + 0x148], rdx
142319b6d: mov r9, rdx
142319b70: jmp 0x142319b84
142319b72: mov r9, qword ptr [rbp + 0x148]
142319b79: mov qword ptr [rbp + 0x1d8], 0
142319b84: inc qword ptr [rbp + 0x218]
142319b8b: movss xmm0, dword ptr [rsi + 0x14]
142319b90: maxss xmm0, xmm6
142319b94: mov edx, dword ptr [rbp + 0x1fc]
142319b9a: mov r8, qword ptr [rbp + 0x190]
142319ba1: mov dword ptr [r8], edx
142319ba4: movss dword ptr [rsi + 0x14], xmm0
142319ba9: movaps xmm0, xmm10
142319bad: test rcx, rcx
142319bb0: je 0x142319bc1
142319bb2: movss xmm0, dword ptr [rcx]
142319bb6: ucomiss xmm0, xmm6
142319bb9: jne 0x142319bc1
142319bbb: jnp 0x142319506
142319bc1: mov qword ptr [rbp + 0x160], r9
142319bc8: movss xmm4, dword ptr [rax + 8]
142319bcd: mulss xmm4, xmm14
142319bd2: mulss xmm4, xmm0
142319bd6: movsd xmm1, qword ptr [rax]
142319bda: mulps xmm1, xmm13
142319bde: movsldup xmm3, xmm0
142319be2: mulps xmm3, xmm1
142319be5: movlps qword ptr [rbp + 0x200], xmm3
142319bec: movss dword ptr [rbp + 0x208], xmm4
142319bf4: movss xmm1, dword ptr [rsi + 0x10]
142319bf9: movss xmm2, dword ptr [rsi + 0x18]
142319bfe: addss xmm3, xmm1
142319c02: addss xmm4, xmm2
142319c06: movss dword ptr [rsp + 0x20], xmm4
142319c0c: lea rcx, [rbp - 0x10]
142319c10: call 0x140c97560 ; utils::geometry::circle::Circle2d::new_rotated
142319c15: lea rcx, [rbp - 0x48]
142319c19: lea rdx, [rbp - 0x10]
142319c1d: call 0x14261f450 ; utils::geometry::line_relation::LineInterval::line_segment
142319c22: movss xmm1, dword ptr [rsi + 0x10]
142319c27: movss xmm2, dword ptr [rsi + 0x18]
142319c2c: addss xmm1, dword ptr [rbp + 0x200]
142319c34: addss xmm2, dword ptr [rbp + 0x208]
142319c3c: mov dword ptr [rsp + 0x20], 0x43020000
142319c44: lea rcx, [rbp + 0xe8]
142319c4b: movaps xmm3, xmm15
142319c4f: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
142319c54: movaps xmm9, xmm0
142319c58: cmp dword ptr [r13], 2
142319c5d: jne 0x142319cb0
142319c5f: movss xmm2, dword ptr [rsi + 0x10]
142319c64: movss xmm3, dword ptr [rsi + 0x18]
142319c69: addss xmm2, dword ptr [rbp + 0x200]
142319c71: addss xmm3, dword ptr [rbp + 0x208]
142319c79: mov rcx, qword ptr [rbp + 0x130]
142319c80: mov rdx, qword ptr [rbp + 0x138]
142319c87: call 0x1409ea800 ; country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz
142319c8c: ucomiss xmm9, xmm6
142319c90: setbe cl
142319c93: seta dl
142319c96: or al, dl
142319c98: jne 0x1423194ff
142319c9e: and cl, byte ptr [rbp + 0x238]
142319ca4: je 0x142319cc3
142319ca6: jmp 0x1423194ff
142319cab: nop dword ptr [rax + rax]
142319cb0: ucomiss xmm9, xmm6
142319cb4: ja 0x142319cc3
142319cb6: test byte ptr [rbp + 0x238], 1
142319cbd: jne 0x1423194ff
142319cc3: test byte ptr [rbp + 0x23c], 1
142319cca: jne 0x142319da4
142319cd0: test byte ptr [rbp + 0x238], 1
142319cd7: jne 0x142319da4
142319cdd: movss xmm1, dword ptr [rsi + 0x10]
142319ce2: movss xmm2, dword ptr [rsi + 0x18]
142319ce7: mov rcx, qword ptr [rbp + 0x150]
142319cee: call 0x140aa5730 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query
142319cf3: mov r14, rax
142319cf6: imul r12, rdx, 0x38
142319cfa: add r12, rax
142319cfd: nop dword ptr [rax]
142319d00: cmp r14, r12
142319d03: je 0x142319dbf
142319d09: mov rdi, r14
142319d0c: add r14, 0x38
142319d10: cmp dword ptr [r13], 3
142319d15: je 0x142319d49
142319d17: mov rdx, qword ptr [rdi + 0x28]
142319d1b: mov rcx, qword ptr [rbp + 0x128]
142319d22: lea r8, [rip + 0xd3139f]
142319d29: call 0x140a91ef0 ; country_core::resources::walls::public_walls::PublicWalls::get
142319d2e: test rax, rax
142319d31: je 0x14231a371
142319d37: mov rcx, rax
142319d3a: call 0x140b2a680 ; country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
142319d3f: movss xmm1, dword ptr [rsi + 0x14]
142319d44: ucomiss xmm1, xmm0
142319d47: ja 0x142319d00
142319d49: movss xmm1, dword ptr [rdi]
142319d4d: movss xmm2, dword ptr [rdi + 4]
142319d52: movss xmm3, dword ptr [rdi + 8]
142319d57: movss xmm0, dword ptr [rdi + 0xc]
142319d5c: movss dword ptr [rsp + 0x20], xmm0
142319d62: lea rcx, [rbp + 0x1b0]
142319d69: call 0x140c97560 ; utils::geometry::circle::Circle2d::new_rotated
142319d6e: lea rcx, [rbp - 0x10]
142319d72: lea rdx, [rbp + 0x1b0]
142319d79: call 0x14261f450 ; utils::geometry::line_relation::LineInterval::line_segment
142319d7e: lea rcx, [rbp + 0x1b0]
142319d85: lea rdx, [rbp - 0x48]
142319d89: lea r8, [rbp - 0x10]
142319d8d: call 0x14261f470 ; utils::geometry::line_relation::LineInterval::relate
142319d92: cmp dword ptr [rbp + 0x1b0], 0
142319d99: jne 0x142319d00
142319d9f: jmp 0x1423194ff
142319da4: mov eax, dword ptr [r13]
142319da8: cmp eax, 6
142319dab: ja 0x14231a0d0
142319db1: mov ecx, 0x63
142319db6: bt ecx, eax
142319db9: jae 0x14231a0d0
142319dbf: mov eax, dword ptr [rbp + 0x1fc]
142319dc5: mov rcx, qword ptr [rbp + 0x190]
142319dcc: mov dword ptr [rcx], eax
142319dce: movss xmm0, dword ptr [rbp + 0x208]
142319dd6: movsd xmm1, qword ptr [rbp + 0x200]
142319dde: movsd xmm2, qword ptr [rsi + 0x10]
142319de3: addps xmm2, xmm1
142319de6: movlps qword ptr [rsi + 0x10], xmm2
142319dea: addss xmm0, dword ptr [rsi + 0x18]
142319def: movss dword ptr [rsi + 0x18], xmm0
142319df4: lea rcx, [rbp - 0x10]
142319df8: lea rdx, [rbp + 0x200]
142319dff: call 0x140c90030 ; utils::swizzlers::impl$3::x0z
142319e04: movsd xmm1, qword ptr [rbp - 0x10]
142319e09: movss xmm0, dword ptr [rbp - 8]
142319e0e: movaps xmm2, xmm1
142319e11: mulps xmm2, xmm1
142319e14: movshdup xmm3, xmm2
142319e18: addss xmm3, xmm2
142319e1c: movaps xmm2, xmm0
142319e1f: mulss xmm2, xmm0
142319e23: addss xmm2, xmm3
142319e27: xorps xmm3, xmm3
142319e2a: sqrtss xmm3, xmm2
142319e2e: movaps xmm2, xmm10
142319e32: divss xmm2, xmm3
142319e36: movd eax, xmm2
142319e3a: test eax, eax
142319e3c: sets cl
142319e3f: mov edx, eax
142319e41: and edx, 0x7fffffff
142319e47: add edx, 0xff800000
142319e4d: cmp edx, 0x7f000000
142319e53: setae dl
142319e56: or dl, cl
142319e58: dec eax
142319e5a: cmp eax, 0x7fffff
142319e5f: setae al
142319e62: test al, dl
142319e64: jne 0x142319f20
142319e6a: mulss xmm0, xmm2
142319e6e: movsldup xmm2, xmm2
142319e72: mulps xmm1, xmm2
142319e75: movlps qword ptr [rbp + 0x108], xmm1
142319e7c: movss dword ptr [rbp + 0x110], xmm0
142319e84: movsd xmm0, qword ptr [rip + 0x6b34a4]
142319e8c: movsd qword ptr [rbp + 0x1b0], xmm0
142319e94: mov dword ptr [rbp + 0x1b8], 0xbf800000
142319e9e: lea rcx, [rbp - 0x10]
142319ea2: lea rdx, [rbp + 0x1b0]
142319ea9: lea r8, [rbp + 0x108]
142319eb0: call 0x140d4ae30 ; glam::f32::sse2::quat::Quat::from_rotation_arc
142319eb5: movaps xmm0, xmmword ptr [rbp - 0x10]
142319eb9: movaps xmm1, xmmword ptr [rsi]
142319ebc: movaps xmm2, xmm8
142319ec0: mulps xmm2, xmm0
142319ec3: mulps xmm0, xmm1
142319ec6: movaps xmm3, xmm0
142319ec9: unpckhpd xmm3, xmm0
142319ecd: addps xmm3, xmm0
142319ed0: movshdup xmm0, xmm3
142319ed4: addss xmm0, xmm3
142319ed8: shufps xmm0, xmm0, 0
142319edc: andps xmm0, xmmword ptr [rip + 0x60ba0d]
142319ee3: xorps xmm0, xmm1
142319ee6: mulps xmm0, xmm7
142319ee9: addps xmm0, xmm2
142319eec: movaps xmm1, xmm0
142319eef: mulps xmm1, xmm0
142319ef2: movaps xmm2, xmm1
142319ef5: unpckhpd xmm2, xmm1
142319ef9: addps xmm2, xmm1
142319efc: movshdup xmm1, xmm2
142319f00: addss xmm1, xmm2
142319f04: shufps xmm1, xmm1, 0
142319f08: sqrtps xmm1, xmm1
142319f0b: divps xmm0, xmm1
142319f0e: mov eax, dword ptr [rbp + 0x1fc]
142319f14: mov rcx, qword ptr [rbp + 0x190]
142319f1b: mov dword ptr [rcx], eax
142319f1d: movaps xmmword ptr [rsi], xmm0
142319f20: mov rcx, qword ptr [rbp + 0x1d8]
142319f27: test rcx, rcx
142319f2a: je 0x1423194ff
142319f30: movups xmm11, xmmword ptr [rsi + 0x1c]
142319f35: movups xmm2, xmmword ptr [rsi + 0x20]
142319f39: movaps xmm12, xmmword ptr [rsi]
142319f3d: movaps xmm0, xmmword ptr [rsi + 0x10]
142319f41: mov eax, dword ptr [rbp + 0x1a4]
142319f47: mov rdx, qword ptr [rbp + 0x160]
142319f4e: mov dword ptr [rdx], eax
142319f50: mov rax, qword ptr [rcx + 8]
142319f54: test rax, rax
142319f57: je 0x1423194ff
142319f5d: movaps xmmword ptr [rbp - 0x20], xmm13
142319f62: movaps xmm13, xmm12
142319f66: unpckhpd xmm13, xmm12
142319f6b: movaps xmm9, xmm13
142319f6f: addss xmm9, xmm13
142319f74: movaps xmm5, xmm12
142319f78: shufps xmm5, xmm12, 0xff
142319f7d: movsldup xmm4, xmm9
142319f82: mulps xmm4, xmm12
142319f86: movaps xmm6, xmm15
142319f8a: movaps xmm15, xmm12
142319f8e: addps xmm15, xmm12
142319f92: movaps xmmword ptr [rbp + 0x160], xmm8
142319f9a: movaps xmm8, xmm5
142319f9e: mulps xmm8, xmm15
142319fa2: shufps xmm8, xmm8, 0xe1
142319fa7: movaps xmm3, xmm4
142319faa: addps xmm3, xmm8
142319fae: subps xmm4, xmm8
142319fb2: movaps xmmword ptr [rbp + 0x190], xmm14
142319fba: movaps xmmword ptr [rbp - 0x30], xmm7
142319fbe: movaps xmm7, xmm12
142319fc2: mulps xmm7, xmm15
142319fc6: movshdup xmm8, xmm7
142319fcb: movaps xmm14, xmm7
142319fcf: addss xmm14, xmm8
142319fd4: movaps xmm1, xmm10
142319fd8: subss xmm10, xmm14
142319fdd: movshdup xmm14, xmm15
142319fe2: movaps xmm15, xmm6
142319fe6: mulss xmm14, xmm12
142319feb: mulss xmm5, xmm9
142319ff0: mulss xmm9, xmm13
142319ff5: movaps xmm13, xmmword ptr [rbp - 0x20]
142319ffa: xorps xmm6, xmm6
142319ffd: addss xmm7, xmm9
14231a002: addss xmm9, xmm8
14231a007: movaps xmm12, xmm1
14231a00b: subss xmm12, xmm9
14231a010: movaps xmm8, xmm14
14231a014: subss xmm8, xmm5
14231a019: addss xmm5, xmm14
14231a01e: unpcklps xmm12, xmm5
14231a022: movlhps xmm12, xmm4
14231a026: shufps xmm4, xmm3, 1
14231a02a: shufps xmm10, xmm3, 0x30
14231a02f: shufps xmm4, xmm10, 0x82
14231a034: movaps xmm10, xmm1
14231a038: movaps xmm5, xmm2
14231a03b: shufps xmm5, xmm2, 0x55
14231a03f: mulps xmm4, xmm5
14231a042: movaps xmm5, xmm1
14231a045: subss xmm5, xmm7
14231a049: movaps xmm7, xmmword ptr [rbp - 0x30]
14231a04d: movaps xmm14, xmmword ptr [rbp + 0x190]
14231a055: unpcklps xmm8, xmm5
14231a059: shufps xmm8, xmm3, 0xd4
14231a05e: shufps xmm2, xmm2, 0
14231a062: mulps xmm8, xmm2
14231a066: shufps xmm11, xmm11, 0
14231a06b: mulps xmm12, xmm11
14231a06f: mulps xmm12, xmmword ptr [rip + 0x60b929]
14231a077: mulps xmm8, xmmword ptr [rip + 0xd30db1]
14231a07f: addps xmm8, xmm12
14231a083: mulps xmm4, xmmword ptr [rip + 0xd30db6]
14231a08a: addps xmm4, xmm8
14231a08e: movaps xmm8, xmmword ptr [rbp + 0x160]
14231a096: addps xmm0, xmm4
14231a099: mov rcx, qword ptr [rcx]
14231a09c: shl rax, 3
14231a0a0: lea rax, [rax + rax*2]
14231a0a4: xor edx, edx
14231a0a6: nop word ptr cs:[rax + rax]
14231a0b0: movlps qword ptr [rcx + rdx + 8], xmm0
14231a0b5: movaps xmm1, xmm0
14231a0b8: unpckhpd xmm1, xmm0
14231a0bc: movss dword ptr [rcx + rdx + 0x10], xmm1
14231a0c2: add rdx, 0x18
14231a0c6: cmp rax, rdx
14231a0c9: jne 0x14231a0b0
14231a0cb: jmp 0x1423194ff
14231a0d0: lea rax, [rsi + 0x10]
14231a0d4: mov ecx, dword ptr [rax + 8]
14231a0d7: mov dword ptr [rbp + 0x1b8], ecx
14231a0dd: mov rax, qword ptr [rax]
14231a0e0: mov qword ptr [rbp + 0x1b0], rax
14231a0e7: movss xmm0, dword ptr [rbp + 0x208]
14231a0ef: movaps xmm1, xmm0
14231a0f2: mulss xmm1, xmm0
14231a0f6: movsd xmm2, qword ptr [rbp + 0x200]
14231a0fe: movaps xmm3, xmm2
14231a101: mulps xmm3, xmm2
14231a104: movshdup xmm4, xmm3
14231a108: addss xmm4, xmm3
14231a10c: addss xmm4, xmm1
14231a110: xorps xmm1, xmm1
14231a113: sqrtss xmm1, xmm4
14231a117: movaps xmm3, xmm10
14231a11b: divss xmm3, xmm1
14231a11f: movsldup xmm1, xmm3
14231a123: mulps xmm1, xmm2
14231a126: mulss xmm3, xmm0
14231a12a: movlps qword ptr [rbp + 0x1bc], xmm1
14231a131: movss dword ptr [rbp + 0x1c4], xmm3
14231a139: movss xmm3, dword ptr [rip + 0x6fcb77]
14231a141: test byte ptr [rbp + 0x23c], 1
14231a148: jne 0x14231a152
14231a14a: movss xmm3, dword ptr [rip + 0x60b71e]
14231a152: lea rcx, [rbp - 0x10]
14231a156: mov rdx, qword ptr [rbp + 0x158]
14231a15d: lea r8, [rbp + 0x1b0]
14231a164: call 0x1408ccc90 ; country_core::systems::collision::world::RaycastWorld::legacy_spherecast
14231a169: cmp dword ptr [rbp + 0x20], 2
14231a16d: je 0x14231a1bd
14231a16f: movss xmm0, dword ptr [rbp + 0xc]
14231a174: subss xmm0, dword ptr [rsi + 0x14]
14231a179: movups xmm1, xmmword ptr [rbp + 8]
14231a17d: mulss xmm0, xmm0
14231a181: movss xmm2, dword ptr [rbp + 0x10]
14231a186: movlhps xmm2, xmm1
14231a189: shufps xmm2, xmm1, 0xe2
14231a18d: movss xmm1, dword ptr [rsi + 0x10]
14231a192: movss xmm3, dword ptr [rsi + 0x18]
14231a197: unpcklps xmm1, xmm3
14231a19a: subps xmm2, xmm1
14231a19d: mulps xmm2, xmm2
14231a1a0: addss xmm0, xmm2
14231a1a4: movshdup xmm1, xmm2
14231a1a8: addss xmm1, xmm0
14231a1ac: xorps xmm0, xmm0
14231a1af: sqrtss xmm0, xmm1
14231a1b3: ucomiss xmm10, xmm0
14231a1b7: ja 0x1423194ff
14231a1bd: test byte ptr [rbp + 0x23c], 1
14231a1c4: je 0x142319dbf
14231a1ca: cmp dword ptr [r13], 4
14231a1cf: je 0x142319dbf
14231a1d5: movss xmm1, dword ptr [rsi + 0x10]
14231a1da: movss xmm2, dword ptr [rsi + 0x18]
14231a1df: mov dword ptr [rsp + 0x20], 0x43020000
14231a1e7: lea rcx, [rbp + 0x118]
14231a1ee: movaps xmm3, xmm15
14231a1f2: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
14231a1f7: movaps xmm9, xmm0
14231a1fb: movss xmm1, dword ptr [rsi + 0x10]
14231a200: movss xmm2, dword ptr [rsi + 0x18]
14231a205: addss xmm1, dword ptr [rbp + 0x200]
14231a20d: addss xmm2, dword ptr [rbp + 0x208]
14231a215: mov dword ptr [rsp + 0x20], 0x43020000
14231a21d: lea rcx, [rbp + 0x118]
14231a224: movaps xmm3, xmm15
14231a228: call 0x140a14df0 ; utils::float_grid::impl$0::bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512> >
14231a22d: ucomiss xmm9, xmm6
14231a231: ja 0x142319dbf
14231a237: ucomiss xmm0, xmm6
14231a23a: ja 0x1423194ff
14231a240: jmp 0x142319dbf
14231a245: cmp qword ptr [rbp + 0x170], 0
14231a24d: je 0x14231a29a
14231a24f: call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14231a254: mov rcx, rax
14231a257: mov rax, qword ptr [rax]
14231a25a: cmp rax, 1
14231a25e: jne 0x14231a2fc
14231a264: add rcx, 8
14231a268: cmp qword ptr [rcx], 0
14231a26c: jne 0x14231a33a
14231a272: mov qword ptr [rcx], 0xffffffffffffffff
14231a279: mov qword ptr [rbp + 0x210], rcx
14231a280: add rcx, 8
14231a284: mov rdx, qword ptr [rbp + 0x178]
14231a28b: call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
14231a290: mov rax, qword ptr [rbp + 0x210]
14231a297: inc qword ptr [rax]
14231a29a: movaps xmm6, xmmword ptr [rbp + 0x250]
14231a2a1: movaps xmm7, xmmword ptr [rbp + 0x260]
14231a2a8: movaps xmm8, xmmword ptr [rbp + 0x270]
14231a2b0: movaps xmm9, xmmword ptr [rbp + 0x280]
14231a2b8: movaps xmm10, xmmword ptr [rbp + 0x290]
14231a2c0: movaps xmm11, xmmword ptr [rbp + 0x2a0]
14231a2c8: movaps xmm12, xmmword ptr [rbp + 0x2b0]
14231a2d0: movaps xmm13, xmmword ptr [rbp + 0x2c0]
14231a2d8: movaps xmm14, xmmword ptr [rbp + 0x2d0]
14231a2e0: movaps xmm15, xmmword ptr [rbp + 0x2e0]
14231a2e8: add rsp, 0x378
14231a2ef: pop rbx
14231a2f0: pop rdi
14231a2f1: pop rsi
14231a2f2: pop r12
14231a2f4: pop r13
14231a2f6: pop r14
14231a2f8: pop r15
14231a2fa: pop rbp
14231a2fb: ret 
14231a2fc: test rax, rax
14231a2ff: jne 0x14231a32e
14231a301: xor edx, edx
14231a303: call 0x14291b030 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14231a308: mov rcx, rax
14231a30b: test rax, rax
14231a30e: jne 0x14231a268
14231a314: jmp 0x14231a32e
14231a316: test rax, rax
14231a319: jne 0x14231a32e
14231a31b: xor edx, edx
14231a31d: call 0x14291b030 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14231a322: mov rcx, rax
14231a325: test rax, rax
14231a328: jne 0x14231920a
14231a32e: lea rcx, [rip + 0xd2fd9b]
14231a335: call 0x1428d80d0 ; std::thread::local::panic_access_error
14231a33a: lea rcx, [rip + 0xd2fdff]
14231a341: call 0x1428d9250 ; core::cell::panic_already_borrowed
14231a346: mov qword ptr [rbp - 0x10], 0
14231a34e: lea rax, [rip + 0xd2ff6b]
14231a355: mov qword ptr [rsp + 0x20], rax
14231a35a: lea rdx, [rbp + 0x1b0]
14231a361: lea r9, [rbp - 0x10]
14231a365: xor ecx, ecx
14231a367: mov r8, r14
14231a36a: call 0x1428cab41 ; core::panicking::assert_failed<ref$<str$>,ref$<str$> >
14231a36f: jmp 0x14231a38b
14231a371: lea rcx, [rip + 0xd30d68]
14231a378: call 0x1428d9310 ; core::option::unwrap_failed
14231a37d: jmp 0x14231a38b
14231a37f: lea rcx, [rip + 0xd30d2a]
14231a386: call 0x1428d9310 ; core::option::unwrap_failed
14231a38b: ud2 
14231a38d: nop dword ptr [rax]
14231a390: mov qword ptr [rsp + 0x10], rdx
14231a395: push rbp
14231a396: push r15
14231a398: push r14
14231a39a: push r13
14231a39c: push r12
14231a39e: push rsi
14231a39f: push rdi
14231a3a0: push rbx
14231a3a1: sub rsp, 0xd8
14231a3a8: lea rbp, [rdx + 0x80]
14231a3af: movaps xmmword ptr [rsp + 0x30], xmm15
14231a3b5: movaps xmmword ptr [rsp + 0x40], xmm14
14231a3bb: movaps xmmword ptr [rsp + 0x50], xmm13
14231a3c1: movaps xmmword ptr [rsp + 0x60], xmm12
14231a3c7: movaps xmmword ptr [rsp + 0x70], xmm11
14231a3cd: movaps xmmword ptr [rsp + 0x80], xmm10
14231a3d6: movaps xmmword ptr [rsp + 0x90], xmm9
14231a3df: movaps xmmword ptr [rsp + 0xa0], xmm8
14231a3e8: movaps xmmword ptr [rsp + 0xb0], xmm7
14231a3f0: movaps xmmword ptr [rsp + 0xc0], xmm6
14231a3f8: mov rax, qword ptr [rbp + 0x218]
14231a3ff: inc qword ptr [rax]
14231a402: movaps xmm6, xmmword ptr [rsp + 0xc0]
14231a40a: movaps xmm7, xmmword ptr [rsp + 0xb0]
14231a412: movaps xmm8, xmmword ptr [rsp + 0xa0]
14231a41b: movaps xmm9, xmmword ptr [rsp + 0x90]
14231a424: movaps xmm10, xmmword ptr [rsp + 0x80]
14231a42d: movaps xmm11, xmmword ptr [rsp + 0x70]
14231a433: movaps xmm12, xmmword ptr [rsp + 0x60]
14231a439: movaps xmm13, xmmword ptr [rsp + 0x50]
14231a43f: movaps xmm14, xmmword ptr [rsp + 0x40]
14231a445: movaps xmm15, xmmword ptr [rsp + 0x30]
14231a44b: add rsp, 0xd8
14231a452: pop rbx
14231a453: pop rdi
14231a454: pop rsi
14231a455: pop r12
14231a457: pop r13
14231a459: pop r14
14231a45b: pop r15
14231a45d: pop rbp
14231a45e: ret 
14231a45f: nop 
14231a460: mov qword ptr [rsp + 0x10], rdx
14231a465: push rbp
14231a466: push r15
14231a468: push r14
14231a46a: push r13
14231a46c: push r12
14231a46e: push rsi
14231a46f: push rdi
14231a470: push rbx
14231a471: sub rsp, 0xd8
14231a478: lea rbp, [rdx + 0x80]
14231a47f: movaps xmmword ptr [rsp + 0x30], xmm15
14231a485: movaps xmmword ptr [rsp + 0x40], xmm14
14231a48b: movaps xmmword ptr [rsp + 0x50], xmm13
14231a491: movaps xmmword ptr [rsp + 0x60], xmm12
14231a497: movaps xmmword ptr [rsp + 0x70], xmm11
14231a49d: movaps xmmword ptr [rsp + 0x80], xmm10
14231a4a6: movaps xmmword ptr [rsp + 0x90], xmm9
14231a4af: movaps xmmword ptr [rsp + 0xa0], xmm8
14231a4b8: movaps xmmword ptr [rsp + 0xb0], xmm7
14231a4c0: movaps xmmword ptr [rsp + 0xc0], xmm6
14231a4c8: mov rax, qword ptr [rbp + 0x210]
14231a4cf: inc qword ptr [rax]
14231a4d2: movaps xmm6, xmmword ptr [rsp + 0xc0]
14231a4da: movaps xmm7, xmmword ptr [rsp + 0xb0]
14231a4e2: movaps xmm8, xmmword ptr [rsp + 0xa0]
14231a4eb: movaps xmm9, xmmword ptr [rsp + 0x90]
14231a4f4: movaps xmm10, xmmword ptr [rsp + 0x80]
14231a4fd: movaps xmm11, xmmword ptr [rsp + 0x70]
14231a503: movaps xmm12, xmmword ptr [rsp + 0x60]
14231a509: movaps xmm13, xmmword ptr [rsp + 0x50]
14231a50f: movaps xmm14, xmmword ptr [rsp + 0x40]
14231a515: movaps xmm15, xmmword ptr [rsp + 0x30]
14231a51b: add rsp, 0xd8
14231a522: pop rbx
14231a523: pop rdi
14231a524: pop rsi
14231a525: pop r12
14231a527: pop r13
14231a529: pop r14
14231a52b: pop r15
14231a52d: pop rbp
14231a52e: ret 
14231a52f: nop 
14231a530: mov qword ptr [rsp + 0x10], rdx
14231a535: push rbp
14231a536: push r15
14231a538: push r14
14231a53a: push r13
14231a53c: push r12
14231a53e: push rsi
14231a53f: push rdi
14231a540: push rbx
14231a541: sub rsp, 0xd8
14231a548: lea rbp, [rdx + 0x80]
14231a54f: movaps xmmword ptr [rsp + 0x30], xmm15
14231a555: movaps xmmword ptr [rsp + 0x40], xmm14
14231a55b: movaps xmmword ptr [rsp + 0x50], xmm13
14231a561: movaps xmmword ptr [rsp + 0x60], xmm12
14231a567: movaps xmmword ptr [rsp + 0x70], xmm11
14231a56d: movaps xmmword ptr [rsp + 0x80], xmm10
14231a576: movaps xmmword ptr [rsp + 0x90], xmm9
14231a57f: movaps xmmword ptr [rsp + 0xa0], xmm8
14231a588: movaps xmmword ptr [rsp + 0xb0], xmm7
14231a590: movaps xmmword ptr [rsp + 0xc0], xmm6
14231a598: cmp qword ptr [rbp + 0x170], 0
14231a5a0: je 0x14231a5d1
14231a5a2: lea rax, [rbp + 0x178]
14231a5a9: mov qword ptr [rbp + 0x100], rax
14231a5b0: lea rax, [rbp + 0x100]
14231a5b7: mov qword ptr [rbp + 0xf8], rax
14231a5be: lea rcx, [rip + 0xd3099b]
14231a5c5: lea rdx, [rbp + 0xf8]
14231a5cc: call 0x14230e5f0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
14231a5d1: movaps xmm6, xmmword ptr [rsp + 0xc0]
14231a5d9: movaps xmm7, xmmword ptr [rsp + 0xb0]
14231a5e1: movaps xmm8, xmmword ptr [rsp + 0xa0]
14231a5ea: movaps xmm9, xmmword ptr [rsp + 0x90]
14231a5f3: movaps xmm10, xmmword ptr [rsp + 0x80]
14231a5fc: movaps xmm11, xmmword ptr [rsp + 0x70]
14231a602: movaps xmm12, xmmword ptr [rsp + 0x60]
14231a608: movaps xmm13, xmmword ptr [rsp + 0x50]
14231a60e: movaps xmm14, xmmword ptr [rsp + 0x40]
14231a614: movaps xmm15, xmmword ptr [rsp + 0x30]
14231a61a: add rsp, 0xd8
14231a621: pop rbx
14231a622: pop rdi
14231a623: pop rsi
14231a624: pop r12
14231a626: pop r13
14231a628: pop r14
14231a62a: pop r15
14231a62c: pop rbp
14231a62d: ret 
