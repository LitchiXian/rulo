package com.l2.web;

import com.l2.stats.ApiStats;
import org.springframework.web.bind.annotation.*;

import java.util.concurrent.TimeUnit;

@RestController
@RequestMapping("/orders")
public class OrderController {

    @GetMapping("/{id}")
    public String getOrder(@PathVariable("id") String id) {
//        // 注册API路径
//        ApiStats.registerApi(
//                "OrderController.getOrder(String)",
//                "GET /orders/{id}"
//        );

        try {
            TimeUnit.SECONDS.sleep(2);
        } catch (InterruptedException e) {
            throw new RuntimeException(e);
        }
        // ...业务逻辑
        return "Hello, Order " + id;
    }

    @PostMapping
    public String createOrder(@RequestBody String order) {
        ApiStats.registerApi(
                "OrderController.createOrder(Order)",
                "POST /orders"
        );

        // ...业务逻辑
        return "Order created: " + order;
    }
}
