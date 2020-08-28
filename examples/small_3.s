.global "main"
"main":
  ".Lmain_entry":
    pushq %rbp
    movq %rsp, %rbp
    subq $16, %rsp
    leaq -8(%rbp), %r10
    leaq -16(%rbp), %r11
    movq $3, (%r11)
    movq $3, (%r10)
    movq -8(%rbp), %r12
    imulq -16(%rbp), %r12
    movq %r12, %rdi
    call "x64::exit_with"
    movq %rax, %r13
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