1407ec5b9: 48898570020000 mov qword ptr [rbp + 0x270], rax
1407ec5c0: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
1407ec5c4: 488d95f0010000 lea rdx, [rbp + 0x1f0]
1407ec5cb: e8a047dcff call 0x1405b0d70 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::systems::ui::GlobalId> > 
1407ec5d0: 4c8b2b mov r13, qword ptr [rbx]
1407ec5d3: 488b4b08 mov rcx, qword ptr [rbx + 8]
1407ec5d7: 4889ca mov rdx, rcx
1407ec5da: 4821c2 and rdx, rax
1407ec5dd: 48c1e839 shr rax, 0x39
1407ec5e1: 660f6ec0 movd xmm0, eax
1407ec5e5: 660f60c0 punpcklbw xmm0, xmm0
1407ec5e9: f20f70c000 pshuflw xmm0, xmm0, 0
1407ec5ee: 660f70c000 pshufd xmm0, xmm0, 0
1407ec5f3: 498d8558fdffff lea rax, [r13 - 0x2a8]
1407ec5fa: 4c8b85f0010000 mov r8, qword ptr [rbp + 0x1f0]
1407ec601: 4531c9 xor r9d, r9d
