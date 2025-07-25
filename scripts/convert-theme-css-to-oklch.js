import fs from 'fs/promises'
import path from 'path'
import { fileURLToPath } from 'url'
import { converter, hsl } from 'culori'

const toOklch = converter('oklch')

// 获取当前文件夹路径（ESM 下用）
const __dirname = path.dirname(fileURLToPath(import.meta.url))

const inputPath = path.resolve(__dirname, 'theme.css')
const outputPath = path.resolve(__dirname, 'output-oklch.css')

const originalCss = await fs.readFile(inputPath, 'utf-8')

const hslVarRegex = /(--[\w-]+):\s*([\d.]+)\s+([\d.]+)%\s+([\d.]+)%\s*;/g

const transformedCss = originalCss.replace(hslVarRegex, (match, varName, h, s, l) => {
  const color = hsl({
    h: parseFloat(h),
    s: parseFloat(s) / 100,
    l: parseFloat(l) / 100
  })

  const oklchColor = toOklch(color)

  if (!oklchColor) {
    console.warn(`❌ Failed to convert ${varName}`)
    return match
  }

  const { l: L, c: C, h: H } = oklchColor

  const hValue = H === undefined || isNaN(H) ? 0 : H

  const formatted = `oklch(${L.toFixed(3)} ${C.toFixed(3)} ${hValue.toFixed(1)})`
  return `${varName}: ${formatted};`
})

await fs.writeFile(outputPath, transformedCss, 'utf-8')

console.log(`✅ 转换完成：已写入 ${outputPath}`)
