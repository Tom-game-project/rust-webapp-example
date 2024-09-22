# Yew example

## build

```bash
wasm-pack build --target web --out-name wasm --out-dir ./dist
```

tailwind
```bash
npx tailwindcss -i ./static/style.css -o ./dist/style.css 
#--watch
```

## server
```bash
python3 -m http.server
```