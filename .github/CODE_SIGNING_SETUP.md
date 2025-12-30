# Code Signing Setup for macOS

This guide explains how to set up code signing for macOS binaries in GitHub Actions.

## Prerequisites

- Apple Developer account ($99/year)
- Access to your GitHub repository settings

## Step 1: Create a Developer ID Certificate

1. Go to [Apple Developer Portal](https://developer.apple.com/account/resources/certificates/list)
2. Click the **+** button to create a new certificate
3. Select **Developer ID Application** (for distribution outside the App Store)
4. Follow the instructions to create a Certificate Signing Request (CSR)
5. Download the certificate (`.cer` file)
6. Double-click the `.cer` file to add it to your Keychain

## Step 2: Export Certificate as .p12

1. Open **Keychain Access** on your Mac
2. Find your **Developer ID Application** certificate
3. Right-click → **Export "Developer ID Application: Your Name"**
4. Choose **Personal Information Exchange (.p12)** format
5. Set a password (you'll need this for the GitHub secret)
6. Save the file

## Step 3: Convert to Base64

Run this command in Terminal (replace the path):

```bash
base64 -i /path/to/your/certificate.p12 | pbcopy
```

This copies the base64-encoded certificate to your clipboard.

## Step 4: Get Your Developer ID

Your Developer ID is in the format: `Developer ID Application: Your Name (TEAM_ID)`

You can find it by:
1. Opening Keychain Access
2. Finding your Developer ID Application certificate
3. The full name is your Developer ID

Or run:
```bash
security find-identity -v -p codesigning
```

Look for the line with "Developer ID Application".

## Step 5: Add GitHub Secrets

1. Go to your GitHub repository
2. Click **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**

Add these three secrets:

### `MACOS_CERTIFICATE`
- **Value**: Paste the base64-encoded certificate from Step 3
- This is the entire output of the `base64` command

### `MACOS_CERTIFICATE_PASSWORD`
- **Value**: The password you set when exporting the .p12 file in Step 2

### `MACOS_DEVELOPER_ID`
- **Value**: Your full Developer ID (e.g., `Developer ID Application: Your Name (TEAM_ID)`)
- Include the full string, including the team ID in parentheses

## Step 6: Verify Setup

After adding the secrets, the next time you create a release tag, the workflow will:
1. Import the certificate
2. Code sign the macOS binaries
3. Verify the signing

You can verify a signed binary locally with:
```bash
codesign --verify --verbose ffhuman
spctl --assess --verbose ffhuman
```

## Troubleshooting

### "No identity found"
- Make sure `MACOS_DEVELOPER_ID` includes the full string with team ID
- Check that the certificate is a "Developer ID Application" type

### "The specified item could not be found in the keychain"
- Verify the certificate password is correct
- Make sure the base64 encoding is complete (no line breaks)

### Binary still shows security warning
- Code signing might take a few minutes to propagate
- Try downloading the binary again after a few minutes
- Make sure you're using the signed binary from the release

## Optional: Notarization

For even better security (no warnings at all), you can add notarization. This requires:
- App-specific password from Apple
- Additional GitHub secrets
- Longer build time

Notarization is optional but recommended for production releases.

