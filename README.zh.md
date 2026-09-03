# pier-rs-mod-template

用 **Rust** 给 Minecraft 基岩版服务端写 [LeviLamina](https://github.com/LiteLDev/LeviLamina)
模组的起手模板，不需要 C++ 工具链。

本模板面向 [**Pier**](https://github.com/Maskviva/pier) —— 让 LeviLamina 能装载其它语言
所写模组的装载器。请先在服务器上装好 Pier，否则这个模组根本不会被扫到。

[English](README.md)

## 上手

1. 点上面的 **Use this template** 生成新仓库，克隆下来。
2. 改三处名字，让它们互相对得上：
   - `Cargo.toml` 的 `[package] name`
   - `manifest.json` 的 `name`（必须和安装目录同名）和 `entry`（cargo 产出的 DLL 名。
     cargo 会把 crate 名里的连字符变成下划线，所以 `my-mod` 产出 `my_mod.dll`）
   - `src/lib.rs` 里的 `HelloWorld` 结构体和提到它的那些字符串
3. 写你的模组。示范的四块里不需要的直接删掉。
4. 构建与安装：

   ```bash
   cargo build --release
   ```

   把 `target/release/<你的 crate>.dll` 和 `manifest.json` 一起放进
   `<服务器>/mods/<你的模组名>/`。`.github/` 里的工作流会在每次推送时自动构建，
   并在每次发版时附上打包好的压缩包。

## 模板示范了什么

`src/lib.rs` 是一个能跑的模组，不是空壳：

- **日志**走模组自己的 logger，所以控制台显示的是 `[你的模组名]` 而不是 `[pier]`。
- **可取消的事件订阅**：记录每条聊天，含 "creeper" 的拦下。订阅句柄存在结构体里，
  丢掉它就自动退订。
- **自定义命令** `/hello [name]`。
- **延迟任务**，回到服务器线程上执行一条控制台命令。

## 动手之前值得知道的几件事

**manifest 里的 `"type": "pier"` 是宿主找到这个模组的唯一依据。** 写错一个字，模组
根本不会被扫到，而且任何地方都不会报错。

**用 `names` 里的常量，不要写事件 id 字面量。** `event::subscribe` 传一个拼错的 id
会订阅成功，然后一次都不触发；用 `names::PLAYER_CHAT` 则在编译期就报错。

**读不出来的值一律是 `Err`，绝不会是一个默认值。** 事件载荷缺字段是真的错误 ——
把读不到的维度当成主世界，正是土地保护被绕过的原因。请处理 `Err`，不要写
`unwrap_or(0)`。

**回调跑在服务器线程上**，数据包拦截回调除外，它跑在抽水那条连接的线程上。
后台线程要把活儿交回服务器线程，用 `Host::get().schedule()` 或 `schedule_after()`。

## 最小的模组

```rust
use levilamina::prelude::*;

struct MyMod;

impl LeviMod for MyMod {
    fn on_load(ctx: &ModContext) -> Result<Self> {
        ctx.logger().info("hello from Rust");
        Ok(MyMod)
    }
}

levilamina::register_mod!(MyMod);
```

`on_enable`、`on_disable`、`on_unload` 都是可选的，默认什么都不做。

## 环境要求

- Rust，目标 `x86_64-pc-windows-msvc`（Windows 上的默认目标）
- [LeviLamina](https://lamina.levimc.org/) 26.20.4，BDS 1.26.20
- 目标服务器上装好 [Pier](https://github.com/Maskviva/pier)

## 去哪里提问

关于这个骨架本身的问题提在这里。缺 API、或者装载器本身的 bug，请提到
[Pier](https://github.com/Maskviva/pier/issues)。

## 许可

CC0-1.0，公有领域。起手代码随便用，不需要署名。

---

*与 Mojang、Microsoft、LeviMC 无关。Minecraft 是 Mojang Synergies AB 的商标。*
