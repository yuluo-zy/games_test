0x140afaed0 push     r15
0x140afaed2 push     r14
0x140afaed4 push     r12
0x140afaed6 push     rsi
0x140afaed7 push     rdi
0x140afaed8 push     rbx
0x140afaed9 sub      rsp, 0x38
0x140afaedd mov      rsi, rcx
0x140afaee0 mov      rdi, qword ptr [rdx]
0x140afaee3 inc      qword ptr [rdx + 0x18]
0x140afaee7 mov      byte ptr [rdx + 0x20], 0
0x140afaeeb mov      rax, qword ptr [rdi + 0x10]
0x140afaeef cmp      qword ptr [rdi], rax
0x140afaef2 je       0x140afb039
0x140afaef8 mov      rcx, qword ptr [rdi + 8]
0x140afaefc mov      byte ptr [rcx + rax], 0x7b
0x140afaf00 inc      rax
0x140afaf03 mov      qword ptr [rdi + 0x10], rax
0x140afaf07 mov      qword ptr [rsp + 0x28], rdx
0x140afaf0c mov      byte ptr [rsp + 0x30], 1
0x140afaf11 lea      rdx, [rip + 0x1fea858] ; @0x142ae5770 'modeslast_selectedstruct ToolbarCategoryModesBuildingPathDecoratorsCustomizationEraserTreeTerrainEdi'
0x140afaf18 lea      rcx, [rsp + 0x28]
0x140afaf1d mov      r8d, 5
0x140afaf23 mov      r9, rsi
0x140afaf26 call     0x141589e70 ; serde::ser::SerializeMap::serialize_entry::hb09a5c142bff288a
0x140afaf2b test     rax, rax
0x140afaf2e jne      0x140afb02b
0x140afaf34 add      rsi, 0x18
0x140afaf38 lea      rdx, [rip + 0x1fea836] ; @0x142ae5775 'last_selectedstruct ToolbarCategoryModesBuildingPathDecoratorsCustomizationEraserTreeTerrainEditingW'
0x140afaf3f lea      rcx, [rsp + 0x28]
0x140afaf44 mov      r8d, 0xd
0x140afaf4a mov      r9, rsi
0x140afaf4d call     0x1415875f0 ; serde::ser::SerializeMap::serialize_entry::h7254fecd735bf589 / serde::ser::SerializeMap::serialize_entry::h8ad77273df22e491
0x140afaf52 test     rax, rax
0x140afaf55 jne      0x140afb02b
0x140afaf5b cmp      byte ptr [rsp + 0x30], 0
0x140afaf60 je       0x140afb029
0x140afaf66 mov      r15, qword ptr [rsp + 0x28]
0x140afaf6b mov      r12, qword ptr [r15 + 0x18]
0x140afaf6f dec      r12
0x140afaf72 mov      qword ptr [r15 + 0x18], r12
0x140afaf76 cmp      byte ptr [r15 + 0x20], 0
0x140afaf7b je       0x140afb00e
0x140afaf81 mov      rsi, qword ptr [r15]
0x140afaf84 mov      rdx, qword ptr [rsi + 0x10]
0x140afaf88 cmp      qword ptr [rsi], rdx
0x140afaf8b je       0x140afb08b
0x140afaf91 mov      rax, qword ptr [rsi + 8]
0x140afaf95 mov      byte ptr [rax + rdx], 0xa
0x140afaf99 inc      rdx
0x140afaf9c mov      qword ptr [rsi + 0x10], rdx
0x140afafa0 test     r12, r12
0x140afafa3 je       0x140afb00e
0x140afafa5 mov      rsi, qword ptr [r15 + 8]
0x140afafa9 mov      rdi, qword ptr [r15 + 0x10]
0x140afafad mov      rbx, qword ptr [r15]
0x140afafb0 mov      r14, qword ptr [rbx + 0x10]
0x140afafb4 nop      word ptr cs:[rax + rax]
0x140afafc0 mov      rax, qword ptr [rbx]
0x140afafc3 sub      rax, r14
0x140afafc6 cmp      rax, rdi
0x140afafc9 jb       0x140afafeb
0x140afafcb mov      rcx, qword ptr [rbx + 8]
0x140afafcf add      rcx, r14
0x140afafd2 mov      rdx, rsi
0x140afafd5 mov      r8, rdi
0x140afafd8 call     0x142923c00 ; memcpy
0x140afafdd add      r14, rdi
0x140afafe0 mov      qword ptr [rbx + 0x10], r14
0x140afafe4 dec      r12
0x140afafe7 jne      0x140afafc0
0x140afafe9 jmp      0x140afb00e ; 
0x140afafeb mov      qword ptr [rsp + 0x20], 1
0x140afaff4 mov      r9d, 1
0x140afaffa mov      rcx, rbx
0x140afaffd mov      rdx, r14
0x140afb000 mov      r8, rdi
0x140afb003 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afb008 mov      r14, qword ptr [rbx + 0x10]
0x140afb00c jmp      0x140afafcb ; 
0x140afb00e mov      rsi, qword ptr [r15]
0x140afb011 mov      rdx, qword ptr [rsi + 0x10]
0x140afb015 cmp      qword ptr [rsi], rdx
0x140afb018 je       0x140afb068
0x140afb01a mov      rax, qword ptr [rsi + 8]
0x140afb01e mov      byte ptr [rax + rdx], 0x7d
0x140afb022 inc      rdx
0x140afb025 mov      qword ptr [rsi + 0x10], rdx
0x140afb029 xor      eax, eax
0x140afb02b add      rsp, 0x38
0x140afb02f pop      rbx
0x140afb030 pop      rdi
0x140afb031 pop      rsi
0x140afb032 pop      r12
0x140afb034 pop      r14
0x140afb036 pop      r15
0x140afb038 ret      
0x140afb039 mov      qword ptr [rsp + 0x20], 1
0x140afb042 mov      r8d, 1
0x140afb048 mov      r9d, 1
0x140afb04e mov      rcx, rdi
0x140afb051 mov      rbx, rdx
0x140afb054 mov      rdx, rax
0x140afb057 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afb05c mov      rdx, rbx
0x140afb05f mov      rax, qword ptr [rdi + 0x10]
0x140afb063 jmp      0x140afaef8 ; 
0x140afb068 mov      qword ptr [rsp + 0x20], 1
0x140afb071 mov      r8d, 1
0x140afb077 mov      r9d, 1
0x140afb07d mov      rcx, rsi
0x140afb080 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afb085 mov      rdx, qword ptr [rsi + 0x10]
0x140afb089 jmp      0x140afb01a ; 
0x140afb08b mov      qword ptr [rsp + 0x20], 1
0x140afb094 mov      r8d, 1
0x140afb09a mov      r9d, 1
0x140afb0a0 mov      rcx, rsi
0x140afb0a3 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afb0a8 mov      rdx, qword ptr [rsi + 0x10]
0x140afb0ac jmp      0x140afaf91 ; 