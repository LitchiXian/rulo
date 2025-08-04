package com.l2.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.l2.domain.Tag;
import com.l2.service.TagService;
import com.l2.mapper.TagMapper;
import org.springframework.stereotype.Service;

/**
* @author Administrator
* @description 针对表【b_tag(标签主表)】的数据库操作Service实现
* @createDate 2025-08-04 12:53:13
*/
@Service
public class TagServiceImpl extends ServiceImpl<TagMapper, Tag>
    implements TagService{

}




