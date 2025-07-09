#!/bin/bash

# Build the demo with the correct public URL for GitHub Pages
echo "Building demo for GitHub Pages..."
cd demo
trunk build --release --public-url /yew-shortcuts/

# Create a temporary directory for the gh-pages branch
echo "Preparing gh-pages branch..."
cd ..
rm -rf /tmp/yew-shortcuts-gh-pages
mkdir -p /tmp/yew-shortcuts-gh-pages
cp -r demo/dist/* /tmp/yew-shortcuts-gh-pages/

# Save current branch
CURRENT_BRANCH=$(git branch --show-current)

# Switch to gh-pages branch (create if doesn't exist)
if git show-ref --verify --quiet refs/heads/gh-pages; then
    git checkout gh-pages
else
    git checkout --orphan gh-pages
    git rm -rf .
fi

# Copy the built files
cp -r /tmp/yew-shortcuts-gh-pages/* .

# Add and commit
git add .
git commit -m "Deploy demo to GitHub Pages"

# Push to gh-pages branch
echo "Pushing to gh-pages branch..."
git push origin gh-pages --force

# Switch back to original branch
git checkout $CURRENT_BRANCH

echo "Deployment complete! The demo will be available at:"
echo "https://madoshakalaka.github.io/yew-shortcuts/"
echo ""
echo "Note: It may take a few minutes for GitHub Pages to update."