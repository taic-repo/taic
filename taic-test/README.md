# taic-test

借用了 Starry 已有的模块来构建 micro-benchmark，修改了对应的 axhal（增加了 context 中关于 User 模式下的一些控制寄存器信息，因为在进入到内核时，这些信息需要与对应的用户态执行流进行绑定，如果不绑定，当切换到用户态执行流时，这些现场都将会失效；增加了注册 Supervisor 软件中断的逻辑）、axtrap（增加了在 User 模式下陷入内核时保存 User 模式的控制寄存器操作），以及 platform 中 riscv64-qemu-virt 的配置。（为了在 FPGA 上能够运行，缩减了使用的堆的空间大小，以及 testcase 内存大小）

在 Supervisor 模式下的测试直接在 Starry Unikernel 模式下进行，主要测试如下：

1. [enqueue & dequeue](./apps/enq_deq_test/)：在 Supervisor 模式下使用控制器的调度功能，测试出/入队需要的时钟周期
2. [supervisor interrupt context overhead](./apps/sintcontext/)：测试在 Supervisor 模式下，从产生中断信号，到开始执行中断处理函数第一条指令之间需要的时钟周期（上下文切换 + 中断分发（解析 scause，找到对应的中断处理函数））
3. [supervisor external interrupt handler](./apps/sextint_latency/)：在 Supervisor 模式下，注册可抢占的外部中断处理函数，测试在产生外部中断信号后，到开始执行中断处理函数第一条指令之间需要的时钟周期（控制器延迟 + 上下文切换 + 中断分发：中断延迟）
4. [supervisor software interrupt handler](./apps/sextint_latency/)：在 Supervisor 模式下，注册发送发，注册接收方（可抢占），测试在发送方发起 IPC 后，接收方所在 CPU 收到控制器中断信号，到开始执行中断处理函数第一条指令之间需要的时钟周期（控制器延迟 + 上下文切换 + 中断分发：中断延迟）

在 User 模式下的测试基于 Starry 宏内核模式下进行，测试的代码在 utestcases 下，通过 apps/user_test 执行这些测试，在初始化时，需要先进入 utestcases 目录下执行 `make build` 生成测试文件，再使用 `./build_img.sh -a riscv64` 生成测试镜像，最后再执行 user_test 测试。（`make A=apps/$(TEST) ARCH=riscv64 SMP=1 LOG=debug run`）。（在进入用户态执行测试时，先在内核模式下，将控制器的地址映射添加到测试的进程的地址空间中，将所处的 CPU 的 hartid 通过环境变量传递到用户态）

1. [enqueue & dequeue](./utestcases/src/bin/user_enq_deq.rs)：在 User 模式下使用控制器的调度功能，测试出/入队需要的时钟周期
2. [user interrupt context overhead](./utestcases/src/bin/user_int_context.rs)：测试在 User 模式下，从产生中断信号，到开始执行中断处理函数第一条指令之间需要的时钟周期（上下文切换 + 中断分发（解析 ucause，找到对应的中断处理函数））
3. [user external interrupt handler](./apps/sextint_latency/)：在 User 模式下，注册可抢占的外部中断处理函数，测试在产生外部中断信号后，到开始执行中断处理函数第一条指令之间需要的时钟周期（控制器延迟 + 上下文切换 + 中断分发：中断延迟）
4. [user software interrupt handler](./apps/sextint_latency/)：在 User 模式下，注册发送发，注册接收方（可抢占），测试在发送方发起 IPC 后，接收方所在 CPU 收到控制器中断信号，到开始执行中断处理函数第一条指令之间需要的时钟周期（控制器延迟 + 上下文切换 + 中断分发：中断延迟）

以上所有测试结果的原数据在 `apps/$(TEST)` 目录下的 `.dat` 文件中，过滤掉异常数值后的数据在 assets 目录下，过滤采用了四分位数范围法：
    使用数据的第一四分位数（Q1）和第三四分位数（Q3）来定义异常值的范围。通常，被认为是异常值的数据点会小于 Q1 - 1.5 * IQR 或大于 Q3 + 1.5 * IQR。其中 IQR 为四分位距（IQR = Q3 - Q1）

综合结果如下：

S 态

| 测试                                                   | 均值（cpu cycle） | 标准差 |
| ------------------------------------------------------ | ----------------- | ------ |
| 入队                                                   | 2                 | 0      |
| 出队                                                   | 2                 | 0      |
| 中断上下文切换 + 中断分发                              | 94                | 0      |
| 注册外部中断处理函数                                   | 2                 | 0      |
| 外部中断处理延迟（控制器延迟 + 上下文切换 + 中断分发） | 264               | 0      |
| 注册软件中断/发送软件中断                              | 8                 | 0      |
| 软件中断处理延迟（控制器延迟 + 上下文切换 + 中断分发） | 98                | 0      |

U 态

| 测试                                                   | 均值（cpu cycle） | 标准差 |
| ------------------------------------------------------ | ----------------- | ------ |
| 入队                                                   | 3                 | 0      |
| 出队                                                   | 4                 | 0      |
| 中断上下文切换 + 中断分发                              | 66                | 0      |
| 注册外部中断处理函数                                   | 2                 | 0      |
| 外部中断处理延迟（控制器延迟 + 上下文切换 + 中断分发） | 216.4             | 1.5    |
| 注册软件中断/发送软件中断                              | 10                | 0      |
| 软件中断处理延迟（控制器延迟 + 上下文切换 + 中断分发） | 72                | 0      |

结论如下：
1. 关于收发双方通过控制器进行通信，在原本的上下文切换和中断分发开销的基础上，只增加了几个时钟周期的开销（98-94，72-66），复合预期
2. 控制器关于外部中断的处理，需要的延迟增加为原来的2.8（264/94），3.27（216/66）倍，推测的结果为控制器增加的额外功能（[具体描述](https://github.com/taic-repo/taic-rocket-chip/blob/f8c44c73c352232ea2245061c091d0488b5a0985/docs/hd_global_queue.md#%E5%A4%96%E9%83%A8%E4%B8%AD%E6%96%AD)），需要进一步验证