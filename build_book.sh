#!/usr/bin/env bash
set -euo pipefail
if ! command -v mdbook >/dev/null 2>&1; then
  echo "mdbook not found. Install with: cargo install mdbook" >&2
  exit 1
fi

# Ensure the output directory exists
mkdir -p book/book

echo "Building English book..."
mdbook build book/en -d "$(pwd)/book/book/en"

echo "Building Japanese book..."
mdbook build book/ja -d "$(pwd)/book/book/ja"

# Create a landing page
cat <<EOF > book/book/index.html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>WASI Virtual Layer Documentation</title>
    <style>
        body { font-family: sans-serif; display: flex; justify-content: center; align-items: center; height: 100vh; margin: 0; background-color: #f4f4f4; }
        .container { text-align: center; background: white; padding: 2rem; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #333; }
        .links { margin-top: 2rem; }
        .links a { display: inline-block; margin: 0 1rem; padding: 0.5rem 1.5rem; background: #007bff; color: white; text-decoration: none; border-radius: 4px; }
        .links a:hover { background: #0056b3; }
    </style>
</head>
<body>
    <div class="container">
        <h1>WASI Virtual Layer Documentation</h1>
        <div class="links">
            <a href="en/index.html">English</a>
            <a href="ja/index.html">日本語</a>
        </div>
    </div>
</body>
</html>
EOF

echo "Done! Book is available in book/book/"
