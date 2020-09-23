.global "main"
"main":
  ".Lmain_entry":
    pushq %rbp
    movq %rsp, %rbp
    subq $0, %rsp
    movq $1, %r11
    cmpq $0, %r11
    je .Lmain_FALSE_0
    jmp .Lmain_TRUE_0

  ".Lmain_TRUE_0":
    movq $15, %rdi
    call "x64::exit_with"
    movq %rax, %r12
    jmp .Lmain_NEXT_0

  ".Lmain_FALSE_0":
    movq $30, %rdi
    call "x64::exit_with"
    movq %rax, %r13
    jmp .Lmain_NEXT_0

  ".Lmain_NEXT_0":
    movq %rbp, %rsp
    popq %rbp
    ret

  .section .rodata
  .text
.global "x64::exit_with"
"x64::exit_with":
  ".Lx64::exit_with_entry":
    pushq %rbp
    movq %rsp, %rbp
    subq $8, %rsp
    movq %rdi, -8(%rbp)
    movq $60, %rax
    syscall
    movq %rbp, %rsp
    popq %rbp
    ret

  .section .rodata
  .text
