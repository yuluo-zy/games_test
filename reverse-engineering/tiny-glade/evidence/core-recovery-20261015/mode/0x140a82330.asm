0x140a82330 sub      rsp, 0x38
0x140a82334 mov      rax, rdx
0x140a82337 cmp      dword ptr [rcx], 0xb
0x140a8233a jne      0x140a82355
0x140a8233c lea      rdx, [rip + 0x2055395] ; @0x142ad76d8 'OutOfBounds'
0x140a82343 mov      r8d, 0xb
0x140a82349 mov      rcx, rax
0x140a8234c add      rsp, 0x38
0x140a82350 jmp      0x140d86b90 ; _$LT$core..fmt..Formatter$u20$as$u20$core..fmt..Write$GT$::write_str::h5c3f9ed06cfb23bd / core::fmt::Formatter::write_str::h47944a04c0e87e3b
0x140a82355 mov      qword ptr [rsp + 0x30], rcx
0x140a8235a lea      rcx, [rip + 0x2055387]
0x140a82361 mov      qword ptr [rsp + 0x20], rcx
0x140a82366 lea      rdx, [rip + 0x205539b] ; @0x142ad7708 'BackendErrorstruct PlacementConfigEntry'
0x140a8236d lea      r9, [rsp + 0x30]
0x140a82372 mov      r8d, 0xc
0x140a82378 mov      rcx, rax
0x140a8237b call     0x140d872d0 ; core::fmt::Formatter::debug_tuple_field1_finish::ha115499c8c7eba6f
0x140a82380 nop      
0x140a82381 add      rsp, 0x38
0x140a82385 ret      