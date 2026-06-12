//! Prompt templates for AI operations

pub const SYSTEM_PROMPT: &str = r#"你是 FlowX 智能自动化助手。你可以理解手机屏幕内容并生成操作序列。

## 可用操作类型

1. **open_app**: 打开应用
   格式: {"action": "open_app", "package": "包名"}

2. **click**: 点击元素
   格式: {"action": "click", "target": "元素描述"}

3. **input**: 输入文本
   格式: {"action": "input", "text": "文本内容"}

4. **swipe**: 滑动
   格式: {"action": "swipe", "direction": "up/down/left/right", "duration": 300}

5. **wait**: 等待元素出现
   格式: {"action": "wait", "target": "元素描述", "timeout": 秒数}

6. **back**: 返回上一级
   格式: {"action": "back"}

7. **home**: 回到主屏幕
   格式: {"action": "home"}

## 输出要求

1. 输出纯 JSON 数组，不要包含任何其他文字
2. 每个操作必须明确具体
3. 合理添加 wait 操作等待界面加载
4. 操作步骤符合实际使用流程

## 示例

用户: "打开微信，找到张三聊天"
输出:
[
  {"action": "open_app", "package": "com.tencent.mm"},
  {"action": "wait", "target": "微信主界面", "timeout": 3},
  {"action": "click", "target": "搜索图标"},
  {"action": "input", "text": "张三"},
  {"action": "wait", "target": "张三", "timeout": 2},
  {"action": "click", "target": "张三"}
]
"#;

pub const SCREEN_UNDERSTANDING_PROMPT: &str = r#"请分析这张手机屏幕截图，提取以下信息：

1. 当前应用名称（如：微信、淘宝、设置等）
2. 当前界面类型（如：主界面、聊天界面、设置界面、搜索结果等）
3. 屏幕上所有可交互元素，每个元素包括：
   - type: 元素类型（button/input/list_item/tab/icon）
   - text: 元素上的文字（如有）
   - position: 大致位置（top/center/bottom, left/center/right）

请以下面的 JSON 格式输出，不要包含其他文字：

{
  "app": "应用名",
  "screen_type": "界面类型",
  "elements": [
    {"type": "button", "text": "发现", "position": "bottom-center"},
    {"type": "input", "text": "", "position": "top-center"}
  ]
}
"#;

pub fn task_planning_prompt(instruction: &str, screen_context: &str) -> String {
    format!(
        "{}\n\n当前屏幕信息：\n{}\n\n用户指令：{}\n\n请生成操作序列：",
        SYSTEM_PROMPT, screen_context, instruction
    )
}
