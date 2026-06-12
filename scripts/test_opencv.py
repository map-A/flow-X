#!/usr/bin/env python3
"""OpenCV 功能验证"""
import numpy as np
import cv2

print("OpenCV 功能验证")
print("=" * 60)

# 测试 1: 模板匹配
print("\n✓ 测试 1: 模板匹配")
# 创建灰度图像更简单
template = np.zeros((100, 100), dtype=np.uint8)
template[:, :] = 255

haystack = np.zeros((500, 500), dtype=np.uint8)
haystack[200:300, 200:300] = 255

result = cv2.matchTemplate(haystack, template, cv2.TM_CCOEFF_NORMED)
_, max_val, _, max_loc = cv2.minMaxLoc(result)
print(f"  匹配度: {max_val:.4f}, 位置: {max_loc}")
assert max_val > 0.9, f"模板匹配失败: {max_val}"
print("  ✅ 通过")

# 测试 2: 颜色查找（使用彩色图）
print("\n✓ 测试 2: 颜色查找")
img = np.zeros((500, 500, 3), dtype=np.uint8)
img[200:300, 200:300] = [0, 0, 255]  # BGR红色

mask = cv2.inRange(img, np.array([0,0,250]), np.array([10,10,255]))
count = cv2.countNonZero(mask)
print(f"  找到红色像素: {count}")
assert count == 10000, f"颜色查找失败: {count}"
print("  ✅ 通过")

# 测试 3: 图像操作
print("\n✓ 测试 3: 图像操作")
gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
assert gray.shape == (500, 500), "灰度转换失败"
print("  ✅ 通过")

print("\n" + "=" * 60)
print("✅ 所有 OpenCV 功能验证通过")
