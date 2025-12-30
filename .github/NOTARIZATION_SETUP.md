# Notarization Setup for macOS

Notarization is required for macOS binaries to pass Gatekeeper checks on macOS Catalina (10.15) and later.

## Step 1: Create App-Specific Password

1. Go to [Apple ID Account Page](https://appleid.apple.com/)
2. Sign in with your Apple ID
3. Go to **Security** section
4. Under **App-Specific Passwords**, click **Generate Password**
5. Give it a label like "GitHub Actions Notarization"
6. Copy the generated password (you'll only see it once!)

## Step 2: Get Your Team ID

Your Team ID is in your Developer ID certificate name. It's the part in parentheses:
- Example: `Developer ID Application: Your Name (TEAM_ID123)`

Or find it at: [Apple Developer Membership](https://developer.apple.com/account)

## Step 3: Add GitHub Secrets

Go to your GitHub repository → **Settings** → **Secrets and variables** → **Actions**

Add these secrets:

### `APPLE_ID`
- **Value**: Your Apple ID email address (the one used for your Developer account)

### `APPLE_APP_SPECIFIC_PASSWORD`
- **Value**: The app-specific password you created in Step 1

### `APPLE_TEAM_ID`
- **Value**: Your Team ID (the part in parentheses from your Developer ID, e.g., `TEAM_ID123`)

## How It Works

When you create a release, the workflow will:
1. Code sign the binary ✅
2. Create a zip file
3. Submit to Apple for notarization
4. Wait for Apple to process (usually 1-5 minutes)
5. Staple the notarization ticket to the binary
6. Verify it passes Gatekeeper

## Verification

After notarization, users can verify with:
```bash
spctl --assess --verbose ffhuman
# Should output: "accepted" instead of "rejected"
```

## Troubleshooting

### "Invalid credentials"
- Make sure your Apple ID is correct
- Regenerate the app-specific password if needed
- Verify Team ID matches your Developer account

### "Notarization timeout"
- Notarization can take up to 30 minutes (workflow waits 30m)
- Check Apple Developer portal for status
- You can manually check status with: `xcrun notarytool history`

### Still shows "Unnotarized"
- Make sure you're downloading the latest release
- Clear browser cache and re-download
- Wait a few minutes for Apple's CDN to update

