# Flashcard CLI

一个用 Rust 编写的命令行抽认卡应用，帮助用户通过间隔重复学习记忆知识。

## 功能特性

- 📚 从 JSON 文件加载抽认卡
- 🎲 随机抽取卡片
- 📊 跟踪学习进度（score 分数）
- 💾 自动保存学习状态
- 🎯 支持多种抽卡模式（random/ordered）

## 项目结构

```
flashcardCLI/
├── Cargo.toml           # 项目配置和依赖
├── src/
│   ├── main.rs          # 程序入口点
│   ├── carddeck.rs      # CardDeck 结构体和方法
│   ├── flashcard.rs     # FlashCard 结构体定义
│   └── config.rs        # 命令行参数解析
└── README.md            # 项目文档
```

## 数据结构

### FlashCard
```rust
pub struct FlashCard {
    pub question: String,  // 问题
    pub answer: String,    // 答案
    pub score: i32,        // 学习分数（权重）
}
```

### CardDeck
```rust
pub struct CardDeck {
    pub cards: Vec<FlashCard>,  // 卡片集合
    pub file_path: String,      // 数据文件路径
}
```

## 安装

### 前置要求
- Rust 1.70+
- Cargo

### 编译安装
```bash
# 克隆项目
git clone <repository-url>
cd flashcardCLI

# 编译
cargo build --release

# 运行
cargo run -- <cards.json>
```

## 使用方法

### 基本用法
```bash
# 默认随机模式
cargo run -- cards.json

# 指定模式
cargo run -- random cards.json
cargo run -- ordered cards.json
```

### 卡片文件格式

创建 `cards.json` 文件：
```json
[
  {
    "question": "What is ownership?",
    "answer": "A set of rules for managing memory",
    "score": 0
  },
  {
    "question": "What is borrowing?",
    "answer": "References that let you use data without taking ownership",
    "score": 0
  }
]
```

### 交互流程
1. 程序显示问题
2. 按回车查看答案
3. 输入 `y` 表示记得，`n` 表示忘记
4. 程序自动更新分数并保存

## 核心方法

### CardDeck::new(path)
- 从 JSON 文件加载卡片
- 返回 `Result<CardDeck, Box<dyn Error>>`

### CardDeck::save()
- 将当前卡片数据保存到文件
- 使用 `serde_json::to_string_pretty` 格式化输出

### CardDeck::pick_random()
- 随机抽取一张卡片
- 可基于 score 分数进行加权随机

### CardDeck::update_score(index, correct)
- 更新指定卡片的学习分数
- `correct: true` 增加分数，`false` 减少分数

## 依赖项

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

可选依赖（用于随机抽取）：
```toml
rand = "0.8"
# 或
fastrand = "2.0"
```

## 开发计划

### 已完成
- [x] 基础项目结构
- [x] FlashCard/CardDeck 结构体定义
- [x] 命令行参数解析（config.rs）

### 待完成
- [ ] 实现 CardDeck::new() 方法
- [ ] 实现 CardDeck::save() 方法
- [ ] 实现 CardDeck::pick_random() 方法
- [ ] 实现 CardDeck::update_score() 方法
- [ ] 添加随机数依赖
- [ ] 实现主循环逻辑
- [ ] 创建示例 cards.json

## 运行示例

```bash
$ cargo run -- cards.json
? What is ownership?
   （按回车查看答案）

A: A set of rules for managing memory

你记得这个答案吗？(y/n): y
✓ 分数已更新: 1

? What is borrowing?
   ...
```

## 错误处理

程序使用 `Box<dyn std::error::Error>` 统一处理错误：
- 文件读取错误
- JSON 解析错误
- 命令行参数错误

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！