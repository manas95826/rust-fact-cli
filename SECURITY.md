# Security Policy

## 🔒 Security Considerations

This project handles sensitive data and external API communications. Please follow these security guidelines:

### **Environment Variables**
- **Never commit** `.env` files containing real credentials
- **Use `.env.example`** as a template for required environment variables
- **Keep your bot token and chat ID secure** - they provide access to your Telegram account

### **API Security**
- The tool makes HTTP requests to external APIs
- **Rate limiting** is implemented to prevent API abuse
- **Timeout handling** prevents hanging requests
- **Error handling** ensures graceful failure without exposing sensitive data

### **Telegram Bot Security**
- **Bot tokens** provide full access to your bot - keep them secret
- **Chat IDs** are personal identifiers - don't share them publicly
- **Bot permissions** should be limited to necessary functions only

## 🛡️ Supported Versions

| Version | Supported          | Security Updates |
| ------- | ------------------ | ---------------- |
| 0.1.x   | :white_check_mark: | :white_check_mark: |
| < 0.1   | :x:                | :x:                |

## 🚨 Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

### **How to Report**
1. **Email**: [your-email@example.com] (replace with your actual email)
2. **Subject**: "Security Vulnerability in Fact CLI"
3. **Include**: 
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### **What to Expect**
- **Response time**: Within 48 hours
- **Updates**: Regular updates on investigation progress
- **Resolution**: Fix will be released as soon as possible
- **Credit**: Contributors will be credited (unless anonymity is requested)

### **Please Do NOT**
- ❌ Open public issues for security vulnerabilities
- ❌ Share sensitive information in public repositories
- ❌ Use the vulnerability for malicious purposes

## 🔧 Security Best Practices

### **For Users**
- Keep your `.env` file secure and never share it
- Regularly rotate your Telegram bot token
- Monitor your bot's activity for unusual behavior
- Keep the tool updated to the latest version

### **For Developers**
- Never log sensitive information
- Use environment variables for all secrets
- Implement proper error handling
- Follow Rust security best practices
- Regular dependency updates

## 📋 Security Checklist

- [ ] `.env` file is in `.gitignore`
- [ ] No hardcoded credentials in source code
- [ ] API requests use HTTPS only
- [ ] Error messages don't expose sensitive data
- [ ] Dependencies are up to date
- [ ] Input validation is implemented
- [ ] Rate limiting is in place

---

**Remember**: Security is everyone's responsibility. When in doubt, ask questions!
