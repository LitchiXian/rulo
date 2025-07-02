package com.l2.web;

import io.minio.GetObjectArgs;
import io.minio.GetObjectResponse;
import io.minio.MinioClient;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@Slf4j
@RequiredArgsConstructor // 构造器注入注解
@RestController
@RequestMapping("/minio")
public class MinioController {

    private final MinioClient minioClient;

    @RequestMapping("/getObject")
    public void testGetObject() throws Exception {
        String path = "l2path/";

        GetObjectArgs getObjectArgs = GetObjectArgs.builder()
                .bucket("l2bucket")
                .object(path + "翻身叩背.jpg")
                .build();

        GetObjectResponse getObjectResponse = minioClient.getObject(getObjectArgs);
        getObjectResponse.transferTo(System.out);
    }

}
