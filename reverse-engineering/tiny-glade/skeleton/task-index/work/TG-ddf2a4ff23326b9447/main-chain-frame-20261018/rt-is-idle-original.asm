00000001408bc530 push      rbp
00000001408bc531 push      r14
00000001408bc533 push      rsi
00000001408bc534 push      rdi
00000001408bc535 push      rbx
00000001408bc536 sub       rsp, 0x40
00000001408bc53a lea       rbp, [rsp + 0x40]
00000001408bc53f mov       qword ptr [rbp - 8], 0xfffffffffffffffe
00000001408bc547 mov       rdi, qword ptr [rcx]
00000001408bc54a lea       rsi, [rdi + 0x10]
00000001408bc54e mov       eax, dword ptr [rdi + 0x10]
00000001408bc551 cmp       eax, 0x3ffffffd
00000001408bc556 ja        0x1408bc5b2
00000001408bc558 lea       ecx, [rax + 1]
00000001408bc55b lock cmpxchg dword ptr [rsi], ecx
00000001408bc55f jne       0x1408bc5b2
00000001408bc561 movzx     eax, byte ptr [rdi + 0x18]
00000001408bc565 test      al, al
00000001408bc567 jne       0x1408bc5c2
00000001408bc569 movzx     r14d, byte ptr [rdi + 0xa0]
00000001408bc571 mov       rbx, qword ptr [rdi + 0x20]
00000001408bc575 movzx     eax, byte ptr [rdi + 0xa8]
00000001408bc57c mov       edx, 0xffffffff
00000001408bc581 lock xadd dword ptr [rdi + 0x10], edx
00000001408bc586 dec       edx
00000001408bc588 mov       ecx, edx
00000001408bc58a and       ecx, 0xbfffffff
00000001408bc590 neg       ecx
00000001408bc592 jo        0x1408bc5f8
00000001408bc594 cmp       r14b, 2
00000001408bc598 sete      cl
00000001408bc59b neg       rbx
00000001408bc59e seto      dl
00000001408bc5a1 and       dl, cl
00000001408bc5a3 xor       al, 1
00000001408bc5a5 and       al, dl
00000001408bc5a7 add       rsp, 0x40
00000001408bc5ab pop       rbx
00000001408bc5ac pop       rdi
00000001408bc5ad pop       rsi
00000001408bc5ae pop       r14
00000001408bc5b0 pop       rbp
00000001408bc5b1 ret       
00000001408bc5b2 mov       rcx, rsi
00000001408bc5b5 call      0x1428d8a30
00000001408bc5ba movzx     eax, byte ptr [rdi + 0x18]
00000001408bc5be test      al, al
00000001408bc5c0 je        0x1408bc569
00000001408bc5c2 add       rdi, 0x20
00000001408bc5c6 mov       qword ptr [rbp - 0x18], rdi
00000001408bc5ca mov       qword ptr [rbp - 0x10], rsi
00000001408bc5ce lea       rax, [rip + 0x21e43eb]
00000001408bc5d5 mov       qword ptr [rsp + 0x20], rax
00000001408bc5da lea       rcx, [rip + 0x21e3dd7]
00000001408bc5e1 lea       r9, [rip + 0x21e3e00]
00000001408bc5e8 lea       r8, [rbp - 0x18]
00000001408bc5ec mov       edx, 0x2b
00000001408bc5f1 call      0x1428d9760
00000001408bc5f6 ud2       
00000001408bc5f8 mov       rcx, rsi
00000001408bc5fb mov       esi, eax
00000001408bc5fd call      0x1428d8d40
00000001408bc602 mov       eax, esi
00000001408bc604 jmp       0x1408bc594
00000001408bc606 nop       word ptr cs:[rax + rax]
00000001408bc610 mov       qword ptr [rsp + 0x10], rdx
00000001408bc615 push      rbp
00000001408bc616 push      r14
00000001408bc618 push      rsi
00000001408bc619 push      rdi
00000001408bc61a push      rbx
00000001408bc61b sub       rsp, 0x30
00000001408bc61f lea       rbp, [rdx + 0x40]
00000001408bc623 mov       rcx, qword ptr [rbp - 0x10]
00000001408bc627 mov       edx, 0xffffffff
00000001408bc62c lock xadd dword ptr [rcx], edx
00000001408bc630 dec       edx
00000001408bc632 mov       eax, edx
00000001408bc634 and       eax, 0xbfffffff
00000001408bc639 neg       eax
00000001408bc63b jo        0x1408bc648
00000001408bc63d add       rsp, 0x30
00000001408bc641 pop       rbx
00000001408bc642 pop       rdi
00000001408bc643 pop       rsi
00000001408bc644 pop       r14
00000001408bc646 pop       rbp
00000001408bc647 ret       
00000001408bc648 call      0x1428d8d40
00000001408bc64d jmp       0x1408bc63d
