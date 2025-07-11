package com.l2.config;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.module.SimpleModule;
import com.fasterxml.jackson.databind.ser.std.ToStringSerializer;
import org.springframework.boot.autoconfigure.jackson.Jackson2ObjectMapperBuilderCustomizer;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.converter.json.Jackson2ObjectMapperBuilder;

@Configuration
public class JacksonConfig {

    @Bean
    public ObjectMapper objectMapper() {
        ObjectMapper objectMapper = new ObjectMapper();
        SimpleModule module = new SimpleModule();

        // 注册Long和long类型的序列化器
        module.addSerializer(Long.class, new ToStringSerializer());
        module.addSerializer(long.class, new ToStringSerializer());

        objectMapper.registerModule(module);

        // 忽略未知字段（不报错）
        objectMapper.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
        return objectMapper;
    }
}

//@Configuration
//public class JacksonConfig implements Jackson2ObjectMapperBuilderCustomizer {
//
//    @Override
//    public void customize(Jackson2ObjectMapperBuilder builder) {
//        // 使用单例实例，避免每次创建新对象
//        builder.serializerByType(Long.class, ToStringSerializer.instance);
//        builder.serializerByType(long.class, ToStringSerializer.instance);
//    }
//}
