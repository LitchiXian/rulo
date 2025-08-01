package com.l2.web;

import cn.hutool.core.util.StrUtil;
import com.l2.common.domain.AjaxResult;
import com.l2.common.exception.ErrorCodeEnum;
import com.l2.framework.config.MinIOConfig;
import com.l2.framework.util.MinIOUtil;
import io.minio.GetObjectArgs;
import io.minio.GetObjectResponse;
import io.minio.MinioClient;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

@Slf4j
// 构造器注入注解
@RequiredArgsConstructor
@RestController
@RequestMapping("/minioTest")
public class MinioController {

    private final MinIOConfig minIOConfig;

    @GetMapping("/getObject")
    public void getObject(String objectName) throws Exception {

        MinIOUtil.getObject(minIOConfig.getBucketName(), objectName);

    }

    @PostMapping("uploadFace")
    public AjaxResult upload(@RequestParam MultipartFile file,
                             String userId) throws Exception {

        if (StrUtil.isBlank(userId)) {

            return AjaxResult.error();
        }

        String filename = file.getOriginalFilename();

        if (StrUtil.isBlank(filename)) {

            return AjaxResult.error(ErrorCodeEnum.FILE_UPLOAD_ERROR);
        }

        filename = "face" +  "/" + userId + "/" + filename;

        MinIOUtil.uploadFile(minIOConfig.getBucketName(), filename, file.getInputStream());

        String faceUrl = minIOConfig.getFileHost()
                + "/"
                + minIOConfig.getBucketName()
                + "/"
                + filename;

        return AjaxResult.success(faceUrl);
    }
}
