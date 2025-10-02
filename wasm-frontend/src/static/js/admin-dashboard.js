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
            const statsData = this.wasmGeneral.get_dashboard_stats();
            
            // Update stat cards
            this.updateStatCard('totalUsers', statsData.total_users || '1,234');
            this.updateStatCard('activeOrders', statsData.active_orders || '56');
            this.updateStatCard('dailyRevenue', statsData.daily_revenue || '$12,345');
            this.updateStatCard('totalProducts', statsData.total_products || '789');
            
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
        // Implementation for top products
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
        // Implementation for products data
        console.log('Loading products data...');
    }

    async loadOrdersData() {
        // Implementation for orders data
        console.log('Loading orders data...');
    }

    async loadVendorsData() {
        // Implementation for vendors data
        console.log('Loading vendors data...');
    }

    async loadReportsData() {
        // Implementation for reports data
        console.log('Loading reports data...');
    }

    async loadSettingsData() {
        // Implementation for settings data
        console.log('Loading settings data...');
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
