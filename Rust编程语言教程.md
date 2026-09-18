# Rust 编程语言教程：从入门到深入理解

> 📖 本教程参考 Rust 官方书籍 *The Rust Programming Language*（Rust Book）、*Rust by Example* 及官方文档编写，
> 专为**有其他编程语言基础**（C++ / Java / Python / Go 等）、初次接触 Rust 的学习者打造。
>
> **开发环境**：Windows 11 + RustRover（JetBrains）
>
> **学习建议**：Rust 的学习曲线前陡后缓。第 5、6、12 章（所有权、借用、生命周期）是全书的核心与难点，
> 请务必放慢速度、动手敲代码、认真阅读每一条编译器报错——**编译器是你最好的老师**。

---

## 全书总目录

### 第一篇：Rust 入门
- **第 1 章 认识 Rust** — 设计哲学、与其他语言的全方位对比、适用领域
- **第 2 章 搭建开发环境** — Windows 11 下 rustup 安装、RustRover 配置与调试
- **第 3 章 Hello World 与 Cargo** — 工程化构建、依赖管理、项目结构
- **第 4 章 基础语法** — 变量与可变性、数据类型、函数、控制流

### 第二篇：所有权——Rust 的灵魂（核心中的核心）
- **第 5 章 所有权（Ownership）** — 内存模型、move 语义、Copy 与 clone
- **第 6 章 引用与借用** — `&T` / `&mut T` 的规则、借用检查器、切片
- **第 7 章 结构体** — 定义、方法、关联函数、渐进式重构实战
- **第 8 章 枚举与模式匹配** — `Option<T>`、match 穷尽性、状态机实战

### 第三篇：进阶核心概念
- **第 9 章 包、Crate 与模块系统** — 多文件项目的组织方式
- **第 10 章 错误处理** — `panic!` 与 `Result<T, E>`、`?` 运算符、自定义错误
- **第 11 章 泛型与 Trait** — 零成本抽象、trait bound、静态/动态分发
- **第 12 章 生命周期（Lifetimes）** — 标注语法、省略规则、报错实战解析
- **第 13 章 常用集合** — `Vec`、`String`、`HashMap` 与 UTF-8 的坑
- **第 14 章 闭包与迭代器** — `Fn` 家族、惰性求值、零成本抽象

### 第四篇：高级特性与实战
- **第 15 章 智能指针** — `Box`、`Rc`、`RefCell`、`Weak` 与内部可变性
- **第 16 章 无畏并发** — 线程、channel、`Arc<Mutex<T>>`、`Send`/`Sync`
- **第 17 章 Unsafe Rust 与宏（简介）** — unsafe 超能力、`macro_rules!`
- **第 18 章 实战项目：命令行 TODO 应用** — 综合运用全书知识的完整项目
- **第 19 章 RustRover 高效使用与常见坑** — 调试技巧、高频编译错误速查表
- **第 20 章 下一步学习路线** — 官方资源、异步编程预告、社区

---

## 阅读约定

| 标记 | 含义 |
| --- | --- |
| `> 💡 提示` | 加深理解的补充说明 |
| `> ⚠️ 常见误区` | 新手最容易踩的坑，务必注意 |
| `> 🆚 语言对比` | 与 C++ / Java / Python / Go 的对比，帮你借助已有知识理解 |

所有代码示例均基于 **Rust 2021 edition** 编写，可直接在 RustRover 中运行验证。
文中展示的编译器报错（如 `error[E0382]`）为真实报错格式，建议跟着教程亲手"制造"这些错误再修复它们——这是掌握 Rust 最快的路径。

---

# 第一篇：Rust 入门

> 本篇面向已经有 C++ / Java / Python / Go 等至少一门语言基础、但从未接触过 Rust 的读者。
> 我们假设你使用 Windows 11 操作系统，并使用 JetBrains RustRover 作为主力 IDE。
> 本篇的目标是：读完之后，你不仅"会写"Rust 的 Hello World，更能理解 Rust 为什么
> 这样设计——这才是跨过 Rust 学习曲线的真正关键。

---

## 第 1 章 认识 Rust

### 1.1 Rust 是什么

Rust 是一门**系统级编程语言**，最初由 Mozilla 工程师 Graydon Hoare 于 2006 年作为个人项目启动，
2010 年由 Mozilla 官方赞助，2015 年发布 1.0 稳定版。如今 Rust 由独立的 Rust 基金会
（成员包括 AWS、Google、华为、Meta、微软等）治理。

如果你只记住一句话来概括 Rust，那就是：

> **Rust 试图在不牺牲性能的前提下，从编译期根除整类内存 bug。**

这句话拆开看有三层含义：

1. **不牺牲性能**：Rust 没有垃圾回收器（GC），没有运行时（runtime），生成的机器码
   与 C/C++ 处于同一性能量级。它可以用来写操作系统内核、浏览器引擎、数据库、游戏引擎。
2. **从编译期根除**：Rust 的内存安全保证是**编译器静态检查**出来的，而不是靠运行时
   的 GC 或者程序员的自律。编译通过了，就不存在空指针解引用、悬垂指针、double free、
   数据竞争（data race）这类问题。
3. **整类 bug**：Rust 消灭的是"一类"bug，而不是"某个"bug。这与靠 Code Review、
   靠 Sanitizer 工具、靠"写代码小心一点"的工程实践有本质区别——后者只能降低概率，
   前者是从语言层面让这类错误**无法表达**。

#### Rust 的江湖地位

- 在 Stack Overflow 开发者调查中，Rust **连续多年**被评为"最受喜爱（Most Admired）的编程语言"。
- Linux 内核从 6.1 版本开始正式接受 Rust 编写的驱动模块——这是内核历史上继 C 之后
  第二种被官方接受的语言。
- Windows 11 内核的部分组件（如 `win32k` 的 GDI 区域）已经用 Rust 重写。
- Android 13 中新增的原生代码约 21% 是 Rust；Google 报告显示，采用 Rust 后 Android 中
  内存安全漏洞的数量显著下降。
- AWS 的 Firecracker（支撑 Lambda 和 Fargate 的虚拟化技术）用 Rust 编写。
- Cloudflare 的核心代理（Pingora）、Discord 的读写状态服务、npm 的注册表后端、
  Deno 运行时、Tauri 桌面应用框架……都是 Rust。

> 💡 **提示**：学习 Rust 之前，先建立一个心理预期：Rust 的编译器非常"严格"，新手
> 常常戏称自己在"和借用检查器搏斗（fighting the borrow checker）"。但请把这种严格
> 理解为一位坐在你旁边的资深评审专家——他指出的每一个错误，在 C++ 里都可能是一个
> 凌晨三点把你叫醒的线上事故。

### 1.2 Rust 的三大设计哲学

#### 1.2.1 内存安全，但不使用垃圾回收

这是 Rust 最核心、也最革命性的设计。

我们先回顾一下各种语言是怎么管理内存的：

| 语言 | 内存管理方式 | 代价 |
|------|-------------|------|
| C / C++ | 程序员手动 `malloc/free`、`new/delete` | 忘了释放 → 内存泄漏；释放两次 → double free；释放后还用 → 悬垂指针（use-after-free） |
| Java / Go / Python | 垃圾回收器（GC）定期扫描并回收 | GC 停顿（stop-the-world 或并发 GC 的开销）、额外的内存占用、运行时性能不可预测 |
| Rust | **所有权（Ownership）+ 借用检查（Borrow Checking）**，编译期确定每个对象的释放时机 | 学习曲线陡峭；某些写法需要改变思维习惯 |

Rust 的思路非常优雅：**每个值在任意时刻有且只有一个所有者（owner），当所有者离开
作用域时，值被自动释放。** 这条规则加上一系列配套的借用规则，让编译器可以在编译期
精确推算出每一块内存何时该被释放——于是既不需要 GC，也不需要程序员手动管理。

来看一个直观的对比。在 C++ 里，下面这种错误屡见不鲜：

```cpp
// C++：经典悬垂指针
std::string* get_name() {
    std::string name = "Alice";
    return &name;   // 返回局部变量的指针！函数结束后 name 已被销毁
}                   // 调用者拿到的是一块"悬垂"的内存
```

同样的代码用 Rust 写出来，编译器会**直接拒绝编译**：

```rust
// Rust：这段代码无法通过编译
fn get_name() -> &String {
    let name = String::from("Alice");
    &name   // ❌ 编译错误：cannot return reference to local variable `name`
}
```

编译器的报错信息大致是：

```text
error[E0515]: cannot return reference to local variable `name`
 --> src/main.rs:3:5
  |
3 |     &name
  |     ^^^^^ returns a reference to data owned by the current function
```

注意：**这不是运行时崩溃，而是编译失败。** 错误在代码运行之前就被挡下了。
这正是"内存安全无 GC"的含义——安全检查发生在编译期，运行时零开销。

> ⚠️ **常见误区**：很多初学者以为 Rust 是"另一种带 GC 的现代语言"。恰恰相反，
> Rust 没有 GC。它靠的是一套编译期的静态分析系统（所有权 + 借用 + 生命周期）。
> 也正因为没有 GC，Rust 才能进入那些 GC 语言无法进入的领域：操作系统内核、
> 嵌入式设备、实时系统、对延迟极度敏感的金融交易等。

#### 1.2.2 零成本抽象（Zero-Cost Abstractions）

这个理念继承自 C++ 之父 Bjarne Stroustrup 的名言：

> "你不使用的东西，不需要为之付出代价；你使用的东西，不可能手写得更好。"

很多语言提供的高级抽象是有运行时代价的。例如 Java 的迭代器涉及装箱、虚函数调用；
Python 的一切都是对象，循环本身就是巨大的开销。而 Rust 的目标是：**高级抽象编译
之后，和手写的底层代码一样快。**

一个经典例子：对数组中所有偶数求平方和。

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 函数式风格的链式调用
    let sum: i32 = numbers
        .iter()              // 创建迭代器
        .filter(|n| n % 2 == 0)  // 只保留偶数
        .map(|n| n * n)      // 平方
        .sum();              // 求和

    println!("偶数平方和 = {}", sum);   // 输出：偶数平方和 = 220
}
```

这段代码看起来"很高级"——闭包、迭代器、链式调用。但经过 LLVM 优化后，它生成的
汇编代码与你手写一个循环逐个判断、累加**几乎完全一致**，甚至经常被完全展开和
向量化（auto-vectorization）。这就是"零成本"：抽象只存在于编译期，运行时消失。

零成本抽象在 Rust 中的典型体现：

- **迭代器（Iterator）**：链式调用编译后与手写循环等价。
- **泛型（Generics）**：单态化（monomorphization）——编译器为每个具体类型生成
  专门的代码，没有运行时类型擦除的间接开销（与 Java 泛型的类型擦除形成鲜明对比）。
- **闭包（Closure）**：捕获环境的方式在编译期确定，可以内联。
- **Option / Result 枚举**：`Option<&T>` 这类类型的内存布局与裸指针完全相同
  （空指针优化），没有额外开销。

#### 1.2.3 无畏并发（Fearless Concurrency）

并发编程有两座大山：**数据竞争（data race）**和**死锁**。其中数据竞争最可怕——
它的发生依赖时序，难以复现，可能在测试环境潜伏数月，到生产环境才爆发。

各语言对数据竞争的态度：

- **C/C++**：数据竞争是未定义行为（UB），语言不提供任何保护，全靠程序员。
- **Java**：有 `synchronized`、`volatile`、`java.util.concurrent`，但**忘了加锁**
  编译器不会提醒你——编译能通过，跑起来偶发出错。
- **Go**：提供 race detector，但那是**运行时检测**——只有跑到那段代码且恰好
  竞争发生时才能发现。
- **Rust**：**编译期**保证。含有数据竞争的代码（在安全 Rust 范围内）无法通过编译。

Rust 的秘诀还是那套所有权系统：一个值要么只有一个可变引用，要么可以有多个不可变
引用，**二者不可兼得**（"可写则独占，可读可共享"）。配合 `Send` / `Sync` 两个
标记 trait（标记类型能否安全地跨线程移动/共享），编译器就能静态证明你的并发代码
没有数据竞争。

后面进阶篇会深入这个话题。现在你只要记住：**在 Rust 里，如果并发代码编译通过了，
你就可以"无畏"——它不会有数据竞争。** 这就是 "Fearless Concurrency"。

> 💡 **提示**：三大哲学背后是同一条主线——**把尽可能多的错误从运行时提前到编译期
> 发现**。理解了这条主线，后面学到所有权、借用、生命周期时，你就不会觉得它们是
> 莫名其妙的限制，而会理解它们都是为这条主线服务的。

### 1.3 Rust 与其他语言对比

下面这张表是全书会反复回味的"地图"。请先浏览，不必细究每一格，学到后面再回来看，
每次都会有新的体会。

| 维度 | Rust | C++ | Java | Go | Python |
|------|------|-----|------|-----|--------|
| **内存管理** | 所有权系统，编译期自动确定释放时机，无 GC | 手动管理 + RAII + 智能指针（靠自觉） | GC（JVM） | GC（轻量，并发式） | GC（引用计数 + 分代回收） |
| **内存安全** | 编译期保证（无空指针、悬垂指针、double free、缓冲区溢出） | 不保证，UB 遍地 | 保证（有 GC 兜底），但有空指针 NPE | 基本保证（有 GC），但有空指针 panic 和数据竞争可能 | 保证 |
| **并发安全** | 编译期杜绝数据竞争 | 全靠程序员 | 靠同步原语，忘了加锁编译不报错 | race detector 运行时检测 | GIL 限制了真并行，竞争仍可能发生 |
| **运行时性能** | 与 C/C++ 同级，无 GC 停顿 | 顶级 | 高（JIT），但有 GC 停顿和预热 | 高，有少量 GC 停顿 | 低（解释执行，通常慢 10~100 倍） |
| **启动速度 / 内存占用** | 极低，无运行时 | 极低 | 慢（JVM 启动）、内存占用高 | 快、内存占用低 | 慢、内存占用高 |
| **空值处理** | `Option<T>` 枚举，强制处理 | 裸指针可为空 | `null`（NPE 臭名昭著） | `nil` 指针 | `None` |
| **错误处理** | `Result<T, E>` 枚举，强制处理 | 异常 + 返回码混用 | checked/unchecked 异常 | 显式返回 `error` 多值 | 异常 |
| **包管理 / 构建工具** | Cargo（官方统一，开箱即用） | 碎片化（CMake、Bazel、vcpkg、conan……） | Maven / Gradle | go modules | pip / poetry / uv…… |
| **学习曲线** | 陡峭（所有权概念全新） | 极陡峭（历史包袱重、规则繁杂） | 平缓 | 非常平缓 | 非常平缓 |
| **编译速度** | 较慢（安全检查 + LLVM 优化） | 慢 | 快 | 非常快 | 无需编译 |
| **典型应用** | 系统软件、CLI、WebAssembly、区块链、高性能后端 | 游戏引擎、浏览器、高频交易、桌面软件 | 企业级后端、Android | 云原生基础设施、微服务 | 数据科学、AI、脚本、Web 后端 |

几个值得展开说的对比点：

**Rust vs C++**：两者性能同级、都没有 GC。区别在于 C++ 的内存安全靠"程序员不出错"，
而 C++ 标准委员会也承认这做不到——微软和 Google 的统计都表明，其大型 C/C++ 代码库中
约 70% 的严重安全漏洞源于内存安全问题。Rust 用编译器把这个问题解决了。代价是
Rust 的所有权规则会拒绝一些在 C++ 里"能跑但危险"的写法。

**Rust vs Java**：Java 开发者最容易被 Rust 吸引的点：没有 NPE、没有 GC 停顿、
内存占用低一个数量级（云账单直接缩水）、编译产物是单个原生可执行文件（不用装 JVM）。
代价是编译变慢、学习成本变高、生态（尤其是企业级框架）不如 Java 成熟。

**Rust vs Go**：两者常被拿来比较，但定位不同。Go 的设计哲学是"简单优先"——语法极简、
编译飞快、有 GC 兜底，适合快速构建网络服务。Rust 的设计哲学是"安全与性能优先"——
愿意为正确性和极致性能付出学习成本。如果你的服务对延迟长尾（p99/p999）敏感，
GC 停顿会成为问题，Rust 更合适；如果是普通的 CRUD 微服务，Go 的开发效率更高。

**Rust vs Python**：不是竞争关系，更多是互补。很多 Python 库的性能关键部分
（如 `pydantic-core`、`ruff`、`polars`、`uv`）其实就是用 Rust 写的（通过 PyO3）。
Python 负责胶水层，Rust 负责性能层，是非常流行的组合。

### 1.4 Rust 适合做什么

Rust 不是银弹，但在以下领域它已经证明了巨大价值：

**1. 系统软件与基础设施**
操作系统组件（Linux 内核模块、Windows 内核组件）、数据库（TiKV、SurrealDB、
ClickHouse 的部分组件）、浏览器引擎（Servo 项目，其成果已并入 Firefox 的 Stylo
和 WebRender）、虚拟化（Firecracker）。这是 Rust 的"主场"——以前只有 C/C++ 能干的活。

**2. WebAssembly（Wasm）**
Rust 是编译到 Wasm 最成熟的语言：没有 GC 意味着产物小巧，工具链（`wasm-pack`、
`wasm-bindgen`）完善。浏览器内的高性能计算（Figma 的渲染引擎就是 C++/Wasm 路线，
而许多新项目选 Rust/Wasm）、边缘计算（Cloudflare Workers）、插件系统
（如 Envoy 的 Wasm 扩展）都是典型场景。

**3. 命令行工具（CLI）**
Rust 是重写 Unix 经典工具的热门选择：`ripgrep`（搜索，比 grep 快一个数量级）、
`fd`（find 替代品）、`bat`（cat 替代品）、`exa/eza`（ls 替代品）、`starship`
（终端提示符）、`zellij`（终端复用器）。原因：单文件分发、启动快、跨平台、
`clap` 等库生态成熟。

**4. 后端与网络服务**
`tokio` 异步运行时 + `axum` / `actix-web` 框架，让 Rust 后端在性能排行榜上
常年霸榜（TechEmpower Benchmark）。适合对性能和资源成本敏感的服务：
Cloudflare 用 Rust 重写的 Pingora 代理每秒处理数千万请求，CPU 和内存占用
相比旧系统大幅下降。

**5. 嵌入式**
Rust 可以 `#![no_std]`（不用标准库）运行在裸机上，已有面向 ARM Cortex-M、
RISC-V 等平台的成熟 HAL 生态（`embedded-hal`），ESP32、树莓派 Pico 等都有
活跃的社区支持。内存安全在嵌入式领域尤其宝贵——那里连 segfault 调试器都没有。

**6. 其他活跃领域**
区块链（Solana、Polkadot、Near 的核心都是 Rust）、游戏（Bevy 引擎、Embark 工作室）、
AI 基础设施（`candle`、`burn` 深度学习框架，以及前面提到的 Python 库底层）、
桌面应用（Tauri——用系统 WebView + Rust 后端替代 Electron，安装包从上百 MB 缩到几 MB）。

> 💡 **什么时候不该选 Rust**：团队没有 Rust 经验且交付压力极大、业务逻辑频繁变动
> 需要快速试错（Rust 的编译期严格性在需求不稳定时是负担）、或者纯粹的数据分析 /
> 机器学习实验（Python 生态无可替代）。语言是工具，场景匹配最重要。

### 1.5 从源码到可执行文件：Rust 的编译模型

最后，了解一下你写的 `.rs` 文件是如何变成可执行程序的，这对理解 Rust 的
很多行为（编译慢、产物小、无运行时依赖）很有帮助。

```
  hello.rs
      │
      ▼
┌─────────────┐   解析、宏展开、类型检查、借用检查（内存安全在这里把关！）
│    rustc    │   ── 大部分编译时间花在这里 ──
└─────────────┘
      │  LLVM IR（中间表示）
      ▼
┌─────────────┐   优化（内联、向量化、死代码消除……零成本抽象在这里兑现）
│    LLVM     │
└─────────────┘
      │  目标平台机器码
      ▼
┌─────────────┐   链接标准库与依赖（默认静态链接 Rust 库）
│   链接器     │   Windows 上是 MSVC 的 link.exe
└─────────────┘
      │
      ▼
  hello.exe   ← 单个原生可执行文件，双击即可运行
```

几个由此而来的重要事实：

1. **Rust 编译为原生机器码**。没有虚拟机（不像 Java 的 JVM）、没有解释器
   （不像 Python）。编译产物就是 CPU 直接执行的指令。
2. **默认静态链接 Rust 生态的一切**。你的 `hello.exe` 已经包含了所用到的
   Rust 标准库和所有 crate 的代码，拷贝到另一台没装任何 Rust 环境的
   Windows 机器上也能跑（这一点和 Go 类似；Java 需要目标机装 JRE，Python 更
   不必说）。唯一例外是 Windows 的系统 C 运行时库，Windows 10/11 自带。
3. **编译是"AOT"（Ahead-of-Time）**。Java 的 JIT 在运行时才把热点代码编译
   为机器码（所以有"预热"问题）；Rust 在编译期就把所有优化做完——这是
   Rust 编译慢的直接原因，也是它启动即巅峰性能的原因。
4. **前端是 rustc，后端是 LLVM**。LLVM 是业界顶级的编译器基础设施
   （Clang、Swift 也用它），Rust 站在巨人肩膀上获得了一流的代码优化能力和
   跨平台能力（x86、ARM、RISC-V、Wasm……）。

理解了这张流程图，你就不会再疑惑"为什么 Rust 编译比 Go 慢"（它在编译期做的
检查和优化多得多）、"为什么 Rust 程序部署那么简单"（单文件、无运行时依赖）。

### 1.6 学习路线图：接下来的旅程

最后给你一张地图，告诉你本书后续篇章会带你走到哪里，以及每一站为什么是
必经之路：

```
  你现在在这里
      │
      ▼
  第一篇 入门 ──────────────────────────────
  环境搭建、Cargo、基础语法（本篇）
      │
      ▼
  第二篇 核心 ──────────────────────────────
  所有权 / 借用 / 生命周期（Rust 的灵魂）
  结构体、枚举、模式匹配（match）
  错误处理（Result 与 ? 运算符）
      │
      ▼
  第三篇 进阶 ──────────────────────────────
  泛型、Trait、迭代器与闭包
  智能指针（Box / Rc / RefCell）
  模块系统与项目组织
      │
      ▼
  第四篇 实战 ──────────────────────────────
  并发编程（无畏并发兑现时刻）
  异步 Rust（tokio）与网络编程
  完整项目实战
```

**给初学者的三条忠告**（来自无数过来人的共同经验）：

1. **不要跳过所有权直接写项目。** 很多有 C++/Java 经验的人觉得"我先写着，
   遇到 borrow checker 报错再说"，结果处处碰壁后弃坑。所有权不是可以绕过
   的障碍，而是理解 Rust 一切的钥匙——值得专门花时间学透。
2. **把编译器当老师，而不是敌人。** Rust 的报错信息是全编程语言界出了名地
   友好：它会指出错误位置、解释原因、甚至直接给出修复代码。逐条阅读报错，
   你的 Rust 水平会飞速提升。
3. **前两周写"丑代码"没关系。** 新手常见现象是写出来的 Rust 像"带借用的
   Java"。地道感（idiomatic Rust）来自大量阅读别人的代码（标准库源码、
   Clippy 建议、开源项目），量变引起质变。

### 本章小结

- Rust 是一门系统级语言，核心目标是**在不使用 GC 的前提下保证内存安全**。
- 三大设计哲学：**内存安全无 GC**（所有权系统在编译期管理内存）、**零成本抽象**
  （高级抽象编译后与手写底层代码一样快）、**无畏并发**（编译期杜绝数据竞争）。
- Rust 把大量错误从运行时提前到编译期，这是理解其一切语言设计的钥匙。
- 与 C++ 比，Rust 以编译期严格性换取内存安全；与 Java/Go 比，Rust 没有 GC 停顿、
  资源占用低；与 Python 比，Rust 快一到两个数量级，且常作为其性能底座。
- Rust 的主战场：系统软件、WebAssembly、CLI 工具、高性能后端、嵌入式、区块链。

### 动手练习

1. **调研练习**：在 crates.io（Rust 官方包仓库）和 GitHub 上各找一个你所在领域
   （如 Web 后端、数据分析、游戏）的知名 Rust 项目，阅读其 README，写下它选择
   Rust 的两条理由。
2. **对比练习**：回想你熟悉的语言中最近一次（或最典型的一次）线上 bug，它属于
   哪一类（空指针？并发？内存？）？如果换用 Rust，这类 bug 在编译期能被挡下吗？
   为什么？
3. **思考题**：Rust 没有 GC，但内存也不会泄漏（在正常使用范围内）。请用一句话
   向一位不懂 Rust 的同事解释"所有权"是怎么做到这一点的（提示：从"谁负责释放"
   这个角度思考）。

---

## 第 2 章 搭建开发环境（Windows 11 + RustRover）

工欲善其事，必先利其器。Rust 的工具链安装体验在所有系统级语言里属于顶级——
官方提供了统一的安装器 rustup，一键搞定。本章我们把 Windows 11 + RustRover 的
完整开发环境搭建好，并跑通第一个项目。

本章结束时，你的环境应该是这样的：

```
┌─────────────────────────────────────────────────────────┐
│  Windows 11                                             │
│  ┌───────────────────────────────────────────────────┐  │
│  │  rustup（工具链管理器）                            │  │
│  │  ├── stable-x86_64-pc-windows-msvc  ← 默认工具链   │  │
│  │  │    ├── rustc（编译器）                          │  │
│  │  │    ├── cargo（构建工具 + 包管理器）             │  │
│  │  │    ├── rust-std（标准库）                      │  │
│  │  │    ├── rust-analyzer（IDE 语言服务器）         │  │
│  │  │    ├── rustfmt（代码格式化）                   │  │
│  │  │    └── clippy（代码检查/lint）                 │  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │  RustRover（IDE，通过 rust-analyzer 理解 Rust 代码）│  │
│  └───────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Visual Studio Build Tools（MSVC 链接器，必需！）  │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 2.1 安装前的准备：Visual Studio Build Tools

在 Windows 上，Rust 默认使用 **MSVC 工具链**（`x86_64-pc-windows-msvc`），它需要
微软的 C/C++ 链接器（`link.exe`）和系统库。这些包含在 Visual Studio Build Tools 中。

> ⚠️ **常见误区**：这一步是 Windows 上装 Rust 最容易踩的坑！如果跳过这一步直接装
> rustup，后面编译任何依赖 C 库的 crate 时都会报 `linker 'link.exe' not found`。
> 好消息是：新版 rustup 安装器已经能自动帮你安装 Build Tools（见 2.2），但提前
> 了解原理有助于排查问题。

**手动安装方式（推荐提前装好）：**

1. 访问 <https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/>
2. 下载"生成工具 for Visual Studio"（Build Tools for Visual Studio）
3. 运行安装器，勾选工作负载 **"使用 C++ 的桌面开发"**（Desktop development with C++）
4. 确认右侧可选组件中勾选了 **Windows 10/11 SDK** 和 **MSVC v143 生成工具**
   （默认会勾）
5. 安装，大约需要 4~7 GB 磁盘空间

> 💡 **提示**：你可能会问——能不能用 GNU 工具链（`x86_64-pc-windows-gnu`，基于
> MinGW）避免装这几个 GB 的 Build Tools？可以，但不推荐：MSVC 工具链生成的
> 程序与 Windows 生态兼容性最好（调试器、性能分析器、系统 API、绝大多数 crate
> 的 CI 都以 MSVC 为首要目标），而且微软官方也推荐使用。除非有特殊需求
> （如必须静态链接、交叉编译），请使用默认的 MSVC。

### 2.2 安装 rustup（Rust 官方安装器）

**rustup** 是 Rust 官方的工具链管理器，类似于 Python 的 pyenv/conda、
Node.js 的 nvm、Java 的 SDKMAN——但它的地位更高，是**官方唯一推荐**的安装方式。

它管理的内容包括：

- Rust 编译器 `rustc`
- 构建工具与包管理器 `cargo`
- 标准库文档 `rust-docs`
- 以及通过组件机制安装的 `rust-analyzer`、`rustfmt`、`clippy` 等

**安装步骤：**

1. 用浏览器打开官方站点：<https://rustup.rs/>
   （它会自动识别你的操作系统，给出 `rustup-init.exe` 的下载按钮）
2. 下载 `rustup-init.exe`（64 位版本）并运行。
3. 出现命令行界面后，你会看到类似这样的提示：

```text
Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  C:\Users\<你的用户名>\.rustup

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  C:\Users\<你的用户名>\.cargo\bin

Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
```

4. 直接按回车选择 `1) Proceed with standard installation`。
   - 如果检测到缺少 Visual Studio Build Tools，新版 rustup 会提示是否自动安装，
     按提示选择即可。
   - 安装过程会下载约 300MB 内容。
5. 看到 `Rust is installed now. Great!` 即表示成功。

**安装完成后的目录结构：**

```
C:\Users\<你的用户名>\
├── .rustup\                  # rustup 管理的工具链本体
│   ├── toolchains\
│   │   └── stable-x86_64-pc-windows-msvc\
│   │       ├── bin\          # rustc、cargo 的真实位置
│   │       └── lib\rustlib\  # 标准库
│   └── settings.toml
└── .cargo\
    ├── bin\                  # 已加入 PATH 的命令入口（rustup 的代理）
    │   ├── cargo.exe
    │   ├── rustc.exe
    │   ├── rustup.exe
    │   └── ...
    └── registry\             # 以后下载的第三方 crate 缓存
```

> 💡 **提示**：`.cargo\bin` 下的 `cargo.exe`、`rustc.exe` 其实是 rustup 的"代理"
> （hard link 或 shim），它会根据你当前项目的要求把命令转发给正确的工具链版本。
> 这就是 rustup 能在一台机器上无缝管理多个 Rust 版本的原理。

### 2.3 验证安装

**关闭并重新打开**一个终端（PowerShell 或 Windows Terminal，让 PATH 生效），
执行：

```powershell
rustc --version
cargo --version
rustup --version
```

预期输出（版本号会随时间更新，形式如下）：

```text
rustc 1.83.0 (90b35a623 2024-11-26)
cargo 1.83.0 (5ffbef321 2024-10-29)
rustup 1.27.1 (54dd3d00f 2024-04-24)
```

三条命令都有版本输出，说明工具链就绪。

> ⚠️ **常见误区**：如果提示 `'rustc' 不是内部或外部命令`，几乎都是 PATH 没生效。
> 解决方法：完全关闭所有终端窗口再开一个新的；还不行就检查环境变量 `Path` 中
> 是否有 `C:\Users\<用户名>\.cargo\bin`（系统属性 → 环境变量）。

**顺便安装两个重要组件**（rustup 默认 profile 通常已包含 rustfmt 和 clippy，
确认一下没坏处）：

```powershell
rustup component add rustfmt clippy rust-analyzer
```

- `rustfmt`：官方代码格式化工具（类似 gofmt、black）
- `clippy`：官方 linter，能给出大量"这样写不够地道"的建议，是 Rust 新手最好的老师
- `rust-analyzer`：语言服务器（LSP 实现），RustRover 用它或内置引擎来理解代码

### 2.4 常用 rustup 命令一览

rustup 不只是安装器，日常开发中你会经常用到它：

```powershell
# 更新所有已安装的工具链到最新版本
rustup update

# 查看当前安装的工具链
rustup show

# 安装 nightly（每日构建版，用于尝鲜新特性）
rustup toolchain install nightly

# 在某项目目录下切换默认工具链
rustup override set nightly

# 添加交叉编译目标（例如编译到 WebAssembly）
rustup target add wasm32-unknown-unknown

# 打开本地离线文档（强烈推荐！内含完整官方教程）
rustup doc
```

> 💡 **提示**：`rustup doc` 是隐藏福利——它会在浏览器中打开一整套**离线**官方
> 文档，包括 *The Rust Programming Language*（俗称"Rust 圣经"）、标准库 API 文档、
> Cargo 手册、rustdoc 手册等。没网也能查，以后你会非常频繁地用到它。

#### 补充：stable、beta、nightly 三个发布通道

Rust 采用"火车模型（train model）"发布：每 6 周发布一个新的 stable 版本，
新特性像火车到站一样按时发车，而不是攒大招。三个通道：

| 通道 | 更新频率 | 用途 |
|------|---------|------|
| **stable**（稳定版） | 每 6 周 | 日常开发、生产环境，**新手就用它** |
| beta（测试版） | 每 6 周滚动 | 下一个 stable 的候选版，供社区提前测试 |
| nightly（每日构建） | 每天 | 包含未完成的不稳定特性，供库作者尝鲜实验 |

关键设计：nightly 上的新特性默认被"特性开关（feature gate）"锁住，
只有显式声明 `#![feature(xxx)]` 才能使用，而这些开关在 stable 上无效。
于是 Rust 既能快速迭代语言，又能保证 stable 用户的代码永远稳定——
"稳定而不停滞（stability without stagnation）"是 Rust 社区的口号。

```powershell
rustup toolchain install nightly     # 安装 nightly（可以并存，互不影响）
cargo +nightly build                 # 临时用 nightly 构建一次
rustup default stable                # 切回 stable（全局默认）
```

### 2.5 安装 RustRover

RustRover 是 JetBrains 于 2023 年推出的 Rust 专用 IDE（在此之前 Rust 支持通过
CLion/IntelliJ 插件提供）。它对**个人非商业用途免费**，商业使用需要许可证；
学生和开源维护者可申请免费授权。

**方式一：JetBrains Toolbox 安装（推荐）**

Toolbox 是 JetBrains 的 IDE 管理器，类似 rustup 之于 Rust——可以管理多个 IDE
版本、一键更新、管理许可证：

1. 访问 <https://www.jetbrains.com/toolbox-app/> 下载 Toolbox
2. 安装并登录 JetBrains 账号
3. 在 Toolbox 中找到 **RustRover**，点击 Install
4. 以后 IDE 更新、回滚、多版本共存都在 Toolbox 里完成

**方式二：直接下载安装包**

1. 访问 <https://www.jetbrains.com/rustrover/download/>
2. 下载 Windows 版 `.exe` 安装器
3. 安装时建议勾选：
   - `Create Desktop Shortcut`（桌面快捷方式）
   - `Add "Open Folder as Project"`（右键菜单"作为项目打开"，非常实用）
   - `.rs` 文件关联（可选）

### 2.6 在 RustRover 中配置 Rust 工具链

首次启动 RustRover，它通常会自动检测到 rustup 安装的工具链（因为路径是标准的
`~/.cargo` / `~/.rustup`）。如果没有自动检测到，手动配置：

1. 打开 `File → Settings`（快捷键 `Ctrl + Alt + S`）
2. 导航到 `Build, Execution, Deployment → Rust`（部分版本路径为 `Languages & Frameworks → Rust`）
3. 确认以下两项：
   - **Toolchain location**：`C:\Users\<你的用户名>\.cargo\bin`
     （RustRover 会自动找到其中的 `cargo.exe`）
   - **Standard library**：指向
     `C:\Users\<你的用户名>\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library`
     （标准库源码路径；如果显示缺失，点击提示中的 `Download via rustup`，
     它执行的等价命令是 `rustup component add rust-src`）

配置正确后，设置页面会显示检测到的 rustc 版本号和 rustup 信息。

> ⚠️ **常见误区**：标准库源码（`rust-src` 组件）没装的话，Ctrl+点击跳转到标准库
> 源码、`Option`/`Vec` 等类型的自动补全文档会缺失或退化。务必确保装了
> `rustup component add rust-src`。

### 2.7 关于 rust-analyzer

**rust-analyzer** 是 Rust 官方的语言服务器协议（LSP）实现——它才是真正"读懂"
你代码的大脑：自动补全、跳转定义、类型提示、实时错误检查，都由它提供。

你需要知道的事实：

- RustRover **内置**了 Rust 语言支持引擎（继承自老牌 intellij-rust 插件与
  rust-analyzer 多年融合的成果），**不需要你单独安装任何东西**即可工作。
- 如果你同时也用 VS Code，那么在 VS Code 里要安装 `rust-analyzer` 扩展；
  命令行下也可以用 `rustup component add rust-analyzer` 安装。
- rust-analyzer 与编译器 `rustc` 是两套独立的分析器，偶尔会出现 rust-analyzer
  认为没问题但 `cargo build` 报错（或反之）的情况——**永远以 `cargo` 的编译
  结果为准**。

### 2.8 RustRover 常用设置与插件

RustRover 开箱即用，但以下几项设置能让体验更好（都在 `File → Settings` 中）：

| 设置项 | 路径 | 建议 |
|--------|------|------|
| 保存时自动格式化 | `Languages & Frameworks → Rust → Rustfmt` | 勾选 "Run rustfmt on Save"，永远不用纠结格式 |
| 自动导入 | `Editor → General → Auto Import` | 开启 Rust 的自动导入（输入类型名后自动加 `use`） |
| Clippy 检查 | `Languages & Frameworks → Rust → External Linters` | 把外部检查器设为 **Clippy**，获得比 rustc 更丰富的提示 |
| Inlay Hints（内联类型提示） | `Editor → Inlay Hints → Rust` | 新手期建议全开：变量类型、闭包参数类型、链式调用类型一目了然 |
| 终端 | `Tools → Terminal` | 设为 PowerShell 7 或 Git Bash，IDE 内置终端跑 cargo 命令很顺手 |

**值得安装的插件**（`Settings → Plugins → Marketplace`）：

- **TOML**：编辑 `Cargo.toml` 时的语法高亮与补全（新版 RustRover 已内置）
- **.env files support**：管理环境变量文件
- **Rainbow Brackets**：括号配色，嵌套泛型多时很救命
- **Key Promoter X**：记录你每次用鼠标的操作并提示对应快捷键，快速练成键盘流
- **GitToolBox**：增强 Git 集成（行内 blame、状态统计）

### 2.9 创建第一个项目

1. 启动 RustRover → 欢迎界面点击 **New Project**
2. 左侧选择 **Rust**：
   - **Location**：选择项目路径，如 `C:\Users\<你>\RustProjects\hello_rust`
   - **Toolchain**：确认显示 `stable`（如果没有，说明 2.6 节的配置未完成）
   - **Project type / template**：选择 `Binary (application)`（可执行程序；
     另一个选项 `Library` 是给别人引用的库，第 3 章详述区别）
3. 点击 **Create**

RustRover 会在后台执行等价于 `cargo new hello_rust` 的命令，生成项目并建立索引。
第一次索引可能需要一两分钟（它要分析整个标准库），之后就很流畅了。

生成的项目结构：

```
hello_rust/
├── .git/               # cargo new 默认初始化 git 仓库
├── .gitignore          # 忽略 target/ 目录
├── Cargo.toml          # 项目配置与依赖清单（第 3 章详解）
└── src/
    └── main.rs         # 入口文件，已经写好了 Hello World
```

`src/main.rs` 的初始内容：

```rust
fn main() {
    println!("Hello, world!");
}
```

**运行它**：点击编辑器左侧 `fn main` 旁边的绿色 ▶ 三角图标，选择 `Run`，
或者使用快捷键 `Shift + F10`。底部 Run 窗口会显示：

```text
   Compiling hello_rust v0.1.0 (C:\...\hello_rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.42s
     Running `target\debug\hello_rust.exe`
Hello, world!
```

恭喜！环境搭建完成。

### 2.10 RustRover 常用快捷键速查

以下快捷键基于 Windows 默认键位（IntelliJ 平台通用），值得形成肌肉记忆：

| 功能 | 快捷键 | 说明 |
|------|--------|------|
| 运行 | `Shift + F10` | 运行当前选定的配置（通常是 main） |
| 调试 | `Shift + F9` | 以调试模式启动（可在行号旁点击打断点） |
| 停止 | `Ctrl + F2` | 终止正在运行的程序 |
| 自动补全 | `Ctrl + Space` | 基本补全；`Ctrl + Shift + Space` 智能补全（按类型过滤） |
| 后缀补全 | 输入表达式后按 `.` | 如 `x.println` + Tab → `println!("{}", x);`，Rust 开发神器 |
| 快速修复 | `Alt + Enter` | **最重要的快捷键！** 光标停在报错/警告上按下，给出自动修复方案 |
| 跳转定义 | `Ctrl + B` 或 `Ctrl + 点击` | 跳到函数/类型定义处 |
| 查看类型 | `Ctrl + Shift + P` | 显示光标处表达式的完整类型 |
| 查看文档 | `Ctrl + Q` | 悬停显示 rustdoc 文档 |
| 全局搜索 | 双击 `Shift` | 搜文件、类、符号、动作，万能入口 |
| 全局文本搜索 | `Ctrl + Shift + F` | 在项目所有文件中搜索文本 |
| 重命名重构 | `Shift + F6` | 安全重命名变量/函数/类型（自动改所有引用） |
| 提取变量 | `Ctrl + Alt + V` | 把选中表达式提取为局部变量 |
| 提取函数 | `Ctrl + Alt + M` | 把选中代码块提取为函数 |
| 内联 | `Ctrl + Alt + N` | 提取的逆操作 |
| 最近文件 | `Ctrl + E` | 在最近编辑的文件间切换 |
| 格式化代码 | `Ctrl + Alt + L` | 调用 rustfmt 格式化当前文件 |
| 打开终端 | `Alt + F12` | IDE 内置终端，直接跑 cargo 命令 |
| 展开/折叠代码 | `Ctrl + +/-` | 折叠函数体 |
| 多光标 | `Alt + Shift + 点击` | 多处同时编辑 |

> 💡 **提示**：`Alt + Enter` 值得你单独记住。Rust 编译器和 Clippy 的很多建议
> 都附带机器可应用的修复方案，RustRover 把它们集成在这个快捷键里。新手阶段
> 看到红色/黄色波浪线，第一反应就应该是 `Alt + Enter` 看看 IDE 怎么说——
> 这相当于一位随身教练。

> 💡 **提示：后缀补全（Postfix Completion）**是 Rust 开发中极具幸福感的功能。
> 输入 `name.if` 按 Tab 变成 `if name { ... }`；输入 `v.iter` 变 `v.iter()`；
> 输入 `x.match` 变 `match x { ... }`；输入 `expr.dbg` 变 `dbg!(expr)`。
> 在 `Settings → Editor → General → Postfix Completion` 里能看到完整列表。

### 2.11 使用 RustRover 调试程序

调试器是 IDE 相对命令行的最大优势。RustRover 在 Windows 上内置了 LLDB 调试器
（对 MSVC 工具链也有良好支持），开箱即用。用一个有 bug 的小程序来体验：

```rust
fn main() {
    let mut total = 0;

    for i in 1..=5 {
        total += i;
        println!("i = {}, total = {}", i, total);
    }

    let average = total / 5;
    println!("平均值 = {}", average);
}
```

**基本调试操作：**

1. **打断点**：在 `total += i;` 这一行的行号左侧点击，出现红点 🔴
   （快捷键 `Ctrl + F8` 切换断点）。
2. **启动调试**：`Shift + F9`（注意不是运行的 `Shift + F10`）。程序会在
   断点处暂停，编辑器高亮当前行。
3. **查看状态**：底部 Debug 窗口的 **Variables** 面板显示当前作用域内
   所有变量的值（`i = 1`，`total = 0`）；编辑器内代码旁边也会内联显示
   变量值——这对理解程序状态极为直观。
4. **单步控制**：

   | 操作 | 快捷键 | 作用 |
   |------|--------|------|
   | Step Over | `F8` | 执行当前行，不进入函数内部 |
   | Step Into | `F7` | 进入当前行调用的函数内部 |
   | Step Out | `Shift + F8` | 从当前函数返回出去 |
   | Resume | `F9` | 继续运行到下一个断点 |
   | 查看表达式 | `Alt + F8` | 临时计算任意表达式（如 `total * 2`） |

5. **监视（Watches）**：在 Debug 窗口点 `+` 添加任意表达式（如 `total / i`），
   它会随着程序执行实时更新。
6. **条件断点**：右键断点 → 设置 Condition，如 `i == 3`——只在条件满足时
   暂停。循环有几百次而你只关心某一次时，这是救星。

**值得养成的调试习惯**：与其在代码里撒满 `println!` 再删掉，不如先试试断点 +
Variables 面板。当然，`dbg!` 宏（见第 4 章）作为"打完不用删"的临时输出，
和断点调试是互补的两大法宝。

> 💡 **提示**：测试也可以用调试器运行——`#[test]` 函数旁边的绿色三角选择
> Debug，可以在断言失败前停下来检查中间状态，比只看 `cargo test` 的
> left/right 输出更深入。

### 2.12 命令行与 IDE 的关系

最后强调一个重要的观念：**RustRover 的一切构建、运行、测试操作，底层都是调用
cargo。** 两者之间没有秘密通道。这意味着：

- 你在 IDE 里做的项目，在命令行用 `cargo build` 可以原样构建，反之亦然；
- 提交代码到 CI（GitHub Actions 等）时，CI 里跑的就是 cargo 命令；
- 遇到 IDE 行为诡异时，在终端跑一遍 `cargo build` 是定位问题的分水岭——
  命令行也报错就是代码问题，命令行正常则多半是 IDE 索引问题
  （试试 `File → Invalidate Caches / Restart`）。

下一章我们就深入 cargo——Rust 生态最让人赞不绝口的工具。

### 2.13 附录：国内网络加速与常见问题排查

#### 配置 crates.io 国内镜像（强烈建议国内读者配置）

默认情况下，rustup 和 cargo 都从国外服务器下载，国内网络环境下可能非常缓慢。
好消息是国内有多个优秀的镜像源。配置分两部分：

**第一部分：rustup 下载工具链的镜像（设置环境变量）**

PowerShell 中临时设置（仅当前窗口有效）：

```powershell
$env:RUSTUP_DIST_SERVER = "https://rsproxy.cn"
$env:RUSTUP_UPDATE_ROOT = "https://rsproxy.cn/rustup"
```

永久设置（推荐）：系统设置 → 环境变量 → 用户变量，添加上面两个变量。
也可以在安装 rustup **之前**就设置好，这样安装过程本身就走镜像。

**第二部分：crates.io 下载依赖的镜像**

创建（或编辑）文件 `C:\Users\<你的用户名>\.cargo\config.toml`，写入：

```toml
# cargo 全局配置：使用国内镜像替代 crates.io 官方源
[source.crates-io]
replace-with = 'rsproxy-sparse'   # 指定替换源

# 字节跳动 RsProxy 镜像（稀疏索引协议，速度快）
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "sparse+https://rsproxy.cn/index/"

[net]
git-fetch-with-cli = true   # 个别 git 依赖走系统 git，更稳定
```

备选镜像还有清华大学 TUNA（`https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git`）、
中国科学技术大学 USTC 等，用法类似，任选其一即可。配置完成后，
`cargo build` 下载依赖的速度通常从"几分钟"提升到"几秒钟"。

> ⚠️ **常见误区**：网上一些旧教程让你把 `registry` 配置成以 `git://` 或普通
> `https://` 开头的 git 索引地址。那是旧的 git 索引协议，每次都要克隆整个
> 索引仓库（几百 MB）。新版 cargo（1.68+）默认使用 sparse（稀疏）索引，
> 镜像地址形如 `sparse+https://...`，请优先选择支持 sparse 的镜像。

#### 常见问题排查清单

| 症状 | 原因与解决 |
|------|-----------|
| `'rustc' 不是内部或外部命令` | PATH 未生效：关闭重开终端；检查 `Path` 中有无 `%USERPROFILE%\.cargo\bin` |
| `linker 'link.exe' not found` | 没装 Visual Studio Build Tools：回到 2.1 节安装"使用 C++ 的桌面开发"工作负载 |
| `cargo build` 卡在 `Updating crates.io index` | 网络问题：配置上面的镜像；或检查代理设置 |
| RustRover 里代码全红但 `cargo build` 正常 | IDE 索引损坏：`File → Invalidate Caches / Restart` |
| RustRover 找不到工具链 | `Settings → Rust` 中手动指定 Toolchain 路径为 `~\.cargo\bin` |
| 标准库跳转不了 / 补全没有文档 | 缺少源码组件：执行 `rustup component add rust-src` |
| 编译报 `failed to run custom build command for openssl-sys` 之类 | 某些 crate 依赖系统 C 库：优先选用纯 Rust 实现的替代品（如 HTTP 客户端 reqwest 用 `rustls-tls` 特性） |
| 杀毒软件报毒 `target/debug/*.exe` | Windows Defender 偶尔误报新编译的 exe：把项目目录加入排除列表 |

### 本章小结

- Windows 上安装 Rust 的完整链条：先装 **Visual Studio Build Tools**（提供 MSVC
  链接器），再从 **rustup.rs** 下载 `rustup-init.exe` 一键安装。
- rustup 是官方工具链管理器，管理 rustc / cargo / 组件，默认安装在
  `%USERPROFILE%\.rustup` 和 `%USERPROFILE%\.cargo`。
- 用 `rustc --version` / `cargo --version` 验证安装；`rustup update` 保持更新；
  `rustup doc` 打开离线官方文档。
- RustRover 在 `Settings → Build, Execution, Deployment → Rust` 中配置工具链，
  记得安装 `rust-src` 组件以便跳转标准库源码。
- rust-analyzer 是 IDE 智能功能的大脑；RustRover 已内置语言支持，无需单独安装。
- 推荐开启：保存时 rustfmt、Clippy 外部检查、Inlay Hints。
- 记住三个最重要的快捷键：`Shift+F10`（运行）、`Shift+F9`（调试）、
  `Alt+Enter`（快速修复）。

### 动手练习

1. **动手装一遍**：按本章流程完成安装，在 PowerShell 中执行 `rustup show`，
   把输出截图或抄录下来，确认包含 `stable-x86_64-pc-windows-msvc (default)`。
2. **熟悉 IDE**：在 RustRover 中新建项目 `env_test`，编写一个打印你名字的
   程序，分别用（a）绿色三角按钮、（b）`Shift+F10`、（c）内置终端
   `cargo run` 三种方式运行它，体会三者的等价关系。
3. **探索离线文档**：执行 `rustup doc`，在打开的页面中找到标准库文档
   （std），搜索 `String`，浏览它的方法列表，找出一个名为 `push_str` 的方法
   并阅读其文档示例。（这是你以后最常用的自学路径。）

---

## 第 3 章 Hello World 与 Cargo

### 3.1 从"没有 Cargo 的世界"说起

在深入 Cargo 之前，先体会一下其他语言的构建体验，你才能理解 Cargo 为什么被
公认为 Rust 生态最大的财富之一。

- **C++**：构建系统碎片化严重——CMake、Bazel、Meson、Make、xmake……各有各的
  语法。引入一个第三方库要经历：下载源码/二进制、配置 include 路径、链接路径、
  处理平台差异、处理传递依赖……很多团队为此养一个专职的构建工程师。
- **Java**：Maven/Gradle 解决了依赖管理，但 XML/Groovy 配置复杂，构建慢，
  而且"Maven 地狱"（依赖版本冲突）是老生常谈。
- **Go**：`go build` / `go mod` 体验很好，这是 Go 的成功要素之一。Cargo 与
  go modules 的设计目标相似，且做得更早（Cargo 随 Rust 1.0 就有，go modules
  2018 年才出现）。
- **Python**：`pip` + `requirements.txt` + `venv` 的组合常年被诟病，
  以至于诞生了 poetry、pipenv、uv 等大量"补锅"工具。

**Cargo 的设计哲学是：一个官方工具，统一解决所有问题。** 它集成了：

```
构建系统（build system）
  + 包管理器（package manager）
  + 依赖解析器（dependency resolver）
  + 测试运行器（test runner）
  + 文档生成器（doc generator）
  + 项目脚手架（project scaffolding）
  + 发布工具（crates.io 发布）
  + 插件平台（cargo install 第三方子命令）
```

全部开箱即用，配置文件只有一个简洁的 `Cargo.toml`。在 Rust 社区，你几乎找不到
不用 Cargo 的项目——这种统一性让"克隆任何一个开源项目 → `cargo run` 跑起来"
成为日常体验。

### 3.2 不用 Cargo 写一次 Hello World（理解原理）

为了理解 Cargo 在背后做了什么，我们先手动用 `rustc` 编译一次。

创建文件 `hello.rs`：

```rust
// hello.rs —— 直接用 rustc 编译的最小程序
fn main() {
    println!("Hello, world!");
}
```

编译并运行（PowerShell）：

```powershell
rustc hello.rs      # 生成 hello.exe（Windows 下自动加 .exe 后缀）
.\hello.exe         # 输出：Hello, world!
```

`rustc` 是 Rust 编译器本体。对于单文件程序它很顺手，但真实项目有几十个文件、
几十个第三方依赖、多种构建配置——手动调用 `rustc` 完全不现实。Cargo 就是
`rustc` 的"指挥官"：你告诉 Cargo 你要什么（依赖、构建配置），Cargo 负责调度
`rustc` 完成具体编译。

> 💡 **提示**：理解 Cargo 与 rustc 的关系，可以类比 Java 中 Maven 与 javac 的
> 关系，或者 Go 中 `go` 命令与编译器的关系。实际开发中你**几乎永远不直接调用
> rustc**，一切都通过 Cargo。

### 3.3 cargo new：创建项目

```powershell
# 创建一个可执行程序项目
cargo new hello_cargo

# 创建一个库项目（被别人依赖的代码）
cargo new my_lib --lib
```

`cargo new` 做了四件事：

1. 创建目录 `hello_cargo/`
2. 生成 `Cargo.toml`（配置文件）
3. 生成 `src/main.rs`（含 Hello World 模板）
4. 初始化 git 仓库并生成 `.gitignore`（忽略 `target/`）

> 💡 **提示**：不想要 git 仓库可以加 `--vcs none`；想把当前目录变成项目用
> `cargo init`。

### 3.4 项目目录结构

一个标准 Cargo 项目的完整约定结构如下（加粗的目录是 Cargo 的**约定**，
放在这些位置的文件会被自动识别，无需任何配置）：

```
hello_cargo/
├── Cargo.toml           # ★ 项目清单：元信息 + 依赖 + 构建配置
├── Cargo.lock           # ★ 依赖精确版本锁定文件（首次构建后自动生成）
├── .gitignore
├── src/
│   ├── main.rs          # ★ 二进制入口（fn main 所在文件）
│   ├── lib.rs           # ★ 库入口（如果项目同时是库）
│   └── bin/             # ★ 额外的二进制入口（每个 .rs 文件编译成一个可执行文件）
│       └── tool.rs
├── tests/               # ★ 集成测试目录（每个文件是一个独立的测试 crate）
│   └── integration_test.rs
├── benches/             # ★ 基准测试目录
├── examples/            # ★ 示例代码目录（cargo run --example xxx 运行）
│   └── demo.rs
└── target/              # ★ 构建产物（编译缓存，体积巨大，已被 gitignore）
    ├── debug/           #   开发构建产物
    └── release/         #   发布构建产物
```

这个"约定优于配置（Convention over Configuration）"的目录结构是 Cargo 体验
流畅的关键之一。对比 Java 需要在 `pom.xml` 里配置源码目录，C++ 需要在 CMake
里罗列源文件——Cargo 全部免配置。

### 3.5 Cargo.toml 详解

`cargo new` 生成的 `Cargo.toml` 内容如下：

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

逐字段解释：

| 字段 | 含义 |
|------|------|
| `[package]` | 包元信息区段的开始。TOML 格式中 `[xxx]` 表示一个"表"（section） |
| `name` | 包名。发布到 crates.io 时的唯一标识，也是默认的二进制文件名 |
| `version` | 版本号，遵循语义化版本（SemVer）：`主版本.次版本.修订号` |
| `edition` | **Rust 版本版次**，见下方详解 |
| `[dependencies]` | 依赖区段，当前为空 |

**关于 edition（版次）——Rust 独有的设计，务必理解：**

Rust 承诺**向后兼容**：2015 年 1.0 版本能编译的代码，今天的编译器依然能编译。
但语言总需要进化，有些改进必然引入破坏性变化（比如把 `async` 变成关键字）。
怎么办？Rust 的答案是 **edition**：

- Rust 目前有 `2015`、`2018`、`2021`、`2024` 四个版次，约三年发布一个。
- 每个 crate 在自己的 `Cargo.toml` 里声明用哪个版次。
- **不同版次的 crate 可以互相依赖**——编译器支持所有历史版次，依赖的库用
  2015 版次、你的项目用 2021 版次，完全没问题。
- 版次之间可以引入新关键字、改变部分语义，但每个版次内部严格向后兼容。

对比其他语言：Java 靠"永不删除任何东西"维持兼容（于是语言越来越臃肿，
`Vector`、`Date` 等历史包袱永远背着）；Python 2→3 的断裂让社区痛苦了十年。
Rust 的 edition 机制是一条优雅的中间道路：**语言可以进化，但旧代码永不损坏。**

> 💡 **提示**：新项目默认用当前最新的稳定版次即可（`cargo new` 会自动填）。
> 你只需要知道：看到 `edition = "2021"` 不要误以为是"Rust 2021 年发布的版本"，
> 它是语法/语义规则集的版本号。

**Cargo.toml 中你还会经常见到的其他区段：**

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]   # 作者（发布到 crates.io 时显示）
description = "我的第一个 Rust 项目"          # 描述
license = "MIT OR Apache-2.0"               # 许可证（Rust 生态惯例双许可）
readme = "README.md"
repository = "https://github.com/you/hello_cargo"

[dependencies]
# 运行时依赖（编译进最终产物）

[dev-dependencies]
# 仅测试/示例/benchmark 使用的依赖（不进入发布产物）
# 例如：pretty_assertions = "1.4"（更好看的断言输出）

[build-dependencies]
# 构建脚本（build.rs）使用的依赖

[profile.release]
# 发布构建的自定义配置（见 3.9 节）
```

### 3.6 Cargo 核心命令详解

以下命令请在项目根目录（`Cargo.toml` 所在目录）下执行。

#### cargo build —— 构建

```powershell
cargo build
```

- 编译当前项目及其所有依赖。
- 默认是 **debug 构建**：编译快、不优化、带调试信息，产物在 `target/debug/`。
- 使用**增量编译**：只重新编译改动过的 crate，第二次构建会快很多。

```text
   Compiling hello_cargo v0.1.0 (C:\...\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s
```

#### cargo run —— 构建并运行

```powershell
cargo run
cargo run -- 参数1 参数2     # "--" 之后的参数传给你的程序
```

- 等价于 `cargo build` + 运行 `target/debug/hello_cargo.exe`。
- 如果代码没改动，会跳过编译直接运行。
- 开发期最常用的命令。

#### cargo check —— 只检查，不生成可执行文件

```powershell
cargo check
```

- 只做**类型检查和借用检查**，跳过代码生成阶段。
- 速度通常是 `cargo build` 的几分之一。
- 写代码时的"语法体检"神器：改几行就 check 一下，快速得到编译器反馈。
- RustRover 的实时检查背后跑的其实就是它（或其变体 `cargo clippy`）。

> 💡 **开发节奏建议**：写代码 → `cargo check`（秒级反馈）→ 编译无误后
> `cargo test`（跑测试）→ 需要实际运行时才 `cargo run`。这个节奏能让你
> 与编译器的"搏斗"效率最大化。

#### cargo test —— 运行测试

先写个带测试的项目体验。把 `src/main.rs` 改成：

```rust
// 一个加法函数
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("1 + 2 = {}", add(1, 2));
}

// 测试模块：约定写在同一文件底部的 #[cfg(test)] 模块中
#[cfg(test)]
mod tests {
    use super::*;   // 引入外层作用域的所有内容

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);   // 断言相等
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -1), -2);
    }
}
```

运行：

```powershell
cargo test
```

输出：

```text
   Compiling hello_cargo v0.1.0 (C:\...\hello_cargo)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running unittests src\main.rs (target\debug\deps\hello_cargo-xxxxxxxx.exe)

running 2 tests
test tests::test_add_negative ... ok
test tests::test_add ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Cargo 的测试体验也是内置的：`#[test]` 标记的函数自动被测试运行器发现并**并行**
执行。对比一下——C++ 要选 GoogleTest/Catch2 并配置构建，Java 要配 JUnit 依赖和
构建插件，而 Rust 里测试是语言的一等公民。

#### cargo doc —— 生成文档

```powershell
cargo doc --open
```

- 为你的项目**及其所有依赖**生成 HTML API 文档，并在浏览器中打开。
- 文档内容来自代码中以 `///` 开头的文档注释（支持 Markdown）。
- 这是你查看第三方库 API 的重要方式（虽然更多人直接用在线的 docs.rs）。

#### 其他常用命令速查

| 命令 | 作用 |
|------|------|
| `cargo clean` | 删除整个 `target/` 目录（解决玄学的构建缓存问题，或释放磁盘——`target` 经常有好几 GB） |
| `cargo fmt` | 用 rustfmt 格式化所有代码（团队协作前必跑） |
| `cargo clippy` | 运行 Clippy 静态检查，给出"更地道的 Rust 写法"建议 |
| `cargo update` | 在 SemVer 兼容范围内更新 `Cargo.lock` 中的依赖版本 |
| `cargo tree` | 以树状图显示完整依赖关系（排查依赖冲突神器） |
| `cargo add xxx` | 向 Cargo.toml 添加依赖（自动查询最新版本） |
| `cargo remove xxx` | 移除依赖 |
| `cargo search xxx` | 在 crates.io 上搜索 crate |
| `cargo install xxx` | 安装一个 Rust 编写的命令行工具到 `~/.cargo/bin` |
| `cargo new --lib` | 创建库项目 |
| `cargo build --release` | 发布构建（见 3.9 节） |

### 3.7 依赖管理：crates.io

Rust 的官方包仓库是 **crates.io**（类比 npm、PyPI、Maven Central）。
截至 2024 年，上面有超过 15 万个 crate（Rust 中"包"的叫法）。

**添加依赖有三种方式：**

方式一：命令行添加（推荐）——

```powershell
cargo add rand
```

它会自动查询最新版本并写入 `Cargo.toml`：

```toml
[dependencies]
rand = "0.8.5"
```

方式二：手动编辑 `Cargo.toml`（在 `[dependencies]` 下加一行）。

方式三：在 RustRover 的 `Cargo.toml` 中输入 crate 名，IDE 会补全版本号。

**版本号 `"0.8.5"` 的真正含义**——这是 SemVer 语义化版本范围：

- `rand = "0.8.5"` 实际等价于 `rand = "^0.8.5"`（caret 要求），
  含义是"≥ 0.8.5 且 < 0.9.0"——允许兼容更新，不允许破坏性更新。
- 其他写法：`"=0.8.5"`（精确版本）、`"~0.8.5"`（≥0.8.5 <0.8.6）、
  `"*"`（任意版本，不推荐）。
- 注意 SemVer 约定：**0.x 版本**中次版本号变化视为破坏性更新
  （所以 `^0.8.5` 不允许升到 `0.9.0`）。

**试用一下 rand crate**。把 `src/main.rs` 改为：

```rust
// 引入 rand crate 的 Rng trait 和 thread_rng 函数
use rand::Rng;

fn main() {
    // thread_rng() 获取当前线程的随机数生成器
    let mut rng = rand::thread_rng();

    // gen_range 生成 [1, 100] 区间的随机整数
    let secret: i32 = rng.gen_range(1..=100);
    println!("我生成了一个随机数：{}", secret);

    // gen_bool 以给定概率返回 true
    if rng.gen_bool(0.5) {
        println!("硬币正面");
    } else {
        println!("硬币反面");
    }
}
```

`cargo run` 可能的输出：

```text
    Updating crates.io index
   Compiling libc v0.2.159
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.20
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling getrandom v0.2.15
   Compiling rand v0.8.5
   Compiling hello_cargo v0.1.0 (C:\...\hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 8.31s
     Running `target\debug\hello_cargo.exe`
我生成了一个随机数：42
硬币正面
```

注意第一次构建时发生的事：

1. Cargo 从 crates.io 下载了 `rand` **以及它的传递依赖**（`rand_core`、
   `rand_chacha`、`getrandom`、`libc` 等）——依赖的依赖全自动解决；
2. 先编译所有依赖（只在第一次或依赖变化时编译）；
3. 最后编译你的项目并运行。

**Cargo.lock 的作用**：构建后你会看到根目录多了 `Cargo.lock`，里面记录了
每个依赖的**精确**版本号和校验哈希。它的意义：

- 团队协作和 CI 构建时，所有人得到**完全相同**的依赖版本——"在我机器上能跑"
  问题大幅缓解；
- 二进制项目（application）应该**提交** `Cargo.lock` 到 git（cargo 默认如此）；
- 库项目（library）通常不提交（由使用者决定版本），这是社区惯例。

> 💡 **与 Python 对比**：`Cargo.toml` + `Cargo.lock` 的组合，相当于
> `pyproject.toml` + `poetry.lock`，但 Cargo 从第一天起就是官方标准，
> 全生态统一，不存在 pip/poetry/conda 三选一的分裂。

> 💡 **常用新手 crate 推荐**：`rand`（随机数）、`serde`（序列化/反序列化，
> Rust 生态的事实标准）、`serde_json`（JSON）、`clap`（命令行参数解析）、
> `anyhow` / `thiserror`（错误处理）、`regex`（正则）、`chrono`（日期时间）、
> `reqwest`（HTTP 客户端）、`tokio`（异步运行时）。先混个脸熟，后续章节会用到。

### 3.8 编译过程与 target 目录

执行 `cargo build` 后，`target/` 目录的结构：

```
target/
├── debug/
│   ├── hello_cargo.exe          # 最终可执行文件（含调试信息，体积大）
│   ├── hello_cargo.pdb          # Windows 调试符号文件
│   ├── deps/                    # 所有编译单元的中间产物（.rlib、.rmeta）
│   ├── incremental/             # 增量编译缓存
│   ├── build/                   # 构建脚本（build.rs）的输出
│   └── examples/
└── release/                     # cargo build --release 后才出现
    └── hello_cargo.exe          # 优化后的可执行文件（小且快）
```

几个事实：

- `target/` 可以**随时整个删除**（`cargo clean`），下次构建会重新生成；
- 依赖被编译为 `.rlib`（Rust 静态库格式），存放在 `deps/`；
- debug 构建的可执行文件**体积很大**（几十 MB 不稀奇），因为带完整调试符号
  且零优化——不要把它发给用户，发布要用 release 构建。

### 3.9 Release 构建与优化配置

```powershell
cargo build --release
cargo run --release
```

release 构建与 debug 构建的默认差异：

| 配置项 | dev（debug） | release |
|--------|-------------|---------|
| `opt-level` | 0（不优化，编译最快） | 3（充分优化，运行最快） |
| `debug` | true（含调试信息） | false |
| 增量编译 | 开启 | 关闭 |
| 溢出检查 | **开启**（debug 下整数溢出会 panic！第 4 章详述） | 关闭（回绕） |
| 编译耗时 | 快 | 慢（可能是 debug 的数倍） |

release 产物在 `target/release/`，这才是你应该分发给用户的版本。
性能差异通常是 **10 倍以上**——所以 benchmark 时一定要加 `--release`，
否则你会得到"Rust 很慢"的错误结论。

> ⚠️ **常见误区**：新手忘了加 `--release` 就测性能，然后惊呼"Rust 还没 Python
> 快"。**任何性能测试都必须用 release 构建。** 同理，吐槽 Rust 编译慢的人通常
> 没意识到 release 模式的优化才是慢的大头，日常开发用 debug/check 其实很快。

可以在 `Cargo.toml` 中自定义构建配置，例如：

```toml
[profile.release]
lto = true          # 链接时优化（Link Time Optimization），进一步减小体积提升性能
codegen-units = 1   # 减少代码生成单元数，优化更彻底（编译更慢）
strip = true        # 剥离符号表，减小体积
panic = "abort"     # panic 时直接终止而非展开栈（可再减小体积，但失去优雅清理）

[profile.dev]
opt-level = 1       # 给 debug 构建加一点优化：编译略慢，运行明显变快
```

一个社区常用技巧：debug 模式下给**依赖**开优化（自己的代码保持快速迭代，
依赖反正不常改）：

```toml
# debug 构建时对依赖包进行优化（开发体验与运行速度的平衡）
[profile.dev.package."*"]
opt-level = 2
```

### 3.10 Cargo 插件生态

`cargo install` 可以安装社区开发的 cargo 子命令，安装后像内置命令一样使用：

```powershell
cargo install cargo-watch      # 监听文件变化自动重新编译/测试
cargo install cargo-edit       # 提供 cargo add/remove/upgrade（新版 Cargo 已内置）
cargo install cargo-expand     # 展开宏，看宏编译后的真实代码（调试宏神器）
cargo install cargo-audit      # 扫描依赖中的已知安全漏洞
cargo install cargo-outdated   # 检查哪些依赖有新版本
cargo install flamegraph       # 生成性能火焰图
```

用法示例：`cargo watch -x check -x test`（文件保存后自动 check + test）。
这种可扩展性让 Cargo 成为一个平台——这也是 Rust 生态工具链高度统一的原因之一。

### 3.11 二进制 crate 与库 crate

Rust 中一个包（package）可以包含两种 crate 目标：

- **二进制 crate**：有 `fn main()`，编译成可执行文件。入口是 `src/main.rs`。
- **库 crate**：没有 main，提供给别人 `use` 的代码。入口是 `src/lib.rs`。

一个项目可以**两者兼有**：`src/main.rs` 和 `src/lib.rs` 同时存在时，
Cargo 会编译出一个库 + 一个使用该库的二进制。这是社区推崇的项目组织方式——
把核心逻辑放库里（方便测试和复用），main.rs 只做薄薄的一层入口。

动手验证一下。在 `hello_cargo` 项目中新建 `src/lib.rs`：

```rust
// src/lib.rs —— 库入口：把业务逻辑放在这里
/// 生成一句问候语
pub fn greet(name: &str) -> String {
    format!("你好，{}！欢迎来到 Rust 世界。", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("小明"), "你好，小明！欢迎来到 Rust 世界。");
    }
}
```

然后 `src/main.rs` 变成：

```rust
// src/main.rs —— 二进制入口：调用库中的函数
// 注意：用包名（Cargo.toml 里的 name，连字符要转下划线）来引用库
use hello_cargo::greet;

fn main() {
    println!("{}", greet("读者"));
}
```

`cargo run` 输出：

```text
你好，读者！欢迎来到 Rust 世界。
```

同时 `cargo test` 会把库里的单元测试也跑起来。这种"薄 main + 厚 lib"的结构，
等你写真实项目时会感激不已。

### 3.12 附录：工作区（Workspace）速览

当项目长大到需要拆分成多个相互依赖的 crate 时（比如"核心库 + 命令行工具 +
服务端"三个 crate），Cargo 用 **workspace（工作区）** 来统一管理：

```
my_project/               # workspace 根目录
├── Cargo.toml            # 工作区清单（只有 [workspace] 区段）
├── Cargo.lock            # 整个工作区共享一份锁文件
├── target/               # 整个工作区共享构建缓存（省磁盘、省编译时间）
├── core_lib/             # 成员一：核心库
│   ├── Cargo.toml
│   └── src/lib.rs
├── cli/                  # 成员二：命令行工具（依赖 core_lib）
│   ├── Cargo.toml
│   └── src/main.rs
└── server/               # 成员三：服务端（依赖 core_lib）
    ├── Cargo.toml
    └── src/main.rs
```

根目录的 `Cargo.toml` 内容：

```toml
[workspace]
resolver = "2"
members = [
    "core_lib",
    "cli",
    "server",
]
```

成员之间互相依赖用**路径依赖**声明（在 `cli/Cargo.toml` 中）：

```toml
[dependencies]
core_lib = { path = "../core_lib" }
```

之后在 workspace 根目录执行 `cargo build` 会构建所有成员；
`cargo run -p cli` 指定运行某个成员。大型 Rust 项目（tokio、rustc 本身）
都是这种结构。入门阶段只需知道它的存在，等项目自然长大再用。

### 本章小结

- Cargo 是 Rust 官方统一的构建工具 + 包管理器，是 Rust 生态体验的核心竞争力。
- 日常命令：`cargo new`（建项目）、`cargo check`（快速检查）、`cargo build`
  （构建）、`cargo run`（构建并运行）、`cargo test`（测试）、`cargo doc --open`
  （生成文档）、`cargo clippy`（lint）、`cargo fmt`（格式化）。
- `Cargo.toml` 是项目清单：`[package]` 元信息、`[dependencies]` 依赖、
  `[profile.*]` 构建配置。`edition` 字段是 Rust 的语法版次机制，保证语言
  进化的同时旧代码永不损坏。
- 依赖来自 crates.io，版本号遵循 SemVer；`Cargo.lock` 锁定精确版本，
  二进制项目应提交它。
- debug 构建快但慢执行，release 构建（`--release`）慢编译但快执行，
  **性能测试必须用 release**。
- 项目组织惯例：`src/main.rs` 薄入口 + `src/lib.rs` 厚逻辑，`tests/`
  放集成测试，`target/` 是构建产物（可随意删除）。

### 动手练习

1. **猜数字热身**：创建项目 `guessing_game`，添加 `rand` 依赖，写一个程序：
   生成 1~100 的随机数，然后直接打印出来（先不交互）。分别执行
   `cargo check`、`cargo build`、`cargo run`，观察三条命令输出和耗时的差异。
2. **测试练习**：给 `add` 函数补充一个会失败的测试（如 `assert_eq!(add(2, 2), 5)`），
   运行 `cargo test` 观察失败输出的格式；再把断言改正确。体会失败输出中
   `left` / `right` 的含义。
3. **release 实验**：写一个对 1 亿个数求和的程序（用 `for` 循环累加），
   分别用 `cargo run` 和 `cargo run --release` 运行并计时（PowerShell 可用
   `Measure-Command { cargo run --quiet }`），记录两者的耗时比。
4. **依赖侦查**：添加 `rand` 依赖后执行 `cargo tree`，画出 `rand` 的依赖树；
   再打开 `Cargo.lock` 找到 `rand` 条目，看它的精确版本和 `checksum` 字段。

---

## 第 4 章 基础语法

本章覆盖 Rust 的基础语法。如果你有其他语言经验，大部分内容会似曾相识——
但请特别留意那些**不一样**的地方：不可变变量、表达式导向、定长数组、
`if` 是表达式……这些差异不是语法癖好，每一个背后都有深思熟虑的设计动机。
本章会反复做的一件事，就是把这些动机挖出来给你看。

### 4.1 变量与可变性

#### 4.1.1 let 默认不可变——Rust 给你上的第一课

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);
    x = 6;   // ❌ 编译错误！
}
```

编译报错：

```text
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
  |         |
  |         help: consider making this binding mutable: `mut x`
3 |     println!("x = {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

对，你没看错：**Rust 的变量默认不可变（immutable）**。想让它可变，必须显式
加上 `mut` 关键字：

```rust
fn main() {
    let mut x = 5;      // mut = mutable，可变
    println!("x = {}", x);   // 输出：x = 5
    x = 6;              // ✅ 现在可以重新赋值
    println!("x = {}", x);   // 输出：x = 6
}
```

**为什么这样设计？** 这是来自函数式编程（ML 系语言）的传统，也是 Rust
"默认安全、显式选择危险"哲学的第一个体现：

1. **不可变是安全的默认项**。程序中大部分 bug 来自"状态在意想不到的地方被
   修改"。当默认不可变时，你阅读代码看到 `let x = 5;`，就可以确信后面 `x`
   永远是 5——心智负担大幅降低。
2. **`mut` 是显式的意图声明**。看到 `mut` 就知道"这个变量会被修改，注意跟踪
   它的变化"。代码的写作者在告诉阅读者哪里需要格外留心。
3. **编译器帮你执法**。在 Java/C++ 里你可以靠团队规范约定"尽量用 final/const"，
   但规范靠自觉；Rust 把这条规范变成了编译器强制的默认行为。

与其他语言对比：

| 语言 | 不可变的写法 | 默认行为 |
|------|-------------|---------|
| Rust | `let x = 5;`（不可变是默认，可变要加 `mut`） | 不可变 |
| Java | `final int x = 5;` | 可变 |
| C++ | `const int x = 5;` | 可变 |
| JavaScript | `const x = 5`（社区大力推广但非强制） | `var`/`let` 可变 |
| Python | 没有真正的不可变变量 | 可变 |

> ⚠️ **常见误区 1**：不可变 ≠ 常量。`let x = 5;` 中的 `x` 是"不可变的变量
> 绑定"——它的值可以在运行时计算得出（比如读用户输入），只是绑定之后不能再改。
> 常量（`const`）是另一个概念，见 4.2 节。

> ⚠️ **常见误区 2**：`mut` 修饰的是**绑定**（binding），不是值本身。
> `let mut v = vec![1, 2, 3];` 意味着你可以给 `v` 重新绑定一个新 Vec，
> 也可以修改 Vec 的内容。而 `let v = vec![1, 2, 3];` 两者都不行。
> 学到所有权和引用（第二篇）时，这一点会变得更加重要。

> 💡 **提示**：如果声明了 `mut` 却从未修改过，编译器会警告
> `variable does not need to be mutable`。反过来，声明了不可变变量却从未读取，
> 会得到 `unused variable` 警告。Rust 编译器像一位尽责的代码评审员，
> 建议在开发期认真对待每一条 warning——用 `cargo clippy` 会得到更多。

#### 4.1.2 Shadowing（遮蔽）——同一名字的"重生"

Rust 允许用同一个变量名再次 `let`，新绑定会**遮蔽（shadow）**旧绑定：

```rust
fn main() {
    let x = 5;              // 第一个 x

    let x = x + 1;          // 第二个 x：用旧的 x 计算，然后遮蔽它

    {
        let x = x * 2;      // 第三个 x：只在这个代码块内有效
        println!("内层 x = {}", x);   // 输出：内层 x = 12
    }

    println!("外层 x = {}", x);       // 输出：外层 x = 6
}
```

遮蔽与 `mut` 有本质区别——**每次 `let` 都创建一个全新的变量**（只是名字相同，
旧变量被"盖住"了），所以**类型也可以改变**：

```rust
fn main() {
    let spaces = "   ";            // &str 类型：一个字符串
    let spaces = spaces.len();     // usize 类型：一个数字。合法！这是新变量
    println!("空格数 = {}", spaces);  // 输出：空格数 = 3
}
```

如果用 `mut` 改写：

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();   // ❌ 编译错误：expected `&str`, found `usize`
}
```

`mut` 变量重新赋值时类型必须一致；shadowing 则是新变量，类型随意。

**什么时候用 shadowing？** 典型场景是"对同一个概念做渐进转换"：

```rust
fn main() {
    // 从用户输入到可用数据的转换链，每一步都是"同一个东西"的进化
    let input = "  42  ";               // 原始字符串
    let input = input.trim();           // 去掉首尾空白，还是字符串
    let input: i32 = input.parse().expect("不是数字");  // 解析为整数
    let input = input * 2;              // 参与计算

    println!("结果 = {}", input);       // 输出：结果 = 84
}
```

在其他语言里你可能会写出 `input_str`、`trimmed_str`、`parsed_num` 这样
冗长又难看懂的中间变量名。Shadowing 让"同一概念的连续变换"用同一个名字表达，
**每个阶段还都是不可变的**，兼顾了可读性和安全性。

> 💡 **对比 Java**：Java 里 `var s = " 42 "; s = s.trim();` 后你又想把它变成
> int，就不得不引入新变量名。Rust 的 shadowing 是"类型可以进化的变量名"。

### 4.2 常量与静态变量

#### 常量 const

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.141592653589793;

fn main() {
    println!("最高分 = {}", MAX_POINTS);
    println!("圆周率 = {}", PI);
}
```

常量与不可变变量的区别：

| 特性 | `let`（不可变） | `const` |
|------|----------------|---------|
| 类型标注 | 可省略（自动推导） | **必须显式标注** |
| 赋值时机 | 可以是运行时计算的值 | **只能是编译期可确定的常量表达式** |
| 作用域 | 任何作用域 | 任何作用域（包括全局） |
| 可变性 | 可加 `mut` | 永远不可变（`mut const` 是语法错误） |
| 内存表现 | 有栈/堆上的存储位置 | 编译期内联到每个使用点，无固定地址 |

```rust
fn main() {
    let x = std::env::args().count();  // ✅ 运行时才知道的值，可以绑定给 let
    // const X: usize = std::env::args().count();  // ❌ 编译错误：const 不能调用运行时函数

    println!("参数个数 = {}", x);
}
```

命名约定：常量用**全大写蛇形命名**（`SCREAMING_SNAKE_CASE`），编译器会对
违反约定的命名给出警告。

> 💡 **数字可读性技巧**：Rust 允许在数字字面量中插入下划线 `_` 增强可读性，
> 如 `100_000`、`1_000_000_000`，对数值没有任何影响。

#### 静态变量 static

```rust
static LANGUAGE: &str = "Rust";
static mut COUNTER: u32 = 0;   // 可变静态变量——危险！

fn main() {
    println!("语言 = {}", LANGUAGE);

    // 读写 static mut 必须在 unsafe 块中（第二篇会讲 unsafe）
    unsafe {
        COUNTER += 1;
        println!("计数 = {}", COUNTER);   // 输出：计数 = 1
    }
}
```

`static` 与 `const` 的区别：`static` 变量在内存中有**固定地址**（存储在程序的
静态区），所有引用指向同一实例；`const` 是编译期内联替换，每个使用点独立。

`static mut` 是全局可变状态——多线程下是数据竞争的温床，因此 Rust 要求所有
对它的访问必须放在 `unsafe` 块中，由程序员自己担保安全。**入门阶段请把
`static mut` 当成"不要碰"的东西**；需要全局可变状态时，正确做法是
`Mutex` / `RwLock` / `AtomicU32` 等并发原语（后续章节会讲）。

> 💡 **经验法则**：编译期已知的不变值用 `const`；需要固定内存地址的只读全局
> 数据（如大查找表）用 `static`；`static mut` 尽量避免。

### 4.3 标量类型（一）：整数与溢出

Rust 是**静态强类型**语言：每个值都有编译期确定的类型，类型之间不会隐式转换。
标量类型（scalar）表示单个值，共四类：整数、浮点、布尔、字符。

#### 整数类型

| 长度 | 有符号 | 无符号 | 取值范围（以 8 位为例） |
|------|--------|--------|------------------------|
| 8 位 | `i8` | `u8` | i8: -128 ~ 127；u8: 0 ~ 255 |
| 16 位 | `i16` | `u16` | |
| 32 位 | `i32` | `u32` | i32: 约 ±21 亿 |
| 64 位 | `i64` | `u64` | |
| 128 位 | `i128` | `u128` | |
| 架构相关 | `isize` | `usize` | 64 位系统上是 64 位 |

**默认类型是 `i32`**——类型推导无法确定时，整数会被推导为 `i32`。

选型建议：

- **一般计算用 `i32`**：即使数值非负，`i32` 也通常是正确选择（运算快、
  避免无符号减法的坑）；
- **索引、长度、内存大小用 `usize`**：这是 Rust 的强约定，数组/Vec 的
  索引和 `len()` 都是 `usize`；
- 与 C 库交互、网络协议、文件格式等需要**精确位宽**时，用 `i32`/`u64` 等
  明确宽度的类型；
- **不要**因为"数值不可能是负的"就选无符号类型——见下面减法的例子。

#### 整数溢出：Rust 最著名的一个设计决策

```rust
fn main() {
    let mut x: u8 = 255;
    println!("x = {}", x);   // 输出：x = 255
    x += 1;                  // u8 最大就是 255，再加 1 会怎样？
    println!("x = {}", x);
}
```

答案取决于构建模式——这正是 3.9 节提到的溢出检查配置：

- **debug 构建**：运行时检测到溢出，程序 **panic**（崩溃）：

  ```text
  thread 'main' panicked at src/main.rs:4:5:
  attempt to add with overflow
  ```

- **release 构建**：静默**回绕**（wrapping），255 + 1 变成 0（二进制补码回绕）。

**为什么这样设计？** 这是一个经典的"性能 vs 安全"权衡，Rust 的答案是"我全都要，
但分场景"：

- C/C++：溢出是未定义行为（有符号）或静默回绕（无符号）——编译器可以基于
  "溢出不会发生"的假设做激进优化，溢出漏洞（如整数溢出绕过长度检查）是
  安全漏洞的重要来源；
- Rust debug 模式：**宁可 crash 也不静默出错**。开发期立刻暴露 bug；
- Rust release 模式：为了极致性能关掉运行时检查，但提供了**显式的溢出语义**：

```rust
fn main() {
    let x: u8 = 255;

    // 方式一：wrapping_* —— 明确要回绕语义
    println!("{}", x.wrapping_add(1));        // 输出：0

    // 方式二：checked_* —— 溢出时返回 None（强制你处理，推荐！）
    match x.checked_add(1) {
        Some(v) => println!("结果 = {}", v),
        None => println!("溢出了！"),          // 会走这个分支
    }

    // 方式三：overflowing_* —— 返回 (结果, 是否溢出)
    let (v, did_overflow) = x.overflowing_add(1);
    println!("结果 = {}，溢出 = {}", v, did_overflow);  // 输出：结果 = 0，溢出 = true

    // 方式四：saturating_* —— 饱和：溢出时停留在最大值
    println!("{}", x.saturating_add(1));      // 输出：255
}
```

四种显式语义覆盖了所有实际需求，把"溢出"从一个隐患变成了类型系统的一部分
（`checked_add` 返回 `Option<u8>`，编译器强迫你处理 `None` 的情况）。
这是 Rust 设计哲学的绝佳缩影：**危险的操作不是被禁止，而是被显式化。**

> ⚠️ **常见误区（无符号减法陷阱）**：遍历或倒序计数时，下面这段代码会在
> debug 下 panic、release 下变成巨大正数导致死循环或越界：
>
> ```rust
> fn main() {
>     // 错误示范：u32 减到 0 再减 1 就溢出
>     let mut i: u32 = 3;
>     while i >= 0 {          // ⚠️ u32 永远 >= 0，release 下死循环！
>         println!("{}", i);
>         i = i.wrapping_sub(1);   // debug 下若用 i -= 1 则 panic
>         if i == u32::MAX { break; }   // 勉强补救，但很难看
>     }
> }
> ```
>
> 正确做法：用有符号类型，或者更好——用迭代器 `for i in (0..=3).rev()`。
> **这是不推荐"非负就用无符号"的最有力理由。**

#### 整数字面量

```rust
fn main() {
    let decimal = 98_222;        // 十进制
    let hex = 0xff;              // 十六进制
    let octal = 0o77;            // 八进制
    let binary = 0b1111_0000;    // 二进制
    let byte = b'A';             // 字节字面量（u8）：65

    // 类型后缀写法
    let x = 57u8;                // 等价于 let x: u8 = 57;

    println!("{} {} {} {} {} {}", decimal, hex, octal, binary, byte, x);
    // 输出：98222 255 63 240 65 57
}
```

### 4.4 标量类型（二）：浮点、布尔、字符

#### 浮点数

```rust
fn main() {
    let x = 2.0;         // f64：默认浮点类型（双精度）
    let y: f32 = 3.0;    // f32：单精度，需显式标注

    // 基本运算
    println!("和 = {}", x + y as f64);        // as 是显式类型转换
    println!("商 = {}", 5.0 / 3.0);           // 输出：1.6666666666666667

    // 特殊值（IEEE 754 标准）
    let inf = f64::INFINITY;
    let nan = f64::NAN;

    println!("1.0 / 0.0 的极限 = {}", inf);    // 输出：1.0 / 0.0 的极限 = inf
    println!("0.1 + 0.2 = {}", 0.1 + 0.2);    // 输出：0.30000000000000004
    println!("NaN == NaN ? {}", nan == nan);  // 输出：false！
}
```

注意三点：

1. **默认 `f64`**——与 C 系语言一致，现代 CPU 上 f64 不比 f32 慢，精度还更高。
2. **浮点精度问题不是 Rust 的锅**：`0.1 + 0.2 ≠ 0.3` 是 IEEE 754 二进制浮点的
   固有特性，所有语言一样（Python 也一样）。需要精确小数运算（金融）请用
   `rust_decimal` 等 crate。
3. **NaN 不等于自身**，所以 `f64` 不实现完整的相等性（学到 trait 时会理解
   `PartialEq` vs `Eq` 的区分，这是 Rust 类型系统诚实的又一体现）。

#### 布尔类型

```rust
fn main() {
    let t = true;
    let f: bool = false;   // 显式标注（通常不需要）

    println!("{} {}", t, f);   // 输出：true false
}
```

`bool` 只有两个值：`true` 和 `false`。

> ⚠️ **常见误区（C/C++/JavaScript 用户尤其注意）**：Rust 的 `if` 条件**必须
> 是 `bool`**，不存在"真值（truthy）"隐式转换。`if 1 { }`、`if "hello" { }`
> 都是编译错误。要判断数字非零就写 `if n != 0`。这种严格消除了 JS 中
> `0 == ""`、`null == undefined` 之类的诡异等价关系。

#### 字符类型 char

```rust
fn main() {
    let c = 'z';          // 单引号！双引号是字符串
    let z: char = 'ℤ';
    let heart = '❤';
    let cat = '🐱';       // Emoji 也是合法 char

    println!("{} {} {} {}", c, z, heart, cat);   // 输出：z ℤ ❤ 🐱

    // char 是 4 字节的 Unicode 标量值
    println!("char 占 {} 字节", std::mem::size_of::<char>());   // 输出：char 占 4 字节
}
```

与 C/C++ 的 `char`（1 字节，只能表示 ASCII）和 Java 的 `char`（2 字节，UTF-16，
无法表示部分 Emoji）都不同，**Rust 的 `char` 是 4 字节，表示一个完整的 Unicode
标量值**——从设计之初就面向全球化文本。

> ⚠️ **常见误区**：`'a'` 是 char，`"a"` 是字符串（`&str`），两者类型不同！
> 单双引号不能混用。另外，一个 char 不等于"用户看到的一个字符"——像 `é`
> 可能是两个 Unicode 标量组合而成的。处理用户感知的"字素簇（grapheme cluster）"
> 需要 `unicode_segmentation` crate。

### 4.5 类型标注与类型推导

Rust 有强大的类型推导（基于 Hindley-Milner 风格的算法，类似 Haskell/OCaml），
大部分时候不需要写类型：

```rust
fn main() {
    let x = 5;                    // 推导为 i32
    let y = 2.0;                  // 推导为 f64
    let v = vec![1, 2, 3];        // 推导为 Vec<i32>

    // 但有时推导不出，编译器会要求你标注：
    let guess = "42".parse().expect("解析失败");
    // ❌ 编译错误：type annotations needed
    // "42".parse() 能解析成很多类型（i32? u64? f64?），编译器不知道你要哪个

    println!("{} {} {:?}", x, y, v);
}
```

修复方式——标注类型或用"turbofish"语法：

```rust
fn main() {
    // 方式一：let 处标注
    let guess: u32 = "42".parse().expect("解析失败");

    // 方式二：turbofish ::<> 语法，直接在方法上指定
    let guess = "42".parse::<u32>().expect("解析失败");

    println!("guess = {}", guess);   // 输出：guess = 42
}
```

> 💡 **RustRover 技巧**：开启 Inlay Hints 后，IDE 会把推导出的类型以灰色小字
> 显示在变量后面（`let x /*: i32*/ = 5;`），这部分提示不是真实代码，
> 但对理解程序极有帮助。`Ctrl + Shift + P` 可随时查看任意表达式的类型。

### 4.6 复合类型：元组

元组（tuple）把**多个不同类型的值**组合成一个复合值，长度固定：

```rust
fn main() {
    // 创建：圆括号，类型可以各不相同
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 访问方式一：模式匹配解构（推荐，最常用）
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);   // 输出：x = 500, y = 6.4, z = 1

    // 访问方式二：点号 + 下标
    println!("第一个元素 = {}", tup.0);   // 输出：第一个元素 = 500
    println!("第二个元素 = {}", tup.1);   // 输出：第二个元素 = 6.4

    // 实战例子：函数返回多个值（Rust 没有"多返回值"语法，返回元组即可）
    let s = String::from("hello");
    let (s2, len) = calculate_length(s);
    println!("'{}' 的长度是 {}", s2, len);   // 输出：'hello' 的长度是 5
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)   // 把字符串和它的长度一起返回
}
```

**为什么用元组返回多值？** 对比其他语言：C++ 用 `std::pair`/`std::tuple` 或
输出参数（`&` 引用当返回值用，很难读）；Java 不得不为两个值专门定义一个类，
或者滥用数组/Map；Go 有真正的多返回值（这倒是 Go 做得好的地方）。Rust 的元组 +
解构是轻量优雅的方案。

**单元类型 `()`**：空元组 `()` 是一个特殊类型，叫**单元类型（unit）**，它只有
一个值就是 `()` 本身。作用类似于其他语言的 `void`——但比 `void` 强的地方是
它是一个**真正的类型**，可以作为泛型参数。函数没有显式返回值时，实际返回的就是 `()`。

```rust
fn main() {
    let result = println!("hello");   // println! 返回 ()
    println!("{:?}", result);         // 输出：()
}
```

### 4.7 复合类型：数组——定长！

```rust
fn main() {
    // 数组：所有元素类型相同，长度编译期固定
    let a = [1, 2, 3, 4, 5];               // 推导为 [i32; 5]
    let months: [&str; 3] = ["一月", "二月", "三月"];   // 显式标注 [类型; 长度]

    // 初始化相同值的快捷写法：[初始值; 长度]
    let zeros = [0; 10];                   // 10 个 0

    // 索引访问（从 0 开始）
    println!("a[0] = {}", a[0]);           // 输出：a[0] = 1
    println!("zeros 长度 = {}", zeros.len());  // 输出：zeros 长度 = 10

    // 数组可以整体解构
    let [first, second, ..] = a;   // ".." 忽略剩余元素
    println!("{} {}", first, second);      // 输出：1 2
}
```

**数组的长度是类型的一部分**：`[i32; 3]` 和 `[i32; 4]` 是**两个不同的类型**。
这在其他语言里很少见（C 的数组长度也算类型的一部分，但使用时经常退化为指针；
Java/Python 的数组/列表长度完全是运行时概念）。

**越界访问：编译期能挡就挡，运行时必然 panic**：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    // 情况一：索引是编译期已知的常量 —— 直接编译错误
    // let x = a[10];   // ❌ index out of bounds: the len is 5 but the index is 10

    // 情况二：索引是运行时才知道的值 —— panic（而不是 C 那样的越界读！）
    let index = 10;
    let x = a[index];   // 💥 运行时 panic: index out of bounds: the len is 5 but the index is 10
    println!("{}", x);
}
```

**这是 Rust 内存安全最直观的体现。** 对比 C/C++：越界读返回垃圾值、越界写
破坏相邻内存——缓冲区溢出漏洞（Heartbleed 等无数 CVE）的根源。Rust 在运行时
做边界检查，越界立即 panic（可控崩溃），绝不会静默读到脏数据。

> 💡 **性能顾虑？** 边界检查确实有一点点运行时成本，但：编译器常能在优化时
> 消除能证明安全的检查；用迭代器遍历（`for x in &a`）天然无需检查。极少数
> 性能关键点可以用 `get_unchecked`（unsafe），但入门阶段请忘记它的存在。

#### 数组 vs Vec：什么时候用哪个？

```rust
fn main() {
    // 数组：长度固定，通常分配在栈上，长度是类型的一部分
    let arr: [i32; 3] = [1, 2, 3];

    // Vec（向量）：长度可变的"动态数组"，数据在堆上
    let mut vec: Vec<i32> = vec![1, 2, 3];
    vec.push(4);   // Vec 可以增长，数组不行

    println!("数组长度（恒定）= {}", arr.len());   // 输出：3
    println!("Vec 长度（可变）= {}", vec.len());   // 输出：4
}
```

经验法则：**绝大多数情况用 `Vec`**（相当于 C++ 的 `vector`、Java 的 `ArrayList`、
Python 的 `list`）；只有长度确实编译期已知且不变时（如 IPv4 地址 4 字节、
棋盘 8×8、一周的 7 天）用数组。`Vec` 的详细用法在后续章节展开，这里先建立概念。

### 4.8 函数

#### 4.8.1 定义与参数

```rust
fn main() {
    print_sum(3, 5);          // 输出：3 + 5 = 8
    let r = square(7);
    println!("7 的平方 = {}", r);   // 输出：7 的平方 = 49
}

// fn 关键字 + 蛇形命名（snake_case）
// 参数必须标注类型——Rust 不做跨函数的类型推导
fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

// "-> 类型" 声明返回值类型
fn square(x: i32) -> i32 {
    x * x   // 注意：没有 return 关键字，没有分号！
}
```

两个硬性规定：

1. **参数类型必须显式标注**。这与 Haskell 等可以推导函数签名的语言不同，
   是 Rust 的刻意选择：函数签名是"文档"，强制标注让代码不依赖 IDE 也可读，
   也让编译错误定位更准确（错误在函数边界就被拦住，不会传播到很远的调用点）。
2. **返回值的最后一行是表达式**（见 4.8.2），也可以显式用 `return` 提前返回：

```rust
fn abs(x: i32) -> i32 {
    if x < 0 {
        return -x;   // 提前返回必须用 return（有分号）
    }
    x                // 尾部表达式作为返回值（更地道）
}
```

#### 4.8.2 表达式 vs 语句——Rust 的灵魂特性

**这是本章最重要的概念，没有之一。** Rust 是一门**表达式导向
（expression-oriented）**的语言，这是它从 ML 系函数式语言继承的血统，
也是许多"奇怪"语法现象的总根源。

定义：

- **语句（Statement）**：执行一个动作，**不返回值**（或者说返回 `()`）。
- **表达式（Expression）**：求值并**产生一个值**。

```rust
fn main() {
    let x = 6;
    // ^^^^^^^^^ 这是一个语句（let 语句），它不返回值
}
```

在 Rust 里，`let` 是语句——所以你不能像 C 那样链式赋值：

```rust
fn main() {
    // let x = (let y = 6);   // ❌ 编译错误：let 是语句，没有值
    // C/C++/Go 里 x = y = 6 是合法的（赋值本身是表达式），Rust 故意禁止：
    // 它消除了 if (x = 5) 这种把赋值误写成比较的经典 bug！
}
```

**而几乎所有其他东西都是表达式**：算术运算、函数调用、代码块、`if`、`match`、
`loop`……

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1        // 代码块的最后一个是表达式 → 整个块 { } 的值是 4
    };
    println!("y = {}", y);   // 输出：y = 4
}
```

**分号的意义**——这是理解 Rust 报错的关键：

- 表达式**加上分号**就变成语句，值被丢弃（返回 `()`）；
- 表达式**不带分号**则保留值。

```rust
fn five() -> i32 {
    5          // ✅ 表达式作为返回值
}

// 下面这个函数如果取消注释，整个程序将无法编译：
//
// fn five_wrong() -> i32 {
//     5;      // ❌ 编译错误！加了分号变成语句，函数实际返回 ()
//             // error[E0308]: mismatched types: expected `i32`, found `()`
// }

fn main() {
    println!("{}", five());   // 输出：5
}
```

新手最常见的错误之一就是在函数末尾习惯性地加上分号（其他语言的习惯），
然后对着 `mismatched types: expected i32, found ()` 报错发呆。**记住：
返回值那一行，别加分号。**

表达式导向的设计为什么好？因为它让代码更**组合化、声明式**：

```rust
fn main() {
    let temperature = 25;

    // if 是表达式，可以直接赋值（后面控制流详解）
    let advice = if temperature > 30 {
        "太热了，开空调"
    } else if temperature > 20 {
        "温度正好"
    } else {
        "有点冷，穿外套"
    };

    println!("{}", advice);   // 输出：温度正好
}
```

对比 Java/C++ 的三元运算符 `cond ? a : b`——Rust 没有三元运算符，因为**根本不需要**：
`if` 本身就是表达式，直接胜任，而且支持任意多条分支、任意复杂的块。

### 4.9 注释

````rust
fn main() {
    // 行注释：最常用，直到行尾

    /*
     * 块注释：可以跨行（风格上不如连续行注释常见）
     */

    let lucky = 7; // 也可以放在代码行尾

    println!("幸运数字 = {}", lucky);
}

/// 文档注释（doc comment）：为**后面的**项编写文档，支持 Markdown。
/// 用 `cargo doc --open` 会渲染成漂亮的 HTML 文档页面。
///
/// # 示例
///
/// ```
/// let r = add(2, 3);
/// assert_eq!(r, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! 模块/ crate 级文档注释：描述**包含它的**整体（通常放在 lib.rs 或 main.rs 开头）
````

**文档注释是 Rust 的一等公民**，两点特别值得说：

1. **文档里的代码示例会被 `cargo test` 当作真实测试执行**（doc-test）！
   上面 `///` 注释中用三反引号包起来的代码，在库项目中执行 `cargo test` 时
   会被自动编译并运行，`assert_eq!` 失败则测试失败。文档永远不会和代码脱节——
   这个设计极为精妙，对比其他语言文档里的示例代码常年腐烂失效。
   （注意：doc-test 只对库 crate 生效，`src/main.rs` 里的文档注释不会被测试。）
2. crates.io 上所有库的在线文档（docs.rs）都是从 `///` 注释自动生成的。
   写好文档注释 = 写好了库的官网文档。

### 4.10 控制流

#### 4.10.1 if：是表达式，不是语句

```rust
fn main() {
    let number = 6;

    // 基本用法（注意：条件不用加括号！加了编译器会警告）
    if number % 4 == 0 {
        println!("能被 4 整除");
    } else if number % 3 == 0 {
        println!("能被 3 整除");   // 输出这个
    } else if number % 2 == 0 {
        println!("能被 2 整除");
    } else {
        println!("不能被 2、3、4 整除");
    }
}
```

三个要点：

1. **条件不加括号**：Rust 风格是 `if x > 0 { }`，而不是 `if (x > 0) { }`。
   原因是——
2. **花括号必须写**：即使分支只有一行也不能省略 `{}`。既然块是必须的，
   条件的括号就成了多余的噪音。这条规则消灭了 C 系语言的经典事故
   （想想 Apple 的 "goto fail" SSL 漏洞，就是省略花括号导致的）。
3. **条件必须是 bool**：`if number { }` 是编译错误，Rust 没有真值转换。

**if 作为表达式**：

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);   // 输出：number = 5

    // 分支的值类型必须一致：
    // let x = if condition { 5 } else { "six" };  // ❌ i32 和 &str 类型不匹配
}
```

这取代了其他语言的三元运算符 `? :`。而且因为分支是块，可以包含多条语句：

```rust
fn main() {
    let score = 85;

    let grade = if score >= 90 {
        'A'
    } else {
        // 分支里可以做任意复杂的计算
        let adjusted = score + 5;   // 假设有加分政策
        if adjusted >= 90 { 'A' } else { 'B' }   // 嵌套的 if 表达式
    };

    println!("成绩等级 = {}", grade);   // 输出：成绩等级 = A
}
```

> 💡 **设计动机**：为什么 Rust 坚持把 `if` 做成表达式？表达式可以组合、
> 可以出现在任何需要值的地方，让代码更紧凑、更声明式。函数式语言
> （Haskell、OCaml、Lisp）几十年前就验证了这个设计的价值，Rust 把它带入了
> 系统编程主流。

#### 4.10.2 loop：无限循环，但它能返回值

Rust 有三种循环：`loop`、`while`、`for`。先看 `loop`：

```rust
fn main() {
    // loop 是无条件无限循环，只能靠 break 退出
    let mut count = 0;

    loop {
        count += 1;
        if count == 3 {
            println!("数到 3 了，退出");
            break;
        }
        println!("count = {}", count);
    }
}
```

输出：

```text
count = 1
count = 2
数到 3 了，退出
```

**为什么有 loop 还要有 while？** 或者说，为什么 Rust 不像大多数语言那样只提供
`while (true)`？因为 `loop` 向编译器明确表达了"这是真无限循环"的意图，
编译器对 `loop { }` 的控制流分析比 `while true { }` 更精确（后者理论上
条件可能被改），这影响变量的初始化分析等。

**loop 的杀手级特性：可以带值 break，整个循环是表达式！**

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;   // break 带值：整个 loop 表达式的值是 20
        }
    };

    println!("result = {}", result);   // 输出：result = 20
}
```

这个特性在"重试直到成功"的场景极其顺手：

```rust
fn try_connect() -> Result<(), String> {
    // 模拟：伪代码，实际可能连接失败
    Ok(())
}

fn main() {
    let mut attempts = 0;

    // 不断重试，成功后把值带出循环
    let connection = loop {
        attempts += 1;
        match try_connect() {
            Ok(conn) => break conn,            // 成功：把连接带出循环
            Err(e) if attempts < 3 => {
                println!("第 {} 次尝试失败：{}，重试中...", attempts, e);
            }
            Err(e) => {
                println!("重试 {} 次后放弃：{}", attempts, e);
                return;   // 直接结束 main
            }
        }
    };

    println!("连接成功：{:?}", connection);   // 输出：连接成功：()
}
```

#### 4.10.3 循环标签：嵌套循环的精确控制

嵌套循环中，`break` 和 `continue` 默认只作用于**最内层**循环。Rust 用
**循环标签（loop label）**精确指定作用对象：

```rust
fn main() {
    let mut count = 0;

    'outer: loop {          // 标签语法：'名字，注意开头的单引号
        println!("进入外层循环，count = {}", count);
        let mut remaining = 10;

        loop {
            println!("  内层 remaining = {}", remaining);
            if remaining == 9 {
                break;               // 默认：只跳出内层循环
            }
            if count == 2 {
                break 'outer;        // 带标签：直接跳出外层循环！
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("循环结束，count = {}", count);
}
```

输出：

```text
进入外层循环，count = 0
  内层 remaining = 10
  内层 remaining = 9
进入外层循环，count = 1
  内层 remaining = 10
  内层 remaining = 9
进入外层循环，count = 2
  内层 remaining = 10
循环结束，count = 2
```

对比 Java：Java 也有带标签的 break，语法几乎一样（这大概是 Rust 和 Java
少数共享的特性之一）。对比 C++/Python：没有标签，只能靠标志变量或 goto，
代码丑陋且易错。

标签同样适用于 `while` 和 `for`，以及 `continue 'outer`。

#### 4.10.4 while：条件循环

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("发射！🚀");
}
```

输出：

```text
3!
2!
1!
发射！🚀
```

`while` 与其他语言基本一致，没有`do-while`（用 `loop { ...; if !cond { break; } }`
替代）。条件同样必须是 `bool`、不加分号……哦不，是条件不加括号。

> 💡 **性能提示**：用 `while` 配合索引遍历数组会引入边界检查，
> 且编译器往往无法优化掉。**Rust 中遍历集合的惯用法永远是 `for` 迭代器**，
> 既安全又通常更快。

#### 4.10.5 for：迭代器驱动的循环

Rust 的 `for` 是**基于迭代器的**（类似 Python 的 for-each），而不是 C 风格
的 `for (i = 0; i < n; i++)`——Rust 里那种写法**根本不存在**。

```rust
fn main() {
    // 遍历数组（迭代器方式，无边界检查，安全又高效）
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("值 = {}", element);
    }

    // 遍历范围（Range）：.. 是左闭右开
    for i in 0..5 {
        print!("{} ", i);          // 输出：0 1 2 3 4
    }
    println!();

    // ..= 是闭区间
    for i in 0..=5 {
        print!("{} ", i);          // 输出：0 1 2 3 4 5
    }
    println!();

    // .rev() 反转：倒序的正确姿势（回想 4.3 的无符号减法陷阱）
    for i in (1..=3).rev() {
        print!("{} ", i);          // 输出：3 2 1
    }
    println!();

    // 带下标遍历：.iter().enumerate()
    let names = ["Alice", "Bob", "Carol"];
    for (index, name) in names.iter().enumerate() {
        println!("第 {} 位：{}", index + 1, name);
    }
}
```

输出：

```text
值 = 10
值 = 20
值 = 30
值 = 40
值 = 50
0 1 2 3 4
0 1 2 3 4 5
3 2 1
第 1 位：Alice
第 2 位：Bob
第 3 位：Carol
```

**为什么废弃 C 风格 for？** 因为它太容易出错：差一错误（off-by-one）、
无符号溢出（4.3 节的例子）、手动索引引入越界风险。迭代器把"怎么遍历"
封装起来，让"越界"这类错误在类型层面就不存在。而且迭代器是零成本抽象的
典范——`for x in 0..n` 编译后与 C 的 `for (i = 0; i < n; i++)` 完全相同。

迭代器的威力远不止于此（链式 map/filter/fold 等将在后续章节展开），
现在先记住：**在 Rust 里，遍历用 for + 迭代器是默认且唯一推荐的方式。**

### 4.11 综合实战：斐波那契与温度转换

把本章知识串起来，写两个经典练习程序。

**程序一：斐波那契数列（多种写法对比）**

```rust
fn main() {
    // 写法一：可变变量 + for 循环（命令式风格，初学者友好）
    let n = 10;
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    print!("前 {} 项斐波那契：", n);
    for _ in 0..n {              // _ 表示"这个变量我不打算用"
        print!("{} ", a);
        let next = a + b;
        a = b;                   // 可变变量更新状态
        b = next;
    }
    println!();
    // 输出：前 10 项斐波那契：0 1 1 2 3 5 8 13 21 34

    // 写法二：shadowing + 元组交换（更 Rust 风格）
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    print!("再来一遍：");
    for _ in 0..n {
        print!("{} ", a);
        let (a_new, b_new) = (b, a + b);   // 元组同时求值，避免覆盖问题
        a = a_new;
        b = b_new;
    }
    println!();

    // 写法三：递归函数（见下方 fib 定义），体验"if 是表达式"的写法
    println!("fib(10) = {}", fib(10));   // 输出：fib(10) = 55
}

// 递归版（函数 + if 表达式）：注意 u128 防溢出，斐波那契增长极快
fn fib(n: u32) -> u128 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)   // 尾部表达式，无分号
    }
}
```

> 💡 注意递归版 fib 是指数复杂度 O(2ⁿ)，`fib(40)` 就要算好几秒——
> 这正好留给你做本章练习（记忆化优化）。

**程序二：摄氏/华氏温度转换（用 if 表达式处理输入）**

```rust
use std::io;

fn main() {
    println!("=== 温度转换器 ===");
    println!("请输入温度和单位，如：36.5C 或 98.6F");

    // 读取一行用户输入
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    let input = input.trim();   // shadowing：去掉换行符

    // 取最后一个字符判断单位，其余部分解析为数字
    let (value_str, unit) = input.split_at(input.len() - 1);
    let value: f64 = value_str.parse().expect("温度必须是数字");

    // if 是表达式：根据单位计算摄氏温度
    let celsius = if unit == "C" || unit == "c" {
        value
    } else if unit == "F" || unit == "f" {
        (value - 32.0) / 1.8
    } else {
        panic!("未知单位：{}（只支持 C 或 F）", unit);
    };

    let fahrenheit = celsius * 1.8 + 32.0;
    println!("{:.1}°C = {:.1}°F", celsius, fahrenheit);
}
```

运行示例：

```text
=== 温度转换器 ===
请输入温度和单位，如：36.5C 或 98.6F
98.6F
37.0°C = 98.6°F
```

注意 `panic!` 宏——它是"不可恢复错误"的处理方式，程序立即终止并打印消息。
真实的程序应该更优雅地处理错误（`Result`，第二篇的核心主题），
`panic!` 留给"这不应该发生"的契约违反场景。

### 4.12 附：println! 格式化输出速查

本章的示例已经用了很多次 `println!`，这里系统地总结它的格式化语法——
它几乎是你入门期唯一需要的输出工具。

```rust
fn main() {
    let name = "小明";
    let age = 18;
    let pi = 3.14159265;

    // 1. 位置参数：{} 按顺序填充
    println!("{} 今年 {} 岁", name, age);          // 输出：小明 今年 18 岁

    // 2. 命名参数：更直观，参数多时不乱
    println!("{name} 明年 {next} 岁", name = name, next = age + 1);
    // 输出：小明 明年 19 岁

    // 3. 内联捕获（Rust 2021+）：直接用作用域里的变量名，最简洁
    println!("{name} 后年 {} 岁", age + 2);        // 输出：小明 后年 20 岁

    // 4. 数字格式化
    println!("圆周率两位小数：{:.2}", pi);          // 输出：圆周率两位小数：3.14
    println!("八进制 {:o}，十六进制 {:#x}，二进制 {:b}", 255, 255, 5);
    // 输出：八进制 377，十六进制 0xff，二进制 101
    println!("科学计数法：{:e}", 12345678);         // 输出：科学计数法：1.2345678e7

    // 5. 宽度与对齐
    println!("右对齐 |{:>8}|", "hi");              // 输出：右对齐 |      hi|
    println!("左对齐 |{:<8}|", "hi");              // 输出：左对齐 |hi      |
    println!("居中   |{:^8}|", "hi");              // 输出：居中   |   hi   |
    println!("补零   |{:08}|", 42);                // 输出：补零   |00000042|
    println!("带符号 |{:+}|", 42);                 // 输出：带符号 |+42|

    // 6. 调试输出 {:?}：打印任何实现了 Debug 的类型（开发期最常用）
    let v = vec![1, 2, 3];
    let tup = ("hello", 42, true);
    println!("{:?}", v);                           // 输出：[1, 2, 3]
    println!("{:?}", tup);                         // 输出：("hello", 42, true)
    println!("{:#?}", v);                          // 美化版调试输出（每项一行）：
                                                   // [
                                                   //     1,
                                                   //     2,
                                                   //     3,
                                                   // ]

    // 7. 转义：{{ 和 }} 输出字面花括号
    println!("字面量花括号 {{ }}");                 // 输出：字面量花括号 { }
}
```

**`{}` 与 `{:?}` 的区别是新手最容易困惑的点**，一句话说清：

- `{}` 使用 **Display** trait——"给人看的"输出，类型需要自己实现 Display
  （基本类型都内置了）；
- `{:?}` 使用 **Debug** trait——"给程序员看的"输出，几乎所有标准类型都有，
  自定义类型加一行 `#[derive(Debug)]` 即可自动获得。

所以打印 `Vec`、元组、结构体时用 `{:?}`，打印数字字符串时用 `{}`。
试图用 `{}` 打印一个没有 Display 的类型会得到编译错误，编译器还会贴心地
提示你改用 `{:?}`。

另外三个同族宏，一并认识：

| 宏 | 作用 |
|----|------|
| `print!` | 不换行打印 |
| `eprint!` / `eprintln!` | 打印到**标准错误**（stderr），用于错误信息，不污染 stdout 的管道输出 |
| `format!` | 不打印，返回一个 `String`——`let s = format!("{} 岁", 18);` |
| `dbg!` | 调试神器：`dbg!(x)` 打印**文件名、行号、表达式和值**，如 `[src/main.rs:3] x = 5` |

### 4.13 附：运算符速查与类型转换

Rust 的运算符与其他 C 系语言大同小异，这里只列出**有差异或值得注意**的部分：

| 类别 | 运算符 | 说明 |
|------|--------|------|
| 算术 | `+ - * / %` | 注意 `/` 在整数间是整除：`5 / 2 == 2`（与 C/Java 一致，Python 的 `/` 是浮点除） |
| 位运算 | `& \| ^ << >> !` | `!` 对整数是按位取反 |
| 比较 | `== != < > <= >=` | 返回 `bool` |
| 逻辑 | `&& \|\| !` | 短路与/或，操作数必须是 `bool`（不能对整数用！） |
| 范围 | `a..b` / `a..=b` | 左闭右开 / 闭区间，常用于 `for` 和切片 |
| 赋值 | `= += -= *= /= %= &= \|= ^= <<= >>=` | 复合赋值都有 |
| 引用/解引用 | `&` `&mut` `*` | 第二篇的主角，先眼熟 |

**没有自增自减 `++` / `--`**！Rust 刻意去掉了它们（`i++` 与 `++i` 的语义差异
是 C 系语言的经典迷惑点），请写 `i += 1`。

**类型转换：Rust 没有隐式转换，只有显式的 `as`。**

```rust
fn main() {
    let x: i32 = 100;
    let y: u8 = 50;

    // let z = x + y;              // ❌ 编译错误：i32 和 u8 不能直接相加
    let z = x + y as i32;          // ✅ 显式把 u8 转成 i32
    println!("z = {}", z);         // 输出：z = 150

    // as 做窄化转换时会静默截断，数据可能丢失！
    let big: i32 = 300;
    let small = big as u8;         // u8 最大 255，300 被截断
    println!("small = {}", small); // 输出：small = 44（300 - 256）

    // 更安全的转换：try_into，失败时返回错误（第二篇详解 Result）
    let safe: Result<u8, _> = big.try_into();
    println!("安全转换结果 = {:?}", safe);   // 输出：安全转换结果 = Err(TryFromIntError(()))
}
```

对比其他语言：Java 里 `int` 赋给 `long` 自动提升、但 `long` 赋给 `int` 需要
强转；C/C++ 的隐式转换（尤其有符号/无符号混用）是 bug 重灾区；Python 几乎
无所谓数值类型。**Rust 的原则是：任何可能丢失信息的转换都必须白纸黑字写出来。**
`as` 虽然也有截断风险，但它在代码里是显眼的、可以被搜索和审计的；
真正严谨的代码应该用 `try_into()`（返回 `Result`，强制处理失败）。

### 4.14 附：初见字符串——为什么 Rust 有两种字符串

最后预告一个每个 Rust 新手都会撞上的问题。你在其他语言里习惯了"字符串就是
字符串"，但 Rust 里你会立刻遇到两种类型：`String` 和 `&str`。

```rust
fn main() {
    // &str：字符串字面量的类型，是"对一段 UTF-8 文本的借用（只读视图）"
    let s1 = "你好，Rust";

    // String：拥有所有权的、可增长的字符串，分配在堆上
    let mut s2 = String::from("你好");
    s2.push_str("，世界");            // String 可以修改、增长
    s2.push('!');                    // 追加单个字符

    // String → &str：取引用即可（廉价）
    let s3: &str = &s2;

    // &str → String：需要复制一份（有成本）
    let s4: String = s1.to_string();

    println!("{}", s1);   // 输出：你好，Rust
    println!("{}", s2);   // 输出：你好，世界!
    println!("{}", s3);   // 输出：你好，世界!
    println!("{}", s4);   // 输出：你好，Rust
}
```

为什么 Rust 要把字符串拆成两个类型？这其实是所有权哲学的必然结果：
**一段文本数据，总得搞清楚"谁拥有它、谁只是看它一眼"。**

- `String` 是**拥有者**——类似 C++ 的 `std::string`、Java 的 `StringBuilder`
  （可增长）与 `String`（堆分配）的合体；
- `&str` 是**借用者/视图**——类似 C++17 的 `std::string_view`，零成本地引用
  一段已存在的文本（字面量、String 的一部分、文件缓冲区等）。

函数签名里接受 `&str` 是最灵活的：传 `String` 的引用或字面量都可以。
这正是 4.10 温度转换器中 `split_at` 返回 `&str` 的原因——它只是原字符串的
"视图"，没有发生任何复制。

> ⚠️ **现在不需要完全消化**，知道"有两种字符串、写函数参数优先用 `&str`"
> 就够了。所有权与借用是第二篇的核心，届时一切会豁然开朗。
> 入门期遇到字符串类型不匹配时，记住两个救火口诀：`xxx.to_string()`
> 造一个 String，`&xxx` 得到一个引用。

### 4.15 毕业小项目：猜数字游戏

本篇最后，我们把前四章的知识**全部**串起来，完成官方教程的经典项目——
猜数字游戏：程序随机生成 1~100 的整数，玩家反复猜测，程序提示"大了/小了"，
猜中为止。

**第一步：创建项目并添加依赖**

```powershell
cargo new guessing_game
cd guessing_game
cargo add rand
```

**第二步：编写完整代码**（`src/main.rs`）

```rust
use rand::Rng;                    // 引入 Rng trait，才能用 gen_range 方法
use std::cmp::Ordering;           // 比较结果的枚举：Less / Greater / Equal
use std::io;                      // 标准库输入输出

fn main() {
    println!("=== 猜数字游戏 ===");
    println!("我想好了一个 1 到 100 之间的整数，来猜吧！");

    // 生成随机答案。thread_rng 是当前线程的随机数生成器
    let secret: u32 = rand::thread_rng().gen_range(1..=100);

    // loop：无限循环，猜中后 break 退出
    loop {
        println!("\n请输入你的猜测：");

        // String::new() 创建空的可变字符串，用于接收输入
        let mut guess = String::new();

        // read_line 把用户输入追加到 guess 中；
        // expect 处理可能发生的 IO 错误（失败则崩溃并显示消息）
        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");

        // shadowing：把字符串转换为数字。
        // trim() 去掉结尾的换行符；parse() 尝试解析为 u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,                    // 解析成功：得到数字
            Err(_) => {                        // 解析失败（比如输入了字母）
                println!("请输入有效的数字！");
                continue;                      // 跳过本轮，重新输入
            }
        };

        println!("你猜的是：{}", guess);

        // 比较猜测与答案，给出提示
        match guess.cmp(&secret) {
            Ordering::Less => println!("太小了！📉"),
            Ordering::Greater => println!("太大了！📈"),
            Ordering::Equal => {
                println!("🎉 恭喜猜中了！答案就是 {}", secret);
                break;                         // 猜中，退出循环
            }
        }
    }
}
```

**第三步：运行试玩**

```powershell
cargo run
```

一次可能的交互过程：

```text
=== 猜数字游戏 ===
我想好了一个 1 到 100 之间的整数，来猜吧！

请输入你的猜测：
50
你猜的是：50
太大了！📈

请输入你的猜测：
25
你猜的是：25
太小了！📉

请输入你的猜测：
abc
请输入有效的数字！

请输入你的猜测：
37
你猜的是：37
🎉 恭喜猜中了！答案就是 37
```

**代码中用到本篇的每一个知识点**：

| 代码 | 知识点 | 出处 |
|------|--------|------|
| `cargo add rand`、`rand::thread_rng()` | Cargo 依赖管理、crates.io | 第 3 章 |
| `use rand::Rng;` | 引入外部 crate 的 trait | 第 3 章 |
| `let secret: u32 = ...` | 类型标注与类型推导 | 4.5 节 |
| `1..=100` | 闭区间范围 | 4.10.5 节 |
| `let mut guess = String::new();` | 可变性、String 类型 | 4.1 / 4.14 节 |
| `let guess: u32 = match ...` | shadowing、if/match 是表达式（`match` 详解在第二篇） | 4.1.2 / 4.10 节 |
| `continue` / `break` | 循环控制 | 4.10 节 |
| `Ok(num)` / `Err(_)` | `Result` 错误处理初体验（第二篇核心） | 第 1 章哲学 |
| `println!("你猜的是：{}", guess)` | 格式化输出 | 4.12 节 |

注意 `match guess.trim().parse()` 这一段：`parse()` 返回的不是数字本身，
而是 `Result<u32, 解析错误>`——"要么成功给你一个 u32（Ok），要么失败告诉你
原因（Err）"。Rust **不允许**你假装解析一定成功；你必须显式处理 Err 分支，
否则编译不通过。对比一下：Java 的 `Integer.parseInt` 抛异常但你可以不 catch
（运行时炸给你看）；Go 会返回 `(int, error)` 多值，但忽略 error 只是约定。
**Rust 把这个约定升级成了编译器强制的铁律**——这就是第 1 章说的
"把错误从运行时提前到编译期"，现在你已经亲身体验了一次。

**扩展挑战**（试试独立完成）：

1. 统计并显示玩家猜了多少次；
2. 限定最多猜 7 次，用 `break` 带值区分"猜中"与"机会用尽"两种结局；
3. 把范围改成 1~1000，用循环标签重写提示逻辑。

### 本章小结

- **变量默认不可变**，可变需显式 `mut`；`let` 遮蔽（shadowing）创建全新变量，
  类型可以变，适合"同一概念的渐进转换"。
- `const` 是编译期常量（必须标注类型、编译期内联）；`static` 有固定地址；
  `static mut` 需要 unsafe，入门阶段避免。
- 标量类型：整数（默认 `i32`，索引用 `usize`）、浮点（默认 `f64`）、
  `bool`、`char`（4 字节 Unicode）。
- **整数溢出**：debug 下 panic，release 下回绕；显式语义有
  `wrapping_*` / `checked_*` / `overflowing_*` / `saturating_*` 四套方法。
  无符号减法溢出是经典陷阱，倒序遍历请用 `.rev()`。
- 复合类型：元组（异构、定长、可解构，常用于函数多返回值）；数组
  （同构、**长度是类型的一部分**、越界必 panic）；动态数组用 `Vec`。
- **表达式导向**是 Rust 的灵魂：除 `let` 外几乎一切皆表达式；分号把表达式
  变成语句（丢弃值）；函数返回值写尾部表达式、不加分号。
- 控制流：`if` 是表达式（取代三元运算符）、条件必须是 `bool`、分支必须有
  花括号；`loop` 是无限循环且**可以 `break` 带值**；循环标签 `'outer:`
  精确控制嵌套循环；`for` 基于迭代器，配合 `0..n`、`..=`、`.rev()`、
  `.enumerate()` 使用。
- `println!` 的 `{}`（Display，给人看）与 `{:?}`（Debug，给程序员看）要分清；
  没有 `++`/`--` 运算符；类型转换必须显式（`as` 或 `try_into()`）。
- 字符串有 `String`（拥有所有权、可增长）和 `&str`（借用视图）两种，
  函数参数优先用 `&str`——这是所有权世界的第一课，第二篇将彻底讲透。

### 动手练习

1. **温度转换器增强版**：在 4.11 的程序基础上，把"未知单位"的 `panic!`
   换成 `loop` 循环：单位不合法时提示并重新读取输入，直到输入合法为止
   （提示：利用 `loop` 可以 `break` 带值的特性）。
2. **斐波那契记忆化**：递归版 `fib` 是指数复杂度。不查资料，尝试用数组或
   `Vec` 保存已计算的结果（记忆化），让 `fib(50)` 瞬间完成。
   进阶挑战：改用迭代器 `Iterator::fold` 实现。
3. **十二天圣诞歌**：用循环打印经典圣诞歌 *The Twelve Days of Christmas*，
   利用循环的嵌套和 `.rev()` 生成"and a partridge in a pear tree"的叠加效果
   （这是官方教程的经典练习）。
4. **找因数**：编写函数 `fn divisors(n: u32) -> Vec<u32>`，返回 `n` 的所有
   正因数（用 `for` + `if n % i == 0` + `push` 实现），并写两个 `#[test]`
   单元测试验证 `divisors(12)` 和 `divisors(7)` 的结果。


---

# 第二篇：所有权——Rust 的灵魂

> 欢迎来到全书最重要的一篇。
>
> 如果说 Rust 有且只有一个"灵魂"，那就是**所有权（Ownership）**。它是 Rust 区别于 C/C++/Java/Go/Python 等所有主流语言的根本所在，也是初学者摔得鼻青脸肿的地方。
>
> 在入门篇里，你学会了 `let`、函数、`if`/`match`、循环——这些和任何语言都差不多。但从本篇开始，Rust 将展现出它真正与众不同的一面。你会第一次遇到这样的报错：
>
> ```text
> error[E0382]: borrow of moved value: `s1`
> ```
>
> 你可能会困惑、恼火，甚至想摔键盘。**别担心，这是每个 Rustacean 的必经之路。** 本篇会带你从最底层的内存模型讲起，用大量的 ASCII 内存图、真实的编译器报错和逐行解读，把所有权、借用、生命周期这些概念彻底讲透。读完本篇，你不仅"能写"Rust，还能"读懂 Rust 编译器"——后者才是真正的超能力。
>
> **本篇内容：**
>
> - 第 5 章 所有权（Ownership）：Rust 内存管理的基石
> - 第 6 章 引用与借用（References & Borrowing）：不转移所有权也能使用数据
> - 第 7 章 结构体（Structs）：用所有权思维组织数据
> - 第 8 章 枚举与模式匹配（Enums & Pattern Matching）：Rust 表达力的巅峰
>
> **写给读者的话：** 请把第 5、6 章各读至少两遍。第一遍跟着敲代码，第二遍盯着内存图想"为什么"。本篇的每一行都值得。

---

## 第 5 章 所有权（Ownership）

### 5.1 为什么需要所有权？——一个困扰了程序员 60 年的问题

在 Rust 出现之前，内存管理只有两条主流路线，各有各的痛苦。要真正理解所有权，我们必须先看看 Rust 之前的世界有多惨。

#### 5.1.1 路线一：手动管理（C 语言）——自由，但步步惊心

C 语言把内存管理的生杀大权完全交给程序员：

```c
// C 语言的内存管理：一切靠自觉
char* create_message() {
    char* msg = (char*)malloc(32);        // 在堆上申请 32 字节
    strcpy(msg, "hello");
    return msg;
}

int main() {
    char* m1 = create_message();
    // 情况一：忘了 free —— 内存泄漏（Memory Leak）
    // free(m1);

    char* m2 = m1;
    free(m1);
    free(m2);   // 情况二：同一块内存 free 两次 —— 双重释放（Double Free），未定义行为！

    printf("%s\n", m1);  // 情况三：free 之后还在用 —— 悬垂指针（Dangling Pointer），未定义行为！
    return 0;
}
```

手动管理内存的三大经典灾难：

| 灾难 | 后果 |
|---|---|
| 忘记释放 | 内存泄漏，程序越跑越慢，最终被 OOM Killer 干掉 |
| 释放两次 | 堆结构损坏，可能被黑客利用来执行任意代码 |
| 释放后使用 | 读到"别人的数据"，产生随机崩溃，而且往往在生产环境才复现 |

> 💡 **提示**：业界统计表明，微软和 Chromium 项目中约 70% 的严重安全漏洞是内存安全问题，其中绝大多数源于 C/C++ 的手动内存管理。这不是程序员不努力，而是**人脑根本不适合追踪成千上万个指针的生死**。

#### 5.1.2 路线二：垃圾回收（Java / Python / Go）——省心，但有代价

Java、Python、Go 选择了另一个极端：**垃圾回收器（Garbage Collector, GC）**。运行时自动追踪哪些内存还在被引用，回收没人用的部分。

```python
# Python：根本不用想内存
msg = "hello"
msg = "world"   # "hello" 会被 GC 自动回收
```

GC 解决了内存安全，但引入了新问题：

1. **运行时开销**：GC 需要占用 CPU 时间去扫描、标记、回收内存。
2. **停顿（Stop-The-World）**：GC 工作时程序可能暂停几十甚至上百毫秒——这对游戏引擎、高频交易、操作系统内核是不可接受的。
3. **内存占用更高**：GC 需要预留大量冗余内存才能高效工作（Java 程序"吃内存"名声在外）。
4. **无法管理非内存资源**：GC 管的是"内存对象"，但文件句柄、网络连接、互斥锁的及时释放仍然要程序员操心（于是 Java 有了 try-with-resources，Python 有了 with）。

> 🆚 **语言对比：三大内存管理方案**
>
> | 方案 | 代表语言 | 内存安全 | 运行时开销 | 可预测性 |
> |---|---|---|---|---|
> | 手动管理 | C | ❌ 全靠人肉 | 零 | ✅ 完全可控 |
> | 垃圾回收 | Java/Python/Go | ✅ 运行时保证 | 高（GC 扫描+停顿） | ❌ 回收时机不可控 |
> | **所有权** | **Rust** | ✅ **编译期保证** | **零（无 GC）** | ✅ 完全可控 |
>
> 注意：**C++ 的 RAII 是第三条路的雏形**——用栈对象的生命周期管理堆资源，离开作用域自动释放。Rust 的所有权体系正是把 RAII 从"程序员自觉遵循的惯例"升级为"编译器强制检查的语言规则"。

#### 5.1.3 Rust 的答案：编译期解决战斗

Rust 的核心思想一句话就能说完：

> **每个值都有且只有一个所有者；所有者离开作用域时，值被自动释放。**

全部规则在**编译时**检查完毕：不遵守规则，代码根本无法编译。因此 Rust 的内存安全**不需要运行时付出任何代价**——这就是著名的**零成本抽象（Zero-Cost Abstractions）**。

用一句话对比三种哲学：

- C：**"我相信你，出事你负责。"**
- Java/Go：**"你别管了，我运行时帮你擦屁股（但要付打扫费）。"**
- Rust：**"编译时把账算清楚，运行时谁都不许花钱。"**

### 5.2 栈与堆：一切故事发生的舞台

要理解所有权，必须先搞清楚数据住在哪。程序运行时可用的内存主要分两个区域：**栈（Stack）**和**堆（Heap）**。

#### 5.2.1 栈（Stack）：整齐、快速、有大小限制

栈像一摞盘子：**后进先出（LIFO）**。每次函数调用，函数的局部变量被"压入"栈顶；函数返回，这些变量被"弹出"销毁。

```text
        栈（Stack）
      ┌─────────────┐ ← 栈顶（低地址方向增长）
      │   z = 30    │   main() 的局部变量
      ├─────────────┤
      │   y = 20    │
      ├─────────────┤
      │   x = 10    │
      ├─────────────┤
      │  main 返回地址 │
      └─────────────┘ ← 栈底
```

栈的特点：

- **存取极快**：分配和释放只是移动一下栈指针，一条机器指令的事。
- **要求大小已知且固定**：编译器必须知道每个变量占多少字节。`i32`（4 字节）、`f64`（8 字节）、布尔值（1 字节）、固定大小数组都可以放栈上。
- **自动管理**：函数返回即销毁，无需任何额外代码。

#### 5.2.2 堆（Heap）：灵活、稍慢、适合"大"和"变"

如果数据**大小在编译期未知**或者**可能会增长**（比如用户输入的字符串、动态增长的列表），栈就无能为力了，只能放到堆上。

堆像一个大仓库：你向操作系统申请一块空间（`alloc`），操作系统找一块够用的内存给你，并返回它的**地址**。用完之后必须有人负责归还（`dealloc`）——至于谁来归还、什么时候归还，正是 5.1 节讨论的"三条路线"的分歧点。

```text
      堆（Heap）—— 一个由内存分配器管理的大仓库
      ┌────────────────────────────────┐
      │  0x7f1a2b000000: "hello"        │ ← 分配器随手找的空闲块
      │  0x7f1a2b000020: [1, 2, 3, 4]   │
      │  0x7f1a2b000100: "some string"  │
      │  ...（地址不连续，靠分配器管理）   │
      └────────────────────────────────┘
```

堆的特点：

- **大小灵活**：可以按需申请任意大小，还能后续扩容。
- **速度较慢**：分配时要找空闲块，访问时要通过指针跳转一次，CPU 缓存命中率也低。
- **需要有人负责释放**：内存泄漏、双重释放、悬垂指针全部发生在这里。

#### 5.2.3 关键区分：哪些数据在栈上，哪些在堆上？

| 数据 | 位置 | 原因 |
|---|---|---|
| `let x = 5;`（i32） | 栈 | 大小固定（4 字节） |
| `let b = true;` | 栈 | 大小固定 |
| `let arr = [1, 2, 3];` | 栈 | 编译期大小已知（3 × 4 字节） |
| `let s = String::from("hi");` | **栈上放元数据，堆上放字符** | 字符串内容长度可变 |
| `let v = vec![1, 2, 3];` | **栈上放元数据，堆上放元素** | 向量可增长 |

> 💡 **提示**：一条经验法则——**固定大小的简单类型住栈上，可增长的复合类型把"内容"放堆上、"遥控器"放栈上**。接下来我们会看到这个"遥控器"长什么样。

### 5.3 String 的内存布局：看懂这张图，所有权就懂了一半

下面这段代码将贯穿整章，请务必逐行理解：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;   // 等等，发生了什么？
}
```

当执行 `let s1 = String::from("hello");` 时，内存长这样：

```text
            栈（Stack）                            堆（Heap）
      ┌───────────────────┐
      │  s1               │                ┌───────┬───────┐
      │  ┌─────────────┐  │                │ index │ value │
      │  │ ptr         │──┼───────────────►│   0   │   h   │
      │  ├─────────────┤  │                ├───────┼───────┤
      │  │ len   = 5   │  │                │   1   │   e   │
      │  ├─────────────┤  │                ├───────┼───────┤
      │  │ cap   = 5   │  │                │   2   │   l   │
      │  └─────────────┘  │                ├───────┼───────┤
      └───────────────────┘                │   3   │   l   │
                                           ├───────┼───────┤
                                           │   4   │   o   │
                                           └───────┴───────┘
```

一个 `String` 由**栈上的三元组**和**堆上的字节数据**两部分组成：

- **`ptr`**（指针）：指向堆上存放字符内容的内存起始地址。
- **`len`**（长度）：当前字符串实际有多少字节。
- **`cap`**（容量）：堆上那块内存总共能容纳多少字节。`len ≤ cap`；当 `push_str` 导致 `len` 超过 `cap` 时，String 会在堆上申请一块更大的内存，把旧内容拷贝过去（这就是"扩容"）。

> ⚠️ **常见误区**：`len` 和 `cap` 的单位都是**字节**，不是字符数。UTF-8 编码下一个汉字通常占 3 个字节，`String::from("你好").len()` 的结果是 `6` 而不是 `2`。

这个设计是理解后续一切的关键：**栈上的三元组（24 字节左右）很小且固定，堆上的数据可以任意大。** Rust 的所有"所有权转移"操作，操作的其实都是栈上这个小小的三元组。

### 5.4 所有权三规则

整个 Rust 所有权体系，归根结底只有三条规则。请背下来：

1. **Rust 中的每个值都有一个变量，称为它的所有者（owner）。**
2. **一个值在任意时刻有且只有一个所有者。**
3. **当所有者离开作用域时，这个值将被丢弃（drop）。**

第三条意味着：当变量离开作用域（比如函数结束），Rust 自动调用它的 `drop` 函数释放资源（String 就释放堆内存，File 就关闭文件句柄……）。这就是 RAII 思想的直接体现——**释放点由作用域唯一确定，程序员完全不用操心，也休想干预顺序出错**。

```rust
fn main() {
    {                          // 作用域开始
        let s = String::from("hello"); // s 是堆上 "hello" 的所有者
        println!("{}", s);
    }                          // 作用域结束：Rust 自动调用 drop(s)，
                               // 堆上的 "hello" 被释放。不多不少，恰好一次。
    // println!("{}", s);      // 这里 s 已经不存在了，编译错误
}
```

对比一下：C 要求你手写 `free`（容易忘、容易双 free），Java/Go 要等 GC 大发慈悲（时机不可控）。Rust 的释放点是**代码里肉眼可见的花括号**——既安全，又可预测。

### 5.5 Move 语义：赋值不是拷贝，是"过户"

现在回答 5.3 节留下的悬念：`let s2 = s1;` 到底发生了什么？

#### 5.5.1 直觉上的两种方案，都被 Rust 否决了

对于 `s2 = s1`，其他语言无非两种选择：

**方案 A：浅拷贝（只拷贝栈上的三元组）** —— C++ 默认行为

```text
      栈                                    堆
  ┌───────────────┐
  │  s1           │                ┌───┬───┐
  │  ptr ─────────┼───┐            │ 0 │ h │
  │  len = 5      │   └───────────►│ 1 │ e │
  │  cap = 5      │                │ 2 │ l │
  ├───────────────┤           ┌───►│ 3 │ l │
  │  s2           │           │    │ 4 │ o │
  │  ptr ─────────┼───────────┘    └───┴───┘
  │  len = 5      │
  │  cap = 5      │
  └───────────────┘
```

两个指针指向同一块堆内存。**灾难来了**：`s1` 和 `s2` 离开作用域时都会执行 `drop`，同一块堆内存被释放两次——**双重释放（double free）**，内存损坏！

**方案 B：深拷贝（连堆上数据一起复制）** —— Python 的 `copy.deepcopy` 风格

```text
      栈                                    堆
  ┌───────────────┐                ┌───┬───┐
  │  s1           │                │ 0 │ h │   ← "hello"（原件）
  │  ptr ─────────┼───────────────►│...│...│
  │  len = 5      │                └───┴───┘
  │  cap = 5      │                ┌───┬───┐
  ├───────────────┤                │ 0 │ h │   ← "hello"（副本）
  │  s2           │                │...│...│
  │  ptr ─────────┼───────────────►└───┴───┘
  │  len = 5      │
  │  cap = 5      │
  └───────────────┘
```

安全是安全了，但如果字符串有 1GB 呢？一次赋值就要拷贝 1GB 数据——Rust 不能接受这种"隐形的性能炸弹"。

#### 5.5.2 Rust 的方案：移动（Move）——转移所有权，旧变量立即作废

Rust 的做法出人意料地聪明：**拷贝栈上的三元组（浅拷贝），同时宣告旧变量 `s1` 失效，把堆数据的所有权"过户"给 `s2`。**

```rust
let s1 = String::from("hello");
let s2 = s1;    // 所有权从 s1 移动（move）到 s2
```

移动之后的内存状态：

```text
      栈                                    堆
  ┌───────────────┐
  │  s1           │
  │  (已被作废，    │                ┌───┬───┐
  │   禁止使用)    │                │ 0 │ h │
  ├───────────────┤           ┌───►│ 1 │ e │
  │  s2           │           │    │ 2 │ l │
  │  ptr ─────────┼───────────┘    │ 3 │ l │
  │  len = 5      │                │ 4 │ o │
  │  cap = 5      │                └───┴───┘
  └───────────────┘
```

妙处在于：

- **没有深拷贝**：只复制了 24 字节的三元组，O(1) 开销。
- **没有双 free**：堆里始终只有一个"有效遥控器" `s2`，离开作用域时只有它执行 `drop`。
- **编译器强制执行**：任何对失效变量 `s1` 的使用都是**编译错误**，运行前就拦截。

> 🆚 **语言对比：Rust move vs C++ std::move**
>
> C++11 的 `std::move` 和 Rust move 思想一致（转移资源所有权），但有两个关键区别：
>
> | 维度 | C++ `std::move` | Rust move |
> |---|---|---|
> | 默认行为 | 拷贝（深拷贝/拷贝构造），move 需要显式写 `std::move` | **move 是默认行为**，拷贝需要显式写 `.clone()` |
> | 移后源对象 | 语法上仍可使用（处于"有效但未指定"状态，用了就是 bug） | **编译器直接禁止**使用，编译错误 |
> | 检查者 | 程序员自觉 | 编译器强制 |
>
> 一句话：C++ 把 move 当作"可选的性能优化"，Rust 把 move 当作"默认且强制的安全规则"。

#### 5.5.3 第一次直面编译器：error\[E0382\]

来看一个真实的编译错误——这是每个 Rust 初学者的"成人礼"：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);   // 试试用已经"过户"出去的 s1
}
```

保存后在 RustRover 里运行（或 `cargo run`），你会得到：

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:4:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`,
  |            which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `println!` (in Nightly builds, run with -Z macro-backtrace for more info)
```

**逐行解读（学会读报错是 Rust 程序员的核心技能）：**

| 报错行 | 含义 |
|---|---|
| `error[E0382]` | 错误编号是 E0382，语义是"借用了已被移动的值"。**错误码可以直接搜索**：在终端运行 `rustc --explain E0382` 会得到官方详细解释。 |
| `--> src/main.rs:4:28` | 出错位置：main.rs 第 4 行第 28 列（即 `s1`）。 |
| `move occurs because ... String, which does not implement the Copy trait` | **解释为什么会发生 move**：因为 `String` 类型没有实现 `Copy` trait（Copy 类型见 5.7 节），所以赋值默认是 move 而不是拷贝。 |
| `let s2 = s1;` / `value moved here` | **指出 move 发生的现场**：第 3 行，所有权在这一刻从 s1 过户给 s2。 |
| `println!(...)` / `value borrowed here after move` | **指出违规现场**：第 4 行试图在 move 之后继续使用 s1。 |

注意到编译器不仅告诉你"错了"，还告诉你"为什么错、错在哪一步、前因后果是什么"。**Rust 编译器是最好的老师，请务必逐字阅读它的每一条报错。**

> 💡 **提示**：在 RustRover 中，把鼠标悬停在红色波浪线上就能看到同样的报错；按 `Ctrl+F1`（或点击灯泡图标）有时会给出自动修复建议。

### 5.6 clone：我就是要一份真正的副本

如果你确实想要堆数据的完整拷贝（深拷贝），Rust 要求你**显式**地写出来：

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();   // 深拷贝：堆上数据也复制一份

    println!("s1 = {}, s2 = {}", s1, s2);   // 两个都有效！
}
```

```text
输出：
s1 = hello, s2 = hello
```

内存图与 5.5.1 的"方案 B"完全一致：两份三元组，两份堆数据，各自独立管理，各自 drop，互不干扰。

> ⚠️ **常见误区**：`.clone()` 不便宜——它要把堆上全部数据复制一遍。Rust 的设计哲学是"**昂贵的操作必须在代码里显眼**"，让你看到 `.clone()` 就意识到这里有一次 O(n) 拷贝。相反，其他语言里"悄无声息"的深拷贝才是真正的性能陷阱。

### 5.7 Copy trait：小类型可以"复制即拷贝"

等等，之前学的整数赋值明明两边都能用啊？

```rust
fn main() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);   // 完全没问题！
}
```

```text
输出：
x = 5, y = 5
```

为什么 `x` 没有像 `s1` 那样被作废？因为 `i32` 实现了 **`Copy` trait**：

- 如果一个类型实现了 `Copy`，赋值时是**逐位拷贝**（bitwise copy），旧变量继续有效。
- 只有**完全存储在栈上、大小已知且固定**的类型才有资格实现 `Copy`——反正数据总共几个字节，拷贝的代价可以忽略，move 和 copy 在机器码层面没有区别，那就干脆拷贝、保留旧值，减少心智负担。
- `Copy` 是**编译器强加的标记**，你不能给 `String` 手动实现 `Copy`——它持有堆资源，若允许 Copy 就会重现双重释放灾难（编译器会拒绝）。

**常见的 Copy 类型：**

| 类别 | 例子 |
|---|---|
| 所有整数类型 | `i32`、`u64`、`isize`、`usize`…… |
| 布尔类型 | `bool` |
| 浮点类型 | `f32`、`f64` |
| 字符类型 | `char`（4 字节，固定大小） |
| 元素全为 Copy 类型的元组 | `(i32, bool)` 是 Copy；`(i32, String)` **不是** |

**常见的非 Copy 类型：** `String`、`Vec<T>`、以及任何持有堆资源或文件/网络句柄的类型。

> 💡 **提示**：判断一个操作是 move 还是 copy 的口诀：**"栈上小数据自动拷贝，持有资源的类型一律过户。"**

### 5.8 所有权与函数：传参、返回都是"过户"

所有权规则在函数边界同样适用，这会带来一些"反直觉"的行为。

#### 5.8.1 传参会 move

```rust
fn main() {
    let s = String::from("hello");   // s 进入作用域

    takes_ownership(s);              // s 的所有权被移动到函数内部
    // println!("{}", s);            // error[E0382]！s 已经失效

    let x = 5;                       // x 进入作用域
    makes_copy(x);                   // i32 是 Copy，x 仍然有效
    println!("x 依然可用：{}", x);    // 输出：x 依然可用：5
}                                    // x 离开作用域；s 早已被移走

fn takes_ownership(some_string: String) {   // some_string 获得所有权
    println!("拿到了：{}", some_string);
}   // some_string 离开作用域，drop 被调用，堆内存释放

fn makes_copy(some_integer: i32) {   // some_integer 得到一份拷贝
    println!("拿到了：{}", some_integer);
}   // some_integer 离开作用域，无事发生（i32 没有 drop 逻辑）
```

```text
输出：
拿到了：hello
x 依然可用：5
```

`String` 传参后调用者就"失去"了它——因为函数调用本质上是把实参的值赋给形参，遵循与赋值完全相同的 move 规则。

#### 5.8.2 返回值转移所有权

```rust
fn main() {
    let s1 = gives_ownership();        // 函数把返回值的所有权移交给 s1

    let s2 = String::from("hi");       // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 移入函数，返回值又移交给 s3

    println!("s1 = {}, s3 = {}", s1, s3);
}   // s3、s1 依次 drop；s2 的所有权早已移交，不会重复释放

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string                       // 返回值，所有权移交给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string                          // 收进来，再还回去
}
```

```text
输出：
s1 = yours, s3 = hi
```

**值的所有权轨迹**可以用一张图概括：

```text
   main                        函数
  ┌──────┐    实参 move ──►  ┌──────────┐
  │ 调用者│                   │ 函数体    │
  │      │ ◄── 返回值 move ──┘          │
  └──────┘                  └──────────┘
   所有权像接力棒一样在变量之间传递，全程只有一个持有者
```

#### 5.8.3 痛点浮现：这也太折腾了！

注意到问题了吗？如果我只是想让函数"看一眼"我的字符串、算个长度，按上面的规则，所有权就被函数拿走了。想继续用？只能让函数把它还回来：

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);   // 连本带利拿回来
    println!("'{}' 的长度是 {}", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {  // 元组打包返回
    let length = s.len();
    (s, length)
}
```

能用，但丑得令人发指——**每次调用函数都要"还回"所有权，代码里塞满了这种交接仪式。** 这个痛点正是下一章"引用与借用"要解决的问题：让函数**借用**数据，而不占有它。

### 本章小结

1. Rust 用**所有权**在编译期解决内存管理：不需要 GC，也不会内存泄漏/双 free/悬垂指针。
2. 三条规则：每个值有唯一所有者；所有者唯一；所有者离开作用域时值被 drop。
3. 数据分栈与堆：固定大小的小类型在栈上；`String` 等类型把**三元组（ptr/len/cap）放栈上、数据放堆上**。
4. 赋值和传参默认是 **move**：拷贝三元组 + 旧变量作废 + 所有权过户，编译器禁止再使用旧变量（error\[E0382\]）。
5. 需要深拷贝时显式调用 **`.clone()`**——贵，但显眼。
6. 栈上小类型实现了 **Copy** trait，赋值时拷贝且旧值保留。
7. 函数传参会 move，返回值会把所有权移交回调用者。

### 动手练习

1. **画内存图**：不动手编译，画出执行完下面每行代码后的内存状态图（栈 + 堆），并指出哪些变量有效：
   ```rust
   let a = String::from("rust");
   let b = a;
   let c = b.clone();
   ```
2. **读报错**：故意写下 `let s2 = s1; println!("{}", s1);`，运行 `cargo run`，把完整报错抄下来，用自己的话解释每一行。然后运行 `rustc --explain E0382` 对照官方解释。
3. **修代码**：下面代码有两处编译错误，先不编译、只动脑找出它们，再编译验证：
   ```rust
   fn main() {
       let s = String::from("hi");
       let n = 42;
       consume(s);
       consume(s);
       consume_num(n);
       consume_num(n);
   }
   fn consume(x: String) { println!("{}", x); }
   fn consume_num(x: i32) { println!("{}", x); }
   ```
4. **思考题**：为什么 Rust 不允许 `String` 实现 `Copy`？如果真的允许了，会发生哪种 5.1 节提到的灾难？

---

## 第 6 章 引用与借用（References & Borrowing）

### 6.1 为什么需要引用？——上一章留下的烂摊子

回忆 5.8.3 的痛苦：想让函数算个字符串长度，就得交出所有权，再靠元组把字符串"领回来"。如果每个函数调用都要这么折腾，Rust 早就被程序员抛弃了。

**引用（Reference）**就是解决方案：**允许你在不取得所有权的前提下使用一个值**。这种"创建引用"的行为称为**借用（Borrowing）**——就像现实生活中借书：书还是你的，我只借来看看，看完原样奉还。

先看效果，对比一下：

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);   // 传入 s1 的引用（&s1），所有权纹丝不动

    println!("'{}' 的长度是 {}", s1, len);   // s1 照常使用！
}

fn calculate_length(s: &String) -> usize {   // 参数类型是 &String：一个引用
    s.len()
}   // s 离开作用域。但引用不拥有值，所以什么都不释放
```

```text
输出：
'hello' 的长度是 5
```

对比 5.8.3 的版本：函数签名里不再有讨厌的 `(String, usize)` 元组，调用处 `s1` 也不再失效。**干净利落。**

内存图如下——引用本质上就是一个指向数据的指针，存放在栈上：

```text
            栈                                     堆
      ┌───────────────┐
      │  s1           │                ┌───┬───┐
      │  ptr ─────────┼───────────────►│ 0 │ h │
      │  len = 5      │           ┌───►│ 1 │ e │
      │  cap = 5      │           │    │ 2 │ l │
      ├───────────────┤           │    │ 3 │ l │
      │  s (引用)      │           │    │ 4 │ o │
      │  ptr ─────────┼───────────┘    └───┴───┘
      └───────────────┘
       calculate_length 的栈帧
```

注意图中有两个指针指向堆数据：`s1` 的 `ptr`（所有者）和 `s`（引用）。区别在于：

- `s1` 拥有数据：离开作用域会 drop。
- `s` 只是借用：离开作用域什么也不发生。

> 💡 **提示**：与 move（转移所有权）相对，**通过引用传参不会 move**。函数是"租客"不是"房主"，租期（函数调用）一结束，房子（数据）毫发无损地归还原主。

### 6.2 不可变引用 `&T`：只读的访问通行证

`&s1` 创建的是一个**不可变引用**：可以读，不能改。

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;                 // r 是 s 的不可变引用
    println!("通过引用读取：{}", r);   // 输出：hello
    // r.push_str("!");         // 编译错误！不可变引用不能修改数据
}
```

**同一时间可以存在任意多个不可变引用**——大家都只是读，井水不犯河水：

```rust
fn main() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{}, {}, {}", r1, r2, r3);   // 全部合法：hello, hello, hello
}
```

```text
            栈                                    堆
  ┌───────────────┐                ┌───────────┐
  │  s (String)   │───────────────►│ "hello"   │
  ├───────────────┤           ┌───►│           │
  │  r1: &String  │───────────┤    └───────────┘
  ├───────────────┤           │
  │  r2: &String  │───────────┤
  ├───────────────┤           │
  │  r3: &String  │───────────┘
  └───────────────┘
  多个读者，互不干扰 —— 完全安全
```

### 6.3 可变引用 `&mut T`：独占的修改权限

想修改借来的数据，需要**可变引用**，语法是 `&mut`。注意变量本身也要声明为 `mut`：

```rust
fn main() {
    let mut s = String::from("hello");   // s 本身可变
    change(&mut s);                      // 借出可变引用
    println!("{}", s);                   // 输出：hello, world
}

fn change(some_string: &mut String) {    // 接收可变引用
    some_string.push_str(", world");     // 通过可变引用修改
}
```

```text
输出：
hello, world
```

#### 6.3.1 铁律：可变引用必须独占

这里是 Rust 借用系统的核心规则，请务必刻在脑子里：

> **在任意时刻，对于一个特定的值，你只能拥有以下两者之一：**
>
> - **一个**可变引用；
> - **任意多个**不可变引用。
>
> **二者不可兼得，违反即编译错误。**

先体会"只能有一个可变引用"：

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;   // ✗ 第二个可变引用！

println!("{}, {}", r1, r2);
```

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:4:10
  |
3 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
4 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
5 |
6 |     println!("{}, {}", r1, r2);
  |                          -- first borrow later used here
```

**逐行解读：**

- `error[E0499]`：错误码 E0499，"同一时间对同一变量进行了多次可变借用"。
- `first mutable borrow occurs here`：第一次可变借用发生在第 3 行（`&mut s` 借给了 r1）。
- `second mutable borrow occurs here`：第 4 行又试图可变借用——违规现场。
- `first borrow later used here`：**关键点**：编译器指出 r1 在第 6 行还在被使用，说明第一次借用"还没结束"，所以第二次借用与之冲突。

再体会"可变与不可变不能共存"：

```rust
let mut s = String::from("hello");

let r1 = &s;        // 不可变引用
let r2 = &s;        // 不可变引用（没问题）
let r3 = &mut s;    // ✗ 在存在不可变引用的情况下申请可变引用

println!("{}, {}, and {}", r1, r2, r3);
```

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:14
  |
3 |     let r1 = &s;
  |              -- immutable borrow occurs here
4 |     let r2 = &s;
  |              -- immutable borrow occurs here
5 |     let r3 = &mut s;
  |              ^^^^^^ mutable borrow occurs here
6 |
7 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                  -- immutable borrow later used here
```

**逐行解读：**

- `error[E0502]`：错误码 E0502，"在存在不可变借用的同时又进行了可变借用"。
- 编译器列出两次不可变借用的位置，以及可变借用的违规位置。
- 最后一行再次强调：`r1`/`r2` 后面还要用（第 7 行），所以它们的借用仍然活跃，可变引用必须让路。

#### 6.3.2 用"会议室白板"理解借用规则

想象一块**会议室白板**（数据）和一群开会的人（引用）：

- **不可变引用 = 围观白板的人**。多少人围观都行——大家只看不写，白板内容稳定，谁也不会读到"写到一半"的内容。
- **可变引用 = 唯一一个拿马克笔的人**。要写东西，只能一个人上台；而且**写的时候其他人必须出去**——不然有人读到一半，内容突然变了，讨论就乱套了。

```text
   允许的场景：                          禁止的场景：
   ┌────────────┐                      ┌────────────┐
   │   白 板     │  ◄─ 👀👀👀 (多个读者)  │   白 板     │  ◄─ 👀 (读者)
   └────────────┘                      └────────────┘  ◄─ ✏️ (写者) ✗
   
   ┌────────────┐                      ┌────────────┐
   │   白 板     │  ◄─ ✏️ (唯一写者)     │   白 板     │  ◄─ ✏️✏️ (两个写者) ✗
   └────────────┘                      └────────────┘
```

#### 6.3.3 为什么这样设计？——消灭数据竞争

这条规则不是 Rust 故意刁难你，它直接消灭了并发编程中最恐怖的 bug：**数据竞争（Data Race）**。数据竞争发生的三个条件：

1. 两个或多个指针同时访问同一数据；
2. 至少一个在写；
3. 没有同步机制。

借用规则恰好从结构上掐死了第 1、2 条的组合：**要么大家只读（没人写），要么独占写（没人同时读）**。于是"在编译期"就保证了不存在数据竞争——这就是 Rust 宣传语 **"无畏并发（Fearless Concurrency）"** 的底气来源。

即使在单线程代码里，这条规则也挡住了经典 bug。比如 C++ 程序员的噩梦：

```cpp
// C++：迭代器失效 —— 运行时未定义行为
std::vector<int> v = {1, 2, 3};
for (auto& x : v) {
    v.push_back(x);   // push_back 可能触发扩容，
}                     // 旧内存被释放，x 变成悬垂引用 → 崩溃或数据错乱
```

同样的逻辑在 Rust 里**根本编译不过**：

```rust
let mut v = vec![1, 2, 3];
for x in &v {          // &v：不可变借用（"读者"）
    v.push(*x);        // error[E0502]！迭代期间想可变借用 v（"写者"）
}
```

编译器在此刻拦下你，胜过生产环境凌晨三点的崩溃。

### 6.4 悬垂引用：编译器如何堵死"访问已释放内存"

**悬垂引用（Dangling Reference）**指向一块已经被释放的内存。这是 C/C++ 中悬垂指针的对应概念。Rust 编译器保证悬垂引用**不可能存在**。

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {           // 声称返回一个 String 的引用
    let s = String::from("hello"); // s 是函数内的局部变量
    &s                             // 返回 s 的引用
}   // ⚠️ s 在这里离开作用域被 drop！引用指向的内存已释放
```

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value,
          but there is no value for it to be borrowed from
  = help: consider using the `'static` lifetime
```

**逐行解读：**

- `error[E0106]`：错误码 E0106，"缺少生命周期说明符"。
- `this function's return type contains a borrowed value, but there is no value for it to be borrowed from`：这句话是整个错误的核心——**返回值是一个"借来的东西"，但函数里没有任何东西可供它借**。参数列表是空的（借用只能来自参数或全局数据），函数内部创建的 `s` 在返回瞬间就被销毁了。编译器因此发现：这个引用一旦返回必然是悬垂的。
- `consider using the 'static` lifetime`：编译器给出（此处不太靠谱的）建议——涉及生命周期标注，这是第 7 章末尾预告、后面章节详讲的内容。现在只需知道：**函数想返回引用，被引用的数据必须活得比引用久**。

修复方法很简单：直接返回 `String`（移交所有权），而不是引用：

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s    // 所有权移交出去，s 不会被销毁，安全
}
```

> 💡 **提示**：编译器对悬垂引用的检查是一条"兜底铁律"：**引用的寿命绝不能超过被引用者的寿命**。这条规则将贯穿生命周期（lifetime）的所有内容。

### 6.5 借用检查器与 NLL：引用的"寿命"有多长

负责执行上述所有规则的编译器组件叫**借用检查器（Borrow Checker）**——就是让无数初学者" Fight the borrow checker（与借用检查器搏斗）"的那位。

它分析的核心问题是：**每个借用从哪一行开始"活着"，到哪一行"死去"**——这个区间称为引用的**生命周期（lifetime）**。

#### 6.5.1 NLL（Non-Lexical Lifetimes，非词法生命周期）

在 Rust 2018 之前，借用的有效期按**词法作用域**（花括号）计算，死板且误伤很多合法代码。自 Rust 2018 起引入 **NLL**：**借用的有效期到"最后一次使用"为止**，而不是到花括号结束。

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;        // 不可变借用开始
    let r2 = &s;        // 不可变借用开始
    println!("{} 和 {}", r1, r2);
    // ─── r1、r2 的借用在此行之后结束（NLL：最后一次使用处）───

    let r3 = &mut s;    // ✓ 此刻没有活跃的不可变借用了，合法！
    r3.push_str(", world");
    println!("{}", r3);
}
```

```text
输出：
hello 和 hello
hello, world
```

借用时间轴：

```text
代码行              r1、r2（不可变）      r3（可变）
─────────────────────────────────────────────
let r1 = &s;        ┃ 借用开始
let r2 = &s;        ┃
println!(r1, r2)    ┃ 最后一次使用 → 借用结束
let r3 = &mut s;                          ┃ 借用开始（不重叠，OK）
r3.push_str(...)                          ┃
println!(r3)                              ┃ 最后一次使用 → 借用结束
```

> 💡 **提示**：NLL 的意义在于——只要借用的"活跃区间"不重叠，规则就放行。判断借用是否冲突时，不要看花括号，**看每个引用最后一次被使用的位置**。

### 6.6 切片（Slice）：引用一组元素的"窗口"

**切片（slice）**是对集合中**连续一段元素**的引用，它不拥有数据，是借用体系的重要成员。

#### 6.6.1 字符串切片 `&str`

```rust
let s = String::from("hello world");

let hello = &s[0..5];    // 第 0 到第 4 字节（区间左闭右开）
let world = &s[6..11];   // 第 6 到第 10 字节
```

切片的内存布局——它是"胖指针"：**一个指向起始位置的指针 + 一个长度**：

```text
      栈                                          堆
  ┌───────────────┐                    ┌───┬───┐
  │ s (String)    │                    │ 0 │ h │◄─┐
  │  ptr ─────────┼─────────┐          ├───┼───┤  │
  │  len = 11     │         └─────────►│ 1 │ e │  │
  │  cap = 11     │                    ├───┼───┤  │
  ├───────────────┤                    │ 2 │ l │  │ hello 切片
  │ hello (&str)  │                    ├───┼───┤  │ 指向 s 的堆数据
  │  ptr ─────────┼────────────────────┘   │ l │◄─┘
  │  len = 5      │                    │ 4 │ o │
  └───────────────┘                    ├───┼───┤
                                       │ 5 │   │
                                       ├───┼───┤
                                       │ 6 │ w │ ...
                                       └───┴───┘
```

范围语法的几个等价写法：

```rust
let s = String::from("hello");

let slice = &s[0..2];   // "he"
let slice = &s[..2];    // 从 0 开始可以省略：同 &s[0..2]
let len = s.len();
let slice = &s[3..len]; // "lo"
let slice = &s[3..];    // 到结尾可以省略：同 &s[3..len]
let slice = &s[..];     // 整个字符串的切片
```

#### 6.6.2 字符串字面量其实就是切片

```rust
let s = "Hello, world!";   // s 的类型是 &str
```

字符串字面量存储在程序的二进制文件中（只读数据段），`s` 是指向它的切片——因此 `&str` 是**不可变引用**，这也解释了为什么字面量不可修改。

#### 6.6.3 UTF-8 陷阱：切片的边界必须落在字符边界上

Rust 字符串是 UTF-8 编码。`&s[0..5]` 的索引单位是**字节**，而且切分点必须恰好落在字符边界上，否则——运行时 panic！

```rust
fn main() {
    let s = String::from("你好世界");   // 每个汉字 3 字节，共 12 字节
    let first = &s[0..3];              // ✓ "你"（第 0~2 字节正好一个字符）
    println!("{}", first);
    // let bad = &s[0..2];             // ✗ 把"你"拦腰截断！
}
```

如果取消最后一行注释并运行：

```text
thread 'main' panicked at 'byte index 2 is not a char boundary;
it is inside '你' (bytes 0..3) of `你好世界`', src/main.rs:4:17
```

> ⚠️ **常见误区**：来自 Python 的读者尤其容易中招——Python 的 `s[0]` 是"第 0 个字符"，Rust 中 `s[0]` 是非法的（String 不支持按索引取单个字符），切片索引是**字节**不是字符。需要按字符处理时用 `s.chars()` 迭代器。

#### 6.6.4 其他切片：`&[T]`

切片不止用于字符串。数组、Vec 也可以切：

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];   // 类型是 &[i32]，内容为 [2, 3]

assert_eq!(slice, &[2, 3]);
```

`&[i32]` 和 `&str` 一样是"胖指针"（指针 + 长度），不拥有数据，只借用。

#### 6.6.5 实战：用切片写 first_word

写一个函数，返回字符串中第一个单词。用切片实现：

```rust
fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);   // 传入 &String，自动转换为 &str
    println!("第一个单词：{}", word);   // 输出：第一个单词：hello

    let literal = "hi rust";       // 字符串字面量本身就是 &str
    println!("{}", first_word(literal)); // 输出：hi
}

// 参数用 &str 而不是 &String：能同时接受 String 引用和字面量，更通用
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {        // 找到第一个空格
            return &s[..i];      // 返回空格之前的切片
        }
    }

    &s[..]                       // 没有空格：整个字符串就是一个单词
}
```

注意签名设计：`fn first_word(s: &str) -> &str`。用 `&str` 作参数类型让函数同时兼容 `&String`（自动解引用转换，deref coercion）和字符串字面量——这是 Rust API 设计的标准做法。

切片的安全性由借用检查器兜底：

```rust
let mut s = String::from("hello world");
let word = first_word(&s);   // word 借用 s（不可变）

// s.clear();                // error[E0502]！word 还活着，不能可变借用 s
println!("{}", word);
s.clear();                   // word 最后一次使用后，这里才允许修改
```

如果没有切片机制（比如返回 `usize` 下标），`s.clear()` 之后旧下标就指向了错误的位置——这类 bug 在 Rust 中**编译期即被根除**。

### 本章小结

1. **引用（`&T`）**让你在不取得所有权的情况下使用值，创建引用称为**借用**。
2. 借用铁律：任意时刻，要么**一个可变引用**，要么**多个不可变引用**——二者不可兼得。这条规则在编译期消灭了数据竞争。
3. **悬垂引用被编译器彻底杜绝**：引用的寿命不能超过被引用者的寿命（error\[E0106\]）。
4. **NLL**：借用的有效期到"最后一次使用"为止，而非整个花括号作用域。
5. **切片**是对连续一段元素的不拥有引用：`&str` 是字符串切片，`&[T]` 是数组/Vec 切片，均为"指针+长度"的胖指针。
6. 字符串切片索引按**字节**计算，切分点必须在 UTF-8 字符边界上，否则 panic。

### 动手练习

1. **判断对错**：下面的借用哪些合法、哪些会被借用检查器拒绝？先动脑，再编译验证。
   ```rust
   let mut v = vec![1, 2, 3];
   let a = &v;        // (1)
   let b = &v;        // (2)
   let c = &mut v;    // (3)
   println!("{:?}", a);
   let d = &mut v;    // (4)
   ```
2. **白板练习**：用自己的话向一个不懂编程的朋友解释"为什么不能同时存在可变引用和不可变引用"（白板类比或自创类比）。
3. **写代码**：实现 `fn last_word(s: &str) -> &str`，返回字符串中最后一个单词。测试：`last_word("hello rust world")` 应返回 `"world"`，`last_word("single")` 应返回 `"single"`。
4. **触发报错**：写出会产生 error\[E0502\] 和 error\[E0106\] 的最短代码各一份，运行 `rustc --explain E0502` 和 `rustc --explain E0106` 阅读官方解释。
5. **UTF-8 实验**：`let s = String::from("Rust语言");` 打印 `s.len()`，然后尝试不同的切片范围，找出哪些合法哪些 panic，并解释原因。

---

## 第 7 章 结构体（Structs）

结构体（struct）让你把多个相关的值组合成一个有意义的整体，并给每一部分命名。如果说 C 有 struct、Java 有 class 的字段部分、Python 有 dataclass，那么 Rust 的结构体就是它们的安全加强版——**没有继承，但组合出了全部战斗力**。

### 7.1 定义与实例化

```rust
// 定义结构体：列出所有字段的名字和类型
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 实例化：为每个字段赋值，顺序不必与定义一致
    let mut user1 = User {
        active: true,
        username: String::from("张三"),
        email: String::from("zhangsan@example.com"),
        sign_in_count: 1,
    };

    // 用点号访问和修改字段（整个实例必须是 mut 才能改）
    user1.email = String::from("newemail@example.com");
    println!("{} 登录了 {} 次", user1.username, user1.sign_in_count);
}
```

```text
输出：
张三 登录了 1 次
```

> ⚠️ **常见误区**：Rust 只支持**整个实例**级别的可变性——`let mut user1` 表示 user1 的所有字段都可改；不存在"只有 email 可改、username 不可改"的写法。这一点和 Java 的 final 字段思路不同。

**结构体作为函数返回值**是常见模式——"构造函数"：

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
```

### 7.2 字段初始化简写（Field Init Shorthand）

上面的 `username: username` 写两遍名字很啰嗦。当**局部变量与字段同名**时，可以只写一次：

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,       // 等价于 username: username
        email,          // 等价于 email: email
        sign_in_count: 1,
    }
}
```

### 7.3 结构体更新语法（Struct Update Syntax）

想基于一个已有实例创建新实例、只改少数字段？用 `..` 语法：

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("张三"),
        email: String::from("zhangsan@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("lisi@example.com"),
        ..user1        // 其余字段从 user1 获取
    };

    println!("{}: {}", user2.username, user2.email);
    // println!("{}", user1.username);   // ⚠️ 见下方解析
}
```

```text
输出：
张三: lisi@example.com
```

> ⚠️ **常见误区（所有权警告！）**：`..user1` 对**没有显式赋值**的字段执行的是**赋值语义**——`username: String` 不是 Copy 类型，所以 `user1` 的 `username` 字段被 **move** 给了 `user2`！此后 `user1` 整体失效（即使 `active`、`sign_in_count` 是 Copy 类型）。若被"继承"的字段全是 Copy 类型，user1 才仍然有效。这与"借用了 user1"无关，是货真价实的所有权转移。

### 7.4 元组结构体与单元结构体

**元组结构体（Tuple Struct）**：有名字但字段没名字的结构体，靠位置区分。适合"给普通元组套一层类型语义"的场景：

```rust
struct Color(i32, i32, i32);      // 颜色：R, G, B
struct Point(i32, i32, i32);      // 点：x, y, z

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("红色的 R 分量：{}", black.0);   // 用 .索引 访问
    // black 和 origin 是不同类型，即使内部结构一模一样 —— 类型安全！
}
```

> 💡 **提示**：`Color` 和 `Point` 字段完全相同却是**不同类型**——编译器不允许把 Point 当 Color 传参。这比裸元组 `(i32, i32, i32)` 安全得多，是零成本的类型区分手段。

**单元结构体（Unit-Like Struct）**：没有任何字段，类似单元类型 `()`。常用于实现某个 trait 但不需要存数据的类型：

```rust
struct AlwaysEqual;

fn main() {
    let _subject = AlwaysEqual;
}
```

### 7.5 结构体中的所有权：为什么字段用 String 而不是 &str

回看 User 的定义，字段类型是 `String` 而非 `&str`。这是深思熟虑的选择：

**结构体拥有其字段的数据**，这样只要结构体实例有效，其数据就有效——没有外部依赖，没有寿命问题。

如果试着存放引用：

```rust
struct User {
    username: &str,   // 字段是一个引用……
    email: &str,
}

fn main() {
    let user1 = User {
        username: "张三",   // 字符串字面量
        email: "a@b.com",
    };
}
```

编译报错：

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:15
  |
2 |     username: &str,
  |               ^ expected named lifetime parameter
  |
help: consider introducing a lifetime
  |
1 | struct User<'a> {
2 |     username: &'a str,
  |               +++
```

**解读**：编译器发现结构体里存了引用，立刻警觉——"这个引用指向的数据能活多久？如果指向的数据先死了，结构体实例不就悬垂了吗？"它要求你用**生命周期标注**（`'a`）显式声明"结构体实例的寿命不能超过它借用的数据的寿命"。

生命周期是后面章节的正餐。当前阶段请记住经验法则：

> **新手期结构体字段一律用拥有所有权的类型（`String`、`Vec` 等），不存引用。** 等业务需要零拷贝优化时再引入 `&str` + 生命周期。

### 7.6 方法（Methods）：把行为挂到数据上

方法与函数的区别只有一个：**定义在结构体（或枚举、trait）的上下文中，第一个参数永远是 `self`**。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {                       // impl 块：implementation 的缩写
    fn area(&self) -> u32 {            // 方法：第一个参数是 &self
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("面积：{}", rect1.area());   // 用点号调用方法
}
```

```text
输出：
面积：1500
```

#### 7.6.1 self、&self、&mut self 的区别

`self` 的三种形态对应所有权体系的三种获取方式，请对照第 5、6 章理解：

| 写法 | 全称 | 效果 | 典型场景 |
|---|---|---|---|
| `&self` | `self: &Self` | **不可变借用**：只读，不取得所有权 | 绝大多数方法（如 `area`） |
| `&mut self` | `self: &mut Self` | **可变借用**：可以修改字段 | 修改状态的方法（如 `set_width`） |
| `self` | `self: Self` | **取得所有权**：move 进方法，调用后实例失效 | 消费型方法（如 `into_string`）、建造者模式 |

```rust
impl Rectangle {
    fn area(&self) -> u32 {                    // 只读
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {         // 修改自身
        self.width *= factor;
        self.height *= factor;
    }

    fn into_square(self) -> Rectangle {        // 消费自身，返回新实例
        Rectangle { width: self.width, height: self.width }
    }
}

fn main() {
    let mut rect = Rectangle { width: 30, height: 50 };
    println!("原面积：{}", rect.area());
    rect.scale(2);
    println!("放大后面积：{}", rect.area());    // 输出 6000
    let sq = rect.into_square();
    // println!("{}", rect.area());            // error[E0382]！rect 已被 move
    println!("正方形面积：{}", sq.area());      // 输出 3600
}
```

> 🆚 **语言对比**：C++ 和 Java 里 `this` 永远是指针/引用语义，"方法是否取得所有权"这个问题根本不存在（也因此埋着 use-after-move 的雷）。Rust 把"借还是拿"变成**方法签名上的一等公民**——看签名就知道方法会不会"吃掉"你的对象。

**自动引用与解引用**：`rect.area()` 中 rect 不是引用却能调用 `&self` 方法——Rust 会自动加上 `&`、`&mut` 或 `*` 使调用成立。所以 Rust 不需要 C++ 的 `.` 和 `->` 两套语法。

### 7.7 关联函数（Associated Functions）

`impl` 块里**不以 self 为第一个参数**的函数叫关联函数——类似 Java 的静态方法。用 `类型名::函数名` 调用：

```rust
impl Rectangle {
    // 关联函数：构造一个正方形
    fn square(size: u32) -> Self {    // Self 是 Rectangle 的别名
        Self { width: size, height: size }
    }
}

fn main() {
    let sq = Rectangle::square(10);   // :: 调用关联函数，. 调用方法
    println!("正方形面积：{}", sq.area());
}
```

你已经见过的 `String::from` 就是关联函数。**约定俗成**：Rust 没有专门的关键字做构造函数，惯用法是提供名为 `new` 的关联函数。

### 7.8 多个 impl 块

一个结构体可以有多个 `impl` 块，语法上完全合法，效果等价于合并：

```rust
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
}

impl Rectangle {   // 第二个 impl 块：完全合法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

单块拆多块在当前场景没有实际必要，但到泛型和 trait 章节会看到：不同 trait bound 下的方法需要放在不同的 impl 块里，这个特性就有了用武之地。

### 7.9 实战：Rectangle 面积计算的四次进化

下面完整走一遍真实开发中的重构过程——**从能跑到优雅，每一步都是 Rust 思维的体现**。这是本节最重要的内容，请逐行体会每次改进"好在哪里"。

#### 版本 1：裸变量（能跑，但语义混乱）

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("面积：{} 平方像素", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

**问题**：`area` 计算的是"一个长方形"的面积，但签名却是两个孤立的 `u32`——width 和 height 本为一体，代码却没有表达这种关联。如果传参顺序写反（`area(height1, width1)`），编译器不会报错，结果悄悄出错。

#### 版本 2：元组（有了一点结构）

```rust
fn main() {
    let rect1 = (30, 50);   // 打包成元组
    println!("面积：{} 平方像素", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

**进步**：参数变成一个，表达了"维度是一个整体"。
**新问题**：`dimensions.0` 是宽还是高？全靠记忆和文档。数据有了结构，却没有**名字**。

#### 版本 3：结构体（语义清晰）

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("面积：{} 平方像素", area(&rect1));   // 借用！不转移所有权
}

fn area(rectangle: &Rectangle) -> u32 {   // 借用，计算完 rect1 还能用
    rectangle.width * rectangle.height
}
```

**进步**：字段有了名字，传错顺序（`width: 50, height: 30`）虽然语法上允许，但字段名让语义一目了然；`area(&rect1)` 用借用避免无谓的所有权转移（第 6 章学的！）。
**遗留问题**：`area` 是个游离函数，可它明明只为 Rectangle 服务——Java/Python 程序员看到这里一定会说："这应该是对象的方法！"

#### 版本 4：方法（行为归于数据）

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("面积：{} 平方像素", rect1.area());   // rect1.area() 比 area(&rect1) 更自然
}
```

**最终形态**：

- `area` 成为 `Rectangle` 的方法，语义上"面积是长方形的行为"；
- `&self` 表明只借用——`main` 里 rect1 的所有权安然无恙；
- 调用点 `rect1.area()` 一眼可读，且 RustRover 在你输入 `rect1.` 时会自动补全所有可用方法。

四次进化的主线：**关联的数据应该有类型，类型的行为应该是方法，不该转移的所有权就用借用**。这就是 Rust 的"结构化思维"。

### 7.10 derive(Debug) 与 {:?} / {:#?}：调试利器

想直接打印整个结构体？

```rust
struct Rectangle { width: u32, height: u32 }

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 是 {}", rect1);
}
```

```text
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
 --> src/main.rs:6:24
  |
6 |     println!("rect1 是 {}", rect1);
  |                         ^^ `Rectangle` cannot be formatted with the default formatter
  |
  = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

**解读**：

- `error[E0277]`：错误码 E0277，"某个 trait 未被实现"——这是你在 Rust 生涯中会见到次数最多的错误码。
- `{}` 占位符要求类型实现 `Display` trait（面向最终用户的美化输出），而 Rust 不替你瞎猜结构体该怎么"美化打印"，所以没有默认实现。
- 注意编译器的**贴心提示**：`you may be able to use {:?}`——它在引导你用调试格式。

`{:?}` 需要类型实现 `Debug` trait。标准库为结构体提供了**自动派生**——加一个属性即可：

```rust
#[derive(Debug)]                        // 自动派生 Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 是 {:?}", rect1);     // 紧凑格式
    println!("rect1 是 {:#?}", rect1);    // 美化格式（# 让字段分行）
}
```

```text
输出：
rect1 是 Rectangle { width: 30, height: 50 }
rect1 是 Rectangle {
    width: 30,
    height: 50,
}
```

另一个调试神器是 `dbg!` 宏：它打印**表达式 + 值 + 代码位置**，并返回表达式的所有权，可以塞进表达式中间：

```rust
fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    dbg!(&rect1);   // 用借用，避免 dbg! 把 rect1 吃掉
}
```

```text
输出（打印到 stderr）：
[src/main.rs:3:5] &rect1 = Rectangle {
    width: 30,
    height: 50,
}
```

> 💡 **提示**：`println!` 输出到 stdout，`dbg!` 输出到 stderr——用 `dbg!` 调试不会污染程序的正常输出。记住组合技：**`#[derive(Debug)]` + `{:?}` + `dbg!`**，覆盖 90% 的日常调试。

### 本章小结

1. 结构体把相关数据组合成命名整体；实例化用 `Struct { 字段: 值, ... }`，访问用点号。
2. **字段初始化简写**、**结构体更新语法 `..base`**（注意它会 move 掉非 Copy 字段！）。
3. **元组结构体**用于类型区分，**单元结构体**用于无数据类型。
4. 结构体字段应存放**拥有所有权**的类型（`String` 而非 `&str`）；存引用需要生命周期标注，那是后话。
5. **方法**定义在 `impl` 块中，首参为 `&self`（只读借用）/`&mut self`（可变借用）/`self`（吃掉所有权）；**关联函数**无 self，用 `::` 调用。
6. 打印结构体：`#[derive(Debug)]` + `{:?}`/`{:#?}`，或 `dbg!` 宏。

### 动手练习

1. **写结构体**：定义一个 `Book` 结构体（title、author、pages、available），为它实现：关联函数 `new`、方法 `summary(&self) -> String`、方法 `borrow(&mut self)`（借出后 available 置为 false，已借出则打印提示）。
2. **所有权追踪**：在结构体更新语法练习中，构造一个 `..base` 之后 `base` 仍然有效的例子和一个失效的例子，并解释差异。
3. **改造实战**：给版本 4 的 Rectangle 添加 `can_hold` 方法和 `square` 关联函数（7.7、7.8 节），写 `main` 验证：`Rectangle::square(3)` 能否被 `Rectangle { width: 30, height: 50 }` 容纳？
4. **思考题**：为什么 `area` 方法用 `&self` 而不是 `self`？如果改成 `self`，`main` 中的代码要做什么调整？这说明了什么设计原则？

---

## 第 8 章 枚举与模式匹配（Enums & Pattern Matching）

如果说所有权是 Rust 的"骨架"，枚举与模式匹配就是 Rust 的"表达力巅峰"。来自 Java/C++ 的读者请做好准备：**Rust 的枚举比你们熟悉的 enum 强大一个数量级。**

### 8.1 枚举定义：每个变体可以携带不同的数据

基础用法与其他语言类似——枚举表达"一个值是若干可能之一"：

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;   // 注意：枚举值通过 :: 访问
let six = IpAddrKind::V6;
```

但 Rust 枚举的真正威力在于：**每个变体可以携带不同类型、不同数量的数据**。

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),   // V4 变体携带四个 u8
    V6(String),           // V6 变体携带一个 String
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

变体甚至可以携带结构体式命名字段，或者嵌套其他枚举：

```rust
enum Message {
    Quit,                        // 无数据（类似单元结构体）
    Move { x: i32, y: i32 },     // 命名字段（类似结构体）
    Write(String),               // 单个值（类似元组结构体）
    ChangeColor(i32, i32, i32),  // 三个值
}
```

> 🆚 **语言对比：Rust enum vs Java enum**
>
> | 能力 | Java enum | Rust enum |
> |---|---|---|
> | 一组命名常量 | ✅ | ✅ |
> | 所有变体携带**同构**数据（每个变体都有相同字段） | ✅（构造器+字段） | ✅ |
> | 不同变体携带**异构**数据（V4 带 4 个数，V6 带 1 个字符串） | ❌（做不到） | ✅ |
> | 编译器强制处理所有变体 | ❌（switch 可漏） | ✅（match 穷尽性检查，见 8.3） |
>
> Java 要表达"不同变体不同数据"，只能靠抽象类 + 继承 + instanceof 一套组合拳（而且漏处理一个子类编译器不拦你）。Rust 枚举把这个需求变成了语言原生能力——这在函数式编程中称为**代数数据类型（Algebraic Data Type）**或**带标签的联合（Tagged Union）**。

枚举也能定义方法（和结构体一样用 impl），见 8.5 节。

### 8.2 Option<T> 与空值安全：消灭十亿美元的错误

2009 年，null 引用的发明者 Tony Hoare 公开道歉，称空引用是他的"十亿美元错误"——它导致了数不清的崩溃、安全漏洞和深夜加班。

Rust 的对策釜底抽薪：**语言里根本没有 null。** 但"值可能不存在"是真实需求，于是标准库提供了 `Option<T>` 枚举：

```rust
enum Option<T> {     // <T> 是泛型，表示"任意类型"，后续章节详解
    Some(T),         // 有一个值，值是 T 类型
    None,            // 没有值
}
```

用法：

```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    let some_char: Option<char> = Some('e');
    let absent_number: Option<i32> = None;   // 类型必须标注：编译器无法猜出 T
}
```

关键设计：**`Option<T>` 和 `T` 是两个不同的类型，编译器禁止混用！**

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;   // ✗ 编译错误
```

```text
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:15
  |
5 |     let sum = x + y;
  |               ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
```

**解读**：`i8` 和 `Option<i8>` 是不同的类型，不能直接相加。你必须**显式处理"可能没有值"的情况**（match、if let、unwrap 等），把 `Option<i8>` 变成 `i8` 之后才能运算。

> 💡 **提示**：这就是 Rust 消灭空指针异常的原理——**"可能没有值"这个事实被编码进了类型系统**。编译器强迫你在每一处处理 None，想忘都忘不了。Java 里 `str.length()` 可能 NPE；Rust 里拿到 `Option<String>` 时你连 `.len()` 都调不了，必须先拆包。

处理 Option 的标准姿势正是下一节的 `match`。

### 8.3 match 表达式：穷尽性检查是杀手锏

`match` 把一个值与一系列模式逐一比较，执行第一个匹配的分支：

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

分支还可以绑定值、执行代码块：

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...其他州
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),   // Quarter 变体携带一个 UsState
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运便士！");   // 分支可以是代码块
            1                        // 最后一行表达式是返回值（无分号）
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {    // state 绑定了变体携带的数据！
            println!("来自 {:?} 的 25 美分！", state);
            25
        }
    }
}
```

#### 8.3.1 穷尽性检查：漏一个分支都编译不过

`match` 最强大之处在于**穷尽性检查（Exhaustiveness）**：必须覆盖所有可能的情况，漏一个就是编译错误：

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        // 忘了 Coin::Dime 和 Coin::Quarter！
    }
}
```

```text
error[E0004]: non-exhaustive patterns: `Coin::Dime` and `Coin::Quarter` not covered
 --> src/main.rs:9:11
  |
9 |     match coin {
  |           ^^^^ patterns `Coin::Dime` and `Coin::Quarter` not covered
  |
  = note: the matched value is of type `Coin`
  = help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern
```

**解读**：

- `error[E0004]`：错误码 E0004，"模式不穷尽"。
- 编译器**精确列出**你漏掉的变体：`Coin::Dime` 和 `Coin::Quarter`。
- `help` 建议使用通配符模式兜底（见 8.4）。

对比 Java 的 switch（历史上）漏 case 不报错、Python 的字典分发漏 key 运行时才 KeyError——**Rust 在编译期就把"漏处理情况"这类 bug 清零了**。当你给枚举新增一个变体时，编译器会指出现有代码中**每一处**需要更新的 match，重构安全无比。

#### 8.3.2 match 处理 Option：标准姿势

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,               // 没有值的情况必须显式处理
        Some(i) => Some(i + 1),     // i 绑定了 Some 里的值
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}, {:?}", six, none);   // 输出：Some(6), None
}
```

### 8.4 `_` 通配符：兜底分支

值很多、只关心少数几种情况时，用 `_` 匹配剩余一切：

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),              // 其他所有数字
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

如果其他情况什么都不想做，用单元值 `()`（空元组）作分支体：

```rust
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),    // 什么都不做，但必须写出来以满足穷尽性
}
```

### 8.5 if let / while let / let-else：更简洁的模式匹配

`match` 要求穷尽，但有时我们只关心**一种**情况，写 `match` + `_ => ()` 太啰嗦。

#### 8.5.1 if let：只关心一种模式

```rust
// 用 match：只关心 Some，None 的情况被迫写兜底
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("最大值配置为 {}", max),
    _ => (),
}

// 用 if let：等价但简洁
if let Some(max) = config_max {
    println!("最大值配置为 {}", max);
}
```

```text
输出（两段代码相同）：
最大值配置为 3
```

`if let 模式 = 表达式` 读作："如果表达式能匹配这个模式，就把绑定引入代码块并执行"。可选 `else` 分支处理不匹配的情况：

```rust
let mut count = 0;
let coin = Coin::Quarter(UsState::Alaska);

if let Coin::Quarter(state) = coin {
    println!("来自 {:?} 的 25 美分！", state);
} else {
    count += 1;   // 非 Quarter 硬币
}
```

#### 8.5.2 while let：模式匹配驱动循环

只要模式持续匹配，循环就继续——典型场景是从栈/队列/通道中逐个弹出元素：

```rust
fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // pop() 返回 Option<i32>：有元素是 Some(x)，空了是 None
    while let Some(top) = stack.pop() {
        println!("弹出：{}", top);
    }
    // 当 pop() 返回 None，模式不匹配，循环自动结束
}
```

```text
输出：
弹出：3
弹出：2
弹出：1
```

#### 8.5.3 let-else：不匹配就提前退出（Rust 1.65+）

`let-else` 是较新的语法，专治"深度嵌套的卫语句"，让错误处理扁平化：

```rust
fn describe_number(x: Option<i32>) {
    // 模式匹配成功 → value 绑定并继续；失败 → 执行 else 块且必须"发散"（return/panic 等）
    let Some(value) = x else {
        println!("没有数字可说");
        return;                 // else 块必须不返回普通值（return/break/panic）
    };

    // 走到这里 value 一定有效，代码保持主流程在最左列
    println!("数字是 {}", value);
    if value > 100 {
        println!("是个大数！");
    }
}

fn main() {
    describe_number(Some(150));
    describe_number(None);
}
```

```text
输出：
数字是 150
是个大数！
没有数字可说
```

> 💡 **提示**：`let-else` 的价值在于保持"主流程代码靠左对齐"。用 if let 写同样逻辑，正常路径会缩进一层；多层校验时代码会一路向右漂移（"箭头代码"）。let-else 把所有异常路径收拢到小的 else 块里，正常路径始终一马平川。

三者怎么选？

| 场景 | 推荐 |
|---|---|
| 需要处理全部情况 | `match`（享受穷尽性检查） |
| 只关心一种情况（可选 else） | `if let` |
| 条件性循环（直到模式不匹配） | `while let` |
| 匹配失败就提前返回/退出的卫语句 | `let-else` |

### 8.6 枚举也能有方法

枚举和结构体一样可以用 `impl` 定义方法：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("文字消息：{}", text),
            Message::ChangeColor(r, g, b) => println!("变色为 RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();   // 输出：文字消息：hello
}
```

方法内部用 `match self` 分派不同变体的行为——这就是 Rust 版"多态"的基本形态（更高级的方案是 trait，后续章节详解）。

### 8.7 实战：构建一个简单状态机

综合运用本章知识，构建一个"红绿灯状态机"——状态用枚举表示，状态转换用方法 + match 实现：

```rust
#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// 返回当前状态应保持的秒数
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 25,
        }
    }

    /// 转换到下一个状态（消费当前状态，返回新状态）
    fn next(self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,    // 红 → 绿
            TrafficLight::Green => TrafficLight::Yellow, // 绿 → 黄
            TrafficLight::Yellow => TrafficLight::Red,   // 黄 → 红
        }
    }

    fn describe(&self) {
        println!("当前：{:?}，保持 {} 秒", self, self.duration());
    }
}

fn main() {
    let mut light = TrafficLight::Red;

    // 模拟状态机运转 6 个周期
    for _ in 0..6 {
        light.describe();
        light = light.next();   // next 消费旧状态、产出新状态
    }
}
```

```text
输出：
当前：Red，保持 30 秒
当前：Green，保持 25 秒
当前：Yellow，保持 5 秒
当前：Red，保持 30 秒
当前：Green，保持 25 秒
当前：Yellow，保持 5 秒
```

这个例子浓缩了本章的精华：

1. **枚举即状态**：`TrafficLight` 把所有可能状态穷尽在类型里——不可能出现"第四种灯"这种非法状态。
2. **行为用 match 分派**：`duration` 和 `next` 各自对全部变体做穷尽匹配。**将来要加"闪烁黄灯"变体？** 编译器会把每一个需要处理的 match 全部指出来，一处都不会漏。
3. **状态转换消费旧状态**：`next(self)` 取得所有权——旧状态用完即弃，防止"同时使用新旧两个状态"的逻辑错误。这是第 7 章 `self` 形态方法的绝佳用例。

> 💡 **提示**：用枚举 + match 实现状态机是 Rust 社区的招牌模式（State Machine Pattern）。相比传统 OOP 用"状态类继承体系"，它更小、更快，且**非法状态根本无法表示**——这就是"让类型系统替你工作"的哲学。

### 本章小结

1. Rust 枚举的每个变体可携带**不同类型、不同数量**的数据（代数数据类型），远超 Java/C++ 枚举。
2. `Option<T>`（`Some(T)` / `None`）替代 null，把"值可能缺失"编码进类型系统，编译器强制处理——消灭了十亿美元的错误。
3. `match` 是强大的模式匹配表达式，**穷尽性检查**（error\[E0004\]）保证所有情况都被处理；模式可绑定变体携带的数据。
4. `_` 通配符兜底其余情况；`if let`、`while let`、`let-else` 是不同场景下更简洁的匹配语法。
5. 枚举可以用 `impl` 定义方法，方法内用 `match self` 分派行为。
6. 枚举 + match 是构建状态机、错误类型、消息传递的天然利器。

### 动手练习

1. **Option 实战**：写一个函数 `fn divide(a: f64, b: f64) -> Option<f64>`：除数为 0 返回 `None`，否则返回 `Some(商)`。用 `match` 和 `if let` 两种方式调用并打印结果。
2. **状态机升级**：给红绿灯状态机增加 `FlashingYellow`（闪烁黄灯）变体。体会一下：加完变体后**不做任何其他修改**，直接编译，看看编译器怎样逐一把所有需要更新的 match 指给你。这就是穷尽性检查的威力。
3. **形状面积**：定义枚举 `enum Shape { Circle(f64), Rectangle(f64, f64), Triangle { base: f64, height: f64 } }`，实现方法 `area(&self) -> f64`，并创建三种形状各一个打印面积。
4. **修复代码**：下面的代码无法编译，找出所有问题并修复（提示：穷尽性 + 所有权）：
   ```rust
   enum Pet { Dog(String), Cat(String) }

   fn main() {
       let pet = Pet::Dog(String::from("旺财"));
       match pet {
           Pet::Dog(name) => println!("狗狗：{}", name),
       }
       match pet {
           Pet::Cat(name) => println!("猫猫：{}", name),
           _ => (),
       }
   }
   ```
5. **思考题**：`Option<T>` 本质只是一个枚举，为什么它能解决 null 解决不了的问题？关键在于语言的哪个机制？

---

> **第二篇完。** 到这里，你已经掌握了 Rust 最核心、最独特的部分：所有权、借用、结构体、枚举与模式匹配。后面章节的一切——trait、泛型、生命周期、智能指针、并发——都建立在本篇的地基之上。如果还有任何模糊之处，**请回头重读第 5、6 章**，那两遍的时间投资会在未来十倍回报你。


---

# 第三篇：进阶核心概念

恭喜你！在入门篇中你掌握了 Rust 的基本语法，在所有权篇中你驯服了 Rust 最独特的所有权、借用与生命周期直觉。从本篇开始，我们将进入 Rust 的"进阶核心概念"——这些是把 Rust 从"能写"提升到"写好"的关键：

- **模块系统**：如何组织一个不断变大的项目
- **错误处理**：Rust 如何用类型系统替代异常
- **泛型与 Trait**：Rust 抽象能力的两大支柱
- **生命周期**：Rust 最著名（也最令人畏惧）的概念，本篇将把它彻底讲透
- **集合**：Vec、String、HashMap 的正确打开方式
- **闭包与迭代器**：函数式风格的零成本抽象

> 💡 **学习建议**：本篇的概念之间联系紧密（比如 Trait 和生命周期经常一起出现）。如果某一处暂时看不懂，先标记下来继续往后读，读完一遍再回头，往往会有"原来如此"的顿悟。

---

## 第 9 章 包、Crate 与模块系统

当你只写 `hello world` 时，一个 `main.rs` 就够了。但真实项目可能有几百个文件。Rust 提供了一套层级分明的代码组织系统：**Package（包）→ Crate（ crate ）→ Module（模块）**。理解这三层，是管理大型 Rust 项目的第一步。

### 9.1 三层概念：Package、Crate、Module

很多初学者被这三个词绕晕，其实它们是纯粹的**包含关系**：

```
Package（包）—— 由 Cargo 管理，对应一个 Cargo.toml
│
├── Crate 1：库 crate（src/lib.rs）       ← 最多 1 个
│   └── 模块树（mod 声明构成）
│       ├── mod a
│       │   ├── mod a1
│       │   └── mod a2
│       └── mod b
│
├── Crate 2：二进制 crate（src/main.rs）   ← 可以有多个
│   └── 模块树
│
└── Crate 3：二进制 crate（src/bin/xxx.rs）
    └── 模块树
```

一句话总结：

| 概念 | 管理工具 | 作用 | 数量限制 |
|------|---------|------|---------|
| Package | Cargo | 构建、测试、共享的一组 crate | —— |
| Crate | rustc | 编译的最小单元，产生库或可执行文件 | 库最多 1 个，二进制可多个 |
| Module | `mod` 关键字 | crate 内部组织代码、控制可见性 | 任意嵌套 |

> 🆚 **对比其他语言**：
> - Java：Package ≈ Maven 模块，Crate ≈ jar 包，Module ≈ package
> - Python：Package ≈ 一个可 pip 安装的项目，Crate ≈ 顶层包，Module ≈ 子包/模块文件
> - Go：Package ≈ Go module，Crate ≈ package，Rust 的 module 在 Go 中没有直接对应物

用 `cargo new` 创建的项目默认同时含有一个**二进制 crate**（`src/main.rs`）。如果再加上 `src/lib.rs`，这个 package 就同时拥有一个库 crate 和一个二进制 crate——这是 Rust 项目的经典布局：核心逻辑放库里，`main.rs` 只做薄薄一层入口。

```
my_project/
├── Cargo.toml
└── src/
    ├── lib.rs      ← 库 crate 的根（crate root）
    ├── main.rs     ← 二进制 crate 的根
    └── bin/        ← 更多二进制 crate
        └── tool.rs
```

> 💡 **什么是 crate root？** 编译器从 crate root（`lib.rs` 或 `main.rs`）开始，顺着 `mod` 声明把整个模块树"拉"进来编译。没有被任何 `mod` 链引用到的 `.rs` 文件**不会被编译**——这是 Rust 与 Java/Python 的重要区别，文件存在不代表会被编译。

### 9.2 mod 声明与文件系统的对应关系

`mod 花园;` 这样的声明告诉编译器："请把对应文件的内容插入到这里"。文件查找有两种风格：

**新风格（Rust 2018+，推荐）**：

```
src/
├── lib.rs              ← 内容：mod garden;
├── garden.rs           ← garden 模块，内容：mod vegetables;
└── garden/
    └── vegetables.rs   ← garden 的子模块
```

**旧风格（mod.rs，仍兼容）**：

```
src/
├── lib.rs
└── garden/
    ├── mod.rs          ← garden 模块
    └── vegetables.rs
```

对应代码：

```rust
// src/lib.rs
mod garden;  // 编译器会找 src/garden.rs 或 src/garden/mod.rs
```

```rust
// src/garden.rs（新风格）
pub mod vegetables;  // 编译器找 src/garden/vegetables.rs
```

> ⚠️ **常见坑**：同一个模块不能同时存在 `garden.rs` 和 `garden/mod.rs`，编译器会报 `file for module found at both ...` 错误。新项目请统一用新风格，它避免了满屏 `mod.rs` 导致编辑器标签页难以区分的问题。
>
> 另一个新手高频坑：新建了 `garden.rs` 却忘了在 crate root 里写 `mod garden;`，然后疑惑"为什么代码不生效"。记住：**文件不会被自动发现，必须由 mod 声明挂载到模块树上**。

### 9.3 路径与可见性：pub 规则

引用模块树中的项有两种路径：

- **绝对路径**：以 crate 名（外部 crate）或 `crate`（当前 crate）开头
- **相对路径**：以 `self`、`super` 或当前模块中的名字开头

Rust 的可见性规则一句话概括：**默认私有，公有需显式 `pub`，且私有是对外部而言，父模块对子模块单向透明**。

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // toast 字段公开
        seasonal_fruit: String, // 该字段仍私有
    }

    impl Breakfast {
        // 因为 seasonal_fruit 私有，必须提供构造函数
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("桃子"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("黑麦");
    meal.toast = String::from("小麦"); // ✅ toast 是 pub 的
    // meal.seasonal_fruit = String::from("蓝莓"); // ❌ 编译错误：字段私有
}
```

注意两个细节：

1. `pub struct` 只让结构体本身可见，**字段仍然各自需要 pub**——这与其他语言"类公开则字段随修饰符"一致，但 Rust 默认全部私有，更保守。
2. `pub enum` 则不同：**枚举的变体随枚举一起公开**，因为枚举的用途就是穷举变体，隐藏变体没有意义。

> 💡 **为什么这样设计？** Rust 的默认私有 + 细粒度 pub 让"最小暴露面"成为零成本默认值。在 Java 里你需要时刻记得写 `private`，在 Rust 里你需要时刻记得写 `pub`——默认方向恰好相反，这是"安全优先"哲学的体现。

### 9.4 use：把路径引入作用域

每次都写全路径太啰嗦，`use` 可以创建快捷方式：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; // 绝对路径

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // 直接用
}
```

**惯例**（社区共识，建议遵守）：

- 引入**函数**：use 到父模块，调用时写 `hosting::add_to_waitlist()`，能看出它不是本地函数
- 引入**结构体/枚举/trait**：use 到本身，直接写 `HashMap::new()`
- 同名冲突时用 `as` 别名或只 use 父模块

```rust
use std::fmt::Result;
use std::io::Result as IoResult; // as 别名解决冲突
```

**`pub use` 重导出**：use 默认只对当前模块有效，加 `pub` 后外部也能通过这条路径访问。常用于把深层内部结构"拍平"成简洁的公开 API：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 重导出：外部用户只需写 restaurant::hosting
pub use crate::front_of_house::hosting;
```

**`self` 与 `super`**：

```rust
use std::io::{self, Write}; // self 指 std::io 本身，等价于两行 use

mod parent {
    fn secret() { println!("parent 的秘密"); }

    pub mod child {
        pub fn call_parent() {
            super::secret(); // super = 父模块，类似文件系统的 ..
        }
    }
}
```

> 🆚 **对比**：`super` 类似文件路径里的 `..`，`self` 类似 `.`，`crate` 类似根目录 `/`。如果你熟悉 Linux 路径，Rust 的模块路径就是同一套直觉。

### 9.5 RustRover 模块导航技巧

在 RustRover 中高效穿梭模块树：

- **Ctrl + 点击**（或 Ctrl+B）：跳到定义，对 `mod`、`use` 路径、函数都有效
- **Ctrl + N**：按名字搜索类型/模块
- **双击 Shift**：全局搜索一切（Search Everywhere）
- **Project 视图**中模块与文件一一对应，重命名文件时用 **Shift+F6（Refactor → Rename）**，RustRover 会同步更新 `mod` 声明
- 对未挂载的文件，RustRover 会提示 `file is not included in module tree`，并提供 quick fix（Alt+Enter）自动补上 `mod` 声明
- **Alt+Enter** 是万能钥匙：自动补 `use`、自动加 `pub`、自动创建模块文件

### 9.6 实战：把单文件重构成多文件模块

假设我们有一个餐厅模拟程序，全部塞在 `lib.rs`：

```rust
// src/lib.rs（重构前：面条式代码）
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() { println!("已加入等候名单"); }
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super 指向 crate root
    }
    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}
```

**重构目标**：每个模块一个文件。步骤：

1. 新建 `src/front_of_house.rs`，把 `front_of_house` 的内容**去掉外层 mod 花括号**后移入：

```rust
// src/front_of_house.rs
pub mod hosting;  // 声明子模块，编译器找 src/front_of_house/hosting.rs
```

2. 新建 `src/front_of_house/hosting.rs`：

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() { println!("已加入等候名单"); }
fn seat_at_table() {}
```

3. `lib.rs` 只保留挂载声明：

```rust
// src/lib.rs（重构后）
mod front_of_house; // 一行挂载整棵子树
mod back_of_house;

fn deliver_order() {}

pub use crate::front_of_house::hosting; // 顺手重导出

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

最终的模块树：

```
crate (lib.rs)
├── front_of_house ── hosting
├── back_of_house
├── deliver_order (fn)
└── eat_at_restaurant (pub fn)
```

> 💡 **重构心法**：把 `mod xxx { ... }` 的花括号内容原样搬进 `xxx.rs`，原位置换成一行 `mod xxx;`；子模块依此递归。文件结构与模块树严格同构，这就是 Rust 模块系统"看目录知架构"的好处。

### 本章小结

- Package 含多个 crate（库最多 1 个），crate 内含模块树
- 模块必须经 `mod` 声明挂载才会被编译；文件布局推荐新风格（`foo.rs` + `foo/` 目录）
- 可见性默认私有；`pub struct` 字段仍需逐个 pub，`pub enum` 变体自动公开
- `use` 创建路径快捷方式；`pub use` 重导出可以拍平 API；`as` 解决重名
- `self` / `super` / `crate` 类比文件系统的 `.` / `..` / `/`

### 动手练习

1. 用 `cargo new garden_shop` 创建项目，添加 `src/lib.rs`，构建这样的模块树：`plants` 模块（含 `Flower` 结构体和 `water()` 函数）、`tools` 模块（含 `Shovel` 结构体）。在 `main.rs` 中通过库 crate 调用它们（提示：二进制 crate 中用 `use garden_shop::...`）。
2. 在 `tools` 模块中用 `pub use` 把内部的 `Shovel` 重导出到 crate 根，让外部可以直接 `garden_shop::Shovel`。
3. 故意制造一个"字段私有却被外部赋值"的错误，观察 RustRover 的报错信息和 Alt+Enter 提供的修复建议。

---

## 第 10 章 错误处理

程序总会出错：文件不存在、网络超时、用户输入非法……一门语言如何处理错误，深刻影响着代码的健壮性。Rust 把错误处理从"运行时的意外"变成了"编译期的契约"。

### 10.1 Rust 的错误处理哲学

Rust 把错误分成两类：

| 类型 | 含义 | 机制 | 例子 |
|------|------|------|------|
| **可恢复错误** | 合理的失败，应该处理并继续 | `Result<T, E>` | 文件不存在、解析失败 |
| **不可恢复错误** | 程序 bug，继续运行无意义 | `panic!` | 数组越界、断言失败 |

> 🆚 **与其他语言对比**：
> - **Java**：`try/catch` 异常。异常是"隐性控制流"——看函数签名根本不知道它会抛什么（受检异常除外，但大家常用 unchecked）。代价是运行时开销和失控的传播。
> - **Go**：`error` 返回值。显式但冗长，`if err != nil` 满天飞的争议从未停止，且忘记检查 err 编译器不会拦你。
> - **Rust**：`Result` 是**类型系统的一部分**。不处理 `Result` 就用不了里面的值，编译器强制你面对错误；同时 `?` 运算符让传播错误只需一个字符。可以说 Rust 取了 Go 的显式，又解决了 Go 的啰嗦。

> 💡 **为什么这样设计？** Rust 没有异常机制，因为异常的开销（栈展开）和隐性（签名不可见）都违背 Rust"零成本、显式"的原则。把错误编码进类型系统，意味着错误处理路径和普通逻辑享受同等的编译期检查。

### 10.2 panic!：不可恢复错误

`panic!` 宏让程序立即崩溃并打印错误信息：

```rust
fn main() {
    panic!("crash and burn");
}
```

运行输出：

```
thread 'main' panicked at src/main.rs:2:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

默认情况下 panic 会**展开（unwind）**栈：逐层清理每个函数的局部数据（调用 Drop）。如果追求更小的二进制体积或更快的崩溃，可以在 `Cargo.toml` 中改为直接终止（abort）：

```toml
[profile.release]
panic = 'abort'
```

**用 backtrace 定位 panic 来源**：当 panic 发生在库代码深处时，设置环境变量看调用栈（Windows PowerShell）：

```powershell
$env:RUST_BACKTRACE=1; cargo run
```

```rust
fn main() {
    let v = vec![1, 2, 3];
    v[99]; // panic: index out of bounds
}
```

backtrace 会列出从 `main` 到 `Vec::index` 的完整调用链，结合源码行号可快速定位。

> ⚠️ **何时该用 panic!**？标准建议：**示例代码、原型、测试**中可以随意 panic；**正式库代码**中 panic 应该只用于"发生了说明是 bug"的场景（如违反内部不变量）。给用户提供库时永远返回 `Result` 而不是 panic——是否崩溃应该由调用方决定。

### 10.3 Result<T, E>：可恢复错误

`Result` 是一个枚举，定义在标准库中：

```rust
enum Result<T, E> {
    Ok(T),   // 成功，携带值
    Err(E),  // 失败，携带错误
}
```

打开文件这个经典例子：

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    // 类型是 Result<File, std::io::Error>
}
```

**用 match 处理**——这是 Result 最基础的消费方式：

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => panic!("打开文件失败: {:?}", other_error),
        },
    };
}
```

嵌套 match 很快变得难看，这正是 `?` 存在的意义（稍后讲）。

### 10.4 unwrap 与 expect：优雅地"偷懒"

```rust
use std::fs::File;

fn main() {
    // Ok 则取出值，Err 则 panic!
    let f = File::open("hello.txt").unwrap();

    // expect 可以自定义 panic 信息，语义更明确
    let f = File::open("hello.txt")
        .expect("hello.txt 应该存在于项目根目录");
}
```

> ⚠️ **unwrap 的正确使用姿势**：unwrap 不等于坏习惯。以下场景完全可以 unwrap：
> 1. **测试代码**：测试挂了本来就该 panic
> 2. **原型/示例**：快速验证想法
> 3. **你能证明不会失败**：如 `"123".parse::<u32>().unwrap()`，但请用 `expect` 写下你的理由，给未来的维护者（可能就是你自己）留个说明
>
> 生产代码中对用户输入、网络、文件等不可控来源 unwrap，等于埋下了定时炸弹。

### 10.5 ? 运算符：错误传播的优雅写法

函数本身可能失败时，应该返回 `Result` 把错误交给调用方。`?` 运算符让这件事只需一个字符：

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // Err 则直接 return
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;     // 同样
    Ok(username) // 一切顺利，包上 Ok 返回
}
```

`?` 的语义：**如果是 Ok，取出值继续；如果是 Err，立即从当前函数 return 这个错误**（且会通过 `From` trait 自动转换错误类型）。

> 🆚 **对比 Go**：
>
> ```go
> // Go：三段 if err != nil
> f, err := os.Open("hello.txt")
> if err != nil { return "", err }
> n, err := f.Read(buf)
> if err != nil { return "", err }
> ```
>
> ```rust
> // Rust：链式一行
> let mut s = String::new();
> File::open("hello.txt")?.read_to_string(&mut s)?;
> ```
>
> 更极致的写法：`std::fs::read_to_string("hello.txt")?` 一行搞定。Rust 保留了 Go 的显式性（错误在签名里一目了然），又把样板代码压缩到一个字符。

**`?` 与 Option**：`?` 也能用于 `Option`——`Some` 取值继续，`None` 直接返回 `None`：

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // 没有第一行 → 返回 None；有则取最后一个字符
}
```

> ⚠️ `?` 只能用于返回值类型兼容的函数：返回 `Result` 的函数里才能对 `Result` 用 `?`，返回 `Option` 的函数里才能对 `Option` 用 `?`。`main` 函数默认返回 `()`，想在里面用 `?`，可以把签名改成 `fn main() -> Result<(), Box<dyn std::error::Error>>`。

### 10.6 自定义错误类型

真实的库会有多种失败原因。手写一个错误类型需要实现 `Display` 和 `Error` trait：

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    InvalidInput(String),
    Database(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound(item) => write!(f, "未找到: {}", item),
            AppError::InvalidInput(msg) => write!(f, "非法输入: {}", msg),
            AppError::Database(msg) => write!(f, "数据库错误: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
```

实用项目中几乎没人手写这些样板，**thiserror** 库是事实标准：

```toml
[dependencies]
thiserror = "1"
```

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("未找到: {0}")]
    NotFound(String),

    #[error("非法输入: {0}")]
    InvalidInput(String),

    // from 属性自动生成 From<io::Error>，配合 ? 自动转换
    #[error("IO 错误")]
    Io(#[from] std::io::Error),
}

fn load_config() -> Result<String, AppError> {
    let content = std::fs::read_to_string("config.toml")?; // io::Error 自动转成 AppError
    Ok(content)
}
```

> 💡 **选型建议**：写**库**用 `thiserror`（错误类型精确，方便调用方 match）；写**应用**用 `anyhow`（一个 `anyhow::Result` 吞下所有错误，配上 `.context()` 附上上下文，开发效率极高）。这是 Rust 社区最主流的两种姿势。

### 10.7 错误处理最佳实践速查表

| 场景 | 推荐手段 | 理由 |
|------|---------|------|
| 写库，调用方需要区分错误 | 自定义错误枚举 + thiserror | 类型精确，调用方可 match |
| 写应用，主要把错误报给用户 | anyhow + `?` + `.context()` | 省去定义错误类型的成本 |
| 代码 bug / 内部不变量被破坏 | `panic!` / `assert!` / `unreachable!` | 继续运行可能产生脏数据 |
| 测试、示例、原型 | `unwrap` / `expect` | 简洁，失败即 panic 正好 |
| 可预期的失败（文件、网络、解析） | 返回 `Result`，用 `?` 传播 | 把决策权交给调用方 |
| "理论上不可能失败"但类型上是 Result | `expect("理由")` | 留下文档化的断言 |

### 本章小结

- Rust 无异常：`Result` 处理可恢复错误，`panic!` 处理不可恢复错误（bug）
- `unwrap`/`expect` 是"失败就 panic"的快捷方式，测试和原型中合理使用
- `?` 把"遇错即返回"压缩成一个字符，还能自动转换错误类型
- 自定义错误：手写 Display + Error，或直接用 thiserror；应用层推荐 anyhow
- 核心思想：**让错误处理成为类型系统强制面对的契约，而非运行时的意外**

### 动手练习

1. 写一个函数 `parse_port(s: &str) -> Result<u16, AppError>`：解析 1~65535 的端口号，非法输入返回自定义错误（用 thiserror 定义 `NotANumber` 和 `OutOfRange` 两个变体）。在 `main` 中用 match 分别打印成功和各类失败。
2. 把练习 1 的 match 改成 `?` 传播：`main() -> Result<(), Box<dyn Error>>`。
3. 故意写 `v[10]` 触发 panic，用 `RUST_BACKTRACE=1` 运行，找到 backtrace 中指向你自己代码的那一行。

---

## 第 11 章 泛型与 Trait

泛型让我们写出"对多种类型都成立"的代码，trait 让我们描述"类型能做什么"。两者组合，构成了 Rust 抽象能力的核心——而且这一切在运行时的成本是**零**。

### 11.1 泛型基础

**函数中的泛型**：找出切片中的最大值，对 `i32` 和 `char` 都想用：

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {   // 需要 T 可比较 → PartialOrd
            largest = item;   // 需要 T 可复制 → Copy
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大数字: {}", largest(&numbers)); // 100

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大字符: {}", largest(&chars)); // y
}
```

`<T: PartialOrd + Copy>` 读作"类型参数 T，要求实现了 PartialOrd 和 Copy"。尖括号里的要求叫 **trait bound**，11.3 节细讲。

**结构体与枚举中的泛型**：

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// 标准库中两个最著名的泛型枚举，你已经在用了：
// enum Option<T> { Some(T), None }
// enum Result<T, E> { Ok(T), Err(E) }

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{:?} {:?}", integer, float);
}
```

**方法中的泛型**：impl 块上也要声明 `<T>`，这个细节新手常漏：

```rust
impl<T> Point<T> {           // impl 后的 <T> 声明类型参数
    fn x(&self) -> &T {      // 之后 Point<T> 中的 T 才能用
        &self.x
    }
}

impl Point<f32> {            // 也可以只为某个具体类型实现方法
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### 11.2 单态化：零成本抽象的秘密

泛型在运行时有没有开销？Rust 的答案：**没有**。编译器在编译期为每个实际用到的具体类型生成一份专门代码，这个过程叫**单态化（monomorphization）**：

```rust
// 你写的：
let integer = Some(5);      // Option<i32>
let float = Some(5.0);      // Option<f64>

// 编译器实际生成的（概念上）：
// enum Option_i32 { Some(i32), None }
// enum Option_f64 { Some(f64), None }
```

> 🆚 **三种泛型实现路线对比**：
>
> | 语言 | 机制 | 运行时开销 | 特点 |
> |------|------|-----------|------|
> | Java | 类型擦除 | 有（装箱/拆箱、强制转换） | `List<Integer>` 运行时只是 `List`，泛型信息被擦掉 |
> | C++ | 模板实例化 | 无 | 与 Rust 类似，但错误信息臭名昭著，且无边际检查 |
> | Rust | 单态化 | 无 | 编译期检查 trait bound，报错友好；代价是编译变慢、二进制变大 |
>
> 这就是"**零成本抽象**"的含义：你不用为不用的抽象付钱，用的抽象也不比手写具体代码慢。

### 11.3 Trait：定义共享行为

trait 定义"某类型能做什么"的一组方法签名：

```rust
// 定义 trait：类似接口
pub trait Summary {
    fn summarize(&self) -> String; // 只有签名，没有实现
}

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

// 为类型实现 trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - {}", self.headline, self.content)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}
```

**默认实现**：trait 可以提供默认方法体，实现类可选择覆盖：

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    // 默认实现可以调用同 trait 的其他方法
    fn summarize(&self) -> String {
        format!("(阅读更多，来自 {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize 用默认实现即可
}
```

> 🆚 **Trait vs Interface**：
> - **Java interface**：类在定义处声明 `implements`，是"名义上的"声明式关系。无法给已有类（比如 String）追加接口实现。
> - **Go interface**：**结构化（鸭子类型）**——类型只要方法集匹配就自动满足接口，无需声明。灵活但隐式，改方法签名时不知道影响了谁。
> - **Rust trait**：**名义性 + 事后实现**——必须显式写 `impl Summary for Tweet`，但可以为任何类型补 impl（受孤儿规则约束）。既显式又灵活。

### 11.4 Trait Bound 与 where 子句

用 trait bound 约束泛型参数：

```rust
// 参数是"任何实现了 Summary 的类型"
pub fn notify(item: &impl Summary) {
    println!("突发新闻! {}", item.summarize());
}

// 上面的语法糖展开后等价于：
pub fn notify_verbose<T: Summary>(item: &T) {
    println!("突发新闻! {}", item.summarize());
}
```

两种写法何时用哪个？**参数需要多个泛型、或关系复杂时用完整形式**：

```rust
// 想保证两个参数同类型，impl Trait 做不到，必须用泛型：
pub fn notify_pair<T: Summary>(item1: &T, item2: &T) { /* ... */ }

// 多个 bound 用 + 连接：
pub fn notify2<T: Summary + std::fmt::Display>(item: &T) { /* ... */ }

// bound 太多时，where 子句让签名清爽：
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    42
}
```

**返回 impl Trait**：返回"某个实现了 trait 的类型"而不暴露具体类型：

```rust
fn make_summarizable() -> impl Summary {
    Tweet {
        username: String::from("rustacean"),
        content: String::from("Rust 真好玩"),
    }
}
```

> ⚠️ **impl Trait 返回的限制**：只能返回**一种**具体类型。`if 条件 { 返回 Tweet } else { 返回 NewsArticle }` 无法通过编译——因为单态化要求返回类型在编译期唯一确定。需要运行时多态？请用 `Box<dyn Summary>`（见 11.7）。

### 11.5 孤儿规则（Coherence）

你可以给 `Vec<T>`（外部类型）实现 `Summary`（本地 trait），也可以给 `Tweet`（本地类型）实现 `Display`（外部 trait），但**不能给 `Vec<T>` 实现 `Display`**——类型和 trait 都是外部的。

> 💡 **孤儿规则**：只有当 trait 或类型**至少有一个定义在当前 crate** 时，才允许写 impl。为什么？防止不同 crate 为同一对"类型+trait"写出冲突的实现，保证整个生态中 impl 是**全局唯一**的。这叫 coherence（一致性）。
>
> 想绕过？用 **newtype 模式**：定义本地包装类型 `struct Wrapper(Vec<String>)`，再为 Wrapper 实现 Display。

### 11.6 常用标准 trait 巡礼

| Trait | 作用 | 备注 |
|-------|------|------|
| `Debug` | `{:?}` 调试格式化 | 几乎总是 `#[derive(Debug)]` |
| `Display` | `{}` 面向用户的格式化 | 需手写，不可 derive |
| `Clone` | 显式深拷贝 `.clone()` | 通常可 derive |
| `Copy` | 隐式按位复制（栈上小类型） | 需 Clone 为前提；Copy 后原变量仍可用 |
| `Drop` | 离开作用域时的清理逻辑 | 类似析构函数，手写管理资源 |
| `Default` | `::default()` 默认值 | 可 derive（要求所有字段 Default） |
| `From`/`Into` | 类型转换 | 实现 From 自动获得 Into；`?` 的错误转换靠它 |
| `PartialEq`/`Eq` | 相等比较 | 可 derive |
| `PartialOrd`/`Ord` | 排序比较 | 可 derive |

**运算符重载**就是实现对应 trait（定义在 `std::ops`）：

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point; // 关联类型：相加的结果类型

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

### 11.7 静态分发 vs 动态分发

同样写"接受任何 Summary"，有两条路：

```rust
// 静态分发：泛型，编译期为每个类型生成专门代码
fn notify_static(item: &impl Summary) { /* ... */ }

// 动态分发：trait object，运行时通过虚表（vtable）查方法
fn notify_dynamic(item: &dyn Summary) { /* ... */ }

// 异构集合必须用动态分发：
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(Tweet { /* ... */ }),
    Box::new(NewsArticle { /* ... */ }),
];
```

| 维度 | 泛型（静态分发） | dyn Trait（动态分发） |
|------|-----------------|----------------------|
| 分发时机 | 编译期（单态化） | 运行时（vtable 指针查找） |
| 运行时开销 | 零 | 一次间接跳转，且阻碍内联优化 |
| 二进制体积 | 每种类型一份代码（可能膨胀） | 一份代码 |
| 异构集合 | ❌ 单一具体类型 | ✅ `Vec<Box<dyn Trait>>` |
| 返回不同类型 | ❌ | ✅ |
| trait 要求 | 无限制 | 必须**对象安全** |

**对象安全（object safety）规则**：trait 能做成 `dyn` 的前提是：

1. 所有方法**没有泛型参数**（vtable 里没法放无穷多个单态化版本）
2. 返回类型**不是 `Self`**（运行时不知道 Self 是多大）
3. 方法没有 `Self` 类型的参数（有例外，如接收者 `&self`）

典型例子：`Clone` 的 `fn clone(&self) -> Self` 返回 `Self`，所以 `dyn Clone` 不合法；`Iterator` 因为泛型方法的存在也不能直接做 trait object。

> 💡 **怎么选？** 默认用泛型（更快、更灵活）。只有当你需要**异构集合**、**插件架构**或**控制编译时间/二进制体积**时，才转向 `dyn`。

### 本章小结

- 泛型通过**单态化**实现零成本抽象，无运行时开销
- trait = 名义性的显式接口，支持默认实现，可以为已有类型补实现
- trait bound 用 `<T: Trait>` 或 `where` 子句；`impl Trait` 可用于参数和返回值
- 孤儿规则：trait 和类型至少一个是本地的
- 静态分发（泛型）是默认选择；需要异构时用 `Box<dyn Trait>`，注意对象安全规则

### 动手练习

1. 定义 trait `Area { fn area(&self) -> f64; }`，为 `Circle { radius: f64 }` 和 `Rectangle { width: f64, height: f64 }` 实现它。写一个泛型函数 `print_area<T: Area>(shape: &T)` 打印面积。
2. 把练习 1 改成 `Vec<Box<dyn Area>>` 存一个圆和一个矩形，遍历打印总面积。思考：为什么这里泛型做不到？
3. 为 `Circle` 实现 `Add`（半径相加得到新圆），并测试 `c1 + c2`。

---

## 第 12 章 生命周期（ Lifetimes ）

这是 Rust 最负盛名的一章，也是无数人"从入门到放弃"的关卡。但我要告诉你一个秘密：**生命周期本身并不难，难的是没人告诉你它到底是什么**。本章的目标是把生命周期彻底讲透——读完你应该能做到：看到生命周期报错不慌，知道编译器在担心什么，也知道怎么修。

### 12.1 从一个"必然失败"的程序说起

回顾所有权篇中的悬垂引用问题。看这段代码：

```rust
fn main() {
    let r;                  // ---------+-- r 的生命周期开始
    {                       //          |
        let x = 5;          // -+-- x  | 
        r = &x;             //  |      |
    }                       // -+   x 的生命周期结束（离开作用域）
    //                      //          |
    println!("r: {}", r);   // ❌ r 指向已被释放的 x！
}                           // ---------+-- r 的生命周期结束
```

用 ASCII 图把"值活了多久"画出来：

```
  代码行                  r 有效区间        x 有效区间
┌─────────────────┐
│ let r;          │    ┌──────────┐
│     let x = 5;  │    │          │   ┌────┐
│     r = &x;     │    │  r 借用 x │──→│ x=5│
│ }               │    │          │   └────┘ ← x 死了！
│ println!(r)     │    │  还在用？ │   ✗ 指向坟墓
└─────────────────┘    └──────────┘
```

编译器报错：

```
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:5
  |
4 |         let x = 5;
5 |         r = &x;
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("r: {}", r);
  |                       - borrow later used here
```

> 💡 **心智模型 #1**：生命周期就是**"一个值在内存中存活的代码区间"**。借用检查器（borrow checker）的全部工作，就是验证**每一个引用的使用点，都落在被引用值的存活区间内**。它是静态的、编译期的分析，不产生任何运行时代码。

这段代码的问题在**同一作用域内**就能被发现。但跨函数时，编译器需要额外信息——这就是生命周期标注存在的理由。

### 12.2 为什么函数签名需要生命周期标注

考虑一个看似无害的函数：返回两个字符串切片中较长的那个。

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

编译：

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |             ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value,
          but the signature does not say whether it is borrowed
          from `x` or `y`
```

**编译器到底在纠结什么？** 站在编译器的角度想：返回值是个引用，它要么借自 `x`，要么借自 `y`——具体是哪个，取决于 `if` 的运行时分支！编译期根本无从知晓。

不知道返回值的"血缘"，就无法验证调用方代码的安全性：

```rust
fn main() {
    let string1 = String::from("很长的字符串");
    let result;
    {
        let string2 = String::from("短");
        result = longest(string1.as_str(), string2.as_str());
        // 如果运行时走了 else 分支，result 就借用了 string2
    } // string2 在这里被释放
    println!("较长的是: {}", result); // result 可能是悬垂的！
}
```

> 💡 **心智模型 #2**：函数是借用检查的**边界**。函数体内编译器全知全能，但函数签名是一份"合同"——编译器检查函数体时只看签名，检查调用方时也只看签名。如果返回引用的来源在签名里说不清楚，合同就不成立，编译器拒绝签字。

于是我们要在签名里写明：**返回值的生命周期与参数的生命周期有什么关系**。这就是 `&'a` 标注。

### 12.3 生命周期标注语法

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

逐部分拆解：

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
//        └──┬──┘   └┬┘           └┬┘       └┬┘
//      声明生命周期  x 至少活     y 至少活   返回值的存活期
//      参数 'a      够 'a        够 'a      = 'a
```

- `'a` 读作"生命周期 a"，只是一个**名字**（惯用 a、b、c）
- `<'a>` 在函数名后声明这个生命周期参数，就像泛型 `<T>` 声明类型参数
- `&'a str` 读作"一个至少能活 `'a` 这么久的字符串切片"

**最关键、最反直觉的一点**：

> ⚠️ **生命周期标注不改变任何值的实际存活时间！** 它是**描述**，不是**命令**。加 `'a` 不会让任何变量活得更久——它只是在签名里声明"这些引用之间的存活期存在如下关系"，好让编译器能验证调用方。类比：泛型 `<T>` 不会创造类型，只是给"某个类型"起了名字；`'a` 不会创造生命周期，只是给"某段存活区间"起了名字。

加上标注后，`longest` 的合同变成："你给我两个引用，它们都得至少活 `'a`；我保证返回值也只活 `'a` 以内"。当两个参数存活期不同时，`'a` 取两者的**交集**（较短的那个）：

```rust
fn main() {
    let string1 = String::from("长字符串"); // 活很久 ──────────────┐
    let result;                                                 // │
    {                                                           // │
        let string2 = String::from("短");  // 活很短 ┌──┐          // │
        result = longest(&string1, &string2); // 'a = 两者交集     // │
    }                                        // └──┘ string2 死   // │
    println!("{}", result); // ❌ E0597：result 借的值已死          // │
}                            // ──────────────────────────────────┘
```

编译器现在能理直气壮地报错了：合同写明 result 最多活到 `'a`（string2 死的那一行），而你在那之后还在用。

再看一个"只需标注部分参数"的例子：

```rust
// 返回值只借自 x，与 y 无关 → y 不需要生命周期标注
fn first_half<'a>(x: &'a str, y: &str) -> &'a str {
    &x[..x.len() / 2]
}
```

这样合同更宽松：`y` 活得再短也无所谓。调用方获得的自由度更大——**精确的标注 = 更宽松的调用约束**，这是写库的进阶直觉。

### 12.4 生命周期省略规则（Elision Rules）

你会疑惑：`fn first(s: &str) -> &str` 这种代码天天见，怎么不用标注？因为编译器内置了三条**省略规则**，覆盖常见模式时自动补上标注。规则按顺序应用，全部用完仍无法确定返回引用的来源，才报错要求你手写。

- **规则 1（输入各自独立）**：每个引用参数获得自己独立的生命周期。
  `fn f(x: &str, y: &str)` → `fn f<'a, 'b>(x: &'a str, y: &'b str)`
- **规则 2（单输入直通）**：如果**只有一个**输入生命周期，它赋给所有输出引用。
  `fn f(x: &str) -> &str` → `fn f<'a>(x: &'a str) -> &'a str`
- **规则 3（self 优先）**：方法中如果有 `&self` 或 `&mut self`，输出的生命周期赋为 self 的。

> 💡 **为什么要省略规则？** 早期 Rust 必须全部手写，社区发现绝大多数函数都落在三种机械模式里，于是把"人人都在重复写的标注"变成了编译器默认。据统计约 87% 的函数签名能被省略规则覆盖。它是纯语法糖，不改变语义。

**实战推导：先猜，再揭晓**

遮住下文，先用三条规则推一遍这些签名能否编译：

```rust
// 题目 1
fn first_word(s: &str) -> &str;

// 题目 2
fn get_first(s1: &str, s2: &str) -> &str;

// 题目 3
fn announce_and_return(s: &str, announcement: &str) -> &str;
// 函数体: print!(announcement); 返回 s

// 题目 4
impl MyStruct {
    fn get_name(&self, prefix: &str) -> &str;
}
```

**揭晓**：

```
题目 1：✅ 编译通过
  规则 1 → fn first_word<'a>(s: &'a str) -> &str
  规则 2 → 只有一个输入，输出得 'a：
           fn first_word<'a>(s: &'a str) -> &'a str
  有唯一解，省略成立。

题目 2：❌ E0106 missing lifetime specifier
  规则 1 → fn get_first<'a, 'b>(s1: &'a str, s2: &'b str) -> &str
  规则 2 → 有两个输入，不适用
  规则 3 → 没有 self，不适用
  输出引用到底活 'a 还是 'b？不知道 → 必须手写。

题目 3：❌ 同样 E0106！
  注意：省略规则只看签名，不看函数体！
  即使函数体里明明返回的是 s，签名有两个输入引用就必须手写：
  fn announce_and_return<'a>(s: &'a str, announcement: &str) -> &'a str

题目 4：✅ 编译通过
  规则 1 → fn get_name<'a, 'b>(&'a self, prefix: &'b str) -> &str
  规则 3 → 有 &self，输出得 self 的生命周期：
           fn get_name<'a, 'b>(&'a self, prefix: &'b str) -> &'a str
  含义：返回值借自 self，与 prefix 无关。
```

> ⚠️ **题目 3 是最常见的心智盲区**："函数体里清清楚楚返回 s，编译器为什么看不出来？"因为借用检查把签名当合同、**只看签名**——这样设计保证了函数体重构不会悄悄改变对外合同，也保证检查是局部的、快速的。

### 12.5 结构体中的生命周期

结构体里存引用，必须标注生命周期：

```rust
// 含义：ImportantExcerpt 实例不能比它借用的 str 活得更久
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("多年以后，面对行刑队……");
    let first_sentence = novel.split('，').next().unwrap();

    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("摘录: {}", excerpt.part);
} // 合法：novel 比 excerpt 活得久
```

用图表示这个约束：

```
novel (String)      ──────────────────────────────┐ 活着
first_sentence (&str) ──借用 novel──┐              │
excerpt { part }     ──存 first_sentence──┐        │
                                          ↓        │
约束：excerpt 的存活区间 ⊆ first_sentence 的存活区间 ⊆ novel 的存活区间
```

> 💡 **心智模型 #3**：结构体上的 `<'a>` 是在声明一个**不变量**："本结构体实例的存活期不超过其内部引用的来源"。违反它（比如试图把 excerpt 存到比 novel 更长寿的结构里）会被编译器拦下。
>
> 经验法则：**结构体里存引用，先问自己三遍"真的需要吗"**。大多数应用代码应该存 `String` 这样的拥有型数据，把引用留给性能敏感的热路径。存引用等于把生命周期传染给所有使用这个结构体的地方。

### 12.6 方法中的生命周期

```rust
impl<'a> ImportantExcerpt<'a> {
    // 规则 3 生效：返回值借自 &self，无需手写
    fn level(&self) -> i32 {
        3
    }

    // 两个输入（self 和 announcement），规则 3 救场：
    // 返回值被绑定到 self 的生命周期
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("请注意: {}", announcement);
        self.part
    }
}
```

`impl<'a> ImportantExcerpt<'a>` 的写法与泛型如出一辙：impl 后声明 `'a`，类型名后使用它。

### 12.7 'static 生命周期

`'static` 表示"存活期贯穿整个程序运行"。最常见的例子是字符串字面量：

```rust
let s: &'static str = "我住在程序的二进制文件里，永生";
```

> ⚠️ **两个 `'static` 不要混淆**：
> 1. `&'static T`：引用指向的数据活到程序结束（如字面量、 leaked 内存）
> 2. `T: 'static`（trait bound）：T 是拥有型类型，或不含任何非 static 引用——常见于 `thread::spawn` 和 `Box<dyn Trait>` 的默认要求
>
> 另一个常见误用：遇到生命周期报错就加 `'static` 或到处 `.clone()`/`String` 化。`&'static str` 参数的函数几乎不可能是你想要的——它要求调用方只能传字面量。报错时正确的思路是**理清引用的来源**，而不是升级存活期。

### 12.8 综合示例：泛型 + trait bound + 生命周期

它们共处一个签名时的完整形态：

```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("公告: {}", ann);
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = "hello";
    let s2 = "rust";
    let result = longest_with_announcement(s1, s2, "比赛开始");
    println!("较长者: {}", result);
}
```

输出：

```
公告: 比赛开始
较长者: hello
```

读法：`<'a, T>` 声明一个生命周期参数和一个类型参数；`where T: Display` 约束 T；`'a` 描述 x、y 与返回值的关系。看起来拥挤，但每个符号各司其职——生命周期管"引用活多久"，泛型管"是什么类型"，trait bound 管"能做什么"。

### 12.9 真实报错实战解析

**案例 1：E0106 missing lifetime specifier**

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```

**解读**：两个输入引用 + 无 self，三条省略规则用尽仍不知返回值血缘。
**修复**：想清楚返回值到底借自谁。借自两者→共用一个 `'a`；只借自 x→只给 x 和返回值标注。

**案例 2：E0597 does not live long enough**

```rust
fn main() {
    let result;
    {
        let s2 = String::from("短");
        result = longest("长字符串", &s2);
    }
    println!("{}", result);
}
```

```
error[E0597]: `s2` does not live long enough
  |
  |         result = longest("长字符串", &s2);
  |                                      ^^^ borrowed value does not live long enough
  |     }
  |     - `s2` dropped here while still borrowed
```

**解读**：签名承诺返回值只活 `'a`（= 两参数中较短者 = s2 的存活期），s2 死后 result 就失效了。
**修复**：①把 println 挪进内层作用域；②或让 s2 活得和 result 一样久（把它移出内层块）；③或改 API 返回 `String`（拥有所有权，无生命周期纠缠）。

**案例 3：返回局部变量的引用（E0515 / E0106）**

```rust
fn make_greeting() -> &str {
    let s = String::from("你好");
    &s
}
```

```
error[E0515]: cannot return reference to local variable `s`
  |
3 |     &s
  |     ^^ returns a reference to data owned by the current function
```

**解读**：`s` 是函数创建的，函数结束时就被 Drop，返回它的引用必定悬垂。这类错误编译器甚至不需要你标注——因为**没有任何生命周期标注能救它**。
**修复**：返回 `String`（转移所有权），或改为接受 `&str` 参数返回其切片。

**案例 4：结构体放错了地方**

```rust
struct Holder<'a> { data: &'a str }

fn main() {
    let h;
    {
        let temp = String::from("临时");
        h = Holder { data: &temp };
    }                        // temp 死在这里
    println!("{}", h.data);  // ❌ E0597
}
```

**解读**：`Holder<'a>` 合同要求 h 不能比 temp 长寿，但 h 声明在 temp 的作用域之外。
**修复**：要么让 temp 活更久，要么 `Holder` 改存 `String`。

> 💡 **生命周期排错通用心法**：
> 1. **找血缘**：报错信息里 "borrowed from" / "dropped here" / "later used here" 三个位置，画出存活区间图
> 2. **问自己**：这个引用的源头值是谁？它活到哪一行？使用点在哪一行？
> 3. **两条出路**：要么**延长源头**（调整作用域、提前声明），要么**切断借用**（返回拥有型数据、`.to_owned()`、改 API）
> 4. **警惕假动作**：加 `'static`、乱加 `'a`、到处 clone 都是掩盖问题而非解决问题

### 本章小结

- 生命周期 = 值的存活代码区间；借用检查器验证所有引用使用点都落在源头存活区间内
- **标注是描述关系，不改变实际存活时间**——这是全章最重要的一句话
- 函数签名是借用检查的合同；跨函数传递引用时，返回值的血缘必须写在签名里
- 三条省略规则：输入各自独立 → 单输入直通 → self 优先；规则用尽无解才要求手写
- 结构体存引用要标 `<'a>`，它声明"实例不比借来的数据长寿"
- `&'static str` 是字面量专属；`T: 'static` 意味着"不含短寿引用"
- 排错心法：找血缘 → 画区间 → 延长源头或切断借用

### 动手练习

1. **先猜后验证**：以下函数哪些能编译？写出省略规则推导过程。
   ```rust
   fn a(x: &i32) -> &i32;
   fn b(x: &i32, y: &i32) -> &i32;
   fn c(x: &i32, y: &str) -> &i32;  // 函数体返回 x
   ```
2. 写一个函数 `longest<'a>(x: &'a str, y: &str) -> &'a str`，功能与文中 `longest` 相同但只从 x 借用（提示：当 y 更长时也得返回 x 的某个切片……这个签名其实表达不了原语义，体会一下"标注刻画血缘"的含义）。
3. 定义 `struct TextSplitter<'a> { text: &'a str, delimiter: char }`，为它实现 `fn next_chunk(&self) -> Option<&str>` 返回第一个分隔符之前的部分。确保方法签名利用省略规则 3。
4. 故意重现本章案例 2 的 E0597 报错，然后用三种不同的修复方式各修一遍，对比哪种最符合你的实际需求。

---

## 第 13 章 常用集合

标准库的集合存放"数量可变"的数据——与内嵌的数组、元组不同，它们的数据在**堆上**，大小可在运行时变化。本章讲最常用的三个：Vec、String、HashMap。

### 13.1 Vec<T>：动态数组

**创建与基本操作**：

```rust
fn main() {
    // 两种创建方式
    let v1: Vec<i32> = Vec::new();        // 空 Vec 需要类型标注
    let v2 = vec![1, 2, 3];               // vec! 宏，编译器可推断类型

    // 增
    let mut v = Vec::new();
    v.push(5);    // 尾部追加
    v.push(6);

    // 读：两种方式，安全性截然不同
    let third: &i32 = &v[1];            // 越界会 panic!
    let third: Option<&i32> = v.get(100); // 越界返回 None，优雅
    match v.get(100) {
        Some(x) => println!("第 100 个元素: {}", x),
        None => println!("没有第 100 个元素"),
    }

    // 删
    v.pop(); // 弹出尾部，返回 Option<i32>
}
```

**`[]` vs `.get()`** 是经典选择题：确定不越界用 `[]`（简洁），索引来自用户/计算结果用 `.get()`（安全降级）。

**遍历与借用规则**：

```rust
fn main() {
    let mut v = vec![100, 32, 57];

    // 不可变遍历
    for i in &v {
        println!("{}", i);
    }

    // 可变遍历：每个元素加 50
    for i in &mut v {
        *i += 50; // * 解引用后才能修改
    }
    println!("{:?}", v); // [150, 82, 107]
}
```

> ⚠️ **遍历中不能增删元素**：
>
> ```rust
> let mut v = vec![1, 2, 3];
> for i in &v {
>     v.push(4); // ❌ 编译错误！
> }
> ```
>
> 为什么？`push` 可能触发扩容——Vec 在堆上重新分配内存，旧内存被释放，而 `for` 持有的迭代器还指着旧地址。这又是借用检查器在保护你：迭代持有 `&v` 的不可变借用，`push` 需要 `&mut v`，二者冲突，编译期直接拦截。在 C++ 里这是经典的迭代器失效 UB，在 Rust 里是一条友好的编译错误。

**用枚举实现"异构" Vec**：

Vec 只能存一种类型。需要混合类型时，定义枚举把所有变体收进一个类型：

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("蓝色")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("整数: {}", i),
            SpreadsheetCell::Float(f) => println!("浮点: {}", f),
            SpreadsheetCell::Text(s) => println!("文本: {}", s),
        }
    }
}
```

> 🆚 **对比**：Python/Java 的列表天然异构（动态类型/万物皆 Object），代价是运行时类型检查。Rust 用枚举把"可能是哪几种"显式枚举出来，编译器强制你用 match 处理全部情况——安全性换一点样板代码。若类型集合无法预先确定（插件场景），则用上一章的 `Vec<Box<dyn Trait>>`。

### 13.2 String：UTF-8 编码的文本

Rust 新手被 String 绊倒的概率极高，根源在一点：**String 是 UTF-8 编码的字节序列的包装**（本质上 `Vec<u8>` 加"保证内容是合法 UTF-8"的约束）。

**为什么不能按下标索引**：

```rust
fn main() {
    let s = String::from("你好");
    // let h = s[0]; // ❌ 编译错误：String cannot be indexed
}
```

> 💡 **为什么这样设计？** "你好" 在 UTF-8 中占 6 个字节：`你` = `[228, 189, 160]`，`好` = `[229, 165, 189]`。如果允许 `s[0]`，它该返回什么？
> - 返回字节 `228`？它不是任何有意义的字符。
> - 返回字符 `'你'`？那索引就不连续了（`你`占 0-2，`好`占 3-5），且每次索引都要从头扫描——O(1) 的直觉被破坏。
>
> 其他语言踩过这个坑：Python 2 的 `len("你好")` 返回 6（字节数），Python 3 改为返回 2（字符数）但索引操作变成了隐藏的 O(n)。Rust 的选择是：**干脆禁止索引，逼你明确说清要字节还是要字符**。

**正确的遍历姿势**：显式选择视角

```rust
fn main() {
    let s = String::from("你好 rust");

    for c in s.chars() {   // 按 Unicode 字符
        print!("{} ", c);  // 你 好   r u s t
    }
    println!();

    for b in s.bytes() {   // 按字节
        print!("{} ", b);  // 228 189 160 229 165 189 32 114 117 115 116
    }
}
```

**拼接与追加**：

```rust
fn main() {
    // push_str：追加 &str，不获取所有权
    let mut s = String::from("你好");
    s.push_str("，世界");
    s.push('！'); // push 追加单个字符

    // + 运算符：本质是 add(self, &str)，会取走左边的所有权！
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 被移动，之后不能再使用；s2 是借用，仍然可用
    // println!("{}", s1); // ❌ 编译错误

    // format!：不取走任何所有权，多段拼接的首选
    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");
    let game = format!("{}-{}-{}", a, b, c); // a、b、c 之后还能用
    println!("{}", game); // tic-tac-toe
}
```

> ⚠️ `+` 的签名 `fn add(self, s: &str) -> String` 解释了为什么 `s1` 消失而 `s2` 存活。拼接多个字符串时 `format!` 几乎总是更好的选择：可读、不转移所有权、性能相当。

**String 与 &str 的转换及 deref coercion**：

```rust
fn main() {
    // String → &str：取引用即可，靠 deref coercion 自动完成
    let s = String::from("hello");
    let slice: &str = &s;        // &String 自动转成 &str
    let slice2: &str = &s[0..2]; // 切片语法也行（注意边界必须落在字符边界上）

    // &str → String：三种等价写法
    let owned1: String = slice.to_string();
    let owned2: String = String::from(slice);
    let owned3: String = slice.to_owned();

    greet(&s);       // &String 自动 deref 成 &str
    greet("world");  // 字面量本来就是 &str
}

fn greet(name: &str) {
    println!("你好, {}!", name);
}
```

> 💡 **deref coercion 直觉**：`String` 实现了 `Deref<Target = str>`，所以 `&String` 可以在需要 `&str` 的地方自动"脱壳"。结论：**函数参数永远写成 `&str` 而不是 `&String`**——这样 String 和 &str 调用者都能用，API 最通用。

### 13.3 HashMap<K, V>：键值映射

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // 插入（key 和 value 的所有权被移入 map！）
    let team = String::from("蓝队");
    scores.insert(team, 10);
    // println!("{}", team); // ❌ team 已被 move
    scores.insert(String::from("红队"), 50);

    // 读取：get 返回 Option<&V>
    let key = String::from("蓝队");
    match scores.get(&key) {
        Some(score) => println!("蓝队: {}", score),
        None => println!("蓝队不存在"),
    }

    // 覆盖：insert 同 key 直接替换
    scores.insert(String::from("蓝队"), 25);

    // 遍历
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
}
```

**entry API：条件插入的瑞士军刀**

经典场景：统计单词出现次数。

```rust
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // entry：key 存在返回 &mut 现有值，不存在则 or_insert 插入默认值后返回 &mut
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
    // {"hello": 1, "world": 2, "wonderful": 1}
}
```

> 💡 **entry API 为什么优雅？** 不用 entry 的写法需要先 `get` 判断再 `insert`——查两次哈希，还要和借用检查器搏斗。`entry(key).or_insert(0)` 一次查找搞定"有则改、无则插"，还直接给你 `&mut V`。常用变体：`or_insert_with(|| 昂贵计算())`（惰性默认值）、`and_modify(|v| *v += 1).or_insert(1)`（组合操作）。

> ⚠️ **所有权注意**：`insert` 会 move 掉 key 和 value（对拥有型类型）。上面 `scores.get(&key)` 用 `&key` 而不是 key，是因为 get 只需要借用。如果 key 要复用，记得用引用或 clone。

### 本章小结

- `Vec<T>`：同质动态数组；`[]` 越界 panic，`.get()` 返回 Option；遍历时禁止增删（借用规则保护迭代器不失效）；异构用枚举或 trait object
- `String` 是 UTF-8 字节序列：禁止下标索引；遍历显式选 `.chars()` 或 `.bytes()`；拼接首选 `format!`；参数类型写 `&str` 最通用
- `HashMap`：insert 取走所有权，get 返回 Option；entry API 一次查找完成"有则改无则插"

### 动手练习

1. 给定 `Vec<i32>`，写函数分别计算均值、中位数、众数（众数用 HashMap 统计）。
2. 写一个 `pig_latin` 函数：把英文单词第一个辅音移到词尾加 "ay"（如 first → irst-fay），元音开头直接加 "hay"。注意用 `chars()` 而不是索引。
3. 用 `HashMap<String, Vec<String>>` 实现一个迷你部门花名册：支持 `add 张三 to 技术部` 和列出某部门所有人。体会 entry API 在"值为 Vec"场景下的用法（`or_insert_with(Vec::new)`）。

---

## 第 14 章 闭包与迭代器

闭包（closure）是可以捕获环境的匿名函数；迭代器（iterator）是处理序列的统一抽象。两者经常搭档出现，共同支撑起 Rust 函数式风格的表达力——而且照例是零成本的。

### 14.1 闭包语法与类型推断

```rust
fn main() {
    // 闭包 vs 函数：语法对比
    fn  add_one_v1(x: u32) -> u32 { x + 1 }   // 函数
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // 闭包，完整标注
    let add_one_v3 = |x| { x + 1 };            // 省略类型标注
    let add_one_v4 = |x| x + 1;                // 单表达式可省花括号

    println!("{}", add_one_v4(5)); // 6
}
```

闭包**通常不需要类型标注**——它不像函数那样是对外 API，编译器可以从使用现场推断。但注意一个坑：

```rust
let example_closure = |x| x;

let s = example_closure(String::from("hello")); // 第一次调用锁定类型为 String
// let n = example_closure(5); // ❌ 编译错误：expected String, found integer
```

> ⚠️ 闭包的类型在**第一次调用时被推断并锁定**，之后不能再换类型。想支持多类型请用泛型函数或泛型闭包参数。

> 🆚 **与其他语言对比**：
> - **Java lambda**：`(x) -> x + 1`，本质是实现单方法接口的匿名类；捕获的外部变量必须是 effectively final（不可变）。
> - **Python lambda**：`lambda x: x + 1`，只能写单个表达式；闭包变量捕获很自由，但可变性和生命周期全靠运行时。
> - **Rust 闭包**：捕获方式由编译器自动分析并受借用规则约束——既能捕获可变引用甚至所有权，又保证内存安全。

### 14.2 捕获环境的三种方式：Fn / FnMut / FnOnce

闭包与普通函数的本质区别是**能捕获环境变量**。Rust 按捕获方式把闭包分为三个 trait 层级：

| Trait | 捕获方式 | 可调用次数 | 类比方法接收者 |
|-------|---------|-----------|---------------|
| `Fn` | 不可变借用（`&T`） | 多次 | `&self` |
| `FnMut` | 可变借用（`&mut T`） | 多次 | `&mut self` |
| `FnOnce` | 获取所有权（`T`） | **一次** | `self` |

编译器自动选择**尽可能宽松**的捕获方式：

```rust
fn main() {
    // Fn：只读取环境
    let list = vec![1, 2, 3];
    let only_borrows = || println!("{:?}", list); // 捕获 &list
    only_borrows();
    only_borrows(); // ✅ 可多次调用

    // FnMut：修改环境
    let mut count = 0;
    let mut inc = || {
        count += 1; // 捕获 &mut count
        count
    };
    println!("{}", inc()); // 1
    println!("{}", inc()); // 2

    // FnOnce：拿走所有权（比如把捕获的值 move 出去）
    let s = String::from("hello");
    let consume = || {
        let _owned = s; // s 被 move 进闭包体内 → 闭包只能调用一次
    };
    consume();
    // consume(); // ❌ 编译错误：closure cannot be invoked more than once
}
```

> 💡 **层级关系**：`Fn` ⊂ `FnMut` ⊂ `FnOnce`。只读借用的闭包自动满足全部三个 trait；要求 `FnOnce` 参数的函数可以接受任何闭包。标准库大量使用这一点：比如 `Option::map` 接受 `FnOnce`（最宽松），而需要反复调用的排序比较器要求 `FnMut`。

**move 闭包：强制转移所有权**

```rust
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("定义闭包前: {:?}", list);

    // move 强制闭包获取 list 的所有权（哪怕只需借用）
    let handle = thread::spawn(move || {
        println!("线程中: {:?}", list);
    });

    // println!("{:?}", list); // ❌ list 已 move 进闭包
    handle.join().unwrap();
}
```

> ⚠️ **为什么线程场景必须 move？** 新线程可能比主线程活得久。如果闭包只是借用 `list`，主线程结束时 list 被释放，子线程里的引用就悬垂了。`move` 把所有权送进闭包，让闭包（线程）成为数据的主人——生命周期问题用所有权消灭，这是 Rust 并发安全的基石（"无畏并发"会在后续篇章展开）。

### 14.3 闭包作为参数与返回值

**作为参数**：泛型 + trait bound（静态分发，首选）或 `Box<dyn Fn>`（动态分发）。

```rust
// 泛型版本：零成本
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn main() {
    let double = |x| x * 2;
    println!("{}", apply(double, 21));        // 42
    println!("{}", apply(|x| x + 100, 1));    // 101
}
```

**作为返回值**：必须 `impl Fn` 或 `Box<dyn Fn>`——闭包的类型是匿名的，写不出具体名字：

```rust
// 返回 impl Fn：现代写法
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// 返回 Box<dyn Fn>：需要在不同闭包类型间选择时
fn make_operator(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match op {
        "+" => Box::new(|a, b| a + b),
        "*" => Box::new(|a, b| a * b),
        _ => Box::new(|a, b| a - b),
    }
}

fn main() {
    let add5 = make_adder(5);
    println!("{}", add5(10)); // 15

    let mul = make_operator("*");
    println!("{}", mul(6, 7)); // 42
}
```

> 💡 **每一个闭包都有独一无二的匿名类型**，哪怕两个闭包长得一模一样。这就是"写不出具体类型名"的原因，也是 `impl Fn` / `dyn Fn` 存在的理由。

### 14.4 迭代器是惰性的（Lazy！）

先看一段"什么都不会发生"的代码：

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter().map(|x| x + 1);
    // 一行警告：unused `Map` that must be used
    // map 的闭包一次都没有执行过！
}
```

> ⚠️ **迭代器是惰性的**：创建迭代器、链式调用适配器，都只是"搭好流水线"，**一个元素都不会被处理**。必须调用消费方法（`collect`、`sum`、`for` 循环等）才真正开工。忘记消费是最常见的迭代器 bug——编译器会给 unused 警告，但不会报错。

> 🆚 **对比**：这与 C# LINQ 的 deferred execution、Java Stream 的"中间操作惰性、终端操作求值"是完全一致的设计。区别是 Rust 把它编译成了零抽象的机器码（见 14.7）。

### 14.5 Iterator trait 与 next

所有迭代器都实现 `Iterator` trait，核心只有一个方法：

```rust
pub trait Iterator {
    type Item; // 关联类型
    fn next(&mut self) -> Option<Self::Item>;
    // ... 其他几十个方法全部有默认实现，都建立在 next 之上
}
```

手动调用 next 看一眼迭代器的本质：

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // next 需要 &mut self，所以 iter 要 mut

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None); // 耗尽后永远返回 None
}
```

`for` 循环本质就是语法糖：自动调用 `into_iter()` 并反复 `next` 直到 `None`。

### 14.6 适配器：组装你的流水线

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    // map：变换每个元素；filter：按条件保留；collect：消费并收集
    let result: Vec<i32> = v
        .iter()
        .filter(|x| *x % 2 == 0)   // 只留偶数
        .map(|x| x * 10)            // 每个乘 10
        .collect();
    println!("{:?}", result); // [20, 40, 60]

    // enumerate：带上索引
    for (i, val) in v.iter().enumerate() {
        println!("索引 {} = 值 {}", i, val);
    }

    // zip：两个迭代器配对（长度以短的为准）
    let names = vec!["甲", "乙", "丙"];
    let scores = vec![90, 85];
    for (name, score) in names.iter().zip(scores.iter()) {
        println!("{} 得分 {}", name, score);
    } // 丙 没有配对对象，被丢弃

    // fold：从初始值开始累积（collect/sum/max 都是 fold 的特化）
    let total = v.iter().fold(0, |acc, x| acc + x);
    println!("总和: {}", total); // 21

    // 常用消费器速览
    println!("个数: {}", v.iter().count());          // 6
    println!("最大: {:?}", v.iter().max());           // Some(6)
    println!("任一>5: {}", v.iter().any(|x| *x > 5)); // true
    println!("第一个偶数: {:?}", v.iter().find(|x| *x % 2 == 0)); // Some(&2)
}
```

**iter / iter_mut / into_iter 三者区别**——集合遍历的三扇门：

| 方法 | 产出 | 等价于 | 原集合之后 |
|------|------|--------|-----------|
| `.iter()` | `&T` 不可变引用 | `for x in &v` | 仍可使用 |
| `.iter_mut()` | `&mut T` 可变引用 | `for x in &mut v` | 仍可使用 |
| `.into_iter()` | `T`（所有权） | `for x in v` | ❌ 已被 move |

```rust
fn main() {
    let v = vec![String::from("a"), String::from("b")];

    for s in v.iter() {
        println!("借: {}", s);
    }
    println!("还能用: {} 个元素", v.len()); // ✅

    for s in v.into_iter() {
        println!("拥有: {}", s); // s 是 String，拿到所有权
    }
    // println!("{:?}", v); // ❌ v 已被 move
}
```

### 14.7 迭代器是零成本抽象

"链式调用这么多层，肯定比 for 循环慢吧？"——恰恰相反。经过单态化和内联优化，编译器能把整条迭代器流水线**展开成与手写循环完全相同的机器码**，有时甚至比手写循环更快（省去边界检查）。

Rust 官方文档中的经典例证：音频解码的 `zip().map().fold()` 链，反汇编后与最优手写汇编几乎一致。这呼应了第 11 章的原则：**你不用为不用的东西付费，你用的抽象也不比手写代码慢**。

> 💡 **风格建议**：简单逻辑用迭代器链（声明式、不易出 off-by-one 错误）；链式调用太长太绕、或需要复杂控制流（break、多层状态）时，老实写 for 循环。可读性永远是第一标准，性能上两者没有区别。

### 本章小结

- 闭包可捕获环境，自动推断类型（首次调用锁定）；语法 `|参数| 表达式`
- 捕获方式决定 trait 层级：Fn（借用）⊂ FnMut（可变借用）⊂ FnOnce（所有权，仅一次）
- `move` 强制所有权捕获，跨线程传闭包时必不可少
- 闭包做参数用 `impl Fn`（零成本）或 `Box<dyn Fn>`（异构）；做返回值必须二者之一
- 迭代器**惰性**：适配器只搭流水线，`collect`/`sum` 等消费器才开工
- `.iter()` / `.iter_mut()` / `.into_iter()` 分别产出 `&T` / `&mut T` / `T`
- 迭代器经单态化内联后是零成本抽象，性能不输手写循环

### 动手练习

1. 用迭代器链实现：给定 `Vec<i32>`，筛选出所有大于 10 的数，平方后求和。分别用 `for` 循环和迭代器写两版，对比可读性。
2. 实现 `fn fibonacci() -> impl FnMut() -> u64`：返回一个闭包，每次调用产出下一个斐波那契数（提示：闭包内捕获两个可变的 `u64` 变量）。调用 10 次打印。
3. 写一个函数 `group_by_parity(v: Vec<i32>) -> (Vec<i32>, Vec<i32>)`，用 `partition` 迭代器方法把奇偶分开（查标准库文档了解 `partition` 的签名，注意它是消费器）。
4. 改造第 10 章练习 1 的 `parse_port`：用 `std::env::args()` 迭代器读取命令行参数作为输入（`env::args().nth(1)`），串起集合、迭代器与错误处理三章的知识。

---

## 本篇结语

进阶篇到此结束。你现在掌握了：

- 用 **模块系统** 组织任意规模的项目
- 用 **Result 与 ?** 构建类型安全的错误处理链
- 用 **泛型与 trait** 写出零成本的抽象
- 用 **生命周期标注** 与借用检查器对话，而不是对抗
- 用 **集合、闭包、迭代器** 写出地道的函数式 Rust

如果只能带走一句话，请记住第 12 章的核心：**生命周期标注不创造生命周期，它只是把引用的血缘关系写进类型系统，让编译器替你把关**。当你不再把编译器报错当敌人，而是当作一位严格的结对编程伙伴，Rust 的学习曲线就真正越过拐点了。

下一篇我们将探索智能指针、并发编程与 Rust 的面向对象特性——那里才是所有权系统真正大放异彩的舞台。


---

# 第四篇：高级特性与实战

恭喜你走到这里！经过前三篇的打磨，你已经掌握了 Rust 的核心武器：所有权、借用、生命周期、trait、泛型和迭代器。但这些都是"单兵作战"的基本功。

本篇将带你进入 Rust 的高级战区：

- **智能指针**：Rust 如何优雅地管理堆内存，以及为什么它能同时做到 C++ 的性能和 Java 的安全；
- **无畏并发**（Fearless Concurrency）：Rust 最引以为傲的领域——把并发 Bug 从运行时挪到编译期；
- **Unsafe 与宏**：掀开 Rust 的"底牌"，理解安全抽象之下的世界；
- **实战项目**：从零构建一个命令行 TODO 应用，综合运用全书知识；
- **工具与路线**：RustRover 高效使用技巧、新手避坑指南和后续学习地图。

> 💡 本篇默认你使用 Windows 11 + RustRover。所有示例代码均已验证可编译运行，实战项目给出了完整的 `Cargo.toml` 配置。

---

## 第 15 章 智能指针

### 15.1 为什么需要智能指针？

**指针**是编程中最古老也最危险的概念之一。在 C 里，指针只是一个裸内存地址，用完后靠人肉 `free`；在 C++ 里，人们发明了 `unique_ptr`（独占所有权）和 `shared_ptr`（共享引用计数）来管理堆内存；在 Java/Python 里，垃圾回收器（GC）帮你搞定一切，但代价是运行时的性能损耗和不可预测的停顿。

Rust 的选择是：**所有权 + 智能指针**。

- 普通的 `&T` / `&mut T` 只是"借来的引用"，不拥有数据；
- **智能指针**（Smart Pointer）是拥有数据的结构体，它们通过实现 `Deref` 和 `Drop` 这两个 trait，表现得像普通指针，却自带资源管理能力。

> 🆚 **与 C++ 对比**：C++ 的 `unique_ptr` ≈ Rust 的 `Box<T>`，`shared_ptr` ≈ Rust 的 `Rc<T>` / `Arc<T>`。但关键区别是：C++ 里你可以把 `shared_ptr` 随手跨线程传递造成数据竞争，编译器不会拦你；而 Rust 会在**编译期**拒绝不安全的用法。

智能指针解决了三类问题：

1. **堆分配**：数据太大或需要跨作用域存活时，放到堆上（`Box<T>`）；
2. **多所有权**：多个所有者共享同一份数据（`Rc<T>`、`Arc<T>`）；
3. **内部可变性**：在不可变引用下修改数据（`RefCell<T>`、`Mutex<T>`）。

### 15.2 Box<T>：最简单的堆分配

`Box<T>` 把数据放到**堆**上，栈上只留一个指针。它只有一个所有者，`Box` 离开作用域时堆内存自动释放。

```rust
fn main() {
    // 5 存在堆上，b 是栈上的指针
    let b = Box::new(5);
    println!("b = {}", b); // b = 5，自动解引用
}
```

你可能会问：一个 `i32` 本来就在栈上，用 `Box` 多此一举。确实如此。`Box` 真正的用武之地是下面两种场景：

**场景一：递归类型**。编译器必须在编译期知道每个类型占多少字节。递归类型（如链表）在定义中包含自身，大小无法确定，编译器会报错：

```rust
// 错误示例：无法编译！
// enum List {
//     Cons(i32, List),   // error[E0072]: recursive type has infinite size
//     Nil,
// }
```

用 ASCII 图看看 Cons List（来自 Lisp 的经典数据结构）在内存中的样子：

```
栈                    堆
┌─────────┐      ┌─────┬──────────┐      ┌─────┬──────────┐      ┌─────┐
│ list ───┼─────▶│  1  │  Box ────┼─────▶│  2  │  Box ────┼─────▶│ Nil │
└─────────┘      │ Cons│          │      │ Cons│          │      └─────┘
                 └─────┴──────────┘      └─────┴──────────┘
  每个 Cons 节点 = i32 + Box 指针，大小固定（指针是 8 字节），编译器满意了！
```

`Box` 把"无限大小"的递归变成了"固定大小"的指针，问题迎刃而解：

```rust
enum List {
    Cons(i32, Box<List>), // Box 大小固定：8 字节指针
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // 构建链表 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("链表创建成功");
}
```

**场景二：trait 对象**（第三篇已见）：`Box<dyn Trait>` 可以在堆上存放"大小未知但实现了某 trait"的类型。

### 15.3 Deref trait 与解引用强制转换

`Box<T>` 用起来之所以像普通引用，是因为它实现了 `Deref` trait。我们也可以自己实现：

```rust
use std::ops::Deref;

// 自定义智能指针：一个包裹 i32 的盒子
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // 解引用后得到的目标类型

    fn deref(&self) -> &Self::Target {
        &self.0 // 返回内部数据的引用
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *y 实际等价于 *(y.deref())
}
```

更妙的是 **deref coercion（解引用强制转换）**：当函数参数需要 `&str` 而你传入 `&String` 时，编译器会自动帮你"解引用链条"：

```rust
fn hello(name: &str) {
    println!("你好, {name}!");
}

fn main() {
    let s = String::from("Rust");
    hello(&s); // &String 自动转成 &str（String 实现了 Deref<Target=str>）

    let b = Box::new(String::from("世界"));
    hello(&b); // &Box<String> -> &String -> &str，连续两级自动转换！
}
```

> 💡 **为什么这样设计？** 解引用强制转换让函数接口只需写最通用的类型（如 `&str`），调用者传什么"装东西的盒子"都能自动适配。没有它，你就得写 `hello(&(*b)[..])` 这样的天书。这是 Rust "零成本抽象"哲学的典型体现：便利是编译期完成的，运行时没有任何开销。

规则总结：
- `&T` → `&U`，当 `T: Deref<Target = U>`
- `&mut T` → `&mut U`，当 `T: DerefMut<Target = U>`
- `&mut T` → `&U`（可变可以降级为不可变，反之不行——所有权规则不可违背）

### 15.4 Drop trait：自定义析构行为

当值离开作用域时，Rust 自动调用其 `Drop` trait 的 `drop` 方法。这类似 C++ 的析构函数（RAII），用于释放资源：

```rust
struct 智能指针示例 {
    数据: String,
}

impl Drop for 智能指针示例 {
    fn drop(&mut self) {
        println!("正在释放 {} 占用的资源...", self.数据);
    }
}

fn main() {
    let a = 智能指针示例 { 数据: String::from("资源A") };
    {
        let b = 智能指针示例 { 数据: String::from("资源B") };
        println!("内部作用域结束");
    } // b 在这里被 drop
    println!("main 函数即将结束");
} // a 在这里被 drop

// 输出顺序：内部作用域结束 → 释放资源B → main 函数即将结束 → 释放资源A
```

> ⚠️ **注意**：你不能手动调用 `x.drop()`（编译器禁止，会导致二次释放）。如需提前释放，使用标准库的 `std::mem::drop(x)`，它拿走所有权并立即销毁。

Drop 的威力在于它是**自动且确定**的：文件、锁、网络连接都可以靠它保证释放，不会像 Java 的 `finalize()` 那样"不知道什么时候才执行"。

### 15.5 Rc<T>：引用计数，多个所有者

`Box<T>` 只有一个所有者。但有时一份数据天然有多个所有者——比如图中的多条边指向同一个节点。这时需要 `Rc<T>`（Reference Counted）：

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a 创建后计数 = {}", Rc::strong_count(&a)); // 1

    // b 和 c 共享 a 的尾部（像两条链表共用一个后缀）
    let b = Cons(3, Rc::clone(&a)); // clone 只增加计数，不拷贝数据！
    println!("b 创建后计数 = {}", Rc::strong_count(&a)); // 2

    {
        let c = Cons(4, Rc::clone(&a));
        println!("c 创建后计数 = {}", Rc::strong_count(&a)); // 3
    } // c 离开作用域

    println!("c 销毁后计数 = {}", Rc::strong_count(&a)); // 2
}
```

内存图示：

```
        ┌─────────────────────────────┐
        │  a: Rc<Cons(5, Cons(10, Nil))> │  ← 引用计数 = 3
        └─────────────────────────────┘
           ▲            ▲
           │            │
      b: Cons(3, ─┘  c: Cons(4, ─┘
      （两条链表共享 a 作为尾部）
```

> ⚠️ **Rc 是单线程的！** `Rc` 的计数增减不是原子操作，跨线程使用会直接**编译报错**（多线程请用第 16 章的 `Arc`）。这不是缺陷而是设计：单线程场景下，`Rc` 比 `Arc` 快，Rust 让你只在需要时才付出原子操作的代价。

另外 `Rc<T>` 只能给你数据的**不可变**引用——如果允许多个所有者可变访问，就破坏了"同一时刻最多一个可变引用"的铁律。想改？配合下面的 `RefCell`。

### 15.6 RefCell<T> 与内部可变性

Rust 的借用规则通常在**编译期**检查：同一时刻，要么多个不可变引用，要么一个可变引用。但有些场景编译器分析不出来（比如运行时才决定借多久），`RefCell<T>` 把检查推迟到**运行时**：

| | `&T` / `&mut T` | `RefCell<T>` |
|---|---|---|
| 检查时机 | 编译期 | 运行时 |
| 违规后果 | 编译错误 | **panic!** |
| 性能 | 零开销 | 有微小运行时开销 |

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    {
        let mut v = data.borrow_mut(); // 运行时获取可变借用
        v.push(4);
        // 此时若再调用 data.borrow() 会直接 panic！
        println!("修改后: {:?}", v);
    } // 可变借用在此释放

    let v = data.borrow(); // 不可变借用，可以有多份
    println!("读取: {:?}", v);
}
```

> 💡 **内部可变性**（Interior Mutability）指"外表不可变、内部可修改"。`borrow()` / `borrow_mut()` 分别对应 `&T` / `&mut T`，只是检查挪到了运行时。这是"我相信你，但运行时盯着"的取舍——适合编译器无法理解但你能保证正确的场景。

**Rc + RefCell 经典组合**：多所有权 + 可修改，比如构建带父指针的树：

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>, // 子节点可增删
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 运行时给 branch 再添加一个子节点
    branch.children.borrow_mut().push(Rc::clone(&leaf));
    println!("branch 的子节点数: {}", branch.children.borrow().len()); // 2
}
```

### 15.7 引用循环与 Weak<T>：内存泄漏

`Rc` 不是万能的——两个对象互相持有对方的 `Rc`，计数永远归不了零，内存就泄漏了：

```
  parent ──Rc──▶ child
    ▲              │
    └──── Rc ──────┘   ← 互相强引用，计数永远 ≥1，谁也释放不了！
   （树结构中"子节点持有父节点"是典型场景）
```

解决方案是 **弱引用 `Weak<T>`**：它指向数据但不增加强引用计数，访问前必须先 `upgrade()` 确认数据还活着：

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,          // 父节点用弱引用，防止循环
    children: RefCell<Vec<Rc<Node>>>,     // 子节点用强引用
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf 强计数 = {}, 弱计数 = {}",
        Rc::strong_count(&leaf), Rc::weak_count(&leaf)); // 1, 0

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // leaf 的 parent 指向 branch（弱引用，不增加强计数）
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch 强计数 = {}", Rc::strong_count(&branch)); // 1
        // 通过弱引用访问父节点，必须先 upgrade
        println!("leaf 的父节点 = {:?}", leaf.parent.borrow().upgrade());
    } // branch 离开作用域，强计数归零，正常释放！

    println!("branch 销毁后，leaf 还能访问父节点吗: {:?}",
        leaf.parent.borrow().upgrade()); // None，安全！
}
```

规则：**父强子弱**——所有者用 `Rc`，被"回望"的一方用 `Weak`。

### 15.8 Rc/RefCell vs Arc/Mutex 对比表

| 维度 | `Rc<T>` | `Arc<T>` |
|---|---|---|
| 线程安全 | ❌ 仅单线程 | ✅ 可跨线程（原子计数） |
| 性能 | 快（普通整数加减） | 略慢（原子操作） |
| 不可变共享 | ✅ | ✅ |

| 维度 | `RefCell<T>` | `Mutex<T>` |
|---|---|---|
| 线程安全 | ❌ 仅单线程 | ✅ 可跨线程 |
| 可变方式 | 运行时借用检查（违规 panic） | 加锁（竞争时阻塞等待） |
| 典型搭配 | `Rc<RefCell<T>>` | `Arc<Mutex<T>>` |

> 💡 记忆口诀：**单线程 Rc + RefCell，多线程 Arc + Mutex**。Rust 让你在编译期就必须想清楚并发模型，选错了编译器会立刻告诉你。

### 本章小结

- `Box<T>`：堆分配，独占所有权，用于递归类型和 trait 对象；
- `Deref`/`Drop`：让智能指针用起来像引用、死得像 RAII；
- `Rc<T>`：单线程多所有权，引用计数；
- `RefCell<T>`：内部可变性，借用规则挪到运行时检查；
- `Weak<T>`：打破 `Rc` 引用循环，防内存泄漏；
- 多线程对应物：`Arc` 和 `Mutex`，下章登场。

### 动手练习

1. 用 `Box` 实现一个二叉树 `enum Tree { Node(i32, Box<Tree>, Box<Tree>), Leaf }`，写一个递归求和的函数；
2. 实现一个 `MyBox<T>`，为它同时实现 `Deref` 和 `DerefMut`，验证可变解引用；
3. 用 `Rc<RefCell<Vec<i32>>>` 模拟"多个观察者共享并修改同一份数据"，打印每次的强引用计数；
4. 故意构造一个 `Rc` 引用循环（两个节点互相持有），用 `std::mem` 观察内存没有释放，然后改成 `Weak` 修复。

---

## 第 16 章 无畏并发

### 16.1 并发：其他语言的噩梦，Rust 的主场

并发编程的 Bug 是出了名地难缠：数据竞争（data race）、死锁、悬垂指针……在 C/C++ 里它们可能潜伏数月后在线上爆发；在 Java 里你得背熟 `synchronized` / `volatile` 的细则；在 Python 里 GIL 干脆让多线程计算形同虚设。

Rust 的口号是 **Fearless Concurrency（无畏并发）**：编译器利用所有权和类型系统，在**编译期**就拒绝大多数并发 Bug。写并发代码时，如果编译通过，你大概率是对的——这种感觉在其他语言里几乎不存在。

### 16.2 创建线程：thread::spawn 与 join

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // spawn 返回 JoinHandle，像一张"取货单"
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("子线程: 计数 {i}");
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..=3 {
        println!("主线程: 计数 {i}");
        thread::sleep(Duration::from_millis(100));
    }

    // join 阻塞等待子线程结束；不 join 的话主线程结束会带走所有子线程
    handle.join().unwrap();
    println!("子线程已结束");
}
```

#### move 闭包：把所有权转移进线程

子线程可能活得比创建它的作用域还久，所以它**不能借用**局部变量——必须用 `move` 拿走所有权：

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // 错误写法：thread::spawn(|| println!("{:?}", v));
    // 编译报错：闭包可能活得比 v 久，借用了不属于自己的数据

    // 正确：move 把 v 的所有权转移给子线程
    let handle = thread::spawn(move || {
        println!("子线程拿到 vec: {v:?}");
    });

    handle.join().unwrap();
    // println!("{v:?}"); // ❌ 编译错误：v 已经被 move 走了
}
```

> 🆚 **与 Java 对比**：Java 里把局部变量传给线程需要 `final` 修饰，但那只是"不能重新赋值"，照样可能共享可变状态造成竞争。Rust 的 `move` 是**所有权转移**，从根上杜绝了"两个线程都认为自己是主人"的可能。

### 16.3 消息传递 channel：通过通信共享内存

Go 语言有句名言，Rust 同样信奉：

> 💡 **"Do not communicate by sharing memory; instead, share memory by communicating."**（不要通过共享内存来通信，而要通过通信来共享内存。）

标准库的 `mpsc`（multiple producer, single consumer，多生产者单消费者）channel 就是这条哲学的基础设施：

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建通道：tx 是发送端，rx 是接收端
    let (tx, rx) = mpsc::channel();

    // clone 出发送端，实现"多生产者"
    let tx1 = tx.clone();
    thread::spawn(move || {
        let msgs = vec!["你好", "来自", "线程一"];
        for msg in msgs {
            tx1.send(msg).unwrap(); // send 转移了 msg 的所有权！
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let msgs = vec!["hi", "from", "thread two"];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // rx 是迭代器，通道关闭（所有 tx 都 drop）后自动结束循环
    for received in rx {
        println!("收到: {received}");
    }
}
```

注意 `send` 会**转移所有权**：发送后消息归接收方所有，发送方不能再碰它。这正是"通过通信共享内存"——数据的所有权随着消息在通道里流动，任何时刻只有一个线程持有它，竞争无从谈起。

> 🆚 **与 Go 对比**：Go 的 channel 语法更轻（`<-` 操作符），但 Go 的 channel 传的是**拷贝或指针**，传指针时两个 goroutine 可能同时持有同一份内存，数据竞争照样发生（Go 有 race detector 帮你**运行时**检测）。Rust 用所有权在**编译期**就保证：发送后发送方失去访问权，零检测开销。

### 16.4 共享状态：Mutex<T> 与 Arc<Mutex<T>>

有时共享状态不可避免，这时需要互斥锁 `Mutex<T>`（mutual exclusion）。Rust 的 `Mutex` 有个反直觉但绝妙的设计：**锁保护的不是代码，而是数据本身**——想碰数据？先拿锁。

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // lock() 返回 Result<MutexGuard>，Guard 是"拿着钥匙的智能指针"
        let mut num = m.lock().unwrap();
        *num = 6; // 通过 DerefMut 直接修改被保护的数据
    } // num 离开作用域，锁自动释放（RAII！永远不会忘解锁）
}
```

现在看多线程版本——经典的 `Arc<Mutex<T>>` 组合：

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc：多线程版 Rc，原子引用计数，可安全跨线程共享
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter); // 每个线程拿一份 Arc（增加计数）
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1; // 拿到锁才能改，10 个线程安全累加
            println!("线程 {i} 完成加一，当前值: {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // 等所有线程结束
    }

    println!("最终计数: {}", *counter.lock().unwrap()); // 10
}
```

**为什么必须是 `Arc` 而不是 `Rc`？** 试试把上面代码里的 `Arc` 换成 `Rc`：

```text
error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
   = help: the trait `Send` is not implemented for `Rc<Mutex<i32>>`
```

编译器直接拒绝！因为 `Rc` 的计数不是原子的：两个线程同时 clone，计数可能从 1 变成 2 而不是 3，最终导致计数错乱、内存提前释放——灾难。`Arc`（Atomic Rc）用原子操作保证计数安全，代价是略微的性能开销。

> 💡 这就是 Rust 设计的美妙之处：你不小心选错工具，**编译器就是你的并发专家**，用一条类型错误拦住运行时灾难。

### 16.5 Send 与 Sync：Rust 最惊艳的设计

几乎所有现代语言都有线程，但只有 Rust 用两个 marker trait 把"线程安全性"编码进了类型系统：

- **`Send`**：该类型的所有权可以安全地**转移**到另一个线程；
- **`Sync`**：该类型可以安全地被多个线程**同时引用**（即 `&T` 是 `Send`）。

规则一览：

| 类型 | Send | Sync | 原因 |
|---|---|---|---|
| `i32`、`String`、`Vec<T>` | ✅ | ✅ | 纯数据，天然安全 |
| `Rc<T>` | ❌ | ❌ | 非原子引用计数 |
| `RefCell<T>` | ✅ | ❌ | 可变可转移，但共享引用不安全 |
| `Mutex<T>` | ✅ | ✅ | 锁提供同步保证 |
| `Arc<T>`（T: Send + Sync） | ✅ | ✅ | 线程安全的多所有权 |

它们是 **auto trait**：编译器自动为几乎所有类型推导实现。你几乎不需要手写 `unsafe impl Send`——但编译器会用它检查一切。`thread::spawn` 的签名要求闭包是 `Send`，于是：

- 试图把 `Rc` 传进线程 → 编译错误；
- 试图跨线程共享 `&RefCell` → 编译错误；
- 忘了 `move` 导致闭包借用局部变量 → 编译错误。

> 💡 **这是 Rust 最惊艳的设计**：其他语言里"线程安全"是写在文档里的约定（靠程序员自觉），Rust 里它是**类型**——违反约定就是编译错误。零运行时开销，却把数据竞争这一整类 Bug 在编译期消灭了。

### 16.6 常见并发 Bug：其他语言 vs Rust

| 并发 Bug | C++/Java 中的表现 | Rust 中的表现 |
|---|---|---|
| 数据竞争（两线程同时写同一变量） | 运行时随机出错，极难复现 | **编译错误**（借用规则 + Send/Sync） |
| 忘记释放锁 | 死锁，进程挂起 | 不可能：Guard 靠 RAII 自动释放 |
| 锁保护范围与数据不匹配 | 锁了 A 却改了 B，编译器不管 | 不可能：不拿锁根本访问不到数据 |
| 悬垂指针（线程访问已释放内存） | 段错误/脏数据 | **编译错误**（生命周期检查） |
| 共享可变迭代器 | ConcurrentModificationException（运行时） | **编译错误**（不允许同时存在多个可变引用） |

> ⚠️ Rust 不能消灭所有并发问题：死锁（两个 Mutex 互相等待）、逻辑错误仍需你自己小心。但危害最大、最难排查的数据竞争类 Bug 已被编译器挡在门外。

### 16.7 rayon：一行代码把串行变并行

手写线程池太繁琐？`rayon` 库提供了"数据并行"的免费午餐——把迭代器的 `iter()` 换成 `par_iter()`，剩下的交给它：

```toml
# Cargo.toml
[dependencies]
rayon = "1"
```

```rust
use rayon::prelude::*;

fn main() {
    let numbers: Vec<u64> = (1..=1_000_000).collect();

    // 串行版本
    let sum: u64 = numbers.iter().map(|x| x * x).sum();

    // 并行版本：只改了两个地方（iter -> par_iter）
    let par_sum: u64 = numbers.par_iter().map(|x| x * x).sum();

    assert_eq!(sum, par_sum);
    println!("并行计算完成: {par_sum}");
}
```

rayon 内部是工作窃取（work-stealing）线程池，自动按 CPU 核数切分任务。而且由于 Rust 的借用检查，`par_iter` 要求每个元素的闭包操作相互独立——**编译器保证了并行不会引入竞争**，这在其他语言的并行库里是不敢想象的承诺。

### 本章小结

- `thread::spawn` + `JoinHandle::join` 管理线程，`move` 闭包转移所有权；
- `mpsc` channel 体现"通过通信共享内存"，所有权随消息流动；
- 共享状态用 `Arc<Mutex<T>>`：Arc 负责跨线程多所有权，Mutex 负责互斥修改；
- `Send`/`Sync` 把线程安全编码进类型系统，编译期消灭数据竞争；
- rayon 让数据并行只需改一个单词。

### 动手练习

1. 写一个程序：主线程通过 channel 向子线程发送 10 个数字，子线程求和后通过另一个 channel 发回结果；
2. 用 `Arc<Mutex<Vec<String>>>` 实现：5 个线程并发地向共享 vec 追加字符串，最后打印所有内容；
3. 故意尝试把 `Rc::new(5)` 传进 `thread::spawn`，阅读并理解编译器报错信息；
4. 用 rayon 并行计算 1 到 1 亿中所有素数的个数（用试除法），对比串行版本的耗时。

---

## 第 17 章 Unsafe Rust 与宏（简介）

### 17.1 为什么需要 unsafe？

Rust 的借用检查器非常强大，但它是个**保守派**：宁可错杀一千（拒绝一些其实安全的代码），不可放过一个。有两种情况它需要人类帮忙：

1. **编译器分析能力有限**：某些操作人类能证明安全，但静态分析做不到；
2. **必须与底层打交道**：操作硬件、调用 C 库（FFI）、实现高性能数据结构时，绕不开裸内存操作。

> 💡 重要澄清：`unsafe` **不是关闭安全检查**！借用规则、类型检查在 unsafe 块里依然生效。它只是解锁了下面五种"超能力"，并告诉你：这里的安全由**程序员**担保，编译器不再背书。

标准库本身就是大量 unsafe 的用武之地：`Vec`、`String`、`Mutex` 的内部实现都是 unsafe 代码，但它们对外暴露了**安全的 API**。这就是 Rust 的核心哲学：**把不安全封装起来，让安全成为默认值**。

### 17.2 unsafe 的五大超能力

在 `unsafe` 块（或 `unsafe fn`）中，你可以做这五件事：

1. **解引用裸指针**（`*const T` / `*mut T`）——不受借用规则约束的 C 风格指针；
2. **调用 unsafe 函数或方法**——包括调用 C 语言的外部函数（FFI）；
3. **访问或修改可变的静态变量**（`static mut`）——全局可变状态；
4. **实现 unsafe trait**——如手动声明 `unsafe impl Send`；
5. **访问 union 的字段**——与 C 互操作的联合体。

除此之外的一切（数组越界检查、借用检查、Drop 语义）照常工作。

### 17.3 一个安全的 unsafe 示例：split_at_mut 的原理

标准库有个方法 `slice.split_at_mut(mid)`，能把一个切片拆成**两个互不重叠的可变切片**。如果用普通 Rust 自己实现：

```rust
// 错误示例：无法编译！
// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     assert!(mid <= len);
//     (&mut slice[..mid], &mut slice[mid..])
//     // ❌ error[E0499]: 不能同时两次可变借用 slice
// }
```

编译器拒绝了：它不理解 `slice[..mid]` 和 `slice[mid..]` 是**互不重叠**的两块内存，只看到"两次可变借用同一个 slice"。但我们人类知道这是安全的！于是标准库的真实实现用了 unsafe：

```rust
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // 拿到裸指针：创建裸指针本身安全，解引用才需要 unsafe
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len); // 用断言保证 mid 合法——安全契约的关键！

    unsafe {
        (
            // from_raw_parts_mut：从裸指针 + 长度重建切片
            slice::from_raw_parts_mut(ptr, mid),
            // ptr.add(mid) 跳过前 mid 个元素，指向后半段
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);

    left[0] = 100;   // 修改前半段
    right[2] = 600;  // 修改后半段

    println!("左半段: {left:?}");   // [100, 2, 3]
    println!("右半段: {right:?}");  // [4, 5, 600]
    println!("原 vec: {v:?}");      // [100, 2, 3, 4, 5, 600]
}
```

unsafe 代码的安全由两个事实担保：

1. `assert!(mid <= len)` 保证不会越界；
2. 两段内存 `[0, mid)` 和 `[mid, len)` 互不重叠，不存在别名冲突。

> 💡 这个例子完美诠释了 unsafe 的正确用法：**小块、封装、有明确的安全论证**。调用者拿到的是完全安全的接口，unsafe 被锁在函数内部。

### 17.4 何时可以用 unsafe 的原则

- **能不用就不用**：先确认安全 Rust 真的无法实现（多数时候它可以）；
- **最小化范围**：unsafe 块越小越好，最好只有一两行；
- **封装成安全 API**：像 `split_at_mut` 一样，对外隐藏 unsafe；
- **写下安全论证**：在注释里说明"为什么这段代码是安全的"，供 reviewer 检查；
- **用 Miri 等工具验证**：`cargo +nightly miri test` 可以检测未定义行为。

> ⚠️ 新手阶段（以及大多数应用开发中）几乎不需要写 unsafe。如果你在业务代码里频繁写下 `unsafe`，大概率是设计出了问题，而不是 Rust 不够灵活。

### 17.5 宏简介：写代码的代码

宏（macro）是 Rust 的**元编程**工具：在编译期展开、生成代码。你早就用过它——`println!`、`vec!`、`assert_eq!` 都是宏（注意感叹号）。

> 🆚 **与 C 宏对比**：C 的 `#define` 是纯粹的文本替换，容易踩坑（比如宏参数被意外展开两次）。Rust 宏是**卫生宏**（hygienic macro）：操作的是语法树而非文本，宏内部的变量不会和外部冲突，安全得多。

#### 声明宏：macro_rules!

`macro_rules!` 让你用"模式匹配 → 生成代码"的方式定义宏。下面是一个简化版 `vec!`：

```rust
// 声明宏：匹配模式，生成代码
macro_rules! my_vec {
    // 模式1：零个或多个表达式，逗号分隔
    // $( $x:expr ),*  表示 "表达式 x，以逗号分隔，重复 0 次或多次"
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x); // 对每个匹配到的表达式生成一行 push
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = my_vec![1, 2, 3];        // 展开成：push(1); push(2); push(3);
    let empty: Vec<i32> = my_vec![]; // 也能匹配空的情况
    println!("v = {v:?}, empty = {empty:?}");
}
```

展开过程：`my_vec![1, 2, 3]` 在编译期被替换成一个代码块，先 `Vec::new()`，再三次 `push`，最后返回 vec。零运行时开销——宏只是编译期的代码生成器。

#### 过程宏与 derive

更强大的是**过程宏**（procedural macro）：它是一个真正的 Rust 函数，接收 TokenStream（代码），输出 TokenStream（代码）。你天天用的 `#[derive(Debug)]` 就是一种过程宏。

最经典的例子是 `serde`（序列化库）：

```rust
use serde::{Serialize, Deserialize};

// 加上这两个 derive，serde 在编译期自动为你的结构体
// 生成序列化/反序列化的全部代码——几百行样板代码瞬间消失
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}
```

如果没有 derive 宏，你得为每个结构体手写"怎么变成 JSON、怎么从 JSON 恢复"的样板代码。过程宏把"根据类型结构生成代码"这件事自动化了。

#### 宏的学习方向

本篇不做深挖，给你一张地图：

- **声明宏**（`macro_rules!`）：适合消除小范围重复代码，语法可参考 Rust Book 附录或 *The Little Book of Rust Macros*；
- **derive 宏**：最常用，先学会"用"（serde、Debug、Clone），再学"写"（需要 `syn` + `quote` 两个 crate）；
- **属性宏**（如 `#[tokio::main]`）和 **函数式宏**（如 `println!` 的完整实现）：进阶主题，写库时才需要。

> 💡 经验法则：先考虑泛型和 trait 能否解决问题，宏是"最后手段"。宏很强大，但错误信息晦涩、可读性差——`macro_rules!` 调试起来足以让人怀念所有权检查器。

### 本章小结

- `unsafe` 解锁五大超能力（裸指针解引用、unsafe 函数、`static mut`、unsafe trait、union），但借用检查依然生效；
- 正确使用姿势：小块、封装成安全 API、注释写清安全论证；
- `split_at_mut` 是"安全接口包裹 unsafe 实现"的教科书案例；
- 宏是编译期代码生成：`macro_rules!` 声明宏处理简单重复，derive/过程宏（如 serde）自动生成样板代码。

### 动手练习

1. 写一个 `unsafe` 函数：解引用一个 `*const i32` 裸指针并打印，思考为什么解引用必须放在 unsafe 块中；
2. 查阅 `std::mem::transmute` 的文档，理解为什么它是"最危险的 unsafe 函数"（不要在你的代码中使用它）；
3. 扩展 `my_vec!` 宏，支持 `my_vec![0; 5]` 这种"重复元素"语法（提示：参考标准库 `vec!` 的第二种模式）；
4. 给结构体加上 `#[derive(Debug, Clone, PartialEq)]`，用 `cargo expand`（需 `cargo install cargo-expand`）观察宏展开后生成了什么代码。

---

## 第 18 章 实战项目：命令行 TODO 应用

前面 17 章的知识，不落到代码上就是纸上谈兵。本章我们从零构建一个**完整可发布**的命令行 TODO 应用，把所有权、`Result`、迭代器、模块、serde 全部用上。

### 18.1 需求分析

我们要做的是一个叫 `todo` 的命令行工具：

```text
todo add "写 Rust 教程"     # 添加任务
todo list                   # 列出所有任务
todo done 1                 # 把 id 为 1 的任务标记为完成
todo remove 1               # 删除 id 为 1 的任务
```

非功能性需求：

- 任务数据要**持久化**到本地 JSON 文件，关闭程序后不丢失；
- 非法输入（比如 `done` 一个不存在的 id）要给出友好提示，不能 panic；
- 代码模块化，便于扩展（以后想加"优先级""截止日期"时不伤筋动骨）。

### 18.2 数据结构与模块划分

核心数据就是一个任务：

```text
Task { id: u32, title: String, done: bool }
```

模块划分（单一职责，每个文件只管一件事）：

```text
todo-cli/
├── Cargo.toml
└── src/
    ├── main.rs      # 程序入口：解析命令 → 调度 → 输出
    ├── cli.rs       # 命令行参数定义（clap）
    ├── task.rs      # Task 数据结构
    └── storage.rs   # JSON 文件的读写
```

> 💡 **为什么这样划分？** `main.rs` 只做"粘合"，业务逻辑分散到各模块。这正是第 8 章模块系统的实战：当你以后想把存储从 JSON 换成 SQLite，只需重写 `storage.rs`，其他文件一行不动。

### 18.3 Cargo.toml：依赖配置

在终端执行 `cargo new todo-cli`，然后把 `Cargo.toml` 改为：

```toml
[package]
name = "todo-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
# serde：序列化框架，derive 特性让我们能用 #[derive(Serialize, Deserialize)]
serde = { version = "1", features = ["derive"] }
# serde_json：JSON 格式的具体实现
serde_json = "1"
# clap：命令行解析，derive 特性让我们用结构体定义命令
clap = { version = "4", features = ["derive"] }
```

> ⚠️ `features = ["derive"]` 千万别漏！没有它，`#[derive(Parser)]` 和 `#[derive(Serialize)]` 会报"找不到宏"的错误——这是新手配置这两个库时最常踩的坑。

### 18.4 完整代码

#### src/task.rs —— 数据模型

```rust
use serde::{Deserialize, Serialize};

/// 一条待办任务
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task {
    /// 创建新任务，默认未完成
    pub fn new(id: u32, title: String) -> Self {
        Task {
            id,
            title,
            done: false,
        }
    }
}
```

#### src/storage.rs —— 持久化层

```rust
use std::fs;
use std::io;
use std::path::Path;

use crate::task::Task;

/// 数据文件路径（程序运行目录下的 todo.json）
const FILE_PATH: &str = "todo.json";

/// 从 JSON 文件加载全部任务
/// 文件不存在时返回空列表（首次运行）；解析失败时返回错误而不是 panic
pub fn load() -> Result<Vec<Task>, io::Error> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(Vec::new()); // 第一次运行，还没有数据文件
    }

    let content = fs::read_to_string(FILE_PATH)?;

    // 文件存在但内容损坏时，明确报错，别默默吞掉数据
    serde_json::from_str(&content)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// 把任务列表写入 JSON 文件（覆盖式保存）
pub fn save(tasks: &[Task]) -> Result<(), io::Error> {
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(FILE_PATH, json)
}
```

#### src/cli.rs —— 命令行定义

```rust
use clap::{Parser, Subcommand};

/// 程序的整体命令行结构
#[derive(Parser)]
#[command(name = "todo", version, about = "一个用 Rust 写的命令行 TODO 应用")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// 支持的子命令
#[derive(Subcommand)]
pub enum Commands {
    /// 添加新任务
    Add {
        /// 任务标题
        title: String,
    },
    /// 列出所有任务
    List,
    /// 将指定 id 的任务标记为完成
    Done {
        /// 任务 id
        id: u32,
    },
    /// 删除指定 id 的任务
    Remove {
        /// 任务 id
        id: u32,
    },
}
```

> 💡 clap 的 derive 模式非常优雅：结构体的**字段**就是参数，**注释**自动变成 `--help` 里的帮助文档。改需求只需改这个枚举，参数解析、错误提示、帮助信息全部由 clap 自动生成。

#### src/main.rs —— 程序入口

```rust
mod cli;
mod storage;
mod task;

use clap::Parser;
use cli::{Cli, Commands};
use task::Task;

fn main() {
    // 解析命令行参数；参数不合法时 clap 自动打印帮助并退出
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        // 顶层统一处理错误，友好提示而不是一堆 panic 堆栈
        eprintln!("错误: {e}");
        std::process::exit(1);
    }
}

/// 核心业务逻辑，返回 Result 让 main 统一处理错误
fn run(cli: Cli) -> Result<(), std::io::Error> {
    // 每次运行：加载 → 修改 → 保存
    let mut tasks = storage::load()?;

    match cli.command {
        Commands::Add { title } => {
            // 迭代器求最大 id，空列表时从 1 开始
            let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            tasks.push(Task::new(next_id, title.clone()));
            println!("✅ 已添加任务 #{next_id}: {title}");
        }

        Commands::List => {
            if tasks.is_empty() {
                println!("暂无任务，用 `todo add \"任务名\"` 添加一个吧！");
            } else {
                for t in &tasks {
                    let mark = if t.done { "✔" } else { " " };
                    println!("[{mark}] #{:<3} {}", t.id, t.title);
                }
            }
        }

        Commands::Done { id } => {
            // iter_mut 取得可变引用，find 定位任务
            match tasks.iter_mut().find(|t| t.id == id) {
                Some(t) => {
                    t.done = true;
                    println!("✅ 任务 #{id} 已完成: {}", t.title);
                }
                None => {
                    eprintln!("⚠️ 找不到 id 为 {id} 的任务");
                }
            }
        }

        Commands::Remove { id } => {
            let before = tasks.len();
            // retain 保留 id 不匹配的任务，相当于删除目标
            tasks.retain(|t| t.id != id);
            if tasks.len() < before {
                println!("🗑️ 已删除任务 #{id}");
            } else {
                eprintln!("⚠️ 找不到 id 为 {id} 的任务");
            }
        }
    }

    storage::save(&tasks)?;
    Ok(())
}
```

通读这份代码，你会发现前面学的知识全在里面：

- **所有权**：`title` 是 `String`（拥有数据），`save(&tasks)` 只借用不拿走；
- **Result 与 `?`**：`load()?` / `save()?` 把 IO 错误一路向上传播，`main` 统一处理；
- **迭代器**：`iter().map().max()` 求最大 id，`iter_mut().find()` 定位任务，`retain()` 删除——没有一个手写索引循环；
- **模块**：四个文件各司其职，`use crate::task::Task` 跨模块引用；
- **serde**：`Task` 加上 derive 就自动获得 JSON 序列化能力。

### 18.5 编译运行演示

在项目根目录（`todo-cli/`）下：

```powershell
# 开发模式运行（cargo run -- 后面跟的是程序的参数）
PS> cargo run -- add "写完 Rust 教程第四篇"
✅ 已添加任务 #1: 写完 Rust 教程第四篇

PS> cargo run -- add "给自己泡一杯咖啡"
✅ 已添加任务 #2: 给自己泡一杯咖啡

PS> cargo run -- list
[ ] #1   写完 Rust 教程第四篇
[ ] #2   给自己泡一杯咖啡

PS> cargo run -- done 1
✅ 任务 #1 已完成: 写完 Rust 教程第四篇

PS> cargo run -- remove 99
⚠️ 找不到 id 为 99 的任务

PS> cargo run -- list
[✔] #1   写完 Rust 教程第四篇
[ ] #2   给自己泡一杯咖啡
```

同时，运行目录下会出现 `todo.json`：

```json
[
  {
    "id": 1,
    "title": "写完 Rust 教程第四篇",
    "done": true
  },
  {
    "id": 2,
    "title": "给自己泡一杯咖啡",
    "done": false
  }
]
```

也可以试试 `--help`，看看 clap 免费生成的帮助文档：

```powershell
PS> cargo run -- --help
一个用 Rust 写的命令行 TODO 应用

Usage: todo-cli <COMMAND>

Commands:
  add     添加新任务
  list    列出所有任务
  done    将指定 id 的任务标记为完成
  remove  删除指定 id 的任务
  help    Print this message or the help of the given subcommand(s)
```

### 18.6 发布为 Windows 可执行文件

```powershell
# 发布模式编译：开启全部优化，生成独立 exe
PS> cargo build --release
```

产物在 `target\release\todo-cli.exe`——一个**单文件、无依赖**的可执行程序（约几百 KB），双击或拷到任何 Windows 机器上都能直接运行：

```powershell
PS> .\target\release\todo-cli.exe add "把 exe 发给朋友炫耀"
✅ 已添加任务 #3: 把 exe 发给朋友炫耀
```

> 🆚 **与其他语言对比**：Python 写完要配虚拟环境、pip 依赖，分发时还得 PyInstaller 打包；Java 要 JVM；Go 同样能编译单文件，这也是 Go/Rust 在 CLI 工具领域碾压脚本语言的原因。Rust 的 `cargo build --release` 一步到位，还没有 GC 带来的启动延迟。

> 💡 **扩展挑战**：试试给任务加 `priority: Option<u8>` 字段、加一个 `todo clear` 清空已完成任务的命令、或者把文件路径换成系统标准数据目录（看看 `dirs` crate）。每加一个功能，都是对前面章节的一次复习。

### 本章小结

- 实战流程：需求分析 → 数据结构设计 → 模块划分 → 编码 → 发布；
- `serde` + `serde_json` 用两个 derive 搞定 JSON 持久化；
- `clap` 的 derive 模式让命令行定义变成"写结构体"；
- `?` 传播错误、顶层统一处理，是 Rust 应用的标准错误处理姿势；
- `cargo build --release` 产出单文件 exe，分发零负担。

---

## 第 19 章 RustRover 高效使用与常见坑

### 19.1 RustRover 调试：像侦探一样工作

JetBrains RustRover 对 Rust 的调试支持基于 LLDB（Windows 上使用 MSVC 工具链时也能正常工作）。核心操作：

- **打断点**：在编辑器行号左侧单击，出现红点；
- **启动调试**：点击 `fn main` 左侧的绿色三角 → 选 "Debug"，或按 `Shift + F9`；
- **步进控制**：`F8` 单步跳过（Step Over）、`F7` 单步进入（Step Into）、`Shift + F8` 跳出函数、`F9` 继续运行；
- **查看变量**：调试面板自动显示当前作用域所有变量的值和类型；结构体、`Vec` 都能展开查看内部元素；
- **表达式求值（REPL 式）**：调试暂停时按 `Alt + F8`（Evaluate Expression），可以输入任意 Rust 表达式立即求值，比如 `tasks.iter().filter(|t| !t.done).count()`。

> 💡 调试时善用 **Watches（监视窗口）**：把你关心的表达式（如 `Rc::strong_count(&leaf)`）加进去，每走一步都能看到它如何变化——学习智能指针时用它观察引用计数的增减，比一百句 `println!` 都直观。

### 19.2 内联提示：IDE 替你做的类型标注

RustRover 默认开启的内联提示（Inlay Hints）是学习期的神兵利器：

- **类型提示**：`let v = vec![1, 2, 3];` 旁边会灰色显示 `: Vec<i32>`——你不用猜编译器推导出了什么类型；
- **参数名提示**：调用 `String::from("hello")` 时显示参数名，不用翻文档；
- **链式调用类型提示**：长的迭代器链条上，每一步 `.map()` `.filter()` 之后的中间类型都会标注出来——调迭代器类型错误时极其有用；
- **借用/移动提示**：变量被 move 之后，原位置会有删除线样式的提示，一眼看出"这玩意儿已经不归你了"。

如果提示太多嫌吵，可以在 `Settings → Editor → Inlay Hints → Rust` 中按需开关。

### 19.3 常用重构

- **重命名**（`Shift + F6`）：安全地重命名变量/函数/类型，包括所有引用点和字符串里的提及；
- **提取函数**（`Ctrl + Alt + M`）：选中一段代码抽成函数，IDE 自动分析需要哪些参数和返回值——所有权关系它都算好了；
- **提取变量**（`Ctrl + Alt + V`）：把复杂表达式抽成局部变量并自动标注类型；
- **内联**（`Ctrl + Alt + N`）：提取的逆操作；
- **快速修复**（`Alt + Enter`）：光标放在编译器报错的红色波浪线上，按它！RustRover 能自动应用编译器建议的修复（比如加上 `.clone()`、补上缺失的 match 分支、导入缺失的 trait）。

> 💡 **新手最重要的一个快捷键就是 `Alt + Enter`**。rustc 的错误信息本身就以"贴心"著称，经常附带修复建议；RustRover 把这些建议变成了一键操作。遇到报错先别急着自己改，按一下看看。

### 19.4 阅读编译器错误的方法论

新手面对满屏红色容易恐慌，其实 rustc 的错误信息是所有主流语言里写得最好的，结构非常固定：

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let r = &s;
  |             -- immutable borrow occurs here   ← 第一次借用发生在这
4 |     s.push_str("!");
  |     ^ mutable borrow occurs here              ← 冲突发生在这
5 |     println!("{r}");
  |                - immutable borrow later used here  ← 第一次借用一直活到这
```

**阅读方法论（四步走）**：

1. **先看错误码**（如 `E0502`），它决定了错误的类别，可以搜 "rust E0502" 查看官方详细解释；
2. **看第一行人话描述**， rustc 会用直白的语言说明发生了什么；
3. **看标注的代码位置**，蓝色箭头会精确指出"第一次借用""冲突操作""借用存活到"三个关键时间点；
4. **看最后的 help 提示**，很多错误直接给出了修复代码。

> ⚠️ 不要只看 RustRover 的行内红点！行内提示常常只有一行摘要。请打开完整的编译输出（`cargo check` 或 Build 窗口），那里有完整的错误图解。

#### 三大高频错误速查表

| 错误码 | 含义 | 典型场景 | 修复方案 |
|---|---|---|---|
| **E0382** | 使用了已被 move 的值 | 把 `String` 传给函数后还想继续用 | ① 函数改收 `&String`（借用）；② 传之前 `.clone()`；③ 用不可克隆类型时重新设计所有权 |
| **E0502** | 不可变借用存活期间尝试可变借用 | `let r = &v;` 之后调用 `v.push(...)`，且 `r` 之后还要用 | ① 调整顺序：先用完 `r` 再修改 `v`；② 把要读的值提前 `clone` 出来；③ 重构代码避免同时读写 |
| **E0106** | 缺少生命周期标注 | 函数返回引用，但编译器无法确定它借自哪个参数 | 显式标注：`fn foo<'a>(x: &'a str) -> &'a str`；或重新设计：返回拥有的值（如 `String`）而不是引用 |

**E0382 示例**：

```rust
// ❌ 错误代码
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("{s}"); // error[E0382]: borrow of moved value: `s`
}
fn takes_ownership(s: String) { println!("{s}"); }

// ✅ 修复：改传引用，所有权不转移
fn main() {
    let s = String::from("hello");
    borrows(&s);
    println!("{s}"); // 正常
}
fn borrows(s: &String) { println!("{s}"); }
```

**E0502 示例**：

```rust
// ❌ 错误代码
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);              // error[E0502]
    println!("{first}");    // first 还要用，所以不可变借用还活着
}

// ✅ 修复一：调整顺序，push 放在使用 first 之后
let first = v[0];           // i32 是 Copy，直接拷贝值
v.push(4);
println!("{first}");

// ✅ 修复二：先打印再 push，借用提前结束
let first = &v[0];
println!("{first}");
v.push(4);                  // 此时 first 已不再使用，借用已结束
```

**E0106 示例**：

```rust
// ❌ 错误代码
// fn longest(x: &str, y: &str) -> &str {  // error[E0106]: missing lifetime specifier
//     if x.len() > y.len() { x } else { y }
// }

// ✅ 修复：标注生命周期，告诉编译器"返回值活得跟两个参数中较短的那个一样久"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 19.5 新手十大常见坑

**坑 1：String 和 &str 傻傻分不清**

```rust
// ❌ 错误
fn greet(name: &str) { println!("你好 {name}") }
fn main() {
    let s = String::from("世界");
    // greet(s); // 编译错误：expected &str, found String
    greet(&s);           // ✅ 传引用
    greet(s.as_str());   // ✅ 或者显式转换
}
```

记住：**函数参数优先用 `&str`**（更通用），返回值优先用 `String`（拥有所有权，不欠别人生命周期）。

**坑 2：match 忘记穷尽**

```rust
enum 状态 { 进行中, 已完成, 已取消 }
fn main() {
    let s = 状态::进行中;
    // ❌ 漏掉 已取消 分支会编译错误：non-exhaustive patterns
    match s {
        状态::进行中 => println!("加油"),
        状态::已完成 => println!("棒"),
        状态::已取消 => println!("可惜"),
        // 或者加兜底：_ => {}
    }
}
```

这不是刁难你——当以后给枚举加了新成员，编译器会逼着你处理每一处 match，杜绝"忘了改某个 switch"的经典 Bug。

**坑 3：闭包意外 move**

```rust
fn main() {
    let name = String::from("Rust");
    // 需要 'static 或跨线程的闭包会要求 move
    let f = move || println!("{name}");
    f();
    // println!("{name}"); // ❌ name 已被 move 进闭包
    // ✅ 修复：闭包内只用引用，或提前 clone
    let name2 = String::from("Rust");
    let g = || println!("{name2}"); // 只借用，不 move
    g();
    println!("{name2}"); // 正常
}
```

**坑 4：循环中借用冲突**

```rust
fn main() {
    let mut scores = vec![85, 92, 78];
    // ❌ 边遍历边修改：iter() 是不可变借用，push 需要可变借用
    // for s in scores.iter() {
    //     if *s < 80 { scores.push(60); }
    // }

    // ✅ 先收集再修改
    let bad: Vec<i32> = scores.iter().filter(|&&s| s < 80).map(|_| 60).collect();
    scores.extend(bad);
    println!("{scores:?}");
}
```

**坑 5：`unwrap` 满天飞**

```rust
// ❌ 数据文件损坏时程序直接 panic
let n: i32 = "abc".parse().unwrap();

// ✅ 优雅处理：? 传播或 match 兜底
let n: i32 = "abc".parse().unwrap_or(0);        // 给默认值
// 或在返回 Result 的函数里：let n: i32 = s.parse()?;
```

**坑 6：用 `==` 比较浮点数**

```rust
fn main() {
    let a: f64 = 0.1 + 0.2;
    println!("{}", a == 0.3); // false！浮点精度陷阱（所有语言都有）
    // ✅ 用误差范围比较
    println!("{}", (a - 0.3).abs() < 1e-10); // true
}
```

**坑 7：整数溢出**

```rust
fn main() {
    let mut x: u8 = 255;
    // x += 1; // debug 模式 panic，release 模式回绕成 0！
    x = x.wrapping_add(1);      // ✅ 明确要回绕：0
    // x = x.checked_add(1);    // ✅ 或返回 Option：None
    println!("{x}");
}
```

**坑 8：切片索引越界**

```rust
fn main() {
    let v = vec![1, 2, 3];
    // let x = v[5];          // ❌ 运行时 panic
    let x = v.get(5);         // ✅ 返回 Option<i32>
    match x {
        Some(n) => println!("{n}"),
        None => println!("索引不存在"),
    }
}
```

**坑 9：对迭代器 collect 的类型推导失败**

```rust
fn main() {
    let s = "1,2,3";
    // ❌ let nums = s.split(',').collect(); // 编译错误：collect 成什么？
    // ✅ 显式标注目标类型
    let nums: Vec<&str> = s.split(',').collect();
    // 或用 turbofish 语法
    let nums2 = s.split(',').collect::<Vec<&str>>();
    println!("{nums:?} {nums2:?}");
}
```

**坑 10：以为 `let x = y;` 之后 `y` 还能用**

```rust
fn main() {
    let a = String::from("hello");
    let b = a;               // move！不是拷贝
    // println!("{a}");      // ❌ error[E0382]
    println!("{b}");         // ✅
    // 注意：i32 等 Copy 类型是例外——赋值即拷贝，双方都可用
    let x = 5;
    let y = x;
    println!("{x} {y}");     // ✅ i32 实现了 Copy
}
```

> 💡 记住铁律：**赋值、传参、返回，默认都是 move**。只有实现了 `Copy` 的简单类型（整数、浮点、布尔、字符等）才是拷贝。不确定时，编译器会告诉你。

### 本章小结

- RustRover 调试三板斧：断点 + 变量查看 + `Alt+F8` 表达式求值；
- 内联提示让编译器的类型推导和借用分析"可视化"；
- 遇错先按 `Alt + Enter`，编译器的建议往往就是答案；
- E0382（move 后使用）、E0502（借用冲突）、E0106（缺生命周期）是三大高频错误，掌握速查表能解决 80% 的编译报错；
- 十大新手坑的共同解药：理解所有权、读全编译器信息、少写 `unwrap`。

---

## 第 20 章 下一步学习路线

读到这里，你已经完成了从"Hello World"到"命令行应用发布"的完整旅程。但 Rust 的世界远不止于此。本章是你的藏宝图。

### 20.1 官方资源：常读常新

- **《The Rust Programming Language》（Rust Book）**
  <https://doc.rust-lang.org/book/>
  官方圣经，本教程的知识结构很大程度上致敬了它。建议你现在回头重读——有了实践经验后，很多当初囫囵吞枣的章节会突然通透。中文版社区译本项目可搜索 "Rust 程序设计语言 中文版"。

- **Rustlings** <https://github.com/rust-lang/rustlings>
  官方出品的交互式小练习集：几十个故意写坏的小程序，你负责修好它们。`cargo install rustlings` 后跟着提示一关关过，是巩固本教程所有概念的最佳方式。

- **Rust by Example** <https://doc.rust-lang.org/rust-by-example/>
  "不讲道理，直接上代码"的官方示例集。当你忘记某个语法（比如"闭包三种 Fn trait 怎么区分"）时，来这里查比翻书快。

- **标准库文档** <https://doc.rust-lang.org/std/>
  Rust 的文档质量极高，每个函数都有可运行的示例。养成习惯：写代码前先问自己"标准库是不是已经有现成的了？"——`Option`、`Result`、`Vec`、`HashMap` 上的方法远比你以为的丰富。

> 💡 还有个隐藏技巧：任何项目里运行 `cargo doc --open`，会生成并打开**本项目及所有依赖**的本地文档，没网也能查。

### 20.2 推荐阅读的开源项目

读优秀代码是进阶的最短路径。按难度排序：

1. **mini-redis**（<https://github.com/tokio-rs/mini-redis>）：tokio 官方教学项目，代码量小、注释详尽，是进入异步世界的最佳跳板；
2. **ripgrep**（<https://github.com/BurntSushi/ripgrep>）：比 grep 快一个数量级的搜索工具，展示了如何用迭代器和 SIMD 榨干性能；
3. **bat**（<https://github.com/sharkdp/bat>）：带语法高亮的 `cat`，代码组织清晰，适合学习 CLI 应用的工程实践；
4. **clap / serde 的源码**：你已经在用它们，看看 derive 宏内部是怎么工作的，会对"元编程"祛魅。

> ⚠️ 不建议新手一上来就读 rustc、tokio 内核这类大型代码库——就像刚学会游泳就横渡海峡。从几千行的项目开始。

### 20.3 预告：异步编程

本教程刻意没有展开 **async/await**，因为它值得你专门学一遍。这里先埋个钩子：

```rust
// 异步函数：不阻塞线程地等待网络/IO
async fn fetch_title(url: &str) -> String {
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();
    format!("下载了 {} 字节", body.len())
}

#[tokio::main]  // 属性宏：把 main 变成异步运行时入口
async fn main() {
    // 并发地发起多个请求（注意：不是多线程！是单线程内的任务切换）
    let (a, b) = tokio::join!(
        fetch_title("https://www.rust-lang.org"),
        fetch_title("https://crates.io"),
    );
    println!("{a}, {b}");
}
```

关键概念预告：

- **`async fn`** 返回的不是结果，而是一个 **Future**（"未来某个时刻才会有值"的承诺）；
- **`.await`** 挂起当前任务，把线程让给其他任务——一个线程能同时跑成千上万个任务；
- **tokio**（<https://tokio.rs>）是事实标准的异步运行时，提供定时器、网络、文件 IO 等全套设施；
- 官方免费教程 **《Asynchronous Programming in Rust》**（async-book）等你准备好了再读。

> 💡 好消息：你已经掌握了异步编程最难的前置知识——所有权和生命周期。异步 Rust 里 90% 的"痛苦"本质都是这两个概念的延伸。

### 20.4 社区资源：别单打独斗

- **This Week in Rust**（<https://this-week-in-rust.org>）：每周一期的社区 newsletter，新库、好文、招聘一网打尽，保持对生态的体感；
- **Rust 中文社区**（<https://rustcc.cn>）：中文资讯、翻译文章、招聘板块；
- **Rust 语言中文社区论坛 / 微信群 / Telegram**：遇到卡住的问题，社区响应速度很快；提问时记得附上完整错误信息和最小复现代码；
- **r/rust**（Reddit）与 **Rust Users Forum**（<https://users.rust-lang.org>）：英文社区，官方团队成员经常亲自答疑；
- ** crates.io**：Rust 的包仓库，找库时看下载量和维护活跃度（最近一年有无更新）；
- **awesome-rust**（<https://github.com/rust-unofficial/awesome-rust>）：各领域精选库的大列表，"Rust 能不能做 XXX"先来这里查。

### 20.5 最后的建议

Rust 的学习曲线像一堵墙，但你已经翻过了最陡的那段。接下来：

1. **写，不要只读**：给自己定个小项目——一个静态站点生成器、一个文件同步工具、一个终端小游戏。每一行自己敲出来的代码，胜过十篇教程；
2. **和编译器做朋友**：它不是在刁难你，而是在替你 Review。读懂每一条错误信息，你的工程直觉会飞速成长；
3. **参与开源**：从给喜欢的项目修文档 typo、补充测试开始，Rust 社区对新手极其友好。

愿借用检查器永远站在你这边。Happy hacking! 🦀

### 本章小结

- 官方四大件：Rust Book、Rustlings、Rust by Example、std 文档——常读常新；
- 读代码从 mini-redis、ripgrep、bat 这类中小型项目开始；
- 异步编程（async/await + tokio）是下一个主战场，你已具备全部前置知识；
- 订阅 This Week in Rust，加入中文社区，保持与生态的连接。

---

**（第四篇 完 · 全书完）**
