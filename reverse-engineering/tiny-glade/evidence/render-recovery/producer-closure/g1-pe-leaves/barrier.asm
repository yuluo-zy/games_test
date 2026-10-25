=== 0x140bd65e0 size=34  rhapsody::raw::barrier::ImageBarrier::new
    ; ImageBarrier::new: out(rcx)=this; writes u64 rdx@+0x00 (image/handle), u32 5th-arg@+0x08, qword 0xffffffffffffffff@+0x0c (prev-index sentinel -1 spanning 0x0c..0x13), u8 r8b@+0x14, u8 r9b@+0x15, u8 0@+0x16. Struct ~0x17B. Field semantics inferred; offsets/args exact.  [medium]
    0x140bd65e0    4889c8                   mov              rax, rcx
    0x140bd65e3    8b4c2428                 mov              ecx, dword ptr [rsp + 0x28]
    0x140bd65e7    488910                   mov              qword ptr [rax], rdx
    0x140bd65ea    44884014                 mov              byte ptr [rax + 0x14], r8b
    0x140bd65ee    44884815                 mov              byte ptr [rax + 0x15], r9b
    0x140bd65f2    894808                   mov              dword ptr [rax + 8], ecx
    0x140bd65f5    48c7400cffffffff         mov              qword ptr [rax + 0xc], 0xffffffffffffffff
    0x140bd65fd    c6401600                 mov              byte ptr [rax + 0x16], 0
    0x140bd6601    c3                       ret              

=== 0x140bd6610 size=23  rhapsody::raw::barrier::BufferBarrier::new
    ; BufferBarrier::new: out(rcx)=this; u64 rdx@+0x00, qword -1@+0x08 (prev-index sentinel), u8 r8b@+0x10, u8 r9b@+0x11. ~0x13B.  [high]
    0x140bd6610    4889c8                   mov              rax, rcx
    0x140bd6613    488911                   mov              qword ptr [rcx], rdx
    0x140bd6616    44884110                 mov              byte ptr [rcx + 0x10], r8b
    0x140bd661a    44884911                 mov              byte ptr [rcx + 0x11], r9b
    0x140bd661e    48c74108ffffffff         mov              qword ptr [rcx + 8], 0xffffffffffffffff
    0x140bd6626    c3                       ret              

=== 0x140bd6630 size=26  rhapsody::raw::barrier::image_aspect_mask_from_format
    ; image_aspect_mask_from_format(fmt): if (fmt-0x7c) <= 6 -> table@0x142b0ffb8[(fmt-0x7c)] else 1. Table bytes 02000000 02000000 02000000 04000000 06000000 06000000 06000000 => fmt 0x7c/0x7d/0x7e->2(DEPTH), 0x7f->4(STENCIL), 0x80/0x81/0x82->6(DEPTH|STENCIL); default 1(COLOR). Returns VkImageAspectFlags.  [high]
    0x140bd6630    83c184                   add              ecx, -0x7c
    0x140bd6633    b801000000               mov              eax, 1
    0x140bd6638    83f906                   cmp              ecx, 6
    0x140bd663b    770c                     ja               0x140bd6649
    0x140bd663d    89c8                     mov              eax, ecx
    0x140bd663f    488d0d7299f301           lea              rcx, [rip + 0x1f39972] -> 0x142b0ffb8
    0x140bd6646    8b0481                   mov              eax, dword ptr [rcx + rax*4]
    0x140bd6649    c3                       ret              

=== 0x140bd6650 size=68  rhapsody::raw::barrier::SynchronizationScope1::new::h597d5eae8e6a9318
    ; SynchronizationScope1::new: builds empty scope. Three Vec fields {cap@+0x00,ptr@+0x08,len@+0x10} at offsets 0x00,0x18,0x30 each set to empty (cap=0, ptr=dangling 8, len=0); accumulating mask u64@+0x48 := 0x200000000001 (bit0 and bit45). Direct byte fact; field naming (vec/mask) per SYNTHESIS.  [high]
    0x140bd6650    4889c8                   mov              rax, rcx
    0x140bd6653    48b90100000000200000     movabs           rcx, 0x200000000001
    0x140bd665d    48894848                 mov              qword ptr [rax + 0x48], rcx
    0x140bd6661    48c70000000000           mov              qword ptr [rax], 0
    0x140bd6668    48c7400808000000         mov              qword ptr [rax + 8], 8
    0x140bd6670    0f57c0                   xorps            xmm0, xmm0
    0x140bd6673    0f114010                 movups           xmmword ptr [rax + 0x10], xmm0
    0x140bd6677    48c7402008000000         mov              qword ptr [rax + 0x20], 8
    0x140bd667f    0f114028                 movups           xmmword ptr [rax + 0x28], xmm0
    0x140bd6683    48c7403808000000         mov              qword ptr [rax + 0x38], 8
    0x140bd668b    48c7404000000000         mov              qword ptr [rax + 0x40], 0
    0x140bd6693    c3                       ret              

=== 0x140bd6950 size=39  rhapsody::raw::barrier::impl$5::reset
    ; impl$5::reset: mask@+0x48 := 0x200000000001; zeroes the three vec length fields (+0x10,+0x28,+0x40). Does not touch ptr/cap (no dealloc). Direct byte fact.  [high]
    0x140bd6950    48b80100000000200000     movabs           rax, 0x200000000001
    0x140bd695a    48894148                 mov              qword ptr [rcx + 0x48], rax
    0x140bd695e    48c7411000000000         mov              qword ptr [rcx + 0x10], 0
    0x140bd6966    48c7412800000000         mov              qword ptr [rcx + 0x28], 0
    0x140bd696e    48c7414000000000         mov              qword ptr [rcx + 0x40], 0
    0x140bd6976    c3                       ret              

=== 0x140bd6980 size=29  rhapsody::raw::barrier::impl$5::is_empty
    ; impl$5::is_empty: returns (len@+0x10==0) && (len@+0x28==0) && (len@+0x40==0). Confirms the three Vec len offsets.  [high]
    0x140bd6980    4883791000               cmp              qword ptr [rcx + 0x10], 0
    0x140bd6985    7403                     je               0x140bd698a
    0x140bd6987    31c0                     xor              eax, eax
    0x140bd6989    c3                       ret              
    0x140bd698a    4883792800               cmp              qword ptr [rcx + 0x28], 0
    0x140bd698f    7403                     je               0x140bd6994
    0x140bd6991    31c0                     xor              eax, eax
    0x140bd6993    c3                       ret              
    0x140bd6994    4883794000               cmp              qword ptr [rcx + 0x40], 0
    0x140bd6999    0f94c0                   sete             al
    0x140bd699c    c3                       ret              
