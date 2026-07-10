# Flashcard CLI 开发指南

> **重要规则：不要直接修改项目代码。** 仅提供指导和建议，让用户自行完成实现。

## 项目状态

项目已搭建基础结构（Cargo.toml、FlashCard/CardDeck 结构体、load_cards），需要完成以下功能：

## 待完成任务

### 1. 完善 `CardDeck` 方法

在 `main.rs` 中为 `CardDeck` 实现方法：

- `new(path: &str) -> Result<CardDeck, ...>` — 从文件加载卡片构建牌组
- `save(&self) -> Result<(), ...>` — 将当前卡片数据写回 JSON 文件
- `pick_random(&self) -> &FlashCard` — 随机抽取一张卡片（使用 `rand` crate 或简单取模）
- `update_score(&mut self, index: usize, correct: bool)` — 根据用户反馈更新卡片 score

### 2. 随机抽取依赖

在 `Cargo.toml` 添加随机数依赖（选一个）：
- `rand = "0.8"` — 标准随机库
- 或使用 `fastrand` 更轻量

### 3. 实现主循环逻辑

在 `main()` 中：
1. 从命令行参数获取 JSON 文件路径
2. 创建 `CardDeck` 实例
3. 循环：
   - 随机选卡并显示问题
   - 等待用户按回车
   - 显示答案
   - 提示用户输入 `y`（记得）或 `n`（忘记）
   - 更新对应卡片 score
   - 保存数据到文件

### 4. 完善 `config.rs`（可选）

添加命令行参数解析，例如使用 `clap` 或手动处理：
- `flashcard <cards.json>` — 指定卡片文件路径
- `flashcard --help` — 显示帮助

### 5. 创建示例 `cards.json`

```json
[
  { "question": "What is ownership?", "answer": "A set of rules for managing memory", "score": 0 },
  { "question": "What is borrowing?", "answer": "References that let you use data without taking ownership", "score": 0 }
]
```

## 运行方式

```bash
cargo run -- cards.json
```

## 关键点

- `serde` derive 宏已在 Cargo.toml 配置，可直接序列化/反序列化
- `score` 字段可作为权重，score 越高越少抽到（或反之）
- 保存时用 `serde_json::to_string_pretty` 格式化输出
