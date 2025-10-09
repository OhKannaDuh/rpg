import { promises as fs } from "fs";
import { Jimp } from "jimp";
import path from "path";

function hexToRgbInt(hex) {
  if (hex.startsWith("#")) {
    hex = hex.substring(1);
  }

  return parseInt(hex, 16);
}

function createColorMap(basePalette, biomePalette) {
  const colorMap = new Map();

  for (const key in biomePalette) {
    if (basePalette[key] && biomePalette[key]) {
      const originalRgbInt = hexToRgbInt(basePalette[key]);
      const newRgbInt = hexToRgbInt(biomePalette[key]);
      colorMap.set(originalRgbInt, newRgbInt);
    }
  }
  return colorMap;
}

function recolorImage(image, colorMap) {
  image.scan(
    0,
    0,
    image.bitmap.width,
    image.bitmap.height,
    function (x, y, idx) {
      const r = this.bitmap.data[idx + 0];
      const g = this.bitmap.data[idx + 1];
      const b = this.bitmap.data[idx + 2];
      const a = this.bitmap.data[idx + 3];

      const originalRgbInt = (r << 16) | (g << 8) | b;

      if (originalRgbInt === 0) {
        return;
      }

      const newRgbInt = colorMap.get(originalRgbInt);
      if (newRgbInt === undefined) {
        console.log(
          `[WARN] No mapping for color #${originalRgbInt
            .toString(16)
            .padStart(6, "0")} at pixel (${x}, ${y})`
        );

        return;
      }

      if (newRgbInt !== undefined) {
        const newR = (newRgbInt >> 16) & 0xff;
        const newG = (newRgbInt >> 8) & 0xff;
        const newB = newRgbInt & 0xff;

        this.bitmap.data[idx + 0] = newR;
        this.bitmap.data[idx + 1] = newG;
        this.bitmap.data[idx + 2] = newB;
      }
    }
  );
  return image;
}

async function generateTiles(config) {
  const { tile_size, input, palette, tiles, components, biomes, generation } =
    config;

  console.log(`[INFO] Loading template image from: ${input}`);
  const templateImage = await Jimp.read(input);
  console.log(
    `[INFO] Template image loaded. Dimensions: ${templateImage.bitmap.width}x${templateImage.bitmap.height}`
  );

  for (const task of generation) {
    const { preset: presetName, output_file, biome: biomeName } = task;
    const preset = config.presets[presetName];
    const biome = biomes[biomeName];

    if (!preset) {
      console.error(`[ERROR] Preset '${presetName}' not found.`);
      continue;
    }
    if (!biome) {
      console.error(`[ERROR] Biome '${biomeName}' not found.`);
      continue;
    }

    console.log(
      `\n[TASK] Generating file: ${output_file} (Preset: ${presetName}, Biome: ${biomeName})`
    );

    const colorMap = createColorMap(palette, biome);

    let maxW = 0;
    let maxH = 0;
    for (const placement of preset) {
      const placementComponents = placement.components ?? [placement.component];

      for (const component of placementComponents) {
        const componentDef = components[component];
        const endX = placement.x + componentDef.w;
        const endY = placement.y + componentDef.h;

        if (endX > maxW) maxW = endX;
        if (endY > maxH) maxH = endY;
      }
    }

    const outputWidth = maxW * tile_size;
    const outputHeight = maxH * tile_size;

    console.log(
      `[INFO] Output dimensions: ${maxW}x${maxH} tiles -> ${outputWidth}x${outputHeight} pixels.`
    );

    const outputImage = await new Jimp({
      width: outputWidth,
      height: outputHeight,
      color: 0x00000000,
    });

    for (const placement of preset) {
      const placementComponents = placement.components ?? [placement.component];

      for (const component of placementComponents) {
        const componentDef = components[component];
        const targetX = placement.x * tile_size;
        const targetY = placement.y * tile_size;

        console.log(
          `[DEBUG] Processing component '${component}' at tile coord (${placement.x}, ${placement.y}), pixel coord (${targetX}, ${targetY}).`
        );

        if (placement.base) {
          const tileDef = tiles[placement.base];
          if (!tileDef) {
            console.error(
              `[WARN] Base tile '${placement.base}' for placement at (${placement.x}, ${placement.y}) not found. Skipping base.`
            );
          } else {
            const tileSourceX = tileDef.x * tile_size;
            const tileSourceY = tileDef.y * tile_size;

            let baseTile = await new Jimp({
              width: tile_size,
              height: tile_size,
              color: 0x00000000,
            });

            baseTile.blit({
              src: templateImage,
              x: 0,
              y: 0,
              srcX: tileSourceX,
              srcY: tileSourceY,
              srcW: tile_size,
              srcH: tile_size,
            });

            recolorImage(baseTile, colorMap);

            for (let x = 0; x < componentDef.w; x++) {
              for (let y = 0; y < componentDef.h; y++) {
                const blitX = targetX + x * tile_size;
                const blitY = targetY + y * tile_size;
                outputImage.blit({ src: baseTile, x: blitX, y: blitY });
              }
            }
            console.log(
              `[INFO] Placed base tile '${placement.base}' underneath component.`
            );
          }
        }

        const componentSourceX = componentDef.x * tile_size;
        const componentSourceY = componentDef.y * tile_size;
        const componentPixelW = componentDef.w * tile_size;
        const componentPixelH = componentDef.h * tile_size;

        let componentImg = await new Jimp({
          width: componentPixelW,
          height: componentPixelH,
          color: 0x00000000,
        });

        componentImg.blit({
          src: templateImage,
          x: 0,
          y: 0,
          srcX: componentSourceX,
          srcY: componentSourceY,
          srcW: componentPixelW,
          srcH: componentPixelH,
        });

        recolorImage(componentImg, colorMap);

        outputImage.blit({ src: componentImg, x: targetX, y: targetY });

        console.log(
          `[INFO] Placed component '${component}' at (${placement.x}, ${placement.y}) (${targetX}, ${targetY}).`
        );
      }
    }

    await outputImage.write(output_file);
    console.log(`[SUCCESS] Generated file saved to: ${output_file}`);
  }
}

async function main() {
  const configPath = process.argv[2];

  if (!configPath) {
    console.error(
      "ERROR: Please provide the path to the configuration file as a command line argument."
    );
    console.error("Usage: node tile_generator.js /path/to/your/config.json");
    return;
  }

  try {
    const configRaw = await fs.readFile(configPath, "utf-8");
    const config = JSON.parse(configRaw);

    if (!path.isAbsolute(config.input)) {
      config.input = path.resolve(process.cwd(), config.input);
    }

    for (const task of config.generation) {
      if (!path.isAbsolute(task.output_file)) {
        task.output_file = path.resolve(process.cwd(), task.output_file);
      }
    }

    await generateTiles(config);
  } catch (error) {
    if (error.code === "ENOENT") {
      console.error(`\n[FATAL ERROR] File not found: ${error.path}`);
      console.error(
        "Please ensure the config file path and the input image path are correct."
      );
    } else if (error instanceof SyntaxError) {
      console.error(
        `\n[FATAL ERROR] Failed to parse config JSON: ${error.message}`
      );
    } else {
      console.error(`\n[FATAL ERROR] An unexpected error occurred:`, error);
    }
  }
}

main();
