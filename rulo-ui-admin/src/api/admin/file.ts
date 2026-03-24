import request from '@/util/request'

const fileApi = {
  /** 上传文件，返回文件访问 URL */
  upload(file: File): Promise<string> {
    const formData = new FormData()
    formData.append('file', file)
    return request({
      url: '/system/file/upload',
      method: 'post',
      data: formData,
      headers: { 'Content-Type': 'multipart/form-data' },
    }) as Promise<string>
  },
}

export default fileApi
