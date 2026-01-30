// ============================================
// Lotus OSINT Platform - Frontend App
// ============================================

const API_BASE = '/api';
let currentScanId = null;
let scanPollInterval = null;

// ============================================
// Page Navigation
// ============================================

document.querySelectorAll('.nav-item').forEach(item => {
    item.addEventListener('click', (e) => {
        e.preventDefault();
        const page = item.dataset.page;
        navigateTo(page);
    });
});

function navigateTo(page) {
    // Update nav
    document.querySelectorAll('.nav-item').forEach(nav => {
        nav.classList.toggle('active', nav.dataset.page === page);
    });
    
    // Update pages
    document.querySelectorAll('.page').forEach(p => {
        p.classList.toggle('active', p.id === `page-${page}`);
    });
    
    // Update title
    const titles = {
        dashboard: '// DASHBOARD',
        scan: '// NEW SCAN',
        results: '// SCAN RESULTS',
        secrets: '// API KEYS',
        tools: '// TOOLS',
        scripts: '// SCRIPTS'
    };
    document.getElementById('page-title').textContent = titles[page] || page.toUpperCase();
    
    // Load page data
    if (page === 'secrets') loadSecrets();
    if (page === 'results') loadResults();
    if (page === 'tools') loadTools();
    if (page === 'scripts') loadScripts();
    if (page === 'scan') loadScriptsForForm();
    if (page === 'dashboard') loadDashboard();
}

// ============================================
// Dashboard
// ============================================

async function loadDashboard() {
    try {
        const [secretsRes, resultsRes] = await Promise.all([
            fetch(`${API_BASE}/secrets`),
            fetch(`${API_BASE}/results`)
        ]);
        
        const secrets = await secretsRes.json();
        const results = await resultsRes.json();
        
        document.getElementById('stat-keys').textContent = secrets.total_configured || 0;
        document.getElementById('stat-scans').textContent = results.total || 0;
        document.getElementById('stat-findings').textContent = 
            results.results?.reduce((acc, r) => acc + (r.findings?.length || 0), 0) || 0;
        
        // Update recent activity
        const activityEl = document.getElementById('recent-activity');
        if (results.results && results.results.length > 0) {
            activityEl.innerHTML = results.results.slice(0, 5).map(r => `
                <div class="activity-item">
                    <div class="activity-icon">🔍</div>
                    <div class="activity-content">
                        <div class="activity-title">${r.target}</div>
                        <div class="activity-time">${r.status} - ${new Date(r.started_at).toLocaleString()}</div>
                    </div>
                </div>
            `).join('');
        }
    } catch (err) {
        console.error('Failed to load dashboard:', err);
    }
}

// ============================================
// Quick Scan
// ============================================

async function startQuickScan() {
    const target = document.getElementById('quick-target').value.trim();
    if (!target) {
        showToast('Enter a target domain or URL', 'error');
        return;
    }
    
    try {
        const res = await fetch(`${API_BASE}/scan`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ target, scan_type: 'osint' })
        });
        
        const data = await res.json();
        if (data.success) {
            showToast(`Scan started: ${data.scan_id}`, 'success');
            currentScanId = data.scan_id;
            navigateTo('scan');
            showActiveScan(target);
            startPolling();
        } else {
            showToast('Failed to start scan', 'error');
        }
    } catch (err) {
        showToast('Error starting scan', 'error');
    }
}

// ============================================
// Scan Management
// ============================================

async function startScan(e) {
    e.preventDefault();
    
    const target = document.getElementById('scan-target').value.trim();
    const scanType = document.getElementById('scan-type').value;
    const script = document.getElementById('scan-script').value;
    
    if (!target) {
        showToast('Enter a target', 'error');
        return;
    }
    
    try {
        const res = await fetch(`${API_BASE}/scan`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ target, scan_type: scanType, script: script || null })
        });
        
        const data = await res.json();
        if (data.success) {
            showToast(`Scan initiated: ${data.scan_id}`, 'success');
            currentScanId = data.scan_id;
            showActiveScan(target);
            startPolling();
        }
    } catch (err) {
        showToast('Failed to start scan', 'error');
    }
}

function showActiveScan(target) {
    document.getElementById('active-scan-card').style.display = 'block';
    document.getElementById('scan-target-display').textContent = target;
    document.getElementById('scan-progress-bar').style.width = '0%';
    document.getElementById('scan-progress-text').textContent = '0%';
    document.getElementById('scan-status-message').textContent = 'Initializing scan...';
}

function startPolling() {
    if (scanPollInterval) clearInterval(scanPollInterval);
    
    scanPollInterval = setInterval(async () => {
        if (!currentScanId) return;
        
        try {
            const res = await fetch(`${API_BASE}/scan/${currentScanId}`);
            const data = await res.json();
            
            if (data.found && data.scan) {
                const scan = data.scan;
                document.getElementById('scan-progress-bar').style.width = `${scan.progress}%`;
                document.getElementById('scan-progress-text').textContent = `${scan.progress}%`;
                document.getElementById('scan-status-message').textContent = scan.message;
                
                if (scan.status === 'completed' || scan.status === 'failed' || scan.status === 'stopped') {
                    clearInterval(scanPollInterval);
                    showToast(`Scan ${scan.status}!`, scan.status === 'completed' ? 'success' : 'info');
                    setTimeout(() => {
                        document.getElementById('active-scan-card').style.display = 'none';
                        loadResults();
                    }, 2000);
                }
            }
        } catch (err) {
            console.error('Polling error:', err);
        }
    }, 1000);
}

async function stopCurrentScan() {
    if (!currentScanId) return;
    
    try {
        await fetch(`${API_BASE}/scan/${currentScanId}/stop`, { method: 'POST' });
        showToast('Scan stopped', 'info');
    } catch (err) {
        showToast('Failed to stop scan', 'error');
    }
}

// ============================================
// Results
// ============================================

async function loadResults() {
    try {
        const res = await fetch(`${API_BASE}/results`);
        const data = await res.json();
        
        const container = document.getElementById('results-list');
        
        if (!data.results || data.results.length === 0) {
            container.innerHTML = '<div class="empty-state">NO SCAN RESULTS YET. INITIATE A SCAN TO BEGIN.</div>';
            return;
        }
        
        container.innerHTML = data.results.map(r => `
            <div class="result-card">
                <div class="result-card-header">
                    <span class="result-target">${r.target}</span>
                    <span class="result-status ${r.status}">${r.status}</span>
                </div>
                <div class="result-meta">
                    Script: ${r.script}<br>
                    Started: ${new Date(r.started_at).toLocaleString()}
                </div>
                <div class="result-findings">
                    <span class="finding-count">${r.findings?.length || 0}</span> findings
                </div>
            </div>
        `).join('');
    } catch (err) {
        console.error('Failed to load results:', err);
    }
}

function refreshResults() {
    loadResults();
    showToast('Results refreshed', 'info');
}

// ============================================
// Secrets Management
// ============================================

async function loadSecrets() {
    try {
        const res = await fetch(`${API_BASE}/secrets`);
        const data = await res.json();
        
        const container = document.getElementById('secrets-list');
        container.innerHTML = data.secrets.map(s => `
            <div class="secret-card">
                <div class="secret-info">
                    <div class="secret-icon">🔑</div>
                    <div>
                        <div class="secret-name">${s.display_name}</div>
                        <div class="secret-value">${s.masked_value}</div>
                    </div>
                </div>
                <span class="secret-status ${s.configured ? 'configured' : 'missing'}">
                    ${s.configured ? 'ACTIVE' : 'MISSING'}
                </span>
            </div>
        `).join('');
    } catch (err) {
        console.error('Failed to load secrets:', err);
    }
}

async function addSecret(e) {
    e.preventDefault();
    
    const key = document.getElementById('secret-key').value;
    const value = document.getElementById('secret-value').value;
    
    try {
        const res = await fetch(`${API_BASE}/secrets`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ key, value })
        });
        
        const data = await res.json();
        if (data.success) {
            showToast('API key saved!', 'success');
            document.getElementById('secret-value').value = '';
            loadSecrets();
        }
    } catch (err) {
        showToast('Failed to save API key', 'error');
    }
}

// ============================================
// Tools
// ============================================

async function loadTools() {
    try {
        const res = await fetch(`${API_BASE}/tools`);
        const data = await res.json();
        
        const container = document.getElementById('tools-list');
        container.innerHTML = data.tools.map(t => `
            <div class="tool-card">
                <div class="tool-card-header">
                    <div class="tool-icon">🛠️</div>
                    <div>
                        <div class="tool-name">${t.name}</div>
                        <div class="tool-category">${t.category}</div>
                    </div>
                </div>
                <div class="tool-desc">${t.description}</div>
                <div class="tool-install">${t.install}</div>
            </div>
        `).join('');
    } catch (err) {
        console.error('Failed to load tools:', err);
    }
}

// ============================================
// Scripts
// ============================================

async function loadScripts() {
    try {
        const res = await fetch(`${API_BASE}/scripts`);
        const data = await res.json();
        
        const container = document.getElementById('scripts-list');
        container.innerHTML = data.scripts.map(s => `
            <div class="script-card">
                <div class="script-card-header">
                    <div class="script-icon">📜</div>
                    <div>
                        <div class="script-name">${s.name}</div>
                        <div class="script-category">${s.category}</div>
                    </div>
                </div>
                <div class="script-desc">${s.description}</div>
                <div class="script-tools">
                    ${s.tools.map(t => `<span class="script-tool-tag">${t}</span>`).join('')}
                </div>
            </div>
        `).join('');
    } catch (err) {
        console.error('Failed to load scripts:', err);
    }
}

async function loadScriptsForForm() {
    try {
        const res = await fetch(`${API_BASE}/scripts`);
        const data = await res.json();
        
        const select = document.getElementById('scan-script');
        select.innerHTML = '<option value="">Auto-select based on scan type</option>' +
            data.scripts.map(s => `<option value="${s.path}">${s.name} - ${s.description}</option>`).join('');
    } catch (err) {
        console.error('Failed to load scripts for form:', err);
    }
}

// ============================================
// Toast Notifications
// ============================================

function showToast(message, type = 'info') {
    const container = document.getElementById('toast-container');
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    toast.textContent = message;
    container.appendChild(toast);
    
    setTimeout(() => {
        toast.style.animation = 'slideIn 0.3s ease reverse';
        setTimeout(() => toast.remove(), 300);
    }, 3000);
}

// ============================================
// Initialize
// ============================================

document.addEventListener('DOMContentLoaded', () => {
    loadDashboard();
    
    // Retro startup effect
    console.log('%c🪷 LOTUS OSINT PLATFORM INITIALIZED', 
        'color: #ff71ce; font-size: 20px; font-weight: bold; text-shadow: 0 0 10px #ff71ce;');
    console.log('%cSystem Status: ONLINE', 
        'color: #01cdfe; font-size: 14px;');
});
