# Mobile WIP Deployment Setup

This branch (`mobile-optimizations-wip`) has its own deployment pipeline to a separate Netlify site for testing mobile optimizations.

## Deployment Workflow

- **File**: `.github/workflows/netlify-deploy-mobile-wip.yml`
- **Triggers**: Push or PR to `mobile-optimizations-wip` branch
- **Deploys to**: Separate Netlify site (different from main)

## Mobile Optimizations Included

This branch includes:

1. **Responsive UI Layout**
   - Desktop: Side panel layout (ingredients left, recipes right)
   - Mobile: Tabbed layout (switch between ingredients and recipes)
   - Automatic detection based on screen width (< 768px)

2. **Touch-Friendly Interface**
   - Larger button padding (12px x 8px)
   - Better spacing between elements
   - Larger text for mobile readability

3. **Mobile-Specific HTML/CSS**
   - Proper viewport configuration
   - Touch-friendly CSS (disabled zooming, better canvas sizing)
   - Mobile-optimized loading indicators

4. **PWA Support**
   - Enhanced service worker with offline support
   - Better manifest.json for mobile installation
   - Improved caching strategy

## Setup Instructions

1. Run the setup script:
   ```bash
   ./setup-mobile-wip-netlify.sh
   ```

2. Follow the printed instructions to:
   - Create a new Netlify site
   - Configure GitHub secrets
   - Trigger the first deployment

## Comparing Deployments

- **Main site**: Standard desktop-focused layout
- **Mobile WIP site**: Mobile-optimized with responsive design

## Development Workflow

```bash
# Work on mobile optimizations
git checkout mobile-optimizations-wip

# Make changes
# ... edit files ...

# Test locally
trunk serve --port 8080

# Deploy to mobile WIP site
git add .
git commit -m "Mobile improvements"
git push origin mobile-optimizations-wip
```

## Testing Mobile

1. **Desktop Browser**: Resize window < 768px width to see mobile layout
2. **Mobile Device**: Access the mobile WIP site directly
3. **Developer Tools**: Use responsive design mode

## Merging Back

When mobile optimizations are ready:

```bash
git checkout main
git merge mobile-optimizations-wip
git push origin main
```

This will deploy the mobile optimizations to the main site as well.
