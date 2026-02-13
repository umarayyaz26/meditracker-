# Submission Checklist

Before submitting, complete these steps:

## 1. Replace placeholders

✓ Done — GitHub username `umarayyaz26` is set in README, Cargo.toml, website, and docs.

## 2. Upload to GitHub

1. Create a new repository on GitHub (e.g. `meditracker`).
2. Push this project to it.

## 3. Build and publish Linux binaries

The GitHub Actions workflow builds Linux binaries automatically when you push a tag:

```bash
git tag v1.0.2
git push origin v1.0.2
```

Wait for the workflow to finish, then download the `.tar.gz` files from the Releases page.

## 4. Publish the website

**Option A — GitHub Pages:**
1. A `docs/` folder is already included with the website content.
2. Go to your repo → Settings → Pages.
3. Source: Deploy from branch → Branch: `main` → Folder: `/docs`.
4. Your site will be at `https://umarayyaz26.github.io/meditracker/`.

**Option B — Manual:**
Include the `website/` folder contents in your submission as the "content of your website."

## 5. What to submit

| Item | Location |
|------|----------|
| **Linux binary (x86_64)** | From Releases: `meditracker-x86_64-unknown-linux-musl.tar.gz` |
| **Linux binary (aarch64)** | From Releases: `meditracker-aarch64-unknown-linux-musl.tar.gz` |
| **Rust source code** | `Cargo.toml` + everything in `src/` |
| **Website content** | `website/` folder (index.html, styles.css) |
| **GitHub link** | Your repo URL |
| **Website link** | Your published site URL |
