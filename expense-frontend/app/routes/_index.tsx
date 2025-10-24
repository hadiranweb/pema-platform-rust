import { useState, useEffect } from "react";

interface Expense {
  id: string;
  date: string;
  category: string;
  amount: number;
  description: string;
}

export default function Index() {
  const [expenses, setExpenses] = useState<Expense[]>([]);
  const [loading, setLoading] = useState(false);
  const [formData, setFormData] = useState({
    date: new Date().toISOString().split("T")[0],
    category: "مواد اولیه",
    amount: "",
    description: "",
  });

  // دریافت لیست هزینه‌ها از Backend
  useEffect(() => {
    fetchExpenses();
  }, []);

  const fetchExpenses = async () => {
    try {
      setLoading(true);
      const response = await fetch("http://localhost:3000/expenses");
      if (response.ok) {
        const data = await response.json();
        setExpenses(data);
      }
    } catch (error) {
      console.error("خطا در دریافت هزینه‌ها:", error);
    } finally {
      setLoading(false);
    }
  };

  const handleInputChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!formData.amount || parseFloat(formData.amount) <= 0) {
      alert("لطفاً مبلغ معتبر وارد کنید");
      return;
    }

    try {
      const response = await fetch("http://localhost:3000/expenses", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          date: formData.date,
          category: formData.category,
          amount: parseFloat(formData.amount),
          description: formData.description,
        }),
      });

      if (response.ok) {
        const newExpense = await response.json();
        setExpenses((prev) => [newExpense, ...prev]);
        setFormData({
          date: new Date().toISOString().split("T")[0],
          category: "مواد اولیه",
          amount: "",
          description: "",
        });
        alert("هزینه با موفقیت ثبت شد");
      } else {
        alert("خطا در ثبت هزینه");
      }
    } catch (error) {
      console.error("خطا:", error);
      alert("خطا در ارتباط با سرور");
    }
  };

  const totalAmount = expenses.reduce((sum, exp) => sum + exp.amount, 0);
  const categories = [
    "مواد اولیه",
    "دستمزد",
    "سربار",
    "اداری",
    "R&D",
    "بازاریابی",
    "سایر",
  ];

  return (
    <div style={{ padding: "20px" }}>
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: "20px" }}>
        {/* فرم ثبت هزینه */}
        <div
          style={{
            backgroundColor: "white",
            borderRadius: "10px",
            padding: "20px",
            boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
          }}
        >
          <h2 style={{ marginBottom: "20px", color: "#333" }}>📝 ثبت هزینه جدید</h2>
          <form onSubmit={handleSubmit}>
            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                تاریخ:
              </label>
              <input
                type="date"
                name="date"
                value={formData.date}
                onChange={handleInputChange}
                required
                style={{
                  width: "100%",
                  padding: "8px",
                  border: "1px solid #ddd",
                  borderRadius: "5px",
                }}
              />
            </div>

            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                دسته‌بندی:
              </label>
              <select
                name="category"
                value={formData.category}
                onChange={handleInputChange}
                required
                style={{
                  width: "100%",
                  padding: "8px",
                  border: "1px solid #ddd",
                  borderRadius: "5px",
                }}
              >
                {categories.map((cat) => (
                  <option key={cat} value={cat}>
                    {cat}
                  </option>
                ))}
              </select>
            </div>

            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                مبلغ (تومان):
              </label>
              <input
                type="number"
                name="amount"
                value={formData.amount}
                onChange={handleInputChange}
                placeholder="0"
                min="0"
                step="1000"
                required
                style={{
                  width: "100%",
                  padding: "8px",
                  border: "1px solid #ddd",
                  borderRadius: "5px",
                }}
              />
            </div>

            <div style={{ marginBottom: "15px" }}>
              <label style={{ display: "block", marginBottom: "5px", fontWeight: "bold" }}>
                توضیحات:
              </label>
              <textarea
                name="description"
                value={formData.description}
                onChange={handleInputChange}
                placeholder="توضیحات اضافی (اختیاری)"
                style={{
                  width: "100%",
                  padding: "8px",
                  border: "1px solid #ddd",
                  borderRadius: "5px",
                  minHeight: "80px",
                  fontFamily: "inherit",
                }}
              />
            </div>

            <button
              type="submit"
              style={{
                width: "100%",
                padding: "10px",
                backgroundColor: "#667eea",
                color: "white",
                border: "none",
                borderRadius: "5px",
                fontSize: "16px",
                fontWeight: "bold",
                cursor: "pointer",
                transition: "background-color 0.3s",
              }}
              onMouseOver={(e) => {
                e.currentTarget.style.backgroundColor = "#764ba2";
              }}
              onMouseOut={(e) => {
                e.currentTarget.style.backgroundColor = "#667eea";
              }}
            >
              ✅ ثبت هزینه
            </button>
          </form>
        </div>

        {/* خلاصه هزینه‌ها */}
        <div
          style={{
            backgroundColor: "white",
            borderRadius: "10px",
            padding: "20px",
            boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
          }}
        >
          <h2 style={{ marginBottom: "20px", color: "#333" }}>📊 خلاصه هزینه‌ها</h2>

          {/* KPI Card */}
          <div
            style={{
              backgroundColor: "#667eea",
              color: "white",
              padding: "15px",
              borderRadius: "8px",
              marginBottom: "20px",
              textAlign: "center",
            }}
          >
            <div style={{ fontSize: "14px", marginBottom: "5px" }}>جمع کل هزینه‌ها</div>
            <div style={{ fontSize: "28px", fontWeight: "bold" }}>
              {totalAmount.toLocaleString("fa-IR")} ریال
            </div>
            <div style={{ fontSize: "12px", marginTop: "5px" }}>
              تعداد: {expenses.length} مورد
            </div>
          </div>

          {/* لیست آخرین هزینه‌ها */}
          <h3 style={{ marginBottom: "10px", color: "#333", fontSize: "16px" }}>
            🔝 آخرین هزینه‌ها
          </h3>
          {loading ? (
            <p style={{ color: "#666" }}>در حال بارگذاری...</p>
          ) : expenses.length === 0 ? (
            <p style={{ color: "#999" }}>هیچ هزینه‌ای ثبت نشده است</p>
          ) : (
            <div style={{ maxHeight: "400px", overflowY: "auto" }}>
              {expenses.slice(0, 10).map((expense) => (
                <div
                  key={expense.id}
                  style={{
                    padding: "10px",
                    borderBottom: "1px solid #eee",
                    display: "flex",
                    justifyContent: "space-between",
                    alignItems: "center",
                  }}
                >
                  <div>
                    <div style={{ fontWeight: "bold", color: "#333" }}>
                      {expense.category}
                    </div>
                    <div style={{ fontSize: "12px", color: "#666" }}>
                      {expense.date} - {expense.description}
                    </div>
                  </div>
                  <div style={{ fontWeight: "bold", color: "#667eea", minWidth: "100px", textAlign: "left" }}>
                    {expense.amount.toLocaleString("fa-IR")} ریال
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

