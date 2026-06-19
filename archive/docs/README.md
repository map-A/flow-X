# FlowX 项目文档中心

欢迎来到FlowX项目！这里包含了完整的项目管理和开发文档。

---

## 📋 项目概述

**FlowX** 是一个AI驱动的智能自动化平台，提供双模式输入（AI自然语言 + 编程脚本），底层由高性能Rust引擎统一执行。

**核心特点**：
- 🤖 AI自然语言驱动
- 🐍 Python脚本（主推）
- 📱 兼容AutoJS（获客）
- ⚡ Rust高性能引擎
- 🔄 跨平台支持

**项目周期**：12周  
**团队规模**：5人  
**目标**：MVP v1.0

---

## 🗂️ 文档导航

### 1️⃣ 项目管理文档

| 文档 | 说明 | 阅读对象 |
|------|------|---------|
| [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md) | 项目总览、里程碑、角色分工 | 全员 |
| [TASK_ASSIGNMENT.md](TASK_ASSIGNMENT.md) | 任务分配总览、进度追踪 | 项目经理、全员 |

### 2️⃣ 架构设计文档（architecture/）

| 任务ID | 文档 | 负责人 | 工时 | 状态 |
|--------|------|--------|------|------|
| ARCH-001 | [系统架构设计](architecture/ARCH-001-系统架构设计.md) | 架构师 | 40h | 📝 待开始 |
| ARCH-002 | [统一指令集设计](architecture/ARCH-002-统一指令集设计.md) | 架构师 | 24h | 📝 待开始 |
| ARCH-003 | [平台抽象层设计](architecture/ARCH-003-平台抽象层设计.md) | 架构师 | 16h | 📝 待开始 |

**总计**：80小时

### 3️⃣ 开发任务文档（development/）

#### Rust核心开发

| 任务ID | 文档 | 负责人 | 工时 | 依赖 | 状态 |
|--------|------|--------|------|------|------|
| DEV-001 | [Rust核心引擎](development/DEV-001-Rust核心引擎.md) | Rust开发 | 80h | ARCH-001,002 | 📝 待开始 |
| DEV-002 | [Android平台实现](development/DEV-002-Android平台实现.md) | Rust开发 | 60h | ARCH-003, DEV-001 | 📝 待开始 |
| DEV-003 | [视觉能力模块](development/DEV-003-视觉能力模块.md) | Rust开发 | 40h | DEV-001 | 📝 待开始 |

**小计**：180小时

#### Python/JS开发

| 任务ID | 文档 | 负责人 | 工时 | 依赖 | 状态 |
|--------|------|--------|------|------|------|
| DEV-004 | [Python FFI绑定](development/DEV-004-Python-FFI绑定.md) | Python开发 | 60h | DEV-001,002 | 📝 待开始 |
| DEV-005 | [JavaScript运行时](development/DEV-005-JavaScript运行时.md) | JS开发 | 40h | DEV-001 | 📝 待开始 |
| DEV-006 | [AutoJS兼容层](development/DEV-006-AutoJS兼容层.md) | JS开发 | 30h | DEV-005 | 📝 待开始 |

**小计**：130小时

#### AI开发

| 任务ID | 文档 | 负责人 | 工时 | 依赖 | 状态 |
|--------|------|--------|------|------|------|
| DEV-007 | [AI模型集成](development/DEV-007-AI模型集成.md) | AI工程师 | 60h | DEV-004 | 📝 待开始 |
| DEV-008 | 自然语言理解 | AI工程师 | 40h | DEV-007 | 📝 待开始 |

**小计**：100小时

### 4️⃣ 测试文档（testing/）

| 任务ID | 文档 | 负责人 | 工时 | 状态 |
|--------|------|--------|------|------|
| TEST-001 | [测试策略](testing/TEST-001-测试策略.md) | 测试工程师 | 40h | 📝 待开始 |
| TEST-002 | 单元测试计划 | 测试工程师 | 30h | 📝 待开始 |
| TEST-003 | 集成测试计划 | 测试工程师 | 30h | 📝 待开始 |
| TEST-004 | 端到端测试 | 测试工程师 | 40h | 📝 待开始 |

**总计**：140小时

---

## 🎯 快速开始

### 对于项目经理
1. 阅读 [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)
2. 查看 [TASK_ASSIGNMENT.md](TASK_ASSIGNMENT.md)
3. 追踪每周进度和风险

### 对于架构设计师
1. 从 [ARCH-001](architecture/ARCH-001-系统架构设计.md) 开始
2. 完成架构设计三部曲（ARCH-001/002/003）
3. 组织架构评审会

### 对于Rust核心开发
1. 等待架构设计完成
2. 阅读 [DEV-001](development/DEV-001-Rust核心引擎.md)
3. 搭建项目骨架

### 对于Python/JS开发
1. 研究PyO3和QuickJS
2. 阅读 [DEV-004](development/DEV-004-Python-FFI绑定.md)
3. 等待Rust核心完成

### 对于AI工程师
1. 调研AI模型（GLM-4V, Qwen-VL）
2. 阅读 [DEV-007](development/DEV-007-AI模型集成.md)
3. 准备模型环境

### 对于测试工程师
1. 阅读 [TEST-001](testing/TEST-001-测试策略.md)
2. 搭建测试框架
3. 编写测试用例

---

## 📅 里程碑

| 里程碑 | 时间 | 交付物 |
|--------|------|--------|
| **M1: 架构完成** | Week 4 | 架构文档、核心框架 |
| **M2: 脚本引擎就绪** | Week 8 | Python/JS可运行脚本 |
| **M3: AI集成完成** | Week 10 | AI自然语言操作 |
| **M4: MVP发布** | Week 12 | 完整产品 |

---

## 📊 任务依赖关系

```
架构设计 (Week 1-4)
    ↓
Rust核心引擎 (Week 3-6)
    ↓
┌─────────────┼─────────────┐
│             │             │
Android平台   Python FFI    JS运行时
(Week 4-6)   (Week 5-7)   (Week 7-8)
│             │             │
│             ↓             ↓
│         AI集成       AutoJS兼容
│        (Week 9-10)   (Week 8)
│             │             │
└─────────────┴─────────────┘
              ↓
       集成测试与发布
         (Week 11-12)
```

---

## 🔍 文档使用指南

### 查找任务
```bash
# 查看所有任务
ls docs/**/*.md

# 按角色查找
ls docs/architecture/  # 架构师
ls docs/development/   # 开发人员
ls docs/testing/       # 测试人员
```

### 更新进度
每个任务文档都有状态标记，完成后更新：
```markdown
**状态**：✅ 已完成  （或 🚧 进行中、📝 待开始）
```

### 添加新任务
1. 复制现有任务文档作为模板
2. 更新任务ID、负责人、工时
3. 更新 [TASK_ASSIGNMENT.md](TASK_ASSIGNMENT.md)

---

## ⚠️ 注意事项

1. **依赖关系**：严格按照依赖顺序执行任务
2. **阻塞处理**：遇到阻塞及时在站会提出
3. **文档更新**：任务完成后及时更新文档状态
4. **代码评审**：关键模块需要团队评审
5. **测试先行**：边开发边测试，不要积压

---

## 📞 联系方式

- **项目经理**：[待定]
- **技术负责人**：[待定]
- **Slack频道**：#flowx-dev
- **每日站会**：9:30-9:45
- **周会**：每周五下午

---

## 📚 参考资料

- [完整PRD](../PRD.md)
- [PRD总结](../PRD_SUMMARY.md)
- [Rust官方文档](https://doc.rust-lang.org/)
- [PyO3文档](https://pyo3.rs/)
- [AutoJS文档](https://www.autojs.cc/docs/)

---

**文档维护人**：项目经理  
**最后更新**：2026-06-10  
**版本**：v1.0
