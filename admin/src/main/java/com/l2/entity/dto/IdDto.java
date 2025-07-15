package com.l2.entity.dto;

import lombok.Data;

import java.io.Serial;
import java.io.Serializable;

@Data
public class IdDto implements Serializable {

    @Serial
    private static final long serialVersionUID = 1L;

    private Long id;

}
