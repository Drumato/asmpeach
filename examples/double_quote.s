    .globl    "main"
    .type    "main", @function
"main":
    pushq    %rbp
    movq    %rsp, %rbp
    movq    $42, %rax
    popq    %rbp
    ret
