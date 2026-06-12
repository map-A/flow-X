from PIL import Image, ImageDraw

# 创建 32x32 图标 (RGBA)
img = Image.new('RGBA', (32, 32), color='#4A90E2')
draw = ImageDraw.Draw(img)
draw.text((8, 8), 'FX', fill='white')
img.save('32x32.png')

# 创建 128x128 图标 (RGBA)
img = Image.new('RGBA', (128, 128), color='#4A90E2')
draw = ImageDraw.Draw(img)
draw.text((32, 32), 'FX', fill='white')
img.save('128x128.png')

# 创建 256x256 图标 (RGBA)
img = Image.new('RGBA', (256, 256), color='#4A90E2')
draw = ImageDraw.Draw(img)
draw.text((64, 64), 'FX', fill='white')
img.save('icon.png')

print("✅ 图标创建完成 (RGBA)")
