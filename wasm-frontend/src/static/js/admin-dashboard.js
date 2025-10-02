/**
 * PEMA Admin Dashboard JavaScript
 * WASM-optimized admin interface with modern UI/UX
 */

class AdminDashboard {
    constructor() {
        this.wasmAuth = null;
        this.wasmGeneral = null;
        this.currentSection = 'dashboard';
        this.sidebarCollapsed = false;
        this.init();
    }

    async init() {
        try {
            // Initialize WASM modules
            await this.initWASM();
            
            // Setup event listeners
            this.setupEventListeners();
            
            // Load initial data
            await this.loadDashboardData();
            
            // Setup real-time updates
            this.setupRealTimeUpdates();
            
            console.log('Admin Dashboard initialized successfully');
        } catch (error) {
            console.error('Failed to initialize Admin Dashboard:', error);
            this.showError('خطا در بارگذاری داشبورد مدیریت');
        }
    }

    async initWASM() {
        try {
            // Load WASM Auth Backend
            const wasmAuthModule = await import('/wasm-auth-backend/pkg/wasm_auth_backend.js');
            await wasmAuthModule.default();
            this.wasmAuth = wasmAuthModule;

            // Load WASM General Backend  
            const wasmGeneralModule = await import('/wasm-general-backend/pkg/wasm_general_backend.js');
            await wasmGeneralModule.default();
            this.wasmGeneral = wasmGeneralModule;

            console.log('WASM modules loaded successfully');
        } catch (error) {
            console.error('Failed to load WASM modules:', error);
            throw new Error('WASM modules initialization failed');
        }
    }

    setupEventListeners() {
        // Sidebar toggle
        const sidebarToggle = document.getElementById('sidebarToggle');
        const mobileSidebarToggle = document.getElementById('mobileSidebarToggle');
        
        if (sidebarToggle) {
            sidebarToggle.addEventListener('click', () => this.toggleSidebar());
        }
        
        if (mobileSidebarToggle) {
            mobileSidebarToggle.addEventListener('click', () => this.toggleMobileSidebar());
        }

        // Navigation items
        document.querySelectorAll('.nav-item').forEach(item => {
            item.addEventListener('click', (e) => {
                e.preventDefault();
                const section = item.getAttribute('data-section');
                if (section) {
                    this.navigateToSection(section);
                }
            });
        });

        // User dropdown
        const userBtn = document.getElementById('userBtn');
        const userDropdown = document.getElementById('userDropdown');
        
        if (userBtn && userDropdown) {
            userBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                this.toggleDropdown(userDropdown);
            });
        }

        // Notifications dropdown
        const notificationsBtn = document.getElementById('notificationsBtn');
        const notificationsDropdown = document.getElementById('notificationsDropdown');
        
        if (notificationsBtn && notificationsDropdown) {
            notificationsBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                this.toggleDropdown(notificationsDropdown);
            });
        }

        // Global search
        const globalSearch = document.getElementById('globalSearch');
        if (globalSearch) {
            globalSearch.addEventListener('input', (e) => {
                this.performGlobalSearch(e.target.value);
            });
        }

        // Close dropdowns when clicking outside
        document.addEventListener('click', () => {
            document.querySelectorAll('.dropdown-menu.show').forEach(dropdown => {
                dropdown.classList.remove('show');
            });
        });

        // User table filters
        this.setupTableFilters();

        // Chart period selector
        const chartPeriod = document.querySelector('.chart-period');
        if (chartPeriod) {
            chartPeriod.addEventListener('change', (e) => {
                this.updateSalesChart(e.target.value);
            });
        }

        // Add user button
        const addUserBtn = document.getElementById('addUserBtn');
        if (addUserBtn) {
            addUserBtn.addEventListener('click', () => this.showAddUserModal());
        }

        // Export users button
        const exportUsersBtn = document.getElementById('exportUsersBtn');
        if (exportUsersBtn) {
            exportUsersBtn.addEventListener('click', () => this.exportUsers());
        }

        // Google Login Placeholder
        const googleLoginBtn = document.getElementById('googleLoginBtn');
        if (googleLoginBtn) {
            googleLoginBtn.addEventListener('click', () => this.handleGoogleLogin());
        }
    }

    toggleSidebar() {
        const sidebar = document.getElementById('adminSidebar');
        if (sidebar) {
            this.sidebarCollapsed = !this.sidebarCollapsed;
            sidebar.classList.toggle('collapsed', this.sidebarCollapsed);
            
            // Save preference
            localStorage.setItem('sidebarCollapsed', this.sidebarCollapsed);
        }
    }

    toggleMobileSidebar() {
        const sidebar = document.getElementById('adminSidebar');
        if (sidebar) {
            sidebar.classList.toggle('open');
        }
    }

    toggleDropdown(dropdown) {
        // Close other dropdowns
        document.querySelectorAll('.dropdown-menu.show').forEach(other => {
            if (other !== dropdown) {
                other.classList.remove('show');
            }
        });
        
        dropdown.classList.toggle('show');
    }

    navigateToSection(sectionName) {
        // Update active nav item
        document.querySelectorAll('.nav-item').forEach(item => {
            item.classList.remove('active');
        });
        
        document.querySelector(`[data-section="${sectionName}"]`)?.classList.add('active');

        // Hide all sections
        document.querySelectorAll('.content-section').forEach(section => {
            section.classList.remove('active');
        });

        // Show target section
        const targetSection = document.getElementById(`${sectionName}-section`);
        if (targetSection) {
            targetSection.classList.add('active');
            this.currentSection = sectionName;
            
            // Update page title
            this.updatePageTitle(sectionName);
            
            // Load section data
            this.loadSectionData(sectionName);
        }
    }

    updatePageTitle(sectionName) {
        const titles = {
            'dashboard': 'داشبورد مدیریت',
            'analytics': 'تحلیل‌ها',
            'users': 'مدیریت کاربران',
            'products': 'مدیریت محصولات',
            'orders': 'مدیریت سفارشات',
            'vendors': 'مدیریت فروشندگان',
            'reports': 'گزارشات',
            'financial': 'مالی',
            'settings': 'تنظیمات',
            'logs': 'لاگ‌ها'
        };
        
        const pageTitle = document.getElementById('pageTitle');
        if (pageTitle && titles[sectionName]) {
            pageTitle.textContent = titles[sectionName];
        }
    }

    async loadDashboardData() {
        try {
            // Load stats
            await this.loadStats();
            
            // Load charts
            await this.loadSalesChart();
            
            // Load recent activity
            await this.loadRecentActivity();
            
            // Load top products
            await this.loadTopProducts();
            
        } catch (error) {
            console.error('Failed to load dashboard data:', error);
        }
    }

    async loadStats() {
        try {
            if (!this.wasmGeneral) return;

            // Use WASM to get stats data
            // const statsData = this.wasmGeneral.get_dashboard_stats();
            const statsData = {
                total_users: '1,234',
                active_orders: '56',
                daily_revenue: '$12,345',
                total_products: '789'
            }; // Mock data
            
            // Update stat cards
            this.updateStatCard('totalUsers', statsData.total_users);
            this.updateStatCard('activeOrders', statsData.active_orders);
            this.updateStatCard('dailyRevenue', statsData.daily_revenue);
            this.updateStatCard('totalProducts', statsData.total_products);
            
        } catch (error) {
            console.error('Failed to load stats:', error);
            // Use fallback data
            this.updateStatCard('totalUsers', '1,234');
            this.updateStatCard('activeOrders', '56');
            this.updateStatCard('dailyRevenue', '$12,345');
            this.updateStatCard('totalProducts', '789');
        }
    }

    updateStatCard(id, value) {
        const element = document.getElementById(id);
        if (element) {
            element.textContent = value;
            
            // Add animation
            element.style.transform = 'scale(1.1)';
            setTimeout(() => {
                element.style.transform = 'scale(1)';
            }, 200);
        }
    }

    async loadSalesChart(period = '30d') {
        try {
            const canvas = document.getElementById('salesChart');
            if (!canvas) return;

            // Mock data for now - replace with WASM call
            const data = this.generateMockChartData(period);
            
            // Simple canvas chart implementation
            this.drawSalesChart(canvas, data);
            
        } catch (error) {
            console.error('Failed to load sales chart:', error);
        }
    }

    generateMockChartData(period) {
        const days = period === '7d' ? 7 : period === '30d' ? 30 : 90;
        const data = [];
        
        for (let i = 0; i < days; i++) {
            data.push({
                date: new Date(Date.now() - (days - i) * 24 * 60 * 60 * 1000),
                value: Math.floor(Math.random() * 1000) + 500
            });
        }
        
        return data;
    }

    drawSalesChart(canvas, data) {
        const ctx = canvas.getContext('2d');
        const width = canvas.width = canvas.offsetWidth;
        const height = canvas.height = canvas.offsetHeight;
        
        // Clear canvas
        ctx.clearRect(0, 0, width, height);
        
        if (data.length === 0) return;
        
        // Calculate bounds
        const maxValue = Math.max(...data.map(d => d.value));
        const minValue = Math.min(...data.map(d => d.value));
        const padding = 40;
        
        // Draw axes
        ctx.strokeStyle = '#E0E0E0';
        ctx.lineWidth = 1;
        
        // Y-axis
        ctx.beginPath();
        ctx.moveTo(padding, padding);
        ctx.lineTo(padding, height - padding);
        ctx.stroke();
        
        // X-axis
        ctx.beginPath();
        ctx.moveTo(padding, height - padding);
        ctx.lineTo(width - padding, height - padding);
        ctx.stroke();
        
        // Draw data line
        ctx.strokeStyle = '#2E7D32';
        ctx.lineWidth = 2;
        ctx.beginPath();
        
        data.forEach((point, index) => {
            const x = padding + (index / (data.length - 1)) * (width - 2 * padding);
            const y = height - padding - ((point.value - minValue) / (maxValue - minValue)) * (height - 2 * padding);
            
            if (index === 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }
        });
        
        ctx.stroke();
        
        // Draw data points
        ctx.fillStyle = '#2E7D32';
        data.forEach((point, index) => {
            const x = padding + (index / (data.length - 1)) * (width - 2 * padding);
            const y = height - padding - ((point.value - minValue) / (maxValue - minValue)) * (height - 2 * padding);
            
            ctx.beginPath();
            ctx.arc(x, y, 3, 0, 2 * Math.PI);
            ctx.fill();
        });
    }

    async loadRecentActivity() {
        try {
            // Mock data - replace with WASM call
            const activities = [
                {
                    type: 'success',
                    icon: 'fas fa-user-plus',
                    title: 'کاربر جدید ثبت‌نام کرد',
                    description: 'احمد محمدی به سیستم پیوست',
                    time: '5 دقیقه پیش'
                },
                {
                    type: 'warning',
                    icon: 'fas fa-shopping-cart',
                    title: 'سفارش جدید',
                    description: 'سفارش #12345 ثبت شد',
                    time: '10 دقیقه پیش'
                },
                {
                    type: 'info',
                    icon: 'fas fa-box',
                    title: 'محصول جدید',
                    description: 'محصول "لپ‌تاپ گیمینگ" اضافه شد',
                    time: '1 ساعت پیش'
                }
            ];
            
            // Update activity list would go here
            
        } catch (error) {
            console.error('Failed to load recent activity:', error);
        }
    }

    async loadTopProducts() {
        try {
            if (!this.wasmGeneral) return;
            // const topProducts = this.wasmGeneral.get_top_products(); // Example WASM call
            const topProducts = [
                { name: 'لپ‌تاپ گیمینگ', sales: '1,200', progress: 90 },
                { name: 'موس بی‌سیم', sales: '850', progress: 75 },
                { name: 'کیبورد مکانیکی', sales: '700', progress: 60 }
            ]; // Mock data
            this.renderTopProducts(topProducts);
        } catch (error) {
            console.error('Failed to load top products:', error);
        }
    }

    renderTopProducts(products) {
        const topProductsContainer = document.getElementById('topProductsList');
        if (!topProductsContainer) return;
        topProductsContainer.innerHTML = products.map(product => `
            <div class="product-item">
                <div class="product-info">
                    <div class="product-name">${product.name}</div>
                    <div class="product-sales">فروش: ${product.sales}</div>
                </div>
                <div class="product-progress">
                    <div class="progress-bar" style="width: ${product.progress}%;"></div>
                </div>
            </div>
        `).join('');
    }

    async loadSectionData(sectionName) {
        switch (sectionName) {
            case 'users':
                await this.loadUsersData();
                break;
            case 'products':
                await this.loadProductsData();
                break;
            case 'orders':
                await this.loadOrdersData();
                break;
            case 'vendors':
                await this.loadVendorsData();
                break;
            case 'reports':
                await this.loadReportsData();
                break;
            case 'settings':
                await this.loadSettingsData();
                break;
            default:
                break;
        }
    }

    async loadUsersData() {
        try {
            if (!this.wasmGeneral) return;

            // Mock users data - replace with WASM call
            // const users = this.wasmGeneral.get_users();
            const users = [
                {
                    id: 1,
                    name: 'احمد محمدی',
                    email: 'ahmad@example.com',
                    role: 'customer',
                    status: 'active',
                    joinDate: '1403/01/15'
                },
                {
                    id: 2,
                    name: 'فاطمه احمدی',
                    email: 'fateme@example.com',
                    role: 'vendor',
                    status: 'active',
                    joinDate: '1403/01/10'
                },
                {
                    id: 3,
                    name: 'علی حسینی',
                    email: 'ali@example.com',
                    role: 'admin',
                    status: 'active',
                    joinDate: '1402/12/01'
                }
            ];
            
            this.renderUsersTable(users);
            
        } catch (error) {
            console.error('Failed to load users data:', error);
        }
    }

    renderUsersTable(users) {
        const tbody = document.getElementById('usersTableBody');
        if (!tbody) return;

        tbody.innerHTML = users.map(user => `
            <tr>
                <td>
                    <input type="checkbox" value="${user.id}">
                </td>
                <td>
                    <div style="display: flex; align-items: center; gap: 8px;">
                        <div class="user-avatar small">
                            <div class="avatar-fallback">${user.name.charAt(0)}</div>
                        </div>
                        <span>${user.name}</span>
                    </div>
                </td>
                <td>${user.email}</td>
                <td>
                    <span class="role-badge ${user.role}">${this.getRoleLabel(user.role)}</span>
                </td>
                <td>
                    <span class="status-badge ${user.status}">${this.getStatusLabel(user.status)}</span>
                </td>
                <td>${user.joinDate}</td>
                <td>
                    <div class="action-buttons">
                        <button class="action-btn" onclick="adminDashboard.editUser(${user.id})" title="ویرایش">
                            <i class="fas fa-edit"></i>
                        </button>
                        <button class="action-btn" onclick="adminDashboard.deleteUser(${user.id})" title="حذف">
                            <i class="fas fa-trash"></i>
                        </button>
                    </div>
                </td>
            </tr>
        `).join('');
    }

    getRoleLabel(role) {
        const labels = {
            'admin': 'مدیر',
            'vendor': 'فروشنده',
            'customer': 'مشتری'
        };
        return labels[role] || role;
    }

    getStatusLabel(status) {
        const labels = {
            'active': 'فعال',
            'inactive': 'غیرفعال',
            'banned': 'مسدود'
        };
        return labels[status] || status;
    }

    setupTableFilters() {
        const userSearch = document.getElementById('userSearch');
        const userRoleFilter = document.getElementById('userRoleFilter');
        const userStatusFilter = document.getElementById('userStatusFilter');

        if (userSearch) {
            userSearch.addEventListener('input', () => this.filterUsers());
        }

        if (userRoleFilter) {
            userRoleFilter.addEventListener('change', () => this.filterUsers());
        }

        if (userStatusFilter) {
            userStatusFilter.addEventListener('change', () => this.filterUsers());
        }
    }

    filterUsers() {
        // Implementation for filtering users table
        console.log('Filtering users...');
    }

    performGlobalSearch(query) {
        if (query.length < 2) return;
        
        console.log('Global search:', query);
        // Implementation for global search
    }

    setupRealTimeUpdates() {
        // Setup periodic updates for dashboard data
        setInterval(() => {
            if (this.currentSection === 'dashboard') {
                this.loadStats();
            }
        }, 30000); // Update every 30 seconds
    }

    showAddUserModal() {
        // Implementation for add user modal
        console.log('Show add user modal');
    }

    exportUsers() {
        // Implementation for exporting users
        console.log('Export users');
    }

    editUser(userId) {
        console.log('Edit user:', userId);
    }

    deleteUser(userId) {
        if (confirm('آیا از حذف این کاربر اطمینان دارید؟')) {
            console.log('Delete user:', userId);
        }
    }

    async loadProductsData() {
        try {
            if (!this.wasmGeneral) return;
            // const products = this.wasmGeneral.get_products(); // Example WASM call
            const products = [
                { id: 1, name: 'لپ‌تاپ گیمینگ', category: 'الکترونیک', price: '$1500', stock: 50, status: 'in-stock' },
                { id: 2, name: 'موس بی‌سیم', category: 'لوازم جانبی', price: '$50', stock: 200, status: 'in-stock' },
                { id: 3, name: 'کیبورد مکانیکی', category: 'لوازم جانبی', price: '$120', stock: 80, status: 'low-stock' }
            ]; // Mock data
            this.renderProductsTable(products);
        } catch (error) {
            console.error('Failed to load products data:', error);
        }
    }

    renderProductsTable(products) {
        const tbody = document.getElementById('productsTableBody');
        if (!tbody) return;
        tbody.innerHTML = products.map(product => `
            <tr>
                <td>${product.id}</td>
                <td>${product.name}</td>
                <td>${product.category}</td>
                <td>${product.price}</td>
                <td>${product.stock}</td>
                <td><span class="status-badge ${product.status}">${product.status}</span></td>
                <td>
                    <div class="action-buttons">
                        <button class="action-btn" onclick="adminDashboard.editProduct(${product.id})"><i class="fas fa-edit"></i></button>
                        <button class="action-btn" onclick="adminDashboard.deleteProduct(${product.id})"><i class="fas fa-trash"></i></button>
                    </div>
                </td>
            </tr>
        `).join('');
    }

    async loadOrdersData() {
        try {
            if (!this.wasmGeneral) return;
            // const orders = this.wasmGeneral.get_orders(); // Example WASM call
            const orders = [
                { id: 101, customer: 'احمد محمدی', total: '$250', date: '1403/07/01', status: 'pending' },
                { id: 102, customer: 'فاطمه احمدی', total: '$1500', date: '1403/06/28', status: 'completed' },
                { id: 103, customer: 'علی حسینی', total: '$75', date: '1403/06/25', status: 'cancelled' }
            ]; // Mock data
            this.renderOrdersTable(orders);
        } catch (error) {
            console.error('Failed to load orders data:', error);
        }
    }

    renderOrdersTable(orders) {
        const tbody = document.getElementById('ordersTableBody');
        if (!tbody) return;
        tbody.innerHTML = orders.map(order => `
            <tr>
                <td>${order.id}</td>
                <td>${order.customer}</td>
                <td>${order.total}</td>
                <td>${order.date}</td>
                <td><span class="status-badge ${order.status}">${order.status}</span></td>
                <td>
                    <div class="action-buttons">
                        <button class="action-btn" onclick="adminDashboard.viewOrder(${order.id})"><i class="fas fa-eye"></i></button>
                        <button class="action-btn" onclick="adminDashboard.editOrder(${order.id})"><i class="fas fa-edit"></i></button>
                    </div>
                </td>
            </tr>
        `).join('');
    }

    async loadVendorsData() {
        try {
            if (!this.wasmGeneral) return;
            // const vendors = this.wasmGeneral.get_vendors(); // Example WASM call
            const vendors = [
                { id: 1, name: 'فروشگاه تکنولوژی', email: 'tech@example.com', products: 150, status: 'active' },
                { id: 2, name: 'لوازم خانگی مدرن', email: 'home@example.com', products: 80, status: 'active' },
                { id: 3, name: 'کتابفروشی دانش', email: 'books@example.com', products: 30, status: 'inactive' }
            ]; // Mock data
            this.renderVendorsTable(vendors);
        } catch (error) {
            console.error('Failed to load vendors data:', error);
        }
    }

    renderVendorsTable(vendors) {
        const tbody = document.getElementById('vendorsTableBody');
        if (!tbody) return;
        tbody.innerHTML = vendors.map(vendor => `
            <tr>
                <td>${vendor.id}</td>
                <td>${vendor.name}</td>
                <td>${vendor.email}</td>
                <td>${vendor.products}</td>
                <td><span class="status-badge ${vendor.status}">${vendor.status}</span></td>
                <td>
                    <div class="action-buttons">
                        <button class="action-btn" onclick="adminDashboard.viewVendor(${vendor.id})"><i class="fas fa-eye"></i></button>
                        <button class="action-btn" onclick="adminDashboard.editVendor(${vendor.id})"><i class="fas fa-edit"></i></button>
                    </div>
                </td>
            </tr>
        `).join('');
    }

    async loadReportsData() {
        try {
            if (!this.wasmGeneral) return;
            // const reports = this.wasmGeneral.get_reports(); // Example WASM call
            const reports = [
                { id: 1, name: 'گزارش فروش ماهانه', date: '1403/07/01', type: 'PDF', actions: 'Download' },
                { id: 2, name: 'گزارش عملکرد کاربر', date: '1403/06/15', type: 'CSV', actions: 'Download' }
            ]; // Mock data
            this.renderReportsTable(reports);
        } catch (error) {
            console.error('Failed to load reports data:', error);
        }
    }

    renderReportsTable(reports) {
        const tbody = document.getElementById('reportsTableBody');
        if (!tbody) return;
        tbody.innerHTML = reports.map(report => `
            <tr>
                <td>${report.id}</td>
                <td>${report.name}</td>
                <td>${report.date}</td>
                <td>${report.type}</td>
                <td>
                    <div class="action-buttons">
                        <button class="action-btn" onclick="adminDashboard.downloadReport(${report.id})"><i class="fas fa-download"></i></button>
                    </div>
                </td>
            </tr>
        `).join('');
    }

    async loadSettingsData() {
        try {
            if (!this.wasmGeneral) return;
            // const settings = this.wasmGeneral.get_settings(); // Example WASM call
            const settings = [
                { id: 1, name: 'نام سایت', value: 'PEMA Platform', type: 'text' },
                { id: 2, name: 'زبان پیش‌فرض', value: 'فارسی', type: 'select', options: ['فارسی', 'انگلیسی'] }
            ]; // Mock data
            this.renderSettingsForm(settings);
        } catch (error) {
            console.error('Failed to load settings data:', error);
        }
    }

    renderSettingsForm(settings) {
        const settingsFormContainer = document.getElementById('settingsForm');
        if (!settingsFormContainer) return;
        settingsFormContainer.innerHTML = settings.map(setting => {
            if (setting.type === 'text') {
                return `
                    <div class="form-group">
                        <label for="setting-${setting.id}">${setting.name}</label>
                        <input type="text" id="setting-${setting.id}" value="${setting.value}">
                    </div>
                `;
            } else if (setting.type === 'select') {
                const optionsHtml = setting.options.map(option => `
                    <option value="${option}" ${setting.value === option ? 'selected' : ''}>${option}</option>
                `).join('');
                return `
                    <div class="form-group">
                        <label for="setting-${setting.id}">${setting.name}</label>
                        <select id="setting-${setting.id}">
                            ${optionsHtml}
                        </select>
                    </div>
                `;
            }
            return '';
        }).join('');
    }

    handleGoogleLogin() {
        console.log("Initiating Google Login...");
        // Google Sign-In button is rendered by the g-signin2 div
        // The onSignIn callback will handle the actual login process
        this.showInfo("Please click the Google Sign-In button.");
    }

    async onSignIn(googleUser) {
        console.log("Google Sign-In successful!");
        const profile = googleUser.getBasicProfile();
        const id_token = googleUser.getAuthResponse().id_token;

        console.log("ID: " + profile.getId()); // Do not send to your backend!
        console.log("Name: " + profile.getName());
        console.log("Image URL: " + profile.getImageUrl());
        console.log("Email: " + profile.getEmail());
        console.log("ID Token: " + id_token);

        try {
            if (!this.wasmAuth) {
                throw new Error("WASM Auth module not loaded.");
            }
            // Send the ID token to your WASM backend for verification
            // The WASM backend should verify the token and return a session token or user data
            const response = await this.wasmAuth.google_login(id_token);
            console.log("WASM Backend Google Login Response:", response);

            if (response && response.success) {
                this.showSuccess("ورود با گوگل با موفقیت انجام شد!");
                // Redirect or update UI based on successful login
                // Example: window.location.href = "/admin/dashboard";
            } else {
                this.showError(response.message || "خطا در ورود با گوگل از طریق بک‌اند.");
            }
        } catch (error) {
            console.error("Error during Google Login with WASM backend:", error);
            this.showError("خطا در ارتباط با بک‌اند برای ورود با گوگل.");
        }
    }
}

// Make onSignIn globally accessible for Google Sign-In library
window.onSignIn = (googleUser) => {
    if (window.adminDashboard) {
        window.adminDashboard.onSignIn(googleUser);
    } else {
        // If adminDashboard is not yet initialized, store the user and process later
        console.warn("adminDashboard not initialized yet, storing googleUser.");
        window._googleUser = googleUser;
    }
};

    showInfo(message) {
        const notification = document.createElement('div');
        notification.className = 'info-notification';
        notification.textContent = message;
        notification.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            background-color: #2196F3;
            color: white;
            padding: 16px;
            border-radius: 8px;
            z-index: 10000;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        `;
        
        document.body.appendChild(notification);
        
        setTimeout(() => {
            notification.remove();
        }, 5000);
    }

    showError(message) {
        // Simple error notification
        const notification = document.createElement('div');
        notification.className = 'error-notification';
        notification.textContent = message;
        notification.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            background-color: #F44336;
            color: white;
            padding: 16px;
            border-radius: 8px;
            z-index: 10000;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        `;
        
        document.body.appendChild(notification);
        
        setTimeout(() => {
            notification.remove();
        }, 5000);
    }

    showSuccess(message) {
        // Simple success notification
        const notification = document.createElement('div');
        notification.className = 'success-notification';
        notification.textContent = message;
        notification.style.cssText = `
            position: fixed;
            top: 20px;
            right: 20px;
            background-color: #4CAF50;
            color: white;
            padding: 16px;
            border-radius: 8px;
            z-index: 10000;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        `;
        
        document.body.appendChild(notification);
        
        setTimeout(() => {
            notification.remove();
        }, 3000);
    }
}

// Initialize dashboard when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.adminDashboard = new AdminDashboard();
});

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = AdminDashboard;
}

