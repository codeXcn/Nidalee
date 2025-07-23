import fs from 'fs/promises'
import path from 'path'
import culori from 'culori'

const { convert, hsl } = culori
const toOklch = convert('oklch')

// ⬇️ 改成兼容 ESM 的方式
const __dirname = new URL('.', import.meta.url).pathname
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

  const formatted = `oklch(${L.toFixed(3)} ${C.toFixed(3)} ${H.toFixed(1)})`
  return `${varName}: ${formatted};`
})

await fs.writeFile(outputPath, transformedCss, 'utf-8')

console.log(`✅ 转换完成：已写入 ${outputPath}`)
