// Global variables
let currentStep = 1;
let currentUser = null;

// Initialize the page
document.addEventListener('DOMContentLoaded', function() {
    initializeStars();
    initializeShootingStars();
    initializeParallax();
    loadProducts();
    setupEventListeners();
    initializeScrollAnimations();
});

// Create stars in the background
function initializeStars() {
    const starsContainer = document.getElementById('stars');
    const numberOfStars = 300;
    
    for (let i = 0; i < numberOfStars; i++) {
        const star = document.createElement('div');
        star.className = 'star';
        
        // Random position
        star.style.left = Math.random() * 100 + '%';
        star.style.top = Math.random() * 100 + '%';
        
        // Random size
        const size = Math.random() * 4 + 1;
        star.style.width = size + 'px';
        star.style.height = size + 'px';
        
        // Random animation delay
        star.style.animationDelay = Math.random() * 4 + 's';
        
        starsContainer.appendChild(star);
    }
}

// Create shooting stars
function initializeShootingStars() {
    setInterval(() => {
        if (Math.random() < 0.4) { // 40% chance every interval
            createShootingStar();
        }
    }, 1500);
}

function createShootingStar() {
    const shootingStar = document.createElement('div');
    shootingStar.className = 'shooting-star';
    
    // Random starting position
    shootingStar.style.left = Math.random() * 200 + 'px';
    shootingStar.style.top = Math.random() * 200 + 'px';
    
    // Random size
    const size = Math.random() * 4 + 2;
    shootingStar.style.width = size + 'px';
    shootingStar.style.height = size + 'px';
    
    document.body.appendChild(shootingStar);
    
    // Remove after animation
    setTimeout(() => {
        if (shootingStar.parentNode) {
            shootingStar.parentNode.removeChild(shootingStar);
        }
    }, 4000);
}

// Enhanced parallax scrolling effects
function initializeParallax() {
    window.addEventListener('scroll', () => {
        const scrolled = window.pageYOffset;
        const rate = scrolled * -0.6;
        const rateTwo = scrolled * -0.4;
        const rateThree = scrolled * 0.3;
        const rateFour = scrolled * -0.2;
        
        // Moon parallax with enhanced movement
        const moon = document.getElementById('moon');
        if (moon) {
            moon.style.transform = `translateY(${rate}px) rotate(${scrolled * 0.05}deg)`;
        }
        
        // Mountains parallax
        const mountains = document.getElementById('mountains');
        if (mountains) {
            mountains.style.transform = `translateY(${rateThree}px)`;
        }
        
        // Stars parallax
        const stars = document.getElementById('stars');
        if (stars) {
            stars.style.transform = `translateY(${rateTwo}px)`;
        }
        
        // Header background opacity and blur
        const header = document.querySelector('.header');
        if (header) {
            const opacity = Math.min(scrolled / 100, 0.95);
            const blur = Math.min(scrolled / 10, 20);
            header.style.background = `rgba(0, 0, 0, ${opacity})`;
            header.style.backdropFilter = `blur(${blur}px)`;
        }
        
        // Galaxy background movement
        const galaxyBg = document.querySelector('.galaxy-bg');
        if (galaxyBg) {
            galaxyBg.style.transform = `translateY(${rateFour}px)`;
        }
    });
}

// Setup event listeners
function setupEventListeners() {
    // Check user form
    document.getElementById('checkUserForm').addEventListener('submit', handleCheckUser);
    
    // Login form
    document.getElementById('loginForm').addEventListener('submit', handleLogin);
    
    // Verify form
    document.getElementById('verifyForm').addEventListener('submit', handleVerifyCode);
    
    // Register form
    document.getElementById('registerForm').addEventListener('submit', handleRegister);
    
    // Close modal when clicking outside
    window.addEventListener('click', function(event) {
        const modal = document.getElementById('loginModal');
        if (event.target === modal) {
            closeLoginModal();
        }
    });
    
    // Smooth scrolling for navigation links
    document.querySelectorAll('.nav-links a').forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href').substring(1);
            const targetElement = document.getElementById(targetId);
            if (targetElement) {
                targetElement.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Enhanced mouse movement effects
    document.addEventListener('mousemove', handleMouseMove);
}

// Enhanced mouse movement effects
function handleMouseMove(e) {
    const moon = document.getElementById('moon');
    if (moon) {
        const x = (e.clientX / window.innerWidth - 0.5) * 20;
        const y = (e.clientY / window.innerHeight - 0.5) * 20;
        moon.style.transform += ` translate(${x}px, ${y}px)`;
    }
    
    // Parallax effect for stars based on mouse position
    const stars = document.getElementById('stars');
    if (stars) {
        const x = (e.clientX / window.innerWidth - 0.5) * 10;
        const y = (e.clientY / window.innerHeight - 0.5) * 10;
        stars.style.transform += ` translate(${x}px, ${y}px)`;
    }
}

// Modal functions
function openLoginModal() {
    document.getElementById('loginModal').style.display = 'block';
    document.body.style.overflow = 'hidden';
    resetAuthSteps();
    
    // Add entrance animation
    const modalContent = document.querySelector('.modal-content');
    modalContent.style.transform = 'scale(0.8) translateY(50px)';
    modalContent.style.opacity = '0';
    
    setTimeout(() => {
        modalContent.style.transform = 'scale(1) translateY(0)';
        modalContent.style.opacity = '1';
        modalContent.style.transition = 'all 0.3s ease';
    }, 10);
}

function closeLoginModal() {
    const modalContent = document.querySelector('.modal-content');
    modalContent.style.transform = 'scale(0.8) translateY(50px)';
    modalContent.style.opacity = '0';
    
    setTimeout(() => {
        document.getElementById('loginModal').style.display = 'none';
        document.body.style.overflow = 'auto';
        resetAuthSteps();
    }, 300);
}

function resetAuthSteps() {
    // Hide all steps
    document.querySelectorAll('.auth-step').forEach(step => {
        step.classList.remove('active');
    });
    
    // Show first step
    document.getElementById('step1').classList.add('active');
    currentStep = 1;
    
    // Clear forms
    document.querySelectorAll('form').forEach(form => {
        form.reset();
    });
}

function showStep(stepNumber) {
    // Hide all steps with animation
    document.querySelectorAll('.auth-step').forEach(step => {
        step.style.opacity = '0';
        step.style.transform = 'translateX(-20px)';
        setTimeout(() => {
            step.classList.remove('active');
        }, 150);
    });
    
    // Show target step with animation
    setTimeout(() => {
        const targetStep = document.getElementById(`step${stepNumber}`);
        targetStep.classList.add('active');
        targetStep.style.opacity = '1';
        targetStep.style.transform = 'translateX(0)';
        targetStep.style.transition = 'all 0.3s ease';
        currentStep = stepNumber;
    }, 150);
}

// Social Login Functions
async function loginWithGoogle() {
    showNotification('در حال اتصال به گوگل...', 'info');
    // Simulate OAuth flow
    setTimeout(() => {
        showNotification('ورود با گوگل موفقیت‌آمیز بود!', 'success');
        closeLoginModal();
        updateUIForLoggedInUser('کاربر گوگل');
    }, 2000);
}

async function loginWithGithub() {
    showNotification('در حال اتصال به گیت‌هاب...', 'info');
    // Simulate OAuth flow
    setTimeout(() => {
        showNotification('ورود با گیت‌هاب موفقیت‌آمیز بود!', 'success');
        closeLoginModal();
        updateUIForLoggedInUser('کاربر گیت‌هاب');
    }, 2000);
}

async function loginWithLinkedin() {
    showNotification('در حال اتصال به لینکدین...', 'info');
    // Simulate OAuth flow
    setTimeout(() => {
        showNotification('ورود با لینکدین موفقیت‌آمیز بود!', 'success');
        closeLoginModal();
        updateUIForLoggedInUser('کاربر لینکدین');
    }, 2000);
}

async function loginWithMeta() {
    showNotification('در حال اتصال به متا...', 'info');
    // Simulate OAuth flow
    setTimeout(() => {
        showNotification('ورود با متا موفقیت‌آمیز بود!', 'success');
        closeLoginModal();
        updateUIForLoggedInUser('کاربر متا');
    }, 2000);
}

// Authentication handlers
async function handleCheckUser(e) {
    e.preventDefault();
    
    const emailOrPhone = document.getElementById('emailOrPhone').value;
    const submitBtn = e.target.querySelector('button[type="submit"]');
    
    // Add loading state
    submitBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> در حال بررسی...';
    submitBtn.disabled = true;
    
    try {
        const response = await fetch('/api/auth/check', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                email_or_phone: emailOrPhone
            })
        });
        
        const data = await response.json();
        
        if (data.exists) {
            // User exists, show password input
            document.getElementById('userMessage').textContent = data.message;
            showStep(2);
        } else {
            // New user, show verification code input
            document.getElementById('codeMessage').textContent = data.message;
            showStep(3);
        }
    } catch (error) {
        console.error('Error checking user:', error);
        showNotification('خطا در بررسی کاربر', 'error');
    } finally {
        // Reset button
        submitBtn.innerHTML = '<span>ادامه</span><i class="fas fa-arrow-left"></i>';
        submitBtn.disabled = false;
    }
}

async function handleLogin(e) {
    e.preventDefault();
    
    const emailOrPhone = document.getElementById('emailOrPhone').value;
    const password = document.getElementById('password').value;
    const submitBtn = e.target.querySelector('button[type="submit"]');
    
    // Add loading state
    submitBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> در حال ورود...';
    submitBtn.disabled = true;
    
    try {
        const response = await fetch('/api/auth/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                email_or_phone: emailOrPhone,
                password: password
            })
        });
        
        const data = await response.json();
        
        if (data.success) {
            showNotification(data.message, 'success');
            closeLoginModal();
            updateUIForLoggedInUser();
        } else {
            showNotification(data.message, 'error');
        }
    } catch (error) {
        console.error('Error logging in:', error);
        showNotification('خطا در ورود', 'error');
    } finally {
        // Reset button
        submitBtn.innerHTML = '<span>ورود</span><i class="fas fa-sign-in-alt"></i>';
        submitBtn.disabled = false;
    }
}

async function handleVerifyCode(e) {
    e.preventDefault();
    
    const emailOrPhone = document.getElementById('emailOrPhone').value;
    const code = document.getElementById('verificationCode').value;
    const submitBtn = e.target.querySelector('button[type="submit"]');
    
    // Add loading state
    submitBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> در حال تایید...';
    submitBtn.disabled = true;
    
    try {
        const response = await fetch('/api/auth/verify', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                email_or_phone: emailOrPhone,
                code: code
            })
        });
        
        const data = await response.json();
        
        if (data.success) {
            showNotification(data.message, 'success');
            // Pre-fill registration form
            document.getElementById('email').value = emailOrPhone.includes('@') ? emailOrPhone : '';
            document.getElementById('phone').value = emailOrPhone.includes('@') ? '' : emailOrPhone;
            showStep(4);
        } else {
            showNotification(data.message, 'error');
        }
    } catch (error) {
        console.error('Error verifying code:', error);
        showNotification('خطا در تایید کد', 'error');
    } finally {
        // Reset button
        submitBtn.innerHTML = '<span>تایید</span><i class="fas fa-check"></i>';
        submitBtn.disabled = false;
    }
}

async function handleRegister(e) {
    e.preventDefault();
    
    const username = document.getElementById('username').value;
    const email = document.getElementById('email').value;
    const phone = document.getElementById('phone').value;
    const code = document.getElementById('verificationCode').value;
    const submitBtn = e.target.querySelector('button[type="submit"]');
    
    // Add loading state
    submitBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> در حال ثبت‌نام...';
    submitBtn.disabled = true;
    
    try {
        const response = await fetch('/api/auth/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                username: username,
                email: email,
                phone: phone,
                code: code
            })
        });
        
        const data = await response.json();
        
        if (data.success) {
            showNotification(data.message, 'success');
            closeLoginModal();
            updateUIForLoggedInUser(username);
        } else {
            showNotification(data.message, 'error');
        }
    } catch (error) {
        console.error('Error registering:', error);
        showNotification('خطا در ثبت‌نام', 'error');
    } finally {
        // Reset button
        submitBtn.innerHTML = '<span>ثبت‌نام</span><i class="fas fa-user-plus"></i>';
        submitBtn.disabled = false;
    }
}

function forgotPassword() {
    showNotification('لینک بازیابی رمز عبور به ایمیل شما ارسال شد', 'info');
    // In a real app, this would trigger a password reset email
}

// Product loading with enhanced animations
async function loadProducts() {
    try {
        const response = await fetch('/api/products');
        const data = await response.json();
        
        // Simulate loading delay for better UX
        setTimeout(() => {
            displayProducts(data.items || []);
        }, 1000);
    } catch (error) {
        console.error('Error loading products:', error);
        setTimeout(() => {
            displayProducts([]); // Show empty state
        }, 1000);
    }
}

function displayProducts(products) {
    const productsGrid = document.getElementById('products-grid');
    
    if (products.length === 0) {
        productsGrid.innerHTML = `
            <div class="loading">
                <i class="fas fa-exclamation-triangle" style="font-size: 2rem; margin-bottom: 1rem;"></i>
                <span>محصولی یافت نشد</span>
            </div>
        `;
        return;
    }
    
    productsGrid.innerHTML = products.map((product, index) => `
        <div class="product-card" style="animation-delay: ${index * 0.1}s">
            <h3>${product.name}</h3>
            <p>${product.description}</p>
            <div class="product-price">${formatPrice(product.price)} تومان</div>
            <p style="color: ${product.stock_quantity > 0 ? '#64ffda' : '#ff6b6b'};">
                ${product.stock_quantity > 0 ? `موجودی: ${product.stock_quantity}` : 'ناموجود'}
            </p>
            <button class="cta-btn primary" style="margin-top: 1rem; width: 100%;" 
                    onclick="addToCart('${product.id}')"
                    ${product.stock_quantity === 0 ? 'disabled' : ''}>
                <i class="fas fa-shopping-cart"></i>
                ${product.stock_quantity > 0 ? 'افزودن به سبد' : 'ناموجود'}
            </button>
        </div>
    `).join('');
    
    // Trigger scroll animations for new products
    setTimeout(() => {
        handleScrollAnimations();
    }, 100);
}

function addToCart(productId) {
    showNotification('محصول به سبد خرید اضافه شد', 'success');
    // In a real app, this would add the product to cart
}

// Scroll animations
function initializeScrollAnimations() {
    const animatedElements = document.querySelectorAll('.product-card, .feature-card, .contact-card, .about-card');
    
    animatedElements.forEach(el => {
        el.style.opacity = '0';
        el.style.transform = 'translateY(50px)';
        el.style.transition = 'opacity 0.8s ease, transform 0.8s ease';
    });
    
    window.addEventListener('scroll', handleScrollAnimations);
    handleScrollAnimations(); // Initial check
}

function isElementInViewport(el) {
    const rect = el.getBoundingClientRect();
    return (
        rect.top >= 0 &&
        rect.left >= 0 &&
        rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) + 100 &&
        rect.right <= (window.innerWidth || document.documentElement.clientWidth)
    );
}

function handleScrollAnimations() {
    const animatedElements = document.querySelectorAll('.product-card, .feature-card, .contact-card, .about-card');
    
    animatedElements.forEach((el, index) => {
        if (isElementInViewport(el)) {
            setTimeout(() => {
                el.style.opacity = '1';
                el.style.transform = 'translateY(0)';
            }, index * 100);
        }
    });
}

// Utility functions
function formatPrice(price) {
    return new Intl.NumberFormat('fa-IR').format(price);
}

function updateUIForLoggedInUser(username = 'کاربر') {
    const loginBtn = document.querySelector('.login-btn');
    if (loginBtn) {
        loginBtn.innerHTML = `
            <i class="fas fa-user-circle"></i>
            ${username}
        `;
        loginBtn.onclick = function() {
            showUserMenu();
        };
    }
    currentUser = username;
}

function showUserMenu() {
    // Create a simple user menu
    const menu = document.createElement('div');
    menu.className = 'user-menu';
    menu.innerHTML = `
        <div class="user-menu-content">
            <div class="user-menu-item" onclick="showNotification('صفحه پروفایل در حال توسعه است', 'info')">
                <i class="fas fa-user"></i> پروفایل
            </div>
            <div class="user-menu-item" onclick="showNotification('صفحه سفارشات در حال توسعه است', 'info')">
                <i class="fas fa-shopping-bag"></i> سفارشات
            </div>
            <div class="user-menu-item" onclick="logout()">
                <i class="fas fa-sign-out-alt"></i> خروج
            </div>
        </div>
    `;
    
    // Add styles
    menu.style.cssText = `
        position: fixed;
        top: 70px;
        right: 20px;
        background: rgba(15, 15, 35, 0.95);
        backdrop-filter: blur(20px);
        border: 1px solid rgba(100, 255, 218, 0.2);
        border-radius: 15px;
        padding: 1rem;
        z-index: 2000;
        min-width: 200px;
    `;
    
    document.body.appendChild(menu);
    
    // Remove menu when clicking outside
    setTimeout(() => {
        document.addEventListener('click', function removeMenu() {
            if (menu.parentNode) {
                menu.parentNode.removeChild(menu);
            }
            document.removeEventListener('click', removeMenu);
        });
    }, 100);
}

function logout() {
    currentUser = null;
    const loginBtn = document.querySelector('.login-btn');
    if (loginBtn) {
        loginBtn.innerHTML = `
            <i class="fas fa-user"></i>
            ورود
        `;
        loginBtn.onclick = openLoginModal;
    }
    showNotification('با موفقیت خارج شدید', 'success');
}

function showNotification(message, type = 'info') {
    // Create notification element
    const notification = document.createElement('div');
    notification.className = `notification notification-${type}`;
    
    const icon = type === 'success' ? 'check-circle' : 
                 type === 'error' ? 'exclamation-circle' : 
                 'info-circle';
    
    notification.innerHTML = `
        <i class="fas fa-${icon}"></i>
        <span>${message}</span>
    `;
    
    // Style the notification
    notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        background: ${type === 'success' ? 'linear-gradient(135deg, #4CAF50, #45a049)' : 
                     type === 'error' ? 'linear-gradient(135deg, #f44336, #d32f2f)' : 
                     'linear-gradient(135deg, #2196F3, #1976D2)'};
        color: white;
        padding: 15px 20px;
        border-radius: 12px;
        z-index: 3000;
        box-shadow: 0 8px 32px rgba(0,0,0,0.3);
        transform: translateX(100%);
        transition: transform 0.4s ease;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-weight: 500;
        backdrop-filter: blur(10px);
        border: 1px solid rgba(255,255,255,0.2);
    `;
    
    document.body.appendChild(notification);
    
    // Animate in
    setTimeout(() => {
        notification.style.transform = 'translateX(0)';
    }, 100);
    
    // Remove after 4 seconds
    setTimeout(() => {
        notification.style.transform = 'translateX(100%)';
        setTimeout(() => {
            if (notification.parentNode) {
                notification.parentNode.removeChild(notification);
            }
        }, 400);
    }, 4000);
}

// Add CSS for user menu items
const userMenuStyles = document.createElement('style');
userMenuStyles.textContent = `
    .user-menu-item {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        padding: 0.75rem;
        color: rgba(255, 255, 255, 0.8);
        cursor: pointer;
        border-radius: 8px;
        transition: all 0.3s ease;
        margin-bottom: 0.5rem;
    }
    
    .user-menu-item:hover {
        background: rgba(100, 255, 218, 0.1);
        color: #64ffda;
    }
    
    .user-menu-item:last-child {
        margin-bottom: 0;
        border-top: 1px solid rgba(255, 255, 255, 0.1);
        margin-top: 0.5rem;
        padding-top: 1rem;
    }
`;
document.head.appendChild(userMenuStyles);
