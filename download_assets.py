#!/usr/bin/env python3
"""
Download textures and sounds for Terraria 3D
Generates procedural textures
"""

import os
import urllib.request
from pathlib import Path

def ensure_directories():
    """Ensure asset directories exist"""
    Path("assets/textures/blocks").mkdir(parents=True, exist_ok=True)
    Path("assets/sounds").mkdir(parents=True, exist_ok=True)

def download_texture(url, filename):
    """Download a texture file"""
    filepath = Path(f"assets/textures/blocks/{filename}")
    try:
        print(f"Downloading {filename}...")
        urllib.request.urlretrieve(url, filepath)
        print(f"✅ Downloaded {filename}")
        return True
    except Exception as e:
        print(f"⚠️  Failed to download {filename}: {e}")
    return False

def generate_procedural_textures():
    """Generate simple procedural textures using Python"""
    try:
        from PIL import Image, ImageDraw
        
        textures = {
            "dirt.png": (lambda: create_gradient_texture((117, 79, 43), (89, 60, 32), 16)),
            "grass.png": (lambda: create_grass_texture()),
            "stone.png": (lambda: create_noise_texture((128, 128, 128), 16)),
            "wood.png": (lambda: create_wood_texture()),
            "leaves.png": (lambda: create_leaves_texture()),
            "sand.png": (lambda: create_noise_texture((217, 199, 148), 16)),
            "water.png": (lambda: create_water_texture()),
            "bedrock.png": (lambda: create_noise_texture((38, 38, 38), 16)),
        }
        
        for filename, generator in textures.items():
            img = generator()
            img.save(f"assets/textures/blocks/{filename}")
            print(f"✅ Generated {filename}")
            
    except ImportError:
        print("⚠️  PIL not available, creating placeholder textures")
        create_placeholder_textures()

def create_gradient_texture(color1, color2, size):
    """Create a gradient texture"""
    img = Image.new('RGB', (size, size))
    for y in range(size):
        for x in range(size):
            ratio = y / size
            r = int(color1[0] * (1 - ratio) + color2[0] * ratio)
            g = int(color1[1] * (1 - ratio) + color2[1] * ratio)
            b = int(color1[2] * (1 - ratio) + color2[2] * ratio)
            img.putpixel((x, y), (r, g, b))
    return img

def create_grass_texture():
    """Create grass texture"""
    img = Image.new('RGB', (16, 16))
    for y in range(16):
        for x in range(16):
            if y < 4:
                r, g, b = 56, 130, 38
            else:
                ratio = (y - 4) / 12
                r = int(56 * (1 - ratio) + 117 * ratio)
                g = int(130 * (1 - ratio) + 79 * ratio)
                b = int(38 * (1 - ratio) + 43 * ratio)
            img.putpixel((x, y), (r, g, b))
    return img

def create_noise_texture(base_color, size):
    """Create noise texture"""
    import random
    random.seed(42)
    img = Image.new('RGB', (size, size))
    for y in range(size):
        for x in range(size):
            noise = random.randint(-20, 20)
            r = max(0, min(255, base_color[0] + noise))
            g = max(0, min(255, base_color[1] + noise))
            b = max(0, min(255, base_color[2] + noise))
            img.putpixel((x, y), (r, g, b))
    return img

def create_wood_texture():
    """Create wood texture with grain"""
    img = Image.new('RGB', (16, 16))
    for y in range(16):
        for x in range(16):
            grain = (x % 4 == 0) * 30
            r = max(0, min(255, 107 - grain))
            g = max(0, min(255, 79 - grain))
            b = max(0, min(255, 43 - grain))
            img.putpixel((x, y), (r, g, b))
    return img

def create_leaves_texture():
    """Create leaves texture"""
    img = Image.new('RGB', (16, 16))
    for y in range(16):
        for x in range(16):
            variation = ((x + y) % 3) * 15
            r = 25 + variation
            g = 100 + variation
            b = 25 + variation
            img.putpixel((x, y), (r, g, b))
    return img

def create_water_texture():
    """Create water texture"""
    img = Image.new('RGBA', (16, 16))
    for y in range(16):
        for x in range(16):
            r = 0
            g = 77
            b = 179
            a = 150  # Semi-transparent
            img.putpixel((x, y), (r, g, b, a))
    return img

def create_placeholder_textures():
    """Create placeholder colored textures"""
    colors = {
        "dirt.png": (117, 79, 43),
        "grass.png": (56, 130, 38),
        "stone.png": (128, 128, 128),
        "wood.png": (107, 79, 43),
        "leaves.png": (25, 100, 25),
        "sand.png": (217, 199, 148),
        "water.png": (0, 77, 179),
        "bedrock.png": (38, 38, 38),
    }
    
    for filename, color in colors.items():
        img = Image.new('RGB', (16, 16), color=color)
        img.save(f"assets/textures/blocks/{filename}")

def main():
    print("📦 Downloading assets for Terraria 3D...")
    
    ensure_directories()
    
    # Try to download from free sources
    texture_urls = {
        "dirt.png": "https://opengameart.org/sites/default/files/dirt_0.png",
        "grass.png": "https://opengameart.org/sites/default/files/grass_block.png",
        "stone.png": "https://opengameart.org/sites/default/files/stone.png",
    }
    
    downloaded = False
    for filename, url in texture_urls.items():
        if download_texture(url, filename):
            downloaded = True
    
    # Generate textures if downloads failed
    if not downloaded:
        print("🎨 Generating procedural textures...")
        generate_procedural_textures()
    
    print("\n✅ Assets ready!")

if __name__ == "__main__":
    main()
