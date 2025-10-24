import { Html, Head, Meta, Links, Scripts, Outlet } from "@remix-run/react";

export default function Root() {
  return (
    <Html lang="fa" dir="rtl">
      <Head>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <title>سیستم مدیریت هزینه‌ها</title>
        <style>{`
          * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
          }
          body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
          }
          .container {
            max-width: 1200px;
            margin: 0 auto;
          }
          h1 {
            color: white;
            text-align: center;
            margin-bottom: 30px;
            font-size: 2.5rem;
            text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);
          }
        `}</style>
      </Head>
      <body>
        <div className="container">
          <h1>🏢 سیستم مدیریت هزینه‌های استارتاپ</h1>
          <Outlet />
        </div>
        <Scripts />
      </body>
    </Html>
  );
}

