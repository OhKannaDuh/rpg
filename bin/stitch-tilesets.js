#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const { Jimp } = require("jimp");


function ensureDirSync(dir) {
  fs.mkdirSync(dir, { recursive: true });
}

function readJSON(filePath) {
  const raw = fs.readFileSync(filePath, "utf8");
  return JSON.parse(raw);
}

async function main() {
  const cfgPath = process.argv[2];
  if (!cfgPath) {
    console.error("Usage: node stitch-tiles.js ./stitch_config.json");
    process.exit(1);
  }

  let config;
  try {
    config = readJSON(cfgPath);
  } catch (err) {
    console.error(`Failed to read config: ${cfgPath}\n${err.message}`);
    process.exit(1);
  }

  const {
    input_directory,
    output_directory,
    output_file,
    tile_size,
    images
  } = config;

  if (!Array.isArray(images) || images.length === 0) {
    console.error("Config must contain a non-empty 'images' array.");
    process.exit(1);
  }

  const inputDir = path.isAbsolute(input_directory)
    ? input_directory
    : path.resolve(path.dirname(cfgPath), input_directory);
  const outDir = path.isAbsolute(output_directory)
    ? output_directory
    : path.resolve(path.dirname(cfgPath), output_directory);

  let maxX = 0, maxY = 0;
  for (const img of images) {
    const x2 = img.x + img.width;
    const y2 = img.y + img.height;
    if (x2 > maxX) maxX = x2;
    if (y2 > maxY) maxY = y2;
  }

  const totalWidth = maxX * tile_size;
  const totalHeight = maxY * tile_size;

  console.log(`→ Stitching atlas: ${images.length} images`);
  console.log(`   Output size: ${maxX}×${maxY} tiles (${totalWidth}×${totalHeight} px)`);

  const outImage = new Jimp({width: totalWidth, height: totalHeight, color: 0x00000000});

  for (const img of images) {
    const inPath = path.resolve(inputDir, img.file);

    let atlas;
    try {
      atlas = await Jimp.read(inPath);
    } catch (err) {
      console.error(`✖ Failed to load image: ${inPath}\n${err.message}`);
      process.exit(1);
    }

    const expectedW = img.width * tile_size;
    const expectedH = img.height * tile_size;

    if (atlas.bitmap.width !== expectedW || atlas.bitmap.height !== expectedH) {
      console.warn(
        `Warning: image ${img.file} is ${atlas.bitmap.width}×${atlas.bitmap.height}, expected ${expectedW}×${expectedH}`
      );
    }

    const pxX = img.x * tile_size;
    const pxY = img.y * tile_size;

    outImage.composite(atlas, pxX, pxY);
    console.log(`   + Placed ${img.file} at ${img.x},${img.y} (${pxX},${pxY} px)`);
  }

  ensureDirSync(outDir);
  const outPath = path.join(outDir, output_file);
  try {
    await outImage.write(outPath);
  } catch (err) {
    console.error(`Failed to write output: ${outPath}\n${err.message}`);
    process.exit(1);
  }

  console.log(`Wrote ${outPath}`);
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
