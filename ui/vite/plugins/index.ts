import vue from '@vitejs/plugin-vue';
import createAutoImport from './auto-import.js'
import createSvgIcon from './svg-icon'
import createCompression from './compression.js'
import createSetupExtend from './setup-extend'
import path from 'path';

export default (viteEnv: any, isBuild = false): [] => {
    const vitePlugins: any = [];
    vitePlugins.push(vue());
    vitePlugins.push(createAutoImport(path));
    vitePlugins.push(createSetupExtend());
    vitePlugins.push(createSvgIcon(path));
    isBuild && vitePlugins.push(createCompression(viteEnv));
    return vitePlugins;
}
