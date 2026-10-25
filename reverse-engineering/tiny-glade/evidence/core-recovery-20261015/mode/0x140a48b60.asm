0x140a48b60 sub      rsp, 0x38
0x140a48b64 mov      rax, rdx
0x140a48b67 mov      qword ptr [rsp + 0x30], rcx
0x140a48b6c lea      rcx, [rip + 0x2085d1d]
0x140a48b73 mov      qword ptr [rsp + 0x20], rcx
0x140a48b78 lea      rdx, [rip + 0x20862b1] ; @0x142acee30 'DecoratorId'
0x140a48b7f lea      r9, [rsp + 0x30]
0x140a48b84 mov      r8d, 0xb
0x140a48b8a mov      rcx, rax
0x140a48b8d call     0x140d872d0 ; core::fmt::Formatter::debug_tuple_field1_finish::ha115499c8c7eba6f
0x140a48b92 nop      
0x140a48b93 add      rsp, 0x38
0x140a48b97 ret      