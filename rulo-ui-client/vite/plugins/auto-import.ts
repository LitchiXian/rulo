import autoImport from 'unplugin-auto-import/vite'

export default (path: any) => {
    return autoImport({
        imports: [
            'vue',
            'vue-router',
            'pinia'
        ],
        // 关闭类型文件生成
        dts: false
        // dts: path.resolve(path.resolve(__dirname, '../../src'), 'types', 'auto-import.d.ts'),
    })
}
