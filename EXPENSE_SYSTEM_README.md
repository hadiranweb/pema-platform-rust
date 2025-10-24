# سیستم مدیریت هزینه‌های استارتاپ
## Expense Management System - MVP

یک سیستم تک‌صفحه‌ای (SPA) برای مدیریت هزینه‌های استارتاپ، ساخته شده با **Rust** (Backend) و **Remix** (Frontend).

---

## 📋 ویژگی‌های MVP

### ✅ بخش‌های پیاده‌شده:

1. **ثبت هزینه جدید**
   - انتخاب تاریخ
   - انتخاب دسته‌بندی (مواد اولیه، دستمزد، سربار، اداری، R&D، بازاریابی، سایر)
   - وارد کردن مبلغ
   - اضافه کردن توضیحات (اختیاری)

2. **نمایش خلاصه هزینه‌ها**
   - کارت KPI: جمع کل هزینه‌ها و تعداد
   - لیست آخرین 10 هزینه‌ی ثبت شده
   - نمایش تاریخ، دسته، مبلغ، و توضیحات برای هر هزینه

3. **Backend API**
   - `GET /expenses` - دریافت لیست تمام هزینه‌ها
   - `POST /expenses` - ثبت هزینه جدید

---

## 🏗️ معماری پروژه

```
pema-platform-rust/
├── expense-backend/          # Backend Rust (Axum)
│   ├── src/
│   │   └── main.rs          # API Server
│   ├── Cargo.toml           # وابستگی‌های Rust
│   └── .gitignore
│
├── expense-frontend/         # Frontend Remix
│   ├── app/
│   │   ├── root.tsx         # Layout اصلی
│   │   └── routes/
│   │       └── _index.tsx   # صفحه اصلی (فرم + خلاصه)
│   ├── package.json
│   ├── vite.config.ts
│   ├── remix.config.js
│   ├── tsconfig.json
│   └── .gitignore
│
└── Cargo.toml               # Workspace Rust
```

---

## 🚀 نحوه اجرا

### پیش‌نیازها:
- **Rust** (1.70+) - [نصب](https://rustup.rs/)
- **Node.js** (18+) - [نصب](https://nodejs.org/)
- **npm** یا **pnpm**

### مراحل اجرا:

#### 1️⃣ Backend (Rust)

```bash
cd expense-backend

# کامپایل و اجرا در حالت توسعه
cargo run

# یا کامپایل Release و اجرا
cargo build --release
./target/release/expense-backend
```

Backend در پورت **3000** اجرا می‌شود.

#### 2️⃣ Frontend (Remix)

```bash
cd expense-frontend

# نصب وابستگی‌ها (اگر قبلاً نصب نشده)
npm install

# اجرا در حالت توسعه
npm run dev
```

Frontend در پورت **3001** اجرا می‌شود.

#### 3️⃣ دسترسی به سیستم

باز کنید: **http://localhost:3001**

---

## 🔌 API Documentation

### GET /expenses
**دریافت لیست تمام هزینه‌ها**

```bash
curl http://localhost:3000/expenses
```

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "date": "2025-10-24",
    "category": "مواد اولیه",
    "amount": 1500000,
    "description": "خرید مواد اولیه"
  }
]
```

### POST /expenses
**ثبت هزینه جدید**

```bash
curl -X POST http://localhost:3000/expenses \
  -H "Content-Type: application/json" \
  -d '{
    "date": "2025-10-24",
    "category": "دستمزد",
    "amount": 5000000,
    "description": "حقوق کارمندان"
  }'
```

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "date": "2025-10-24",
  "category": "دستمزد",
  "amount": 5000000,
  "description": "حقوق کارمندان"
}
```

---

## 🎨 رابط کاربری

صفحه اصلی شامل دو بخش است:

### سمت چپ: فرم ثبت هزینه
- **تاریخ**: انتخاب از تقویم
- **دسته‌بندی**: لیست کشویی 7 دسته
- **مبلغ**: ورودی عددی
- **توضیحات**: متن آزاد

### سمت راست: خلاصه هزینه‌ها
- **کارت KPI**: نمایش جمع کل و تعداد
- **لیست هزینه‌ها**: آخرین 10 مورد با جزئیات

---

## 🔄 CORS Configuration

Backend به طور خودکار CORS را برای تمام دامنه‌ها فعال می‌کند (مناسب برای توسعه).

**برای محیط Production:**
تغییر این خط در `expense-backend/src/main.rs`:

```rust
let cors = CorsLayer::new()
    .allow_origin("https://yourdomain.com".parse().unwrap())
    .allow_methods(Any)
    .allow_headers(Any);
```

---

## 📦 Build برای Production

### Backend:
```bash
cd expense-backend
cargo build --release
# Binary در: target/release/expense-backend
```

### Frontend:
```bash
cd expense-frontend
npm run build
# Build output در: build/ و public/build/
```

---

## 🐛 Troubleshooting

### خطا: "Cannot connect to backend"
- مطمئن شوید Backend در پورت 3000 اجرا می‌شود
- بررسی کنید که CORS فعال است

### خطا: "Port already in use"
```bash
# تغییر پورت Backend در src/main.rs
# یا بسته کردن فرآیند قبلی
lsof -i :3000  # برای پیدا کردن PID
kill -9 <PID>
```

### خطا: "npm install fails"
```bash
npm cache clean --force
rm -rf node_modules package-lock.json
npm install
```

---

## 📚 منابع و مراجع

- [Axum Documentation](https://docs.rs/axum/)
- [Remix Documentation](https://remix.run/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [React Documentation](https://react.dev/)

---

## 🚀 مراحل بعدی (برای توسعه بیشتر)

- [ ] اضافه کردن Database (PostgreSQL)
- [ ] اضافه کردن Authentication
- [ ] اضافه کردن گزارش‌های پیشرفته
- [ ] اضافه کردن Export به PDF/Excel
- [ ] اضافه کردن Dashboard با نمودارها
- [ ] اضافه کردن Mobile App

---

## 📝 License

MIT

---

**نسخه:** 0.1.0 (MVP)  
**تاریخ:** 24 اکتبر 2025  
**توسعه‌دهندگان:** Pema Platform Team

