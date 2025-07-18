import autoImport from 'unplugin-auto-import/vite'

export default (path: any) => {
    return autoImport({
        imports: [
            'vue',
            'vue-router',
            'pinia'
        ],
        dts: path.resolve(path.resolve(__dirname, '../../src'), 'types', 'auto-import.d.ts'),
    })
}
