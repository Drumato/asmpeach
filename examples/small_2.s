.global "main"
"main":
  ".Lmain_entry":
    pushq %rbp
    movq %rsp, %rbp
    subq $8, %rsp
    leaq -8(%rbp), %r10
    movq $30, (%r10)
    movq -8(%rbp), %rdi
    call "x64::exit_with"
    movq %rax, %r11
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