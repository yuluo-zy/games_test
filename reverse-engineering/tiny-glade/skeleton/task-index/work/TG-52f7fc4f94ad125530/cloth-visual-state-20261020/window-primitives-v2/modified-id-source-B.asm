1407ec47b: 4489f1 mov ecx, r14d
1407ec47e: f30fbcc9 tzcnt ecx, ecx
1407ec482: 4189c6 mov r14d, eax
1407ec485: c1e102 shl ecx, 2
1407ec488: 4c89e0 mov rax, r12
1407ec48b: 4829c8 sub rax, rcx
1407ec48e: 8b48fc mov ecx, dword ptr [rax - 4]
1407ec491: 898d10020000 mov dword ptr [rbp + 0x210], ecx
1407ec497: 488b4548 mov rax, qword ptr [rbp + 0x48]
1407ec49b: 4883784800 cmp qword ptr [rax + 0x48], 0
1407ec4a0: 0f8498260000 je 0x1407eeb3e
1407ec4a6: 48898d78020000 mov qword ptr [rbp + 0x278], rcx
1407ec4ad: 488b4d10 mov rcx, qword ptr [rbp + 0x10]
1407ec4b1: 488d9510020000 lea rdx, [rbp + 0x210]
1407ec4b8: e8134adcff call 0x1405b0ed0 ; core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<shader_pipeline::reflection::BindingIndex> > | core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<country_core::resources::walls::d
1407ec4bd: 48ff4d40 dec qword ptr [rbp + 0x40]
1407ec4c1: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
