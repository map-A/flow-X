package com.flowx.automation

import android.accessibilityservice.AccessibilityService
import android.accessibilityservice.GestureDescription
import android.app.ActivityManager
import android.content.Context
import android.graphics.Bitmap
import android.graphics.Path
import android.graphics.Rect
import android.util.Base64
import android.view.accessibility.AccessibilityEvent
import android.view.accessibility.AccessibilityNodeInfo
import java.io.ByteArrayOutputStream
import java.net.InetSocketAddress
import org.java_websocket.WebSocket
import org.java_websocket.server.WebSocketServer
import com.google.gson.Gson

class FlowXService : AccessibilityService() {

    private var server: FlowXWebSocketServer? = null
    private val gson = Gson()

    override fun onServiceConnected() {
        super.onServiceConnected()
        // 启动 WebSocket 服务器
        server = FlowXWebSocketServer(6789, this)
        server?.start()
        println("FlowX WebSocket server started on port 6789")
    }

    override fun onAccessibilityEvent(event: AccessibilityEvent?) {
        // 监听 UI 事件
    }

    override fun onInterrupt() {}

    override fun onDestroy() {
        server?.stop()
        super.onDestroy()
    }

    // 获取屏幕尺寸
    fun getScreenSize(): ScreenInfo {
        val displayMetrics = resources.displayMetrics
        return ScreenInfo(displayMetrics.widthPixels, displayMetrics.heightPixels)
    }

    // 点击操作
    fun performClick(x: Int, y: Int): Boolean {
        val path = Path().apply {
            moveTo(x.toFloat(), y.toFloat())
        }
        val gesture = GestureDescription.Builder()
            .addStroke(GestureDescription.StrokeDescription(path, 0, 100))
            .build()
        return dispatchGesture(gesture, null, null)
    }

    // 滑动操作
    fun performSwipe(x1: Int, y1: Int, x2: Int, y2: Int, duration: Long): Boolean {
        val path = Path().apply {
            moveTo(x1.toFloat(), y1.toFloat())
            lineTo(x2.toFloat(), y2.toFloat())
        }
        val gesture = GestureDescription.Builder()
            .addStroke(GestureDescription.StrokeDescription(path, 0, duration))
            .build()
        return dispatchGesture(gesture, null, null)
    }

    // 输入文本
    fun inputText(text: String): Boolean {
        val focusedNode = findFocusedNode(rootInActiveWindow)
        if (focusedNode != null) {
            val arguments = android.os.Bundle()
            arguments.putCharSequence(
                AccessibilityNodeInfo.ACTION_ARGUMENT_SET_TEXT_CHARSEQUENCE,
                text
            )
            return focusedNode.performAction(
                AccessibilityNodeInfo.ACTION_SET_TEXT,
                arguments
            )
        }
        return false
    }

    private fun findFocusedNode(node: AccessibilityNodeInfo?): AccessibilityNodeInfo? {
        node ?: return null
        if (node.isFocused) return node
        for (i in 0 until node.childCount) {
            val child = node.getChild(i) ?: continue
            val found = findFocusedNode(child)
            if (found != null) return found
        }
        return null
    }

    // 按键操作
    fun pressKey(key: String): Boolean {
        return when (key.uppercase()) {
            "BACK" -> performGlobalAction(GLOBAL_ACTION_BACK)
            "HOME" -> performGlobalAction(GLOBAL_ACTION_HOME)
            "RECENT_APPS", "RECENTS" -> performGlobalAction(GLOBAL_ACTION_RECENTS)
            "NOTIFICATIONS" -> performGlobalAction(GLOBAL_ACTION_NOTIFICATIONS)
            "QUICK_SETTINGS" -> performGlobalAction(GLOBAL_ACTION_QUICK_SETTINGS)
            else -> false
        }
    }

    // 查找元素
    fun findElement(text: String): AccessibilityNodeInfo? {
        val root = rootInActiveWindow ?: return null
        return findNodeByText(root, text)
    }

    private fun findNodeByText(node: AccessibilityNodeInfo, text: String): AccessibilityNodeInfo? {
        if (node.text?.toString()?.contains(text) == true) {
            return node
        }
        if (node.contentDescription?.toString()?.contains(text) == true) {
            return node
        }
        for (i in 0 until node.childCount) {
            val child = node.getChild(i) ?: continue
            val found = findNodeByText(child, text)
            if (found != null) return found
        }
        return null
    }

    // 获取当前包名
    fun getCurrentPackage(): String? {
        return rootInActiveWindow?.packageName?.toString()
    }

    // 获取当前Activity (需要 USAGE_STATS 权限，这里返回简化版本)
    fun getCurrentActivity(): String? {
        val activityManager = getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager
        val runningTasks = activityManager.getRunningTasks(1)
        return if (runningTasks.isNotEmpty()) {
            runningTasks[0].topActivity?.className
        } else {
            null
        }
    }

    // 截图功能 (使用 AccessibilityService 的 takeScreenshot API - Android 11+)
    fun takeScreenshot(): String? {
        try {
            if (android.os.Build.VERSION.SDK_INT >= android.os.Build.VERSION_CODES.R) {
                // Android 11+ 支持 AccessibilityService.takeScreenshot()
                val callback = object : AccessibilityService.TakeScreenshotCallback {
                    var result: String? = null
                    val latch = java.util.concurrent.CountDownLatch(1)

                    override fun onSuccess(screenshot: AccessibilityService.ScreenshotResult) {
                        try {
                            val bitmap = Bitmap.wrapHardwareBuffer(
                                screenshot.hardwareBuffer,
                                screenshot.colorSpace
                            )
                            if (bitmap != null) {
                                val outputStream = ByteArrayOutputStream()
                                bitmap.compress(Bitmap.CompressFormat.PNG, 100, outputStream)
                                result = Base64.encodeToString(outputStream.toByteArray(), Base64.NO_WRAP)
                                bitmap.recycle()
                            }
                            screenshot.hardwareBuffer.close()
                        } catch (e: Exception) {
                            println("FlowX: Screenshot bitmap conversion failed: ${e.message}")
                        }
                        latch.countDown()
                    }

                    override fun onFailure(errorCode: Int) {
                        println("FlowX: Screenshot failed with error code: $errorCode")
                        latch.countDown()
                    }
                }

                takeScreenshot(
                    android.view.Display.DEFAULT_DISPLAY,
                    { runnable -> runnable.run() },
                    callback
                )

                // 等待截图完成 (最多 5 秒)
                callback.latch.await(5, java.util.concurrent.TimeUnit.SECONDS)
                return callback.result
            } else {
                println("FlowX: Screenshot requires Android 11+ (API 30+)")
                return null
            }
        } catch (e: Exception) {
            println("FlowX: Screenshot failed: ${e.message}")
            e.printStackTrace()
        }
        return null
    }
}

// WebSocket 服务器
class FlowXWebSocketServer(port: Int, private val service: FlowXService) :
    WebSocketServer(InetSocketAddress(port)) {

    private val gson = Gson()

    override fun onOpen(conn: WebSocket?, handshake: org.java_websocket.handshake.ClientHandshake?) {
        println("FlowX: Client connected from ${conn?.remoteSocketAddress}")
    }

    override fun onMessage(conn: WebSocket?, message: String?) {
        message ?: return

        try {
            val command = gson.fromJson(message, Command::class.java)
            println("FlowX: Received command: ${command.type}")

            val result = when (command.type) {
                "click" -> {
                    val success = service.performClick(command.x ?: 0, command.y ?: 0)
                    Response(if (success) "success" else "failed")
                }
                "swipe" -> {
                    val success = service.performSwipe(
                        command.x1 ?: 0, command.y1 ?: 0,
                        command.x2 ?: 0, command.y2 ?: 0,
                        command.duration ?: 300
                    )
                    Response(if (success) "success" else "failed")
                }
                "find" -> {
                    val element = service.findElement(command.text ?: "")
                    if (element != null) {
                        val bounds = Rect()
                        element.getBoundsInScreen(bounds)
                        Response("success", ElementInfo(
                            bounds.centerX(), bounds.centerY(),
                            element.text?.toString()
                        ))
                    } else {
                        Response("not_found")
                    }
                }
                "input_text" -> {
                    val success = service.inputText(command.text ?: "")
                    Response(if (success) "success" else "failed")
                }
                "press_key" -> {
                    val success = service.pressKey(command.key ?: "")
                    Response(if (success) "success" else "failed")
                }
                "screen_size" -> {
                    val screenInfo = service.getScreenSize()
                    Response("success", screenInfo)
                }
                "current_package" -> {
                    val packageName = service.getCurrentPackage()
                    if (packageName != null) {
                        Response("success", packageName)
                    } else {
                        Response("failed")
                    }
                }
                "current_activity" -> {
                    val activity = service.getCurrentActivity()
                    if (activity != null) {
                        Response("success", activity)
                    } else {
                        Response("failed")
                    }
                }
                "screenshot" -> {
                    val screenshot = service.takeScreenshot()
                    if (screenshot != null) {
                        Response("success", screenshot)
                    } else {
                        Response("not_implemented", "Screenshot requires MediaProjection setup")
                    }
                }
                else -> Response("unknown_command", "Command type '${command.type}' not recognized")
            }

            conn?.send(gson.toJson(result))
            println("FlowX: Sent response: ${result.status}")
        } catch (e: Exception) {
            println("FlowX: Error handling command: ${e.message}")
            e.printStackTrace()
            val errorResponse = Response("error", e.message)
            conn?.send(gson.toJson(errorResponse))
        }
    }

    override fun onClose(conn: WebSocket?, code: Int, reason: String?, remote: Boolean) {
        println("FlowX: Client disconnected: $reason")
    }

    override fun onError(conn: WebSocket?, ex: Exception?) {
        println("FlowX: WebSocket error: ${ex?.message}")
        ex?.printStackTrace()
    }

    override fun onStart() {
        println("FlowX: WebSocket server started successfully")
    }
}

data class Command(
    val type: String,
    val x: Int? = null,
    val y: Int? = null,
    val x1: Int? = null,
    val y1: Int? = null,
    val x2: Int? = null,
    val y2: Int? = null,
    val duration: Long? = null,
    val text: String? = null,
    val key: String? = null
)

data class Response(
    val status: String,
    val data: Any? = null
)

data class ElementInfo(
    val x: Int,
    val y: Int,
    val text: String?
)

data class ScreenInfo(
    val width: Int,
    val height: Int
)
