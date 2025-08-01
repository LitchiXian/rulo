package com.l2.util;

import com.l2.config.EmailConfig;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.apache.commons.collections4.CollectionUtils;
import org.springframework.stereotype.Component;

import javax.mail.*;
import javax.mail.internet.InternetAddress;
import javax.mail.internet.MimeMessage;
import java.io.UnsupportedEncodingException;
import java.util.Date;
import java.util.List;
import java.util.Properties;
import java.util.Set;
import java.util.regex.Pattern;
import java.util.stream.Collectors;

/**
 * @Author: Linzx
 * @Date: 2025/7/21 19:41
 * @Desc: MailUtil
 */
@Component
@RequiredArgsConstructor
@Slf4j
public class MailUtil {
    private final EmailConfig config;

    // 邮箱正则验证
    private static final Pattern EMAIL_PATTERN =
            Pattern.compile("^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,6}$");

    /**
     * 发送简单文本邮件
     */
    public boolean sendTextEmail(Set<String> toUsers, String subject, String content) {
        return sendEmail(toUsers, subject, content, false);
    }

    /**
     * 发送HTML格式邮件
     */
    public boolean sendHtmlEmail(Set<String> toUsers, String subject, String content) {
        return sendEmail(toUsers, subject, content, true);
    }

    /**
     * 核心邮件发送方法
     */
    private boolean sendEmail(Set<String> toUsers, String subject, String content, boolean isHtml) {
        if (CollectionUtils.isEmpty(toUsers)) {
            log.warn("收件人列表为空，取消邮件发送");
            return false;
        }

        // 邮箱地址校验
        List<String> invalidEmails = toUsers.stream()
                .filter(email -> !EMAIL_PATTERN.matcher(email).matches())
                .collect(Collectors.toList());

        if (!invalidEmails.isEmpty()) {
            log.error("发现无效邮箱地址: {}", invalidEmails);
            throw new IllegalArgumentException("Invalid email address");
        }

        try {
            Properties props = new Properties();
            props.put("mail.transport.protocol", config.getProtocol());
            props.put("mail.smtp.host", config.getHost());
            props.put("mail.smtp.port", config.getPort());
            props.put("mail.smtp.auth", "true");

            if (config.isSslEnable()) {
                props.put("mail.smtp.ssl.enable", "true");
                props.put("mail.smtp.ssl.protocols", "TLSv1.2");
            }

            // 设置超时时间
            props.put("mail.smtp.connectiontimeout", config.getTimeout());
            props.put("mail.smtp.timeout", config.getTimeout());

            Session session = Session.getInstance(props);

            MimeMessage message = createMimeMessage(session, toUsers, subject, content, isHtml);

            Transport transport = session.getTransport();
            transport.connect(config.getAccount(), config.getPassword());
            transport.sendMessage(message, message.getAllRecipients());
            transport.close();

            return true;
        } catch (AuthenticationFailedException e) {
            log.error("邮件认证失败，请检查账号密码", e);
        } catch (MessagingException e) {
            log.error("邮件发送异常", e);
        } catch (IllegalArgumentException e) {
            log.error("参数校验失败", e);
        } catch (UnsupportedEncodingException e) {
            throw new RuntimeException(e);
        }

        return false;
    }

    // 创建MimeMessage的工厂方法
    private MimeMessage createMimeMessage(Session session, Set<String> toUsers,
                                          String subject, String content, boolean isHtml)
            throws MessagingException, UnsupportedEncodingException {

        MimeMessage message = new MimeMessage(session);
        message.setFrom(new InternetAddress(config.getAccount(), config.getFromName()));

        // 处理收件人
        InternetAddress[] addresses = toUsers.stream()
                .map(email -> {
                    try {
                        return new InternetAddress(email, email);
                    } catch (UnsupportedEncodingException e) {
                        throw new RuntimeException(e);
                    }
                })
                .toArray(InternetAddress[]::new);
        message.setRecipients(Message.RecipientType.TO, addresses);

        message.setSubject(subject, "UTF-8");
        message.setContent(content, isHtml ? "text/html;charset=UTF-8" : "text/plain;charset=UTF-8");
        message.setSentDate(new Date());

        return message;
    }
}
