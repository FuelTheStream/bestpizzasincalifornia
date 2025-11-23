# Google AdSense Integration Guide

Your BestPizzasInCalifornia.com website now has Google AdSense fully integrated! Follow this guide to activate real ads.

## 📋 Overview

The website has **4 AdSense placements**:
1. **Top Banner** - Below the header
2. **In-Feed Ad** - Between restaurants 1 and 2
3. **Middle Banner** - Between restaurants 2 and 3
4. **Bottom Banner** - After the last restaurant

## 🚀 Step-by-Step Setup

### Step 1: Sign Up for Google AdSense

1. Go to [https://www.google.com/adsense](https://www.google.com/adsense)
2. Click **"Get Started"**
3. Sign in with your Google account
4. Enter your website URL: `bestpizzasincalifornia.com` (or your actual domain)
5. Complete the application form
6. Wait for approval (typically 24-48 hours)

### Step 2: Get Your Publisher ID

Once approved:

1. Log into your AdSense account
2. Go to **Account → Account Information**
3. Find your **Publisher ID** (format: `ca-pub-1234567890123456`)
4. Copy this ID

### Step 3: Create Ad Units

Create 4 separate ad units for optimal tracking:

1. In AdSense, go to **Ads → By ad unit**
2. Click **+ New ad unit**
3. Create these 4 ad units:

   **Ad Unit 1: Top Banner**
   - Name: `Pizza-Top-Banner`
   - Type: Display ads
   - Size: Responsive
   - Copy the **Ad Slot ID** (10-digit number)

   **Ad Unit 2: In-Feed**
   - Name: `Pizza-InFeed`
   - Type: In-feed ads
   - Size: Responsive
   - Copy the **Ad Slot ID**

   **Ad Unit 3: Middle Banner**
   - Name: `Pizza-Middle-Banner`
   - Type: Display ads
   - Size: Responsive
   - Copy the **Ad Slot ID**

   **Ad Unit 4: Bottom Banner**
   - Name: `Pizza-Bottom-Banner`
   - Type: Display ads
   - Size: Responsive
   - Copy the **Ad Slot ID**

### Step 4: Update Your Code

Open `src/main.rs` and replace the placeholder values:

#### 1. Update Publisher ID (appears twice):

**Line ~27**: Replace in the script tag
```rust
document::Script {
    src: "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-YOUR_PUBLISHER_ID",
    //                                                                           ^^^^^^^^^^^^^^^^^^^
    //                                                                           Replace this!
    crossorigin: "anonymous",
    async: true
}
```

**Line ~199**: Replace in the AdSense component
```rust
"data-ad-client": "ca-pub-YOUR_PUBLISHER_ID",
//                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
//                 Replace this!
```

#### 2. Update Ad Slot IDs:

Find these lines and replace with your actual ad slot IDs:

**Top Banner (Line ~40):**
```rust
AdSense {
    banner_type: "banner",
    ad_slot: "YOUR_TOP_BANNER_SLOT_ID"  // Replace with actual slot ID
}
```

**In-Feed (Line ~65):**
```rust
AdSense {
    banner_type: "infeed",
    ad_slot: "YOUR_INFEED_SLOT_ID"  // Replace with actual slot ID
}
```

**Middle Banner (Line ~90):**
```rust
AdSense {
    banner_type: "banner",
    ad_slot: "YOUR_MIDDLE_BANNER_SLOT_ID"  // Replace with actual slot ID
}
```

**Bottom Banner (Line ~115):**
```rust
AdSense {
    banner_type: "banner",
    ad_slot: "YOUR_BOTTOM_BANNER_SLOT_ID"  // Replace with actual slot ID
}
```

### Step 5: Build and Deploy

1. **Build the project:**
   ```bash
   dx build --release
   ```

2. **Test locally first:**
   ```bash
   dx serve
   ```

   Note: Ads may show as blank or test ads in development. This is normal!

3. **Deploy to your domain:**
   - Upload the contents of `target/dx/app/release/web/public/` to your web server
   - Ensure your domain is added to AdSense

### Step 6: Verify Ad Implementation

1. In AdSense, go to **Sites → Your site**
2. Click **"Check ads.txt"**
3. If needed, create an `ads.txt` file in your website root:
   ```
   google.com, pub-XXXXXXXXXXXXXXXXX, DIRECT, f08c47fec0942fa0
   ```
   Replace `pub-XXXXXXXXXXXXXXXXX` with your publisher ID (without `ca-` prefix)

## 💡 Important Notes

### Ad Review Process
- New ads may take **up to 24 hours** to start showing
- Initially, you might see **blank spaces** - this is normal during review
- Test ads or PSAs (public service announcements) may show first

### AdSense Policies
Ensure your site complies with [AdSense Program Policies](https://support.google.com/adsense/answer/48182):
- ✅ Original content
- ✅ User-friendly navigation
- ✅ Appropriate content (food/restaurants = perfect!)
- ❌ No prohibited content
- ❌ No invalid click activity

### Optimization Tips

1. **Ad Placement:**
   - Current placements are optimized for user experience
   - Don't add more than 3 ads above the fold

2. **Performance:**
   - AdSense script loads asynchronously (won't slow down your site)
   - Lazy loading is automatic

3. **Revenue:**
   - Monitor performance in AdSense dashboard
   - Adjust ad types based on performance data
   - Consider enabling Auto ads after gathering data

## 🔧 Troubleshooting

### Ads Not Showing?

1. **Check Browser Console** (F12):
   - Look for AdSense-related errors
   - Ensure scripts are loading

2. **Verify AdSense Account:**
   - Account must be approved
   - Site must be added and verified
   - Payment information completed

3. **Check Ad Blockers:**
   - Disable ad blockers for testing
   - Many users have ad blockers (this is normal)

4. **Wait 24-48 Hours:**
   - New sites/accounts need time to activate

### Common Issues

**"AdSense account not approved"**
- Wait for email confirmation
- Check spam folder
- Ensure site meets quality guidelines

**"Blank ad spaces"**
- Normal during initial review period
- Check that publisher ID and slot IDs are correct
- Verify ads.txt file

**"Ads showing but no clicks"**
- Never click your own ads (AdSense violation!)
- Build traffic through SEO and marketing
- Consider pizza blogger partnerships

## 📊 Tracking Revenue

1. Go to AdSense dashboard
2. View **Reports** section
3. Track:
   - Page views
   - Ad impressions
   - Click-through rate (CTR)
   - Earnings per 1000 impressions (RPM)
   - Total revenue

## 🎯 Next Steps

After AdSense is working:

1. **Add Google Analytics** to track user behavior
2. **Optimize SEO** to increase traffic
3. **Add more restaurants** to increase page views
4. **Create blog content** about California pizza
5. **Build social media presence**

## 📞 Support

- **AdSense Help:** [support.google.com/adsense](https://support.google.com/adsense)
- **AdSense Community:** [support.google.com/adsense/community](https://support.google.com/adsense/community)

---

**Pro Tip:** Focus on building quality traffic first! Revenue follows traffic. A pizza review site with authentic content and good SEO can generate significant ad revenue. 🍕💰
