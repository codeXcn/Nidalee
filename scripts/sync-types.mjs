import fs from 'fs/promises'
import path from 'path'
import { fileURLToPath } from 'url'

// --- 配置 ---
const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const projectRoot = path.resolve(__dirname, '..')
const sourceDir = path.resolve(projectRoot, 'src/types/generated')
const targetFile = path.resolve(projectRoot, 'src/types/global.d.ts')
const header = `/**
 * 全局类型声明 - 由 scripts/sync-types.mjs 自动生成
 *
 * ************************************************
 * *                                               *
 * *           !!!   请勿手动修改此文件   !!!            *
 * *                                               *
 * *************************************************
 *
 * 如需更新，请运行 'odescripts/sync-types.mjs'
 */

`
// --- 主逻辑 --
async function syncTypes() {
  try {
    console.log('🚀 开始同步型定义...')
    console.log(`- 源目录: ${sourceDir}`)
    console.log(`- 目标文件: ${targetFile}`)

    const files = await fs.readdir(sourceDir)
    const tsFiles = files.filter((file) => file.endsWith('.ts') && !file.endsWith('.d.ts') && file !== 'index.ts')
    if (tsFiles.length === 0) {
      console.warn('⚠️ 在源目录中未找到需要同步的 .ts 文件。')
      return
    }

    let combinedContent = header

    for (const file of tsFiles) {
      const filePath = path.join(sourceDir, file)
      const content = await fs.readFile(filePath, 'utf-8')

      // 移除所有 import 行 (用户建议的、更可靠的方法)
      const lines = content.split(/\r?\n/)
      const contentWithoutImports = lines.filter((line) => !line.trim().startsWith('import')).join('\n')

      // 移除 'export' 关键字
      const contentWithoutExports = contentWithoutImports.replace(/export /g, '')

      combinedContent += `// --- 从 ${file} 同步 ---\n`
      combinedContent += contentWithoutExports
      combinedContent += '\n\n'
    }

    await fs.writeFile(targetFile, combinedContent)

    console.log(`✅ 类型同步成功！ ${tsFiles.length} 个文件的内容已合并到 global.d.ts。`)
  } catch (error) {
    console.error('❌ 类型同步失败:', error)
    process.exit(1)
  }
}

syncTypes()
