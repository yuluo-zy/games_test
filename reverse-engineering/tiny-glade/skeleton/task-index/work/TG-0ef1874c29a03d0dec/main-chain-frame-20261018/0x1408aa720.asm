0x1408aa720 push rsi
0x1408aa721 push rdi
0x1408aa722 push rbx
0x1408aa723 sub rsp, 0x40
0x1408aa727 mov rsi, qword ptr [rcx]
0x1408aa72a mov rcx, qword ptr [rsi]
0x1408aa72d add rcx, 0x10
0x1408aa731 mov rdi, qword ptr [r8]
0x1408aa734 mov rbx, qword ptr [rdx]
0x1408aa737 mov qword ptr [rsp + 0x30], rbx
0x1408aa73c mov qword ptr [rsp + 0x28], rdi
0x1408aa741 mov qword ptr [rsp + 0x20], 0x16
0x1408aa74a lea rdx, [rip + 0x21f1623] ; '/vermeer/raster_strokes.raster.hlsl/vermeer/combine.raster.hlsl/vermeer/generate'
0x1408aa751 lea r9, [rip + 0x21f1580] ; 'vermeer_raster_strokes\x00\x00���B\x01\x00\x00\x006\x00\x00\x00\x00\x00\x00\x00j\x01\x00\x00\x0e\x00\x00\x00Vermeer raster strokes\x00\x00���B\x01\x00\x00\x00'
0x1408aa758 mov r8d, 0x23
0x1408aa75e call 0x140b3fe40
0x1408aa763 mov rcx, qword ptr [rsi]
0x1408aa766 add rcx, 0x10
0x1408aa76a mov qword ptr [rsp + 0x30], rbx
0x1408aa76f mov qword ptr [rsp + 0x28], rdi
0x1408aa774 mov qword ptr [rsp + 0x20], 0xf
0x1408aa77d lea rdx, [rip + 0x21f1613] ; '/vermeer/combine.raster.hlsl/vermeer/generate_strokes.cs.hlsl:cs_main/vermeer/ge'
0x1408aa784 lea r9, [rip + 0x21f15ad] ; 'vermeer_combine\x00���B\x01\x00\x00\x006\x00\x00\x00\x00\x00\x00\x00�\x01\x00\x00\x0e\x00\x00\x00Vermeer debug target/vermeer/raster_stro'
0x1408aa78b mov r8d, 0x1c
0x1408aa791 call 0x140b3fe40
0x1408aa796 nop 
0x1408aa797 add rsp, 0x40
0x1408aa79b pop rbx
0x1408aa79c pop rdi
0x1408aa79d pop rsi
0x1408aa79e ret 