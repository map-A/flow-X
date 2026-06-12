# DEV-005: JavaScript运行时集成

**任务ID**：DEV-005  
**负责角色**：Python/JS开发  
**优先级**：P1（高）  
**预计工时**：40小时  
**依赖**：DEV-001  
**阶段**：Phase 2 (Week 7-8)

---

## 任务目标

集成QuickJS引擎，实现JavaScript脚本执行能力，为AutoJS兼容层提供运行时支持。

---

## 技术栈

- QuickJS (轻量级JS引擎)
- rquickjs (Rust绑定)

---

## 交付物

### 1. 依赖配置

```toml
# flowx-core/Cargo.toml
[dependencies]
rquickjs = { version = "0.4", features = ["array-buffer", "classes"] }
```

### 2. JS运行时封装 (`scripting/js_runtime.rs`)

```rust
use rquickjs::{Context, Runtime, Error as JsError};
use crate::engine::{Command, CommandExecutor};
use std::sync::Arc;

pub struct JsRuntime {
    runtime: Runtime,
    executor: Arc<CommandExecutor>,
}

impl JsRuntime {
    pub fn new(executor: Arc<CommandExecutor>) -> Result<Self> {
        let runtime = Runtime::new()?;
        Ok(Self { runtime, executor })
    }
    
    pub fn execute_script(&self, script: &str) -> Result<()> {
        let context = Context::full(&self.runtime)?;
        
        context.with(|ctx| {
            // 注册全局函数
            self.register_globals(&ctx)?;
            
            // 执行脚本
            ctx.eval(script)?;
            
            Ok(())
        })
    }
    
    fn register_globals(&self, ctx: &rquickjs::Ctx) -> Result<()> {
        let executor = self.executor.clone();
        
        // 注册click函数
        ctx.globals().set("click", 
            rquickjs::Function::new(ctx.clone(), move |x: i32, y: i32| {
                // 调用executor
                executor.execute(Command::Click { x, y })
            })?
        )?;
        
        // TODO: 注册其他全局函数
        
        Ok(())
    }
}
```

### 3. AutoJS兼容API实现 (`js-runtime/stdlib/`)

```javascript
// app.js - 应用管理
const app = {
    launch: function(packageName) {
        __native_open_app(packageName);
    },
    
    launchApp: function(name) {
        this.launch(name);
    }
};

// ui.js - 元素查找
function text(str) {
    return new UiSelector("text", str);
}

function id(str) {
    return new UiSelector("id", str);
}

class UiSelector {
    constructor(type, value) {
        this.type = type;
        this.value = value;
    }
    
    findOne() {
        let result = __native_find_element(this.type, this.value);
        if (result) {
            return new UiObject(result);
        }
        return null;
    }
}

class UiObject {
    constructor(data) {
        this.bounds = data.bounds;
        this.text = data.text;
    }
    
    click() {
        let center = {
            x: this.bounds.x + this.bounds.width / 2,
            y: this.bounds.y + this.bounds.height / 2
        };
        __native_click(center.x, center.y);
        return true;
    }
}

// 全局函数
function sleep(ms) {
    __native_sleep(ms);
}

function toast(message) {
    __native_toast(message);
}
```

### 4. 原生函数注册

```rust
fn register_native_functions(ctx: &rquickjs::Ctx, executor: Arc<CommandExecutor>) -> Result<()> {
    let globals = ctx.globals();
    
    // __native_click
    {
        let exec = executor.clone();
        globals.set("__native_click", 
            rquickjs::Function::new(ctx.clone(), move |x: i32, y: i32| {
                exec.execute(Command::Click { x, y })
            })?
        )?;
    }
    
    // __native_find_element
    {
        let exec = executor.clone();
        globals.set("__native_find_element",
            rquickjs::Function::new(ctx.clone(), move |type_: String, value: String| {
                let selector = match type_.as_str() {
                    "text" => Selector::Text(value),
                    "id" => Selector::Id(value),
                    _ => return Err("Unknown selector type"),
                };
                
                exec.execute(Command::FindElement { selector })
            })?
        )?;
    }
    
    // __native_sleep
    globals.set("__native_sleep",
        rquickjs::Function::new(ctx.clone(), |ms: u64| {
            std::thread::sleep(std::time::Duration::from_millis(ms));
            Ok(())
        })?
    )?;
    
    Ok(())
}
```

---

## 验收标准

- [ ] QuickJS集成成功
- [ ] 能够执行简单JS脚本
- [ ] AutoJS核心API实现（app, text, id, click, sleep）
- [ ] 运行AutoJS示例脚本成功

---

## 测试脚本

```javascript
// test.js
auto.waitFor();

// 打开微信
app.launch("com.tencent.mm");
sleep(2000);

// 查找元素
let btn = text("发现").findOne();
if (btn) {
    btn.click();
    toast("点击成功");
}
```

---

**创建日期**：2026-06-10  
**状态**：待开始
