package com.l2;

import org.mybatis.spring.annotation.MapperScan;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
@MapperScan("com.l2.mapper")
public class L2Application {

    public static void main(String[] args) {
        SpringApplication.run(L2Application.class, args);
    }

}
