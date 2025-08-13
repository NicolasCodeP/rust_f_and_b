#!/bin/bash

# Mobile WIP Netlify Setup Script
echo "🚀 Setting up Netlify deployment for mobile-optimizations-wip branch"
echo ""

echo "📋 Steps to complete the setup:"
echo ""

echo "1. Create a new Netlify site for your mobile WIP:"
echo "   - Go to https://app.netlify.com/"
echo "   - Click 'Add new site' > 'Import an existing project'"
echo "   - Connect to your GitHub repository"
echo "   - Choose the 'mobile-optimizations-wip' branch"
echo "   - Set build command: './trunk build --release'"
echo "   - Set publish directory: 'dist'"
echo "   - Deploy the site"
echo ""

echo "2. Get your new site ID:"
echo "   - In your new Netlify site dashboard"
echo "   - Go to Site settings > General"
echo "   - Copy the Site ID (looks like: a1b2c3d4-e5f6-7890-abcd-ef1234567890)"
echo ""

echo "3. Add GitHub Secret:"
echo "   - Go to your GitHub repository"
echo "   - Settings > Secrets and variables > Actions"
echo "   - Click 'New repository secret'"
echo "   - Name: NETLIFY_MOBILE_WIP_SITE_ID"
echo "   - Value: [paste your site ID from step 2]"
echo ""

echo "4. Optional - Update the workflow file:"
echo "   - Edit .github/workflows/netlify-deploy-mobile-wip.yml"
echo "   - Replace 'your-mobile-wip-site.netlify.app' with your actual site URL"
echo ""

echo "5. Push this branch to trigger the deployment:"
echo "   git add ."
echo "   git commit -m 'Add mobile WIP deployment workflow'"
echo "   git push origin mobile-optimizations-wip"
echo ""

echo "✅ Your mobile WIP branch will now deploy to a separate Netlify site!"
echo "🔗 Main site: [your main site]"
echo "🔗 Mobile WIP site: [your mobile wip site]"
echo ""

echo "🔧 Useful commands:"
echo "   - View logs: git log --oneline"
echo "   - Switch branches: git checkout main"
echo "   - Back to mobile: git checkout mobile-optimizations-wip"
