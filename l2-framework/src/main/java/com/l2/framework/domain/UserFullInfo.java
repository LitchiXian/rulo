package com.l2.framework.domain;

import java.io.Serializable;

/**
 * @Author: LitchiXian
 * @Date: 2025/8/2 15:43
 * @Desc: UserFullInfo
 */
public class UserFullInfo implements Serializable {
    private static final long serialVersionUID = 1L;

    private SysUser sysUser;

    // 角色集合,权限集合,菜单集合


    public SysUser getSysUser() {
        return sysUser;
    }

    public void setSysUser(SysUser sysUser) {
        this.sysUser = sysUser;
    }

    @Override
    public String toString() {
        return "UserFullInfo{" +
                "sysUser=" + sysUser +
                '}';
    }
}
