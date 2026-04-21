import compression from 'vite-plugin-compression'

export default (_viteEnv: any) => {
  return compression({
    verbose: true,
    disable: false,
    threshold: 10240,
    algorithm: 'gzip',
    ext: '.gz',
  })
}
