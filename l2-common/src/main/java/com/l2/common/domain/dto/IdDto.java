package com.l2.common.domain.dto;

import java.io.Serializable;

public class IdDto implements Serializable {
    private static final long serialVersionUID = 1L;

    private Long id;

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
    }

    @Override
    public String toString() {
        return "IdDto{" +
                "id=" + id +
                '}';
    }
}
