import { useState, useEffect } from 'react';
import Editor from '@monaco-editor/react';
import './App.css';

interface Script {
  name: string;
  path: string;
  content: string;
}

interface Device {
  id: string;
  name: string;
  platform: string;
  status: string;
  resolution: [number, number];
}

// 直接使用 window.__TAURI__ 的全局变量
declare global {
  interface Window {
    __TAURI__: any;
  }
}

function App() {
  const [scripts, setScripts] = useState<Script[]>([]);
  const [devices, setDevices] = useState<Device[]>([]);
  const [currentScript, setCurrentScript] = useState<Script | null>(null);
  const [code, setCode] = useState('');
  const [output, setOutput] = useState('FlowX Desktop\n');
  const [screenshot, setScreenshot] = useState('');
  const [ready, setReady] = useState(false);
  const [showConnectInput, setShowConnectInput] = useState(false);
  const [connectUri, setConnectUri] = useState('android://localhost:6789');
  const [mouseCoords, setMouseCoords] = useState({ x: 0, y: 0 });
  const [autoCapture, setAutoCapture] = useState(false);
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [newScriptName, setNewScriptName] = useState('');
  const [showRenameDialog, setShowRenameDialog] = useState(false);
  const [renameTarget, setRenameTarget] = useState<Script | null>(null);
  const [renameName, setRenameName] = useState('');

  // 初始化 - 等待 Tauri 就绪
  useEffect(() => {
    let count = 0;
    const timer = setInterval(() => {
      if (window.__TAURI__?.core?.invoke) {
        console.log('Tauri API ready!');
        setReady(true);
        setOutput(prev => prev + 'Tauri API ready\n');
        clearInterval(timer);

        // 加载初始数据
        loadScripts();
        loadDevices();
      } else {
        count++;
        console.log(`Waiting for Tauri... attempt ${count}`);
        if (count > 100) {
          setOutput(prev => prev + 'ERROR: Tauri API not available\n');
          clearInterval(timer);
        }
      }
    }, 100);

    return () => clearInterval(timer);
  }, []);

  const invoke = (cmd: string, args?: any) => {
    if (!window.__TAURI__?.core?.invoke) {
      throw new Error('Tauri not ready');
    }
    return window.__TAURI__.core.invoke(cmd, args);
  };

  const loadScripts = async () => {
    try {
      const result = await invoke('get_scripts');
      setScripts(result);
      console.log('Scripts loaded:', result);
    } catch (e) {
      console.error('Load scripts error:', e);
      setOutput(prev => prev + `Error: ${e}\n`);
    }
  };

  const loadDevices = async () => {
    try {
      const result = await invoke('get_devices');
      setDevices(result);
      console.log('Devices loaded:', result);
    } catch (e) {
      console.error('Load devices error:', e);
    }
  };

  const createScript = async () => {
    setNewScriptName(`script_${Date.now()}.py`);
    setShowCreateDialog(true);
  };

  const doCreateScript = async () => {
    const name = newScriptName.trim();
    if (!name) {
      setOutput(prev => prev + 'Error: Script name is empty\n');
      return;
    }

    if (!name.endsWith('.py')) {
      setOutput(prev => prev + 'Error: Script name must end with .py\n');
      return;
    }

    setShowCreateDialog(false);

    try {
      await invoke('create_script', { name });
      await loadScripts();
      setOutput(prev => prev + `Created: ${name}\n`);
    } catch (e) {
      setOutput(prev => prev + `Error: ${e}\n`);
    }
  };

  const openScript = async (script: Script) => {
    try {
      const loaded = await invoke('load_script', { name: script.name });
      setCurrentScript(loaded);
      setCode(loaded.content);
      setOutput(prev => prev + `Opened: ${script.name}\n`);
    } catch (e) {
      setOutput(prev => prev + `Error: ${e}\n`);
    }
  };

  const saveScript = async () => {
    if (!currentScript) return;

    try {
      await invoke('save_script', {
        name: currentScript.name,
        content: code,
      });
      setOutput(prev => prev + `Saved: ${currentScript.name}\n`);
    } catch (e) {
      setOutput(prev => prev + `Error: ${e}\n`);
    }
  };

  const deleteScript = async (name: string) => {
    const confirmed = window.confirm(`确定删除脚本 "${name}" 吗？`);
    if (!confirmed) return;

    try {
      await invoke('delete_script', { name });
      await loadScripts();
      if (currentScript?.name === name) {
        setCurrentScript(null);
        setCode('');
      }
      setOutput(prev => prev + `Deleted: ${name}\n`);
    } catch (e) {
      setOutput(prev => prev + `Error: ${e}\n`);
    }
  };

  const startRename = (script: Script) => {
    setRenameTarget(script);
    setRenameName(script.name);
    setShowRenameDialog(true);
  };

  const doRename = async () => {
    if (!renameTarget) return;

    const newName = renameName.trim();
    if (!newName) {
      setOutput(prev => prev + 'Error: Name is empty\n');
      return;
    }

    if (!newName.endsWith('.py')) {
      setOutput(prev => prev + 'Error: Name must end with .py\n');
      return;
    }

    if (newName === renameTarget.name) {
      setShowRenameDialog(false);
      return;
    }

    setShowRenameDialog(false);

    try {
      // 读取旧文件内容
      const oldScript = (await invoke('load_script', { name: renameTarget.name })) as Script;
      // 创建新文件
      await invoke('create_script', { name: newName });
      // 写入内容
      await invoke('save_script', { name: newName, content: oldScript.content });
      // 删除旧文件
      await invoke('delete_script', { name: renameTarget.name });

      await loadScripts();

      if (currentScript?.name === renameTarget.name) {
        // 如果当前打开的是被重命名的脚本，更新引用
        const newScript = (await invoke('load_script', { name: newName })) as Script;
        setCurrentScript(newScript);
      }

      setOutput(prev => prev + `Renamed: ${renameTarget.name} → ${newName}\n`);
    } catch (e) {
      setOutput(prev => prev + `Error: ${e}\n`);
    }
  };

  const connectDevice = async () => {
    console.log('=== connectDevice function called ===');

    if (!ready) {
      alert('Tauri API not ready yet, please wait...');
      return;
    }

    // 显示输入界面
    setShowConnectInput(true);
  };

  const doConnect = async () => {
    const uri = connectUri.trim();
    console.log('Connecting to:', uri);

    if (!uri) {
      setOutput(prev => prev + 'Error: URI is empty\n');
      return;
    }

    setShowConnectInput(false);
    setOutput(prev => prev + `Connecting to ${uri}...\n`);

    try {
      console.log('Calling invoke with connect_device');
      const result = await invoke('connect_device', { uri });
      console.log('Connect result:', result);

      setOutput(prev => prev + `${result}\n`);
      await loadDevices();
      setTimeout(() => takeScreenshot(), 500);
      // 启动自动截图
      setAutoCapture(true);
    } catch (e) {
      console.error('Connection error:', e);
      setOutput(prev => prev + `Connection failed: ${e}\n`);
    }
  };

  // 自动截图 - 每500ms更新一次
  useEffect(() => {
    if (!autoCapture || !ready) return;

    const interval = setInterval(() => {
      takeScreenshot();
    }, 500);

    return () => clearInterval(interval);
  }, [autoCapture, ready]);

  const takeScreenshot = async () => {
    try {
      const base64 = await invoke('take_screenshot');
      setScreenshot(`data:image/png;base64,${base64}`);
      // 不再输出截图成功信息，避免控制台刷屏
    } catch (e) {
      // 只在真正失败时输出
      console.error('Screenshot failed:', e);
    }
  };

  const runScript = async () => {
    if (!currentScript) return;

    try {
      const result = await invoke('run_script', {
        name: currentScript.name,
      });
      setOutput(prev => prev + result + '\n');
    } catch (e) {
      setOutput(prev => prev + `Error: ${e}\n`);
    }
  };

  const handleScreenClick = (e: React.MouseEvent<HTMLImageElement>) => {
    if (!screenshot) return;

    const rect = e.currentTarget.getBoundingClientRect();
    const scaleX = devices[0]?.resolution[0] || 1080;
    const scaleY = devices[0]?.resolution[1] || 1920;

    const x = Math.round(((e.clientX - rect.left) / rect.width) * scaleX);
    const y = Math.round(((e.clientY - rect.top) / rect.height) * scaleY);

    setCode(prev => prev + `\ndevice.click(${x}, ${y})`);
    setOutput(prev => prev + `Added: click(${x}, ${y})\n`);
  };

  const handleScreenMouseMove = (e: React.MouseEvent<HTMLImageElement>) => {
    if (!screenshot) return;

    const rect = e.currentTarget.getBoundingClientRect();
    const scaleX = devices[0]?.resolution[0] || 1080;
    const scaleY = devices[0]?.resolution[1] || 1920;

    const x = Math.round(((e.clientX - rect.left) / rect.width) * scaleX);
    const y = Math.round(((e.clientY - rect.top) / rect.height) * scaleY);

    setMouseCoords({ x, y });
  };

  return (
    <div className="app">
      {showConnectInput && (
        <div style={{
          position: 'fixed',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          background: 'rgba(0,0,0,0.8)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 1000
        }}>
          <div style={{
            background: '#2d2d30',
            padding: '30px',
            borderRadius: '8px',
            minWidth: '400px'
          }}>
            <h3 style={{ marginBottom: '20px', color: '#d4d4d4' }}>连接设备</h3>
            <input
              type="text"
              value={connectUri}
              onChange={e => setConnectUri(e.target.value)}
              style={{
                width: '100%',
                padding: '10px',
                background: '#1e1e1e',
                border: '1px solid #3e3e42',
                color: '#d4d4d4',
                borderRadius: '4px',
                fontSize: '14px',
                marginBottom: '20px'
              }}
              placeholder="android://localhost:6789"
              autoFocus
            />
            <div style={{ display: 'flex', gap: '10px', justifyContent: 'flex-end' }}>
              <button onClick={() => setShowConnectInput(false)} className="btn-sm">
                取消
              </button>
              <button onClick={doConnect} className="btn-primary">
                连接
              </button>
            </div>
          </div>
        </div>
      )}

      {/* 创建脚本对话框 */}
      {showCreateDialog && (
        <div style={{
          position: 'fixed',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          background: 'rgba(0,0,0,0.8)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 1000
        }}>
          <div style={{
            background: '#2d2d30',
            padding: '30px',
            borderRadius: '8px',
            minWidth: '400px'
          }}>
            <h3 style={{ marginBottom: '20px', color: '#d4d4d4' }}>新建脚本</h3>
            <input
              type="text"
              value={newScriptName}
              onChange={e => setNewScriptName(e.target.value)}
              style={{
                width: '100%',
                padding: '10px',
                background: '#1e1e1e',
                border: '1px solid #3e3e42',
                color: '#d4d4d4',
                borderRadius: '4px',
                fontSize: '14px',
                marginBottom: '20px'
              }}
              placeholder="script_name.py"
              autoFocus
              onKeyPress={e => e.key === 'Enter' && doCreateScript()}
            />
            <div style={{ display: 'flex', gap: '10px', justifyContent: 'flex-end' }}>
              <button onClick={() => setShowCreateDialog(false)} className="btn-sm">
                取消
              </button>
              <button onClick={doCreateScript} className="btn-primary">
                创建
              </button>
            </div>
          </div>
        </div>
      )}

      {/* 重命名脚本对话框 */}
      {showRenameDialog && (
        <div style={{
          position: 'fixed',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          background: 'rgba(0,0,0,0.8)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 1000
        }}>
          <div style={{
            background: '#2d2d30',
            padding: '30px',
            borderRadius: '8px',
            minWidth: '400px'
          }}>
            <h3 style={{ marginBottom: '20px', color: '#d4d4d4' }}>重命名脚本</h3>
            <input
              type="text"
              value={renameName}
              onChange={e => setRenameName(e.target.value)}
              style={{
                width: '100%',
                padding: '10px',
                background: '#1e1e1e',
                border: '1px solid #3e3e42',
                color: '#d4d4d4',
                borderRadius: '4px',
                fontSize: '14px',
                marginBottom: '20px'
              }}
              placeholder="new_name.py"
              autoFocus
              onKeyPress={e => e.key === 'Enter' && doRename()}
            />
            <div style={{ display: 'flex', gap: '10px', justifyContent: 'flex-end' }}>
              <button onClick={() => setShowRenameDialog(false)} className="btn-sm">
                取消
              </button>
              <button onClick={doRename} className="btn-primary">
                重命名
              </button>
            </div>
          </div>
        </div>
      )}

      <header className="header">
        <h1>FlowX Desktop {ready ? '✅' : '⏳'}</h1>
      </header>

      <div className="main">
        {/* Left: Scripts */}
        <aside className="sidebar left">
          <div className="panel">
            <div className="panel-header">
              <h2>Scripts</h2>
              <button onClick={createScript} className="btn-icon" title="New Script">+</button>
            </div>
            <div className="list">
              {scripts.map(s => (
                <div key={s.name} className="list-item" onClick={() => openScript(s)}>
                  <span className="script-name">{s.name}</span>
                  <div style={{ display: 'flex', gap: '4px' }}>
                    <button
                      onClick={(e) => { e.stopPropagation(); startRename(s); }}
                      className="btn-icon-sm"
                      title="重命名"
                    >
                      ✎
                    </button>
                    <button
                      onClick={(e) => { e.stopPropagation(); deleteScript(s.name); }}
                      className="btn-icon-sm"
                      title="删除"
                    >
                      ×
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>

          <div className="panel">
            <div className="panel-header">
              <h2>Devices</h2>
              <button onClick={connectDevice} className="btn-sm" disabled={!ready}>
                Connect
              </button>
            </div>
            <div className="list">
              {devices.map(d => (
                <div key={d.id} className="list-item">
                  <div className="device-name">{d.name}</div>
                  <div className="device-info">{d.resolution[0]}×{d.resolution[1]}</div>
                </div>
              ))}
            </div>
          </div>
        </aside>

        {/* Center: Editor + Console */}
        <main className="content">
          <div className="editor-section">
            <div className="toolbar">
              <span className="file-name">{currentScript?.name || 'No script'}</span>
              <div className="toolbar-actions">
                <button onClick={saveScript} className="btn-sm" disabled={!ready}>Save</button>
                <button onClick={runScript} className="btn-primary" disabled={!ready}>Run</button>
              </div>
            </div>
            <Editor
              height="100%"
              defaultLanguage="python"
              theme="vs-dark"
              value={code}
              onChange={(value) => setCode(value || '')}
              options={{
                minimap: { enabled: false },
                fontSize: 14,
                lineNumbers: 'on',
                roundedSelection: false,
                scrollBeyondLastLine: false,
                automaticLayout: true,
                tabSize: 4,
                wordWrap: 'on',
              }}
            />
          </div>

          <div className="console">
            <div className="console-header">Console</div>
            <pre className="console-output">{output}</pre>
          </div>
        </main>

        {/* Right: Screenshot */}
        <aside className="sidebar right">
          <div className="panel">
            <div className="panel-header">
              <h2>Device Screen</h2>
              <button onClick={takeScreenshot} className="btn-sm" disabled={!ready}>Capture</button>
            </div>
            <div className="screenshot-container">
              {screenshot ? (
                <>
                  <img
                    src={screenshot}
                    alt="Device"
                    className="screenshot"
                    onClick={handleScreenClick}
                    onMouseMove={handleScreenMouseMove}
                  />
                  <div style={{
                    marginTop: '10px',
                    padding: '8px',
                    background: '#2d2d30',
                    borderRadius: '4px',
                    textAlign: 'center',
                    fontSize: '13px',
                    fontFamily: 'monospace'
                  }}>
                    坐标: ({mouseCoords.x}, {mouseCoords.y})
                  </div>
                </>
              ) : (
                <div className="screenshot-placeholder">No screenshot</div>
              )}
            </div>
          </div>
        </aside>
      </div>
    </div>
  );
}

export default App;
