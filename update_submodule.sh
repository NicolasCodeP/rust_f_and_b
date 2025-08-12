#!/bin/bash
# Steps to update submodule in main repository

# Step 1: Update the submodule to latest version
cd poc_fnb
git checkout main
git pull origin main
cd ..

# Step 2: Check status
git submodule status

# Step 3: Add and commit the updated reference
git add poc_fnb
git commit -m "Update poc_fnb submodule to latest version"

echo "Done! Now push with: git push"
