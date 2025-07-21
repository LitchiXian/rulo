package com.l2.util;

import com.l2.config.EmailConfig;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;

import javax.mail.Message;
import javax.mail.Session;
import javax.mail.Transport;
import javax.mail.internet.InternetAddress;
import javax.mail.internet.MimeMessage;
import java.util.Date;
import java.util.Properties;
import java.util.Set;

/**
 * @Author: Linzx
 * @Date: 2025/7/21 19:41
 * @Desc: MailUtil
 */
@Component
@RequiredArgsConstructor
@Slf4j
public class MailUtil {
    private final EmailConfig emailConfig;

    /**
     * 发送邮件
     *
     * @param emails  目标邮件地址
     * @param title   邮件标题
     * @param content 邮件内容
     */
    public  boolean sendEmail(Set<String> emails, String title, String content) {
        // 未传收件人邮箱地址则直接返回
        if (emails == null || emails.isEmpty()) return false;
        try {
            // 1. 创建参数配置, 用于连接邮件服务器的参数配置
            Properties props = new Properties();
            props.setProperty("mail.transport.protocol", emailConfig.getProtocol()); // 使用的协议（JavaMail规范要求）
            props.setProperty("mail.smtp.host", emailConfig.getHost()); // 指定smtp服务器地址
            props.setProperty("mail.smtp.port", emailConfig.getPort()); // 指定smtp端口号
            // 使用smtp身份验证
            props.setProperty("mail.smtp.auth", "true"); // 需要请求认证
            props.put("mail.smtp.ssl.enable", "true"); // 开启SSL
            props.put("mail.smtp.ssl.protocols", "TLSv1.2"); // 指定SSL版本
            props.put("mail.smtp.socketFactory.class", "javax.net.ssl.SSLSocketFactory");
            // 由于Properties默认不限制请求时间，可能会导致线程阻塞，所以指定请求时长
            props.setProperty("mail.smtp.connectiontimeout", "10000");// 与邮件服务器建立连接的时间限制
            props.setProperty("mail.smtp.timeout", "10000");// 邮件smtp读取的时间限制
            props.setProperty("mail.smtp.writetimeout", "10000");// 邮件内容上传的时间限制
            // 2. 根据配置创建会话对象, 用于和邮件服务器交互
            Session session = Session.getDefaultInstance(props);
            session.setDebug(false); // 设置为debug模式, 可以查看详细的发送log
            // 3. 创建邮件
            MimeMessage message = new MimeMessage(session);
            // 4. From: 发件人（昵称有广告嫌疑，避免被邮件服务器误认为是滥发广告以至返回失败，请修改昵称）
            message.setFrom(new InternetAddress(emailConfig.getAccount(), "L2", "UTF-8"));
            // 5. To: 收件人（可以增加多个收件人、抄送、密送）
            // MimeMessage.RecipientType.TO: 发送 MimeMessage.RecipientType.CC：抄送 MimeMessage.RecipientType.BCC：密送
            int size = emails.size();
            // 单个目标邮箱还是多个
            if (size == 1) {
                String email = emails.iterator().next();
                message.setRecipient(Message.RecipientType.TO, new InternetAddress(email, email, "UTF-8"));
            } else {
                InternetAddress[] addresses = new InternetAddress[emails.size()];
                int i = 0;
                for (String email : emails) {
                    addresses[i++] = new InternetAddress(email, email, "UTF-8");
                }
                message.setRecipients(MimeMessage.RecipientType.TO, addresses);
            }
            // 6. Subject: 邮件主题（标题有广告嫌疑，避免被邮件服务器误认为是滥发广告以至返回失败，请修改标题）
            message.setSubject(title, "UTF-8");
            // 7. Content: 邮件正文（可以使用html标签）（内容有广告嫌疑，避免被邮件服务器误认为是滥发广告以至返回失败，请修改发送内容）
            message.setContent(content, "text/html;charset=UTF-8");
            // 8. 设置发件时间
            message.setSentDate(new Date());
            // 9. 保存设置
            message.saveChanges();
            // 10. 根据 Session 获取邮件传输对象
            Transport transport = session.getTransport();
            transport.connect(emailConfig.getAccount(), emailConfig.getPassword());
            // 11. 发送邮件, 发到所有的收件地址, message.getAllRecipients()获取到的是在创建邮件对象时添加的所有收件人, 抄送人, 密送人
            transport.sendMessage(message, message.getAllRecipients());
            // 12. 关闭传输连接
            transport.close();
            return true;
        } catch (Exception e) {
            log.error("发送邮件失败,系统错误！", e);
            return false;
        }
    }
}
