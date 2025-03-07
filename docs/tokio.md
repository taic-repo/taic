# Tokio 修改方案

## 调度

原始的数据结构实现的对比，这里需要直接在 tokio 的源码中进行修改，例如 shared、core 这些结构，直接操作这些数据结构，来对比使用不同的实现的差距，主要体现出同步互斥的开销。避免掉 Tokio 运行时的其他开销。

涉及两种运行时：

- current_thread：在这个运行时中，存在局部队列和全局队列。单只有一个线程来运行协程，全局队列是用于 Timer、IO 事件进行交互。这里还存在着与 BlockingPool 之间的交互。这个运行时可以用来测试将 IO 事件与任务调度结合能够带来的好处。
- multi_thread：在这个运行时中，存在 N 个 worker，每个 worker 拥有一个局部队列和一个全局队列，这里可以测试与调度（任务窃取相关）的开销。

## 事件循环

tokio 定义了外部事件的循环，称之为 driver，它与外部事件的交互通过主动调用 driver.park 或 driver.park_timeout，否则无法进行交互。

### current_thread

全局队列主要是用来与 IO 进行交互，因此可以使用第一个局部队列，硬件的唤醒操作也是将任务放在第一个局部队列中。修改后，软件使用第二个局部队列，不操作第一个局部队列。

### multi_thread

这里的修改会涉及到任务控制块中的结构的变化，因为在任务控制块的 Header 中直接定义了 queue_next 来找到下一个任务，这里可以不修改数据结构的定义，直接修改 get_queue_next 方法。软件使用的局部队列从第二个开始，第一个用于 IO 事件，而 steal 操作实际上是针对局部队列进行，每次窃取一半的任务放到自己的队列中，而硬件直接实现了这个操作，区别在于硬件实现没有随机性，且硬件每次窃取是取出一个任务。硬件实现可以省略掉很多同步互斥的开销。

