# Quick Deploy Commands

## After creating GitHub repo, run:

```bash
# Add your GitHub repo (replace YOUR_USERNAME)
git remote add origin https://github.com/YOUR_USERNAME/bestpizzasincalifornia.git

# Push to GitHub
git push -u origin main
```

## Then go to Cloudflare Pages:

1. Visit: https://dash.cloudflare.com/
2. Click **Pages** → **Create a project**
3. Click **Connect to Git**
4. Select your GitHub repository: `bestpizzasincalifornia`
5. Configure build settings:
   - **Framework preset**: None
   - **Build command**: `dx build --release --platform web`
   - **Build output directory**: `target/dx/app/release/web/public`
   - **Root directory**: (leave empty)
   - **Environment variables**: (none needed)
6. Click **Save and Deploy**

## Your site will be live at:
`https://bestpizzasincalifornia.pages.dev`

## After it's deployed:

1. Test the site at the `.pages.dev` URL
2. Once confirmed working, we'll update GoDaddy nameservers
3. Then add custom domain in Cloudflare Pages

---

**Need help?** Check `DEPLOYMENT_GUIDE.md` for detailed instructions!
