0x140afb990 push     r15
0x140afb992 push     r14
0x140afb994 push     r12
0x140afb996 push     rsi
0x140afb997 push     rdi
0x140afb998 push     rbx
0x140afb999 sub      rsp, 0x38
0x140afb99d mov      rsi, rcx
0x140afb9a0 mov      rdi, qword ptr [rdx]
0x140afb9a3 inc      qword ptr [rdx + 0x18]
0x140afb9a7 mov      byte ptr [rdx + 0x20], 0
0x140afb9ab mov      rax, qword ptr [rdi + 0x10]
0x140afb9af cmp      qword ptr [rdi], rax
0x140afb9b2 je       0x140afbaf9
0x140afb9b8 mov      rcx, qword ptr [rdi + 8]
0x140afb9bc mov      byte ptr [rcx + rax], 0x7b
0x140afb9c0 inc      rax
0x140afb9c3 mov      qword ptr [rdi + 0x10], rax
0x140afb9c7 mov      qword ptr [rsp + 0x28], rdx
0x140afb9cc mov      byte ptr [rsp + 0x30], 1
0x140afb9d1 lea      rdx, [rip + 0x1fea022] ; @0x142ae59fa 'categoriesstruct ToolbarAllWallBulldoze'
0x140afb9d8 lea      rcx, [rsp + 0x28]
0x140afb9dd mov      r8d, 0xa
0x140afb9e3 mov      r9, rsi
0x140afb9e6 call     0x141587120 ; serde::ser::SerializeMap::serialize_entry::h70a18da9912259c4
0x140afb9eb test     rax, rax
0x140afb9ee jne      0x140afbaeb
0x140afb9f4 add      rsi, 0x18
0x140afb9f8 lea      rdx, [rip + 0x1fe9d76] ; @0x142ae5775 'last_selectedstruct ToolbarCategoryModesBuildingPathDecoratorsCustomizationEraserTreeTerrainEditingW'
0x140afb9ff lea      rcx, [rsp + 0x28]
0x140afba04 mov      r8d, 0xd
0x140afba0a mov      r9, rsi
0x140afba0d call     0x1415889b0 ; serde::ser::SerializeMap::serialize_entry::h94bac9c399381907
0x140afba12 test     rax, rax
0x140afba15 jne      0x140afbaeb
0x140afba1b cmp      byte ptr [rsp + 0x30], 0
0x140afba20 je       0x140afbae9
0x140afba26 mov      r15, qword ptr [rsp + 0x28]
0x140afba2b mov      r12, qword ptr [r15 + 0x18]
0x140afba2f dec      r12
0x140afba32 mov      qword ptr [r15 + 0x18], r12
0x140afba36 cmp      byte ptr [r15 + 0x20], 0
0x140afba3b je       0x140afbace
0x140afba41 mov      rsi, qword ptr [r15]
0x140afba44 mov      rdx, qword ptr [rsi + 0x10]
0x140afba48 cmp      qword ptr [rsi], rdx
0x140afba4b je       0x140afbb4b
0x140afba51 mov      rax, qword ptr [rsi + 8]
0x140afba55 mov      byte ptr [rax + rdx], 0xa
0x140afba59 inc      rdx
0x140afba5c mov      qword ptr [rsi + 0x10], rdx
0x140afba60 test     r12, r12
0x140afba63 je       0x140afbace
0x140afba65 mov      rsi, qword ptr [r15 + 8]
0x140afba69 mov      rdi, qword ptr [r15 + 0x10]
0x140afba6d mov      rbx, qword ptr [r15]
0x140afba70 mov      r14, qword ptr [rbx + 0x10]
0x140afba74 nop      word ptr cs:[rax + rax]
0x140afba80 mov      rax, qword ptr [rbx]
0x140afba83 sub      rax, r14
0x140afba86 cmp      rax, rdi
0x140afba89 jb       0x140afbaab
0x140afba8b mov      rcx, qword ptr [rbx + 8]
0x140afba8f add      rcx, r14
0x140afba92 mov      rdx, rsi
0x140afba95 mov      r8, rdi
0x140afba98 call     0x142923c00 ; memcpy
0x140afba9d add      r14, rdi
0x140afbaa0 mov      qword ptr [rbx + 0x10], r14
0x140afbaa4 dec      r12
0x140afbaa7 jne      0x140afba80
0x140afbaa9 jmp      0x140afbace ; 
0x140afbaab mov      qword ptr [rsp + 0x20], 1
0x140afbab4 mov      r9d, 1
0x140afbaba mov      rcx, rbx
0x140afbabd mov      rdx, r14
0x140afbac0 mov      r8, rdi
0x140afbac3 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afbac8 mov      r14, qword ptr [rbx + 0x10]
0x140afbacc jmp      0x140afba8b ; 
0x140afbace mov      rsi, qword ptr [r15]
0x140afbad1 mov      rdx, qword ptr [rsi + 0x10]
0x140afbad5 cmp      qword ptr [rsi], rdx
0x140afbad8 je       0x140afbb28
0x140afbada mov      rax, qword ptr [rsi + 8]
0x140afbade mov      byte ptr [rax + rdx], 0x7d
0x140afbae2 inc      rdx
0x140afbae5 mov      qword ptr [rsi + 0x10], rdx
0x140afbae9 xor      eax, eax
0x140afbaeb add      rsp, 0x38
0x140afbaef pop      rbx
0x140afbaf0 pop      rdi
0x140afbaf1 pop      rsi
0x140afbaf2 pop      r12
0x140afbaf4 pop      r14
0x140afbaf6 pop      r15
0x140afbaf8 ret      
0x140afbaf9 mov      qword ptr [rsp + 0x20], 1
0x140afbb02 mov      r8d, 1
0x140afbb08 mov      r9d, 1
0x140afbb0e mov      rcx, rdi
0x140afbb11 mov      rbx, rdx
0x140afbb14 mov      rdx, rax
0x140afbb17 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afbb1c mov      rdx, rbx
0x140afbb1f mov      rax, qword ptr [rdi + 0x10]
0x140afbb23 jmp      0x140afb9b8 ; 
0x140afbb28 mov      qword ptr [rsp + 0x20], 1
0x140afbb31 mov      r8d, 1
0x140afbb37 mov      r9d, 1
0x140afbb3d mov      rcx, rsi
0x140afbb40 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afbb45 mov      rdx, qword ptr [rsi + 0x10]
0x140afbb49 jmp      0x140afbada ; 
0x140afbb4b mov      qword ptr [rsp + 0x20], 1
0x140afbb54 mov      r8d, 1
0x140afbb5a mov      r9d, 1
0x140afbb60 mov      rcx, rsi
0x140afbb63 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afbb68 mov      rdx, qword ptr [rsi + 0x10]
0x140afbb6c jmp      0x140afba51 ; 