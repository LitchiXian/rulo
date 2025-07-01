package com.l2.web;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/hello")
public class HelloController {

    @GetMapping("/test1")
    public String test1() {
        return "Hello world";
    }

    @GetMapping("/test2")
    public String test2() {
        return "Hello world2";
    }


}
