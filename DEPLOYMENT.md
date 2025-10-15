# 🚀 PEMA Platform - راهنمای Deployment

## 📋 خلاصه اسکریپت‌های موجود

پلتفرم PEMA دارای چندین اسکریپت deployment است که هر کدام برای موقعیت خاصی طراحی شده‌اند:

### 🎯 اسکریپت‌های اصلی

| اسکریپت | هدف | SSL | پورت‌ها | محیط |
|---------|------|-----|--------|-------|
| `deploy-manager.sh` | **مدیر کلی** - ترکیب همه اسکریپت‌ها | متغیر | متغیر | همه |
| `unified-deploy.sh` | **یکپارچه** - بدون SSL، ساده | ❌ | 80, 8000 | توسعه/تست |
| `setup_server.sh` | **تولید** - کامل با SSL | ✅ | 443, 8081, 8082 | تولید |
| `scripts/deploy.sh` | **Plugin Architecture** | ✅ | 8000 | تولید |
| `scripts/local-deploy.sh` | **توسعه محلی** | ❌ | 8000 | توسعه |

### 🔧 ابزارهای کمکی

- **Makefile**: سیستم build خودکار
- **nginx/**: تنظیمات Nginx
- **pema-backend-server.service**: Systemd service

## 🚀 راهنمای استفاده

### 1. استفاده از Deploy Manager (پیشنهادی)

```bash
# نمایش تمام گزینه‌ها
./deploy-manager.sh help

# توسعه محلی (بدون SSL)
./deploy-manager.sh dev

# تولید کامل (با SSL) - نیاز به sudo
sudo ./deploy-manager.sh prod

# یکپارچه (بدون SSL، پورت 80) - نیاز به sudo
sudo ./deploy-manager.sh unified

# فقط build
./deploy-manager.sh build

# بررسی سلامت سرویس‌ها
./deploy-manager.sh health
```

### 2. استفاده مستقیم از اسکریپت‌ها

#### 🔹 توسعه سریع
```bash
# build و اجرا
./scripts/local-deploy.sh run

# فقط تست compilation
./scripts/local-deploy.sh test
```

#### 🔹 تولید کامل (SSL)
```bash
# نصب کامل با SSL و Certbot
sudo ./setup_server.sh
```

#### 🔹 یکپارچه (بدون SSL)
```bash
# نصب یکپارچه روی پورت 80
sudo ./unified-deploy.sh
```

#### 🔹 Plugin Architecture
```bash
# معماری plugin-based
sudo ./scripts/deploy.sh
```

## 🏗️ معماری Deployment

### 📊 اولویت‌بندی سرویس‌ها

```
1. PostgreSQL (بدون SSL)
   ↓
2. Nginx (پورت 80، بدون SSL)
   ↓
3. Backend Services (پورت 8000)
   ↓
4. Frontend (Static files در Nginx)
```

### 🔌 پورت‌های استفاده شده

| سرویس | پورت | توضیحات |
|--------|------|---------|
| Frontend | 80 | Nginx static files |
| Backend API | 8000 | اصلی API server |
| Auth Backend | 8081 | سرویس احراز هویت (در برخی configs) |
| General Backend | 8082 | سرویس عمومی (در برخی configs) |
| PostgreSQL | 5432 | پایگاه داده |

### 🗄️ پایگاه داده

```bash
# تنظیمات پیش‌فرض
Database: pema_db
User: pema_user
Password: F8s77@98
Host: localhost:5432
SSL: Disabled
```

## 🔧 مدیریت سرویس‌ها

### 📊 بررسی وضعیت
```bash
# وضعیت کلی
./deploy-manager.sh status

# بررسی سلامت
./deploy-manager.sh health

# مشاهده لاگ‌ها
./deploy-manager.sh logs
```

### ▶️ کنترل سرویس‌ها
```bash
# شروع همه سرویس‌ها
sudo ./deploy-manager.sh start

# توقف همه سرویس‌ها
sudo ./deploy-manager.sh stop

# راه‌اندازی مجدد
sudo ./deploy-manager.sh restart
```

### 🔍 دستورات مفید Systemd
```bash
# وضعیت backend
sudo systemctl status pema-backend.service

# لاگ‌های زنده
sudo journalctl -u pema-backend.service -f

# راه‌اندازی مجدد
sudo systemctl restart pema-backend.service
```

## 🌐 تست Deployment

### 🧪 تست‌های اولیه
```bash
# تست compilation
./deploy-manager.sh test

# بررسی سلامت
./deploy-manager.sh health

# تست API
curl http://localhost/health
curl http://localhost/api/

# تست Frontend
curl http://localhost/
```

### 🔍 عیب‌یابی

#### مشکلات رایج:

1. **Backend start نمی‌شود**
   ```bash
   sudo journalctl -u pema-backend.service -n 50
   ```

2. **Database connection error**
   ```bash
   sudo systemctl status postgresql
   sudo -u postgres psql -c "\l"
   ```

3. **Nginx 502 Bad Gateway**
   ```bash
   sudo nginx -t
   sudo systemctl status nginx
   netstat -tuln | grep :8000
   ```

4. **Port already in use**
   ```bash
   sudo netstat -tulpn | grep :80
   sudo fuser -k 80/tcp
   ```

## 📁 ساختار فایل‌ها

```
pema-platform-rust/
├── deploy-manager.sh          # مدیر کلی deployment
├── unified-deploy.sh          # deployment یکپارچه
├── setup_server.sh           # تولید کامل
├── scripts/
│   ├── deploy.sh            # plugin architecture
│   ├── local-deploy.sh      # توسعه محلی
│   └── validate-dependabot.sh
├── nginx/
│   ├── nginx.conf
│   └── conf.d/default.conf
├── Makefile                 # build automation
└── DEPLOYMENT.md           # این فایل
```

## 🎯 انتخاب اسکریپت مناسب

### 🔹 برای توسعه:
```bash
./deploy-manager.sh dev
# یا
./scripts/local-deploy.sh run
```

### 🔹 برای تست محلی:
```bash
sudo ./deploy-manager.sh unified
```

### 🔹 برای تولید:
```bash
sudo ./deploy-manager.sh prod
```

### 🔹 برای معماری plugin:
```bash
sudo ./deploy-manager.sh plugin
```

## 🚨 نکات مهم

1. **SSL**: اسکریپت‌های unified و development بدون SSL هستند
2. **Root Access**: اکثر deployments نیاز به sudo دارند
3. **Database**: PostgreSQL باید قبل از backend راه‌اندازی شود
4. **Ports**: مطمئن شوید پورت‌ها آزاد هستند
5. **Dependencies**: Rust, Cargo, و PostgreSQL باید نصب باشند

## 📞 پشتیبانی

در صورت مشکل:
1. `./deploy-manager.sh health` را اجرا کنید
2. `./deploy-manager.sh logs` را بررسی کنید
3. لاگ‌های systemd را چک کنید
4. تنظیمات nginx را بررسی کنید

---

**نکته**: همه اسکریپت‌ها حفظ شده‌اند و `deploy-manager.sh` آنها را به صورت یکپارچه مدیریت می‌کند.