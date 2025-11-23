# Deployment Guide: GoDaddy + Cloudflare

This guide will help you deploy BestPizzasInCalifornia.com using:
- **GoDaddy**: Domain registrar
- **Cloudflare Pages**: Frontend hosting (your Dioxus WASM app)
- **Cloudflare Workers**: Backend/API (if needed)

---

## 📦 Step 1: Build for Production

First, build your optimized production site:

```bash
dx build --release --platform web
```

Your built files will be in:
```
target/dx/app/release/web/public/
```

This folder contains:
- `index.html`
- `wasm-bindgen/` (WASM files)
- `assets/` (CSS, images)

---

## ☁️ Step 2: Deploy to Cloudflare Pages

### Option A: Deploy via Git (Recommended)

1. **Create GitHub Repository**
   ```bash
   git init
   git add .
   git commit -m "Initial commit - BestPizzasInCalifornia.com"
   git branch -M main
   git remote add origin https://github.com/YOUR_USERNAME/bestpizzasincalifornia.git
   git push -u origin main
   ```

2. **Connect to Cloudflare Pages**
   - Go to [Cloudflare Dashboard](https://dash.cloudflare.com)
   - Click **Pages** → **Create a project**
   - Click **Connect to Git**
   - Select your GitHub repository
   - Configure build settings:
     - **Framework preset**: None
     - **Build command**: `dx build --release --platform web`
     - **Build output directory**: `target/dx/app/release/web/public`
   - Click **Save and Deploy**

3. **Wait for Build**
   - Cloudflare will build and deploy automatically
   - You'll get a URL like: `bestpizzasincalifornia.pages.dev`

### Option B: Deploy via CLI (Direct Upload)

1. **Install Wrangler (Cloudflare CLI)**
   ```bash
   npm install -g wrangler
   ```

2. **Login to Cloudflare**
   ```bash
   wrangler login
   ```

3. **Deploy the site**
   ```bash
   cd target/dx/app/release/web/public
   wrangler pages deploy . --project-name=bestpizzasincalifornia
   ```

---

## 🌐 Step 3: Connect GoDaddy Domain to Cloudflare

### A. Add Domain to Cloudflare

1. **Go to Cloudflare Dashboard**
   - Click **Add a Site**
   - Enter: `bestpizzasincalifornia.com`
   - Choose **Free Plan**

2. **Cloudflare will scan your DNS records**
   - Review and click **Continue**

3. **Get Cloudflare Nameservers**
   - Cloudflare will show you 2 nameservers like:
     ```
     lola.ns.cloudflare.com
     tim.ns.cloudflare.com
     ```
   - **Copy these!**

### B. Update Nameservers in GoDaddy

1. **Login to GoDaddy**
   - Go to [GoDaddy Domain Manager](https://dcc.godaddy.com/manage/)
   - Find `bestpizzasincalifornia.com`

2. **Change Nameservers**
   - Click on your domain
   - Scroll to **Nameservers**
   - Click **Change**
   - Select **I'll use my own nameservers**
   - Enter the 2 Cloudflare nameservers
   - Click **Save**

3. **Wait for Propagation** (2-48 hours, usually < 1 hour)

### C. Configure DNS in Cloudflare

1. **Go to Cloudflare DNS Settings**
   - Click **DNS** in the dashboard

2. **Add CNAME Record for Cloudflare Pages**
   - **Type**: CNAME
   - **Name**: `@` (for root domain)
   - **Target**: `bestpizzasincalifornia.pages.dev`
   - **Proxy status**: Proxied (orange cloud) ✓
   - Click **Save**

3. **Add CNAME for www (optional)**
   - **Type**: CNAME
   - **Name**: `www`
   - **Target**: `bestpizzasincalifornia.pages.dev`
   - **Proxy status**: Proxied ✓
   - Click **Save**

---

## 🔧 Step 4: Configure Cloudflare Pages Custom Domain

1. **In Cloudflare Pages Dashboard**
   - Go to your project: `bestpizzasincalifornia`
   - Click **Custom domains**
   - Click **Set up a custom domain**

2. **Add Your Domains**
   - Add: `bestpizzasincalifornia.com`
   - Add: `www.bestpizzasincalifornia.com`
   - Cloudflare will automatically configure everything

3. **Enable HTTPS**
   - Cloudflare automatically provisions SSL certificates
   - Your site will be available at:
     - `https://bestpizzasincalifornia.com`
     - `https://www.bestpizzasincalifornia.com`

---

## 🚀 Step 5: Optimize Cloudflare Settings

### A. Speed Optimizations

1. **Go to Speed → Optimization**
   - ✅ Enable **Auto Minify** (JavaScript, CSS, HTML)
   - ✅ Enable **Brotli**
   - ✅ Enable **Rocket Loader**

2. **Caching**
   - Go to **Caching** → **Configuration**
   - Set **Browser Cache TTL**: 4 hours
   - ✅ Enable **Always Online**

### B. Security

1. **SSL/TLS**
   - Go to **SSL/TLS** → **Overview**
   - Set to **Full (strict)**

2. **Security Level**
   - Go to **Security** → **Settings**
   - Set to **Medium** or **High**

---

## 🔄 Step 6: Set Up Automatic Deployments

With Git integration, every time you push to GitHub:

```bash
# Make changes to your site
git add .
git commit -m "Update restaurant menu"
git push

# Cloudflare automatically rebuilds and deploys!
```

---

## 📊 Step 7: Verify AdSense Works

1. **Test on Live Domain**
   - Visit `https://bestpizzasincalifornia.com`
   - Open browser DevTools (F12)
   - Check Console for AdSense script loading

2. **Update AdSense**
   - Go to [AdSense Dashboard](https://adsense.google.com)
   - Add your domain: `bestpizzasincalifornia.com`
   - Wait up to 24 hours for ads to start showing

3. **Add ads.txt File**
   - Create `ads.txt` in your project root:
     ```
     google.com, pub-5369604889706863, DIRECT, f08c47fec0942fa0
     ```
   - Commit and push to deploy

---

## 🔧 Optional: Cloudflare Workers for Backend

If you need backend functionality (API, database, etc.):

### Create a Worker

1. **Create `wrangler.toml`** in project root:
   ```toml
   name = "bestpizzas-api"
   type = "javascript"
   account_id = "YOUR_ACCOUNT_ID"
   workers_dev = true
   route = "https://bestpizzasincalifornia.com/api/*"
   ```

2. **Create `worker.js`**:
   ```javascript
   export default {
     async fetch(request, env) {
       // Example API endpoint
       if (request.url.includes('/api/restaurants')) {
         return new Response(JSON.stringify({
           restaurants: [
             { name: 'Pitfire Pizza', locations: 9 },
             { name: 'Ciao Pasta', city: 'San Juan Capistrano' },
             { name: 'Mr Motto', city: 'San Diego' }
           ]
         }), {
           headers: { 'Content-Type': 'application/json' }
         });
       }

       return new Response('Not Found', { status: 404 });
     }
   };
   ```

3. **Deploy Worker**:
   ```bash
   wrangler deploy
   ```

---

## ✅ Final Checklist

- [ ] Build production site: `dx build --release`
- [ ] Deploy to Cloudflare Pages
- [ ] Update GoDaddy nameservers to Cloudflare
- [ ] Configure custom domain in Cloudflare Pages
- [ ] Test site at `https://bestpizzasincalifornia.com`
- [ ] Verify AdSense is loading
- [ ] Add `ads.txt` file
- [ ] Enable Cloudflare optimizations
- [ ] Set up automatic deployments via Git

---

## 📞 Troubleshooting

### Site not loading?
- Check nameserver propagation: https://www.whatsmydns.net/
- Verify DNS records in Cloudflare
- Check Cloudflare Pages deployment logs

### Ads not showing?
- Wait 24-48 hours after domain change
- Verify `ads.txt` file is accessible
- Check AdSense account status
- Ensure script is loading (F12 DevTools)

### Build failing?
- Check build logs in Cloudflare Pages
- Verify build command is correct
- Ensure all dependencies are in `Cargo.toml`

---

## 🎯 Quick Deploy Commands

```bash
# Build production
dx build --release --platform web

# Deploy via Wrangler
cd target/dx/app/release/web/public
wrangler pages deploy . --project-name=bestpizzasincalifornia

# Or commit and push (if using Git)
git add .
git commit -m "Deploy to production"
git push
```

---

## 💰 Costs

- **GoDaddy Domain**: ~$12-15/year
- **Cloudflare Pages**: **FREE** (up to 500 builds/month)
- **Cloudflare Workers**: **FREE** (up to 100k requests/day)
- **Cloudflare CDN**: **FREE**
- **SSL Certificate**: **FREE** (auto-provisioned)

**Total**: Just the domain cost! (~$15/year)

---

**Your site will be blazing fast with Cloudflare's global CDN and ready to earn AdSense revenue!** 🚀🍕
