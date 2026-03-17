import vue from '@vitejs/plugin-vue'
import createAutoImport from './auto-import'
import createComponents from './components'
import createCompression from './compression'
import createSetupExtend from './setup-extend'
import path from 'path'

export default (viteEnv: any, isBuild = false): any[] => {
  const plugins: any[] = []
  plugins.push(vue())
  plugins.push(createAutoImport(path))
  plugins.push(createComponents())
  plugins.push(createSetupExtend())
  isBuild && plugins.push(createCompression(viteEnv))
  return plugins
}
